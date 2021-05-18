use crate::*;
use futures::future::Future;
use notify::*;

pub fn watch<F>(dir: String, handeler: F) -> impl Future
where
    F: EventFn,
{
    //spawn a new async thread
    tokio::task::spawn(async move {
        let dir = dir;
        let mut watcher: RecommendedWatcher = Watcher::new_immediate(handeler)
        .with_context(|| "Error initialising watcher")?;
        watcher
            .watch(format!("{}/src", &dir), RecursiveMode::Recursive)
            .with_context(|| format!("error watching {}/src", &dir))?;
        println!("watching");
        // block the thread to prevent the programme from quitting
        std::thread::park();
        Ok::<(), anyhow::Error>(())
    })
}
