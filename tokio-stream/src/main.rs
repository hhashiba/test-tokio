use anyhow::{anyhow, Error, Result};
use futures::stream::{StreamExt, TryStreamExt};
use serde::Deserialize;
use tokio::time::{sleep, Duration as TokioDuration};
use tracing::info;

const BUFFER_SIZE: usize = 4;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default)]
    pub require_sort: bool,
}

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

    let envs = envy::from_env::<Config>().unwrap();

    if envs.require_sort {
        stream_ordered().await
    } else {
        stream_unordered().await
    }
}

async fn stream_ordered() {
    info!("call stream_ordered");
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let results: Vec<Result<u16>> = tokio_stream::iter(values)
        .map(|v| tokio::spawn(async move { calc(v).await }))
        .buffered(BUFFER_SIZE)
        .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
        .collect()
        .await;

    // the below codes is used try_collect
    // let results: Result<Vec<u16>> = tokio_stream::iter(values)
    //     .map(|v| tokio::spawn(async move { calc(v).await }))
    //     .buffered(BUFFER_SIZE)
    //     .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
    //     .try_collect()
    //     .await;

    println!("{:?}", results);
}

async fn stream_unordered() {
    info!("call stream_unordered");
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let results: Vec<Result<u16>> = tokio_stream::iter(values)
        .map(|v| tokio::spawn(async move { calc(v).await }))
        .buffer_unordered(BUFFER_SIZE)
        .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
        .collect()
        .await;

    // the below codes is used try_collect
    // let results: Result<Vec<u16>> = tokio_stream::iter(values)
    //     .map(|v| tokio::spawn(async move { calc(v).await }))
    //     .buffer_unordered(BUFFER_SIZE)
    //     .map(|res| res.map_err(|e| anyhow!("Task failed: {}", e))?)
    //     .try_collect()
    //     .await;

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
