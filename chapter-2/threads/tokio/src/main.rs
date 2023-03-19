use rand::prelude::*;
use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    // spawn 100 tasks on async runtime
    let task_handles: Vec<task::JoinHandle<_>> = (1..=100)
        .map(|index| {
            // 1-20 seconds of delay
            let delay = rand::thread_rng().gen_range(1000..20000);

            // Spawn a new task on async runtime
            task::spawn(async move {
                // simulate execution with a delay
                tokio::time::sleep(Duration::from_millis(delay)).await;

                // message when the task is finished
                println!(
                    "☑️ Thread: {} Done in {}ms",
                    format!("{}-0x{}",std::thread::current().name().unwrap(), index),
                    delay
                );
            })
        })
        .collect();

    // Wait for all tasks to finish
    for task in task_handles {
        let _ = task.await;
    }

    println!("✅ all job finished");
}
