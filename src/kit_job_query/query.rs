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
    US1_Machine(Rc<str>),
}
#[derive(Clone)]
enum US0 {
    US0_Local,
    US0_OnMachine(US1),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US1),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobFinished(i32),
    US4_JobTimedOut,
    US4_JobStopped,
    US4_JobSpawnFailed(Rc<str>),
    US4_JobSkipped(Rc<str>),
    US4_JobLost,
    US4_JobMemoryCapped,
    US4_JobExpiredInQueue,
}
#[derive(Clone)]
enum US6 {
    US6_JobPending,
    US6_JobRunning,
}
#[derive(Clone)]
enum US5 {
    US5_Live(US6),
    US5_Settled(US4),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_WaitFinished,
    US8_WaitTimedOut,
    US8_WaitCallerGone,
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
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn job_placement_2(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_3(v0, v1);
    let mut v3: Rc<str> = text_or_empty_7(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let mut v6: i32 = method8(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method9(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: bool = &*v11 == "";
    let v14: bool = if v12 {
        true
    } else {
        let v13: bool = &*v11 == "local";
        v13
    };
    let v49: US2 = if v14 {
        let v15: US0 = US0::US0_Local;
        US2::US2_Some(v15)
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
            let v27: i32 = method8(v24.clone(), v25, v26);
            let v28: i32 = v25.wrapping_sub(1i32);
            let v29: Rc<str> = string_slice(&v24, v27 as i64, v28 as i64);
            let v30: i32 = (v29.clone().len() as i32);
            let v31: i32 = method9(v29.clone(), v30);
            let v32: Rc<str> = string_slice(&v29, 0i32 as i64, v31 as i64);
            let v33: bool = &*v32 == "";
            let v36: bool = if v33 {
                true
            } else {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v35: bool = v32.contains(&*v34);
                v35
            };
            let v40: US3 = if v36 {
                US3::US3_None
            } else {
                let v38: US1 = US1::US1_Machine(v32);
                US3::US3_Some(v38)
            };
            match v40 {
                US3::US3_None => {
                    US2::US2_None
                }
                US3::US3_Some(v41) => {
                    let v41: US1 = v41;
                    let v42: US0 = US0::US0_OnMachine(v41);
                    US2::US2_Some(v42)
                }
            }
        } else {
            US2::US2_None
        }
    };
    match v49 {
        US2::US2_None => {
            US0::US0_Local
        }
        US2::US2_Some(v50) => {
            let v50: US0 = v50;
            v50
        }
    }
}
fn placed_1(mut v0: Rc<str>, mut v1: Rc<dyn Fn() -> i32>) -> i32 {
    let mut v2: US0 = job_placement_2(v0);
    match v2 {
        US0::US0_Local => {
            v1()
        }
        US0::US0_OnMachine(v3) => {
            let mut v3: US1 = v3;
            let mut v4: Rc<str> = jobs_root_6();
            let mut v6: Rc<str> = match v3 {
                US1::US1_Machine(v5) => {
                    let v5: Rc<str> = v5;
                    v5
                }
            };
            let v7: i32 = kit_job_dispatch::kit_job_forward_run(&v4, &v6);
            v7
        }
    }
}
fn concat5_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn stderr_line_13(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_11(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_12(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_13(v3);
    2i32
}
fn directory_exists_14(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn shared_summary_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = kit_job_summary::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_16(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn job_status_10(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        let mut v3: Rc<str> = job_directory_5(v0.clone());
        directory_exists_14(v3)
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = kit_job_host_request::kit_job_host_revive();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = shared_summary_15(v7, v0);
        stdout_line_16(v8);
        0i32
    } else {
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) }, v0));
        rejected_11(v9)
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        job_status_10(v0.clone())
    })
}
fn status_placed_0(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure1(v0.clone());
    placed_1(v0, v1)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        status_placed_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn settled_exit_20(mut v0: US4) -> i32 {
    match v0 {
        US4::US4_JobExpiredInQueue => {
            75i32
        }
        US4::US4_JobFinished(v1) => {
            let mut v1: i32 = v1;
            v1
        }
        US4::US4_JobLost => {
            125i32
        }
        US4::US4_JobMemoryCapped => {
            137i32
        }
        US4::US4_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3;
            125i32
        }
        US4::US4_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2;
            125i32
        }
        US4::US4_JobStopped => {
            143i32
        }
        US4::US4_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_19(mut v0: US5) -> i32 {
    match v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1;
            3i32
        }
        US5::US5_Settled(v2) => {
            let mut v2: US4 = v2;
            settled_exit_20(v2)
        }
    }
}
fn now_ms_21() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method23(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let mut v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_22(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method23(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure4();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let mut v6: US7 = US7::US7_None;
    let v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn parent_pid_25() -> u64 {
    let v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_24() -> u64 {
    let v0: u64 = std::env::var("KIT_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let mut v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_25()
    } else {
        v0
    }
}
fn state_of_parts_28(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US5 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US6 = US6::US6_JobPending;
        US5::US5_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US6 = US6::US6_JobRunning;
            US5::US5_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US4 = US4::US4_JobFinished(v1);
                US5::US5_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US4 = US4::US4_JobTimedOut;
                    US5::US5_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US4 = US4::US4_JobStopped;
                        US5::US5_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US4 = US4::US4_JobLost;
                            US5::US5_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US4 = US4::US4_JobMemoryCapped;
                                US5::US5_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US4 = US4::US4_JobExpiredInQueue;
                                    US5::US5_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US4 = US4::US4_JobSkipped(v2);
                                        US5::US5_Settled(v28)
                                    } else {
                                        let v30: US4 = US4::US4_JobSpawnFailed(v2);
                                        US5::US5_Settled(v30)
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
fn shared_state_27(mut v0: Rc<str>) -> US5 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_28(v1, v2, v3)
}
fn state_settled_29(mut v0: US5) -> bool {
    match v0 {
        US5::US5_Live(v1) => {
            let mut v1: US6 = v1;
            false
        }
        US5::US5_Settled(v2) => {
            let mut v2: US4 = v2;
            true
        }
    }
}
fn process_alive_30(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn waited_26(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US5 {
    loop {
        let mut v3: US5 = shared_state_27(v0.clone());
        let mut v4: bool = state_settled_29(v3.clone());
        let v7: bool = if v4 {
            true
        } else {
            let v5: u64 = now_ms_21();
            let mut v6: bool = v5 > v1;
            v6
        };
        let v10: bool = if v7 {
            true
        } else {
            let v8: bool = process_alive_30(v2);
            let v9: bool = v8 == false;
            v9
        };
        if v10 {
            return v3;
        } else {
            let v11: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v11));
            continue;
        }
    }
}
fn wait_end_of_31(mut v0: US5, mut v1: u64) -> US8 {
    let mut v2: bool = state_settled_29(v0);
    if v2 {
        US8::US8_WaitFinished
    } else {
        let mut v4: bool = process_alive_30(v1);
        let v5: bool = v4 == false;
        if v5 {
            US8::US8_WaitCallerGone
        } else {
            US8::US8_WaitTimedOut
        }
    }
}
fn kebab_from_33(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: usize = ((v2) as usize);
            let mut v6: i32 = v2.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
            let v10: bool = v2 > 0i32;
            let v12: bool = if v10 {
                let v11: bool = &*v8 != &*v9;
                v11
            } else {
                false
            };
            let v15: Rc<str> = if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v15));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
            (v2, v3) = (v6, v17);
            continue;
        }
    }
}
fn wait_end_word_32(mut v0: US8) -> Rc<str> {
    let mut v4: Rc<str> = match v0 {
        US8::US8_WaitCallerGone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CallerGone"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US8::US8_WaitFinished => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Finished"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US8::US8_WaitTimedOut => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TimedOut"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    };
    let v5: i32 = (v4.clone().len() as i32);
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    kebab_from_33(v4, v5, v6, v7)
}
fn job_waited_18(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_5(v0.clone());
        directory_exists_14(v4)
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host_request::kit_job_host_revive();
        let v8: u64 = now_ms_21();
        let v9: bool = &*v1 == "";
        let v11: u64 = if v9 {
            864000000u64
        } else {
            decimal_u64_22(v1)
        };
        let v12: u64 = v8.wrapping_add(v11);
        let v13: u64 = watched_caller_24();
        let v14: US5 = waited_26(v0.clone(), v12, v13);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = shared_summary_15(v15, v0);
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" end="); } LIT.with(|lit| lit.clone()) }));
        let v18: US8 = wait_end_of_31(v14.clone(), v13);
        let v19: Rc<str> = wait_end_word_32(v18);
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v19));
        stdout_line_16(v20);
        state_exit_19(v14)
    } else {
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) }, v0));
        rejected_11(v22)
    }
}
fn closure3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn() -> i32> {
    Rc::new(move || -> i32 {
        job_waited_18(v1.clone(), v0.clone())
    })
}
fn wait_placed_17(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<dyn Fn() -> i32> = closure3(v0, v1.clone());
    placed_1(v1, v2)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        wait_placed_17(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = kit_job_control::kit_job_tail_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tail_placed_34(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure6();
    placed_1(v0, v1)
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        tail_placed_34(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: i32 = kit_job_control::kit_job_stop_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stop_placed_35(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<dyn Fn() -> i32> = closure8();
    placed_1(v0, v1)
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        stop_placed_35(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_status_placed(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_job_wait_placed(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_tail_placed(v0: &str) -> i32 {
    closure5()(Rc::<str>::from(v0))
}
pub fn kit_job_stop_placed(v0: &str) -> i32 {
    closure7()(Rc::<str>::from(v0))
}
