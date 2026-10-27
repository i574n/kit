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
struct Mut0 { l0: i32 }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum US0 {
    US0_HomePresent,
    US0_HomeGone,
}
#[derive(Clone)]
enum US1 {
    US1_KeepServing,
    US1_HostIdleExit,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_HostOwns,
    US3_OtherHostServes(u64),
}
#[derive(Clone)]
enum US4 {
    US4_HostMissing,
    US4_HostRunning(u64),
}
#[derive(Clone)]
enum US5 {
    US5_SupervisorUp(u64),
    US5_SupervisorHeld(Rc<str>),
    US5_SupervisorLate,
}
#[derive(Clone)]
enum US6 {
    US6_Some(US5),
    US6_None,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn host_home_directory_4() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn requests_directory_2() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn next_request_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".start")).min().unwrap_or_default());
    v1.clone()
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn request_removed_8(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(6i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn request_id_9(mut v0: Rc<str>) -> Rc<str> {
    loop_10(v0)
}
fn job_directory_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_15(v0);
    joined_3(v2, v1)
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn held_beat_13(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: u64 = now_ms_16();
    let mut v4: Rc<str> = decimal_text_17(v3);
    let v5: Rc<str> = text_written_18(v2.clone(), v4.clone());
    ()
}
fn held_directory_19() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn held_file_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = held_directory_19();
    joined_3(v1, v0)
}
fn held_recorded_12(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = held_directory_19();
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v2).err().map(|error| format!("create {}: {error}", &*v2)).unwrap_or_default());
    let mut v4: Rc<str> = held_file_20(v0.clone());
    let v5: Rc<str> = text_written_18(v4.clone(), v1.clone());
    held_beat_13(v0.clone())
}
fn queue_decide_21(mut v0: Rc<str>) -> i32 {
    let v1: i32 = kit_job_queue::kit_job_queue_decide(&*v0);
    v1
}
fn supervisor_child_started_22(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = { let mut command = std::process::Command::new(&*v1); command.arg("job").arg("supervise").arg(&*v2).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; const DETACHED_PROCESS: u32 = 0x0000_0008; command.creation_flags(DETACHED_PROCESS); } match command.spawn() { Ok(child) => { (v0).borrow_mut().push(child); true } Err(_) => false } };
    v3
}
fn request_served_11(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    let mut v3: i32 = queue_decide_21(v2.clone());
    let mut v4: bool = v3 == 0i32;
    if v4 {
        let v5: bool = supervisor_child_started_22(v0, v1.clone(), v2.clone());
        ()
    } else {
        let mut v6: bool = v3 == 1i32;
        if v6 {
            held_recorded_12(v2, v1)
        }
    }
}
fn live_children_23(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let v1: u64 = { let mut kids = (v0).borrow_mut(); kids.retain_mut(|child| matches!(child.try_wait(), Ok(None))); kids.len() as u64 };
    v1
}
fn held_released_26(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = held_file_20(v0);
    { let _ = std::fs::remove_file(&*v1); };
    ()
}
fn held_from_25(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: u64) -> u64 {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let v7: i32 = queue_decide_21(v6.clone());
            let v8: bool = v7 == 1i32;
            if v8 {
                held_beat_13(v6.clone());
                let v9: i32 = v2.wrapping_add(1i32);
                let v10: u64 = v3.wrapping_add(1u64);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v9, v10);
                continue;
            } else {
                let v12: bool = v7 == 0i32;
                if v12 {
                    let v13: Rc<str> = held_file_20(v6.clone());
                    let v14: Rc<str> = text_or_empty_7(v13.clone());
                    held_released_26(v6.clone());
                    let v15: bool = supervisor_child_started_22(v0.clone(), v14.clone(), v6.clone());
                    let v16: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v16, v3);
                    continue;
                } else {
                    held_released_26(v6.clone());
                    let v18: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v18, v3);
                    continue;
                }
            }
        }
    }
}
fn directory_exists_28(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method29(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method30(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method31() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method32(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut1>>) -> bool {
    let mut v3: i32 = v2.borrow().l0.clone();
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let v5: i32 = v2.borrow().l0.clone();
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let v8: bool = &*v7 < &*v1;
        let mut v11: i32 = if v8 {
            -1i32
        } else {
            let v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn held_names_27() -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v0: Rc<str> = held_directory_19();
    let v1: bool = directory_exists_28(v0.clone());
    if v1 {
        let v2: Rc<str> = held_directory_19();
        let mut v4: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Vec<std::string::String> = v4.unwrap();
        let v7: Vec<std::string::String> = method29(v6.clone());
        let v8: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v7));
        let v9: i32 = (v8.clone().borrow().len() as i32);
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v9 as usize]));
        let mut v11: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
        while method30(v9, v11.clone()) {
            let v13: i32 = v11.borrow().l0.clone();
            let v14: std::string::String = v8.clone().borrow()[v13 as usize].clone();
            let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            v10.clone().borrow_mut()[v13 as usize] = v16.clone();
            let v17: i32 = v13.wrapping_add(1i32);
            v11.borrow_mut().l0 = v17;
            ()
        };
        let v18: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
        let mut v19: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method31();
        let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().clone()));
        let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().iter().map(|x| v19(x.clone())).collect::<Vec<_>>()));
        let v22: i32 = (v18.clone().borrow().len() as i32);
        let mut v23: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
        while method30(v22, v23.clone()) {
            let v25: i32 = v23.borrow().l0.clone();
            let v26: Rc<str> = v20.clone().borrow()[v25 as usize].clone();
            let v27: Rc<str> = v21.clone().borrow()[v25 as usize].clone();
            let mut v28: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v25 }));
            while method32(v21.clone(), v27.clone(), v28.clone()) {
                let v30: i32 = v28.borrow().l0.clone();
                let v31: i32 = v30.wrapping_sub(1i32);
                let v32: Rc<str> = v20.clone().borrow()[v31 as usize].clone();
                v20.clone().borrow_mut()[v30 as usize] = v32.clone();
                let v33: Rc<str> = v21.clone().borrow()[v31 as usize].clone();
                v21.clone().borrow_mut()[v30 as usize] = v33.clone();
                v28.borrow_mut().l0 = v31;
                ()
            };
            let v34: i32 = v28.borrow().l0.clone();
            v20.clone().borrow_mut()[v34 as usize] = v26.clone();
            let v35: i32 = v28.borrow().l0.clone();
            v21.clone().borrow_mut()[v35 as usize] = v27.clone();
            let v36: i32 = v25.wrapping_add(1i32);
            v23.borrow_mut().l0 = v36;
            ()
        };
        let v37: Rc<RefCell<Vec<Rc<str>>>> = v20 ;
        v37.clone()
    } else {
        let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v38.clone()
    }
}
fn held_served_24(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = held_names_27();
    let v2: i32 = 0i32;
    let mut v3: u64 = 0u64;
    held_from_25(v0, v1, v2, v3)
}
fn directory_exists_34(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn home_of_33() -> US0 {
    let mut v0: Rc<str> = requests_directory_2();
    let v1: bool = directory_exists_34(v0);
    if v1 {
        US0::US0_HomePresent
    } else {
        US0::US0_HomeGone
    }
}
fn host_step_of_35(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: US0) -> US1 {
    let mut v4: bool = v0 > 0u64;
    if v4 {
        US1::US1_KeepServing
    } else {
        let mut v6: bool = match &v3 {
            US0::US0_HomeGone => {
                true
            }
            _ => {
                false
            }
        };
        if v6 {
            US1::US1_HostIdleExit
        } else {
            let v8: u64 = v2.wrapping_sub(v1);
            let v9: bool = v8 >= 600000u64;
            if v9 {
                US1::US1_HostIdleExit
            } else {
                US1::US1_KeepServing
            }
        }
    }
}
fn host_pid_file_36() -> Rc<str> {
    let mut v0: Rc<str> = host_home_directory_4();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn method38(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_37(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method38(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure2();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let mut v6: US2 = US2::US2_None;
    let v7: US2 = v6.clone();
    let v8: US2 = v4.unwrap_or(v7);
    match &v8 {
        US2::US2_None => {
            0u64
        }
        US2::US2_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn own_pid_39() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn host_served_1(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: u64) -> i32 {
    loop {
        let v2: Rc<str> = requests_directory_2();
        let mut v3: Rc<str> = next_request_6(v2.clone());
        let mut v4: bool = &*v3 != "";
        if v4 {
            let v5: Rc<str> = joined_3(v2.clone(), v3.clone());
            let mut v6: Rc<str> = text_or_empty_7(v5.clone());
            let v7: bool = request_removed_8(v5.clone());
            let v8: Rc<str> = request_id_9(v3.clone());
            request_served_11(v0.clone(), v6.clone(), v8.clone());
            let v9: u64 = now_ms_16();
            (v0, v1) = (v0.clone(), v9);
            continue;
        } else {
            let mut v11: u64 = live_children_23(v0.clone());
            let v12: u64 = held_served_24(v0.clone());
            let v13: u64 = v11.wrapping_add(v12);
            let v14: u64 = now_ms_16();
            let v15: US0 = home_of_33();
            let v16: US1 = host_step_of_35(v13, v1, v14, v15.clone());
            match &v16 {
                US1::US1_HostIdleExit => {
                    let v17: Rc<str> = host_pid_file_36();
                    let v18: Rc<str> = text_or_empty_7(v17.clone());
                    let mut v19: u64 = decimal_u64_37(v18.clone());
                    let v20: u64 = own_pid_39();
                    let v21: bool = v19 == v20;
                    let v24: bool = if v21 {
                        let v22: Rc<str> = host_pid_file_36();
                        request_removed_8(v22.clone())
                    } else {
                        false
                    };
                    return 0i32;
                }
                US1::US1_KeepServing => {
                    let v25: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v25));
                    let v26: bool = v13 > 0u64;
                    let v27: u64 = if v26 {
                        v14
                    } else {
                        v1
                    };
                    (v0, v1) = (v0.clone(), v27);
                    continue;
                }
            }
        }
    }
}
fn process_alive_40(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn host_claim_of_41(mut v0: u64, mut v1: u64, mut v2: bool) -> US3 {
    let mut v3: bool = v0 != 0u64;
    let v5: bool = if v3 {
        let mut v4: bool = v0 != v1;
        v4
    } else {
        false
    };
    let mut v6: bool = v5 && v2;
    if v6 {
        US3::US3_OtherHostServes(v0)
    } else {
        US3::US3_HostOwns
    }
}
fn no_children_42() -> std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> {
    let mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    v0
}
fn host_run_0() -> i32 {
    let mut v0: Rc<str> = host_pid_file_36();
    let v1: Rc<str> = text_or_empty_7(v0.clone());
    let v2: u64 = decimal_u64_37(v1.clone());
    let mut v3: u64 = own_pid_39();
    let mut v4: bool = process_alive_40(v2);
    let v5: US3 = host_claim_of_41(v2, v3, v4);
    match &v5 {
        US3::US3_HostOwns => {
            let v7: Rc<str> = requests_directory_2();
            let v8: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v7).err().map(|error| format!("create {}: {error}", &*v7)).unwrap_or_default());
            let v9: Rc<str> = host_pid_file_36();
            let v10: u64 = own_pid_39();
            let mut v11: Rc<str> = decimal_text_17(v10);
            let v12: Rc<str> = text_written_18(v9.clone(), v11.clone());
            let v13: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = no_children_42();
            let v14: u64 = now_ms_16();
            host_served_1(v13.clone(), v14)
        }
        US3::US3_OtherHostServes(v6) => {
            let mut v6: u64 = *v6;
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        host_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn host_state_of_44(mut v0: u64, mut v1: bool) -> US4 {
    let v2: bool = v0 != 0u64;
    let mut v3: bool = v2 && v1;
    if v3 {
        US4::US4_HostRunning(v0)
    } else {
        US4::US4_HostMissing
    }
}
fn supervisor_executable_45() -> Rc<str> {
    let mut v0: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("kit").join("supervisor"); std::fs::create_dir_all(&folder)?; let stem = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default(); let target = folder.join(format!("{}-{}-{}{}", stem, metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; let day = std::time::Duration::from_secs(86400); for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let same_stem = path.file_name().map(|name| name.to_string_lossy().starts_with(&format!("{stem}-"))).unwrap_or(false); let stale = entry.metadata().and_then(|metadata| metadata.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age > day).unwrap_or(false); if same_stem && stale && path != target { let _ = std::fs::remove_file(&path); } } } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v0.clone()
}
fn args_mark_46() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    v0
}
fn detached_started_47(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let v2: u64 = kit_os_launch::kit_shell_parented_start(&*v0, &(v1).split(char::from(31u8)).filter(|arg| !arg.is_empty()).collect::<Vec<_>>()).0;
    v2
}
fn held_revived_43() -> Rc<str> {
    let mut v0: Rc<RefCell<Vec<Rc<str>>>> = held_names_27();
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = host_pid_file_36();
        let v5: Rc<str> = text_or_empty_7(v4.clone());
        let mut v6: u64 = decimal_u64_37(v5.clone());
        let v7: bool = process_alive_40(v6);
        let v8: US4 = host_state_of_44(v6, v7);
        match &v8 {
            US4::US4_HostMissing => {
                let mut v11: Rc<str> = jobs_root_5();
                let v12: Rc<str> = supervisor_executable_45();
                let v13: Rc<str> = args_mark_46();
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                let v18: u64 = detached_started_47(v12.clone(), v17.clone());
                let mut v19: bool = v18 == 0u64;
                if v19 {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: held jobs wait and no job host could be started"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("revived"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
            }
            US4::US4_HostRunning(v9) => {
                let v9: u64 = *v9;
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        held_revived_43()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn request_name_49(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    v2.clone()
}
fn supervisor_wait_of_51(mut v0: u64, mut v1: u64, mut v2: u64) -> US6 {
    let mut v3: bool = v0 != 0u64;
    if v3 {
        let mut v4: US5 = US5::US5_SupervisorUp(v0);
        US6::US6_Some(v4)
    } else {
        let mut v6: bool = v1 >= v2;
        if v6 {
            let v7: US5 = US5::US5_SupervisorLate;
            US6::US6_Some(v7)
        } else {
            US6::US6_None
        }
    }
}
fn file_exists_52(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn supervisor_awaited_50(mut v0: Rc<str>, mut v1: u64) -> US5 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_14(v0.clone(), v2.clone());
        let mut v4: Rc<str> = text_or_empty_7(v3.clone());
        let v5: u64 = decimal_u64_37(v4.clone());
        let mut v6: u64 = now_ms_16();
        let v7: US6 = supervisor_wait_of_51(v5, v6, v1);
        match &v7 {
            US6::US6_None => {
                let v9: Rc<str> = held_file_20(v0.clone());
                let v10: bool = file_exists_52(v9.clone());
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    return US5::US5_SupervisorHeld(v11.clone());
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = job_file_14(v0.clone(), v13.clone());
                    let v15: bool = file_exists_52(v14.clone());
                    if v15 {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled"); } LIT.with(|lit| lit.clone()) };
                        return US5::US5_SupervisorHeld(v16.clone());
                    } else {
                        let v18: u64 = 50u64;
                        std::thread::sleep(std::time::Duration::from_millis(v18));
                        (v0, v1) = (v0.clone(), v1);
                        continue;
                    }
                }
            }
            US6::US6_Some(v8) => {
                let v8: US5 = v8.clone();
                return v8.clone();
            }
        }
    }
}
fn supervisor_requested_48(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    let v2: Rc<str> = supervisor_executable_45();
    let mut v3: Rc<str> = requests_directory_2();
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v3).err().map(|error| format!("create {}: {error}", &*v3)).unwrap_or_default());
    let v5: Rc<str> = request_name_49(v0.clone());
    let mut v6: Rc<str> = joined_3(v3.clone(), v5.clone());
    let v7: Rc<str> = text_written_18(v6.clone(), v2.clone());
    let v8: Rc<str> = host_pid_file_36();
    let v9: Rc<str> = text_or_empty_7(v8.clone());
    let v10: u64 = decimal_u64_37(v9.clone());
    let mut v11: bool = process_alive_40(v10);
    let v12: US4 = host_state_of_44(v10, v11);
    let v21: u64 = match &v12 {
        US4::US4_HostMissing => {
            let v14: Rc<str> = args_mark_46();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            detached_started_47(v2.clone(), v18.clone())
        }
        US4::US4_HostRunning(v13) => {
            let v13: u64 = *v13;
            0u64
        }
    };
    let v22: u64 = now_ms_16();
    let mut v23: u64 = v22.wrapping_add(5000u64);
    let v24: US5 = supervisor_awaited_50(v0.clone(), v23);
    match &v24 {
        US5::US5_SupervisorHeld(v27) => {
            let v27: Rc<str> = v27.clone();
            v27.clone()
        }
        US5::US5_SupervisorLate => {
            let mut v28: bool = request_removed_8(v6.clone());
            if v28 {
                let v29: Rc<str> = args_mark_46();
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let v34: Rc<str> = args_mark_46();
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v0));
                let v37: u64 = detached_started_47(v2.clone(), v36.clone());
                let v38: bool = v37 == 0u64;
                if v38 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: neither the job host nor a detached start launched the supervisor"); } LIT.with(|lit| lit.clone()) };
                    v39.clone()
                } else {
                    decimal_text_17(v37)
                }
            } else {
                let v42: u64 = now_ms_16();
                let v43: u64 = v42.wrapping_add(60000u64);
                let v44: US5 = supervisor_awaited_50(v0.clone(), v43);
                match &v44 {
                    US5::US5_SupervisorHeld(v47) => {
                        let v47: Rc<str> = v47.clone();
                        v47.clone()
                    }
                    US5::US5_SupervisorLate => {
                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: the job host took the request but the supervisor never recorded its pid"); } LIT.with(|lit| lit.clone()) };
                        v48.clone()
                    }
                    US5::US5_SupervisorUp(v45) => {
                        let v45: u64 = *v45;
                        decimal_text_17(v45)
                    }
                }
            }
        }
        US5::US5_SupervisorUp(v25) => {
            let v25: u64 = *v25;
            decimal_text_17(v25)
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        supervisor_requested_48(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_host_run() -> i32 {
    closure0()()
}
pub fn kit_job_host_revive() -> Rc<str> {
    closure3()()
}
pub fn kit_job_host_request(v0: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0))
}
