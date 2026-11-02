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
    US0_ExitAfter(u64),
    US0_NeverExit,
}
#[derive(Clone)]
enum US1 {
    US1_Unlimited,
    US1_AtMost(u64),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_StartNow,
    US2_HoldBack,
    US2_DropRequest,
}
struct Mut1 { l0: i32 }
struct Mut2 { l0: i32 }
#[derive(Clone)]
enum US3 {
    US3_KeepWaiting,
    US3_StartSupervisor,
    US3_SettleStopped(Rc<str>),
    US3_SettleSkipped(Rc<str>),
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
enum US5 {
    US5_HomePresent,
    US5_HomeGone,
}
#[derive(Clone)]
enum US6 {
    US6_KeepServing,
    US6_HostIdleExit,
    US6_HostDraining,
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_HostOwns,
    US8_OtherHostServes(u64),
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn host_home_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let mut v2: Rc<str> = jobs_root_5();
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_3(v2, v3)
    } else {
        let v5: Rc<str> = jobs_root_5();
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = joined_3(v5, v6);
        joined_3(v7, v0)
    }
}
fn requests_directory_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = host_home_directory_4(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1, v2)
}
fn next_request_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.ends_with(".start")).min().unwrap_or_default());
    v1.clone()
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn request_removed_8(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
    let mut v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_15(v0);
    joined_3(v2, v1)
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method19(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    format_real_18(v0)
}
fn text_written_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn held_beat_13(mut v0: Rc<str>) -> () {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = job_file_14(v0, v1);
    let mut v3: u64 = now_ms_16();
    let mut v4: Rc<str> = decimal_text_17(v3);
    let v5: Rc<str> = text_written_20(v2, v4);
    ()
}
fn held_directory_21() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_5();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0, v1)
}
fn held_file_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = held_directory_21();
    joined_3(v1, v0)
}
fn held_recorded_12(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = held_directory_21();
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v2).err().map(|error| format!("create {}: {error}", &*v2)).unwrap_or_default());
    let mut v4: Rc<str> = held_file_22(v0.clone());
    let v5: Rc<str> = text_written_20(v4.clone(), v1.clone());
    held_beat_13(v0.clone())
}
fn live_children_24(mut v0: kit_os_launch::KitChildren) -> u64 {
    let mut v1: u64 = kit_os_launch::kit_children_live(&v0);
    v1
}
fn queue_verdict_25(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = kit_job_queue::kit_job_queue_decide(&*v0);
    let mut v2: bool = v1 == 0i32;
    if v2 {
        US2::US2_StartNow
    } else {
        let mut v4: bool = v1 == 1i32;
        if v4 {
            US2::US2_HoldBack
        } else {
            US2::US2_DropRequest
        }
    }
}
fn verdict_for_23(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: US0, mut v5: US1, mut v6: Rc<str>) -> US2 {
    let mut v7: u64 = live_children_24(v0);
    let v8: US2 = queue_verdict_25(v6);
    let v9: bool = match &v8 {
        US2::US2_StartNow => {
            true
        }
        _ => {
            false
        }
    };
    let mut v15: bool = if v9 {
        let v13: bool = match &v5 {
            US1::US1_AtMost(v10) => {
                let mut v10: u64 = *v10;
                let mut v11: bool = v7 < v10;
                v11
            }
            US1::US1_Unlimited => {
                true
            }
        };
        let v14: bool = v13 == false;
        v14
    } else {
        false
    };
    if v15 {
        US2::US2_HoldBack
    } else {
        v8
    }
}
fn child_started_27(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>) -> bool {
    let mut v3: bool = kit_os_launch::kit_child_started(&v0, &*v1, &(v2).borrow());
    v3
}
fn supervisor_child_started_26(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v3, v4, v2]));
    child_started_27(v0, v1, v5)
}
fn request_served_11(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: US0, mut v5: US1, mut v6: Rc<str>, mut v7: Rc<str>) -> () {
    let v8: US2 = verdict_for_23(v0.clone(), v1, v2, v3, v4, v5, v7.clone());
    match &v8 {
        US2::US2_DropRequest => {
            ()
        }
        US2::US2_HoldBack => {
            held_recorded_12(v7.clone(), v6.clone())
        }
        US2::US2_StartNow => {
            let v9: bool = supervisor_child_started_26(v0, v6, v7);
            ()
        }
    }
}
fn owner_present_28(mut v0: u64, mut v1: u64) -> bool {
    let mut v2: bool = kit_os_launch::kit_owner_present(v0, v1);
    v2
}
fn directory_exists_30(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method31(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method32(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method33() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method34(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut2>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.borrow()[v6 as usize].clone();
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
fn held_names_29() -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v0: Rc<str> = held_directory_21();
    let mut v1: bool = directory_exists_30(v0.clone());
    if v1 {
        let mut v2: Rc<str> = held_directory_21();
        let mut v4: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Vec<std::string::String> = v4.unwrap();
        let mut v7: Vec<std::string::String> = method31(v6.clone());
        let v8: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v7));
        let v9: i32 = (v8.clone().borrow().len() as i32);
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v9 as usize]));
        let mut v11: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
        while method32(v9, v11.clone()) {
            let v13: i32 = v11.borrow().l0;
            let v14: std::string::String = v8.borrow()[v13 as usize].clone();
            let mut v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            v10.borrow_mut()[v13 as usize] = v16.clone();
            let v17: i32 = v13.wrapping_add(1i32);
            v11.borrow_mut().l0 = v17;
            ()
        };
        let v18: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
        let mut v19: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method33();
        let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().clone()));
        let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().iter().map(|x| v19(x.clone())).collect::<Vec<_>>()));
        let v22: i32 = (v18.clone().borrow().len() as i32);
        let mut v23: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
        while method32(v22, v23.clone()) {
            let v25: i32 = v23.borrow().l0;
            let v26: Rc<str> = v20.borrow()[v25 as usize].clone();
            let v27: Rc<str> = v21.borrow()[v25 as usize].clone();
            let mut v28: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v25 }));
            while method34(v21.clone(), v27.clone(), v28.clone()) {
                let v30: i32 = v28.borrow().l0;
                let v31: i32 = v30.wrapping_sub(1i32);
                let v32: Rc<str> = v20.borrow()[v31 as usize].clone();
                v20.borrow_mut()[v30 as usize] = v32.clone();
                let v33: Rc<str> = v21.borrow()[v31 as usize].clone();
                v21.borrow_mut()[v30 as usize] = v33.clone();
                v28.borrow_mut().l0 = v31;
                ()
            };
            let v34: i32 = v28.borrow().l0;
            v20.borrow_mut()[v34 as usize] = v26.clone();
            let v35: i32 = v28.borrow().l0;
            v21.borrow_mut()[v35 as usize] = v27.clone();
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
fn held_owned_by_36(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_14(v0, v2);
    let mut v4: Rc<str> = text_or_empty_7(v3);
    let v5: bool = &*v4 == &*v1;
    v5
}
fn settled_word_38(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_JobExpiredInQueue => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US4::US4_JobFinished(v1) => {
            let mut v1: i32 = *v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US4::US4_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US4::US4_JobMemoryCapped => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US4::US4_JobSkipped(v7) => {
            let mut v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US4::US4_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US4::US4_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US4::US4_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn concat5_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn settled_in_queue_37(mut v0: Rc<str>, mut v1: US3) -> () {
    match &v1 {
        US3::US3_SettleSkipped(v16) => {
            let mut v16: Rc<str> = v16.clone();
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = job_file_14(v0.clone(), v17);
            let mut v19: u64 = now_ms_16();
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v21: US4 = US4::US4_JobSkipped(v20.clone());
            let v22: Rc<str> = settled_word_38(v21);
            let mut v23: Rc<str> = std::rc::Rc::<str>::from((125i32).to_string());
            let v24: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = concat5_39(v22, v25.clone(), v16, v26, v20);
            let mut v28: Rc<str> = concat5_39(v23, v25.clone(), v24, v25, v27);
            let v29: Rc<str> = text_written_20(v18, v28);
            ()
        }
        US3::US3_SettleStopped(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = job_file_14(v0, v3);
            let v5: u64 = now_ms_16();
            let mut v6: US4 = US4::US4_JobStopped;
            let mut v7: Rc<str> = settled_word_38(v6);
            let v8: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = concat5_39(v7, v10.clone(), v2, v11, v12);
            let v14: Rc<str> = concat5_39(v8, v10.clone(), v9, v10, v13);
            let mut v15: Rc<str> = text_written_20(v4, v14);
            ()
        }
        _ => {
            ()
        }
    }
}
fn held_released_40(mut v0: Rc<str>) -> () {
    let mut v1: Rc<str> = held_file_22(v0);
    { let _ = std::fs::remove_file(&*v1); };
    ()
}
fn held_stopped_from_35(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32) -> () {
    loop {
        let mut v3: i32 = (v1.clone().borrow().len() as i32);
        let mut v4: bool = v2 >= v3;
        if v4 {
            return ();
        } else {
            let v5: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v6: bool = held_owned_by_36(v5.clone(), v0.clone());
            if v6 {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("its owner session ended before the job started"); } LIT.with(|lit| lit.clone()) };
                let v8: US3 = US3::US3_SettleStopped(v7.clone());
                settled_in_queue_37(v5.clone(), v8.clone());
                held_released_40(v5.clone())
            };
            let v9: i32 = v2.wrapping_add(1i32);
            v2 = v9;
            continue;
        }
        return ();
    }
}
fn held_from_42(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: US0, mut v5: US1, mut v6: Rc<RefCell<Vec<Rc<str>>>>, mut v7: i32, mut v8: u64) -> u64 {
    loop {
        let v9: i32 = (v6.clone().borrow().len() as i32);
        let mut v10: bool = v7 >= v9;
        if v10 {
            return v8;
        } else {
            let mut v11: Rc<str> = v6.borrow()[v7 as usize].clone();
            let v12: bool = held_owned_by_36(v11.clone(), v1.clone());
            let v13: bool = v12 == false;
            if v13 {
                let v14: i32 = v7.wrapping_add(1i32);
                v7 = v14;
                continue;
            } else {
                let mut v16: Rc<str> = v6.borrow()[v7 as usize].clone();
                let v17: US2 = verdict_for_23(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v16.clone());
                match &v17 {
                    US2::US2_DropRequest => {
                        held_released_40(v16.clone());
                        let v26: i32 = v7.wrapping_add(1i32);
                        v7 = v26;
                        continue;
                    }
                    US2::US2_HoldBack => {
                        held_beat_13(v16.clone());
                        let v18: i32 = v7.wrapping_add(1i32);
                        let mut v19: u64 = v8.wrapping_add(1u64);
                        (v7, v8) = (v18, v19);
                        continue;
                    }
                    US2::US2_StartNow => {
                        let mut v21: Rc<str> = held_file_22(v16.clone());
                        let v22: Rc<str> = text_or_empty_7(v21.clone());
                        held_released_40(v16.clone());
                        let mut v23: bool = supervisor_child_started_26(v0.clone(), v22.clone(), v16.clone());
                        let v24: i32 = v7.wrapping_add(1i32);
                        v7 = v24;
                        continue;
                    }
                }
            }
        }
    }
}
fn held_served_41(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: US0, mut v5: US1) -> u64 {
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = held_names_29();
    let mut v7: i32 = 0i32;
    let v8: u64 = 0u64;
    held_from_42(v0, v1, v2, v3, v4, v5, v6, v7, v8)
}
fn directory_exists_44(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn home_of_43(mut v0: Rc<str>) -> US5 {
    let mut v1: Rc<str> = requests_directory_2(v0);
    let mut v2: bool = directory_exists_44(v1);
    if v2 {
        US5::US5_HomePresent
    } else {
        US5::US5_HomeGone
    }
}
fn host_step_of_45(mut v0: US0, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: US5, mut v5: bool) -> US6 {
    let mut v6: bool = v5 == false;
    if v6 {
        let mut v7: bool = v1 > 0u64;
        if v7 {
            US6::US6_HostDraining
        } else {
            US6::US6_HostIdleExit
        }
    } else {
        let mut v11: bool = v1 > 0u64;
        if v11 {
            US6::US6_KeepServing
        } else {
            let v13: bool = match &v4 {
                US5::US5_HomeGone => {
                    true
                }
                _ => {
                    false
                }
            };
            if v13 {
                US6::US6_HostIdleExit
            } else {
                let mut v15: u64 = v3.wrapping_sub(v2);
                let mut v19: bool = match &v0 {
                    US0::US0_ExitAfter(v16) => {
                        let mut v16: u64 = *v16;
                        let v17: bool = v15 >= v16;
                        v17
                    }
                    US0::US0_NeverExit => {
                        false
                    }
                };
                if v19 {
                    US6::US6_HostIdleExit
                } else {
                    US6::US6_KeepServing
                }
            }
        }
    }
}
fn owner_ended_file_46(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = host_home_directory_4(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("owner-ended"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1, v2)
}
fn host_pid_file_47(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = host_home_directory_4(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1, v2)
}
fn method49(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let mut v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_48(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method49(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure2();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let mut v6: US7 = US7::US7_None;
    let mut v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match &v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn own_pid_50() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn host_served_1(mut v0: kit_os_launch::KitChildren, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: US0, mut v5: US1, mut v6: u64) -> i32 {
    loop {
        let mut v7: Rc<str> = requests_directory_2(v1.clone());
        let v8: Rc<str> = next_request_6(v7.clone());
        let v9: bool = &*v8 != "";
        if v9 {
            let mut v10: Rc<str> = joined_3(v7.clone(), v8.clone());
            let mut v11: Rc<str> = text_or_empty_7(v10.clone());
            let v12: bool = request_removed_8(v10.clone());
            let v13: Rc<str> = request_id_9(v8.clone());
            request_served_11(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v11.clone(), v13.clone());
            let v14: u64 = now_ms_16();
            v6 = v14;
            continue;
        } else {
            let mut v16: bool = owner_present_28(v2, v3);
            if v16 {
                ()
            } else {
                let v17: Rc<RefCell<Vec<Rc<str>>>> = held_names_29();
                let v18: i32 = 0i32;
                held_stopped_from_35(v1.clone(), v17.clone(), v18)
            };
            let mut v19: u64 = live_children_24(v0.clone());
            let mut v20: u64 = held_served_41(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone());
            let mut v21: u64 = v19.wrapping_add(v20);
            let v22: u64 = now_ms_16();
            let mut v23: US5 = home_of_43(v1.clone());
            let v24: US6 = host_step_of_45(v4.clone(), v21, v6, v22, v23.clone(), v16);
            match &v24 {
                US6::US6_HostDraining => {
                    let v33: Rc<str> = owner_ended_file_46(v1.clone());
                    let v34: Rc<str> = decimal_text_17(v22);
                    let v35: Rc<str> = text_written_20(v33.clone(), v34.clone());
                    let v36: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v36));
                    continue;
                }
                US6::US6_HostIdleExit => {
                    let v25: Rc<str> = host_pid_file_47(v1.clone());
                    let v26: Rc<str> = text_or_empty_7(v25.clone());
                    let v27: u64 = decimal_u64_48(v26.clone());
                    let mut v28: u64 = own_pid_50();
                    let v29: bool = v27 == v28;
                    let v32: bool = if v29 {
                        let v30: Rc<str> = host_pid_file_47(v1.clone());
                        request_removed_8(v30.clone())
                    } else {
                        false
                    };
                    return 0i32;
                }
                US6::US6_KeepServing => {
                    let v38: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v38));
                    let v39: bool = v21 > 0u64;
                    let v40: u64 = if v39 {
                        v22
                    } else {
                        v6
                    };
                    v6 = v40;
                    continue;
                }
            }
        }
    }
}
fn owner_home_of_52(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v0 == 0u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let mut v4: Rc<str> = decimal_text_17(v0);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        let mut v7: Rc<str> = decimal_text_17(v1);
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
        v8
    }
}
fn get_environment_variable_53(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn method54(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn method55(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn host_terms_of_51(mut v0: u64, mut v1: u64) -> (Rc<str>, u64, u64, US0, US1) {
    let mut v2: Rc<str> = owner_home_of_52(v0, v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_JOB_HOST_IDLE_EXIT_SECONDS"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = get_environment_variable_53(v3.clone());
    let v5: i32 = (v4.clone().len() as i32);
    let mut v6: i32 = 0i32;
    let mut v7: i32 = method54(v4.clone(), v5, v6);
    let v8: i32 = v5.wrapping_sub(1i32);
    let v9: Rc<str> = string_slice(&v4.clone(), v7 as i64, v8 as i64);
    let mut v10: i32 = (v9.clone().len() as i32);
    let mut v11: i32 = method55(v9.clone(), v10);
    let v12: Rc<str> = string_slice(&v9.clone(), 0i32 as i64, v11 as i64);
    let v13: Option<u64> = (v12).trim().parse::<u64>().ok();
    let v14: Option<u64> = method49(v13.clone());
    let mut v15: Rc<dyn Fn((u64)) -> US7> = closure2();
    let mut v16: Option<US7> = v14.map(|x| v15(x));
    let v18: US7 = US7::US7_None;
    let mut v19: US7 = v18.clone();
    let mut v20: US7 = v16.unwrap_or(v19);
    let v29: US0 = match &v20 {
        US7::US7_None => {
            US0::US0_ExitAfter(600000u64)
        }
        US7::US7_Some(v21) => {
            let mut v21: u64 = *v21;
            let v22: bool = v21 == 0u64;
            if v22 {
                US0::US0_NeverExit
            } else {
                let v24: u64 = v21.wrapping_mul(1000u64);
                US0::US0_ExitAfter(v24)
            }
        }
    };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_JOB_SLOTS"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = get_environment_variable_53(v30.clone());
    let v32: i32 = (v31.clone().len() as i32);
    let v33: i32 = 0i32;
    let v34: i32 = method54(v31.clone(), v32, v33);
    let v35: i32 = v32.wrapping_sub(1i32);
    let v36: Rc<str> = string_slice(&v31.clone(), v34 as i64, v35 as i64);
    let v37: i32 = (v36.clone().len() as i32);
    let v38: i32 = method55(v36.clone(), v37);
    let v39: Rc<str> = string_slice(&v36.clone(), 0i32 as i64, v38 as i64);
    let v40: Option<u64> = (v39).trim().parse::<u64>().ok();
    let v41: Option<u64> = method49(v40.clone());
    let mut v42: Option<US7> = v41.map(|x| v15(x));
    let v44: US7 = US7::US7_None;
    let v45: US7 = v44.clone();
    let v46: US7 = v42.unwrap_or(v45);
    let v54: US1 = match &v46 {
        US7::US7_None => {
            US1::US1_Unlimited
        }
        US7::US7_Some(v47) => {
            let v47: u64 = *v47;
            let v48: bool = v47 > 0u64;
            if v48 {
                US1::US1_AtMost(v47)
            } else {
                US1::US1_Unlimited
            }
        }
    };
    (v2.clone(), v0, v1, v29.clone(), v54.clone())
}
fn process_alive_56(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn host_claim_of_57(mut v0: u64, mut v1: u64, mut v2: bool) -> US8 {
    let mut v3: bool = v0 != 0u64;
    let v5: bool = if v3 {
        let mut v4: bool = v0 != v1;
        v4
    } else {
        false
    };
    let mut v6: bool = v5 && v2;
    if v6 {
        US8::US8_OtherHostServes(v0)
    } else {
        US8::US8_HostOwns
    }
}
fn no_children_58() -> kit_os_launch::KitChildren {
    let mut v0: kit_os_launch::KitChildren = kit_os_launch::kit_children();
    v0
}
fn host_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: u64 = decimal_u64_48(v1.clone());
    let mut v3: i32 = 4i32;
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
    let v5: u64 = decimal_u64_48(v4.clone());
    let (mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<str>, u64, u64, US0, US1) = host_terms_of_51(v2, v5);
    let mut v11: Rc<str> = host_pid_file_47(v6.clone());
    let v12: Rc<str> = text_or_empty_7(v11.clone());
    let v13: u64 = decimal_u64_48(v12.clone());
    let v14: u64 = own_pid_50();
    let mut v15: bool = process_alive_56(v13);
    let mut v16: US8 = host_claim_of_57(v13, v14, v15);
    match &v16 {
        US8::US8_HostOwns => {
            let v18: Rc<str> = requests_directory_2(v6.clone());
            let mut v19: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v18).err().map(|error| format!("create {}: {error}", &*v18)).unwrap_or_default());
            let mut v20: Rc<str> = host_pid_file_47(v6.clone());
            let mut v21: u64 = own_pid_50();
            let v22: Rc<str> = decimal_text_17(v21);
            let mut v23: Rc<str> = text_written_20(v20.clone(), v22.clone());
            let v24: kit_os_launch::KitChildren = no_children_58();
            let v25: u64 = now_ms_16();
            host_served_1(v24.clone(), v6.clone(), v7, v8, v9.clone(), v10.clone(), v25)
        }
        US8::US8_OtherHostServes(v17) => {
            let v17: u64 = *v17;
            0i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        host_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_host_run() -> i32 {
    closure0()()
}
