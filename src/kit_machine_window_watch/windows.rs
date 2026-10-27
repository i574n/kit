#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WindowFine,
    US0_WindowStale(Rc<str>),
}
fn window_health_of_1(mut v0: bool, mut v1: Rc<str>) -> US0 {
    let v3: bool = if v0 {
        let v2: bool = &*v1 != "";
        v2
    } else {
        false
    };
    let v5: bool = if v3 {
        let v4: bool = &*v1 != "running";
        v4
    } else {
        false
    };
    let v7: bool = if v5 {
        let v6: bool = &*v1 != "queued";
        v6
    } else {
        false
    };
    if v7 {
        US0::US0_WindowStale(v1)
    } else {
        US0::US0_WindowFine
    }
}
fn windows_watched_0(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (&*v0).lines().filter(|line| !line.is_empty()).count() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((&*v0).lines().filter(|line| !line.is_empty()).nth(v1 as usize).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let v7: Rc<str> = std::rc::Rc::<str>::from((&*v6).split("(kit job ").nth(1).and_then(|rest| rest.split(char::from(41u8)).next()).unwrap_or(""));
            let v8: bool = &*v7 == "";
            let v11: Rc<str> = if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let v10: Rc<str> = kit_job_control::kit_job_state_word(&*v7);
                v10.clone()
            };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let v14: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
            let v15: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let v16: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(3i32 as usize).unwrap_or(""));
            let v17: Rc<str> = std::rc::Rc::<str>::from((&*v5).split(char::from(9u8)).nth(2i32 as usize).unwrap_or(""));
            let v18: bool = &*v17 == "held";
            let v19: US0 = window_health_of_1(v18, v11.clone());
            let v26: Rc<str> = match &v19 {
                US0::US0_WindowFine => {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                }
                US0::US0_WindowStale(v21) => {
                    let v21: Rc<str> = v21.clone();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- STALE: job is "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v21, &*v23, &*v23, &*v23].concat());
                    v24.clone()
                }
            };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v15, &*v12, &*v16, &*v26].concat());
            let v28: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v13, &*v12, &*v14, &*v27].concat());
            let v29: i32 = v1.wrapping_add(1i32);
            let v30: bool = &*v2 == "";
            let v34: Rc<str> = if v30 {
                v28.clone()
            } else {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v31, &*v28, &*v32, &*v32].concat());
                v33.clone()
            };
            (v0, v1, v2) = (v0.clone(), v29, v34.clone());
            continue;
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        windows_watched_0(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn gate_summary_2() -> Rc<str> {
    let v0: Rc<str> = { let path = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(".claude").join("autonomous").join("gate.log"); let text = std::fs::read_to_string(&path).unwrap_or_default(); let mut sessions: std::collections::BTreeMap<String, (u64, u64)> = std::collections::BTreeMap::new(); let lines = text.lines().collect::<Vec<_>>(); for line in lines.iter().rev().take(4000) { let fields = line.split_whitespace().collect::<Vec<_>>(); if fields.len() < 4 { continue; } let entry = sessions.entry(fields[2].to_owned()).or_insert((0, 0)); if fields[3] == "block" { entry.0 += 1; } else if line.contains("idle") && fields[3] == "pass" { entry.1 += 1; } } std::rc::Rc::<str>::from(sessions.into_iter().map(|(session, (blocks, idle))| format!("  {session}: blocks {blocks}, idle passes {idle}{}", if idle > 0 { "  <- idled" } else { "" })).collect::<Vec<_>>().join("
").as_str()) };
    v0.clone()
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        gate_summary_2()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_machine_windows_lines(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_machine_gate_summary() -> Rc<str> {
    closure1()()
}
