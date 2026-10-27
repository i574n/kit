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
    US1_AfterAny,
    US1_AfterOk,
}
#[derive(Clone)]
enum US2 {
    US2_PredecessorLive,
    US2_PredecessorSettled(Rc<str>, i32),
}
#[derive(Clone)]
enum US0 {
    US0_NoChain,
    US0_Chained(Rc<str>, US1, US2),
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
    US6_KeepWaiting,
    US6_StartSupervisor,
    US6_SettleStopped(Rc<str>),
    US6_SettleSkipped(Rc<str>),
}
struct Mut0 { l0: Rc<str> }
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_4(v0);
    joined_3(v2, v1)
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn file_exists_7(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn state_of_parts_11(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US3 {
    let v3: bool = &*v0 == "pending";
    if v3 {
        let v4: US4 = US4::US4_JobPending;
        US3::US3_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
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
                    let v13: US5 = US5::US5_JobTimedOut;
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
fn shared_state_10(mut v0: Rc<str>) -> US3 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_11(v1, v2, v3)
}
fn state_settled_12(mut v0: US3) -> bool {
    match &v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1.clone();
            false
        }
        US3::US3_Settled(v2) => {
            let v2: US5 = v2.clone();
            true
        }
    }
}
fn live_word_14(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US4::US4_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn settled_word_15(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_JobExpiredInQueue => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US5::US5_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US5::US5_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US5::US5_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US5::US5_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US5::US5_JobStopped => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_JobTimedOut => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn state_word_13(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1.clone();
            live_word_14(v1)
        }
        US3::US3_Settled(v3) => {
            let v3: US5 = v3.clone();
            settled_word_15(v3)
        }
    }
}
fn settled_exit_17(mut v0: US5) -> i32 {
    match &v0 {
        US5::US5_JobExpiredInQueue => {
            75i32
        }
        US5::US5_JobFinished(v1) => {
            let v1: i32 = *v1;
            v1
        }
        US5::US5_JobLost => {
            125i32
        }
        US5::US5_JobMemoryCapped => {
            137i32
        }
        US5::US5_JobSkipped(v3) => {
            let v3: Rc<str> = v3.clone();
            125i32
        }
        US5::US5_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2.clone();
            125i32
        }
        US5::US5_JobStopped => {
            143i32
        }
        US5::US5_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_16(mut v0: US3) -> i32 {
    match &v0 {
        US3::US3_Live(v1) => {
            let v1: US4 = v1.clone();
            3i32
        }
        US3::US3_Settled(v2) => {
            let v2: US5 = v2.clone();
            settled_exit_17(v2)
        }
    }
}
fn queued_of_1(mut v0: Rc<str>) -> (Rc<str>, bool, US0) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_2(v0.clone(), v1.clone());
    let v3: Rc<str> = text_or_empty_6(v2.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = job_file_2(v0.clone(), v4.clone());
    let mut v6: bool = file_exists_7(v5.clone());
    let v7: bool = &*v3 == "";
    let (mut v50, mut v51, mut v52): (Rc<str>, bool, US0) = if v7 {
        let v8: US0 = US0::US0_NoChain;
        (v0.clone(), v6, v8.clone())
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v11: i32 = (v10.clone().borrow().len() as i32);
        let v12: bool = 0i32 < v11;
        let v23: Rc<str> = if v12 {
            let v13: Rc<str> = v10.clone().borrow()[0i32 as usize].clone();
            let v14: i32 = (v13.clone().len() as i32);
            let v15: i32 = 0i32;
            let v16: i32 = method8(v13.clone(), v14, v15);
            let v17: i32 = v14.wrapping_sub(1i32);
            let v18: Rc<str> = string_slice(&v13.clone(), v16 as i64, v17 as i64);
            let v19: i32 = (v18.clone().len() as i32);
            let v20: i32 = method9(v18.clone(), v19);
            let v21: Rc<str> = string_slice(&v18.clone(), 0i32 as i64, v20 as i64);
            v21.clone()
        } else {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v22.clone()
        };
        let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v25: i32 = (v24.clone().borrow().len() as i32);
        let v26: bool = 1i32 < v25;
        let v37: Rc<str> = if v26 {
            let v27: Rc<str> = v24.clone().borrow()[1i32 as usize].clone();
            let v28: i32 = (v27.clone().len() as i32);
            let v29: i32 = 0i32;
            let v30: i32 = method8(v27.clone(), v28, v29);
            let v31: i32 = v28.wrapping_sub(1i32);
            let v32: Rc<str> = string_slice(&v27.clone(), v30 as i64, v31 as i64);
            let v33: i32 = (v32.clone().len() as i32);
            let v34: i32 = method9(v32.clone(), v33);
            let v35: Rc<str> = string_slice(&v32.clone(), 0i32 as i64, v34 as i64);
            v35.clone()
        } else {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v36.clone()
        };
        let v38: bool = &*v37 == "ok";
        let v41: US1 = if v38 {
            US1::US1_AfterOk
        } else {
            US1::US1_AfterAny
        };
        let v42: US3 = shared_state_10(v23.clone());
        let v43: bool = state_settled_12(v42.clone());
        let v48: US2 = if v43 {
            let v44: Rc<str> = state_word_13(v42.clone());
            let v45: i32 = state_exit_16(v42.clone());
            US2::US2_PredecessorSettled(v44.clone(), v45)
        } else {
            US2::US2_PredecessorLive
        };
        let v49: US0 = US0::US0_Chained(v23.clone(), v41.clone(), v48.clone());
        (v0.clone(), v6, v49.clone())
    };
    (v50.clone(), v51, v52.clone())
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
fn now_ms_21() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn text_written_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn settled_in_queue_20(mut v0: Rc<str>, mut v1: US6) -> () {
    match &v1 {
        US6::US6_SettleSkipped(v16) => {
            let v16: Rc<str> = v16.clone();
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = job_file_2(v0.clone(), v17.clone());
            let v19: u64 = now_ms_21();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v21: US5 = US5::US5_JobSkipped(v20.clone());
            let v22: Rc<str> = settled_word_15(v21.clone());
            let v23: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v24: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v16, &*v26, &*v20].concat());
            let v28: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v25, &*v24, &*v25, &*v27].concat());
            let v29: Rc<str> = text_written_22(v18.clone(), v28.clone());
            ()
        }
        US6::US6_SettleStopped(v2) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = job_file_2(v0.clone(), v3.clone());
            let v5: u64 = now_ms_21();
            let mut v6: US5 = US5::US5_JobStopped;
            let v7: Rc<str> = settled_word_15(v6.clone());
            let v8: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v10, &*v2, &*v11, &*v12].concat());
            let v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v10, &*v9, &*v10, &*v13].concat());
            let v15: Rc<str> = text_written_22(v4.clone(), v14.clone());
            ()
        }
        _ => {
            ()
        }
    }
}
fn queue_decide_0(mut v0: Rc<str>) -> i32 {
    let (mut v1, mut v2, mut v3): (Rc<str>, bool, US0) = queued_of_1(v0.clone());
    let v40: US6 = if v2 {
        let v9: Rc<str> = match &v3 {
            US0::US0_Chained(v5, v6, v7) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: US1 = v6.clone();
                let v7: US2 = v7.clone();
                v5.clone()
            }
            US0::US0_NoChain => {
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v4.clone()
            }
        };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request while waiting for "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
        US6::US6_SettleStopped(v11.clone())
    } else {
        match &v3 {
            US0::US0_Chained(v14, v15, v16) => {
                let v14: Rc<str> = v14.clone();
                let v15: US1 = v15.clone();
                let v16: US2 = v16.clone();
                match &v16 {
                    US2::US2_PredecessorLive => {
                        US6::US6_KeepWaiting
                    }
                    _ => {
                        match &v15 {
                            US1::US1_AfterAny => {
                                US6::US6_StartSupervisor
                            }
                            US1::US1_AfterOk => {
                                match &v16 {
                                    US2::US2_PredecessorSettled(v19, v20) => {
                                        let v19: Rc<str> = v19.clone();
                                        let v20: i32 = *v20;
                                        let v21: bool = v20 == 0i32;
                                        if v21 {
                                            US6::US6_StartSupervisor
                                        } else {
                                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok: "); } LIT.with(|lit| lit.clone()) };
                                            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v14));
                                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ended "); } LIT.with(|lit| lit.clone()) };
                                            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v25)), v19));
                                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit "); } LIT.with(|lit| lit.clone()) };
                                            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                                            let v30: Rc<str> = format_real_18(v20);
                                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                                            US6::US6_SettleSkipped(v31.clone())
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                }
            }
            US0::US0_NoChain => {
                US6::US6_StartSupervisor
            }
        }
    };
    match &v40 {
        US6::US6_KeepWaiting => {
            1i32
        }
        US6::US6_StartSupervisor => {
            0i32
        }
        _ => {
            settled_in_queue_20(v0.clone(), v40.clone());
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        queue_decide_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_queue_decide(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
