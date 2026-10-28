#[path = "../wire.rs"]
mod wire;

use futures_util::StreamExt;
use iggy::prelude::*;
use std::str::FromStr;
use wire::Body;

const STREAM: &str = "kit-jobs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let label = std::env::args().nth(1).ok_or("usage: follow <job label> [timeout minutes]")?;
    let minutes: u64 = std::env::args().nth(2).and_then(|text| text.parse().ok()).unwrap_or(240);
    let address = std::env::var("IGGY_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8090".to_owned());
    let username = std::env::var("IGGY_USERNAME").unwrap_or_else(|_| "iggy".to_owned());
    let password = std::env::var("IGGY_PASSWORD").map_err(|_| "set IGGY_PASSWORD; it is never printed")?;

    let client = IggyClient::from_connection_string(&format!("iggy://{username}:{password}@{address}"))?;
    client.connect().await?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(minutes * 60);
    let mut consumer = loop {
        let mut candidate = client
            .consumer("kit-follow", STREAM, &label, 0)?
            .auto_commit(AutoCommit::Disabled)
            .polling_strategy(PollingStrategy::offset(0))
            .poll_interval(IggyDuration::from_str("50ms")?)
            .batch_length(100)
            .build();
        match candidate.init().await {
            Ok(()) => break candidate,
            Err(_) if std::time::Instant::now() < deadline => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
            Err(problem) => { println!("topic {STREAM}/{label} never appeared within {minutes} min: {problem}"); std::process::exit(3); }
        }
    };
    let mut last_seq = 0u64;
    while std::time::Instant::now() < deadline {
        let Ok(Some(message)) = tokio::time::timeout(std::time::Duration::from_secs(30), consumer.next()).await else { continue };
        let message = message?;
        let event = match wire::decode(&message.message.payload) {
            Ok(event) => event,
            Err(problem) => { println!("undecodable event at offset {}: {problem:?}", message.message.header.offset); continue; }
        };
        if event.seq <= last_seq { println!("out-of-order seq {} after {last_seq}", event.seq); }
        last_seq = event.seq;
        match event.body {
            Body::JobStarted { job, label } => println!("started {job} ({label}) on {}", event.machine),
            Body::JobStep { step, exit, detail, .. } => println!("step {step} exit {exit}: {detail}"),
            Body::JobDone { job, exit } => { println!("done {job} exit {exit}"); std::process::exit(if exit == 0 { 0 } else { 1 }); }
            Body::JobStop { job, reason } => { println!("stopped {job}: {reason}"); std::process::exit(2); }
            other => println!("event {other:?}"),
        }
    }
    println!("no JobDone or JobStop for {label} within {minutes} min");
    std::process::exit(3);
}
