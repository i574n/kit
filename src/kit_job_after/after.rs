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
pub struct Mut0 { pub l0: Rc<str> }
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_7();
    joined_5(v1, v0)
}
fn job_file_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_6(v0);
    joined_5(v2, v1)
}
fn text_or_empty_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_14(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method15(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let mut v6: US4 = US4::US4_None;
    let v7: US4 = v6.clone();
    let v8: US4 = v4.unwrap_or(v7);
    match v8 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn job_record_of_13(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_4(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_8(v5);
    let v7: u64 = decimal_u64_14(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_4(v0, v8);
    let v10: Rc<str> = text_or_empty_8(v9);
    let v11: u64 = decimal_u64_14(v10);
    let v12: u64 = now_ms_2();
    (v3, v7, v11, v12)
}
fn state_of_parts_17(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US1 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US2 = US2::US2_JobPending;
        US1::US1_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US2 = US2::US2_JobRunning;
            US1::US1_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US3 = US3::US3_JobFinished(v1);
                US1::US1_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US3 = US3::US3_JobTimedOut;
                    US1::US1_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US3 = US3::US3_JobStopped;
                        US1::US1_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US3 = US3::US3_JobLost;
                            US1::US1_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US3 = US3::US3_JobMemoryCapped;
                                US1::US1_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US3 = US3::US3_JobExpiredInQueue;
                                    US1::US1_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US3 = US3::US3_JobSkipped(v2);
                                        US1::US1_Settled(v28)
                                    } else {
                                        let v30: US3 = US3::US3_JobSpawnFailed(v2);
                                        US1::US1_Settled(v30)
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
fn job_state_of_16(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US1 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_17(v5, v6, v7)
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US3 = US3::US3_JobLost;
                US1::US1_Settled(v12)
            } else {
                let v14: US2 = US2::US2_JobRunning;
                US1::US1_Live(v14)
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US3 = US3::US3_JobLost;
                US1::US1_Settled(v19)
            } else {
                let v21: US2 = US2::US2_JobPending;
                US1::US1_Live(v21)
            }
        }
    }
}
fn start_request_path_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let mut v6: i32 = method10(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method11(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: bool = &*v11 == "";
    let v20: Rc<str> = if v12 {
        let v13: Rc<str> = jobs_root_7();
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_5(v13, v14)
    } else {
        let v16: Rc<str> = jobs_root_7();
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = joined_5(v16, v17);
        joined_5(v18, v11)
    };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = joined_5(v20, v21);
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) }));
    joined_5(v22, v23)
}
fn process_alive_19(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn concat5_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn text_written_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn job_state_now_12(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_13(v0.clone());
    let v5: US1 = job_state_of_16(v1.clone(), v2, v3, v4);
    match &v5 {
        US1::US1_Live(v6) => {
            let mut v6: US2 = v6.clone();
            match v6 {
                US2::US2_JobRunning => {
                    let v7: Rc<str> = jobs_root_7();
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = joined_5(v7, v8);
                    let v10: Rc<str> = joined_5(v9, v0);
                    let v11: bool = file_exists_9(v10);
                    if v11 {
                        let v12: US2 = US2::US2_JobPending;
                        US1::US1_Live(v12)
                    } else {
                        v5
                    }
                }
                _ => {
                    v5
                }
            }
        }
        US1::US1_Settled(v16) => {
            let v16: US3 = v16.clone();
            match v16 {
                US3::US3_JobLost => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = job_file_4(v0.clone(), v17);
                    let v19: Rc<str> = text_or_empty_8(v18);
                    let v20: u64 = decimal_u64_14(v19);
                    let v21: Rc<str> = jobs_root_7();
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = joined_5(v21, v22);
                    let v24: Rc<str> = joined_5(v23, v0.clone());
                    let v25: bool = file_exists_9(v24);
                    let v28: bool = if v25 {
                        true
                    } else {
                        let v26: Rc<str> = start_request_path_18(v0.clone());
                        file_exists_9(v26)
                    };
                    if v28 {
                        let v29: US2 = US2::US2_JobPending;
                        US1::US1_Live(v29)
                    } else {
                        let v31: bool = v20 != 0u64;
                        let v33: bool = if v31 {
                            process_alive_19(v20)
                        } else {
                            false
                        };
                        if v33 {
                            let v34: US2 = US2::US2_JobRunning;
                            US1::US1_Live(v34)
                        } else {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let v37: Rc<str> = job_file_4(v0.clone(), v36);
                            let v38: Rc<str> = text_or_empty_8(v37);
                            let v39: u64 = decimal_u64_14(v38);
                            let v40: bool = v39 == 0u64;
                            let v43: bool = if v40 {
                                true
                            } else {
                                let v41: bool = process_alive_19(v39);
                                let v42: bool = v41 == false;
                                v42
                            };
                            let v44: bool = &*v1 == "";
                            let v45: bool = v44 && v43;
                            let v61: Rc<str> = if v45 {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v47: Rc<str> = job_file_4(v0, v46);
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
                                let v57: Rc<str> = concat5_20(v51, v54.clone(), v50, v55, v56);
                                let v58: Rc<str> = concat5_20(v52, v54.clone(), v53, v54, v57);
                                text_written_21(v47, v58)
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
fn state_settled_22(mut v0: US1) -> bool {
    match v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1;
            false
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2;
            true
        }
    }
}
fn method25(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_24(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method25(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_23(mut v0: u64) -> Rc<str> {
    format_real_24(v0)
}
fn awaited_3(mut v0: Rc<str>, mut v1: u64) -> (US1, bool, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_4(v0.clone(), v2);
    let mut v4: Rc<str> = text_or_empty_8(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = job_file_4(v0.clone(), v5);
    let v7: bool = file_exists_9(v6);
    let (mut v42, mut v43, mut v44): (US1, bool, Rc<str>) = if v7 {
        let v8: US3 = US3::US3_JobStopped;
        let v9: US1 = US1::US1_Settled(v8);
        (v9, true, v4)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v10).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v12: i32 = (v11.clone().borrow().len() as i32);
        let v13: bool = 0i32 < v12;
        let v24: Rc<str> = if v13 {
            let v14: Rc<str> = v11.borrow()[0i32 as usize].clone();
            let v15: i32 = (v14.clone().len() as i32);
            let v16: i32 = 0i32;
            let v17: i32 = method10(v14.clone(), v15, v16);
            let v18: i32 = v15.wrapping_sub(1i32);
            let v19: Rc<str> = string_slice(&v14, v17 as i64, v18 as i64);
            let v20: i32 = (v19.clone().len() as i32);
            let v21: i32 = method11(v19.clone(), v20);
            let v22: Rc<str> = string_slice(&v19, 0i32 as i64, v21 as i64);
            v22
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23
        };
        let v25: US1 = job_state_now_12(v24);
        let v26: bool = state_settled_22(v25.clone());
        let (mut v39, mut v40, mut v41): (US1, bool, Rc<str>) = if v26 {
            (v25, false, v4)
        } else {
            let v27: u64 = now_ms_2();
            let v28: u64 = v1.wrapping_add(1000u64);
            let v29: bool = v27 > v28;
            let v34: u64 = if v29 {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = job_file_4(v0.clone(), v30);
                let v32: Rc<str> = decimal_text_23(v27);
                let v33: Rc<str> = text_written_21(v31, v32);
                v27
            } else {
                v1
            };
            let v35: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v35));
            let (mut v36, mut v37, mut v38): (US1, bool, Rc<str>) = awaited_3(v0, v34);
            (v36, v37, v38)
        };
        (v39, v40, v41)
    };
    (v42, v43, v44)
}
fn settled_exit_27(mut v0: US3) -> i32 {
    match v0 {
        US3::US3_JobExpiredInQueue => {
            75i32
        }
        US3::US3_JobFinished(v1) => {
            let mut v1: i32 = v1;
            v1
        }
        US3::US3_JobLost => {
            125i32
        }
        US3::US3_JobMemoryCapped => {
            137i32
        }
        US3::US3_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3;
            125i32
        }
        US3::US3_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2;
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
fn state_exit_26(mut v0: US1) -> i32 {
    match v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1;
            3i32
        }
        US1::US1_Settled(v2) => {
            let mut v2: US3 = v2;
            settled_exit_27(v2)
        }
    }
}
fn live_word_29(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_JobPending => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_JobRunning => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    }
}
fn state_word_28(mut v0: US1) -> Rc<str> {
    match v0 {
        US1::US1_Live(v1) => {
            let mut v1: US2 = v1;
            live_word_29(v1)
        }
        US1::US1_Settled(v3) => {
            let mut v3: US3 = v3;
            match v3 {
                US3::US3_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US3::US3_JobFinished(v4) => {
                    let mut v4: i32 = v4;
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
                US3::US3_JobLost => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US3::US3_JobMemoryCapped => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US3::US3_JobSkipped(v10) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US3::US3_JobSpawnFailed(v8) => {
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                US3::US3_JobStopped => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US3::US3_JobTimedOut => {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn format_real_30(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method25(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn after_outcome_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: u64 = now_ms_2();
    let mut v2: u64 = 0u64;
    let (mut v3, mut v4, mut v5): (US1, bool, Rc<str>) = awaited_3(v0.clone(), v2);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v8: i32 = (v7.clone().borrow().len() as i32);
    let v9: bool = 0i32 < v8;
    let v20: Rc<str> = if v9 {
        let v10: Rc<str> = v7.borrow()[0i32 as usize].clone();
        let v11: i32 = (v10.clone().len() as i32);
        let v12: i32 = 0i32;
        let v13: i32 = method10(v10.clone(), v11, v12);
        let v14: i32 = v11.wrapping_sub(1i32);
        let v15: Rc<str> = string_slice(&v10, v13 as i64, v14 as i64);
        let v16: i32 = (v15.clone().len() as i32);
        let v17: i32 = method11(v15.clone(), v16);
        let v18: Rc<str> = string_slice(&v15, 0i32 as i64, v17 as i64);
        v18
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19
    };
    if v4 {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = job_file_4(v0, v21);
        let v23: u64 = now_ms_2();
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) }, v20));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
        let v27: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = concat5_20(v25, v6.clone(), v24, v28, v29);
        let v31: Rc<str> = concat5_20(v26, v6.clone(), v27, v6.clone(), v30);
        let v32: Rc<str> = text_written_21(v22, v31);
        US0::US0_AfterHalted(143i32)
    } else {
        let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v35: i32 = (v34.clone().borrow().len() as i32);
        let v36: bool = 1i32 < v35;
        let v47: Rc<str> = if v36 {
            let v37: Rc<str> = v34.borrow()[1i32 as usize].clone();
            let v38: i32 = (v37.clone().len() as i32);
            let v39: i32 = 0i32;
            let v40: i32 = method10(v37.clone(), v38, v39);
            let v41: i32 = v38.wrapping_sub(1i32);
            let v42: Rc<str> = string_slice(&v37, v40 as i64, v41 as i64);
            let v43: i32 = (v42.clone().len() as i32);
            let v44: i32 = method11(v42.clone(), v43);
            let v45: Rc<str> = string_slice(&v42, 0i32 as i64, v44 as i64);
            v45
        } else {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v46
        };
        let v48: bool = &*v47 == "ok";
        let v51: bool = if v48 {
            let v49: i32 = state_exit_26(v3.clone());
            let v50: bool = v49 != 0i32;
            v50
        } else {
            false
        };
        if v51 {
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) }, v20));
            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) }));
            let v54: Rc<str> = state_word_28(v3.clone());
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) }));
            let v57: i32 = state_exit_26(v3);
            let v58: Rc<str> = format_real_30(v57);
            let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v58));
            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v61: Rc<str> = job_file_4(v0, v60);
            let v62: u64 = now_ms_2();
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            let v64: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v65: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v68: Rc<str> = concat5_20(v63, v6.clone(), v59, v66, v67);
            let v69: Rc<str> = concat5_20(v64, v6.clone(), v65, v6.clone(), v68);
            let v70: Rc<str> = text_written_21(v61, v69);
            US0::US0_AfterHalted(125i32)
        } else {
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
            let v73: Rc<str> = job_file_4(v0, v72);
            let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v75: Rc<str> = state_word_28(v3);
            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) }));
            let v78: u64 = now_ms_2();
            let v79: u64 = v78.wrapping_sub(v1);
            let v80: Rc<str> = decimal_text_23(v79);
            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v80));
            let v82: Rc<str> = text_written_21(v73, v81);
            US0::US0_AfterCleared
        }
    }
}
fn after_wait_0(mut v0: Rc<str>) -> i32 {
    let mut v1: US0 = after_outcome_of_1(v0);
    match v1 {
        US0::US0_AfterCleared => {
            0i32
        }
        US0::US0_AfterHalted(v2) => {
            let mut v2: i32 = v2;
            v2
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        after_wait_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_after_wait(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
