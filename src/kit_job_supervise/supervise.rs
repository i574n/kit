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
    US0_KeepRunning,
    US0_HaltOnTimeout,
    US0_HaltOnRequest,
    US0_HaltOnMemory,
}
#[derive(Clone)]
enum US1 {
    US1_TreeStopped,
    US1_SurvivorsLeft(u64),
}
struct Mut0 { l0: Rc<str> }
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
enum US3 {
    US3_NotWaiting,
    US3_WaitingAfter(Rc<str>),
    US3_WaitingWindow(Rc<str>),
    US3_WaitingSlot,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
fn child_container_4() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn container() -> u64 { static CONTAINER: std::sync::OnceLock<u64> = std::sync::OnceLock::new(); *CONTAINER.get_or_init(|| { unsafe extern "system" { fn CreateJobObjectW(attributes: *mut std::ffi::c_void, name: *const u16) -> *mut std::ffi::c_void; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; } unsafe { let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()); if !job.is_null() { let mut limits = [0u64; 18]; limits[2] = 0x2800; SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144); } job as u64 } }) } #[cfg(not(windows))] fn container() -> u64 { 0 } container() };
    v0
}
fn joined_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_8() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_8();
    joined_6(v1, v0)
}
fn job_file_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_7(v0);
    joined_6(v2, v1)
}
fn now_ms_9() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn text_written_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn halted_3(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let mut v3: u64 = child_container_4();
    let mut v4: u64 = { let mut child = v0.borrow_mut(); let root = child.id(); #[cfg(windows)] let survivors = { unsafe extern "system" { fn TerminateJobObject(job: *mut std::ffi::c_void, code: u32) -> i32; fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; } let _ = root; let job = v3 as *mut std::ffi::c_void; if !job.is_null() { unsafe { TerminateJobObject(job, 124); } } let _ = child.kill(); let _ = child.wait(); let active = || -> u64 { let mut accounting = [0u64; 6]; if job.is_null() || unsafe { QueryInformationJobObject(job, 1, accounting.as_mut_ptr().cast(), 48, std::ptr::null_mut()) } == 0 { 0 } else { accounting[5] & 0xffff_ffff } }; let mut waited = 0u64; while active() > 0 && waited < 3000 { std::thread::sleep(std::time::Duration::from_millis(100)); waited += 100; } active() }; #[cfg(unix)] let survivors = { let _ = v3; let _ = std::process::Command::new("kill").arg("-KILL").arg("--").arg(format!("-{root}")).status(); let _ = child.kill(); let _ = child.wait(); std::thread::sleep(std::time::Duration::from_millis(300)); std::fs::read_dir("/proc").map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| std::fs::read_to_string(entry.path().join("stat")).ok().and_then(|stat| stat.rsplit_once(char::from(41u8)).map(|(_, rest)| rest.split_whitespace().nth(2) == Some(root.to_string().as_str()))).unwrap_or(false)).count() as u64).unwrap_or(0) }; survivors };
    let v5: bool = v4 == 0u64;
    let v8: US1 = if v5 {
        US1::US1_TreeStopped
    } else {
        US1::US1_SurvivorsLeft(v4)
    };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = job_file_5(v1.clone(), v9.clone());
    let v11: u64 = now_ms_9();
    let (mut v21, mut v22, mut v23): (i32, Rc<str>, Rc<str>) = match &v2 {
        US0::US0_HaltOnMemory => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("private memory cap reached"); } LIT.with(|lit| lit.clone()) };
            (137i32, v14.clone(), v15.clone())
        }
        US0::US0_HaltOnRequest => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            (143i32, v12.clone(), v13.clone())
        }
        _ => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout"); } LIT.with(|lit| lit.clone()) };
            (124i32, v16.clone(), v17.clone())
        }
    };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = match &v8 {
        US1::US1_SurvivorsLeft(v26) => {
            let v26: u64 = *v26;
            let v27: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
            v30.clone()
        }
        US1::US1_TreeStopped => {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
            v25.clone()
        }
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v32, &*v33, &*v33].concat());
    let v35: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v36: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
    let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v37, &*v34, &*v38, &*v33].concat());
    let v40: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v37, &*v36, &*v37, &*v39].concat());
    let v41: Rc<str> = text_written_10(v10.clone(), v40.clone());
    let v42: u64 = kit_machine_windows::kit_job_windows_dropped(&*v1);
    let v52: i32 = match &v2 {
        US0::US0_HaltOnMemory => {
            137i32
        }
        US0::US0_HaltOnRequest => {
            143i32
        }
        _ => {
            124i32
        }
    };
    v52
}
fn container_peak_mb_11() -> u64 {
    let mut v0: u64 = child_container_4();
    let v1: u64 = { #[cfg(windows)] let peak = { unsafe extern "system" { fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; } let job = v0 as *mut std::ffi::c_void; let mut limits = [0u64; 18]; if job.is_null() || unsafe { QueryInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144, std::ptr::null_mut()) } == 0 { 0 } else { limits[17] / 1048576 } }; #[cfg(not(windows))] let peak = { let _ = v0; 0u64 }; peak };
    v1
}
fn method14(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_12(mut v0: u64) -> Rc<str> {
    format_real_13(v0)
}
fn settled_word_15(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_JobExpiredInQueue => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US2::US2_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US2::US2_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US2::US2_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US2::US2_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn text_or_empty_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn gate_wait_of_18(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_16(v2.clone());
    let v8: bool = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = job_file_5(v0.clone(), v4.clone());
        let mut v6: bool = file_exists_16(v5.clone());
        let v7: bool = v6 == false;
        v7
    } else {
        false
    };
    if v8 {
        let v9: Rc<str> = job_file_5(v0.clone(), v1.clone());
        let v10: Rc<str> = text_or_empty_19(v9.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v11).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v13: Rc<str> = v12.clone().borrow()[0i32 as usize].clone();
        let v14: i32 = (v13.clone().len() as i32);
        let v15: i32 = 0i32;
        let v16: i32 = method20(v13.clone(), v14, v15);
        let v17: i32 = v14.wrapping_sub(1i32);
        let v18: Rc<str> = string_slice(&v13.clone(), v16 as i64, v17 as i64);
        let v19: i32 = (v18.clone().len() as i32);
        let v20: i32 = method21(v18.clone(), v19);
        let v21: Rc<str> = string_slice(&v18.clone(), 0i32 as i64, v20 as i64);
        US3::US3_WaitingAfter(v21.clone())
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = job_file_5(v0.clone(), v23.clone());
        let v25: bool = file_exists_16(v24.clone());
        if v25 {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = job_file_5(v0.clone(), v26.clone());
            let v28: Rc<str> = text_or_empty_19(v27.clone());
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v28.starts_with(&*v29);
            if v30 {
                US3::US3_NotWaiting
            } else {
                let v32: Rc<str> = job_file_5(v0.clone(), v23.clone());
                let v33: Rc<str> = text_or_empty_19(v32.clone());
                let v34: i32 = (v33.clone().len() as i32);
                let v35: i32 = 0i32;
                let v36: i32 = method20(v33.clone(), v34, v35);
                let mut v37: i32 = v34.wrapping_sub(1i32);
                let v38: Rc<str> = string_slice(&v33.clone(), v36 as i64, v37 as i64);
                let v39: i32 = (v38.clone().len() as i32);
                let v40: i32 = method21(v38.clone(), v39);
                let v41: Rc<str> = string_slice(&v38.clone(), 0i32 as i64, v40 as i64);
                US3::US3_WaitingWindow(v41.clone())
            }
        } else {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = job_file_5(v0.clone(), v44.clone());
            let v46: bool = file_exists_16(v45.clone());
            let v53: bool = if v46 {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = job_file_5(v0.clone(), v47.clone());
                let v49: Rc<str> = text_or_empty_19(v48.clone());
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy: acquired slot"); } LIT.with(|lit| lit.clone()) };
                let v51: bool = v49.contains(&*v50);
                let v52: bool = v51 == false;
                v52
            } else {
                false
            };
            if v53 {
                US3::US3_WaitingSlot
            } else {
                US3::US3_NotWaiting
            }
        }
    }
}
fn gate_queued_17(mut v0: Rc<str>) -> bool {
    let v1: US3 = gate_wait_of_18(v0);
    let v2: bool = match &v1 {
        US3::US3_NotWaiting => {
            true
        }
        _ => {
            false
        }
    };
    let mut v3: bool = v2 != true;
    v3
}
fn supervised_2(mut v0: std::rc::Rc<std::cell::RefCell<std::process::Child>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64) -> i32 {
    loop {
        let mut v6: i64 = match v0.clone().borrow_mut().try_wait() { Ok(Some(status)) => status.code().map(|code| code as i64).unwrap_or(-2), Ok(None) => -1, Err(_) => -2 };
        let v7: bool = v4 > 0u64;
        let v9: u64 = if v7 {
            container_peak_mb_11()
        } else {
            0u64
        };
        let v10: bool = v6 != -1i64;
        let v15: bool = if v10 {
            if v7 {
                let v11: u64 = v9.wrapping_mul(100u64);
                let v12: u64 = v4.wrapping_mul(98u64);
                let v13: bool = v11 >= v12;
                v13
            } else {
                false
            }
        } else {
            false
        };
        if v15 {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = job_file_5(v1.clone(), v16.clone());
            let v18: Rc<str> = decimal_text_12(v9);
            let v19: Rc<str> = text_written_10(v17.clone(), v18.clone());
            let v20: US0 = US0::US0_HaltOnMemory;
            return halted_3(v0.clone(), v1.clone(), v20.clone());
        } else {
            if v10 {
                let v22: i32 = (v6 as i32);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = job_file_5(v1.clone(), v23.clone());
                let v25: u64 = now_ms_9();
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v27: US2 = US2::US2_JobFinished(v22);
                let v28: Rc<str> = settled_word_15(v27.clone());
                let v29: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                let v30: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v31, &*v26, &*v32, &*v26].concat());
                let v34: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v31, &*v30, &*v31, &*v33].concat());
                let v35: Rc<str> = text_written_10(v24.clone(), v34.clone());
                let v36: u64 = kit_machine_windows::kit_job_windows_dropped(&*v1);
                return v22;
            } else {
                let mut v37: u64 = now_ms_9();
                let v38: bool = v37 > v2;
                let v43: bool = if v38 {
                    true
                } else {
                    let v39: bool = v5 != 0u64;
                    if v39 {
                        let v40: u64 = now_ms_9();
                        let v41: bool = v40 > v5;
                        v41
                    } else {
                        false
                    }
                };
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = job_file_5(v1.clone(), v44.clone());
                let v46: bool = file_exists_16(v45.clone());
                let v50: bool = if v7 {
                    let v47: u64 = v9.wrapping_mul(100u64);
                    let v48: u64 = v4.wrapping_mul(98u64);
                    let v49: bool = v47 >= v48;
                    v49
                } else {
                    false
                };
                let v57: US0 = if v46 {
                    US0::US0_HaltOnRequest
                } else {
                    if v50 {
                        US0::US0_HaltOnMemory
                    } else {
                        if v43 {
                            US0::US0_HaltOnTimeout
                        } else {
                            US0::US0_KeepRunning
                        }
                    }
                };
                match &v57 {
                    US0::US0_KeepRunning => {
                        let v58: u64 = now_ms_9();
                        let v59: u64 = v3.wrapping_add(1000u64);
                        let v60: bool = v58 > v59;
                        let v71: u64 = if v60 {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
                            let v62: Rc<str> = job_file_5(v1.clone(), v61.clone());
                            let v63: Rc<str> = decimal_text_12(v58);
                            let v64: Rc<str> = text_written_10(v62.clone(), v63.clone());
                            let v70: Rc<str> = if v7 {
                                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
                                let v66: Rc<str> = job_file_5(v1.clone(), v65.clone());
                                let v67: Rc<str> = decimal_text_12(v9);
                                text_written_10(v66.clone(), v67.clone())
                            } else {
                                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v69.clone()
                            };
                            v58
                        } else {
                            v3
                        };
                        let v73: bool = if v60 {
                            gate_queued_17(v1.clone())
                        } else {
                            false
                        };
                        let v76: u64 = if v73 {
                            let v74: u64 = v58.wrapping_sub(v3);
                            let v75: u64 = v2.wrapping_add(v74);
                            v75
                        } else {
                            v2
                        };
                        let v77: u64 = 200u64;
                        std::thread::sleep(std::time::Duration::from_millis(v77));
                        (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v76, v71, v4, v5);
                        continue;
                    }
                    _ => {
                        let v84: Rc<str> = if v7 {
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("peak_private_mb"); } LIT.with(|lit| lit.clone()) };
                            let v80: Rc<str> = job_file_5(v1.clone(), v79.clone());
                            let v81: Rc<str> = decimal_text_12(v9);
                            text_written_10(v80.clone(), v81.clone())
                        } else {
                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v83.clone()
                        };
                        return halted_3(v0.clone(), v1.clone(), v57.clone());
                    }
                }
            }
        }
    }
}
fn method23(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_22(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method23(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure1();
    let mut v4: Option<US4> = v2.map(|x| v3(x));
    let mut v6: US4 = US4::US4_None;
    let v7: US4 = v6.clone();
    let v8: US4 = v4.unwrap_or(v7);
    match &v8 {
        US4::US4_None => {
            0u64
        }
        US4::US4_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn container_capped_24(mut v0: u64) -> bool {
    let v1: u64 = child_container_4();
    let v2: bool = { #[cfg(windows)] let capped = { unsafe extern "system" { fn QueryInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32, returned: *mut u32) -> i32; fn SetInformationJobObject(job: *mut std::ffi::c_void, class: i32, information: *mut std::ffi::c_void, length: u32) -> i32; } let job = v1 as *mut std::ffi::c_void; let mut limits = [0u64; 18]; !job.is_null() && unsafe { QueryInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144, std::ptr::null_mut()) } != 0 && { limits[2] |= 0x200; limits[15] = (v0).saturating_mul(1048576); (unsafe { SetInformationJobObject(job, 9, limits.as_mut_ptr().cast(), 144) } != 0) } }; #[cfg(not(windows))] let capped = { let _ = (v1, v0); false }; capped };
    v2
}
fn supervise_cleared_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_16(v2.clone());
    let v10: i32 = if v3 {
        let mut v4: i32 = kit_job_gate::kit_job_window_gate(&*v0);
        v4
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = job_file_5(v0.clone(), v5.clone());
        let v7: bool = file_exists_16(v6.clone());
        if v7 {
            let v8: i32 = kit_job_gate::kit_job_compile_gate(&*v0);
            v8
        } else {
            0i32
        }
    };
    let v11: bool = v10 != 0i32;
    if v11 {
        v10
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = job_file_5(v0.clone(), v12.clone());
        let v14: Rc<str> = text_or_empty_19(v13.clone());
        let v15: u64 = decimal_u64_22(v14.clone());
        let v16: bool = v15 > 0u64;
        let v19: bool = if v16 {
            let v17: bool = container_capped_24(v15);
            let v18: bool = v17 == false;
            v18
        } else {
            false
        };
        let v25: Rc<str> = if v19 {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_unenforced"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = job_file_5(v0.clone(), v20.clone());
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the job object refused the memory limit; the supervisor still stops the tree at the cap"); } LIT.with(|lit| lit.clone()) };
            text_written_10(v21.clone(), v22.clone())
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = job_file_5(v0.clone(), v26.clone());
        let v28: Rc<str> = text_or_empty_19(v27.clone());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = job_file_5(v0.clone(), v29.clone());
        let v31: Rc<str> = text_or_empty_19(v30.clone());
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = job_file_5(v0.clone(), v32.clone());
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = job_file_5(v0.clone(), v34.clone());
        let v36: u64 = child_container_4();
        let mut v37: Result<std::rc::Rc<std::cell::RefCell<std::process::Child>>, std::rc::Rc<str>> = { let mut parts = v28.lines(); let program = parts.next().unwrap_or_default().to_owned(); let arguments = parts.map(str::to_owned).collect::<Vec<_>>(); let outcome = (|| -> std::io::Result<std::process::Child> { let stdout = std::fs::File::create(&*v33)?; let stderr = std::fs::File::create(&*v35)?; let mut command = std::process::Command::new(&program); command.args(&arguments).current_dir(&*v31).stdin(std::process::Stdio::null()).stdout(stdout).stderr(stderr); #[cfg(windows)] { use std::os::windows::process::CommandExt; command.creation_flags(0x0800_0000).env_remove("NoDefaultCurrentDirectoryInExePath"); } #[cfg(unix)] { use std::os::unix::process::CommandExt; command.process_group(0); } let child = command.spawn()?; #[cfg(windows)] { unsafe extern "system" { fn AssignProcessToJobObject(job: *mut std::ffi::c_void, process: *mut std::ffi::c_void) -> i32; } use std::os::windows::io::AsRawHandle; if v36 != 0 { unsafe { AssignProcessToJobObject(v36 as *mut std::ffi::c_void, child.as_raw_handle()); } } } #[cfg(not(windows))] { let _ = v36; } Ok(child) })(); outcome.map(|child| std::rc::Rc::new(std::cell::RefCell::new(child))).map_err(|error| std::rc::Rc::<str>::from(format!("spawn {program}: {error}"))) };
        let v38: bool = v37.is_ok();
        if v38 {
            let v39: std::rc::Rc<std::cell::RefCell<std::process::Child>> = v37.clone().unwrap();
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = job_file_5(v0.clone(), v40.clone());
            let v42: u64 = v39.borrow().id() as u64;
            let v43: Rc<str> = decimal_text_12(v42);
            let v44: Rc<str> = text_written_10(v41.clone(), v43.clone());
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = job_file_5(v0.clone(), v45.clone());
            let v47: u64 = now_ms_9();
            let v48: Rc<str> = decimal_text_12(v47);
            let v49: Rc<str> = text_written_10(v46.clone(), v48.clone());
            let v50: u64 = now_ms_9();
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = job_file_5(v0.clone(), v51.clone());
            let v53: Rc<str> = text_or_empty_19(v52.clone());
            let v54: u64 = decimal_u64_22(v53.clone());
            let v55: u64 = v50.wrapping_add(v54);
            let v56: u64 = now_ms_9();
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
            let v58: Rc<str> = job_file_5(v0.clone(), v57.clone());
            let v59: Rc<str> = text_or_empty_19(v58.clone());
            let v60: u64 = decimal_u64_22(v59.clone());
            supervised_2(v39.clone(), v0.clone(), v55, v56, v15, v60)
        } else {
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<str> = job_file_5(v0.clone(), v62.clone());
            let v64: u64 = now_ms_9();
            let v65: Rc<str> = v37.clone().err().unwrap_or_default();
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v67: US2 = US2::US2_JobSpawnFailed(v66.clone());
            let v68: Rc<str> = settled_word_15(v67.clone());
            let v69: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v70: Rc<str> = std::rc::Rc::<str>::from((v64).to_string());
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v73: Rc<str> = std::rc::Rc::<str>::from([&*v68, &*v71, &*v65, &*v72, &*v66].concat());
            let v74: Rc<str> = std::rc::Rc::<str>::from([&*v69, &*v71, &*v70, &*v71, &*v73].concat());
            let v75: Rc<str> = text_written_10(v63.clone(), v74.clone());
            125i32
        }
    }
}
fn supervise_job_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_5(v0.clone(), v1.clone());
    let mut v3: u64 = std::process::id() as u64;
    let mut v4: Rc<str> = decimal_text_12(v3);
    let v5: Rc<str> = text_written_10(v2.clone(), v4.clone());
    kit_job_env::kit_job_environment_adopted(&*v0);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = job_file_5(v0.clone(), v6.clone());
    let v8: bool = file_exists_16(v7.clone());
    let v10: i32 = if v8 {
        let v9: i32 = kit_job_after::kit_job_after_wait(&*v0);
        v9
    } else {
        0i32
    };
    let v11: bool = v10 != 0i32;
    if v11 {
        v10
    } else {
        supervise_cleared_1(v0.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        supervise_job_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_supervise(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
