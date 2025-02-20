use anyhow::{Error, Result};
use serde::Deserialize;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration as TokioDuration},
};
use tracing::info;

const CHANNEL_SIZE: usize = 4;

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
        mpsc_ordered().await
    } else {
        mpsc_unordered().await
    }
}

async fn mpsc_ordered() {
    info!("call mpsc_ordered");
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE);

    for (i, v) in values.into_iter().enumerate() {
        let tx2 = tx.to_owned();
        tokio::spawn(async move {
            let result = calc(v).await;
            tx2.send((i, result)).await
        });
    }
    drop(tx);

    let mut results = vec![];
    while let Some(result) = rx.recv().await {
        results.push(result);
    }

    results.sort_by_key(|&(index, _)| index);

    println!("{:?}", results);
}

async fn mpsc_unordered() {
    info!("call mpsc_unordered");
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (tx, mut rx) = mpsc::channel(CHANNEL_SIZE);

    for v in values {
        let tx2 = tx.to_owned();
        tokio::spawn(async move {
            let result = calc(v).await;
            tx2.send(result).await
        });
    }
    drop(tx);

    let mut results = vec![];
    while let Some(result) = rx.recv().await {
        results.push(result);
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
