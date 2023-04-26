use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;

pub fn start(path: &String) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
