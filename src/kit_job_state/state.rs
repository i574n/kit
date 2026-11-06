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
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_6() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_6();
    joined_4(v1, v0)
}
fn job_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_5(v0);
    joined_4(v2, v1)
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method9(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_8(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method9(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn now_ms_10() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_2(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_3(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_7(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_3(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_7(v5);
    let v7: u64 = decimal_u64_8(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_3(v0, v8);
    let v10: Rc<str> = text_or_empty_7(v9);
    let v11: u64 = decimal_u64_8(v10);
    let v12: u64 = now_ms_10();
    (v3, v7, v11, v12)
}
fn state_of_parts_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US1 = US1::US1_JobPending;
        US0::US0_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US1 = US1::US1_JobRunning;
            US0::US0_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US2 = US2::US2_JobFinished(v1);
                US0::US0_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US2 = US2::US2_JobTimedOut;
                    US0::US0_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US2 = US2::US2_JobStopped;
                        US0::US0_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US2 = US2::US2_JobLost;
                            US0::US0_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US2 = US2::US2_JobMemoryCapped;
                                US0::US0_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US2 = US2::US2_JobExpiredInQueue;
                                    US0::US0_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US2 = US2::US2_JobSkipped(v2);
                                        US0::US0_Settled(v28)
                                    } else {
                                        let v30: US2 = US2::US2_JobSpawnFailed(v2);
                                        US0::US0_Settled(v30)
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
fn job_state_of_11(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_12(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US2 = US2::US2_JobLost;
                US0::US0_Settled(v12)
            } else {
                let v14: US1 = US1::US1_JobRunning;
                US0::US0_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US2 = US2::US2_JobLost;
                US0::US0_Settled(v19)
            } else {
                let v21: US1 = US1::US1_JobPending;
                US0::US0_Live(v21)
            }
        }
    }
}
fn file_exists_13(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn start_request_path_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_3(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_7(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let mut v6: i32 = method15(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method16(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: bool = &*v11 == "";
    let v20: Rc<str> = if v12 {
        let v13: Rc<str> = jobs_root_6();
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_4(v13, v14)
    } else {
        let v16: Rc<str> = jobs_root_6();
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = joined_4(v16, v17);
        joined_4(v18, v11)
    };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = joined_4(v20, v21);
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) }));
    joined_4(v22, v23)
}
fn process_alive_17(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn concat5_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn text_written_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_state_now_1(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_2(v0.clone());
    let v5: US0 = job_state_of_11(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Live(v6) => {
            let mut v6: US1 = v6.clone();
            match v6 {
                US1::US1_JobRunning => {
                    let v7: Rc<str> = jobs_root_6();
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = joined_4(v7, v8);
                    let v10: Rc<str> = joined_4(v9, v0);
                    let v11: bool = file_exists_13(v10);
                    if v11 {
                        let v12: US1 = US1::US1_JobPending;
                        US0::US0_Live(v12)
                    } else {
                        v5
                    }
                }
                _ => {
                    v5
                }
            }
        }
        US0::US0_Settled(v16) => {
            let v16: US2 = v16.clone();
            match v16 {
                US2::US2_JobLost => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = job_file_3(v0.clone(), v17);
                    let v19: Rc<str> = text_or_empty_7(v18);
                    let v20: u64 = decimal_u64_8(v19);
                    let v21: Rc<str> = jobs_root_6();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = joined_4(v21, v22);
                    let v24: Rc<str> = joined_4(v23, v0.clone());
                    let v25: bool = file_exists_13(v24);
                    let v28: bool = if v25 {
                        true
                    } else {
                        let v26: Rc<str> = start_request_path_14(v0.clone());
                        file_exists_13(v26)
                    };
                    if v28 {
                        let v29: US1 = US1::US1_JobPending;
                        US0::US0_Live(v29)
                    } else {
                        let v31: bool = v20 != 0u64;
                        let v33: bool = if v31 {
                            process_alive_17(v20)
                        } else {
                            false
                        };
                        if v33 {
                            let v34: US1 = US1::US1_JobRunning;
                            US0::US0_Live(v34)
                        } else {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = job_file_3(v0.clone(), v36);
                            let v38: Rc<str> = text_or_empty_7(v37);
                            let v39: u64 = decimal_u64_8(v38);
                            let v40: bool = v39 == 0u64;
                            let v43: bool = if v40 {
                                true
                            } else {
                                let v41: bool = process_alive_17(v39);
                                let v42: bool = v41 == false;
                                v42
                            };
                            let v44: bool = &*v1 == "";
                            let v45: bool = v44 && v43;
                            let v61: Rc<str> = if v45 {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v47: Rc<str> = job_file_3(v0, v46);
                                let v48: bool = v2 > 0u64;
                                let v49: u64 = if v48 {
                                    v2
                                } else {
                                    v3
                                };
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                let v52: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let v53: Rc<str> = std::rc::Rc::<str>::from((v49).to_string());
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v57: Rc<str> = concat5_18(v51, v54.clone(), v50, v55, v56);
                                let v58: Rc<str> = concat5_18(v52, v54.clone(), v53, v54, v57);
                                text_written_19(v47, v58)
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v60
                            };
                            v5
                        }
                    }
                }
                _ => {
                    v5
                }
            }
        }
    }
}
fn live_word_21(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_20(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1;
            live_word_21(v1)
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3;
            match v3 {
                US2::US2_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US2::US2_JobFinished(v4) => {
                    let mut v4: i32 = v4;
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US2::US2_JobLost => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US2::US2_JobMemoryCapped => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US2::US2_JobSkipped(v10) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US2::US2_JobSpawnFailed(v8) => {
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US2::US2_JobStopped => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US2::US2_JobTimedOut => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn settled_exit_23(mut v0: US2) -> i32 {
    match v0 {
        US2::US2_JobExpiredInQueue => {
            75i32
        }
        US2::US2_JobFinished(v1) => {
            let mut v1: i32 = v1;
            v1
        }
        US2::US2_JobLost => {
            125i32
        }
        US2::US2_JobMemoryCapped => {
            137i32
        }
        US2::US2_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3;
            125i32
        }
        US2::US2_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2;
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
fn state_exit_22(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1;
            3i32
        }
        US0::US0_Settled(v2) => {
            let mut v2: US2 = v2;
            settled_exit_23(v2)
        }
    }
}
fn state_detail_24(mut v0: US0) -> Rc<str> {
    match v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3;
            match v3 {
                US2::US2_JobSkipped(v5) => {
                    let v5: Rc<str> = v5;
                    v5
                }
                US2::US2_JobSpawnFailed(v4) => {
                    let mut v4: Rc<str> = v4;
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
fn state_parts_0(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    let mut v1: bool = kit_job_orphans::kit_job_orphan_settled(&v0);
    let mut v2: US0 = job_state_now_1(v0);
    let mut v3: Rc<str> = state_word_20(v2.clone());
    let mut v4: i32 = state_exit_22(v2.clone());
    let v5: Rc<str> = state_detail_24(v2);
    (v3, v4, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, i32, Rc<str>) {
        state_parts_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_state_parts(v0: &str) -> (Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
