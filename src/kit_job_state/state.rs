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
#[derive(Clone)]
enum US6 {
    US6_Machine(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Local,
    US5_OnMachine(US6),
}
#[derive(Clone)]
enum US7 {
    US7_Some(US5),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US6),
    US8_None,
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn joined_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
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
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method10(v1.clone());
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
fn now_ms_11() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn job_record_of_3(mut v0: Rc<str>) -> (Rc<str>, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = job_file_4(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_8(v5);
    let v7: u64 = decimal_u64_9(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_4(v0, v8);
    let v10: Rc<str> = text_or_empty_8(v9);
    let v11: u64 = decimal_u64_9(v10);
    let v12: u64 = now_ms_11();
    (v3, v7, v11, v12)
}
fn state_of_parts_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
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
                                        let v28: US2 = US2::US2_JobSkipped(v2.clone());
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
fn job_state_of_12(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64) -> US0 {
    let mut v4: bool = &*v0 != "";
    if v4 {
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(2).unwrap_or("").trim());
        let mut v6: i32 = v0.split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(1);
        let v7: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(3).unwrap_or("").trim());
        state_of_parts_13(v5.clone(), v6, v7.clone())
    } else {
        let v9: bool = v1 > 0u64;
        if v9 {
            let v10: u64 = v1.wrapping_add(15000u64);
            let v11: bool = v10 < v3;
            if v11 {
                let v12: US2 = US2::US2_JobLost;
                US0::US0_Settled(v12.clone())
            } else {
                let v14: US1 = US1::US1_JobRunning;
                US0::US0_Live(v14.clone())
            }
        } else {
            let v17: u64 = v2.wrapping_add(15000u64);
            let v18: bool = v17 < v3;
            if v18 {
                let v19: US2 = US2::US2_JobLost;
                US0::US0_Settled(v19.clone())
            } else {
                let v21: US1 = US1::US1_JobPending;
                US0::US0_Live(v21.clone())
            }
        }
    }
}
fn file_exists_14(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn process_alive_15(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn text_written_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn job_state_now_2(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_3(v0.clone());
    let mut v5: US0 = job_state_of_12(v1.clone(), v2, v3, v4);
    match &v5 {
        US0::US0_Settled(v6) => {
            let mut v6: US2 = v6.clone();
            match &v6 {
                US2::US2_JobLost => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = job_file_4(v0.clone(), v7);
                    let v9: Rc<str> = text_or_empty_8(v8);
                    let v10: u64 = decimal_u64_9(v9);
                    let v11: Rc<str> = jobs_root_7();
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = joined_5(v11, v12);
                    let v14: Rc<str> = joined_5(v13, v0.clone());
                    let v15: bool = file_exists_14(v14);
                    if v15 {
                        let v16: US1 = US1::US1_JobPending;
                        US0::US0_Live(v16)
                    } else {
                        let v18: bool = v10 != 0u64;
                        let v20: bool = if v18 {
                            process_alive_15(v10)
                        } else {
                            false
                        };
                        if v20 {
                            let v21: US1 = US1::US1_JobRunning;
                            US0::US0_Live(v21)
                        } else {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                            let v24: Rc<str> = job_file_4(v0.clone(), v23);
                            let v25: Rc<str> = text_or_empty_8(v24);
                            let v26: u64 = decimal_u64_9(v25);
                            let v27: bool = v26 == 0u64;
                            let v30: bool = if v27 {
                                true
                            } else {
                                let v28: bool = process_alive_15(v26);
                                let v29: bool = v28 == false;
                                v29
                            };
                            let v31: bool = &*v1 == "";
                            let v32: bool = v31 && v30;
                            let v48: Rc<str> = if v32 {
                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                                let v34: Rc<str> = job_file_4(v0, v33);
                                let v35: bool = v2 > 0u64;
                                let v36: u64 = if v35 {
                                    v2
                                } else {
                                    v3
                                };
                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor stopped beating: it was killed (with its caller, by the memory reaper) or crashed, and its job tree ended with it"); } LIT.with(|lit| lit.clone()) };
                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                let v39: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
                                let v40: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v44: Rc<str> = concat5_1(v38, v41.clone(), v37, v42, v43);
                                let v45: Rc<str> = concat5_1(v39, v41.clone(), v40, v41, v44);
                                text_written_16(v34, v45)
                            } else {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v47
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
fn state_settled_17(mut v0: US0) -> bool {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            false
        }
        US0::US0_Settled(v2) => {
            let mut v2: US2 = v2.clone();
            true
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn window_reason_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method22(v2.clone());
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
        let v20: i32 = method19(v17.clone(), v18, v19);
        let v21: i32 = v18.wrapping_sub(1i32);
        let v22: Rc<str> = string_slice(&v17.clone(), v20 as i64, v21 as i64);
        let v23: i32 = (v22.clone().len() as i32);
        let v24: i32 = method20(v22.clone(), v23);
        let v25: Rc<str> = string_slice(&v22.clone(), 0i32 as i64, v24 as i64);
        v25.clone()
    }
}
fn gate_wait_of_18(mut v0: Rc<str>) -> US4 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_14(v2.clone());
    let v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = job_file_4(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_14(v5.clone());
        let v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        let v9: Rc<str> = job_file_4(v0.clone(), v1.clone());
        let v10: Rc<str> = text_or_empty_8(v9.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v11).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v13: Rc<str> = v12.borrow()[0i32 as usize].clone();
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = 0i32;
        let v16: i32 = method19(v13.clone(), v14, v15);
        let v17: i32 = v14.wrapping_sub(1i32);
        let v18: Rc<str> = string_slice(&v13.clone(), v16 as i64, v17 as i64);
        let v19: i32 = (v18.clone().len() as i32);
        let v20: i32 = method20(v18.clone(), v19);
        let v21: Rc<str> = string_slice(&v18.clone(), 0i32 as i64, v20 as i64);
        US4::US4_WaitingAfter(v21.clone())
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = job_file_4(v0.clone(), v23.clone());
        let v25: bool = file_exists_14(v24.clone());
        if v25 {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = job_file_4(v0.clone(), v26.clone());
            let v28: Rc<str> = text_or_empty_8(v27.clone());
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v28.starts_with(&*v29);
            if v30 {
                US4::US4_NotWaiting
            } else {
                let v32: Rc<str> = job_file_4(v0.clone(), v23.clone());
                let v33: Rc<str> = text_or_empty_8(v32.clone());
                let v34: i32 = (v33.clone().len() as i32);
                let v35: i32 = 0i32;
                let v36: i32 = method19(v33.clone(), v34, v35);
                let v37: i32 = v34.wrapping_sub(1i32);
                let v38: Rc<str> = string_slice(&v33.clone(), v36 as i64, v37 as i64);
                let v39: i32 = (v38.clone().len() as i32);
                let v40: i32 = method20(v38.clone(), v39);
                let v41: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v40 as i64);
                let v42: Rc<str> = window_reason_21(v28.clone());
                US4::US4_WaitingWindow(v41.clone(), v42.clone())
            }
        } else {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = job_file_4(v0.clone(), v45.clone());
            let v47: bool = file_exists_14(v46.clone());
            let v54: bool = if v47 {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = job_file_4(v0.clone(), v48.clone());
                let v50: Rc<str> = text_or_empty_8(v49.clone());
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
fn live_word_24(mut v0: US1) -> Rc<str> {
    match &v0 {
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
fn state_word_23(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            live_word_24(v1)
        }
        US0::US0_Settled(v3) => {
            let mut v3: US2 = v3.clone();
            match &v3 {
                US2::US2_JobExpiredInQueue => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                    v14
                }
                US2::US2_JobFinished(v4) => {
                    let mut v4: i32 = *v4;
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
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
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US2::US2_JobSpawnFailed(v8) => {
                    let v8: Rc<str> = v8.clone();
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
fn settled_exit_25(mut v0: US2) -> i32 {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            75i32
        }
        US2::US2_JobFinished(v1) => {
            let mut v1: i32 = *v1;
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
            let mut v2: Rc<str> = v2.clone();
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
fn elapsed_of_26(mut v0: Rc<str>) -> u64 {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<str>, u64, u64, u64) = job_record_of_3(v0);
    let mut v5: bool = &*v1 == "";
    let v8: u64 = if v5 {
        v4
    } else {
        let mut v6: Rc<str> = std::rc::Rc::<str>::from(v1.split(char::from(9u8)).nth(1).unwrap_or("0"));
        decimal_u64_9(v6)
    };
    let v9: bool = v8 > v3;
    if v9 {
        let v10: u64 = v8.wrapping_sub(v3);
        v10
    } else {
        0u64
    }
}
fn method29(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_28(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method29(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    format_real_28(v0)
}
fn file_bytes_30(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn job_placement_31(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_4(v0, v1);
    let mut v3: Rc<str> = text_or_empty_8(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method19(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method20(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: bool = &*v11 == "";
    let v14: bool = if v12 {
        true
    } else {
        let v13: bool = &*v11 == "local";
        v13
    };
    let v49: US7 = if v14 {
        let v15: US5 = US5::US5_Local;
        US7::US7_Some(v15)
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("on "); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v11.starts_with(&*v17);
        if v18 {
            let v19: i32 = (v11.clone().len() as i32);
            let v20: i32 = v19.wrapping_sub(3i32);
            let v21: usize = ((3i32) as usize);
            let v22: i32 = 3i32.wrapping_add(v20);
            let v23: usize = ((v22) as usize);
            let v24: Rc<str> = Rc::<str>::from(&(v11)[v21..v23]);
            let v25: i32 = (v24.clone().len() as i32);
            let v26: i32 = 0i32;
            let v27: i32 = method19(v24.clone(), v25, v26);
            let v28: i32 = v25.wrapping_sub(1i32);
            let v29: Rc<str> = string_slice(&v24, v27 as i64, v28 as i64);
            let v30: i32 = (v29.clone().len() as i32);
            let v31: i32 = method20(v29.clone(), v30);
            let v32: Rc<str> = string_slice(&v29, 0i32 as i64, v31 as i64);
            let v33: bool = &*v32 == "";
            let v36: bool = if v33 {
                true
            } else {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v35: bool = v32.contains(&*v34);
                v35
            };
            let v40: US8 = if v36 {
                US8::US8_None
            } else {
                let v38: US6 = US6::US6_Machine(v32);
                US8::US8_Some(v38)
            };
            match &v40 {
                US8::US8_None => {
                    US7::US7_None
                }
                US8::US8_Some(v41) => {
                    let v41: US6 = v41.clone();
                    let v42: US5 = US5::US5_OnMachine(v41);
                    US7::US7_Some(v42)
                }
            }
        } else {
            US7::US7_None
        }
    };
    match &v49 {
        US7::US7_None => {
            US5::US5_Local
        }
        US7::US7_Some(v50) => {
            let v50: US5 = v50.clone();
            v50
        }
    }
}
fn job_summary_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = job_state_now_2(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_1(v3, v0, v4, v1.clone(), v5);
    let v7: bool = state_settled_17(v2.clone());
    let v10: US4 = if v7 {
        US4::US4_NotWaiting
    } else {
        gate_wait_of_18(v1.clone())
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
        let v13: bool = state_settled_17(v2.clone());
        let v14: bool = v13 == false;
        v14
    } else {
        false
    };
    let v18: Rc<str> = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
        v16
    } else {
        state_word_23(v2.clone())
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
            let v44: i32 = settled_exit_25(v43);
            let v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
            v45
        }
    };
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v47));
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
    let v51: u64 = elapsed_of_26(v1.clone());
    let v52: Rc<str> = decimal_text_27(v51);
    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v52));
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = job_file_4(v1.clone(), v55);
    let v57: u64 = file_bytes_30(v56);
    let v58: Rc<str> = decimal_text_27(v57);
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = job_file_4(v1.clone(), v60);
    let v62: u64 = file_bytes_30(v61);
    let v63: Rc<str> = decimal_text_27(v62);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v65: Rc<str> = concat5_1(v54, v58, v59, v63, v64.clone());
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = job_file_4(v1.clone(), v66);
    let v68: Rc<str> = text_or_empty_8(v67);
    let v69: Rc<str> = std::rc::Rc::<str>::from(v68.split(char::from(9u8)).nth(3).unwrap_or("").trim());
    let v70: bool = &*v69 == "";
    let v74: Rc<str> = if v70 {
        v64.clone()
    } else {
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v71, v69, v72, v64.clone(), v64.clone())
    };
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v77: Rc<str> = job_file_4(v1.clone(), v76);
    let v78: Rc<str> = text_or_empty_8(v77);
    let v79: Rc<str> = concat5_1(v75, v78, v64.clone(), v64.clone(), v64.clone());
    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = job_file_4(v1.clone(), v80);
    let v82: Rc<str> = text_or_empty_8(v81);
    let v83: bool = &*v82 == "";
    let v94: Rc<str> = if v83 {
        v64.clone()
    } else {
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" peak_private_mb="); } LIT.with(|lit| lit.clone()) };
        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
        let v87: Rc<str> = job_file_4(v1.clone(), v86.clone());
        let v88: bool = file_exists_14(v87);
        let v92: Rc<str> = if v88 {
            let v89: Rc<str> = job_file_4(v1.clone(), v86);
            text_or_empty_8(v89)
        } else {
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            v91
        };
        concat5_1(v84, v82, v85, v92, v64.clone())
    };
    let v95: US5 = job_placement_31(v1);
    let v102: Rc<str> = match &v95 {
        US5::US5_Local => {
            v64.clone()
        }
        US5::US5_OnMachine(v96) => {
            let v96: US6 = v96.clone();
            let v98: Rc<str> = match &v96 {
                US6::US6_Machine(v97) => {
                    let v97: Rc<str> = v97.clone();
                    v97
                }
            };
            let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" placement="); } LIT.with(|lit| lit.clone()) };
            let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v98));
            v100
        }
    };
    let v103: Rc<str> = concat5_1(v94, v79, v102, v64.clone(), v64);
    concat5_1(v6, v53, v65, v74, v103)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        job_summary_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_exit_33(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            3i32
        }
        US0::US0_Settled(v2) => {
            let mut v2: US2 = v2.clone();
            settled_exit_25(v2)
        }
    }
}
fn state_detail_34(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Live(v1) => {
            let mut v1: US1 = v1.clone();
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
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
fn state_parts_32(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    let mut v1: US0 = job_state_now_2(v0);
    let mut v2: Rc<str> = state_word_23(v1.clone());
    let mut v3: i32 = state_exit_33(v1.clone());
    let mut v4: Rc<str> = state_detail_34(v1);
    (v2, v3, v4)
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<str>, i32, Rc<str>) {
        state_parts_32(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_summary(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_state_parts(v0: &str) -> (Rc<str>, i32, Rc<str>) {
    closure3()(Rc::<str>::from(v0))
}
