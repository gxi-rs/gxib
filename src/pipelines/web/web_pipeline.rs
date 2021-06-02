use crate::*;
use futures::future::Future;
use notify::{event, RecommendedWatcher, RecursiveMode, Watcher};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use tokio::sync::watch;
use tokio::task;

pub const WEB_FEATURE: &str = "web";
pub const WEB_TARGET: &str = "wasm32-unknown-unknown";

/// web pipeline using wasm
pub struct WebPipeline {
    args: Args,
    cargo_toml: CargoToml,
    /// hash of wasm file generated by cargo build
    wasm_hashed_name: String,
}

impl WebPipeline {
    /// builds and generates Self struct
    pub async fn new(mut args: Args, mut cargo_toml: CargoToml) -> Result<WebPipeline> {
        // write web feature
        {
            cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
            cargo_toml.write_to_file().await?;
        }
        // init args
        {
            let web_args = args.subcmd.as_web_mut()?;
            // make target dir absolute
            web_args.target_dir = web_args.target_dir
                .absolutize()
                .with_context(|| {
                    format!(
                        "error getting absolute path for --target-dir {}",
                        web_args.target_dir.to_str().unwrap()
                    )
                })?
                .to_path_buf();
            // make output dir absolute
            web_args.output_dir = web_args.output_dir
                .absolutize()
                .with_context(|| {
                    format!(
                        "error getting absolute path for --output-dir {}",
                        web_args.output_dir.to_str().unwrap()
                    )
                })?
                .to_path_buf();
        }
        let mut this = Self {
            args,
            cargo_toml,
            //make dummy wasm_hashed_name
            wasm_hashed_name: String::new(),
        };
        // build to make valid hashed_wasm_name
        this.build().await?;
        this.wasm_hashed_name = this.generate_wasm_hashed_name().await?;
        Ok(this)
    }

    /// runs commands according to args
    pub async fn run(this: Self) -> Result<()> {
        // check args
        {
            let web_args = &this.args.subcmd.as_web()?;
            // do a full build
            this.build_full().await?;
            const WATCHER_ERROR: &str = "Error while watching local file changes";
            const SERVER_ERROR: &str = "Error while launching server";

            // watch and serve
            if web_args.watch && web_args.serve.is_some() {
                // create channels only when hot reload is enabled
                let (build_tx, build_rx) = if web_args.hot_reload {
                    let (build_tx, build_rx) = watch::channel(ActorMsg::None);
                    (Some(build_tx), Some(build_rx))
                } else {
                    (None, None)
                };

                let server = start_web_server(build_rx, web_args.output_dir.clone(), web_args.serve.as_ref().unwrap().clone());
                let watcher = Self::watch(this, build_tx);

                // wait for both to complete and set context for each
                let (watcher_result, server_result) = tokio::join!(watcher, server);
                watcher_result.with_context(|| WATCHER_ERROR)??;
                server_result.with_context(|| SERVER_ERROR)??;
            }
            // if only serve
            else if let Some(serve) = &web_args.serve {
                start_web_server(None, web_args.output_dir.clone(), serve.clone()).await.with_context(|| SERVER_ERROR)??;
            }
            // if only watch
            else if web_args.watch {
                Self::watch(this, None).await.with_context(|| WATCHER_ERROR)??;
            }
        }
        Ok(())
    }

    pub fn watch(
        this: Self,
        build_tx: Option<watch::Sender<ActorMsg>>,
    ) -> impl Future<Output=Result<Result<()>, task::JoinError>> {
        task::spawn(async move {
            info!("Watching");
            let (tx, mut rx) = watch::channel(());
            let mut watcher: RecommendedWatcher =
                Watcher::new_immediate(move |res: notify::Result<event::Event>| match res {
                    Ok(event) => {
                        if let event::EventKind::Modify(modify_event) = &event.kind {
                            if let event::ModifyKind::Data(_) = modify_event {
                                tx.send(()).unwrap();
                            }
                        }
                    }
                    Err(e) => error!("Error while watching dir\n{}", e),
                })
                    .with_context(|| "Error initialising watcher")?;

            watcher
                .watch(format!("{}/src", &this.args.dir), RecursiveMode::Recursive)
                .with_context(|| format!("error watching {}/src", &this.args.dir))?;

            while rx.changed().await.is_ok() {
                info!("Re-building");
                match this.build().await {
                    Err(err) => error!("Error while building\n{}", err),
                    // build full only when cargo build is successful
                    _ => {
                        this.build_full().await?;
                        if let Some(build_tx) = &build_tx {
                            build_tx.send(ActorMsg::FileChange(this.wasm_hashed_name.clone()))?;
                        }
                    }
                }
            }

            Err::<(), anyhow::Error>(anyhow!("Watch exited unexpectedly"))
        })
    }

    /// builds bindings
    /// optimises build if release
    /// writes html
    pub async fn build_full(&self) -> Result<()> {
        let web_args = self.args.subcmd.as_web()?;
        // need not build again because it has already been done in new block
        // run wasm bindgen
        self.build_bindings().await?;
        // optimise bindings if release using binaryen
        if web_args.release {
            self.optimise_build().await?;
        }
        // write html
        write(
            Path::new(&web_args.output_dir).join("index.html"),
            self.generate_html(),
        )
            .await?;
        Ok(())
    }

    pub async fn optimise_build(&self) -> Result<()> {
        let web_subcmd = self.args.subcmd.as_web()?;
        let file_name = format!("{}.wasm", &self.wasm_hashed_name);
        exec_cmd(
            "wasm-opt",
            &["-Oz", "--dce", &file_name, "-o", &file_name],
            Some(&web_subcmd.output_dir),
            None,
        )
            .await?;
        Ok(())
    }

    /// generates path .wasm file generated by cargo build
    /// target/wasm32-unknown-unknown/{release/debug}/<name>.wasm
    pub fn generate_path_to_target_wasm(&self) -> Result<PathBuf> {
        let web_subcmd = self.args.subcmd.as_web()?;
        let path = Path::new(&web_subcmd.target_dir)
            .join(WEB_TARGET)
            .join(if web_subcmd.release {
                "release"
            } else {
                "debug"
            })
            .join(&format!("{}.wasm", &self.cargo_toml.package_name));
        let path = path.absolutize()?;
        if !path.exists() {
            bail!("Expected wasm at {}", path.to_str().unwrap())
        }
        Ok(PathBuf::from(path))
    }

    /// generates hash for the wasm generated by cargo build
    /// prefixes index-
    pub async fn generate_wasm_hashed_name(&self) -> Result<String> {
        Ok(format!(
            "index-{}",
            get_file_hash(self.generate_path_to_target_wasm()?).await?
        ))
    }

    /// run cargo build
    pub async fn build(&self) -> Result<()> {
        let web_subcmd = self.args.subcmd.as_web()?;
        let mut args = vec![
            "build",
            "--target",
            WEB_TARGET,
            "--target-dir",
            &web_subcmd.target_dir.to_str().unwrap(),
        ];
        if web_subcmd.release {
            args.push("--release")
        }
        exec_cmd("cargo", &args, Some(&self.args.dir), None)
            .await
            .with_context(|| format!("error running cargo to build for web"))?;
        Ok(())
    }

    /// rust wasm-bindgen on the target binary
    pub async fn build_bindings(&self) -> Result<()> {
        let web_subcmd = self.args.subcmd.as_web()?;
        exec_cmd(
            "wasm-bindgen",
            &vec![
                self.generate_path_to_target_wasm()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                // build for web
                "--target",
                "web",
                // no type script
                "--no-typescript",
                // dir to place the assets at
                "--out-dir",
                web_subcmd.output_dir.to_str().unwrap(),
                // name of output file
                "--out-name",
                &format!("{}.wasm", self.wasm_hashed_name.as_str()),
            ],
            Option::<&str>::None,
            None,
        )
            .await
            .with_context(|| format!("error running cargo-bindgen on "))?;
        Ok(())
    }

    /// generates html
    pub fn generate_html(&self) -> String {
        let web_args = self.args.subcmd.as_web().unwrap();
        let hot_reload_script = if web_args.hot_reload {
            format!(r#"(function () {{
    const socket = new WebSocket('ws://{serve_addrs}/__gxi__');
    socket.addEventListener('open', function (event) {{
        console.log("Gxib > Connected to Server: Hot Reload Enabled");
    }});
    socket.addEventListener('close', function (event) {{
        console.error("Gxib > Disconnected from server");
        if (confirm('Disconnected from server. Refresh ?'))
            location.reload()
    }});
    socket.addEventListener('message', event => {{
        const data = JSON.parse(event.data);
        if (data.event === "FileChange")
            location.reload()
        console.log('Gxib > Message from server ', data);
    }});
}})()"#, serve_addrs = web_args.serve.as_ref().unwrap())
        } else {
            String::new()
        };
        format!(
            r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <link rel="preload" href="/{name}.wasm" as="fetch" type="application/wasm">
    <link rel="modulepreload" href="/{name}.js">
  </head>
  <body>
    <script type="module">
        {hot_reload_script}
        import init from '/{name}.js'; init('/{name}.wasm');
    </script>
  </body>
</html>"#,
            name = self.wasm_hashed_name,
            hot_reload_script = hot_reload_script
        )
    }
}
