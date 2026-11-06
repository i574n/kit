mod wire;

use futures_util::StreamExt;
use iggy::prelude::*;
use std::str::FromStr;
use wire::{Body, Event};

const STREAM: &str = "kit-m2";

fn event_at(seq: u64) -> Event {
    Event {
        seq,
        machine: "node".to_owned(),
        body: Body::JobStep { job: "m2".to_owned(), step: format!("step-{seq}"), exit: (seq % 3) as i32, detail: "round trip".to_owned() },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count: u64 = std::env::args().nth(1).and_then(|text| text.parse().ok()).unwrap_or(10_000);
    let address = std::env::var("IGGY_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8090".to_owned());
    let username = std::env::var("IGGY_USERNAME").unwrap_or_else(|_| "iggy".to_owned());
    let password = std::env::var("IGGY_PASSWORD").map_err(|_| "set IGGY_PASSWORD (the root password from secret iggy/iggy-root); it is never printed")?;
    let topic = format!("events-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_millis());

    let client = IggyClient::from_connection_string(&format!("iggy://{username}:{password}@{address}"))?;
    client.connect().await?;

    let producer = client
        .producer(STREAM, &topic)?
        .direct(DirectConfig::builder().batch_length(1000).linger_time(IggyDuration::from_str("1ms")?).build())
        .create_stream_if_not_exists()
        .create_topic_if_not_exists(1, IggyExpiry::ServerDefault, MaxTopicSize::ServerDefault)
        .partitioning(Partitioning::balanced())
        .build();
    producer.init().await?;
    let sent_at = std::time::Instant::now();
    let mut batch = Vec::with_capacity(1000);
    for seq in 1..=count {
        batch.push(IggyMessage::builder().payload(bytes::Bytes::from(wire::encode(&event_at(seq)))).build()?);
        if batch.len() == 1000 { producer.send(std::mem::take(&mut batch)).await?; }
    }
    if !batch.is_empty() { producer.send(batch).await?; }
    let send_ms = sent_at.elapsed().as_millis();

    let mut consumer = client
        .consumer("kit-m2-check", STREAM, &topic, 0)?
        .auto_commit(AutoCommit::Disabled)
        .polling_strategy(PollingStrategy::offset(0))
        .poll_interval(IggyDuration::from_str("1ms")?)
        .batch_length(1000)
        .build();
    consumer.init().await?;

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    let (mut received, mut duplicates, mut out_of_order, mut not_identical, mut undecodable) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut seen = vec![false; count as usize + 1];
    let mut last = 0u64;
    while received < count && std::time::Instant::now() < deadline {
        let next = tokio::time::timeout(std::time::Duration::from_secs(10), consumer.next()).await;
        let Ok(Some(message)) = next else { break };
        let message = message?;
        let payload = &message.message.payload;
        match wire::decode(payload) {
            Ok(event) => {
                if wire::encode(&event).as_slice() != payload.as_ref() { not_identical += 1; }
                if event.seq <= last { out_of_order += 1; }
                last = event.seq;
                match seen.get_mut(event.seq as usize) {
                    Some(flag) if *flag => duplicates += 1,
                    Some(flag) => *flag = true,
                    None => undecodable += 1,
                }
                received += 1;
            }
            Err(_) => { undecodable += 1; received += 1; }
        }
    }
    let missing = seen.iter().skip(1).filter(|flag| !**flag).count();
    let ok = received == count && missing == 0 && duplicates == 0 && out_of_order == 0 && not_identical == 0 && undecodable == 0;
    println!(
        "{} sent {count} in {send_ms} ms, received {received}, missing {missing}, duplicates {duplicates}, out_of_order {out_of_order}, not_byte_identical {not_identical}, undecodable {undecodable}, topic {STREAM}/{topic}",
        if ok { "M2-OK" } else { "M2-FAIL" }
    );
    std::process::exit(if ok { 0 } else { 1 });
}
