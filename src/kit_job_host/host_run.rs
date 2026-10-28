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
struct Mut0 { l0: Rc<str> }
struct Mut1 { l0: i32 }
struct Mut2 { l0: i32 }
#[derive(Clone)]
enum US0 {
    US0_KeepWaiting,
    US0_StartSupervisor,
    US0_SettleStopped(Rc<str>),
    US0_SettleSkipped(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_JobFinished(i32),
    US1_JobTimedOut,
    US1_JobStopped,
    US1_JobSpawnFailed(Rc<str>),
    US1_JobSkipped(Rc<str>),
    US1_JobLost,
    US1_JobMemoryCapped,
    US1_JobExpiredInQueue,
}
#[derive(Clone)]
enum US2 {
    US2_HomePresent,
    US2_HomeGone,
}
#[derive(Clone)]
enum US3 {
    US3_KeepServing,
    US3_HostIdleExit,
    US3_HostDraining,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_HostOwns,
    US5_OtherHostServes(u64),
}
#[derive(Clone)]
enum US6 {
    US6_HostMissing,
    US6_HostRunning(u64),
}
#[derive(Clone)]
enum US7 {
    US7_SupervisorUp(u64),
    US7_SupervisorHeld(Rc<str>),
    US7_SupervisorLate,
}
#[derive(Clone)]
enum US8 {
    US8_Some(US7),
    US8_None,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn host_home_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "";
    if v1 {
        let v2: Rc<str> = jobs_root_5();
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_3(v2.clone(), v3.clone())
    } else {
        let v5: Rc<str> = jobs_root_5();
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = joined_3(v5.clone(), v6.clone());
        joined_3(v7.clone(), v0.clone())
    }
}
fn requests_directory_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = host_home_directory_4(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1.clone(), v2.clone())
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
fn method19(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    format_real_18(v0)
}
fn text_written_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn held_beat_13(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_14(v0.clone(), v1.clone());
    let mut v3: u64 = now_ms_16();
    let mut v4: Rc<str> = decimal_text_17(v3);
    let v5: Rc<str> = text_written_20(v2.clone(), v4.clone());
    ()
}
fn held_directory_21() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0.clone(), v1.clone())
}
fn held_file_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = held_directory_21();
    joined_3(v1, v0)
}
fn held_recorded_12(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = held_directory_21();
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v2).err().map(|error| format!("create {}: {error}", &*v2)).unwrap_or_default());
    let mut v4: Rc<str> = held_file_22(v0.clone());
    let v5: Rc<str> = text_written_20(v4.clone(), v1.clone());
    held_beat_13(v0.clone())
}
fn queue_decide_23(mut v0: Rc<str>) -> i32 {
    let v1: i32 = kit_job_queue::kit_job_queue_decide(&*v0);
    v1
}
fn supervisor_child_started_24(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: bool = { let mut command = std::process::Command::new(&*v1); command.arg("job").arg("supervise").arg(&*v2).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()); #[cfg(windows)] { use std::os::windows::process::CommandExt; const DETACHED_PROCESS: u32 = 0x0000_0008; command.creation_flags(DETACHED_PROCESS); } match command.spawn() { Ok(child) => { (v0).borrow_mut().push(child); true } Err(_) => false } };
    v3
}
fn request_served_11(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    let mut v3: i32 = queue_decide_23(v2.clone());
    let mut v4: bool = v3 == 0i32;
    if v4 {
        let v5: bool = supervisor_child_started_24(v0, v1.clone(), v2.clone());
        ()
    } else {
        let mut v6: bool = v3 == 1i32;
        if v6 {
            held_recorded_12(v2, v1)
        }
    }
}
fn owner_present_25(mut v0: u64, mut v1: u64) -> bool {
    let v2: bool = kit_os_launch::kit_owner_present(v0, v1);
    v2
}
fn directory_exists_27(mut v0: Rc<str>) -> bool {
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
fn method28(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method29(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
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
fn method30() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut2>>) -> bool {
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
fn held_names_26() -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v0: Rc<str> = held_directory_21();
    let v1: bool = directory_exists_27(v0.clone());
    if v1 {
        let v2: Rc<str> = held_directory_21();
        let mut v4: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Vec<std::string::String> = v4.unwrap();
        let v7: Vec<std::string::String> = method28(v6.clone());
        let v8: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v7));
        let v9: i32 = (v8.clone().borrow().len() as i32);
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v9 as usize]));
        let mut v11: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
        while method29(v9, v11.clone()) {
            let v13: i32 = v11.borrow().l0.clone();
            let v14: std::string::String = v8.clone().borrow()[v13 as usize].clone();
            let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            v10.clone().borrow_mut()[v13 as usize] = v16.clone();
            let v17: i32 = v13.wrapping_add(1i32);
            v11.borrow_mut().l0 = v17;
            ()
        };
        let v18: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
        let mut v19: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method30();
        let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().clone()));
        let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().iter().map(|x| v19(x.clone())).collect::<Vec<_>>()));
        let v22: i32 = (v18.clone().borrow().len() as i32);
        let mut v23: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
        while method29(v22, v23.clone()) {
            let v25: i32 = v23.borrow().l0.clone();
            let v26: Rc<str> = v20.clone().borrow()[v25 as usize].clone();
            let v27: Rc<str> = v21.clone().borrow()[v25 as usize].clone();
            let mut v28: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v25 }));
            while method31(v21.clone(), v27.clone(), v28.clone()) {
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
fn held_owned_by_33(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_14(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_7(v3.clone());
    let v5: bool = &*v4 == &*v1;
    v5
}
fn settled_word_35(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US1::US1_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US1::US1_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US1::US1_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn settled_in_queue_34(mut v0: Rc<str>, mut v1: US0) -> () {
    match &v1 {
        US0::US0_SettleSkipped(v16) => {
            let v16: Rc<str> = v16.clone();
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = job_file_14(v0.clone(), v17.clone());
            let mut v19: u64 = now_ms_16();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v21: US1 = US1::US1_JobSkipped(v20.clone());
            let v22: Rc<str> = settled_word_35(v21.clone());
            let mut v23: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v24: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v16, &*v26, &*v20].concat());
            let mut v28: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v25, &*v24, &*v25, &*v27].concat());
            let v29: Rc<str> = text_written_20(v18.clone(), v28.clone());
            ()
        }
        US0::US0_SettleStopped(v2) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = job_file_14(v0.clone(), v3.clone());
            let v5: u64 = now_ms_16();
            let mut v6: US1 = US1::US1_JobStopped;
            let v7: Rc<str> = settled_word_35(v6.clone());
            let v8: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v10, &*v2, &*v11, &*v12].concat());
            let v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v10, &*v9, &*v10, &*v13].concat());
            let v15: Rc<str> = text_written_20(v4.clone(), v14.clone());
            ()
        }
        _ => {
            ()
        }
    }
}
fn held_released_36(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = held_file_22(v0);
    { let _ = std::fs::remove_file(&*v1); };
    ()
}
fn held_stopped_from_32(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32) -> () {
    loop {
        let mut v3: i32 = (v1.clone().borrow().len() as i32);
        let mut v4: bool = v2 >= v3;
        if v4 {
            return ();
        } else {
            let v5: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let mut v6: bool = held_owned_by_33(v5.clone(), v0.clone());
            if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("its owner session ended before the job started"); } LIT.with(|lit| lit.clone()) };
                let v8: US0 = US0::US0_SettleStopped(v7.clone());
                settled_in_queue_34(v5.clone(), v8.clone());
                held_released_36(v5.clone())
            };
            let v9: i32 = v2.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
            continue;
        }
        return ();
    }
}
fn live_children_37(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>) -> u64 {
    let v1: u64 = { let mut kids = (v0).borrow_mut(); kids.retain_mut(|child| matches!(child.try_wait(), Ok(None))); kids.len() as u64 };
    v1
}
fn held_from_39(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: u64) -> u64 {
    loop {
        let v5: i32 = (v2.clone().borrow().len() as i32);
        let mut v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let v7: Rc<str> = v2.clone().borrow()[v3 as usize].clone();
            let v8: bool = held_owned_by_33(v7.clone(), v1.clone());
            let v9: bool = v8 == false;
            if v9 {
                let v10: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v10, v4);
                continue;
            } else {
                let v12: Rc<str> = v2.clone().borrow()[v3 as usize].clone();
                let v13: i32 = queue_decide_23(v12.clone());
                let v14: bool = v13 == 1i32;
                if v14 {
                    held_beat_13(v12.clone());
                    let v15: i32 = v3.wrapping_add(1i32);
                    let v16: u64 = v4.wrapping_add(1u64);
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v15, v16);
                    continue;
                } else {
                    let v18: bool = v13 == 0i32;
                    if v18 {
                        let mut v19: Rc<str> = held_file_22(v12.clone());
                        let v20: Rc<str> = text_or_empty_7(v19.clone());
                        held_released_36(v12.clone());
                        let v21: bool = supervisor_child_started_24(v0.clone(), v20.clone(), v12.clone());
                        let v22: i32 = v3.wrapping_add(1i32);
                        (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v22, v4);
                        continue;
                    } else {
                        held_released_36(v12.clone());
                        let v24: i32 = v3.wrapping_add(1i32);
                        (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v24, v4);
                        continue;
                    }
                }
            }
        }
    }
}
fn held_served_38(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>) -> u64 {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = held_names_26();
    let mut v3: i32 = 0i32;
    let mut v4: u64 = 0u64;
    held_from_39(v0, v1, v2, v3, v4)
}
fn directory_exists_41(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn home_of_40(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = requests_directory_2(v0);
    let v2: bool = directory_exists_41(v1);
    if v2 {
        US2::US2_HomePresent
    } else {
        US2::US2_HomeGone
    }
}
fn host_step_of_42(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: US2, mut v4: bool) -> US3 {
    let v5: bool = v4 == false;
    if v5 {
        let mut v6: bool = v0 > 0u64;
        if v6 {
            US3::US3_HostDraining
        } else {
            US3::US3_HostIdleExit
        }
    } else {
        let v10: bool = v0 > 0u64;
        if v10 {
            US3::US3_KeepServing
        } else {
            let v12: bool = match &v3 {
                US2::US2_HomeGone => {
                    true
                }
                _ => {
                    false
                }
            };
            if v12 {
                US3::US3_HostIdleExit
            } else {
                let v14: u64 = v2.wrapping_sub(v1);
                let v15: bool = v14 >= 600000u64;
                if v15 {
                    US3::US3_HostIdleExit
                } else {
                    US3::US3_KeepServing
                }
            }
        }
    }
}
fn owner_ended_file_43(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = host_home_directory_4(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("owner-ended"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1.clone(), v2.clone())
}
fn host_pid_file_44(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = host_home_directory_4(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1.clone(), v2.clone())
}
fn method46(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_45(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method46(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US4> = closure2();
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
fn own_pid_47() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn host_served_1(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> i32 {
    loop {
        let v5: Rc<str> = requests_directory_2(v1.clone());
        let mut v6: Rc<str> = next_request_6(v5.clone());
        let v7: bool = &*v6 != "";
        if v7 {
            let v8: Rc<str> = joined_3(v5.clone(), v6.clone());
            let v9: Rc<str> = text_or_empty_7(v8.clone());
            let v10: bool = request_removed_8(v8.clone());
            let mut v11: Rc<str> = request_id_9(v6.clone());
            request_served_11(v0.clone(), v9.clone(), v11.clone());
            let v12: u64 = now_ms_16();
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v3, v12);
            continue;
        } else {
            let v14: bool = owner_present_25(v2, v3);
            if v14 {
                ()
            } else {
                let v15: Rc<RefCell<Vec<Rc<str>>>> = held_names_26();
                let v16: i32 = 0i32;
                held_stopped_from_32(v1.clone(), v15.clone(), v16)
            };
            let v17: u64 = live_children_37(v0.clone());
            let v18: u64 = held_served_38(v0.clone(), v1.clone());
            let mut v19: u64 = v17.wrapping_add(v18);
            let v20: u64 = now_ms_16();
            let v21: US2 = home_of_40(v1.clone());
            let v22: US3 = host_step_of_42(v19, v4, v20, v21.clone(), v14);
            match &v22 {
                US3::US3_HostDraining => {
                    let v31: Rc<str> = owner_ended_file_43(v1.clone());
                    let v32: Rc<str> = decimal_text_17(v20);
                    let v33: Rc<str> = text_written_20(v31.clone(), v32.clone());
                    let v34: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v34));
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v3, v4);
                    continue;
                }
                US3::US3_HostIdleExit => {
                    let mut v23: Rc<str> = host_pid_file_44(v1.clone());
                    let v24: Rc<str> = text_or_empty_7(v23.clone());
                    let v25: u64 = decimal_u64_45(v24.clone());
                    let v26: u64 = own_pid_47();
                    let v27: bool = v25 == v26;
                    let v30: bool = if v27 {
                        let mut v28: Rc<str> = host_pid_file_44(v1.clone());
                        request_removed_8(v28.clone())
                    } else {
                        false
                    };
                    return 0i32;
                }
                US3::US3_KeepServing => {
                    let v36: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v36));
                    let v37: bool = v19 > 0u64;
                    let v38: u64 = if v37 {
                        v20
                    } else {
                        v4
                    };
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v3, v38);
                    continue;
                }
            }
        }
    }
}
fn owner_home_of_48(mut v0: u64, mut v1: u64) -> Rc<str> {
    let v2: bool = v0 == 0u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = decimal_text_17(v0);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        let v7: Rc<str> = decimal_text_17(v1);
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
        v8.clone()
    }
}
fn process_alive_49(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn host_claim_of_50(mut v0: u64, mut v1: u64, mut v2: bool) -> US5 {
    let mut v3: bool = v0 != 0u64;
    let v5: bool = if v3 {
        let mut v4: bool = v0 != v1;
        v4
    } else {
        false
    };
    let mut v6: bool = v5 && v2;
    if v6 {
        US5::US5_OtherHostServes(v0)
    } else {
        US5::US5_HostOwns
    }
}
fn no_children_51() -> std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> {
    let mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    v0
}
fn host_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: u64 = decimal_u64_45(v1.clone());
    let mut v3: i32 = 4i32;
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
    let v5: u64 = decimal_u64_45(v4.clone());
    let mut v6: Rc<str> = owner_home_of_48(v2, v5);
    let v7: Rc<str> = host_pid_file_44(v6.clone());
    let v8: Rc<str> = text_or_empty_7(v7.clone());
    let v9: u64 = decimal_u64_45(v8.clone());
    let v10: u64 = own_pid_47();
    let mut v11: bool = process_alive_49(v9);
    let v12: US5 = host_claim_of_50(v9, v10, v11);
    match &v12 {
        US5::US5_HostOwns => {
            let v14: Rc<str> = requests_directory_2(v6.clone());
            let v15: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v14).err().map(|error| format!("create {}: {error}", &*v14)).unwrap_or_default());
            let v16: Rc<str> = host_pid_file_44(v6.clone());
            let v17: u64 = own_pid_47();
            let v18: Rc<str> = decimal_text_17(v17);
            let mut v19: Rc<str> = text_written_20(v16.clone(), v18.clone());
            let v20: std::rc::Rc<std::cell::RefCell<Vec<std::process::Child>>> = no_children_51();
            let v21: u64 = now_ms_16();
            host_served_1(v20.clone(), v6.clone(), v2, v5, v21)
        }
        US5::US5_OtherHostServes(v13) => {
            let v13: u64 = *v13;
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        host_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn caller_owner_53() -> (Rc<str>, u64, u64) {
    let (mut v0, mut v1, mut v2): (Rc<str>, u64, u64) = kit_os_launch::kit_caller_owner();
    (v0, v1, v2)
}
fn host_state_of_54(mut v0: u64, mut v1: bool) -> US6 {
    let v2: bool = v0 != 0u64;
    let mut v3: bool = v2 && v1;
    if v3 {
        US6::US6_HostRunning(v0)
    } else {
        US6::US6_HostMissing
    }
}
fn supervisor_executable_55() -> Rc<str> {
    let mut v0: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("kit").join("supervisor"); std::fs::create_dir_all(&folder)?; let stem = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default(); let target = folder.join(format!("{}-{}-{}{}", stem, metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; let day = std::time::Duration::from_secs(86400); for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let same_stem = path.file_name().map(|name| name.to_string_lossy().starts_with(&format!("{stem}-"))).unwrap_or(false); let stale = entry.metadata().and_then(|metadata| metadata.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age > day).unwrap_or(false); if same_stem && stale && path != target { let _ = std::fs::remove_file(&path); } } } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v0.clone()
}
fn detached_started_57(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let v2: u64 = kit_os_launch::kit_shell_parented_start(&*v0, &(v1).split(char::from(31u8)).filter(|arg| !arg.is_empty()).collect::<Vec<_>>()).0;
    v2
}
fn args_mark_58() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    v0
}
fn owner_parented_started_59(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> u64 {
    let mut v3: u64 = kit_os_launch::kit_parented_start(&*v0, &(v1).split(char::from(31u8)).filter(|arg| !arg.is_empty()).collect::<Vec<_>>(), v2 as u32).0;
    v3
}
fn host_started_56(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    let mut v3: bool = v1 == 0u64;
    if v3 {
        let mut v4: Rc<str> = args_mark_58();
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
        detached_started_57(v0.clone(), v8.clone())
    } else {
        let v10: Rc<str> = args_mark_58();
        let mut v11: Rc<str> = decimal_text_17(v1);
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
        let v13: Rc<str> = args_mark_58();
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        let v15: Rc<str> = decimal_text_17(v2);
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
        let v17: Rc<str> = args_mark_58();
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v16));
        let mut v23: u64 = owner_parented_started_59(v0.clone(), v22.clone(), v1);
        let v24: bool = v23 == 0u64;
        if v24 {
            detached_started_57(v0.clone(), v22.clone())
        } else {
            v23
        }
    }
}
fn held_revived_52() -> Rc<str> {
    let mut v0: Rc<RefCell<Vec<Rc<str>>>> = held_names_26();
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle"); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let (mut v4, mut v5, mut v6): (Rc<str>, u64, u64) = caller_owner_53();
        let v7: Rc<str> = host_pid_file_44(v4.clone());
        let v8: Rc<str> = text_or_empty_7(v7.clone());
        let v9: u64 = decimal_u64_45(v8.clone());
        let v10: bool = process_alive_49(v9);
        let mut v11: US6 = host_state_of_54(v9, v10);
        match &v11 {
            US6::US6_HostMissing => {
                let v14: Rc<str> = jobs_root_5();
                let v15: Rc<str> = supervisor_executable_55();
                let v16: u64 = host_started_56(v15.clone(), v5, v6);
                let v17: bool = v16 == 0u64;
                if v17 {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: held jobs wait and no job host could be started"); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                } else {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("revived"); } LIT.with(|lit| lit.clone()) };
                    v19.clone()
                }
            }
            US6::US6_HostRunning(v12) => {
                let v12: u64 = *v12;
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
                v13.clone()
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        held_revived_52()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn request_name_61(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    v2.clone()
}
fn supervisor_wait_of_63(mut v0: u64, mut v1: u64, mut v2: u64) -> US8 {
    let mut v3: bool = v0 != 0u64;
    if v3 {
        let mut v4: US7 = US7::US7_SupervisorUp(v0);
        US8::US8_Some(v4)
    } else {
        let mut v6: bool = v1 >= v2;
        if v6 {
            let v7: US7 = US7::US7_SupervisorLate;
            US8::US8_Some(v7)
        } else {
            US8::US8_None
        }
    }
}
fn file_exists_64(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn supervisor_awaited_62(mut v0: Rc<str>, mut v1: u64) -> US7 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_14(v0.clone(), v2.clone());
        let mut v4: Rc<str> = text_or_empty_7(v3.clone());
        let v5: u64 = decimal_u64_45(v4.clone());
        let mut v6: u64 = now_ms_16();
        let v7: US8 = supervisor_wait_of_63(v5, v6, v1);
        match &v7 {
            US8::US8_None => {
                let v9: Rc<str> = held_file_22(v0.clone());
                let v10: bool = file_exists_64(v9.clone());
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    return US7::US7_SupervisorHeld(v11.clone());
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = job_file_14(v0.clone(), v13.clone());
                    let v15: bool = file_exists_64(v14.clone());
                    if v15 {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled"); } LIT.with(|lit| lit.clone()) };
                        return US7::US7_SupervisorHeld(v16.clone());
                    } else {
                        let v18: u64 = 50u64;
                        std::thread::sleep(std::time::Duration::from_millis(v18));
                        (v0, v1) = (v0.clone(), v1);
                        continue;
                    }
                }
            }
            US8::US8_Some(v8) => {
                let v8: US7 = v8.clone();
                return v8.clone();
            }
        }
    }
}
fn supervisor_requested_60(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    let v2: Rc<str> = supervisor_executable_55();
    let (mut v3, mut v4, mut v5): (Rc<str>, u64, u64) = caller_owner_53();
    let mut v6: Rc<str> = requests_directory_2(v3.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v6).err().map(|error| format!("create {}: {error}", &*v6)).unwrap_or_default());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_14(v0.clone(), v8.clone());
    let v10: Rc<str> = text_written_20(v9.clone(), v3.clone());
    let mut v11: Rc<str> = request_name_61(v0.clone());
    let v12: Rc<str> = joined_3(v6.clone(), v11.clone());
    let v13: Rc<str> = text_written_20(v12.clone(), v2.clone());
    let v14: Rc<str> = host_pid_file_44(v3.clone());
    let v15: Rc<str> = text_or_empty_7(v14.clone());
    let v16: u64 = decimal_u64_45(v15.clone());
    let v17: bool = process_alive_49(v16);
    let v18: US6 = host_state_of_54(v16, v17);
    let v22: u64 = match &v18 {
        US6::US6_HostMissing => {
            host_started_56(v2.clone(), v4, v5)
        }
        US6::US6_HostRunning(v19) => {
            let mut v19: u64 = *v19;
            0u64
        }
    };
    let mut v23: u64 = now_ms_16();
    let v24: u64 = v23.wrapping_add(5000u64);
    let v25: US7 = supervisor_awaited_62(v0.clone(), v24);
    match &v25 {
        US7::US7_SupervisorHeld(v28) => {
            let mut v28: Rc<str> = v28.clone();
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" host_home="); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let v31: bool = &*v3 == "";
            let v33: Rc<str> = if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shared"); } LIT.with(|lit| lit.clone()) };
                v32.clone()
            } else {
                v3.clone()
            };
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v33));
            v34.clone()
        }
        US7::US7_SupervisorLate => {
            let v35: bool = request_removed_8(v12.clone());
            if v35 {
                let v36: Rc<str> = args_mark_58();
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v36));
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
                let v41: Rc<str> = args_mark_58();
                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v40, v41)), v0));
                let v44: u64 = detached_started_57(v2.clone(), v43.clone());
                let v45: bool = v44 == 0u64;
                if v45 {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: neither the job host nor a detached start launched the supervisor"); } LIT.with(|lit| lit.clone()) };
                    v46.clone()
                } else {
                    decimal_text_17(v44)
                }
            } else {
                let v49: u64 = now_ms_16();
                let v50: u64 = v49.wrapping_add(60000u64);
                let v51: US7 = supervisor_awaited_62(v0.clone(), v50);
                match &v51 {
                    US7::US7_SupervisorHeld(v54) => {
                        let v54: Rc<str> = v54.clone();
                        v54.clone()
                    }
                    US7::US7_SupervisorLate => {
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: the job host took the request but the supervisor never recorded its pid"); } LIT.with(|lit| lit.clone()) };
                        v55.clone()
                    }
                    US7::US7_SupervisorUp(v52) => {
                        let v52: u64 = *v52;
                        decimal_text_17(v52)
                    }
                }
            }
        }
        US7::US7_SupervisorUp(v26) => {
            let v26: u64 = *v26;
            decimal_text_17(v26)
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        supervisor_requested_60(v0.clone())
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
