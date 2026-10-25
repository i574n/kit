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
    US0_AfterCleared,
    US0_AfterHalted(i32),
}
#[derive(Clone)]
enum US2 {
    US2_JobPending,
    US2_JobRunning,
}
#[derive(Clone)]
enum US3 {
    US3_JobFinished(i32),
    US3_JobTimedOut,
    US3_JobStopped,
    US3_JobSpawnFailed(Rc<str>),
    US3_JobSkipped(Rc<str>),
    US3_JobLost,
    US3_JobMemoryCapped,
    US3_JobExpiredInQueue,
}
#[derive(Clone)]
enum US1 {
    US1_Live(US2),
    US1_Settled(US3),
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
struct Mut0 { l0: Rc<str> }
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_7();
    joined_5(v1.clone(), v0.clone())
}
fn job_file_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_6(v0.clone());
    joined_5(v2.clone(), v1.clone())
}
fn text_or_empty_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_14(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method15(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let mut v6: US4 = US4::US4_None;
    let mut v7: US4 = v6.clone();
    let mut v8: US4 = v4.unwrap_or(v7);
    match &v8 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn job_record_of_13(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_8(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_4(v0.clone(), v4.clone());
    let mut v6: Rc<str> = text_or_empty_8(v5.clone());
    let mut v7: u64 = decimal_u64_14(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = job_file_4(v0.clone(), v8.clone());
    let mut v10: Rc<str> = text_or_empty_8(v9.clone());
    let mut v11: u64 = decimal_u64_14(v10.clone());
    let mut v12: u64 = now_ms_2();
    (v3.clone(), v7, v11, v12)
}
fn job_state_of_16(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = v0.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            let mut v8: US3 = US3::US3_JobFinished(v7);
            US1::US1_Settled(v8.clone())
        } else {
            let mut v10: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: US3 = US3::US3_JobTimedOut;
                US1::US1_Settled(v11.clone())
            } else {
                let mut v13: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                if v13 {
                    let mut v14: US3 = US3::US3_JobStopped;
                    US1::US1_Settled(v14.clone())
                } else {
                    let mut v16: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    if v16 {
                        let mut v17: US3 = US3::US3_JobLost;
                        US1::US1_Settled(v17.clone())
                    } else {
                        let mut v19: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                        if v19 {
                            let mut v20: US3 = US3::US3_JobMemoryCapped;
                            US1::US1_Settled(v20.clone())
                        } else {
                            let mut v22: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                            if v22 {
                                let mut v23: US3 = US3::US3_JobExpiredInQueue;
                                US1::US1_Settled(v23.clone())
                            } else {
                                let mut v25: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                if v25 {
                                    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v27: US3 = US3::US3_JobSkipped(v26.clone());
                                    US1::US1_Settled(v27.clone())
                                } else {
                                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let mut v30: US3 = US3::US3_JobSpawnFailed(v29.clone());
                                    US1::US1_Settled(v30.clone())
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
                let mut v42: US3 = US3::US3_JobLost;
                US1::US1_Settled(v42.clone())
            } else {
                let mut v44: US2 = US2::US2_JobRunning;
                US1::US1_Live(v44.clone())
            }
        } else {
            let mut v47: u64 = v2.wrapping_add(15000u64);
            let mut v48: bool = v47 < v3;
            if v48 {
                let mut v49: US3 = US3::US3_JobLost;
                US1::US1_Settled(v49.clone())
            } else {
                let mut v51: US2 = US2::US2_JobPending;
                US1::US1_Live(v51.clone())
            }
        }
    }
}
fn process_alive_17(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn settled_word_18(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US3::US3_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US3::US3_JobLost => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US3::US3_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US3::US3_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn text_written_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_12(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_13(v0.clone());
    let mut v5: US1 = job_state_of_16(v1.clone(), v2, v3, v4);
    match &v5 {
        US1::US1_Settled(v6) => {
            let mut v6: US3 = v6.clone();
            match &v6 {
                US3::US3_JobLost => {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = job_file_4(v0.clone(), v7.clone());
                    let mut v9: Rc<str> = text_or_empty_8(v8.clone());
                    let mut v10: u64 = decimal_u64_14(v9.clone());
                    let mut v11: Rc<str> = jobs_root_7();
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = joined_5(v11.clone(), v12.clone());
                    let mut v14: Rc<str> = joined_5(v13.clone(), v0.clone());
                    let mut v15: bool = file_exists_9(v14.clone());
                    if v15 {
                        let mut v16: US2 = US2::US2_JobPending;
                        US1::US1_Live(v16.clone())
                    } else {
                        let mut v18: bool = v10 != 0u64;
                        let mut v20: bool = if v18 {
                            process_alive_17(v10)
                        } else {
                            false
                        };
                        if v20 {
                            let mut v21: US2 = US2::US2_JobRunning;
                            US1::US1_Live(v21.clone())
                        } else {
                            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let mut v24: Rc<str> = job_file_4(v0.clone(), v23.clone());
                            let mut v25: Rc<str> = text_or_empty_8(v24.clone());
                            let mut v26: u64 = decimal_u64_14(v25.clone());
                            let mut v27: bool = v26 == 0u64;
                            let mut v30: bool = if v27 {
                                true
                            } else {
                                let mut v28: bool = process_alive_17(v26);
                                let mut v29: bool = v28 == false;
                                v29
                            };
                            let mut v31: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let mut v32: bool = v31 && v30;
                            let mut v49: Rc<str> = if v32 {
                                let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let mut v34: Rc<str> = job_file_4(v0.clone(), v33.clone());
                                let mut v35: bool = v2 > 0u64;
                                let mut v36: u64 = if v35 {
                                    v2
                                } else {
                                    v3
                                };
                                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                let mut v38: US3 = US3::US3_JobLost;
                                let mut v39: Rc<str> = settled_word_18(v38.clone());
                                let mut v40: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let mut v41: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v45: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v42, &*v37, &*v43, &*v44].concat());
                                let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v42, &*v41, &*v42, &*v45].concat());
                                text_written_19(v34.clone(), v46.clone())
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
fn state_settled_20(mut v0: US1) -> bool {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            false
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2.clone();
            true
        }
    }
}
fn decimal_text_21(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn awaited_3(mut v0: Rc<str>, mut v1: u64) -> (US1, bool, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_4(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_8(v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = job_file_4(v0.clone(), v5.clone());
    let mut v7: bool = file_exists_9(v6.clone());
    let (mut v42, mut v43, mut v44): (US1, bool, Rc<str>) = if v7 {
        let mut v8: US3 = US3::US3_JobStopped;
        let mut v9: US1 = US1::US1_Settled(v8.clone());
        (v9.clone(), true, v4.clone())
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v10).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v12: i32 = (v11.clone().borrow().len() as i32);
        let mut v13: bool = 0i32 < v12;
        let mut v24: Rc<str> = if v13 {
            let mut v14: Rc<str> = v11.clone().borrow()[0i32 as usize].clone();
            let mut v15: i32 = (v14.clone().len() as i32);
            let mut v16: i32 = 0i32;
            let mut v17: i32 = method10(v14.clone(), v15, v16);
            let mut v18: i32 = v15.wrapping_sub(1i32);
            let mut v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
            let mut v20: i32 = (v19.clone().len() as i32);
            let mut v21: i32 = method11(v19.clone(), v20);
            let mut v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
            v22.clone()
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23.clone()
        };
        let mut v25: US1 = job_state_now_12(v24.clone());
        let mut v26: bool = state_settled_20(v25.clone());
        let (mut v39, mut v40, mut v41): (US1, bool, Rc<str>) = if v26 {
            (v25.clone(), false, v4.clone())
        } else {
            let mut v27: u64 = now_ms_2();
            let mut v28: u64 = v1.wrapping_add(1000u64);
            let mut v29: bool = v27 > v28;
            let mut v34: u64 = if v29 {
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = job_file_4(v0.clone(), v30.clone());
                let mut v32: Rc<str> = decimal_text_21(v27);
                let mut v33: Rc<str> = text_written_19(v31.clone(), v32.clone());
                v27
            } else {
                v1
            };
            let mut v35: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v35));
            let (mut v36, mut v37, mut v38): (US1, bool, Rc<str>) = awaited_3(v0.clone(), v34);
            (v36.clone(), v37, v38.clone())
        };
        (v39.clone(), v40, v41.clone())
    };
    (v42.clone(), v43, v44.clone())
}
fn settled_exit_23(mut v0: US3) -> i32 {
    match &v0 {
        US3::US3_JobExpiredInQueue => {
            75i32
        }
        US3::US3_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            v1
        }
        US3::US3_JobLost => {
            125i32
        }
        US3::US3_JobMemoryCapped => {
            137i32
        }
        US3::US3_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US3::US3_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2.clone();
            125i32
        }
        US3::US3_JobStopped => {
            143i32
        }
        US3::US3_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_22(mut v0: US1) -> i32 {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            3i32
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2.clone();
            settled_exit_23(v2.clone())
        }
    }
}
fn live_word_25(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_24(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1.clone();
            live_word_25(v1.clone())
        }
        US1::US1_Settled(v3) => {
            let mut v3: US3 = v3.clone();
            settled_word_18(v3.clone())
        }
    }
}
fn method27(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_26(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn after_outcome_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: u64 = now_ms_2();
    let mut v2: u64 = 0u64;
    let (mut v3, mut v4, mut v5): (US1, bool, Rc<str>) = awaited_3(v0.clone(), v2);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v8: i32 = (v7.clone().borrow().len() as i32);
    let mut v9: bool = 0i32 < v8;
    let mut v20: Rc<str> = if v9 {
        let mut v10: Rc<str> = v7.clone().borrow()[0i32 as usize].clone();
        let mut v11: i32 = (v10.clone().len() as i32);
        let mut v12: i32 = 0i32;
        let mut v13: i32 = method10(v10.clone(), v11, v12);
        let mut v14: i32 = v11.wrapping_sub(1i32);
        let mut v15: Rc<str> = string_slice(&v10.clone(), v13 as i64, v14 as i64);
        let mut v16: i32 = (v15.clone().len() as i32);
        let mut v17: i32 = method11(v15.clone(), v16);
        let mut v18: Rc<str> = string_slice(&v15.clone(), 0i32 as i64, v17 as i64);
        v18.clone()
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    };
    if v4 {
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = job_file_4(v0.clone(), v21.clone());
        let mut v23: u64 = now_ms_2();
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v20));
        let mut v26: US3 = US3::US3_JobStopped;
        let mut v27: Rc<str> = settled_word_18(v26.clone());
        let mut v28: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
        let mut v29: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v6, &*v25, &*v30, &*v31].concat());
        let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v6, &*v29, &*v6, &*v32].concat());
        let mut v34: Rc<str> = text_written_19(v22.clone(), v33.clone());
        US0::US0_AfterHalted(143i32)
    } else {
        let mut v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v37: i32 = (v36.clone().borrow().len() as i32);
        let mut v38: bool = 1i32 < v37;
        let mut v49: Rc<str> = if v38 {
            let mut v39: Rc<str> = v36.clone().borrow()[1i32 as usize].clone();
            let mut v40: i32 = (v39.clone().len() as i32);
            let mut v41: i32 = 0i32;
            let mut v42: i32 = method10(v39.clone(), v40, v41);
            let mut v43: i32 = v40.wrapping_sub(1i32);
            let mut v44: Rc<str> = string_slice(&v39.clone(), v42 as i64, v43 as i64);
            let mut v45: i32 = (v44.clone().len() as i32);
            let mut v46: i32 = method11(v44.clone(), v45);
            let mut v47: Rc<str> = string_slice(&v44.clone(), 0i32 as i64, v46 as i64);
            v47.clone()
        } else {
            let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v48.clone()
        };
        let mut v50: bool = v49.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        let mut v53: bool = if v50 {
            let mut v51: i32 = state_exit_22(v3.clone());
            let mut v52: bool = v51 != 0i32;
            v52
        } else {
            false
        };
        if v53 {
            let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) };
            let mut v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v20));
            let mut v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) };
            let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            let mut v58: Rc<str> = state_word_24(v3.clone());
            let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
            let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) };
            let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
            let mut v62: i32 = state_exit_22(v3.clone());
            let mut v63: Rc<str> = format_real_26(v62);
            let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v63));
            let mut v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v66: Rc<str> = job_file_4(v0.clone(), v65.clone());
            let mut v67: u64 = now_ms_2();
            let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v69: US3 = US3::US3_JobSkipped(v68.clone());
            let mut v70: Rc<str> = settled_word_18(v69.clone());
            let mut v71: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let mut v72: Rc<str> = std::rc::Rc::<str>::from((v67).to_string());
            let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v74: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v6, &*v64, &*v73, &*v68].concat());
            let mut v75: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v6, &*v72, &*v6, &*v74].concat());
            let mut v76: Rc<str> = text_written_19(v66.clone(), v75.clone());
            US0::US0_AfterHalted(125i32)
        } else {
            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
            let mut v79: Rc<str> = job_file_4(v0.clone(), v78.clone());
            let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v80));
            let mut v82: Rc<str> = state_word_24(v3.clone());
            let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
            let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
            let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
            let mut v86: u64 = now_ms_2();
            let mut v87: u64 = v86.wrapping_sub(v1);
            let mut v88: Rc<str> = decimal_text_21(v87);
            let mut v89: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v88));
            let mut v90: Rc<str> = text_written_19(v79.clone(), v89.clone());
            US0::US0_AfterCleared
        }
    }
}
fn after_wait_0(mut v0: Rc<str>) -> i32 {
    let mut v1: US0 = after_outcome_of_1(v0.clone());
    match &v1 {
        US0::US0_AfterCleared => {
            0i32
        }
        US0::US0_AfterHalted(v2) => {
            let mut v2: i32 = *v2;
            v2
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        after_wait_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_after_wait(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
