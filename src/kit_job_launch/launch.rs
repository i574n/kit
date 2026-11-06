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
    US0_StartPlain,
    US0_StartHeavy,
    US0_StartCompile,
}
#[derive(Clone)]
enum US1 {
    US1_Launched(Rc<str>, Rc<str>),
    US1_LaunchRefused(Rc<str>),
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
enum US2 {
    US2_SettleHealthy,
    US2_SettleFailed(US3, Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_JobPending,
    US5_JobRunning,
}
#[derive(Clone)]
enum US4 {
    US4_Live(US5),
    US4_Settled(US3),
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn stderr_line_4(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = concat5_3(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_4(v3);
    2i32
}
fn shape_of_word_5(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = "plain" == &*v0;
    if v1 {
        US0::US0_StartPlain
    } else {
        let v3: bool = "heavy" == &*v0;
        if v3 {
            US0::US0_StartHeavy
        } else {
            let v5: bool = "compile" == &*v0;
            if v5 {
                US0::US0_StartCompile
            } else {
                US0::US0_StartPlain
            }
        }
    }
}
fn job_launched_6(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>) -> US1 {
    let v18: Rc<str> = match v0 {
        US0::US0_StartCompile => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v17
        }
        US0::US0_StartHeavy => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US0::US0_StartPlain => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
            v15
        }
    };
    let (mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>) = kit_job_create::kit_job_create(&v18, &v1, &v2, v3, v4, v5, v6, v7, v8, v9, v10, &v11, v12, &v13, &v14);
    let v22: bool = &*v21 != "";
    if v22 {
        US1::US1_LaunchRefused(v21)
    } else {
        US1::US1_Launched(v19, v20)
    }
}
fn state_of_parts_10(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US4 {
    let v3: bool = &*v0 == "pending";
    if v3 {
        let v4: US5 = US5::US5_JobPending;
        US4::US4_Live(v4)
    } else {
        let v6: bool = &*v0 == "running";
        if v6 {
            let v7: US5 = US5::US5_JobRunning;
            US4::US4_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US3 = US3::US3_JobFinished(v1);
                US4::US4_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let mut v13: US3 = US3::US3_JobTimedOut;
                    US4::US4_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US3 = US3::US3_JobStopped;
                        US4::US4_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US3 = US3::US3_JobLost;
                            US4::US4_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US3 = US3::US3_JobMemoryCapped;
                                US4::US4_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US3 = US3::US3_JobExpiredInQueue;
                                    US4::US4_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US3 = US3::US3_JobSkipped(v2);
                                        US4::US4_Settled(v28)
                                    } else {
                                        let v30: US3 = US3::US3_JobSpawnFailed(v2);
                                        US4::US4_Settled(v30)
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
fn shared_state_9(mut v0: Rc<str>) -> US4 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_10(v1, v2, v3)
}
fn settled_within_8(mut v0: Rc<str>, mut v1: u64) -> US4 {
    loop {
        let mut v2: US4 = shared_state_9(v0.clone());
        match &v2 {
            US4::US4_Live(v3) => {
                let v3: US5 = v3.clone();
                let v4: bool = v1 == 0u64;
                if v4 {
                    return v2;
                } else {
                    let v5: u64 = 100u64;
                    std::thread::sleep(std::time::Duration::from_millis(v5));
                    let v6: bool = v1 < 100u64;
                    let v8: u64 = if v6 {
                        0u64
                    } else {
                        let v7: u64 = v1.wrapping_sub(100u64);
                        v7
                    };
                    v1 = v8;
                    continue;
                }
            }
            US4::US4_Settled(v11) => {
                let v11: US3 = v11.clone();
                return v2;
            }
        }
    }
}
fn joined_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_14() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_14();
    joined_12(v1, v0)
}
fn job_file_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_13(v0);
    joined_12(v2, v1)
}
fn text_or_empty_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn method18(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
fn state_detail_19(mut v0: US4) -> Rc<str> {
    match v0 {
        US4::US4_Live(v1) => {
            let mut v1: US5 = v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US4::US4_Settled(v3) => {
            let v3: US3 = v3;
            match v3 {
                US3::US3_JobSkipped(v5) => {
                    let v5: Rc<str> = v5;
                    v5
                }
                US3::US3_JobSpawnFailed(v4) => {
                    let v4: Rc<str> = v4;
                    v4
                }
                _ => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v6
                }
            }
        }
    }
}
fn start_settle_of_7(mut v0: Rc<str>, mut v1: u64) -> US2 {
    let mut v2: US4 = settled_within_8(v0.clone(), v1);
    match v2 {
        US4::US4_Live(v41) => {
            let v41: US5 = v41;
            US2::US2_SettleHealthy
        }
        US4::US4_Settled(v3) => {
            let v3: US3 = v3;
            match &v3 {
                US3::US3_JobFinished(v4) => {
                    let v4: i32 = *v4;
                    let v5: bool = v4 == 0i32;
                    if v5 {
                        US2::US2_SettleHealthy
                    } else {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
                        let v8: Rc<str> = job_file_11(v0, v7);
                        let v9: Rc<str> = text_or_empty_15(v8);
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v11: Option<usize> = v9.find(&*v10);
                        let v12: Option<usize> = method16(v11);
                        let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
                        let mut v14: Option<i32> = v12.map(|x| v13(x));
                        let v16: i32 = v14.unwrap_or(-1i32);
                        let v17: bool = v16 < 0i32;
                        let v21: Rc<str> = if v17 {
                            v9
                        } else {
                            let v18: usize = ((0i32) as usize);
                            let v19: usize = ((v16) as usize);
                            let v20: Rc<str> = Rc::<str>::from(&(v9)[v18..v19]);
                            v20
                        };
                        let v22: i32 = (v21.clone().len() as i32);
                        let v23: i32 = 0i32;
                        let v24: i32 = method17(v21.clone(), v22, v23);
                        let v25: i32 = v22.wrapping_sub(1i32);
                        let v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
                        let v27: i32 = (v26.clone().len() as i32);
                        let v28: i32 = method18(v26.clone(), v27);
                        let v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
                        let v30: US3 = US3::US3_JobFinished(v4);
                        US2::US2_SettleFailed(v30, v29)
                    }
                }
                US3::US3_JobLost => {
                    let v33: US3 = US3::US3_JobLost;
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor ended before the job settled"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_SettleFailed(v33, v34)
                }
                _ => {
                    let v36: US4 = US4::US4_Settled(v3.clone());
                    let v37: Rc<str> = state_detail_19(v36);
                    US2::US2_SettleFailed(v3, v37)
                }
            }
        }
    }
}
fn settled_exit_20(mut v0: US3) -> i32 {
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
            let v3: Rc<str> = v3;
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
fn method22(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_21(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_24(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method22(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_23(mut v0: u64) -> Rc<str> {
    format_real_24(v0)
}
fn stdout_line_25(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn job_started_1(mut v0: bool) -> (i32, Rc<str>) {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool) = kit_job_start_args::kit_job_start_args();
    let v16: bool = v1 == false;
    if v16 {
        let v17: i32 = rejected_2(v2);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v17, v18)
    } else {
        let v19: US0 = shape_of_word_5(v3);
        let v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v13 as usize).unwrap_or_default());
        let v21: i32 = v13.wrapping_add(2i32);
        let v22: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v21 as usize).collect::<Vec<_>>().join("
"));
        let v23: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
        let v24: US1 = job_launched_6(v19.clone(), v4.clone(), v5.clone(), v0, v6, v7, v8, v9, v10, v11, v15, v20.clone(), v14, v22, v23);
        match v24 {
            US1::US1_LaunchRefused(v25) => {
                let v25: Rc<str> = v25;
                let v26: i32 = rejected_2(v25);
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v26, v27)
            }
            US1::US1_Launched(v28, v29) => {
                let v28: Rc<str> = v28;
                let v29: Rc<str> = v29;
                let v30: bool = v0 == false;
                let v32: bool = if v30 {
                    let v31: bool = &*v4 == "";
                    v31
                } else {
                    false
                };
                let v34: bool = if v32 {
                    match &v19 {
                        US0::US0_StartPlain => {
                            true
                        }
                        _ => {
                            false
                        }
                    }
                } else {
                    false
                };
                let v36: bool = if v34 {
                    let v35: bool = v12 > 0u64;
                    v35
                } else {
                    false
                };
                let v39: US2 = if v36 {
                    start_settle_of_7(v28.clone(), v12)
                } else {
                    US2::US2_SettleHealthy
                };
                match v39 {
                    US2::US2_SettleFailed(v40, v41) => {
                        let v40: US3 = v40;
                        let v41: Rc<str> = v41;
                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start failed id="); } LIT.with(|lit| lit.clone()) }, v28.clone()));
                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state="); } LIT.with(|lit| lit.clone()) }));
                        let v55: Rc<str> = match &v40 {
                            US3::US3_JobExpiredInQueue => {
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                                v54
                            }
                            US3::US3_JobFinished(v44) => {
                                let v44: i32 = *v44;
                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
                                v45
                            }
                            US3::US3_JobLost => {
                                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                                v52
                            }
                            US3::US3_JobMemoryCapped => {
                                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                                v53
                            }
                            US3::US3_JobSkipped(v50) => {
                                let v50: Rc<str> = v50.clone();
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                v51
                            }
                            US3::US3_JobSpawnFailed(v48) => {
                                let v48: Rc<str> = v48.clone();
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
                                v49
                            }
                            US3::US3_JobStopped => {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                                v47
                            }
                            US3::US3_JobTimedOut => {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                                v46
                            }
                        };
                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v55));
                        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) }));
                        let v58: i32 = settled_exit_20(v40.clone());
                        let v59: Rc<str> = format_real_21(v58);
                        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v59));
                        let v61: bool = &*v41 == "";
                        let v65: Rc<str> = if v61 {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v62
                        } else {
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" detail=\""); } LIT.with(|lit| lit.clone()) }, v41));
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            v64
                        };
                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v65));
                        stderr_line_4(v66);
                        let v67: i32 = settled_exit_20(v40);
                        (v67, v28)
                    }
                    US2::US2_SettleHealthy => {
                        let v70: Rc<str> = if v0 {
                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job run started id="); } LIT.with(|lit| lit.clone()) };
                            v68
                        } else {
                            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start ok id="); } LIT.with(|lit| lit.clone()) };
                            v69
                        };
                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v28.clone()));
                        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) }));
                        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v29));
                        let v74: Rc<str> = decimal_text_23(v14);
                        let v75: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) }, v74));
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) }));
                        let v80: Rc<str> = match v19 {
                            US0::US0_StartCompile => {
                                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                v79
                            }
                            US0::US0_StartHeavy => {
                                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                v78
                            }
                            US0::US0_StartPlain => {
                                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                v77
                            }
                        };
                        let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v80));
                        let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) }, v20));
                        let v83: bool = &*v4 == "";
                        let v88: Rc<str> = if v83 {
                            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v84
                        } else {
                            let v85: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) }, v5));
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_mode="); } LIT.with(|lit| lit.clone()) }));
                            let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v4));
                            v87
                        };
                        let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v88));
                        let v90: bool = v6 == 0u64;
                        let v94: Rc<str> = if v90 {
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v91
                        } else {
                            let v92: Rc<str> = decimal_text_23(v6);
                            let v93: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) }, v92));
                            v93
                        };
                        let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v94));
                        let v96: bool = v7 == 0u64;
                        let v100: Rc<str> = if v96 {
                            let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v97
                        } else {
                            let v98: Rc<str> = decimal_text_23(v7);
                            let v99: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" slot_within_min="); } LIT.with(|lit| lit.clone()) }, v98));
                            v99
                        };
                        let v101: bool = v8 == 0u64;
                        let v105: Rc<str> = if v101 {
                            let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v102
                        } else {
                            let v103: Rc<str> = decimal_text_23(v8);
                            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done_within_min="); } LIT.with(|lit| lit.clone()) }, v103));
                            v104
                        };
                        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v105));
                        let v107: bool = v9 == 0u64;
                        let v114: Rc<str> = if v107 {
                            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v108
                        } else {
                            let v109: Rc<str> = decimal_text_23(v9);
                            let v110: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_hold_min="); } LIT.with(|lit| lit.clone()) }, v109));
                            let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_wait_min="); } LIT.with(|lit| lit.clone()) }));
                            let v112: Rc<str> = decimal_text_23(v10);
                            let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
                            v113
                        };
                        let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v114));
                        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v115));
                        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v116));
                        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v117));
                        stdout_line_25(v118);
                        if v0 {
                            let v119: i32 = kit_job_supervise::kit_job_supervise(&*v28);
                            (v119, v28)
                        } else {
                            (0i32, v28)
                        }
                    }
                }
            }
        }
    }
}
fn start_answered_0(mut v0: bool) -> Rc<RefCell<Vec<(i32, Rc<str>)>>> {
    let (mut v1, mut v2): (i32, Rc<str>) = job_started_1(v0);
    let v3: Rc<RefCell<Vec<(i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v1, v2)]));
    v3
}
fn closure0() -> Rc<dyn Fn(bool) -> Rc<RefCell<Vec<(i32, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool) -> Rc<RefCell<Vec<(i32, Rc<str>)>>>> = Rc::new(move |mut v0: bool| -> Rc<RefCell<Vec<(i32, Rc<str>)>>> {
        start_answered_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_26(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: US0 = US0::US0_StartPlain;
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: bool = false;
    let v7: u64 = 0u64;
    let v8: u64 = 0u64;
    let v9: u64 = 0u64;
    let v10: u64 = 0u64;
    let v11: u64 = 0u64;
    let v12: u64 = 0u64;
    let mut v13: bool = false;
    let mut v14: US1 = job_launched_6(v4, v5.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v0, v1, v2, v3);
    match v14 {
        US1::US1_LaunchRefused(v20) => {
            let v20: Rc<str> = v20;
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) }, v20));
            v21
        }
        US1::US1_Launched(v15, v16) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = v16;
            let v17: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v17));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v16));
            v19
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_26(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_start(v0: bool) -> Rc<RefCell<Vec<(i32, Rc<str>)>>> {
    closure0()(v0)
}
pub fn kit_job_launched_text(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
use kit_job_create::SpiralShape_97df63ebabc3 as Mut0;
