#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_AfterAny,
    US2_AfterOk,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobPending,
    US4_JobRunning,
}
#[derive(Clone)]
enum US5 {
    US5_JobFinished(i32),
    US5_JobTimedOut,
    US5_JobStopped,
    US5_JobSpawnFailed(Rc<str>),
    US5_JobSkipped(Rc<str>),
    US5_JobLost,
    US5_JobMemoryCapped,
    US5_JobExpiredInQueue,
}
#[derive(Clone)]
enum US3 {
    US3_Live(US4),
    US3_Settled(US5),
}
#[derive(Clone)]
enum US6 {
    US6_Some(u64),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_UnknownJob,
    US8_UnknownPredecessor,
    US8_SelfAfter,
    US8_NotChained,
    US8_NotWaiting(Rc<str>),
    US8_WouldCycle,
}
#[derive(Clone)]
enum US7 {
    US7_Repointed(Rc<str>),
    US7_Refused(US8),
}
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn joined_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_9() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_9();
    joined_7(v1.clone(), v0.clone())
}
fn job_file_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_8(v0.clone());
    joined_7(v2.clone(), v1.clone())
}
fn text_or_empty_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US6> = Rc::new(move |mut v0: (u64)| -> US6 {
        let mut v1: u64 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_11(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method12(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US6> = closure1();
    let mut v4: Option<US6> = v2.map(|x| v3(x));
    let mut v6: US6 = US6::US6_None;
    let mut v7: US6 = v6.clone();
    let mut v8: US6 = v4.unwrap_or(v7);
    match &v8 {
        US6::US6_None => {
            0u64
        }
        US6::US6_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn now_ms_13() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_5(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_6(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_10(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_6(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_10(v5.clone());
    let mut v7: u64 = decimal_u64_11(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_6(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_10(v9.clone());
    let mut v11: u64 = decimal_u64_11(v10.clone());
    let mut v12: u64 = now_ms_13();
    (v3.clone(), v7, v11, v12)
}
fn job_state_of_14(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US3 {
    let mut v4: bool = v0.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            let mut v8: US5 = US5::US5_JobFinished(v7);
            US3::US3_Settled(v8.clone())
        } else {
            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: US5 = US5::US5_JobTimedOut;
                US3::US3_Settled(v11.clone())
            } else {
                let mut v13: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: US5 = US5::US5_JobStopped;
                    US3::US3_Settled(v14.clone())
                } else {
                    let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        let mut v17: US5 = US5::US5_JobLost;
                        US3::US3_Settled(v17.clone())
                    } else {
                        let mut v19: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                        if v19 {
                            let mut v20: US5 = US5::US5_JobMemoryCapped;
                            US3::US3_Settled(v20.clone())
                        } else {
                            let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                            if v22 {
                                let mut v23: US5 = US5::US5_JobExpiredInQueue;
                                US3::US3_Settled(v23.clone())
                            } else {
                                let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                if v25 {
                                    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v27: US5 = US5::US5_JobSkipped(v26.clone());
                                    US3::US3_Settled(v27.clone())
                                } else {
                                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v30: US5 = US5::US5_JobSpawnFailed(v29.clone());
                                    US3::US3_Settled(v30.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut v39: bool = v1 > 0u64;
        if v39 {
            let mut v40: u64 = v1.wrapping_add(15000u64);
            let mut v41: bool = v40 < v3;
            if v41 {
                let mut v42: US5 = US5::US5_JobLost;
                US3::US3_Settled(v42.clone())
            } else {
                let mut v44: US4 = US4::US4_JobRunning;
                US3::US3_Live(v44.clone())
            }
        } else {
            let mut v47: u64 = v2.wrapping_add(15000u64);
            let mut v48: bool = v47 < v3;
            if v48 {
                let mut v49: US5 = US5::US5_JobLost;
                US3::US3_Settled(v49.clone())
            } else {
                let mut v51: US4 = US4::US4_JobPending;
                US3::US3_Live(v51.clone())
            }
        }
    }
}
fn file_exists_15(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn process_alive_16(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn settled_word_17(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US5::US5_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US5::US5_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US5::US5_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US5::US5_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US5::US5_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_4(mut v0: Rc<str>) -> US3 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_5(v0.clone());
    let mut v5: US3 = job_state_of_14(v1.clone(), v2, v3, v4);
    match &v5 {
        US3::US3_Settled(v6) => {
            let mut v6: US5 = v6.clone();
            match &v6 {
                US5::US5_JobLost => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = job_file_6(v0.clone(), v7.clone());
                    let mut v9: Rc<str> = text_or_empty_10(v8.clone());
                    let mut v10: u64 = decimal_u64_11(v9.clone());
                    let mut v11: Rc<str> = jobs_root_9();
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = joined_7(v11.clone(), v12.clone());
                    let mut v14: Rc<str> = joined_7(v13.clone(), v0.clone());
                    let mut v15: bool = file_exists_15(v14.clone());
                    if v15 {
                        let mut v16: US4 = US4::US4_JobPending;
                        US3::US3_Live(v16.clone())
                    } else {
                        let mut v18: bool = v10 != 0u64;
                        let mut v20: bool = if v18 {
                            process_alive_16(v10)
                        } else {
                            false
                        };
                        if v20 {
                            let mut v21: US4 = US4::US4_JobRunning;
                            US3::US3_Live(v21.clone())
                        } else {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let mut v24: Rc<str> = job_file_6(v0.clone(), v23.clone());
                            let mut v25: Rc<str> = text_or_empty_10(v24.clone());
                            let mut v26: u64 = decimal_u64_11(v25.clone());
                            let mut v27: bool = v26 == 0u64;
                            let mut v30: bool = if v27 {
                                true
                            } else {
                                let mut v28: bool = process_alive_16(v26);
                                let mut v29: bool = v28 == false;
                                v29
                            };
                            let mut v31: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v32: bool = v31 && v30;
                            let mut v49: Rc<str> = if v32 {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let mut v34: Rc<str> = job_file_6(v0.clone(), v33.clone());
                                let mut v35: bool = v2 > 0u64;
                                let mut v36: u64 = if v35 {
                                    v2
                                } else {
                                    v3
                                };
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                let mut v38: US5 = US5::US5_JobLost;
                                let mut v39: Rc<str> = settled_word_17(v38.clone());
                                let mut v40: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let mut v41: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v42, &*v37, &*v43, &*v44].concat());
                                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v42, &*v41, &*v42, &*v45].concat());
                                text_written_18(v34.clone(), v46.clone())
                            } else {
                                let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            };
                            v5.clone()
                        }
                    }
                }
                _ => {
                    v5.clone()
                }
            }
        }
        _ => {
            v5.clone()
        }
    }
}
fn directory_exists_19(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
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
fn state_settled_21(mut v0: US3) -> bool {
    match &v0 {
        US3::US3_Live(v1) => {
            let mut v1: US4 = v1.clone();
            false
        }
        US3::US3_Settled(v2) => {
            let mut v2: US5 = v2.clone();
            true
        }
    }
}
fn live_word_23(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US4::US4_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_22(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_Live(v1) => {
            let mut v1: US4 = v1.clone();
            live_word_23(v1.clone())
        }
        US3::US3_Settled(v3) => {
            let mut v3: US5 = v3.clone();
            settled_word_17(v3.clone())
        }
    }
}
fn method24(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
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
fn chain_reaches_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v0.clone() == v1.clone();
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 0i32;
            let mut v14: bool = if v4 {
                true
            } else {
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = job_file_6(v0.clone(), v5.clone());
                let mut v7: bool = file_exists_20(v6.clone());
                let mut v12: bool = if v7 {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = job_file_6(v0.clone(), v8.clone());
                    let mut v10: bool = file_exists_20(v9.clone());
                    let mut v11: bool = v10 == false;
                    v11
                } else {
                    false
                };
                let mut v13: bool = v12 == false;
                v13
            };
            if v14 {
                return false;
            } else {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = job_file_6(v0.clone(), v15.clone());
                let mut v17: bool = file_exists_20(v16.clone());
                let mut v30: Rc<str> = if v17 {
                    let mut v19: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v16);
                    let mut v21: Vec<u8> = v19.unwrap();
                    let mut v22: Vec<u8> = method24(v21.clone());
                    let mut v24: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v22);
                    let mut v26: std::string::String = v24.unwrap();
                    let mut v28: Rc<str> = Rc::<str>::from(String::as_str(&v26));
                    v28.clone()
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                };
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v30.split(&*v31).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v33: Rc<str> = v32.clone().borrow()[0i32 as usize].clone();
                let mut v34: i32 = (v33.clone().len() as i32);
                let mut v35: i32 = 0i32;
                let mut v36: i32 = method25(v33.clone(), v34, v35);
                let mut v37: i32 = v34.wrapping_sub(1i32);
                let mut v38: Rc<str> = string_slice(&v33.clone(), v36 as i64, v37 as i64);
                let mut v39: i32 = (v38.clone().len() as i32);
                let mut v40: i32 = method26(v38.clone(), v39);
                let mut v41: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v40 as i64);
                let mut v42: i32 = v2.wrapping_sub(1i32);
                (v0, v1, v2) = (v41.clone(), v1.clone(), v42);
                continue;
            }
        }
    }
}
fn facts_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, bool, bool, bool, bool, Rc<str>, Rc<str>, bool) {
    let mut v2: US3 = job_state_now_4(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0.starts_with(&*v3);
    let mut v7: bool = if v4 {
        let mut v5: Rc<str> = job_directory_8(v0.clone());
        directory_exists_19(v5.clone())
    } else {
        false
    };
    let mut v8: bool = v1.starts_with(&*v3);
    let mut v11: bool = if v8 {
        let mut v9: Rc<str> = job_directory_8(v1.clone());
        directory_exists_19(v9.clone())
    } else {
        false
    };
    let mut v12: bool = v0.clone() == v1.clone();
    let mut v16: bool = if v7 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = job_file_6(v0.clone(), v13.clone());
        file_exists_20(v14.clone())
    } else {
        false
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = job_file_6(v0.clone(), v17.clone());
    let mut v19: bool = file_exists_20(v18.clone());
    let mut v24: bool = if v19 {
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = job_file_6(v0.clone(), v20.clone());
        let mut v22: bool = file_exists_20(v21.clone());
        let mut v23: bool = v22 == false;
        v23
    } else {
        false
    };
    let mut v27: bool = if v24 {
        let mut v25: bool = state_settled_21(v2.clone());
        let mut v26: bool = v25 == false;
        v26
    } else {
        false
    };
    let mut v28: Rc<str> = state_word_22(v2.clone());
    let mut v29: Rc<str> = job_file_6(v0.clone(), v17.clone());
    let mut v30: bool = file_exists_20(v29.clone());
    let mut v43: Rc<str> = if v30 {
        let mut v32: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v29);
        let mut v34: Vec<u8> = v32.unwrap();
        let mut v35: Vec<u8> = method24(v34.clone());
        let mut v37: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v35);
        let mut v39: std::string::String = v37.unwrap();
        let mut v41: Rc<str> = Rc::<str>::from(String::as_str(&v39));
        v41.clone()
    } else {
        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42.clone()
    };
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v43.split(&*v44).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v46: Rc<str> = v45.clone().borrow()[0i32 as usize].clone();
    let mut v47: i32 = (v46.clone().len() as i32);
    let mut v48: i32 = 0i32;
    let mut v49: i32 = method25(v46.clone(), v47, v48);
    let mut v50: i32 = v47.wrapping_sub(1i32);
    let mut v51: Rc<str> = string_slice(&v46.clone(), v49 as i64, v50 as i64);
    let mut v52: i32 = (v51.clone().len() as i32);
    let mut v53: i32 = method26(v51.clone(), v52);
    let mut v54: Rc<str> = string_slice(&v51.clone(), 0i32 as i64, v53 as i64);
    let mut v55: i32 = 64i32;
    let mut v56: bool = chain_reaches_27(v1.clone(), v0.clone(), v55);
    (v7, v11, v12, v16, v27, v28.clone(), v54.clone(), v56)
}
fn rechain_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: bool = 2i32 < v0;
    let mut v6: US0 = if v1 {
        let mut v2: i32 = 2i32;
        let mut v3: Rc<str> = command_line_argument_2(v2);
        US0::US0_Some(v3.clone())
    } else {
        US0::US0_None
    };
    let mut v10: Rc<str> = match &v6 {
        US0::US0_None => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_Some(v7) => {
            let mut v7: Rc<str> = v7.clone();
            v7.clone()
        }
    };
    let mut v11: i32 = command_line_argument_count_1();
    let mut v12: bool = 3i32 < v11;
    let mut v17: US0 = if v12 {
        let mut v13: i32 = 3i32;
        let mut v14: Rc<str> = command_line_argument_2(v13);
        US0::US0_Some(v14.clone())
    } else {
        US0::US0_None
    };
    let mut v21: Rc<str> = match &v17 {
        US0::US0_None => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US0::US0_Some(v18) => {
            let mut v18: Rc<str> = v18.clone();
            v18.clone()
        }
    };
    let mut v22: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: bool = if v22 {
        true
    } else {
        let mut v23: bool = v21.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v23
    };
    let mut v46: US1 = if v24 {
        US1::US1_None
    } else {
        let mut v26: i32 = command_line_argument_count_1();
        let mut v27: bool = 4i32 < v26;
        let mut v32: US0 = if v27 {
            let mut v28: i32 = 4i32;
            let mut v29: Rc<str> = command_line_argument_2(v28);
            US0::US0_Some(v29.clone())
        } else {
            US0::US0_None
        };
        let mut v36: Rc<str> = match &v32 {
            US0::US0_None => {
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v34.clone()
            }
            US0::US0_Some(v33) => {
                let mut v33: Rc<str> = v33.clone();
                v33.clone()
            }
        };
        let mut v37: bool = v36.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v37 {
            let mut v38: US2 = US2::US2_AfterAny;
            US1::US1_Some(v38.clone())
        } else {
            let mut v40: bool = v36.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ok"); } LIT.with(|lit| lit.clone()) };
            if v40 {
                let mut v41: US2 = US2::US2_AfterOk;
                US1::US1_Some(v41.clone())
            } else {
                US1::US1_None
            }
        }
    };
    match &v46 {
        US1::US1_None => {
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job usage: "); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept)"); } LIT.with(|lit| lit.clone()) };
            let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job usage: job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v49);
            2i32
        }
        US1::US1_Some(v50) => {
            let mut v50: US2 = v50.clone();
            let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (bool, bool, bool, bool, bool, Rc<str>, Rc<str>, bool) = facts_of_3(v10.clone(), v21.clone());
            let mut v59: bool = v51 == false;
            let mut v81: US7 = if v59 {
                let mut v60: US8 = US8::US8_UnknownJob;
                US7::US7_Refused(v60.clone())
            } else {
                let mut v62: bool = v52 == false;
                if v62 {
                    let mut v63: US8 = US8::US8_UnknownPredecessor;
                    US7::US7_Refused(v63.clone())
                } else {
                    if v53 {
                        let mut v65: US8 = US8::US8_SelfAfter;
                        US7::US7_Refused(v65.clone())
                    } else {
                        let mut v67: bool = v54 == false;
                        if v67 {
                            let mut v68: US8 = US8::US8_NotChained;
                            US7::US7_Refused(v68.clone())
                        } else {
                            let mut v70: bool = v55 == false;
                            if v70 {
                                let mut v71: US8 = US8::US8_NotWaiting(v56.clone());
                                US7::US7_Refused(v71.clone())
                            } else {
                                if v58 {
                                    let mut v73: US8 = US8::US8_WouldCycle;
                                    US7::US7_Refused(v73.clone())
                                } else {
                                    US7::US7_Repointed(v57.clone())
                                }
                            }
                        }
                    }
                }
            };
            match &v81 {
                US7::US7_Refused(v82) => {
                    let mut v82: US8 = v82.clone();
                    let mut v106: Rc<str> = match &v82 {
                        US8::US8_NotChained => {
                            let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was not started with --after or --after-ok; stop it and start it again with --after"); } LIT.with(|lit| lit.clone()) };
                            let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v88));
                            v89.clone()
                        }
                        US8::US8_NotWaiting(v90) => {
                            let mut v90: Rc<str> = v90.clone();
                            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is "); } LIT.with(|lit| lit.clone()) };
                            let mut v92: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v91));
                            let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v90));
                            let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", no longer waiting for its predecessor; nothing changed"); } LIT.with(|lit| lit.clone()) };
                            let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
                            v95.clone()
                        }
                        US8::US8_SelfAfter => {
                            let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a job cannot wait for itself"); } LIT.with(|lit| lit.clone()) };
                            v87.clone()
                        }
                        US8::US8_UnknownJob => {
                            let mut v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown job id "); } LIT.with(|lit| lit.clone()) };
                            let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v10));
                            v84.clone()
                        }
                        US8::US8_UnknownPredecessor => {
                            let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown predecessor id "); } LIT.with(|lit| lit.clone()) };
                            let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v21));
                            v86.clone()
                        }
                        US8::US8_WouldCycle => {
                            let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" already waits, directly or down its chain, for "); } LIT.with(|lit| lit.clone()) };
                            let mut v97: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v96));
                            let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v10));
                            let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; nothing changed"); } LIT.with(|lit| lit.clone()) };
                            let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
                            v100.clone()
                        }
                    };
                    let mut v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job after refused: "); } LIT.with(|lit| lit.clone()) };
                    let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v106));
                    eprintln!("{}", v108);
                    2i32
                }
                US7::US7_Repointed(v109) => {
                    let mut v109: Rc<str> = v109.clone();
                    let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                    let mut v111: Rc<str> = job_file_6(v10.clone(), v110.clone());
                    let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v113: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v112));
                    let mut v117: Rc<str> = match &v50 {
                        US2::US2_AfterAny => {
                            let mut v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
                            v114.clone()
                        }
                        US2::US2_AfterOk => {
                            let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                            v115.clone()
                        }
                    };
                    let mut v118: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v117));
                    std::fs::write(&*v111, &*v118).unwrap();
                    let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job after ok id="); } LIT.with(|lit| lit.clone()) };
                    let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v10));
                    let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                    let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                    let mut v124: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v21));
                    let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_mode="); } LIT.with(|lit| lit.clone()) };
                    let mut v126: Rc<str> = Rc::<str>::from(format!("{}{}", v124, v125));
                    let mut v130: Rc<str> = match &v50 {
                        US2::US2_AfterAny => {
                            let mut v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
                            v127.clone()
                        }
                        US2::US2_AfterOk => {
                            let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                            v128.clone()
                        }
                    };
                    let mut v131: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v130));
                    let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" previous="); } LIT.with(|lit| lit.clone()) };
                    let mut v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                    let mut v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v109));
                    println!("{}", v134);
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rechain_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_rechain_run() -> i32 {
    closure0()()
}
