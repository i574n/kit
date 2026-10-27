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
    US0_PlainStart,
    US0_HeavyStart,
    US0_CompileStart,
}
#[derive(Clone)]
enum US1 {
    US1_Launched(Rc<str>, Rc<str>),
    US1_LaunchRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_SettleHealthy,
    US2_SettleFailed(i32, Rc<str>),
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
struct Mut0 { l0: Rc<str> }
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_2(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_3(v3.clone());
    2i32
}
fn shape_of_word_4(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "heavy";
    if v1 {
        US0::US0_HeavyStart
    } else {
        let v3: bool = &*v0 == "compile";
        if v3 {
            US0::US0_CompileStart
        } else {
            US0::US0_PlainStart
        }
    }
}
fn job_launched_5(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: u64, mut v12: Rc<str>, mut v13: Rc<str>) -> US1 {
    let v19: Rc<str> = match &v0 {
        US0::US0_CompileStart => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v16.clone()
        }
        US0::US0_HeavyStart => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        }
        US0::US0_PlainStart => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
    };
    let (mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>) = kit_job_create::kit_job_create(&*v19, &*v1, &*v2, v3, v4, v5, v6, v7, v8, v9, &*v10, v11, &*v12, &*v13);
    let v23: bool = &*v22 != "";
    if v23 {
        US1::US1_LaunchRefused(v22.clone())
    } else {
        US1::US1_Launched(v20.clone(), v21.clone())
    }
}
fn state_of_parts_9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US3 {
    let v3: bool = &*v0 == "pending";
    if v3 {
        let v4: US4 = US4::US4_JobPending;
        US3::US3_Live(v4)
    } else {
        let v6: bool = &*v0 == "running";
        if v6 {
            let v7: US4 = US4::US4_JobRunning;
            US3::US3_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US5 = US5::US5_JobFinished(v1);
                US3::US3_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let mut v13: US5 = US5::US5_JobTimedOut;
                    US3::US3_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US5 = US5::US5_JobStopped;
                        US3::US3_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US5 = US5::US5_JobLost;
                            US3::US3_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US5 = US5::US5_JobMemoryCapped;
                                US3::US3_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US5 = US5::US5_JobExpiredInQueue;
                                    US3::US3_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US5 = US5::US5_JobSkipped(v2.clone());
                                        US3::US3_Settled(v28)
                                    } else {
                                        let v30: US5 = US5::US5_JobSpawnFailed(v2);
                                        US3::US3_Settled(v30)
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
fn shared_state_8(mut v0: Rc<str>) -> US3 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_9(v1, v2, v3)
}
fn settled_within_7(mut v0: Rc<str>, mut v1: u64) -> US3 {
    loop {
        let v2: US3 = shared_state_8(v0.clone());
        match &v2 {
            US3::US3_Live(v3) => {
                let v3: US4 = v3.clone();
                let v4: bool = v1 == 0u64;
                if v4 {
                    return v2.clone();
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
                    (v0, v1) = (v0.clone(), v8);
                    continue;
                }
            }
            US3::US3_Settled(v11) => {
                let v11: US5 = v11.clone();
                return v2.clone();
            }
        }
    }
}
fn joined_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_13() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_12(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_13();
    joined_11(v1, v0)
}
fn job_file_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_12(v0);
    joined_11(v2, v1)
}
fn text_or_empty_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method15(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn start_settle_of_6(mut v0: Rc<str>, mut v1: u64) -> US2 {
    let v2: US3 = settled_within_7(v0.clone(), v1);
    match &v2 {
        US3::US3_Settled(v3) => {
            let v3: US5 = v3.clone();
            match &v3 {
                US5::US5_JobFinished(v4) => {
                    let v4: i32 = *v4;
                    let v5: bool = v4 == 0i32;
                    if v5 {
                        US2::US2_SettleHealthy
                    } else {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
                        let v8: Rc<str> = job_file_10(v0.clone(), v7.clone());
                        let v9: Rc<str> = text_or_empty_14(v8.clone());
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v11: Option<usize> = v9.find(&*v10);
                        let v12: Option<usize> = method15(v11.clone());
                        let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
                        let mut v14: Option<i32> = v12.map(|x| v13(x));
                        let v16: i32 = v14.unwrap_or(-1i32);
                        let v17: bool = v16 < 0i32;
                        let v21: Rc<str> = if v17 {
                            v9.clone()
                        } else {
                            let v18: usize = ((0i32) as usize);
                            let v19: usize = ((v16) as usize);
                            let v20: Rc<str> = Rc::<str>::from(&(v9)[v18..v19]);
                            v20.clone()
                        };
                        let v22: i32 = (v21.clone().len() as i32);
                        let v23: i32 = 0i32;
                        let v24: i32 = method16(v21.clone(), v22, v23);
                        let v25: i32 = v22.wrapping_sub(1i32);
                        let v26: Rc<str> = string_slice(&v21.clone(), v24 as i64, v25 as i64);
                        let v27: i32 = (v26.clone().len() as i32);
                        let v28: i32 = method17(v26.clone(), v27);
                        let v29: Rc<str> = string_slice(&v26.clone(), 0i32 as i64, v28 as i64);
                        US2::US2_SettleFailed(v4, v29.clone())
                    }
                }
                US5::US5_JobLost => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor ended before the job settled"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_SettleFailed(125i32, v34.clone())
                }
                US5::US5_JobSpawnFailed(v32) => {
                    let v32: Rc<str> = v32.clone();
                    US2::US2_SettleFailed(127i32, v32.clone())
                }
                _ => {
                    US2::US2_SettleHealthy
                }
            }
        }
        _ => {
            US2::US2_SettleHealthy
        }
    }
}
fn method19(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_20(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn stdout_line_21(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn job_started_1(mut v0: bool) -> (i32, Rc<str>) {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64) = kit_job_start_args::kit_job_start_args();
    let v15: bool = v1 == false;
    if v15 {
        let v16: i32 = rejected_2(v2.clone());
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v16, v17.clone())
    } else {
        let v18: US0 = shape_of_word_4(v3.clone());
        let v19: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v13 as usize).unwrap_or_default());
        let v20: i32 = v13.wrapping_add(2i32);
        let v21: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v20 as usize).collect::<Vec<_>>().join("
"));
        let v22: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
        let v23: US1 = job_launched_5(v18.clone(), v4.clone(), v5.clone(), v0, v6, v7, v8, v9, v10, v11, v19.clone(), v14, v21.clone(), v22.clone());
        match &v23 {
            US1::US1_LaunchRefused(v24) => {
                let v24: Rc<str> = v24.clone();
                let v25: i32 = rejected_2(v24.clone());
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v25, v26.clone())
            }
            US1::US1_Launched(v27, v28) => {
                let v27: Rc<str> = v27.clone();
                let v28: Rc<str> = v28.clone();
                let v29: bool = v0 == false;
                let v31: bool = if v29 {
                    let v30: bool = &*v4 == "";
                    v30
                } else {
                    false
                };
                let v33: bool = if v31 {
                    match &v18 {
                        US0::US0_PlainStart => {
                            true
                        }
                        _ => {
                            false
                        }
                    }
                } else {
                    false
                };
                let v35: bool = if v33 {
                    let v34: bool = v12 > 0u64;
                    v34
                } else {
                    false
                };
                let v38: US2 = if v35 {
                    start_settle_of_6(v27.clone(), v12)
                } else {
                    US2::US2_SettleHealthy
                };
                match &v38 {
                    US2::US2_SettleFailed(v39, v40) => {
                        let v39: i32 = *v39;
                        let v40: Rc<str> = v40.clone();
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start failed id="); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v27));
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
                        let v45: Rc<str> = format_real_18(v39);
                        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr="); } LIT.with(|lit| lit.clone()) };
                        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v47)), v40));
                        stderr_line_3(v49.clone());
                        (v39, v27.clone())
                    }
                    US2::US2_SettleHealthy => {
                        let v52: Rc<str> = if v0 {
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job run started id="); } LIT.with(|lit| lit.clone()) };
                            v50.clone()
                        } else {
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start ok id="); } LIT.with(|lit| lit.clone()) };
                            v51.clone()
                        };
                        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v27));
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v53, v54)), v28));
                        let v57: Rc<str> = decimal_text_20(v14);
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
                        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                        let v67: Rc<str> = match &v18 {
                            US0::US0_CompileStart => {
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                v64.clone()
                            }
                            US0::US0_HeavyStart => {
                                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                v63.clone()
                            }
                            US0::US0_PlainStart => {
                                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                v62.clone()
                            }
                        };
                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v67));
                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
                        let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v19));
                        let v71: bool = &*v4 == "";
                        let v78: Rc<str> = if v71 {
                            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v72.clone()
                        } else {
                            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                            let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v5));
                            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_mode="); } LIT.with(|lit| lit.clone()) };
                            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v74, v75)), v4));
                            v77.clone()
                        };
                        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v78));
                        let v80: bool = v6 == 0u64;
                        let v85: Rc<str> = if v80 {
                            let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v81.clone()
                        } else {
                            let v82: Rc<str> = decimal_text_20(v6);
                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
                            let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v82));
                            v84.clone()
                        };
                        let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v85));
                        let v87: bool = v7 == 0u64;
                        let v92: Rc<str> = if v87 {
                            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v88.clone()
                        } else {
                            let v89: Rc<str> = decimal_text_20(v7);
                            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" slot_within_min="); } LIT.with(|lit| lit.clone()) };
                            let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v89));
                            v91.clone()
                        };
                        let v93: bool = v8 == 0u64;
                        let v98: Rc<str> = if v93 {
                            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v94.clone()
                        } else {
                            let v95: Rc<str> = decimal_text_20(v8);
                            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done_within_min="); } LIT.with(|lit| lit.clone()) };
                            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v95));
                            v97.clone()
                        };
                        let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v98));
                        let v100: bool = v9 == 0u64;
                        let v109: Rc<str> = if v100 {
                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v101.clone()
                        } else {
                            let v102: Rc<str> = decimal_text_20(v9);
                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_hold_min="); } LIT.with(|lit| lit.clone()) };
                            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v102));
                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_wait_min="); } LIT.with(|lit| lit.clone()) };
                            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                            let v107: Rc<str> = decimal_text_20(v10);
                            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
                            v108.clone()
                        };
                        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v56, Rc::<str>::from(format!("{}{}", v68, Rc::<str>::from(format!("{}{}", v86, Rc::<str>::from(format!("{}{}", v99, v109))))))));
                        stdout_line_21(v113.clone());
                        if v0 {
                            let v114: i32 = kit_job_supervise::kit_job_supervise(&*v27);
                            (v114, v27.clone())
                        } else {
                            (0i32, v27.clone())
                        }
                    }
                }
            }
        }
    }
}
fn started_text_0(mut v0: bool) -> Rc<str> {
    let (mut v1, mut v2): (i32, Rc<str>) = job_started_1(v0);
    let v3: Rc<str> = std::rc::Rc::<str>::from((v1).to_string().as_str());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v2));
    v6.clone()
}
fn closure0() -> Rc<dyn Fn(bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool) -> Rc<str>> = Rc::new(move |mut v0: bool| -> Rc<str> {
        started_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_22(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: US0 = US0::US0_PlainStart;
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: bool = false;
    let v7: u64 = 0u64;
    let v8: u64 = 0u64;
    let v9: u64 = 0u64;
    let v10: u64 = 0u64;
    let v11: u64 = 0u64;
    let v12: u64 = 0u64;
    let mut v13: US1 = job_launched_5(v4.clone(), v5.clone(), v5.clone(), v6, v7, v8, v9, v10, v11, v12, v0.clone(), v1, v2.clone(), v3.clone());
    match &v13 {
        US1::US1_LaunchRefused(v19) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
            v21.clone()
        }
        US1::US1_Launched(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v16)), v15));
            v18.clone()
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_22(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_start(v0: bool) -> Rc<str> {
    closure0()(v0)
}
pub fn kit_job_launched_text(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
