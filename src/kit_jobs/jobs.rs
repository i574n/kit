#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_JobFinished(i32),
    US0_JobTimedOut,
    US0_JobStopped,
    US0_JobSpawnFailed(Rc<str>),
    US0_JobSkipped(Rc<str>),
    US0_JobLost,
    US0_JobMemoryCapped,
    US0_JobExpiredInQueue,
}
#[derive(Clone)]
enum US2 {
    US2_JobPending,
    US2_JobRunning,
}
#[derive(Clone)]
enum US1 {
    US1_Live(US2),
    US1_Settled(US0),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_JobStart,
    US4_JobRun,
    US4_JobSupervise,
    US4_JobHost,
    US4_JobStatus,
    US4_JobWait,
    US4_JobList,
    US4_JobTail,
    US4_JobStop,
    US4_JobAfter,
    US4_JobChain,
    US4_JobAmend,
    US4_JobDoctor,
    US4_JobUnknown,
}
#[derive(Clone)]
enum US5 {
    US5_Some(US4),
    US5_None,
}
struct Mut0 { l0: i32, l1: US5 }
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>),
    US6_None,
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_2(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_3(v3);
    2i32
}
fn shared_summary_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_job_state::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_6(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn started_code_4(mut v0: bool) -> i32 {
    let mut v3: Rc<str> = if v0 {
        let v1: Rc<str> = kit_job_launch::kit_job_start(true);
        v1.clone()
    } else {
        let v2: Rc<str> = kit_job_launch::kit_job_start(false);
        v2.clone()
    };
    let mut v4: i32 = (v3).split(char::from(9u8)).next().and_then(|code| code.trim().parse::<i32>().ok()).unwrap_or(2);
    let v5: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(9u8)).nth(1).unwrap_or("").trim());
    let v7: bool = if v0 {
        let mut v6: bool = &*v5 != "";
        v6
    } else {
        false
    };
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = shared_summary_5(v8.clone(), v5.clone());
        stdout_line_6(v9.clone())
    };
    v4
}
fn joined_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_10() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_10();
    joined_9(v1, v0)
}
fn directory_exists_11(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn job_status_7() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_8(v1.clone());
        directory_exists_11(v4)
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host_request::kit_job_host_revive();
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = shared_summary_5(v8, v1.clone());
        stdout_line_6(v9);
        0i32
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = concat5_2(v10, v1, v11.clone(), v11.clone(), v11);
        rejected_1(v12)
    }
}
fn settled_exit_14(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_JobExpiredInQueue => {
            75i32
        }
        US0::US0_JobFinished(v1) => {
            let v1: i32 = *v1;
            v1
        }
        US0::US0_JobLost => {
            125i32
        }
        US0::US0_JobMemoryCapped => {
            137i32
        }
        US0::US0_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US0::US0_JobSpawnFailed(v2) => {
            let v2: Rc<str> = v2.clone();
            125i32
        }
        US0::US0_JobStopped => {
            143i32
        }
        US0::US0_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_13(mut v0: US1) -> i32 {
    match &v0 {
        US1::US1_Live(v1) => {
            let v1: US2 = v1.clone();
            3i32
        }
        US1::US1_Settled(v2) => {
            let v2: US0 = v2.clone();
            settled_exit_14(v2)
        }
    }
}
fn now_ms_15() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_17() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_16() -> i32 {
    let v0: i32 = command_line_argument_count_17();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn method19(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_18(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method19(v1.clone());
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
fn parent_pid_21() -> u64 {
    let v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_20() -> u64 {
    let v0: u64 = std::env::var("KIT_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_21()
    } else {
        v0
    }
}
fn state_of_parts_24(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US1 {
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
                let v10: US0 = US0::US0_JobFinished(v1);
                US1::US1_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US0 = US0::US0_JobTimedOut;
                    US1::US1_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US0 = US0::US0_JobStopped;
                        US1::US1_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US0 = US0::US0_JobLost;
                            US1::US1_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US0 = US0::US0_JobMemoryCapped;
                                US1::US1_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US0 = US0::US0_JobExpiredInQueue;
                                    US1::US1_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US0 = US0::US0_JobSkipped(v2.clone());
                                        US1::US1_Settled(v28)
                                    } else {
                                        let v30: US0 = US0::US0_JobSpawnFailed(v2);
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
fn shared_state_23(mut v0: Rc<str>) -> US1 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_24(v1, v2, v3)
}
fn state_settled_25(mut v0: US1) -> bool {
    match &v0 {
        US1::US1_Live(v1) => {
            let v1: US2 = v1.clone();
            false
        }
        US1::US1_Settled(v2) => {
            let v2: US0 = v2.clone();
            true
        }
    }
}
fn process_alive_26(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn waited_22(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US1 {
    loop {
        let mut v3: US1 = shared_state_23(v0.clone());
        let mut v4: bool = state_settled_25(v3.clone());
        let v7: bool = if v4 {
            true
        } else {
            let v5: u64 = now_ms_15();
            let mut v6: bool = v5 > v1;
            v6
        };
        let v10: bool = if v7 {
            true
        } else {
            let v8: bool = process_alive_26(v2);
            let v9: bool = v8 == false;
            v9
        };
        if v10 {
            return v3.clone();
        } else {
            let v11: u64 = 250u64;
            std::thread::sleep(std::time::Duration::from_millis(v11));
            continue;
        }
    }
}
fn job_waited_12() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_8(v1.clone());
        directory_exists_11(v4)
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host_request::kit_job_host_revive();
        let v8: u64 = now_ms_15();
        let v9: i32 = argument_count_16();
        let v10: bool = v9 > 4i32;
        let v14: u64 = if v10 {
            let v11: i32 = 4i32;
            let v12: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v11 as usize).unwrap_or_default());
            decimal_u64_18(v12)
        } else {
            864000000u64
        };
        let v15: u64 = v8.wrapping_add(v14);
        let v16: u64 = watched_caller_20();
        let v17: US1 = waited_22(v1.clone(), v15, v16);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = shared_summary_5(v18, v1.clone());
        stdout_line_6(v19);
        state_exit_13(v17)
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = concat5_2(v21, v1, v22.clone(), v22.clone(), v22);
        rejected_1(v23)
    }
}
fn method27(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let v1: i32 = v0.borrow().l0;
    let v2: bool = v1 > 0i32;
    v2
}
fn command_line_argument_28(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn job_run_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    ;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobUnknown"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobDoctor"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobAmend"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobChain"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobAfter"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v12.to_lowercase());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStop"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v14.to_lowercase());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobTail"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v16.to_lowercase());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobList"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v18.to_lowercase());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobWait"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v20.to_lowercase());
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStatus"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v22.to_lowercase());
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobHost"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(v24.to_lowercase());
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobSupervise"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(v26.to_lowercase());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobRun"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v28.to_lowercase());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("JobStart"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(v30.to_lowercase());
    let mut v32: Rc<RefCell<Vec<(Rc<str>, US4)>>> = Rc::new(RefCell::new(Vec::new()));
    let v33: US4 = US4::US4_JobStart;
    v32.borrow_mut().push((v30.clone(), v33.clone()));
    let v34: US4 = US4::US4_JobRun;
    v32.borrow_mut().push((v28.clone(), v34.clone()));
    let v35: US4 = US4::US4_JobSupervise;
    v32.borrow_mut().push((v26.clone(), v35.clone()));
    let v36: US4 = US4::US4_JobHost;
    v32.borrow_mut().push((v24.clone(), v36.clone()));
    let v37: US4 = US4::US4_JobStatus;
    v32.borrow_mut().push((v22.clone(), v37.clone()));
    let v38: US4 = US4::US4_JobWait;
    v32.borrow_mut().push((v20.clone(), v38.clone()));
    let v39: US4 = US4::US4_JobList;
    v32.borrow_mut().push((v18.clone(), v39.clone()));
    let v40: US4 = US4::US4_JobTail;
    v32.borrow_mut().push((v16.clone(), v40.clone()));
    let v41: US4 = US4::US4_JobStop;
    v32.borrow_mut().push((v14.clone(), v41.clone()));
    let v42: US4 = US4::US4_JobAfter;
    v32.borrow_mut().push((v12.clone(), v42.clone()));
    let v43: US4 = US4::US4_JobChain;
    v32.borrow_mut().push((v10.clone(), v43.clone()));
    let v44: US4 = US4::US4_JobAmend;
    v32.borrow_mut().push((v8.clone(), v44.clone()));
    let v45: US4 = US4::US4_JobDoctor;
    v32.borrow_mut().push((v6.clone(), v45.clone()));
    let v46: US4 = US4::US4_JobUnknown;
    v32.borrow_mut().push((v4.clone(), v46.clone()));
    let v47: US4 = US4::US4_JobStart;
    v32.borrow_mut().push((v31.clone(), v47.clone()));
    let v48: US4 = US4::US4_JobRun;
    v32.borrow_mut().push((v29.clone(), v48.clone()));
    let v49: US4 = US4::US4_JobSupervise;
    v32.borrow_mut().push((v27.clone(), v49.clone()));
    let v50: US4 = US4::US4_JobHost;
    v32.borrow_mut().push((v25.clone(), v50.clone()));
    let v51: US4 = US4::US4_JobStatus;
    v32.borrow_mut().push((v23.clone(), v51.clone()));
    let v52: US4 = US4::US4_JobWait;
    v32.borrow_mut().push((v21.clone(), v52.clone()));
    let v53: US4 = US4::US4_JobList;
    v32.borrow_mut().push((v19.clone(), v53.clone()));
    let v54: US4 = US4::US4_JobTail;
    v32.borrow_mut().push((v17.clone(), v54.clone()));
    let v55: US4 = US4::US4_JobStop;
    v32.borrow_mut().push((v15.clone(), v55.clone()));
    let v56: US4 = US4::US4_JobAfter;
    v32.borrow_mut().push((v13.clone(), v56.clone()));
    let v57: US4 = US4::US4_JobChain;
    v32.borrow_mut().push((v11.clone(), v57.clone()));
    let v58: US4 = US4::US4_JobAmend;
    v32.borrow_mut().push((v9.clone(), v58.clone()));
    let v59: US4 = US4::US4_JobDoctor;
    v32.borrow_mut().push((v7.clone(), v59.clone()));
    let v60: US4 = US4::US4_JobUnknown;
    v32.borrow_mut().push((v5.clone(), v60.clone()));
    let v61: Rc<Vec<(Rc<str>, US4)>> = Rc::new(v32.borrow().clone());
    let v62: Rc<RefCell<Vec<(Rc<str>, US4)>>> = Rc::new(RefCell::new((v61).as_ref().clone()));
    let v63: i32 = (v62.clone().borrow().len() as i32);
    let v64: US5 = US5::US5_None;
    let mut v65: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v63, l1: v64.clone() }));
    while method27(v65.clone()) {
        let v67: i32 = v65.borrow().l0;
        let v68: i32 = v67.wrapping_sub(1i32);
        let v69: US5 = v65.borrow().l1.clone();
        let (mut v70, mut v71): (Rc<str>, US4) = v62.borrow()[v68 as usize].clone();
        let v78: US5 = match &v69 {
            US5::US5_None => {
                let v73: bool = v70 == v3 ;
                if v73 {
                    US5::US5_Some(v71.clone())
                } else {
                    US5::US5_None
                }
            }
            US5::US5_Some(v72) => {
                let v72: US4 = v72.clone();
                v69.clone()
            }
        };
        v65.borrow_mut().l0 = v68;
        v65.borrow_mut().l1 = v78.clone();
        ()
    };
    let v79: US5 = v65.borrow().l1.clone();
    let v83: US4 = match &v79 {
        US5::US5_None => {
            US4::US4_JobUnknown
        }
        US5::US5_Some(v80) => {
            let v80: US4 = v80.clone();
            v80.clone()
        }
    };
    match &v83 {
        US4::US4_JobAfter => {
            let v104: i32 = kit_job_rechain::kit_job_rechain_run();
            v104
        }
        US4::US4_JobAmend => {
            let v120: i32 = kit_job_control::kit_job_amend_run();
            v120
        }
        US4::US4_JobChain => {
            let v105: i32 = command_line_argument_count_17();
            let v106: bool = 2i32 < v105;
            let v111: US6 = if v106 {
                let v107: i32 = 2i32;
                let v108: Rc<str> = command_line_argument_28(v107);
                US6::US6_Some(v108.clone())
            } else {
                US6::US6_None
            };
            let v115: Rc<str> = match &v111 {
                US6::US6_None => {
                    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v113.clone()
                }
                US6::US6_Some(v112) => {
                    let v112: Rc<str> = v112.clone();
                    v112.clone()
                }
            };
            let v116: bool = &*v115 == "--run-serial";
            if v116 {
                let v117: i32 = kit_job_serial::kit_job_serial_run();
                v117
            } else {
                let v118: i32 = kit_job_chain::kit_job_chain_run();
                v118
            }
        }
        US4::US4_JobDoctor => {
            let v121: i32 = kit_jobs_doctor::kit_job_doctor_run();
            v121
        }
        US4::US4_JobHost => {
            let v91: i32 = 3i32;
            let v92: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v91 as usize).unwrap_or_default());
            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v94: bool = v92.starts_with(&*v93);
            if v94 {
                let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host is started by kit itself and takes no flags; see job --help"); } LIT.with(|lit| lit.clone()) };
                rejected_1(v95.clone())
            } else {
                let v97: i32 = kit_job_host::kit_job_host_run();
                v97
            }
        }
        US4::US4_JobList => {
            let v101: i32 = kit_job_control::kit_job_list_run();
            v101
        }
        US4::US4_JobRun => {
            let v86: bool = true;
            started_code_4(v86)
        }
        US4::US4_JobStart => {
            let v84: bool = false;
            started_code_4(v84)
        }
        US4::US4_JobStatus => {
            job_status_7()
        }
        US4::US4_JobStop => {
            let v103: i32 = kit_job_control::kit_job_stop_run();
            v103
        }
        US4::US4_JobSupervise => {
            let v88: i32 = 3i32;
            let v89: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v88 as usize).unwrap_or_default());
            let v90: i32 = kit_job_supervise::kit_job_supervise(&*v89);
            v90
        }
        US4::US4_JobTail => {
            let v102: i32 = kit_job_control::kit_job_tail_run();
            v102
        }
        US4::US4_JobUnknown => {
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
            rejected_1(v122.clone())
        }
        US4::US4_JobWait => {
            job_waited_12()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_run() -> i32 {
    closure0()()
}
