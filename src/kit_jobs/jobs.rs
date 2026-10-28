#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    US5_Some(Rc<str>),
    US5_None,
}
fn shared_summary_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_job_state::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_3(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn started_code_1(mut v0: bool) -> i32 {
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
        let v9: Rc<str> = shared_summary_2(v8.clone(), v5.clone());
        stdout_line_3(v9.clone())
    };
    v4
}
fn stderr_line_6(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn rejected_5(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_6(v3.clone());
    2i32
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_9() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_9();
    joined_8(v1, v0)
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn job_status_4() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_7(v1.clone());
        directory_exists_10(v4.clone())
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host::kit_job_host_revive();
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = shared_summary_2(v8.clone(), v1.clone());
        stdout_line_3(v9.clone());
        0i32
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v1, &*v11, &*v11, &*v11].concat());
        rejected_5(v12.clone())
    }
}
fn settled_exit_13(mut v0: US2) -> i32 {
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
fn state_exit_12(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_Live(v1) => {
            let v1: US1 = v1.clone();
            3i32
        }
        US0::US0_Settled(v2) => {
            let v2: US2 = v2.clone();
            settled_exit_13(v2)
        }
    }
}
fn now_ms_14() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_16() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_15() -> i32 {
    let v0: i32 = command_line_argument_count_16();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn method18(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_17(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method18(v1.clone());
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
fn parent_pid_20() -> u64 {
    let v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_19() -> u64 {
    let v0: u64 = std::env::var("KIT_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_20()
    } else {
        v0
    }
}
fn state_of_parts_23(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US0 {
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
fn shared_state_22(mut v0: Rc<str>) -> US0 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_23(v1, v2, v3)
}
fn state_settled_24(mut v0: US0) -> bool {
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
fn process_alive_25(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn waited_21(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US0 {
    loop {
        let mut v3: US0 = shared_state_22(v0.clone());
        let mut v4: bool = state_settled_24(v3.clone());
        let v7: bool = if v4 {
            true
        } else {
            let v5: u64 = now_ms_14();
            let mut v6: bool = v5 > v1;
            v6
        };
        let v10: bool = if v7 {
            true
        } else {
            let v8: bool = process_alive_25(v2);
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
fn job_waited_11() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_7(v1.clone());
        directory_exists_10(v4.clone())
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host::kit_job_host_revive();
        let v8: u64 = now_ms_14();
        let v9: i32 = argument_count_15();
        let v10: bool = v9 > 4i32;
        let v14: u64 = if v10 {
            let v11: i32 = 4i32;
            let v12: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v11 as usize).unwrap_or_default());
            decimal_u64_17(v12.clone())
        } else {
            864000000u64
        };
        let v15: u64 = v8.wrapping_add(v14);
        let v16: u64 = watched_caller_19();
        let v17: US0 = waited_21(v1.clone(), v15, v16);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = shared_summary_2(v18.clone(), v1.clone());
        stdout_line_3(v19.clone());
        state_exit_12(v17.clone())
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v1, &*v22, &*v22, &*v22].concat());
        rejected_5(v23.clone())
    }
}
fn command_line_argument_26(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn job_run_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: bool = &*v1 == "start";
    let v41: US4 = if v2 {
        US4::US4_JobStart
    } else {
        let mut v4: bool = &*v1 == "run";
        if v4 {
            US4::US4_JobRun
        } else {
            let mut v6: bool = &*v1 == "supervise";
            if v6 {
                US4::US4_JobSupervise
            } else {
                let v8: bool = &*v1 == "host";
                if v8 {
                    US4::US4_JobHost
                } else {
                    let v10: bool = &*v1 == "status";
                    if v10 {
                        US4::US4_JobStatus
                    } else {
                        let v12: bool = &*v1 == "wait";
                        if v12 {
                            US4::US4_JobWait
                        } else {
                            let v14: bool = &*v1 == "list";
                            if v14 {
                                US4::US4_JobList
                            } else {
                                let v16: bool = &*v1 == "tail";
                                if v16 {
                                    US4::US4_JobTail
                                } else {
                                    let v18: bool = &*v1 == "stop";
                                    if v18 {
                                        US4::US4_JobStop
                                    } else {
                                        let v20: bool = &*v1 == "after";
                                        if v20 {
                                            US4::US4_JobAfter
                                        } else {
                                            let v22: bool = &*v1 == "chain";
                                            if v22 {
                                                US4::US4_JobChain
                                            } else {
                                                let v24: bool = &*v1 == "amend";
                                                if v24 {
                                                    US4::US4_JobAmend
                                                } else {
                                                    let v26: bool = &*v1 == "doctor";
                                                    if v26 {
                                                        US4::US4_JobDoctor
                                                    } else {
                                                        US4::US4_JobUnknown
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
            }
        }
    };
    match &v41 {
        US4::US4_JobAfter => {
            let v55: i32 = kit_job_rechain::kit_job_rechain_run();
            v55
        }
        US4::US4_JobAmend => {
            let v71: i32 = kit_job_control::kit_job_amend_run();
            v71
        }
        US4::US4_JobChain => {
            let v56: i32 = command_line_argument_count_16();
            let v57: bool = 2i32 < v56;
            let v62: US5 = if v57 {
                let v58: i32 = 2i32;
                let v59: Rc<str> = command_line_argument_26(v58);
                US5::US5_Some(v59.clone())
            } else {
                US5::US5_None
            };
            let v66: Rc<str> = match &v62 {
                US5::US5_None => {
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v64.clone()
                }
                US5::US5_Some(v63) => {
                    let v63: Rc<str> = v63.clone();
                    v63.clone()
                }
            };
            let v67: bool = &*v66 == "--run-serial";
            if v67 {
                let v68: i32 = kit_job_serial::kit_job_serial_run();
                v68
            } else {
                let v69: i32 = kit_job_chain::kit_job_chain_run();
                v69
            }
        }
        US4::US4_JobDoctor => {
            let v72: i32 = kit_jobs_doctor::kit_job_doctor_run();
            v72
        }
        US4::US4_JobHost => {
            let v49: i32 = kit_job_host::kit_job_host_run();
            v49
        }
        US4::US4_JobList => {
            let v52: i32 = kit_job_control::kit_job_list_run();
            v52
        }
        US4::US4_JobRun => {
            let v44: bool = true;
            started_code_1(v44)
        }
        US4::US4_JobStart => {
            let v42: bool = false;
            started_code_1(v42)
        }
        US4::US4_JobStatus => {
            job_status_4()
        }
        US4::US4_JobStop => {
            let v54: i32 = kit_job_control::kit_job_stop_run();
            v54
        }
        US4::US4_JobSupervise => {
            let v46: i32 = 3i32;
            let v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v46 as usize).unwrap_or_default());
            let v48: i32 = kit_job_supervise::kit_job_supervise(&*v47);
            v48
        }
        US4::US4_JobTail => {
            let v53: i32 = kit_job_control::kit_job_tail_run();
            v53
        }
        US4::US4_JobUnknown => {
            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
            rejected_5(v73.clone())
        }
        US4::US4_JobWait => {
            job_waited_11()
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
