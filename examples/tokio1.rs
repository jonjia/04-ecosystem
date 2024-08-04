use std::{thread, time::Duration};

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Future 1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("File content length: {}", content.len());
        });

        rt.spawn(async {
            println!("Future 2");
            let ret = expensive_blocking_task("Future 2".to_string());
            println!("Hash: {}", ret);
        });

        rt.block_on(async {
            println!("Blocking task");
            sleep(Duration::from_secs(1)).await;
            println!("Blocking task done");
        });
    });

    handle.join().unwrap();
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(std::time::Duration::from_millis(1800));
    blake3::hash(s.as_bytes()).to_string()
}
