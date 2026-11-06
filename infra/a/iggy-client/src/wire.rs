#[path = "codec_decode.rs"]
mod decode;
#[path = "codec_encode.rs"]
mod encode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    JobStarted { job: String, label: String },
    JobStep { job: String, step: String, exit: i32, detail: String },
    JobDone { job: String, exit: i32 },
    JobStop { job: String, reason: String },
    LeaseAcquired { tree: String, owner: String },
    LeaseReleased { tree: String, owner: String },
    SlotAdmitted { slot: i32, owner: String },
    SlotReleased { slot: i32, owner: String },
    Health { cpu: i32, free_mem_mb: i32, free_disk_gb: i32 },
    Toolchain { tool: String, version: String, path: String, mtime: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub seq: u64,
    pub machine: String,
    pub body: Body,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    Truncated,
    UnknownTag(u8),
    NotUtf8,
    TrailingBytes(usize),
}

pub fn encode(event: &Event) -> Vec<u8> {
    let (seq, machine) = (event.seq, event.machine.as_str());
    let bytes = match &event.body {
        Body::JobStarted { job, label } => encode::iggy_job_started(seq, machine, job, label),
        Body::JobStep { job, step, exit, detail } => encode::iggy_job_step(seq, machine, job, step, *exit, detail),
        Body::JobDone { job, exit } => encode::iggy_job_done(seq, machine, job, *exit),
        Body::JobStop { job, reason } => encode::iggy_job_stop(seq, machine, job, reason),
        Body::LeaseAcquired { tree, owner } => encode::iggy_lease_acquired(seq, machine, tree, owner),
        Body::LeaseReleased { tree, owner } => encode::iggy_lease_released(seq, machine, tree, owner),
        Body::SlotAdmitted { slot, owner } => encode::iggy_slot_admitted(seq, machine, *slot, owner),
        Body::SlotReleased { slot, owner } => encode::iggy_slot_released(seq, machine, *slot, owner),
        Body::Health { cpu, free_mem_mb, free_disk_gb } => encode::iggy_health(seq, machine, *cpu, *free_mem_mb, *free_disk_gb),
        Body::Toolchain { tool, version, path, mtime } => encode::iggy_toolchain(seq, machine, tool, version, path, mtime),
    };
    bytes.borrow().clone()
}

pub fn decode(bytes: &[u8]) -> Result<Event, DecodeError> {
    let (status, refused, seq, machine, tag, first, second, third, fourth, n1, n2, n3) =
        decode::iggy_decode(Rc::new(RefCell::new(bytes.to_vec())));
    let text = |value: Rc<str>| value.to_string();
    let body = match (status, tag) {
        (1, _) => return Err(DecodeError::Truncated),
        (2, _) => return Err(DecodeError::UnknownTag(refused as u8)),
        (3, _) => return Err(DecodeError::NotUtf8),
        (4, _) => return Err(DecodeError::TrailingBytes(refused as usize)),
        (_, 0) => Body::JobStarted { job: text(first), label: text(second) },
        (_, 1) => Body::JobStep { job: text(first), step: text(second), exit: n1, detail: text(third) },
        (_, 2) => Body::JobDone { job: text(first), exit: n1 },
        (_, 3) => Body::JobStop { job: text(first), reason: text(second) },
        (_, 4) => Body::LeaseAcquired { tree: text(first), owner: text(second) },
        (_, 5) => Body::LeaseReleased { tree: text(first), owner: text(second) },
        (_, 6) => Body::SlotAdmitted { slot: n1, owner: text(first) },
        (_, 7) => Body::SlotReleased { slot: n1, owner: text(first) },
        (_, 8) => Body::Health { cpu: n1, free_mem_mb: n2, free_disk_gb: n3 },
        (_, _) => Body::Toolchain { tool: text(first), version: text(second), path: text(third), mtime: text(fourth) },
    };
    Ok(Event { seq, machine: text(machine), body })
}
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02X}")).collect()
}

#[cfg(test)]
mod fixtures {
    use super::*;

    fn at_node(body: Body) -> Event { Event { seq: 1, machine: "node".into(), body } }

    fn frozen() -> Vec<(Event, &'static str)> {
        let s = |text: &str| text.to_string();
        vec![
            (at_node(Body::JobStarted { job: s("j1"), label: s("build") }), "0000000000000001000000046E6F646500000000026A31000000056275696C64"),
            (at_node(Body::JobStep { job: s("j1"), step: s("compile"), exit: 0, detail: s("ok") }), "0000000000000001000000046E6F646501000000026A3100000007636F6D70696C6500000000000000026F6B"),
            (at_node(Body::JobDone { job: s("j1"), exit: 0 }), "0000000000000001000000046E6F646502000000026A3100000000"),
            (at_node(Body::JobStop { job: s("j1"), reason: s("memory") }), "0000000000000001000000046E6F646503000000026A31000000066D656D6F7279"),
            (at_node(Body::LeaseAcquired { tree: s("eoie"), owner: s("compiler") }), "0000000000000001000000046E6F64650400000004656F696500000008636F6D70696C6572"),
            (at_node(Body::LeaseReleased { tree: s("eoie"), owner: s("compiler") }), "0000000000000001000000046E6F64650500000004656F696500000008636F6D70696C6572"),
            (at_node(Body::SlotAdmitted { slot: 1, owner: s("compiler") }), "0000000000000001000000046E6F6465060000000100000008636F6D70696C6572"),
            (at_node(Body::SlotReleased { slot: 1, owner: s("compiler") }), "0000000000000001000000046E6F6465070000000100000008636F6D70696C6572"),
            (at_node(Body::Health { cpu: 63, free_mem_mb: 7500, free_disk_gb: 84 }), "0000000000000001000000046E6F6465080000003F00001D4C00000054"),
            (at_node(Body::Toolchain { tool: s("perl"), version: s("5.42.3"), path: s("p"), mtime: s("2026-10-26") }), "0000000000000001000000046E6F646509000000047065726C00000006352E34322E3300000001700000000A323032362D31302D3236"),
        ]
    }

    #[test]
    fn every_tag_encodes_to_its_frozen_bytes() {
        for (event, expected) in frozen() { assert_eq!(hex(&encode(&event)), expected, "{event:?}"); }
    }

    #[test]
    fn every_frozen_byte_string_decodes_back_to_its_event() {
        for (event, _) in frozen() { assert_eq!(decode(&encode(&event)), Ok(event)); }
    }

    #[test]
    fn malformed_input_is_refused_not_misread() {
        let bytes = encode(&frozen()[2].0);
        assert_eq!(decode(&bytes[..bytes.len() - 1]), Err(DecodeError::Truncated));
        let mut extra = bytes.clone(); extra.push(0);
        assert_eq!(decode(&extra), Err(DecodeError::TrailingBytes(1)));
        let mut tagged = bytes.clone(); tagged[16] = 42;
        assert_eq!(decode(&tagged), Err(DecodeError::UnknownTag(42)));
    }
}
