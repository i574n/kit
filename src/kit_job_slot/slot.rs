#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_TreeStopped,
    US0_SurvivorsLeft(u64),
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
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
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn joined_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_6();
    joined_4(v1, v0)
}
fn job_file_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_5(v0);
    joined_4(v2, v1)
}
fn method9(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_7(mut v0: u64) -> Rc<str> {
    format_real_8(v0)
}
fn text_written_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn file_removed_12(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn text_or_empty_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method16(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_15(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method16(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let mut v6: US1 = US1::US1_None;
    let v7: US1 = v6.clone();
    let v8: US1 = v4.unwrap_or(v7);
    match &v8 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn slot_deadline_passed_13(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_14(v3.clone());
    let v5: u64 = decimal_u64_15(v4.clone());
    let mut v6: bool = v5 != 0u64;
    if v6 {
        let v7: bool = v1 > v5;
        v7
    } else {
        false
    }
}
fn settled_word_17(mut v0: US2) -> Rc<str> {
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
fn tickets_sorted_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".t")).collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
    v1.clone()
}
fn first_row_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn ticket_pid_21(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).trim_end_matches(".t").split(char::from(45u8)).nth(1).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_22(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn other_rows_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn first_live_ticket_19(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: bool = &*v0 == "";
        if v1 {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v2.clone();
        } else {
            let mut v3: Rc<str> = first_row_20(v0.clone());
            let mut v4: u64 = ticket_pid_21(v3.clone());
            let v5: bool = process_alive_22(v4);
            if v5 {
                return v3.clone();
            } else {
                let mut v6: Rc<str> = other_rows_23(v0.clone());
                v0 = v6.clone();
                continue;
            }
        }
    }
}
fn free_mb_24() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn slot_free_25(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn probe(name: &str) -> bool { unsafe extern "system" { fn OpenMutexW(access: u32, inherit: i32, name: *const u16) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn ReleaseMutex(handle: *mut std::ffi::c_void) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } let name = name.encode_utf16().chain(std::iter::once(0u16)).collect::<Vec<u16>>(); unsafe { let handle = OpenMutexW(0x0010_0001, 0, name.as_ptr()); if handle.is_null() { return true; } let state = WaitForSingleObject(handle, 0); let taken = state == 0 || state == 0x80; if taken { ReleaseMutex(handle); } CloseHandle(handle); taken } } #[cfg(not(windows))] fn probe(_name: &str) -> bool { true } let base = std::env::var("KIT_HEAVY_MUTEX").unwrap_or_else(|_| format!("Global{}spiral-heavy-job", char::from(92u8))); probe(&base) || (v0 > 3072 && probe(&format!("{base}-2"))) };
    v1
}
fn ticket_adopted_26(mut v0: Rc<str>) -> () {
    unsafe { std::env::set_var("SPIRAL_HEAVY_TICKET", &*v0); };
    ()
}
fn slot_loop_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    loop {
        let mut v4: u64 = now_ms_2();
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = job_file_3(v0.clone(), v5.clone());
        let v7: Rc<str> = decimal_text_7(v4);
        let v8: Rc<str> = text_written_10(v6.clone(), v7.clone());
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = job_file_3(v0.clone(), v9.clone());
        let v11: bool = file_exists_11(v10.clone());
        if v11 {
            let mut v12: Rc<str> = joined_4(v1.clone(), v2.clone());
            let v13: bool = file_removed_12(v12.clone());
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = job_file_3(v0.clone(), v14.clone());
            let mut v16: u64 = 0u64;
            let v17: bool = v16 == 0u64;
            let v20: US0 = if v17 {
                US0::US0_TreeStopped
            } else {
                US0::US0_SurvivorsLeft(v16)
            };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = match &v20 {
                US0::US0_SurvivorsLeft(v25) => {
                    let v25: u64 = *v25;
                    let v26: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v27, &*v28, &*v28, &*v28].concat());
                    v29.clone()
                }
                US0::US0_TreeStopped => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
            };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v31, &*v32, &*v32].concat());
            let v34: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v35: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v36, &*v33, &*v37, &*v32].concat());
            let v39: Rc<str> = std::rc::Rc::<str>::from([&*v34, &*v36, &*v35, &*v36, &*v38].concat());
            let v40: Rc<str> = text_written_10(v15.clone(), v39.clone());
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = job_file_3(v0.clone(), v41.clone());
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled ticket="); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v2));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state=stopped"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
            let v47: Rc<str> = text_written_10(v42.clone(), v46.clone());
            return 143i32;
        } else {
            let v48: bool = slot_deadline_passed_13(v0.clone(), v4);
            if v48 {
                let v49: Rc<str> = joined_4(v1.clone(), v2.clone());
                let v50: bool = file_removed_12(v49.clone());
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = job_file_3(v0.clone(), v51.clone());
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no heavy slot within --slot-within; the job never started"); } LIT.with(|lit| lit.clone()) };
                let v54: US2 = US2::US2_JobExpiredInQueue;
                let v55: Rc<str> = settled_word_17(v54.clone());
                let v56: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let v57: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v58, &*v53, &*v59, &*v60].concat());
                let v62: Rc<str> = std::rc::Rc::<str>::from([&*v56, &*v58, &*v57, &*v58, &*v61].concat());
                let v63: Rc<str> = text_written_10(v52.clone(), v62.clone());
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<str> = job_file_3(v0.clone(), v64.clone());
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled ticket="); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v2));
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state=expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                let v70: Rc<str> = text_written_10(v65.clone(), v69.clone());
                return 75i32;
            } else {
                let v71: Rc<str> = tickets_sorted_18(v1.clone());
                let v72: Rc<str> = first_live_ticket_19(v71.clone());
                let v73: bool = &*v72 == &*v2;
                let v76: bool = if v73 {
                    let v74: u64 = free_mb_24();
                    slot_free_25(v74)
                } else {
                    false
                };
                if v76 {
                    ticket_adopted_26(v2.clone());
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v78: Rc<str> = job_file_3(v0.clone(), v77.clone());
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready ticket="); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v2));
                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                    let v83: u64 = v4.wrapping_sub(v3);
                    let v84: Rc<str> = decimal_text_7(v83);
                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v84));
                    let v86: Rc<str> = text_written_10(v78.clone(), v85.clone());
                    return 0i32;
                } else {
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<str> = job_file_3(v0.clone(), v87.clone());
                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting ticket="); } LIT.with(|lit| lit.clone()) };
                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v2));
                    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" first="); } LIT.with(|lit| lit.clone()) };
                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                    let v93: Rc<str> = tickets_sorted_18(v1.clone());
                    let v94: Rc<str> = first_live_ticket_19(v93.clone());
                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v94));
                    let v96: Rc<str> = text_written_10(v88.clone(), v95.clone());
                    let v97: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v97));
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v3);
                    continue;
                }
            }
        }
    }
}
fn heavy_queue_27() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_QUEUE").map(|queue| std::path::PathBuf::from(queue).to_string_lossy().into_owned()).or_else(|| std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("HOME")).map(|base| std::path::PathBuf::from(base).join("spiral-bin").join("heavy-queue").to_string_lossy().into_owned())).unwrap_or_default().as_str());
    v0.clone()
}
fn ticket_ticks_29(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).split(char::from(45u8)).next().and_then(|ticks| ticks.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn own_pid_30() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn closure2(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let mut v4: bool = v2 >= v0;
        if v4 {
            v3.clone()
        } else {
            let v5: i32 = v2.wrapping_add(1i32);
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method31(v0, v1.clone(), v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
            v6(v7.clone())
        }
    })
}
fn method31(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure2(v0, v1, v2)
}
fn ticket_renamed_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = std::fs::rename(std::path::Path::new(&*v0).join(&*v1), std::path::Path::new(&*v0).join(&*v2)).is_ok();
    v3
}
fn handed_ticket_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_14(v3.clone());
    let v5: u64 = ticket_ticks_29(v4.clone());
    let mut v6: u64 = own_pid_30();
    let v7: Rc<str> = format_real_8(v5);
    let v8: Rc<str> = Rc::<str>::from((b'0' as char).encode_utf8(&mut [0u8; 4]) as &str);
    let v9: i32 = (v7.clone().len() as i32);
    let v10: i32 = 20i32.wrapping_sub(v9);
    let v11: i32 = 0i32;
    let mut v12: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method31(v10, v8.clone(), v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = v12(v13.clone());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v7));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = format_real_8(v6);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: bool = &*v4 != "";
    let v24: bool = if v22 {
        ticket_renamed_32(v1.clone(), v4.clone(), v21.clone())
    } else {
        false
    };
    if v24 {
        v21.clone()
    } else {
        v13.clone()
    }
}
fn ticket_created_34(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let ticks = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_nanos() as u64 / 100).unwrap_or(0) + 621355968000000000u64; let name = format!("{:020}-{}.t", ticks, std::process::id()); let _ = std::fs::create_dir_all(&*v0); match std::fs::write(std::path::Path::new(&*v0).join(&name), b"") { Ok(()) => name, Err(_) => String::new() } }.as_str());
    v1.clone()
}
fn ticket_written_35(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = std::fs::create_dir_all(&*v0).and_then(|_| std::fs::write(std::path::Path::new(&*v0).join(&*v1), b"")).is_ok();
    v2
}
fn front_ticket_33(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = tickets_sorted_18(v0.clone());
    let v2: Rc<str> = first_live_ticket_19(v1.clone());
    let mut v3: bool = &*v2 == "";
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: u64 = ticket_ticks_29(v2.clone());
        let v5: bool = v4 == 0u64;
        v5
    };
    if v6 {
        ticket_created_34(v0.clone())
    } else {
        let v8: u64 = ticket_ticks_29(v2.clone());
        let v9: u64 = own_pid_30();
        let v10: u64 = v8.wrapping_sub(1u64);
        let v11: Rc<str> = format_real_8(v10);
        let mut v12: Rc<str> = Rc::<str>::from((b'0' as char).encode_utf8(&mut [0u8; 4]) as &str);
        let v13: i32 = (v11.clone().len() as i32);
        let v14: i32 = 20i32.wrapping_sub(v13);
        let v15: i32 = 0i32;
        let mut v16: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method31(v14, v12.clone(), v15);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = v16(v17.clone());
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let v22: Rc<str> = format_real_8(v9);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        let v26: bool = ticket_written_35(v0.clone(), v25.clone());
        if v26 {
            v25.clone()
        } else {
            v17.clone()
        }
    }
}
fn slot_gate_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = heavy_queue_27();
    let v2: bool = &*v1 == "";
    let v5: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        handed_ticket_28(v0.clone(), v1.clone())
    };
    let v7: bool = if v2 {
        true
    } else {
        let mut v6: bool = &*v5 != "";
        v6
    };
    let v14: Rc<str> = if v7 {
        v5.clone()
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = job_file_3(v0.clone(), v8.clone());
        let v10: bool = file_exists_11(v9.clone());
        if v10 {
            front_ticket_33(v1.clone())
        } else {
            ticket_created_34(v1.clone())
        }
    };
    let v15: bool = &*v14 == "";
    if v15 {
        0i32
    } else {
        let mut v16: u64 = now_ms_2();
        slot_loop_1(v0.clone(), v1.clone(), v14.clone(), v16)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        slot_gate_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn held_ticket_37(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_14(v3.clone());
    let v5: bool = &*v4 != "";
    let v8: bool = if v5 {
        let mut v6: Rc<str> = joined_4(v1.clone(), v4.clone());
        file_exists_11(v6.clone())
    } else {
        false
    };
    if v8 {
        v4.clone()
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = job_file_3(v0.clone(), v9.clone());
        let v11: bool = file_exists_11(v10.clone());
        let v14: Rc<str> = if v11 {
            front_ticket_33(v1.clone())
        } else {
            ticket_created_34(v1.clone())
        };
        let v15: Rc<str> = job_file_3(v0.clone(), v2.clone());
        let mut v16: Rc<str> = text_written_10(v15.clone(), v14.clone());
        let v17: Rc<str> = joined_4(v1.clone(), v14.clone());
        let v18: Rc<str> = text_written_10(v17.clone(), v0.clone());
        v14.clone()
    }
}
fn abandoned_swept_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v3: Rc<str> = joined_4(v0.clone(), v1.clone());
    let mut v4: Rc<str> = text_or_empty_14(v3.clone());
    let v5: Rc<str> = first_row_20(v4.clone());
    let mut v6: Rc<str> = other_rows_23(v4.clone());
    let v7: u64 = decimal_u64_15(v6.clone());
    let v8: Rc<str> = jobs_root_6();
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = joined_4(v8.clone(), v9.clone());
    let v11: Rc<str> = joined_4(v10.clone(), v5.clone());
    let mut v12: bool = file_exists_11(v11.clone());
    let v13: bool = &*v1 == "";
    let v17: bool = if v13 {
        true
    } else {
        let v14: u64 = ticket_pid_21(v1.clone());
        let v15: u64 = own_pid_30();
        let mut v16: bool = v14 != v15;
        v16
    };
    let v18: bool = v17 || v12;
    let v23: bool = if v18 {
        true
    } else {
        let v19: bool = v7 != 0u64;
        if v19 {
            let v20: u64 = v2.wrapping_sub(v7);
            let v21: bool = v20 < 60000u64;
            v21
        } else {
            false
        }
    };
    if v23 {
        ()
    } else {
        let v24: Rc<str> = joined_4(v0.clone(), v1.clone());
        let v25: bool = file_removed_12(v24.clone());
        ()
    }
}
fn slot_held_step_36(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = heavy_queue_27();
    let v2: bool = &*v1 == "";
    let v7: bool = if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = job_file_3(v0.clone(), v3.clone());
        let v5: bool = file_exists_11(v4.clone());
        let mut v6: bool = v5 == false;
        v6
    };
    if v7 {
        0i32
    } else {
        let v8: u64 = now_ms_2();
        let v9: Rc<str> = held_ticket_37(v0.clone(), v1.clone());
        let v10: Rc<str> = tickets_sorted_18(v1.clone());
        let v11: Rc<str> = first_live_ticket_19(v10.clone());
        abandoned_swept_38(v1.clone(), v11.clone(), v8);
        let mut v12: Rc<str> = tickets_sorted_18(v1.clone());
        let v13: Rc<str> = first_live_ticket_19(v12.clone());
        let v14: bool = &*v9 == "";
        if v14 {
            0i32
        } else {
            let v15: bool = slot_deadline_passed_13(v0.clone(), v8);
            if v15 {
                let mut v16: Rc<str> = joined_4(v1.clone(), v9.clone());
                let v17: bool = file_removed_12(v16.clone());
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = job_file_3(v0.clone(), v18.clone());
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no heavy slot within --slot-within; the job never started"); } LIT.with(|lit| lit.clone()) };
                let v21: US2 = US2::US2_JobExpiredInQueue;
                let v22: Rc<str> = settled_word_17(v21.clone());
                let v23: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let v24: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v20, &*v26, &*v27].concat());
                let v29: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v25, &*v24, &*v25, &*v28].concat());
                let v30: Rc<str> = text_written_10(v19.clone(), v29.clone());
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = job_file_3(v0.clone(), v31.clone());
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled ticket="); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v9));
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state=expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                let v37: Rc<str> = text_written_10(v32.clone(), v36.clone());
                2i32
            } else {
                let v38: bool = &*v13 == &*v9;
                let v41: bool = if v38 {
                    let v39: u64 = free_mb_24();
                    slot_free_25(v39)
                } else {
                    false
                };
                if v41 {
                    let v42: Rc<str> = joined_4(v1.clone(), v9.clone());
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v43));
                    let v45: Rc<str> = decimal_text_7(v8);
                    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
                    let v47: Rc<str> = text_written_10(v42.clone(), v46.clone());
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = job_file_3(v0.clone(), v48.clone());
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handed ticket="); } LIT.with(|lit| lit.clone()) };
                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v9));
                    let v52: Rc<str> = text_written_10(v49.clone(), v51.clone());
                    0i32
                } else {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v54: Rc<str> = job_file_3(v0.clone(), v53.clone());
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held ticket="); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v9));
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" first="); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v56, v57)), v13));
                    let v60: Rc<str> = text_written_10(v54.clone(), v59.clone());
                    1i32
                }
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        slot_held_step_36(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn slot_ticket_dropped_39(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_3(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_14(v2.clone());
    let mut v4: bool = &*v3 == "";
    if v4 {
        ()
    } else {
        let v5: Rc<str> = heavy_queue_27();
        let mut v6: Rc<str> = joined_4(v5.clone(), v3.clone());
        let v7: bool = file_removed_12(v6.clone());
        ()
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        slot_ticket_dropped_39(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_slot_gate(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_job_slot_held_step(v0: &str) -> i32 {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_job_slot_ticket_dropped(v0: &str) -> () {
    closure4()(Rc::<str>::from(v0))
}
