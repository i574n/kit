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
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
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
fn concat5_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn text_or_empty_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method17(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_16(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method17(v1.clone());
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
fn slot_deadline_passed_14(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0, v2);
    let mut v4: Rc<str> = text_or_empty_15(v3);
    let v5: u64 = decimal_u64_16(v4);
    let mut v6: bool = v5 != 0u64;
    if v6 {
        let v7: bool = v1 > v5;
        v7
    } else {
        false
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
    let v1: bool = kit_process_alive::kit_process_alive(v0);
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
fn set_environment_variable_27(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    unsafe { std::env::set_var(&*v0, &*v1) };
    ()
}
fn ticket_adopted_26(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_HEAVY_TICKET"); } LIT.with(|lit| lit.clone()) };
    set_environment_variable_27(v1, v0)
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
                    concat5_13(v26.clone(), v27.clone(), v28.clone(), v28.clone(), v28.clone())
                }
                US0::US0_TreeStopped => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
            };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = concat5_13(v22.clone(), v23.clone(), v31.clone(), v32.clone(), v32.clone());
            let v34: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v35: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = concat5_13(v21.clone(), v36.clone(), v33.clone(), v37.clone(), v32.clone());
            let v39: Rc<str> = concat5_13(v34.clone(), v36.clone(), v35.clone(), v36.clone(), v38.clone());
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
            let v48: bool = slot_deadline_passed_14(v0.clone(), v4);
            if v48 {
                let v49: Rc<str> = joined_4(v1.clone(), v2.clone());
                let v50: bool = file_removed_12(v49.clone());
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = job_file_3(v0.clone(), v51.clone());
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no heavy slot within --slot-within; the job never started"); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v55: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let v56: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v60: Rc<str> = concat5_13(v54.clone(), v57.clone(), v53.clone(), v58.clone(), v59.clone());
                let v61: Rc<str> = concat5_13(v55.clone(), v57.clone(), v56.clone(), v57.clone(), v60.clone());
                let v62: Rc<str> = text_written_10(v52.clone(), v61.clone());
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = job_file_3(v0.clone(), v63.clone());
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled ticket="); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v2));
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state=expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
                let v69: Rc<str> = text_written_10(v64.clone(), v68.clone());
                return 75i32;
            } else {
                let v70: Rc<str> = tickets_sorted_18(v1.clone());
                let v71: Rc<str> = first_live_ticket_19(v70.clone());
                let v72: bool = &*v71 == &*v2;
                let v75: bool = if v72 {
                    let v73: u64 = free_mb_24();
                    slot_free_25(v73)
                } else {
                    false
                };
                if v75 {
                    ticket_adopted_26(v2.clone());
                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v77: Rc<str> = job_file_3(v0.clone(), v76.clone());
                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ready ticket="); } LIT.with(|lit| lit.clone()) };
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v2));
                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                    let v82: u64 = v4.wrapping_sub(v3);
                    let v83: Rc<str> = decimal_text_7(v82);
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v83));
                    let v85: Rc<str> = text_written_10(v77.clone(), v84.clone());
                    return 0i32;
                } else {
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v87: Rc<str> = job_file_3(v0.clone(), v86.clone());
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting ticket="); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v2));
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" first="); } LIT.with(|lit| lit.clone()) };
                    let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
                    let v92: Rc<str> = tickets_sorted_18(v1.clone());
                    let v93: Rc<str> = first_live_ticket_19(v92.clone());
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v93));
                    let v95: Rc<str> = text_written_10(v87.clone(), v94.clone());
                    let v96: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v96));
                    continue;
                }
            }
        }
    }
}
fn heavy_queue_28() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_QUEUE").map(|queue| std::path::PathBuf::from(queue).to_string_lossy().into_owned()).or_else(|| std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("HOME")).map(|base| std::path::PathBuf::from(base).join("spiral-bin").join("heavy-queue").to_string_lossy().into_owned())).unwrap_or_default().as_str());
    v0.clone()
}
fn ticket_ticks_30(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).split(char::from(45u8)).next().and_then(|ticks| ticks.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn own_pid_31() -> u64 {
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
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method32(v0, v1.clone(), v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
            v6(v7.clone())
        }
    })
}
fn method32(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure2(v0, v1, v2)
}
fn ticket_renamed_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = std::fs::rename(std::path::Path::new(&*v0).join(&*v1), std::path::Path::new(&*v0).join(&*v2)).is_ok();
    v3
}
fn handed_ticket_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0, v2);
    let mut v4: Rc<str> = text_or_empty_15(v3);
    let v5: u64 = ticket_ticks_30(v4.clone());
    let mut v6: u64 = own_pid_31();
    let v7: Rc<str> = format_real_8(v5);
    let v8: Rc<str> = Rc::<str>::from(((b'0') as char).to_string());
    let v9: i32 = (v7.clone().len() as i32);
    let v10: i32 = 20i32.wrapping_sub(v9);
    let v11: i32 = 0i32;
    let mut v12: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method32(v10, v8, v11);
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
        ticket_renamed_33(v1, v4, v21.clone())
    } else {
        false
    };
    if v24 {
        v21
    } else {
        v13
    }
}
fn ticket_created_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let ticks = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_nanos() as u64 / 100).unwrap_or(0) + 621355968000000000u64; let name = format!("{:020}-{}.t", ticks, std::process::id()); let _ = std::fs::create_dir_all(&*v0); match std::fs::write(std::path::Path::new(&*v0).join(&name), b"") { Ok(()) => name, Err(_) => String::new() } }.as_str());
    v1.clone()
}
fn ticket_written_36(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = std::fs::create_dir_all(&*v0).and_then(|_| std::fs::write(std::path::Path::new(&*v0).join(&*v1), b"")).is_ok();
    v2
}
fn front_ticket_34(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = tickets_sorted_18(v0.clone());
    let v2: Rc<str> = first_live_ticket_19(v1);
    let mut v3: bool = &*v2 == "";
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: u64 = ticket_ticks_30(v2.clone());
        let v5: bool = v4 == 0u64;
        v5
    };
    if v6 {
        ticket_created_35(v0.clone())
    } else {
        let v8: u64 = ticket_ticks_30(v2);
        let v9: u64 = own_pid_31();
        let v10: u64 = v8.wrapping_sub(1u64);
        let v11: Rc<str> = format_real_8(v10);
        let mut v12: Rc<str> = Rc::<str>::from(((b'0') as char).to_string());
        let v13: i32 = (v11.clone().len() as i32);
        let v14: i32 = 20i32.wrapping_sub(v13);
        let v15: i32 = 0i32;
        let mut v16: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method32(v14, v12, v15);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = v16(v17.clone());
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let v22: Rc<str> = format_real_8(v9);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".t"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        let v26: bool = ticket_written_36(v0, v25.clone());
        if v26 {
            v25
        } else {
            v17
        }
    }
}
fn slot_gate_0(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = heavy_queue_28();
    let v2: bool = &*v1 == "";
    let v5: Rc<str> = if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        handed_ticket_29(v0.clone(), v1.clone())
    };
    let v7: bool = if v2 {
        true
    } else {
        let mut v6: bool = &*v5 != "";
        v6
    };
    let v14: Rc<str> = if v7 {
        v5
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = job_file_3(v0.clone(), v8);
        let v10: bool = file_exists_11(v9);
        if v10 {
            front_ticket_34(v1.clone())
        } else {
            ticket_created_35(v1.clone())
        }
    };
    let v15: bool = &*v14 == "";
    if v15 {
        0i32
    } else {
        let mut v16: u64 = now_ms_2();
        slot_loop_1(v0, v1, v14, v16)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        slot_gate_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn held_ticket_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_3(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_15(v3);
    let v5: bool = &*v4 != "";
    let v8: bool = if v5 {
        let mut v6: Rc<str> = joined_4(v1.clone(), v4.clone());
        file_exists_11(v6)
    } else {
        false
    };
    if v8 {
        v4
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = job_file_3(v0.clone(), v9);
        let v11: bool = file_exists_11(v10);
        let v14: Rc<str> = if v11 {
            front_ticket_34(v1.clone())
        } else {
            ticket_created_35(v1.clone())
        };
        let v15: Rc<str> = job_file_3(v0.clone(), v2);
        let mut v16: Rc<str> = text_written_10(v15, v14.clone());
        let v17: Rc<str> = joined_4(v1, v14.clone());
        let v18: Rc<str> = text_written_10(v17, v0);
        v14
    }
}
fn abandoned_swept_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v3: Rc<str> = joined_4(v0.clone(), v1.clone());
    let mut v4: Rc<str> = text_or_empty_15(v3.clone());
    let v5: Rc<str> = first_row_20(v4.clone());
    let mut v6: Rc<str> = other_rows_23(v4.clone());
    let v7: u64 = decimal_u64_16(v6.clone());
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
        let v15: u64 = own_pid_31();
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
fn slot_held_step_37(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = heavy_queue_28();
    let v2: bool = &*v1 == "";
    let v7: bool = if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = job_file_3(v0.clone(), v3);
        let v5: bool = file_exists_11(v4);
        let mut v6: bool = v5 == false;
        v6
    };
    if v7 {
        0i32
    } else {
        let v8: u64 = now_ms_2();
        let v9: Rc<str> = held_ticket_38(v0.clone(), v1.clone());
        let v10: Rc<str> = tickets_sorted_18(v1.clone());
        let v11: Rc<str> = first_live_ticket_19(v10);
        abandoned_swept_39(v1.clone(), v11, v8);
        let mut v12: Rc<str> = tickets_sorted_18(v1.clone());
        let v13: Rc<str> = first_live_ticket_19(v12);
        let v14: bool = &*v9 == "";
        if v14 {
            0i32
        } else {
            let v15: bool = slot_deadline_passed_14(v0.clone(), v8);
            if v15 {
                let mut v16: Rc<str> = joined_4(v1.clone(), v9.clone());
                let v17: bool = file_removed_12(v16);
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = job_file_3(v0.clone(), v18);
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no heavy slot within --slot-within; the job never started"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let v23: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = concat5_13(v21, v24.clone(), v20, v25, v26);
                let v28: Rc<str> = concat5_13(v22, v24.clone(), v23, v24, v27);
                let v29: Rc<str> = text_written_10(v19, v28);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = job_file_3(v0.clone(), v30);
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled ticket="); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v9));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" state=expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                let v36: Rc<str> = text_written_10(v31, v35);
                2i32
            } else {
                let v37: bool = &*v13 == &*v9;
                let v40: bool = if v37 {
                    let v38: u64 = free_mb_24();
                    slot_free_25(v38)
                } else {
                    false
                };
                if v40 {
                    let v41: Rc<str> = joined_4(v1, v9.clone());
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v42));
                    let v44: Rc<str> = decimal_text_7(v8);
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                    let v46: Rc<str> = text_written_10(v41, v45);
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = job_file_3(v0.clone(), v47);
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handed ticket="); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v9));
                    let v51: Rc<str> = text_written_10(v48, v50);
                    0i32
                } else {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot"); } LIT.with(|lit| lit.clone()) };
                    let v53: Rc<str> = job_file_3(v0, v52);
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held ticket="); } LIT.with(|lit| lit.clone()) };
                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v9));
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" first="); } LIT.with(|lit| lit.clone()) };
                    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v55, v56)), v13));
                    let v59: Rc<str> = text_written_10(v53, v58);
                    1i32
                }
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        slot_held_step_37(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn slot_ticket_dropped_40(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_ticket"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_3(v0, v1);
    let mut v3: Rc<str> = text_or_empty_15(v2);
    let mut v4: bool = &*v3 == "";
    if v4 {
        ()
    } else {
        let v5: Rc<str> = heavy_queue_28();
        let mut v6: Rc<str> = joined_4(v5, v3);
        let v7: bool = file_removed_12(v6);
        ()
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> ()> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> ()> = Rc::new(move |mut v0: Rc<str>| -> () {
        slot_ticket_dropped_40(v0.clone())
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
