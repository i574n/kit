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
#[derive(Clone)]
enum US7 {
    US7_JobFinished(i32),
    US7_JobTimedOut,
    US7_JobStopped,
    US7_JobSpawnFailed(Rc<str>),
    US7_JobSkipped(Rc<str>),
    US7_JobLost,
    US7_JobMemoryCapped,
    US7_JobExpiredInQueue,
}
#[derive(Clone)]
enum US9 {
    US9_JobPending,
    US9_JobRunning,
}
#[derive(Clone)]
enum US8 {
    US8_Live(US9),
    US8_Settled(US7),
}
#[derive(Clone)]
enum US10 {
    US10_Some(u64),
    US10_None,
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
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_2(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_3(v3);
    2i32
}
fn shared_summary_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = kit_job_state::kit_job_summary(&*v0, &*v1);
    v2
}
fn stdout_line_6(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn started_code_4(mut v0: bool) -> i32 {
    let mut v3: Rc<str> = if v0 {
        let mut v1: Rc<str> = kit_job_launch::kit_job_start(true);
        v1.clone()
    } else {
        let mut v2: Rc<str> = kit_job_launch::kit_job_start(false);
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
fn jobs_root_8() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn dispatched_7() -> i32 {
    let mut v0: Rc<str> = jobs_root_8();
    let mut v1: i32 = kit_job_dispatch::kit_job_dispatch_run(&*v0);
    v1
}
fn joined_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn job_directory_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_8();
    joined_12(v1, v0)
}
fn job_file_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_13(v0);
    joined_12(v2, v1)
}
fn text_or_empty_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
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
fn job_placement_10(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_11(v0, v1);
    let mut v3: Rc<str> = text_or_empty_14(v2);
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let mut v6: i32 = method15(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method16(v8.clone(), v9);
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
            let v27: i32 = method15(v24.clone(), v25, v26);
            let v28: i32 = v25.wrapping_sub(1i32);
            let v29: Rc<str> = string_slice(&v24, v27 as i64, v28 as i64);
            let v30: i32 = (v29.clone().len() as i32);
            let v31: i32 = method16(v29.clone(), v30);
            let mut v32: Rc<str> = string_slice(&v29, 0i32 as i64, v31 as i64);
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
            match &v40 {
                US3::US3_None => {
                    US2::US2_None
                }
                US3::US3_Some(v41) => {
                    let v41: US1 = v41.clone();
                    let v42: US0 = US0::US0_OnMachine(v41);
                    US2::US2_Some(v42)
                }
            }
        } else {
            US2::US2_None
        }
    };
    match &v49 {
        US2::US2_None => {
            US0::US0_Local
        }
        US2::US2_Some(v50) => {
            let v50: US0 = v50.clone();
            v50
        }
    }
}
fn placed_9(mut v0: Rc<dyn Fn() -> i32>) -> i32 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: US0 = job_placement_10(v2);
    match &v3 {
        US0::US0_Local => {
            v0()
        }
        US0::US0_OnMachine(v4) => {
            let mut v4: US1 = v4.clone();
            let v5: Rc<str> = jobs_root_8();
            let v7: Rc<str> = match &v4 {
                US1::US1_Machine(v6) => {
                    let mut v6: Rc<str> = v6.clone();
                    v6
                }
            };
            let v8: i32 = kit_job_dispatch::kit_job_forward_run(&*v5, &*v7);
            v8
        }
    }
}
fn method17(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0;
    let mut v2: bool = v1 > 0i32;
    v2
}
fn command_line_argument_count_18() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_19(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn directory_exists_21(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn job_status_20() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_13(v1.clone());
        directory_exists_21(v4)
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
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_status_20()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = kit_job_control::kit_job_stop_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: i32 = kit_job_control::kit_job_tail_run();
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn settled_exit_24(mut v0: US7) -> i32 {
    match &v0 {
        US7::US7_JobExpiredInQueue => {
            75i32
        }
        US7::US7_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            v1
        }
        US7::US7_JobLost => {
            125i32
        }
        US7::US7_JobMemoryCapped => {
            137i32
        }
        US7::US7_JobSkipped(v3) => {
            let mut v3: Rc<str> = v3.clone();
            125i32
        }
        US7::US7_JobSpawnFailed(v2) => {
            let mut v2: Rc<str> = v2.clone();
            125i32
        }
        US7::US7_JobStopped => {
            143i32
        }
        US7::US7_JobTimedOut => {
            124i32
        }
    }
}
fn state_exit_23(mut v0: US8) -> i32 {
    match &v0 {
        US8::US8_Live(v1) => {
            let mut v1: US9 = v1.clone();
            3i32
        }
        US8::US8_Settled(v2) => {
            let mut v2: US7 = v2.clone();
            settled_exit_24(v2)
        }
    }
}
fn now_ms_25() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn argument_count_26() -> i32 {
    let mut v0: i32 = command_line_argument_count_18();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn method28(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure5() -> Rc<dyn Fn((u64)) -> US10> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US10> = Rc::new(move |mut v0: (u64)| -> US10 {
        let mut v1: u64 = (v0);
        US10::US10_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_27(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method28(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US10> = closure5();
    let mut v4: Option<US10> = v2.map(|x| v3(x));
    let mut v6: US10 = US10::US10_None;
    let v7: US10 = v6.clone();
    let v8: US10 = v4.unwrap_or(v7);
    match &v8 {
        US10::US10_None => {
            0u64
        }
        US10::US10_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn parent_pid_30() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn parent() -> u64 { #[repr(C)] struct Entry { size: u32, usage: u32, pid: u32, heap: usize, module: u32, threads: u32, parent: u32, base: i32, flags: u32, exe: [u16; 260] } unsafe extern "system" { fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut std::ffi::c_void; fn Process32FirstW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn Process32NextW(snapshot: *mut std::ffi::c_void, entry: *mut Entry) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let me = std::process::id(); unsafe { let snapshot = CreateToolhelp32Snapshot(2, 0); if snapshot as isize == -1 { return 0; } let mut entry: Entry = std::mem::zeroed(); entry.size = std::mem::size_of::<Entry>() as u32; let mut found = 0u64; let mut more = Process32FirstW(snapshot, &mut entry) != 0; while more { if entry.pid == me { found = entry.parent as u64; break; } more = Process32NextW(snapshot, &mut entry) != 0; } CloseHandle(snapshot); found } } #[cfg(not(windows))] fn parent() -> u64 { std::os::unix::process::parent_id() as u64 } parent() };
    v0
}
fn watched_caller_29() -> u64 {
    let mut v0: u64 = std::env::var("KIT_JOB_WATCH").ok().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0);
    let mut v1: bool = v0 == 0u64;
    if v1 {
        parent_pid_30()
    } else {
        v0
    }
}
fn state_of_parts_33(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US8 {
    let mut v3: bool = &*v0 == "pending";
    if v3 {
        let mut v4: US9 = US9::US9_JobPending;
        US8::US8_Live(v4)
    } else {
        let mut v6: bool = &*v0 == "running";
        if v6 {
            let v7: US9 = US9::US9_JobRunning;
            US8::US8_Live(v7)
        } else {
            let v9: bool = &*v0 == "finished";
            if v9 {
                let v10: US7 = US7::US7_JobFinished(v1);
                US8::US8_Settled(v10)
            } else {
                let v12: bool = &*v0 == "timed-out";
                if v12 {
                    let v13: US7 = US7::US7_JobTimedOut;
                    US8::US8_Settled(v13)
                } else {
                    let v15: bool = &*v0 == "stopped";
                    if v15 {
                        let v16: US7 = US7::US7_JobStopped;
                        US8::US8_Settled(v16)
                    } else {
                        let v18: bool = &*v0 == "lost";
                        if v18 {
                            let v19: US7 = US7::US7_JobLost;
                            US8::US8_Settled(v19)
                        } else {
                            let v21: bool = &*v0 == "memory-capped";
                            if v21 {
                                let v22: US7 = US7::US7_JobMemoryCapped;
                                US8::US8_Settled(v22)
                            } else {
                                let v24: bool = &*v0 == "expired-in-queue";
                                if v24 {
                                    let v25: US7 = US7::US7_JobExpiredInQueue;
                                    US8::US8_Settled(v25)
                                } else {
                                    let v27: bool = &*v0 == "skipped";
                                    if v27 {
                                        let v28: US7 = US7::US7_JobSkipped(v2.clone());
                                        US8::US8_Settled(v28)
                                    } else {
                                        let v30: US7 = US7::US7_JobSpawnFailed(v2);
                                        US8::US8_Settled(v30)
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
fn shared_state_32(mut v0: Rc<str>) -> US8 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = kit_job_state::kit_job_state_parts(&*v0);
    state_of_parts_33(v1, v2, v3)
}
fn state_settled_34(mut v0: US8) -> bool {
    match &v0 {
        US8::US8_Live(v1) => {
            let mut v1: US9 = v1.clone();
            false
        }
        US8::US8_Settled(v2) => {
            let mut v2: US7 = v2.clone();
            true
        }
    }
}
fn process_alive_35(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn waited_31(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> US8 {
    loop {
        let mut v3: US8 = shared_state_32(v0.clone());
        let mut v4: bool = state_settled_34(v3.clone());
        let v7: bool = if v4 {
            true
        } else {
            let v5: u64 = now_ms_25();
            let mut v6: bool = v5 > v1;
            v6
        };
        let v10: bool = if v7 {
            true
        } else {
            let v8: bool = process_alive_35(v2);
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
fn job_waited_22() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v1.starts_with(&*v2);
    let mut v6: bool = if v3 {
        let mut v4: Rc<str> = job_directory_13(v1.clone());
        directory_exists_21(v4)
    } else {
        false
    };
    if v6 {
        let v7: Rc<str> = kit_job_host_request::kit_job_host_revive();
        let v8: u64 = now_ms_25();
        let v9: i32 = argument_count_26();
        let v10: bool = v9 > 4i32;
        let v14: u64 = if v10 {
            let v11: i32 = 4i32;
            let v12: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v11 as usize).unwrap_or_default());
            decimal_u64_27(v12)
        } else {
            864000000u64
        };
        let v15: u64 = v8.wrapping_add(v14);
        let v16: u64 = watched_caller_29();
        let v17: US8 = waited_31(v1.clone(), v15, v16);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = shared_summary_5(v18, v1.clone());
        stdout_line_6(v19);
        state_exit_23(v17)
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: unknown job id "); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = concat5_2(v21, v1, v22.clone(), v22.clone(), v22);
        rejected_1(v23)
    }
}
fn closure4() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        job_waited_22()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
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
    while method17(v65.clone()) {
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
            let v113: i32 = kit_job_rechain::kit_job_rechain_run();
            v113
        }
        US4::US4_JobAmend => {
            let v129: i32 = kit_job_control::kit_job_amend_run();
            v129
        }
        US4::US4_JobChain => {
            let v114: i32 = command_line_argument_count_18();
            let v115: bool = 2i32 < v114;
            let v120: US6 = if v115 {
                let v116: i32 = 2i32;
                let v117: Rc<str> = command_line_argument_19(v116);
                US6::US6_Some(v117.clone())
            } else {
                US6::US6_None
            };
            let v124: Rc<str> = match &v120 {
                US6::US6_None => {
                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v122.clone()
                }
                US6::US6_Some(v121) => {
                    let v121: Rc<str> = v121.clone();
                    v121.clone()
                }
            };
            let v125: bool = &*v124 == "--run-serial";
            if v125 {
                let v126: i32 = kit_job_serial::kit_job_serial_run();
                v126
            } else {
                let v127: i32 = kit_job_chain::kit_job_chain_run();
                v127
            }
        }
        US4::US4_JobDoctor => {
            let v130: i32 = kit_jobs_doctor::kit_job_doctor_run();
            v130
        }
        US4::US4_JobHost => {
            let v96: i32 = 3i32;
            let v97: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v96 as usize).unwrap_or_default());
            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v99: bool = v97.starts_with(&*v98);
            if v99 {
                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host is started by kit itself and takes no flags; see job --help"); } LIT.with(|lit| lit.clone()) };
                rejected_1(v100.clone())
            } else {
                let v102: i32 = kit_job_host::kit_job_host_run();
                v102
            }
        }
        US4::US4_JobList => {
            let v108: i32 = kit_job_control::kit_job_list_run();
            v108
        }
        US4::US4_JobRun => {
            let v91: bool = true;
            started_code_4(v91)
        }
        US4::US4_JobStart => {
            let v84: i32 = 3i32;
            let v85: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v84 as usize).unwrap_or_default());
            let v86: bool = &*v85 == "--on";
            if v86 {
                dispatched_7()
            } else {
                let v88: bool = false;
                started_code_4(v88)
            }
        }
        US4::US4_JobStatus => {
            let v104: Rc<dyn Fn() -> i32> = closure1();
            placed_9(v104.clone())
        }
        US4::US4_JobStop => {
            let v111: Rc<dyn Fn() -> i32> = closure2();
            placed_9(v111.clone())
        }
        US4::US4_JobSupervise => {
            let v93: i32 = 3i32;
            let v94: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v93 as usize).unwrap_or_default());
            let v95: i32 = kit_job_supervise::kit_job_supervise(&*v94);
            v95
        }
        US4::US4_JobTail => {
            let v109: Rc<dyn Fn() -> i32> = closure3();
            placed_9(v109.clone())
        }
        US4::US4_JobUnknown => {
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
            rejected_1(v131.clone())
        }
        US4::US4_JobWait => {
            let v106: Rc<dyn Fn() -> i32> = closure4();
            placed_9(v106.clone())
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
