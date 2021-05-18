use notify::{*};
use crate::{*};
use futures::future::Future;

pub fn watch(dir:String) -> impl Future {
    //spawn a new async thread
    tokio::task::spawn(async move {
        let dir = dir;
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        })
            .with_context(|| "Error initialising watcher")?;
        watcher
            .watch(format!("{}/src", &dir), RecursiveMode::Recursive)
            .with_context(|| format!("error watching {}/src", &dir))?;
        println!("watching");
        // block the thread to prevent the programme from quitting 
        std::thread::park();
        Ok::<(),anyhow::Error>(())
    })
}
