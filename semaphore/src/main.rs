use anyhow::{anyhow, Error, Result};
use std::sync::Arc;
use tokio::{
    sync::Semaphore,
    time::{sleep, Duration as TokioDuration},
};
use tracing::error;

const MAX_PERMITS: usize = 4;

#[tokio::main]
async fn main() {
    // console_subscriber::init(); // use for tokio-console

    tracing_subscriber::fmt()
        .json()
        .with_current_span(false)
        .flatten_event(true)
        .with_span_list(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    semaphore_sorted().await
}

async fn semaphore_sorted() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let semaphore = Arc::new(Semaphore::new(MAX_PERMITS));
    let mut handles = vec![];

    for v in values {
        let permit = semaphore.clone().acquire_owned().await.map_err(|e| {
            let err_message = "Failed to acquire semaphore permit";
            error!("{}: {}", err_message, e);
            return anyhow!(err_message);
        });

        let handle = tokio::spawn(async move {
            let res = calc(v).await;
            drop(permit);
            res
        });
        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.await);
    }

    println!("{:?}", results);
}

async fn calc(v: u16) -> Result<u16, Error> {
    let thread_id = std::thread::current().id();
    println!("thread_id: {:?} value: {}", thread_id, v);

    if v % 2 == 0 {
        println!("this value is even. num: {}", v);
        println!("sleep 1 second. num :{}", v);

        sleep(TokioDuration::from_secs(5)).await;
    } else {
        println!("this value is odd. num: {}", v);
        println!("sleep 3 second. num :{}", v);
        sleep(TokioDuration::from_secs(3)).await;
    }

    Ok(v)
}
