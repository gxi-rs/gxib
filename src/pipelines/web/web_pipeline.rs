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
    #[allow(dead_code)]
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
            web_args.target_dir = Path::new(&web_args.target_dir)
                .absolutize()
                .with_context(|| {
                    format!(
                        "error getting absolute path for --target-dir {}",
                        web_args.target_dir
                    )
                })?
                .to_str()
                .unwrap()
                .to_string();
            // make output dir absolute
            web_args.output_dir = Path::new(&web_args.output_dir)
                .absolutize()
                .with_context(|| {
                    format!(
                        "error getting absolute path for --output-dir {}",
                        web_args.output_dir
                    )
                })?
                .to_str()
                .unwrap()
                .to_string();
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
        println!("building web");
        // check args
        {
            let web_args = &this.args.subcmd.as_web()?;
            // do a full build
            this.build_full().await?;
            // check if serve
            if web_args.serve {
                // channel wrote to when build is complete which is read by the server
                let (build_tx, build_rx) = watch::channel(ActorMsg::None);

                let watcher = Self::watch(this, build_tx);
                let server = start_web_server(build_rx);

                // wait for both to complete and set context for each
                let (watcher_result, server_result) = tokio::join!(watcher, server);
                watcher_result.with_context(|| "Error while watching local file changes")??;
                server_result.with_context(|| "Error while launching server")??;
            }
        }
        Ok(())
    }

    pub fn watch(
        this: Self,
        build_tx: watch::Sender<ActorMsg>,
    ) -> impl Future<Output=Result<Result<()>, task::JoinError>> {
        task::spawn(async move {
            // watcher

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
                    Err(e) => eprintln!("Error while watching dir\n{}", e),
                })
                    .with_context(|| "Error initialising watcher")?;

            watcher
                .watch(format!("{}/src", &this.args.dir), RecursiveMode::Recursive)
                .with_context(|| format!("error watching {}/src", &this.args.dir))?;

            while rx.changed().await.is_ok() {
                match this.build().await {
                    Err(err) => eprintln!("Error while building\n{}", err),
                    // build full only when cargo build is successful
                    _ => {
                        this.build_full().await?;
                        build_tx.send(ActorMsg::FileChange(this.wasm_hashed_name.clone()))?;
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
            &web_subcmd.target_dir,
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
                "--keep-debug",
                "--target",
                "web",
                // no type script
                "--no-typescript",
                // dir to place the assets at
                "--out-dir",
                &web_subcmd.output_dir,
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
        format!(
            r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <link class="gxib-pre-link" rel="preload" href="/{name}.wasm" as="fetch" type="application/wasm">
    <link class="gxib-pre-link" rel="modulepreload" href="/{name}.js">
  </head>
  <body>
    <script type="module">
        window.set_tree_pointer = pointer => {{
            window.tree_pointer = pointer;
        }};
        window.get_tree_pointer = () => {{
            return window.tree_pointer | 0;
        }};
        (function () {{
            const socket = new WebSocket('ws://localhost:8080/__gxi__');
            socket.addEventListener('open', function (event) {{
                console.log("connected");
            }});
            socket.addEventListener('message', function (event) {{
                const data = JSON.parse(event.data);
                if (data.event === "FileChange") {{
                    {{
                        const pre_links = document.getElementsByClassName("gxib-pre-link")
                        for ( const el of pre_links )
                            el.remove();
                    }}
                    const [js_name,wasm_name] = [`/${{data.hashed_name}}.js`,`/${{data.hashed_name}}.wasm`];
                    {{
                        for ( const attrs of [[
                            ["rel","preload"],
                            ["href",js_name],
                            ["as","fetch"],
                            ["type","application/wasm"]
                        ],[
                            ["rel","modulepreload"],
                            ["href",wasm_name]
                        ]] ) {{
                            const pre_link = document.createElement("link");
                            pre_link.setAttribute("class","gxib-pre-link");
                            for ( const attr of attrs )
                                pre_link.setAttribute(attr[0],attr[1]);
                            document.head.appendChild(pre_link)
                        }}
                    }}
                    import(js_name).then(mod => mod.default(wasm_name))
                }}
                console.log('Message from server ', data);
            }});
        }})()
      import init from '/{name}.js'; init('/{name}.wasm');
    </script>
  </body>
</html>"#,
            name = self.wasm_hashed_name
        )
    }
}
