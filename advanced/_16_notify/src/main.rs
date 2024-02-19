use banner::print_banner;
use notify::{Event, Watcher};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    print_banner();

    if let Some(home) = home::home_dir() {
        let source_path = home.as_path();

        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(source_path, notify::RecursiveMode::NonRecursive)?;

        for res in rx {
            handle_watcher_result(res);
        }
    }

    Ok(())
}

fn handle_watcher_result(res: notify::Result<Event>) {
    match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    }
}
