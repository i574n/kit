#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_trace_hold<T: Clone>(fresh: &std::rc::Rc<dyn Fn() -> T>) -> T {
    use std::sync::OnceLock;
    static SLOT: OnceLock<usize> = OnceLock::new();
    let raw = *SLOT.get_or_init(|| Box::into_raw(Box::new((*fresh)())) as usize);
    unsafe { (*(raw as *const T)).clone() }
}
#[cfg(target_arch = "wasm32")]
fn spiral_trace_near_log(text: &str) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LOGGED: AtomicUsize = AtomicUsize::new(0);
    let cs: Vec<char> = text.chars().collect();
    for c in cs.chunks(15000) {
        let s: String = c.iter().collect();
        let used = LOGGED.load(Ordering::Relaxed);
        let budget = 16000usize.saturating_sub(used);
        if budget < 13 {
            break;
        }
        let s = if s.len() <= budget {
            s
        } else {
            let mut end = budget - 12;
            while !s.is_char_boundary(end) {
                end -= 1;
            }
            format!("{} [truncated]", &s[..end])
        };
        LOGGED.store(used + s.len(), Ordering::Relaxed);
        near_sdk::env::log_str(&s);
    }
}
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_KeepWaiting,
    US0_StartSupervisor,
    US0_SettleStopped(Rc<str>),
    US0_SettleSkipped(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_JobFinished(i32),
    US1_JobTimedOut,
    US1_JobStopped,
    US1_JobSpawnFailed(Rc<str>),
    US1_JobSkipped(Rc<str>),
    US1_JobLost,
    US1_JobMemoryCapped,
    US1_JobExpiredInQueue,
}
#[derive(Clone)]
enum US3 {
    US3_AfterAny,
    US3_AfterOk,
}
#[derive(Clone)]
enum US4 {
    US4_PredecessorLive,
    US4_PredecessorSettled(Rc<str>, i32),
}
#[derive(Clone)]
enum US2 {
    US2_NoChain,
    US2_Chained(Rc<str>, US3, US4),
}
#[derive(Clone)]
enum US6 {
    US6_JobPending,
    US6_JobRunning,
}
#[derive(Clone)]
enum US5 {
    US5_Live(US6),
    US5_Settled(US1),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US7 {
    US7_Ok,
    US7_Error(Rc<str>),
}
struct Mut1 { l0: i64 }
struct Mut2 { l0: Rc<dyn Fn(Rc<str>) -> ()> }
struct Mut3 { l0: bool }
#[derive(Clone)]
enum US8 {
    US8_Verbose,
    US8_Debug,
    US8_Info,
    US8_Warning,
    US8_Critical,
}
struct Mut4 { l0: US8 }
#[derive(Clone)]
enum US9 {
    US9_Some(US8),
    US9_None,
}
struct Mut5 { l0: i32, l1: US9 }
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>),
    US10_None,
}
struct Mut6 { l0: i32 }
struct Mut7 { l0: i32 }
#[derive(Clone)]
enum US11 {
    US11_HomePresent,
    US11_HomeGone,
}
#[derive(Clone)]
enum US12 {
    US12_KeepServing,
    US12_HostIdleExit,
}
#[derive(Clone)]
enum US13 {
    US13_Some(u64),
    US13_None,
}
#[derive(Clone)]
enum US14 {
    US14_HostOwns,
    US14_OtherHostServes(u64),
}
#[derive(Clone)]
enum US15 {
    US15_HostMissing,
    US15_HostRunning(u64),
}
#[derive(Clone)]
enum US16 {
    US16_SupervisorUp(u64),
    US16_SupervisorHeld(Rc<str>),
    US16_SupervisorLate,
}
#[derive(Clone)]
enum US17 {
    US17_Some(US16),
    US17_None,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn host_home_directory_4() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn requests_directory_2() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn next_request_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".start")).min().unwrap_or_default());
    v1.clone()
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn request_removed_8(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn request_id_9(mut v0: Rc<str>) -> Rc<str> {
    loop_10(v0.clone())
}
fn job_directory_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1.clone(), v0.clone())
}
fn job_file_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_15(v0.clone());
    joined_3(v2.clone(), v1.clone())
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn held_beat_13(mut v0: Rc<str>) -> () {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: u64 = now_ms_16();
    let mut v4: Rc<str> = decimal_text_17(v3);
    let mut v5: Rc<str> = text_written_18(v2.clone(), v4.clone());
    ()
}
fn held_directory_19() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn held_file_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = held_directory_19();
    joined_3(v1.clone(), v0.clone())
}
fn held_recorded_12(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = held_directory_19();
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v2).err().map(|error| format!("create {}: {error}", &*v2)).unwrap_or_default());
    let mut v4: Rc<str> = held_file_20(v0.clone());
    let mut v5: Rc<str> = text_written_18(v4.clone(), v1.clone());
    held_beat_13(v0.clone())
}
fn settled_word_22(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US1::US1_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US1::US1_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US1::US1_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn settled_in_queue_21(mut v0: Rc<str>, mut v1: US0) -> () {
    match &v1 {
        US0::US0_SettleSkipped(v16) => {
            let mut v16: Rc<str> = v16.clone();
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = job_file_14(v0.clone(), v17.clone());
            let mut v19: u64 = now_ms_16();
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: US1 = US1::US1_JobSkipped(v20.clone());
            let mut v22: Rc<str> = settled_word_22(v21.clone());
            let mut v23: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let mut v24: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v16, &*v26, &*v20].concat());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v25, &*v24, &*v25, &*v27].concat());
            let mut v29: Rc<str> = text_written_18(v18.clone(), v28.clone());
            ()
        }
        US0::US0_SettleStopped(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = job_file_14(v0.clone(), v3.clone());
            let mut v5: u64 = now_ms_16();
            let mut v6: US1 = US1::US1_JobStopped;
            let mut v7: Rc<str> = settled_word_22(v6.clone());
            let mut v8: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v10, &*v2, &*v11, &*v12].concat());
            let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v10, &*v9, &*v10, &*v13].concat());
            let mut v15: Rc<str> = text_written_18(v4.clone(), v14.clone());
            ()
        }
        _ => {
            ()
        }
    }
}
fn file_exists_24(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn method25(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn state_of_parts_28(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US5 {
    let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: US6 = US6::US6_JobPending;
        US5::US5_Live(v4.clone())
    } else {
        let mut v6: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: US6 = US6::US6_JobRunning;
            US5::US5_Live(v7.clone())
        } else {
            let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: US1 = US1::US1_JobFinished(v1);
                US5::US5_Settled(v10.clone())
            } else {
                let mut v12: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                if v12 {
                    let mut v13: US1 = US1::US1_JobTimedOut;
                    US5::US5_Settled(v13.clone())
                } else {
                    let mut v15: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    if v15 {
                        let mut v16: US1 = US1::US1_JobStopped;
                        US5::US5_Settled(v16.clone())
                    } else {
                        let mut v18: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                        if v18 {
                            let mut v19: US1 = US1::US1_JobLost;
                            US5::US5_Settled(v19.clone())
                        } else {
                            let mut v21: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                            if v21 {
                                let mut v22: US1 = US1::US1_JobMemoryCapped;
                                US5::US5_Settled(v22.clone())
                            } else {
                                let mut v24: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                                if v24 {
                                    let mut v25: US1 = US1::US1_JobExpiredInQueue;
                                    US5::US5_Settled(v25.clone())
                                } else {
                                    let mut v27: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                    if v27 {
                                        let mut v28: US1 = US1::US1_JobSkipped(v2.clone());
                                        US5::US5_Settled(v28.clone())
                                    } else {
                                        let mut v30: US1 = US1::US1_JobSpawnFailed(v2.clone());
                                        US5::US5_Settled(v30.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn shared_state_27(mut v0: Rc<str>) -> US5 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = eoie_job_state::eoie_job_state_parts(&*v0);
    state_of_parts_28(v1.clone(), v2, v3.clone())
}
fn state_settled_29(mut v0: US5) -> bool {
    match &v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1.clone();
            false
        }
        US5::US5_Settled(v2) => {
            let mut v2: US1 = v2.clone();
            true
        }
    }
}
fn live_word_31(mut v0: US6) -> Rc<str> {
    match &v0 {
        US6::US6_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US6::US6_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_30(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1.clone();
            live_word_31(v1.clone())
        }
        US5::US5_Settled(v3) => {
            let mut v3: US1 = v3.clone();
            settled_word_22(v3.clone())
        }
    }
}
fn settled_exit_33(mut v0: US1) -> i32 {
    match &v0 {
        US1::US1_JobExpiredInQueue => {
            75i32
        }
        US1::US1_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            v1
        }
        US1::US1_JobLost => {
            125i32
        }
        US1::US1_JobMemoryCapped => {
            137i32
        }
        US1::US1_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US1::US1_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2.clone();
            125i32
        }
        US1::US1_JobStopped => {
            143i32
        }
        US1::US1_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_32(mut v0: US5) -> i32 {
    match &v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1.clone();
            3i32
        }
        US5::US5_Settled(v2) => {
            let mut v2: US1 = v2.clone();
            settled_exit_33(v2.clone())
        }
    }
}
fn queued_of_23(mut v0: Rc<str>) -> (Rc<str>, bool, US2) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_7(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_14(v0.clone(), v4.clone());
    let mut v6: bool = file_exists_24(v5.clone());
    let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v50, mut v51, mut v52): (Rc<str>, bool, US2) = if v7 {
        let mut v8: US2 = US2::US2_NoChain;
        (v0.clone(), v6, v8.clone())
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v11: i32 = (v10.clone().borrow().len() as i32);
        let mut v12: bool = 0i32 < v11;
        let mut v23: Rc<str> = if v12 {
            let mut v13: Rc<str> = v10.clone().borrow()[0i32 as usize].clone();
            let mut v14: i32 = (v13.clone().len() as i32);
            let mut v15: i32 = 0i32;
            let mut v16: i32 = method25(v13.clone(), v14, v15);
            let mut v17: i32 = v14.wrapping_sub(1i32);
            let mut v18: Rc<str> = string_slice(&v13.clone(), v16 as i64, v17 as i64);
            let mut v19: i32 = (v18.clone().len() as i32);
            let mut v20: i32 = method26(v18.clone(), v19);
            let mut v21: Rc<str> = string_slice(&v18.clone(), 0i32 as i64, v20 as i64);
            v21.clone()
        } else {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v22.clone()
        };
        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v25: i32 = (v24.clone().borrow().len() as i32);
        let mut v26: bool = 1i32 < v25;
        let mut v37: Rc<str> = if v26 {
            let mut v27: Rc<str> = v24.clone().borrow()[1i32 as usize].clone();
            let mut v28: i32 = (v27.clone().len() as i32);
            let mut v29: i32 = 0i32;
            let mut v30: i32 = method25(v27.clone(), v28, v29);
            let mut v31: i32 = v28.wrapping_sub(1i32);
            let mut v32: Rc<str> = string_slice(&v27.clone(), v30 as i64, v31 as i64);
            let mut v33: i32 = (v32.clone().len() as i32);
            let mut v34: i32 = method26(v32.clone(), v33);
            let mut v35: Rc<str> = string_slice(&v32.clone(), 0i32 as i64, v34 as i64);
            v35.clone()
        } else {
            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v36.clone()
        };
        let mut v38: bool = v37.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        let mut v41: US3 = if v38 {
            US3::US3_AfterOk
        } else {
            US3::US3_AfterAny
        };
        let mut v42: US5 = shared_state_27(v23.clone());
        let mut v43: bool = state_settled_29(v42.clone());
        let mut v48: US4 = if v43 {
            let mut v44: Rc<str> = state_word_30(v42.clone());
            let mut v45: i32 = state_exit_32(v42.clone());
            US4::US4_PredecessorSettled(v44.clone(), v45)
        } else {
            US4::US4_PredecessorLive
        };
        let mut v49: US2 = US2::US2_Chained(v23.clone(), v41.clone(), v48.clone());
        (v0.clone(), v6, v49.clone())
    };
    (v50.clone(), v51, v52.clone())
}
fn method35(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_34(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn supervisor_child_started_36(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = { let mut command = std::process::Command::new(&*v1); command.arg("job").arg("supervise").arg(&*v2).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; const DETACHED_PROCESS: u32 = 0x0000_0008; command.creation_flags(DETACHED_PROCESS); } match command.spawn() { Ok(child) => { (v0).borrow_mut().push(child); true } Err(_) => false } };
    v3
}
fn request_served_11(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    let (mut v3, mut v4, mut v5): (Rc<str>, bool, US2) = queued_of_23(v2.clone());
    let mut v42: US0 = if v4 {
        let mut v11: Rc<str> = match &v5 {
            US2::US2_Chained(v7, v8, v9) => {
                let mut v7: Rc<str> = v7.clone();
                let mut v8: US3 = v8.clone();
                let mut v9: US4 = v9.clone();
                v7.clone()
            }
            US2::US2_NoChain => {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v6.clone()
            }
        };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
        US0::US0_SettleStopped(v13.clone())
    } else {
        match &v5 {
            US2::US2_Chained(v16, v17, v18) => {
                let mut v16: Rc<str> = v16.clone();
                let mut v17: US3 = v17.clone();
                let mut v18: US4 = v18.clone();
                match &v18 {
                    US4::US4_PredecessorLive => {
                        US0::US0_KeepWaiting
                    }
                    _ => {
                        match &v17 {
                            US3::US3_AfterAny => {
                                US0::US0_StartSupervisor
                            }
                            US3::US3_AfterOk => {
                                match &v18 {
                                    US4::US4_PredecessorSettled(v21, v22) => {
                                        let mut v21: Rc<str> = v21.clone();
                                        let mut v22: i32 = *v22;
                                        let mut v23: bool = v22 == 0i32;
                                        if v23 {
                                            US0::US0_StartSupervisor
                                        } else {
                                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) };
                                            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v16));
                                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) };
                                            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                                            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v21));
                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) };
                                            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                                            let mut v32: Rc<str> = format_real_34(v22);
                                            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                                            US0::US0_SettleSkipped(v33.clone())
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                }
            }
            US2::US2_NoChain => {
                US0::US0_StartSupervisor
            }
        }
    };
    match &v42 {
        US0::US0_KeepWaiting => {
            held_recorded_12(v2.clone(), v1.clone())
        }
        US0::US0_StartSupervisor => {
            let mut v43: bool = supervisor_child_started_36(v0.clone(), v1.clone(), v2.clone());
            ()
        }
        _ => {
            settled_in_queue_21(v2.clone(), v42.clone())
        }
    }
}
fn live_children_37(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let mut v1: u64 = { let mut kids = (v0).borrow_mut(); kids.retain_mut(|child| matches!(child.try_wait(), Ok(None))); kids.len() as u64 };
    v1
}
fn file_exists_41(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure1() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method42() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure1()
}
fn ok_c2() -> Rc<dyn Fn() -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US7> = Rc::new(move || -> US7 {
        US7::US7_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method43() -> Rc<dyn Fn() -> US7> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(Rc<str>) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US7> = Rc::new(move |mut v0: Rc<str>| -> US7 {
        US7::US7_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method44() -> Rc<dyn Fn(Rc<str>) -> US7> {
    error_c3()
}
fn get_environment_variable_46(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1.clone()
}
fn method47(mut v0: Rc<RefCell<Mut5>>) -> bool {
    let mut v1: i32 = v0.borrow().l0.clone();
    let mut v2: bool = v1 > 0i32;
    v2
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn new_trace_state_45(mut v0: US8) -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TRACE_LEVEL"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_46(v1.clone());
    ;
    ;
    ;
    ;
    ;
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Critical"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Warning"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v5.to_lowercase());
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Info"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Debug"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from(v9.to_lowercase());
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Verbose"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
    let mut v13: Rc<RefCell<Vec<(Rc<str>, US8)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v14: US8 = US8::US8_Verbose;
    v13.borrow_mut().push((v11.clone(), v14.clone()));
    let mut v15: US8 = US8::US8_Debug;
    v13.borrow_mut().push((v9.clone(), v15.clone()));
    let mut v16: US8 = US8::US8_Info;
    v13.borrow_mut().push((v7.clone(), v16.clone()));
    let mut v17: US8 = US8::US8_Warning;
    v13.borrow_mut().push((v5.clone(), v17.clone()));
    let mut v18: US8 = US8::US8_Critical;
    v13.borrow_mut().push((v3.clone(), v18.clone()));
    let mut v19: US8 = US8::US8_Verbose;
    v13.borrow_mut().push((v12.clone(), v19.clone()));
    let mut v20: US8 = US8::US8_Debug;
    v13.borrow_mut().push((v10.clone(), v20.clone()));
    let mut v21: US8 = US8::US8_Info;
    v13.borrow_mut().push((v8.clone(), v21.clone()));
    let mut v22: US8 = US8::US8_Warning;
    v13.borrow_mut().push((v6.clone(), v22.clone()));
    let mut v23: US8 = US8::US8_Critical;
    v13.borrow_mut().push((v4.clone(), v23.clone()));
    let mut v24: Rc<Vec<(Rc<str>, US8)>> = Rc::new(v13.borrow().clone());
    let mut v25: Rc<RefCell<Vec<(Rc<str>, US8)>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
    let mut v26: i32 = (v25.clone().borrow().len() as i32);
    let mut v27: US9 = US9::US9_None;
    let mut v28: Rc<RefCell<Mut5>> = Rc::new(RefCell::new(Mut5 { l0: v26, l1: v27.clone() }));
    while method47(v28.clone()) {
        let mut v30: i32 = v28.borrow().l0.clone();
        let mut v31: i32 = v30.wrapping_sub(1i32);
        let mut v32: US9 = v28.borrow().l1.clone();
        let (mut v33, mut v34): (Rc<str>, US8) = v25.clone().borrow()[v31 as usize].clone();
        let mut v41: US9 = match &v32 {
            US9::US9_None => {
                let mut v36: bool = v33 == v2 ;
                if v36 {
                    US9::US9_Some(v34.clone())
                } else {
                    US9::US9_None
                }
            }
            US9::US9_Some(v35) => {
                let mut v35: US8 = v35.clone();
                v32.clone()
            }
        };
        v28.borrow_mut().l0 = v31;
        v28.borrow_mut().l1 = v41.clone();
        ()
    };
    let mut v42: US9 = v28.borrow().l1.clone();
    let mut v43: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i64 }));
    let mut v44: Rc<dyn Fn(Rc<str>) -> ()> = closure5();
    let mut v45: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v44.clone() }));
    let mut v46: Rc<RefCell<Mut3>> = Rc::new(RefCell::new(Mut3 { l0: true }));
    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v48: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v47.clone() }));
    let mut v51: US8 = match &v42 {
        US9::US9_None => {
            v0.clone()
        }
        US9::US9_Some(v49) => {
            let mut v49: US8 = v49.clone();
            v49.clone()
        }
    };
    let mut v52: Rc<RefCell<Mut4>> = Rc::new(RefCell::new(Mut4 { l0: v51.clone() }));
    let mut v53: Option<i64> = None;
    (v43.clone(), v45.clone(), v46.clone(), v48.clone(), v52.clone(), v53.clone())
}
fn fresh_c4() -> Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US8 = US8::US8_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_45(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fresh_c6() -> Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> = Rc::new(move || -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) {
        let mut v0: US8 = US8::US8_Verbose;
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) = new_trace_state_45(v0.clone());
        (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method48(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<RefCell<Mut2>>, mut v2: Rc<RefCell<Mut3>>, mut v3: Rc<RefCell<Mut0>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>) -> Rc<str> {
    let mut v6: u64 = { #[cfg(target_arch = "wasm32")] let (h, m, s) = { let secs = near_sdk::env::block_timestamp() / 1_000_000_000; ((secs / 3600) % 24, (secs / 60) % 60, secs % 60) }; #[cfg(all(windows, not(target_arch = "wasm32")))] let (h, m, s) = { #[repr(C)] struct St([u16; 8]); unsafe extern "system" { fn GetLocalTime(t: *mut St); } let mut t = St([0; 8]); unsafe { GetLocalTime(&mut t) }; (t.0[4] as u64, t.0[5] as u64, t.0[6] as u64) }; #[cfg(all(unix, not(target_arch = "wasm32")))] let (h, m, s) = { #[repr(C, align(8))] struct Tm([i32; 16]); unsafe extern "C" { fn localtime_r(t: *const std::os::raw::c_long, tm: *mut Tm) -> *mut Tm; } let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as std::os::raw::c_long; let mut tm = Tm([0; 16]); unsafe { localtime_r(&secs, &mut tm) }; (tm.0[2] as u64, tm.0[1] as u64, tm.0[0] as u64) }; #[cfg(not(any(windows, unix, target_arch = "wasm32")))] let (h, m, s) = { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0); ((secs / 3600) % 24, (secs / 60) % 60, secs % 60) }; h * 3600 + m * 60 + s };
    let mut v7: u64 = v6.wrapping_div(3600u64);
    let mut v8: bool = v7 < 10u64;
    let mut v11: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    };
    let mut v12: Rc<str> = Rc::<str>::from(format!("{:?}", v7));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: u64 = v6.wrapping_div(60u64);
    let mut v17: u64 = v16.wrapping_rem(60u64);
    let mut v18: bool = v17 < 10u64;
    let mut v21: Rc<str> = if v18 {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    } else {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20.clone()
    };
    let mut v22: Rc<str> = Rc::<str>::from(format!("{:?}", v17));
    let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v23));
    let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v14));
    let mut v26: u64 = v6.wrapping_rem(60u64);
    let mut v27: bool = v26 < 10u64;
    let mut v30: Rc<str> = if v27 {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    } else {
        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v29.clone()
    };
    let mut v31: Rc<str> = Rc::<str>::from(format!("{:?}", v26));
    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v32));
    v33.clone()
}
fn format_real_50(mut v0: u8) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0 as char));
    method35(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method49() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[91m"); } LIT.with(|lit| lit.clone()) };
    ;
    ;
    ;
    ;
    ;
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Critical"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let mut v3: u8 = v2.clone().as_bytes()[0i32 as usize];
    let mut v4: Rc<str> = format_real_50(v3);
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[0m"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    v7.clone()
}
fn method53(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v7: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'/';
                v6
            };
            if v7 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method52(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method25(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method53(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    v8.clone()
}
fn format_real_54(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method35(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method51(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<RefCell<Mut2>>, mut v2: Rc<RefCell<Mut3>>, mut v3: Rc<RefCell<Mut0>>, mut v4: Rc<RefCell<Mut4>>, mut v5: Option<i64>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: i64 = v0.borrow().l0.clone();
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v10));
    let mut v12: Rc<str> = format_real_54(v9);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v7));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v10));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" / "); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error'"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error'"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = "); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error' = "); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v8));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ "); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" }"); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v30));
    method52(v31.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        println!("{}", v0);
        ()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn held_released_40(mut v0: Rc<str>) -> () {
    let mut v1: Rc<str> = held_file_20(v0.clone());
    let mut v2: bool = file_exists_41(v1.clone());
    let mut v6: Result<(), std::io::Error> = if v2 {
        let mut v4: Result<(), std::io::Error> = std::fs::remove_file(&*v1);
        v4
    } else {
        let mut v5: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
        v5
    };
    let mut v7: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method42();
    let mut v9: Result<(), Rc<str>> = v6.map_err(|x| v7(x));
    let mut v10: Rc<dyn Fn() -> US7> = method43();
    let mut v11: Rc<dyn Fn(Rc<str>) -> US7> = method44();
    let mut v12: US7 = match v9 { Ok(()) => v10(), Err(e) => v11(e) };
    match &v12 {
        US7::US7_Error(v13) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v15: Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c4();
            { let _ = spiral_trace_hold(&v15); };
            let mut v17: Rc<dyn Fn() -> (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>)> = fresh_c6();
            let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v17) };
            let mut v24: US8 = v22.borrow().l0.clone();
            let mut v29: i32 = match &v24 {
                US8::US8_Critical => {
                    50i32
                }
                US8::US8_Debug => {
                    20i32
                }
                US8::US8_Info => {
                    30i32
                }
                US8::US8_Verbose => {
                    10i32
                }
                US8::US8_Warning => {
                    40i32
                }
            };
            let mut v30: bool = v20.borrow().l0.clone();
            let mut v31: bool = v30 == false;
            let mut v33: bool = if v31 {
                false
            } else {
                let mut v32: bool = 50i32 >= v29;
                v32
            };
            let mut v34: bool = v33 == false;
            let mut v82: US10 = if v34 {
                US10::US10_None
            } else {
                { let _ = spiral_trace_hold(&v15); };
                let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v17) };
                let mut v44: Rc<str> = method48(v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone());
                let mut v45: Rc<str> = method49();
                let mut v46: Rc<str> = method51(v38.clone(), v39.clone(), v40.clone(), v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v13.clone());
                { let _ = spiral_trace_hold(&v15); };
                let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<RefCell<Mut1>>, Rc<RefCell<Mut2>>, Rc<RefCell<Mut3>>, Rc<RefCell<Mut0>>, Rc<RefCell<Mut4>>, Option<i64>) = { spiral_trace_hold(&v17) };
                let mut v55: i64 = v49.borrow().l0.clone();
                let mut v56: i64 = v55.wrapping_add(1i64);
                v49.borrow_mut().l0 = v56;
                let mut v57: Rc<dyn Fn(Rc<str>) -> ()> = closure7();
                let mut v58: bool = cfg!(target_arch = "wasm32");
                if v58 {
                    let mut v59: Rc<str> = v52.borrow().l0.clone();
                    let mut v60: bool = v59.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v68: Rc<str> = if v60 {
                        v46.clone()
                    } else {
                        let mut v61: bool = v46.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        if v61 {
                            let mut v62: Rc<str> = v52.borrow().l0.clone();
                            v62.clone()
                        } else {
                            let mut v63: Rc<str> = v52.borrow().l0.clone();
                            let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                            let mut v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v46));
                            v66.clone()
                        }
                    };
                    let mut v70: usize = v68.chars().count();
                    let mut v71: i32 = ((v70) as i32);
                    let mut v72: i32 = v71.wrapping_add(14999i32);
                    let mut v73: i32 = v72.wrapping_div(15000i32);
                    let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v75: bool = v46 != v74 ;
                    let mut v77: bool = if v75 {
                        let mut v76: bool = v73 <= 1i32;
                        v76
                    } else {
                        false
                    };
                    if v77 {
                        v52.borrow_mut().l0 = v68.clone();
                        ()
                    } else {
                        v52.borrow_mut().l0 = v74.clone();
                        { #[cfg(target_arch = "wasm32")] spiral_trace_near_log(&v68); };
                        ()
                    }
                } else {
                    println!("{}", v46);
                    ()
                };
                let mut v80: Rc<dyn Fn(Rc<str>) -> ()> = v50.borrow().l0.clone();
                v80(v46.clone());
                US10::US10_Some(v49.clone(), v50.clone(), v51.clone(), v52.clone(), v53.clone(), v54.clone())
            };
            ()
        }
        US7::US7_Ok => {
            ()
        }
    }
}
fn held_from_39(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: u64) -> u64 {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let mut v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let (mut v7, mut v8, mut v9): (Rc<str>, bool, US2) = queued_of_23(v6.clone());
            let mut v46: US0 = if v8 {
                let mut v15: Rc<str> = match &v9 {
                    US2::US2_Chained(v11, v12, v13) => {
                        let mut v11: Rc<str> = v11.clone();
                        let mut v12: US3 = v12.clone();
                        let mut v13: US4 = v13.clone();
                        v11.clone()
                    }
                    US2::US2_NoChain => {
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v10.clone()
                    }
                };
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
                US0::US0_SettleStopped(v17.clone())
            } else {
                match &v9 {
                    US2::US2_Chained(v20, v21, v22) => {
                        let mut v20: Rc<str> = v20.clone();
                        let mut v21: US3 = v21.clone();
                        let mut v22: US4 = v22.clone();
                        match &v22 {
                            US4::US4_PredecessorLive => {
                                US0::US0_KeepWaiting
                            }
                            _ => {
                                match &v21 {
                                    US3::US3_AfterAny => {
                                        US0::US0_StartSupervisor
                                    }
                                    US3::US3_AfterOk => {
                                        match &v22 {
                                            US4::US4_PredecessorSettled(v25, v26) => {
                                                let mut v25: Rc<str> = v25.clone();
                                                let mut v26: i32 = *v26;
                                                let mut v27: bool = v26 == 0i32;
                                                if v27 {
                                                    US0::US0_StartSupervisor
                                                } else {
                                                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v20));
                                                    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                                                    let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v25));
                                                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                                                    let mut v36: Rc<str> = format_real_34(v26);
                                                    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                                                    US0::US0_SettleSkipped(v37.clone())
                                                }
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                    US2::US2_NoChain => {
                        US0::US0_StartSupervisor
                    }
                }
            };
            match &v46 {
                US0::US0_KeepWaiting => {
                    held_beat_13(v6.clone());
                    let mut v47: i32 = v2.wrapping_add(1i32);
                    let mut v48: u64 = v3.wrapping_add(1u64);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v47, v48);
                    continue;
                }
                US0::US0_StartSupervisor => {
                    let mut v50: Rc<str> = held_file_20(v6.clone());
                    let mut v51: Rc<str> = text_or_empty_7(v50.clone());
                    held_released_40(v6.clone());
                    let mut v52: bool = supervisor_child_started_36(v0.clone(), v51.clone(), v6.clone());
                    let mut v53: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v53, v3);
                    continue;
                }
                _ => {
                    settled_in_queue_21(v6.clone(), v46.clone());
                    held_released_40(v6.clone());
                    let mut v55: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v55, v3);
                    continue;
                }
            }
        }
    }
}
fn directory_exists_56(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method57(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0.clone()
}
fn method58(mut v0: i32, mut v1: Rc<RefCell<Mut6>>) -> bool {
    let mut v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn closure8() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method59() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure8()
}
fn method60(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut7>>) -> bool {
    let mut v3: i32 = v2.borrow().l0.clone();
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0.clone();
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.clone().borrow()[v6 as usize].clone();
        let mut v8: bool = v7.clone() < v1.clone();
        let mut v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = v7.clone() > v1.clone();
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let mut v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn held_names_55() -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v0: Rc<str> = held_directory_19();
    let mut v1: bool = directory_exists_56(v0.clone());
    if v1 {
        let mut v2: Rc<str> = held_directory_19();
        let mut v4: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Vec<std::string::String> = v4.unwrap();
        let mut v7: Vec<std::string::String> = method57(v6.clone());
        let mut v8: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v7));
        let mut v9: i32 = (v8.clone().borrow().len() as i32);
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v9 as usize]));
        let mut v11: Rc<RefCell<Mut6>> = Rc::new(RefCell::new(Mut6 { l0: 0i32 }));
        while method58(v9, v11.clone()) {
            let mut v13: i32 = v11.borrow().l0.clone();
            let mut v14: std::string::String = v8.clone().borrow()[v13 as usize].clone();
            let mut v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            v10.clone().borrow_mut()[v13 as usize] = v16.clone();
            let mut v17: i32 = v13.wrapping_add(1i32);
            v11.borrow_mut().l0 = v17;
            ()
        };
        let mut v18: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
        let mut v19: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method59();
        let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().clone()));
        let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().iter().map(|x| v19(x.clone())).collect::<Vec<_>>()));
        let mut v22: i32 = (v18.clone().borrow().len() as i32);
        let mut v23: Rc<RefCell<Mut6>> = Rc::new(RefCell::new(Mut6 { l0: 1i32 }));
        while method58(v22, v23.clone()) {
            let mut v25: i32 = v23.borrow().l0.clone();
            let mut v26: Rc<str> = v20.clone().borrow()[v25 as usize].clone();
            let mut v27: Rc<str> = v21.clone().borrow()[v25 as usize].clone();
            let mut v28: Rc<RefCell<Mut7>> = Rc::new(RefCell::new(Mut7 { l0: v25 }));
            while method60(v21.clone(), v27.clone(), v28.clone()) {
                let mut v30: i32 = v28.borrow().l0.clone();
                let mut v31: i32 = v30.wrapping_sub(1i32);
                let mut v32: Rc<str> = v20.clone().borrow()[v31 as usize].clone();
                v20.clone().borrow_mut()[v30 as usize] = v32.clone();
                let mut v33: Rc<str> = v21.clone().borrow()[v31 as usize].clone();
                v21.clone().borrow_mut()[v30 as usize] = v33.clone();
                v28.borrow_mut().l0 = v31;
                ()
            };
            let mut v34: i32 = v28.borrow().l0.clone();
            v20.clone().borrow_mut()[v34 as usize] = v26.clone();
            let mut v35: i32 = v28.borrow().l0.clone();
            v21.clone().borrow_mut()[v35 as usize] = v27.clone();
            let mut v36: i32 = v25.wrapping_add(1i32);
            v23.borrow_mut().l0 = v36;
            ()
        };
        let mut v37: Rc<RefCell<Vec<Rc<str>>>> = v20 ;
        v37.clone()
    } else {
        let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v38.clone()
    }
}
fn held_served_38(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = held_names_55();
    let mut v2: i32 = 0i32;
    let mut v3: u64 = 0u64;
    held_from_39(v0.clone(), v1.clone(), v2, v3)
}
fn directory_exists_62(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn home_of_61() -> US11 {
    let mut v0: Rc<str> = requests_directory_2();
    let mut v1: bool = directory_exists_62(v0.clone());
    if v1 {
        US11::US11_HomePresent
    } else {
        US11::US11_HomeGone
    }
}
fn host_step_of_63(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: US11) -> US12 {
    let mut v4: bool = v0 > 0u64;
    if v4 {
        US12::US12_KeepServing
    } else {
        let mut v6: bool = match &v3 {
            US11::US11_HomeGone => {
                true
            }
            _ => {
                false
            }
        };
        if v6 {
            US12::US12_HostIdleExit
        } else {
            let mut v8: u64 = v2.wrapping_sub(v1);
            let mut v9: bool = v8 >= 600000u64;
            if v9 {
                US12::US12_HostIdleExit
            } else {
                US12::US12_KeepServing
            }
        }
    }
}
fn host_pid_file_64() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn method66(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure9() -> Rc<dyn Fn((u64)) -> US13> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US13> = Rc::new(move |mut v0: (u64)| -> US13 {
        let mut v1: u64 = (v0);
        US13::US13_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_65(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method66(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US13> = closure9();
    let mut v4: Option<US13> = v2.map(|x| v3(x));
    let mut v6: US13 = US13::US13_None;
    let mut v7: US13 = v6.clone();
    let mut v8: US13 = v4.unwrap_or(v7);
    match &v8 {
        US13::US13_None => {
            0u64
        }
        US13::US13_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn own_pid_67() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn host_served_1(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: u64) -> i32 {
    loop {
        let mut v2: Rc<str> = requests_directory_2();
        let mut v3: Rc<str> = next_request_6(v2.clone());
        let mut v4: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: Rc<str> = joined_3(v2.clone(), v3.clone());
            let mut v6: Rc<str> = text_or_empty_7(v5.clone());
            let mut v7: bool = request_removed_8(v5.clone());
            let mut v8: Rc<str> = request_id_9(v3.clone());
            request_served_11(v0.clone(), v6.clone(), v8.clone());
            let mut v9: u64 = now_ms_16();
            (v0, v1) = (v0.clone(), v9);
            continue;
        } else {
            let mut v11: u64 = live_children_37(v0.clone());
            let mut v12: u64 = held_served_38(v0.clone());
            let mut v13: u64 = v11.wrapping_add(v12);
            let mut v14: u64 = now_ms_16();
            let mut v15: US11 = home_of_61();
            let mut v16: US12 = host_step_of_63(v13, v1, v14, v15.clone());
            match &v16 {
                US12::US12_HostIdleExit => {
                    let mut v17: Rc<str> = host_pid_file_64();
                    let mut v18: Rc<str> = text_or_empty_7(v17.clone());
                    let mut v19: u64 = decimal_u64_65(v18.clone());
                    let mut v20: u64 = own_pid_67();
                    let mut v21: bool = v19 == v20;
                    let mut v24: bool = if v21 {
                        let mut v22: Rc<str> = host_pid_file_64();
                        request_removed_8(v22.clone())
                    } else {
                        false
                    };
                    return 0i32;
                }
                US12::US12_KeepServing => {
                    let mut v25: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v25));
                    let mut v26: bool = v13 > 0u64;
                    let mut v27: u64 = if v26 {
                        v14
                    } else {
                        v1
                    };
                    (v0, v1) = (v0.clone(), v27);
                    continue;
                }
            }
        }
    }
}
fn process_alive_68(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn host_claim_of_69(mut v0: u64, mut v1: u64, mut v2: bool) -> US14 {
    let mut v3: bool = v0 != 0u64;
    let mut v5: bool = if v3 {
        let mut v4: bool = v0 != v1;
        v4
    } else {
        false
    };
    let mut v6: bool = v5 && v2;
    if v6 {
        US14::US14_OtherHostServes(v0)
    } else {
        US14::US14_HostOwns
    }
}
fn no_children_70() -> std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> {
    let mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    v0.clone()
}
fn host_run_0() -> i32 {
    let mut v0: Rc<str> = host_pid_file_64();
    let mut v1: Rc<str> = text_or_empty_7(v0.clone());
    let mut v2: u64 = decimal_u64_65(v1.clone());
    let mut v3: u64 = own_pid_67();
    let mut v4: bool = process_alive_68(v2);
    let mut v5: US14 = host_claim_of_69(v2, v3, v4);
    match &v5 {
        US14::US14_HostOwns => {
            let mut v7: Rc<str> = requests_directory_2();
            let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v7).err().map(|error| format!("create {}: {error}", &*v7)).unwrap_or_default());
            let mut v9: Rc<str> = host_pid_file_64();
            let mut v10: u64 = own_pid_67();
            let mut v11: Rc<str> = decimal_text_17(v10);
            let mut v12: Rc<str> = text_written_18(v9.clone(), v11.clone());
            let mut v13: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = no_children_70();
            let mut v14: u64 = now_ms_16();
            host_served_1(v13.clone(), v14)
        }
        US14::US14_OtherHostServes(v6) => {
            let mut v6: u64 = *v6;
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        host_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn host_state_of_72(mut v0: u64, mut v1: bool) -> US15 {
    let mut v2: bool = v0 != 0u64;
    let mut v3: bool = v2 && v1;
    if v3 {
        US15::US15_HostRunning(v0)
    } else {
        US15::US15_HostMissing
    }
}
fn supervisor_executable_73() -> Rc<str> {
    let mut v0: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("eoie").join("supervisor"); std::fs::create_dir_all(&folder)?; let stem = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default(); let target = folder.join(format!("{}-{}-{}{}", stem, metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; let day = std::time::Duration::from_secs(86400); for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let same_stem = path.file_name().map(|name| name.to_string_lossy().starts_with(&format!("{stem}-"))).unwrap_or(false); let stale = entry.metadata().and_then(|metadata| metadata.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age > day).unwrap_or(false); if same_stem && stale && path != target { let _ = std::fs::remove_file(&path); } } } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v0.clone()
}
fn args_mark_74() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    v0.clone()
}
fn detached_started_75(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: u64 = eoie_os_launch::eoie_shell_parented_start(&*v0, &(v1).split(char::from(31u8)).filter(|arg| !arg.is_empty()).collect::<Vec<_>>()).0;
    v2
}
fn held_revived_71() -> Rc<str> {
    let mut v0: Rc<RefCell<Vec<Rc<str>>>> = held_names_55();
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = host_pid_file_64();
        let mut v5: Rc<str> = text_or_empty_7(v4.clone());
        let mut v6: u64 = decimal_u64_65(v5.clone());
        let mut v7: bool = process_alive_68(v6);
        let mut v8: US15 = host_state_of_72(v6, v7);
        match &v8 {
            US15::US15_HostMissing => {
                let mut v11: Rc<str> = jobs_root_5();
                let mut v12: Rc<str> = supervisor_executable_73();
                let mut v13: Rc<str> = args_mark_74();
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
                let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                let mut v18: u64 = detached_started_75(v12.clone(), v17.clone());
                let mut v19: bool = v18 == 0u64;
                if v19 {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: held jobs wait and no job host could be started"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                } else {
                    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("revived"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
            }
            US15::US15_HostRunning(v9) => {
                let mut v9: u64 = *v9;
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            }
        }
    }
}
fn closure10() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        held_revived_71()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn request_name_77(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    v2.clone()
}
fn supervisor_wait_of_79(mut v0: u64, mut v1: u64, mut v2: u64) -> US17 {
    let mut v3: bool = v0 != 0u64;
    if v3 {
        let mut v4: US16 = US16::US16_SupervisorUp(v0);
        US17::US17_Some(v4.clone())
    } else {
        let mut v6: bool = v1 >= v2;
        if v6 {
            let mut v7: US16 = US16::US16_SupervisorLate;
            US17::US17_Some(v7.clone())
        } else {
            US17::US17_None
        }
    }
}
fn supervisor_awaited_78(mut v0: Rc<str>, mut v1: u64) -> US16 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_14(v0.clone(), v2.clone());
        let mut v4: Rc<str> = text_or_empty_7(v3.clone());
        let mut v5: u64 = decimal_u64_65(v4.clone());
        let mut v6: u64 = now_ms_16();
        let mut v7: US17 = supervisor_wait_of_79(v5, v6, v1);
        match &v7 {
            US17::US17_None => {
                let mut v9: Rc<str> = held_file_20(v0.clone());
                let mut v10: bool = file_exists_24(v9.clone());
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    return US16::US16_SupervisorHeld(v11.clone());
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = job_file_14(v0.clone(), v13.clone());
                    let mut v15: bool = file_exists_24(v14.clone());
                    if v15 {
                        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled"); } LIT.with(|lit| lit.clone()) };
                        return US16::US16_SupervisorHeld(v16.clone());
                    } else {
                        let mut v18: u64 = 50u64;
                        std::thread::sleep(std::time::Duration::from_millis(v18));
                        (v0, v1) = (v0.clone(), v1);
                        continue;
                    }
                }
            }
            US17::US17_Some(v8) => {
                let mut v8: US16 = v8.clone();
                return v8.clone();
            }
        }
    }
}
fn supervisor_requested_76(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_5();
    let mut v2: Rc<str> = supervisor_executable_73();
    let mut v3: Rc<str> = requests_directory_2();
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v3).err().map(|error| format!("create {}: {error}", &*v3)).unwrap_or_default());
    let mut v5: Rc<str> = request_name_77(v0.clone());
    let mut v6: Rc<str> = joined_3(v3.clone(), v5.clone());
    let mut v7: Rc<str> = text_written_18(v6.clone(), v2.clone());
    let mut v8: Rc<str> = host_pid_file_64();
    let mut v9: Rc<str> = text_or_empty_7(v8.clone());
    let mut v10: u64 = decimal_u64_65(v9.clone());
    let mut v11: bool = process_alive_68(v10);
    let mut v12: US15 = host_state_of_72(v10, v11);
    let mut v21: u64 = match &v12 {
        US15::US15_HostMissing => {
            let mut v14: Rc<str> = args_mark_74();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            detached_started_75(v2.clone(), v18.clone())
        }
        US15::US15_HostRunning(v13) => {
            let mut v13: u64 = *v13;
            0u64
        }
    };
    let mut v22: u64 = now_ms_16();
    let mut v23: u64 = v22.wrapping_add(5000u64);
    let mut v24: US16 = supervisor_awaited_78(v0.clone(), v23);
    match &v24 {
        US16::US16_SupervisorHeld(v27) => {
            let mut v27: Rc<str> = v27.clone();
            v27.clone()
        }
        US16::US16_SupervisorLate => {
            let mut v28: bool = request_removed_8(v6.clone());
            if v28 {
                let mut v29: Rc<str> = args_mark_74();
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let mut v34: Rc<str> = args_mark_74();
                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v0));
                let mut v37: u64 = detached_started_75(v2.clone(), v36.clone());
                let mut v38: bool = v37 == 0u64;
                if v38 {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: neither the job host nor a detached start launched the supervisor"); } LIT.with(|lit| lit.clone()) };
                    v39.clone()
                } else {
                    decimal_text_17(v37)
                }
            } else {
                let mut v42: u64 = now_ms_16();
                let mut v43: u64 = v42.wrapping_add(60000u64);
                let mut v44: US16 = supervisor_awaited_78(v0.clone(), v43);
                match &v44 {
                    US16::US16_SupervisorHeld(v47) => {
                        let mut v47: Rc<str> = v47.clone();
                        v47.clone()
                    }
                    US16::US16_SupervisorLate => {
                        let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: the job host took the request but the supervisor never recorded its pid"); } LIT.with(|lit| lit.clone()) };
                        v48.clone()
                    }
                    US16::US16_SupervisorUp(v45) => {
                        let mut v45: u64 = *v45;
                        decimal_text_17(v45)
                    }
                }
            }
        }
        US16::US16_SupervisorUp(v25) => {
            let mut v25: u64 = *v25;
            decimal_text_17(v25)
        }
    }
}
fn closure11() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        supervisor_requested_76(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_host_run() -> i32 {
    closure0()()
}
pub fn eoie_job_host_revive() -> Rc<str> {
    closure10()()
}
pub fn eoie_job_host_request(v0: &str) -> Rc<str> {
    closure11()(Rc::<str>::from(v0))
}
