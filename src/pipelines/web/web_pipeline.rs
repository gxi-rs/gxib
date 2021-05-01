use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::*;

pub const WEB_FEATURE: &str = "web";
pub const WEB_TARGET: &str = "wasm32-unknown-unknown";

/// web pipeline using wasm
pub struct WebPipeline<'a> {
    pub args: &'a Args,
    pub cargo_toml: &'a mut CargoToml,
}

impl WebPipeline<'_> {
    pub async fn run(&mut self) -> Result<()> {
        println!("building web");
        // write web feature
        {
            self.cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
            self.cargo_toml.write_to_file().await?;
        }
        // check args
        {
            let web_args = self.args.subcmd.as_web()?;
            let build_future = self.build();
            if web_args.serve {
                //join both instead of spawning a new thread
                {
                    let (build, watch) = tokio::join!(build_future, self.watch());
                    build?;
                    watch?;
                }
            } else {
                build_future.await?;
            }
        }
        Ok(())
    }

    pub async fn build(&self) -> Result<()> {
        let mut args = vec!["build", "--target", WEB_TARGET];
        if self.args.subcmd.as_web()?.release {
            args.push("--release")
        }
        exec_cmd(
            "cargo",
            &args,
            Some(&self.args.dir),
            None,
        ).await.with_context(|| format!("error building for web"))?;
        Ok(())
    }

    pub async fn watch(&self) -> Result<()> {
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        })
            .with_context(|| "Error initialising watcher")?;

        watcher
            .watch(format!("{}/src", self.args.dir), RecursiveMode::Recursive)
            .with_context(|| format!("error watching {}/src", self.args.dir))?;
        Ok(())
    }
}
