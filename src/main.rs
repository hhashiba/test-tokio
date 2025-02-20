use anyhow::{anyhow, Error, Result};
use futures::stream::{StreamExt, TryStreamExt};
use std::sync::Arc;
use threadpool::ThreadPool;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration as TokioDuration};
use tracing::error;

const THREADS_SIZE: usize = 4;

#[tokio::main]
async fn main() -> Result<(), Error> {
    console_subscriber::init();

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // let (tx, mut rx) = mpsc::channel(4);

    // for v in values {
    //     let tx2 = tx.to_owned();
    //     tokio::spawn(async move {
    //         let result = calc(v).await;
    //         tx2.send(result).await
    //     });
    // }
    // drop(tx);

    // let mut results = vec![];
    // while let Some(result) = rx.recv().await {
    //     results.push(result);
    // }

    // let semaphore = Arc::new(Semaphore::new(4));
    // let mut handles = vec![];

    // for v in values {
    //     let permit = semaphore.clone().acquire_owned().await.map_err(|e| {
    //         let err_message = "Failed to acquire semaphore permit";
    //         error!("{}: {}", err_message, e);
    //         return anyhow!(err_message);
    //     });

    //     let handle = tokio::spawn(async move {
    //         let res = calc(v).await;
    //         drop(permit);
    //         res
    //     });
    //     handles.push(handle);
    // }

    // let mut results = vec![];

    // for handle in handles {
    //     results.push(handle.await);
    // }

    // let (tx2, mut rx2) = mpsc::channel(4);

    // while let Some(result) = rx.recv().await {
    //     let tx22 = tx2.to_owned();
    //     tokio::spawn(async move {
    //         let res = result.await;
    //         tx22.send(res).await
    //     });
    // }
    // drop(tx2);

    // let mut results: Vec<Result<u16>> = vec![];
    // while let Some(result) = rx.recv().await {
    //     results.push(result.await);
    // }

    let results: Vec<u16> = tokio_stream::iter(values)
        .map(|v| tokio::spawn(async move { calc(v).await }))
        .buffered(THREADS_SIZE)
        .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
        .try_collect()
        .await?;

    // let results: Vec<Result<u16>> = tokio_stream::iter(values)
    //     .map(|v| tokio::spawn(async move { calc(v).await }))
    //     .buffer_unordered(4)
    //     .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
    //     .collect()
    //     .await;

    println!("{:?}", results);

    Ok(())
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

// use std::future::Future;

// /// Executes asynchronous functions in parallel while maintaining the order of results.
// async fn execute_parallel_ordered<I, F, Fut, T, E>(
//     inputs: &I,
//     parallelism: usize,
//     func: F,
// ) -> Vec<Result<T, E>>
// where
//     I: IntoIterator + Clone,
//     F: Fn(&I::Item) -> Fut + Send + Sync,
//     Fut: Future<Output = Result<T, E>> + Send,
//     I::Item: Send + 'static,
//     T: Send + 'static,
//     E: Send + 'static,
// {
//     let arg_inputs = inputs.to_owned();
//     futures::stream::iter(arg_inputs)
//         .map(|v| func(&v))
//         .buffered(parallelism)
//         .collect()
//         .await
// }

// /// Executes asynchronous functions in parallel without maintaining the order of results.
// async fn execute_parallel_unordered<I, F, Fut, T>(
//     inputs: &I,
//     parallelism: usize,
//     func: F,
// ) -> Vec<Result<T, Error>>
// where
//     I: IntoIterator + Clone,
//     F: Fn(&I::Item) -> Fut + Send + Sync,
//     Fut: Future<Output = Result<T, Error>> + Send,
//     I::Item: Send + 'static,
//     T: Send + 'static,
// {
//     let arg_inputs = inputs.to_owned();
//     futures::stream::iter(arg_inputs)
//         .map(|v| tokio::spawn(async move { func(&v) }))
//         .buffer_unordered(parallelism)
//         .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
//         .collect()
//         .await
// }
