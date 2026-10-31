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
enum US1 {
    US1_JobPending,
    US1_JobRunning,
}
#[derive(Clone)]
enum US2 {
    US2_JobFinished(i32),
    US2_JobTimedOut,
    US2_JobStopped,
    US2_JobSpawnFailed(Rc<str>),
    US2_JobSkipped(Rc<str>),
    US2_JobLost,
    US2_JobMemoryCapped,
    US2_JobExpiredInQueue,
}
#[derive(Clone)]
enum US0 {
    US0_Live(US1),
    US0_Settled(US2),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_NotWaiting,
    US4_WaitingAfter(Rc<str>),
    US4_WaitingWindow(Rc<str>, Rc<str>),
    US4_WaitingSlot,
}
struct Mut0 { l0: Rc<str> }
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_6();
    joined_4(v1, v0)
}
fn job_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_5(v0);
    joined_4(v2, v1)
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method9(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_8(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method9(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn now_ms_10() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_2(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_3(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_7(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_3(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_7(v5);
    let v7: u64 = decimal_u64_8(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_3(v0, v8);
    let v10: Rc<str> = text_or_empty_7(v9);
    let v11: u64 = decimal_u64_8(v10);
    let v12: u64 = now_ms_10();
    (v3, v7, v11, v12)
}
fn job_state_of_11(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: bool = &*v5 == "finished";
        if v6 {
            let v7: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
            let v8: US2 = US2::US2_JobFinished(v7);
            US0::US0_Settled(v8.clone())
        } else {
            let v10: bool = &*v5 == "timed-out";
            if v10 {
                let v11: US2 = US2::US2_JobTimedOut;
                US0::US0_Settled(v11.clone())
            } else {
                let v13: bool = &*v5 == "stopped";
                if v13 {
                    let v14: US2 = US2::US2_JobStopped;
                    US0::US0_Settled(v14.clone())
                } else {
                    let v16: bool = &*v5 == "lost";
                    if v16 {
                        let v17: US2 = US2::US2_JobLost;
                        US0::US0_Settled(v17.clone())
                    } else {
                        let v19: bool = &*v5 == "memory-capped";
                        if v19 {
                            let v20: US2 = US2::US2_JobMemoryCapped;
                            US0::US0_Settled(v20.clone())
                        } else {
                            let v22: bool = &*v5 == "expired-in-queue";
                            if v22 {
                                let v23: US2 = US2::US2_JobExpiredInQueue;
                                US0::US0_Settled(v23.clone())
                            } else {
                                let v25: bool = &*v5 == "skipped";
                                if v25 {
                                    let v26: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let v27: US2 = US2::US2_JobSkipped(v26.clone());
                                    US0::US0_Settled(v27.clone())
                                } else {
                                    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
                                    let v30: US2 = US2::US2_JobSpawnFailed(v29.clone());
                                    US0::US0_Settled(v30.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let v39: bool = v1 > 0u64;
        if v39 {
            let v40: u64 = v1.wrapping_add(15000u64);
            let v41: bool = v40 < v3;
            if v41 {
                let v42: US2 = US2::US2_JobLost;
                US0::US0_Settled(v42.clone())
            } else {
                let v44: US1 = US1::US1_JobRunning;
                US0::US0_Live(v44.clone())
            }
        } else {
            let v47: u64 = v2.wrapping_add(15000u64);
            let v48: bool = v47 < v3;
            if v48 {
                let v49: US2 = US2::US2_JobLost;
                US0::US0_Settled(v49.clone())
            } else {
                let v51: US1 = US1::US1_JobPending;
                US0::US0_Live(v51.clone())
            }
        }
    }
}
fn file_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn process_alive_13(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn settled_word_14(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US2::US2_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US2::US2_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_JobSpawnFailed(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn text_written_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_1(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_2(v0.clone());
    let mut v5: US0 = job_state_of_11(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Settled(v6) => {
            let mut v6: US2 = v6.clone();
            match &v6 {
                US2::US2_JobLost => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = job_file_3(v0.clone(), v7);
                    let v9: Rc<str> = text_or_empty_7(v8);
                    let v10: u64 = decimal_u64_8(v9);
                    let v11: Rc<str> = jobs_root_6();
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = joined_4(v11, v12);
                    let v14: Rc<str> = joined_4(v13, v0.clone());
                    let v15: bool = file_exists_12(v14);
                    if v15 {
                        let v16: US1 = US1::US1_JobPending;
                        US0::US0_Live(v16)
                    } else {
                        let v18: bool = v10 != 0u64;
                        let v20: bool = if v18 {
                            process_alive_13(v10)
                        } else {
                            false
                        };
                        if v20 {
                            let v21: US1 = US1::US1_JobRunning;
                            US0::US0_Live(v21)
                        } else {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let v24: Rc<str> = job_file_3(v0.clone(), v23);
                            let v25: Rc<str> = text_or_empty_7(v24);
                            let v26: u64 = decimal_u64_8(v25);
                            let v27: bool = v26 == 0u64;
                            let v30: bool = if v27 {
                                true
                            } else {
                                let v28: bool = process_alive_13(v26);
                                let v29: bool = v28 == false;
                                v29
                            };
                            let v31: bool = &*v1 == "";
                            let v32: bool = v31 && v30;
                            let v49: Rc<str> = if v32 {
                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v34: Rc<str> = job_file_3(v0, v33);
                                let v35: bool = v2 > 0u64;
                                let v36: u64 = if v35 {
                                    v2
                                } else {
                                    v3
                                };
                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                let v38: US2 = US2::US2_JobLost;
                                let v39: Rc<str> = settled_word_14(v38);
                                let v40: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let v41: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v45: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v42, &*v37, &*v43, &*v44].concat());
                                let v46: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v42, &*v41, &*v42, &*v45].concat());
                                text_written_15(v34, v46)
                            } else {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v48
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
            v5
        }
    }
}
fn state_settled_16(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1.clone();
            false
        }
        US0::US0_Settled(v2) => {
            let v2: US2 = v2.clone();
            true
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
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
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn window_reason_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method21(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: i32 = v7.wrapping_add(1i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let v13: i32 = v12.wrapping_sub(1i32);
        let v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: i32 = (v17.clone().len() as i32);
        let v19: i32 = 0i32;
        let v20: i32 = method18(v17.clone(), v18, v19);
        let v21: i32 = v18.wrapping_sub(1i32);
        let v22: Rc<str> = string_slice(&v17.clone(), v20 as i64, v21 as i64);
        let v23: i32 = (v22.clone().len() as i32);
        let v24: i32 = method19(v22.clone(), v23);
        let v25: Rc<str> = string_slice(&v22.clone(), 0i32 as i64, v24 as i64);
        v25.clone()
    }
}
fn gate_wait_of_17(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_3(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_12(v2.clone());
    let v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_3(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_12(v5.clone());
        let v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        let v9: Rc<str> = job_file_3(v0.clone(), v1.clone());
        let v10: Rc<str> = text_or_empty_7(v9.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v11).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v13: Rc<str> = v12.clone().borrow()[0i32 as usize].clone();
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = 0i32;
        let v16: i32 = method18(v13.clone(), v14, v15);
        let v17: i32 = v14.wrapping_sub(1i32);
        let v18: Rc<str> = string_slice(&v13.clone(), v16 as i64, v17 as i64);
        let v19: i32 = (v18.clone().len() as i32);
        let v20: i32 = method19(v18.clone(), v19);
        let v21: Rc<str> = string_slice(&v18.clone(), 0i32 as i64, v20 as i64);
        US4::US4_WaitingAfter(v21.clone())
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = job_file_3(v0.clone(), v23.clone());
        let v25: bool = file_exists_12(v24.clone());
        if v25 {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = job_file_3(v0.clone(), v26.clone());
            let v28: Rc<str> = text_or_empty_7(v27.clone());
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v28.starts_with(&*v29);
            if v30 {
                US4::US4_NotWaiting
            } else {
                let v32: Rc<str> = job_file_3(v0.clone(), v23.clone());
                let v33: Rc<str> = text_or_empty_7(v32.clone());
                let v34: i32 = (v33.clone().len() as i32);
                let v35: i32 = 0i32;
                let v36: i32 = method18(v33.clone(), v34, v35);
                let v37: i32 = v34.wrapping_sub(1i32);
                let v38: Rc<str> = string_slice(&v33.clone(), v36 as i64, v37 as i64);
                let v39: i32 = (v38.clone().len() as i32);
                let v40: i32 = method19(v38.clone(), v39);
                let v41: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v40 as i64);
                let v42: Rc<str> = window_reason_20(v28.clone());
                US4::US4_WaitingWindow(v41.clone(), v42.clone())
            }
        } else {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = job_file_3(v0.clone(), v45.clone());
            let v47: bool = file_exists_12(v46.clone());
            let v54: bool = if v47 {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = job_file_3(v0.clone(), v48.clone());
                let v50: Rc<str> = text_or_empty_7(v49.clone());
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy: acquired slot"); } LIT.with(|lit| lit.clone()) };
                let v52: bool = v50.contains(&*v51);
                let v53: bool = v52 == false;
                v53
            } else {
                false
            };
            if v54 {
                US4::US4_WaitingSlot
            } else {
                US4::US4_NotWaiting
            }
        }
    }
}
fn live_word_23(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_22(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1.clone();
            live_word_23(v1)
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3.clone();
            settled_word_14(v3)
        }
    }
}
fn settled_exit_24(mut v0: US2) -> i32 {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            75i32
        }
        US2::US2_JobFinished(v1) => {
            let v1: i32 = *v1;
            v1
        }
        US2::US2_JobLost => {
            125i32
        }
        US2::US2_JobMemoryCapped => {
            137i32
        }
        US2::US2_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US2::US2_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2.clone();
            125i32
        }
        US2::US2_JobStopped => {
            143i32
        }
        US2::US2_JobTimedOut => {
            124i32
        }
    }
}
fn elapsed_of_25(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_2(v0);
    let mut v5: bool = &*v1 == "";
    let v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_8(v6)
    };
    let v9: bool = v8 > v3;
    if v9 {
        let v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn method28(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_27(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method28(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_26(mut v0: u64) -> Rc<str> {
    format_real_27(v0)
}
fn file_bytes_29(mut v0: Rc<str>) -> u64 {
    let v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_summary_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = job_state_now_1(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v1, &*v5].concat());
    let v7: bool = state_settled_16(v2.clone());
    let v10: US4 = if v7 {
        US4::US4_NotWaiting
    } else {
        gate_wait_of_17(v1.clone())
    };
    let v11: bool = match &v10 {
        US4::US4_NotWaiting => {
            true
        }
        _ => {
            false
        }
    };
    let v12: bool = v11 != true;
    let v15: bool = if v12 {
        let v13: bool = state_settled_16(v2.clone());
        let v14: bool = v13 == false;
        v14
    } else {
        false
    };
    let v18: Rc<str> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        state_word_22(v2.clone())
    };
    let v37: Rc<str> = match &v10 {
        US4::US4_NotWaiting => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19
        }
        US4::US4_WaitingAfter(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waiting_for=after:"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v20));
            v22
        }
        US4::US4_WaitingSlot => {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waiting_for=heavy-slot"); } LIT.with(|lit| lit.clone()) };
            v33
        }
        US4::US4_WaitingWindow(v23, v24) => {
            let v23: Rc<str> = v23.clone();
            let v24: Rc<str> = v24.clone();
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waiting_for=window:"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v23));
            let v27: bool = &*v24 == "";
            let v31: Rc<str> = if v27 {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v28
            } else {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v24));
                v30
            };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v31));
            v32
        }
    };
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v37));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
    let v47: Rc<str> = match &v2 {
        US0::US0_Live(v41) => {
            let v41: US1 = v41.clone();
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v42
        }
        US0::US0_Settled(v43) => {
            let v43: US2 = v43.clone();
            let v44: i32 = settled_exit_24(v43);
            let v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
            v45
        }
    };
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v47));
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
    let v51: u64 = elapsed_of_25(v1.clone());
    let v52: Rc<str> = decimal_text_26(v51);
    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v52));
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = job_file_3(v1.clone(), v55);
    let v57: u64 = file_bytes_29(v56);
    let v58: Rc<str> = decimal_text_26(v57);
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = job_file_3(v1.clone(), v60);
    let v62: u64 = file_bytes_29(v61);
    let v63: Rc<str> = decimal_text_26(v62);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v65: Rc<str> = std::rc::Rc::<str>::from([&*v54, &*v58, &*v59, &*v63, &*v64].concat());
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = job_file_3(v1.clone(), v66);
    let v68: Rc<str> = text_or_empty_7(v67);
    let v69: Rc<str> = std::rc::Rc::<str>::from(v68.split(char::from(9u8)).nth(3).unwrap_or("").trim());
    let v70: bool = &*v69 == "";
    let v74: Rc<str> = if v70 {
        v64.clone()
    } else {
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v73: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v69, &*v72, &*v64, &*v64].concat());
        v73
    };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v77: Rc<str> = job_file_3(v1.clone(), v76);
    let v78: Rc<str> = text_or_empty_7(v77);
    let v79: Rc<str> = std::rc::Rc::<str>::from([&*v75, &*v78, &*v64, &*v64, &*v64].concat());
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = job_file_3(v1.clone(), v80);
    let v82: Rc<str> = text_or_empty_7(v81);
    let v83: bool = &*v82 == "";
    let v94: Rc<str> = if v83 {
        v64.clone()
    } else {
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" peak_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
        let v87: Rc<str> = job_file_3(v1.clone(), v86.clone());
        let v88: bool = file_exists_12(v87);
        let v92: Rc<str> = if v88 {
            let v89: Rc<str> = job_file_3(v1, v86);
            text_or_empty_7(v89)
        } else {
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            v91
        };
        let v93: Rc<str> = std::rc::Rc::<str>::from([&*v84, &*v82, &*v85, &*v92, &*v64].concat());
        v93
    };
    let v95: Rc<str> = std::rc::Rc::<str>::from([&*v94, &*v79, &*v64, &*v64, &*v64].concat());
    let v96: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v53, &*v65, &*v74, &*v95].concat());
    v96
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        job_summary_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_exit_31(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1.clone();
            3i32
        }
        US0::US0_Settled(v2) => {
            let v2: US2 = v2.clone();
            settled_exit_24(v2)
        }
    }
}
fn state_detail_32(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1.clone();
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3.clone();
            match &v3 {
                US2::US2_JobSkipped(v5) => {
                    let mut v5: Rc<str> = v5.clone();
                    v5
                }
                US2::US2_JobSpawnFailed(v4) => {
                    let mut v4: Rc<str> = v4.clone();
                    v4
                }
                _ => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn state_parts_30(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    let v1: US0 = job_state_now_1(v0);
    let v2: Rc<str> = state_word_22(v1.clone());
    let mut v3: i32 = state_exit_31(v1.clone());
    let mut v4: Rc<str> = state_detail_32(v1);
    (v2, v3, v4)
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, i32, Rc<str>) {
        state_parts_30(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_summary(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_state_parts(v0: &str) -> (Rc<str>, i32, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
