#[path = "../wire.rs"]
mod wire;

use iggy::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::str::FromStr;
use wire::{Body, Event};

const STREAM: &str = "kit-jobs";

fn verdict(line: &str) -> Option<(String, i32, String)> {
    let text = line.get(9..).unwrap_or(line).trim();
    if text.starts_with("STOP") { return None; }
    let (step, rest) = text.split_once(' ').unwrap_or((text, ""));
    let exit = rest.split_whitespace().skip_while(|word| *word != "exit").nth(1).and_then(|value| value.trim_end_matches(';').parse::<i32>().ok());
    if text.starts_with("notebooks done") || text.starts_with("positive control") || exit.is_some() {
        Some((step.trim_end_matches(':').to_owned(), exit.unwrap_or(0), text.to_owned()))
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let label = args.next().ok_or("usage: publish <job label> <script.ps1> [script args...]")?;
    let script = args.next().ok_or("usage: publish <job label> <script.ps1> [script args...]")?;
    let rest: Vec<String> = args.collect();
    let address = std::env::var("IGGY_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8090".to_owned());
    let username = std::env::var("IGGY_USERNAME").unwrap_or_else(|_| "iggy".to_owned());
    let password = std::env::var("IGGY_PASSWORD").map_err(|_| "set IGGY_PASSWORD; it is never printed")?;
    let machine = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "node".to_owned());

    let client = IggyClient::from_connection_string(&format!("iggy://{username}:{password}@{address}"))?;
    client.connect().await?;
    let producer = client
        .producer(STREAM, &label)?
        .direct(DirectConfig::builder().batch_length(1).linger_time(IggyDuration::from_str("1ms")?).build())
        .create_stream_if_not_exists()
        .create_topic_if_not_exists(1, IggyExpiry::ServerDefault, MaxTopicSize::ServerDefault)
        .partitioning(Partitioning::balanced())
        .build();
    producer.init().await?;

    let mut seq = 0u64;
    let mut publish = |body: Body| {
        seq += 1;
        let bytes = wire::encode(&Event { seq, machine: machine.clone(), body });
        IggyMessage::builder().payload(bytes::Bytes::from(bytes)).build()
    };
    producer.send(vec![publish(Body::JobStarted { job: label.clone(), label: script.clone() })?]).await?;

    let mut child = Command::new("pwsh").arg("-NoProfile").arg("-File").arg(&script).args(&rest).stdout(Stdio::piped()).spawn()?;
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let mut stopped: Option<String> = None;
    for line in BufReader::new(stdout).lines() {
        let line = line?;
        println!("{line}");
        let text = line.get(9..).unwrap_or(&line).trim().to_owned();
        if text.starts_with("STOP") { stopped = Some(text.trim_start_matches("STOP:").trim().to_owned()); continue; }
        if let Some((step, exit, detail)) = verdict(&line) {
            producer.send(vec![publish(Body::JobStep { job: label.clone(), step, exit, detail })?]).await?;
        }
    }
    let status = child.wait()?;
    let last = match stopped {
        Some(reason) => Body::JobStop { job: label.clone(), reason },
        None => Body::JobDone { job: label.clone(), exit: status.code().unwrap_or(-1) },
    };
    producer.send(vec![publish(last)?]).await?;
    std::process::exit(status.code().unwrap_or(1));
}
