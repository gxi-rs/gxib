use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use crate::*;

pub const WEB_FEATURE: &str = "web";

/// web pipeline using wasm
pub struct WebPipeline<'a> {
    pub args: &'a Args,
    pub cargo_toml: &'a mut CargoToml,
}

impl WebPipeline<'_> {
    pub async fn run(&mut self) -> Result<()> {
        // write web feature
        {
            self.cargo_toml.add_features(vec![WEB_FEATURE.to_string()]);
            self.cargo_toml.write_to_file().await?;
        }
        let web_args = self.args.subcmd.as_web()?;

        if web_args.serve {
            let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            })
            .with_context(|| "Error initialising watcher")?;

            watcher
                .watch(format!("{}/src", self.args.dir), RecursiveMode::Recursive)
                .with_context(|| format!("error watching {}/src", self.args.dir))?;
        }
        println!("building web");
        Ok(())
    }
}
