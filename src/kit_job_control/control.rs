#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
struct Mut0 { l0: Rc<str> }
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
    US4_StopRequested,
    US4_StopRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_JobAmendAllowed(u64),
    US5_JobAmendRefused(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_UnsettledPass,
    US6_SettledPass,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    joined_4(v1, v0)
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_8() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_7() -> i32 {
    let mut v0: i32 = command_line_argument_count_8();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_9(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method10(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match &v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn shared_summary_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_job_state::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_12(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn method15(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_13(mut v0: u64) -> Rc<str> {
    format_real_14(v0)
}
fn job_file_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_3(v0);
    joined_4(v2, v1)
}
fn last_lines_17(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2
}
fn job_tailed_0() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_3(v1.clone());
        directory_exists_6(v4.clone())
    } else {
        false
    };
    if v6 {
        let v7: i32 = argument_count_7();
        let v8: bool = v7 > 4i32;
        let v12: u64 = if v8 {
            let v9: i32 = 4i32;
            let v10: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v9 as usize).unwrap_or_default());
            decimal_u64_9(v10.clone())
        } else {
            20u64
        };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = shared_summary_11(v13.clone(), v1.clone());
        stdout_line_12(v14.clone());
        let v15: Rc<str> = decimal_text_13(v12);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stdout (last "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)\n"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = job_file_16(v1.clone(), v20.clone());
        let v22: Rc<str> = last_lines_17(v21.clone(), v12);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v22));
        stdout_line_12(v23.clone());
        let v24: Rc<str> = decimal_text_13(v12);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--- stderr (last "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v25, v24)), v18));
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = job_file_16(v1.clone(), v28.clone());
        let v30: Rc<str> = last_lines_17(v29.clone(), v12);
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v30));
        stdout_line_12(v31.clone());
        0i32
    } else {
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
        rejected_1(v33.clone())
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_tailed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn state_of_parts_21(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US1 {
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
                                        let v28: US3 = US3::US3_JobSkipped(v2.clone());
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
fn shared_state_20(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_21(v1, v2, v3)
}
fn settled_word_22(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_JobExpiredInQueue => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US3::US3_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US3::US3_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US3::US3_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
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
fn now_ms_23() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn text_written_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn state_settled_26(mut v0: US1) -> bool {
    match &v0 {
        US1::US1_Live(v1) => {
            let v1: US2 = v1.clone();
            false
        }
        US1::US1_Settled(v2) => {
            let v2: US3 = v2.clone();
            true
        }
    }
}
fn process_alive_27(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn waited_25(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US1 {
    loop {
        let mut v3: US1 = shared_state_20(v0.clone());
        let mut v4: bool = state_settled_26(v3.clone());
        let v7: bool = if v4 {
            true
        } else {
            let v5: u64 = now_ms_23();
            let mut v6: bool = v5 > v1;
            v6
        };
        let v10: bool = if v7 {
            true
        } else {
            let v8: bool = process_alive_27(v2);
            let v9: bool = v8 == false;
            v9
        };
        if v10 {
            return v3.clone();
        } else {
            let v11: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v11));
            (v0, v1, v2) = (v0.clone(), v1, v2);
            continue;
        }
    }
}
fn stop_text_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        let mut v3: Rc<str> = job_directory_3(v0.clone());
        directory_exists_6(v3.clone())
    } else {
        false
    };
    let mut v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
        v8.clone()
    } else {
        let v9: US1 = shared_state_20(v0.clone());
        let v20: US4 = match &v9 {
            US1::US1_Live(v10) => {
                let v10: US2 = v10.clone();
                US4::US4_StopRequested
            }
            US1::US1_Settled(v12) => {
                let v12: US3 = v12.clone();
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("is already "); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = settled_word_22(v12.clone());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; nothing to stop"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v16].concat());
                US4::US4_StopRefused(v17.clone())
            }
        };
        match &v20 {
            US4::US4_StopRefused(v21) => {
                let v21: Rc<str> = v21.clone();
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: job "); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v0));
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v24)), v21));
                v26.clone()
            }
            US4::US4_StopRequested => {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = job_file_16(v0.clone(), v27.clone());
                let v29: u64 = now_ms_23();
                let v30: Rc<str> = decimal_text_13(v29);
                let v31: Rc<str> = text_written_24(v28.clone(), v30.clone());
                let v32: u64 = now_ms_23();
                let v33: u64 = v32.wrapping_add(15000u64);
                let v34: u64 = 0u64;
                let v35: US1 = waited_25(v0.clone(), v33, v34);
                let v36: bool = state_settled_26(v35.clone());
                if v36 {
                    shared_summary_11(v27.clone(), v0.clone())
                } else {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: stop: job "); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v0));
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" did not settle within 15 s; its supervisor may predate job stop"); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    v41.clone()
                }
            }
        }
    }
}
fn job_stopped_18() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = stop_text_19(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.starts_with(&*v3);
    if v4 {
        let v5: Rc<str> = std::rc::Rc::<str>::from(&(v2)[7..]);
        rejected_1(v5.clone())
    } else {
        stdout_line_12(v2.clone());
        0i32
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_stopped_18()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_30(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn amend_verdict_of_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> US5 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0.starts_with(&*v3);
    let v7: bool = if v4 {
        let v5: Rc<str> = job_directory_3(v0.clone());
        directory_exists_6(v5.clone())
    } else {
        false
    };
    let v8: bool = v7 == false;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v0));
        US5::US5_JobAmendRefused(v10.clone())
    } else {
        let v12: bool = &*v1 != "--timeout";
        if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expects --timeout <ms>"); } LIT.with(|lit| lit.clone()) };
            US5::US5_JobAmendRefused(v13.clone())
        } else {
            let v15: bool = v2 == 0u64;
            if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--timeout needs a positive number of milliseconds"); } LIT.with(|lit| lit.clone()) };
                US5::US5_JobAmendRefused(v16.clone())
            } else {
                let v18: US1 = shared_state_20(v0.clone());
                let v19: bool = state_settled_26(v18.clone());
                if v19 {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v0));
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already settled"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                    US5::US5_JobAmendRefused(v23.clone())
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = job_file_16(v0.clone(), v25.clone());
                    let v27: bool = file_exists_30(v26.clone());
                    if v27 {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v0));
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has started; its supervisor read the limit when the child started"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                        US5::US5_JobAmendRefused(v31.clone())
                    } else {
                        US5::US5_JobAmendAllowed(v2)
                    }
                }
            }
        }
    }
}
fn text_or_empty_31(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn job_amended_28() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: i32 = 4i32;
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
    let mut v4: i32 = 5i32;
    let v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v4 as usize).unwrap_or_default());
    let mut v6: u64 = decimal_u64_9(v5.clone());
    let v7: US5 = amend_verdict_of_29(v1.clone(), v3.clone(), v6);
    match &v7 {
        US5::US5_JobAmendAllowed(v12) => {
            let v12: u64 = *v12;
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = job_file_16(v1.clone(), v13.clone());
            let v15: Rc<str> = text_or_empty_31(v14.clone());
            let v16: u64 = decimal_u64_9(v15.clone());
            let v17: Rc<str> = job_file_16(v1.clone(), v13.clone());
            let v18: Rc<str> = decimal_text_13(v12);
            let v19: Rc<str> = text_written_24(v17.clone(), v18.clone());
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = job_file_16(v1.clone(), v20.clone());
            let v22: bool = file_exists_30(v21.clone());
            let v39: Rc<str> = if v22 {
                let v23: u64 = v16.wrapping_div(60000u64);
                let v24: u64 = v23.wrapping_add(1u64);
                let v25: Rc<str> = decimal_text_13(v24);
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-MaxMinutes "); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                let v30: u64 = v12.wrapping_div(60000u64);
                let v31: u64 = v30.wrapping_add(1u64);
                let v32: Rc<str> = decimal_text_13(v31);
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v32)), v28));
                let v35: Rc<str> = text_or_empty_31(v21.clone());
                let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v29, &*v34));
                text_written_24(v21.clone(), v36.clone())
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v38.clone()
            };
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job amend ok id="); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
            let v44: Rc<str> = decimal_text_13(v12);
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was="); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
            let v48: Rc<str> = decimal_text_13(v16);
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
            stdout_line_12(v49.clone());
            0i32
        }
        US5::US5_JobAmendRefused(v8) => {
            let v8: Rc<str> = v8.clone();
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: "); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v8));
            rejected_1(v10.clone())
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_amended_28()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn listed_33(mut v0: Rc<str>, mut v1: US6, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = &*v0 == "";
        let v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 == 0u64;
            v4
        };
        if v5 {
            return v2;
        } else {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
            let v7: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
            let v8: US1 = shared_state_20(v6.clone());
            let v13: bool = match &v1 {
                US6::US6_SettledPass => {
                    state_settled_26(v8.clone())
                }
                US6::US6_UnsettledPass => {
                    let v9: bool = state_settled_26(v8.clone());
                    let v10: bool = v9 == false;
                    v10
                }
            };
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = shared_summary_11(v14.clone(), v6.clone());
                stdout_line_12(v15.clone());
                let v16: u64 = v2.wrapping_sub(1u64);
                (v0, v1, v2) = (v7.clone(), v1.clone(), v16);
                continue;
            } else {
                (v0, v1, v2) = (v7.clone(), v1.clone(), v2);
                continue;
            }
        }
    }
}
fn job_listed_32() -> i32 {
    let mut v0: i32 = argument_count_7();
    let v1: bool = v0 > 3i32;
    let v5: u64 = if v1 {
        let v2: i32 = 3i32;
        let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
        decimal_u64_9(v3.clone())
    } else {
        20u64
    };
    let mut v6: Rc<str> = jobs_root_5();
    let v7: bool = v5 > 120u64;
    let v8: u64 = if v7 {
        v5
    } else {
        120u64
    };
    let v9: Rc<str> = { let mut names = std::fs::read_dir(&*v6).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v8 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let v10: US6 = US6::US6_UnsettledPass;
    let v11: u64 = 120u64;
    let v12: u64 = listed_33(v9.clone(), v10.clone(), v11);
    let v13: u64 = 120u64.wrapping_sub(v12);
    let v14: bool = v13 < v5;
    let v18: u64 = if v14 {
        let v15: US6 = US6::US6_SettledPass;
        let v16: u64 = v5.wrapping_sub(v13);
        listed_33(v9.clone(), v15.clone(), v16)
    } else {
        0u64
    };
    0i32
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_listed_32()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        stop_text_19(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_34(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = kit_job_launch::kit_job_launched_text(&*v0, v1, &*v2, &*v3);
    v4
}
fn closure6() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_34(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn parent_pid_37() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_36() -> u64 {
    let mut v0: u64 = std::env::var("KIT_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_37()
    } else {
        v0
    }
}
fn settled_text_35(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: u64 = now_ms_23();
    let mut v3: u64 = v2.wrapping_add(v1);
    let mut v4: u64 = watched_caller_36();
    let v5: US1 = waited_25(v0.clone(), v3, v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = shared_summary_11(v6.clone(), v0.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pid="); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = job_file_16(v0.clone(), v10.clone());
    let v12: Rc<str> = text_or_empty_31(v11.clone());
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = job_file_16(v0.clone(), v16.clone());
    let v18: Rc<str> = text_or_empty_31(v17.clone());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v18));
    v19.clone()
}
fn closure7() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        settled_text_35(v0.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_16(v0.clone(), v2.clone());
        last_lines_17(v3.clone(), v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn newest_labeled_38(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v1).into_iter().flatten().flatten().filter(|entry| std::fs::read_to_string(entry.path().join("label")).map(|text| text.trim() == &*v0).unwrap_or(false)).map(|entry| entry.file_name().to_string_lossy().into_owned()).max_by_key(|name| name.split(char::from(45u8)).nth(1).and_then(|stamp| stamp.parse::<u64>().ok()).unwrap_or(0)).unwrap_or_default());
    v2.clone()
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        newest_labeled_38(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn live_word_40(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobPending => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_JobRunning => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn state_word_39(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Live(v1) => {
            let v1: US2 = v1.clone();
            live_word_40(v1)
        }
        US1::US1_Settled(v3) => {
            let mut v3: US3 = v3.clone();
            settled_word_22(v3)
        }
    }
}
fn closure10() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: US1 = shared_state_20(v0.clone());
        state_word_39(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_tail_run() -> i32 {
    closure0()()
}
pub fn kit_job_stop_run() -> i32 {
    closure2()()
}
pub fn kit_job_amend_run() -> i32 {
    closure3()()
}
pub fn kit_job_list_run() -> i32 {
    closure4()()
}
pub fn kit_job_stop_text(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub fn kit_job_launched(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn kit_job_settled(v0: &str, v1: u64) -> Rc<str> {
    closure7()(Rc::<str>::from(v0), v1)
}
pub fn kit_job_output(v0: &str, v1: u64) -> Rc<str> {
    closure8()(Rc::<str>::from(v0), v1)
}
pub fn kit_job_labeled(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
pub fn kit_job_state_word(v0: &str) -> Rc<str> {
    closure10()(Rc::<str>::from(v0))
}
