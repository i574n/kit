#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mut0 { pub l0: i32 }
pub struct Mut1 { pub l0: i32 }
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_HostMissing,
    US1_HostRunning(u64),
}
pub struct Mut2 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_SupervisorUp(u64),
    US2_SupervisorHeld(Rc<str>),
    US2_SupervisorLate,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US2),
    US3_None,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn held_directory_2() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_4();
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
    joined_3(v0, v1)
}
fn directory_exists_5(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
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
fn method6(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method9(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut1>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let v5: i32 = v2.borrow().l0;
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
fn held_names_1() -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v0: Rc<str> = held_directory_2();
    let v1: bool = directory_exists_5(v0);
    if v1 {
        let v2: Rc<str> = held_directory_2();
        let mut v4: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Vec<std::string::String> = v4.unwrap();
        let v7: Vec<std::string::String> = method6(v6);
        let v8: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v7));
        let v9: i32 = (v8.clone().borrow().len() as i32);
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v9 as usize]));
        let mut v11: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
        while method7(v9, &v11) {
            let v13: i32 = v11.borrow().l0;
            let v14: std::string::String = v8.borrow()[v13 as usize].clone();
            let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            v10.borrow_mut()[v13 as usize] = v16;
            let v17: i32 = v13.wrapping_add(1i32);
            v11.borrow_mut().l0 = v17;
            ()
        };
        let v18: Rc<RefCell<Vec<Rc<str>>>> = v10 ;
        let mut v19: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method8();
        let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().clone()));
        let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v18.borrow().iter().map(|x| v19(x.clone())).collect::<Vec<_>>()));
        let v22: i32 = (v18.borrow().len() as i32);
        let mut v23: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
        while method7(v22, &v23) {
            let v25: i32 = v23.borrow().l0;
            let v26: Rc<str> = v20.borrow()[v25 as usize].clone();
            let v27: Rc<str> = v21.borrow()[v25 as usize].clone();
            let mut v28: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v25 }));
            while method9(&v21, v27.clone(), &v28) {
                let v30: i32 = v28.borrow().l0;
                let v31: i32 = v30.wrapping_sub(1i32);
                let v32: Rc<str> = v20.borrow()[v31 as usize].clone();
                v20.borrow_mut()[v30 as usize] = v32;
                let v33: Rc<str> = v21.borrow()[v31 as usize].clone();
                v21.borrow_mut()[v30 as usize] = v33;
                v28.borrow_mut().l0 = v31;
                ()
            };
            let v34: i32 = v28.borrow().l0;
            v20.borrow_mut()[v34 as usize] = v26;
            let v35: i32 = v28.borrow().l0;
            v21.borrow_mut()[v35 as usize] = v27;
            let v36: i32 = v25.wrapping_add(1i32);
            v23.borrow_mut().l0 = v36;
            ()
        };
        let v37: Rc<RefCell<Vec<Rc<str>>>> = v20 ;
        v37
    } else {
        let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v38
    }
}
fn caller_owner_10() -> (Rc<str>, u64, u64) {
    let (mut v0, mut v1, mut v2): (Rc<str>, u64, u64) = kit_os_launch::kit_caller_owner();
    (v0, v1, v2)
}
fn host_home_directory_12(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "";
    if v1 {
        let v2: Rc<str> = jobs_root_4();
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shell-host"); } LIT.with(|lit| lit.clone()) };
        joined_3(v2, v3)
    } else {
        let v5: Rc<str> = jobs_root_4();
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hosts"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = joined_3(v5, v6);
        joined_3(v7, v0)
    }
}
fn host_pid_file_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = host_home_directory_12(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1, v2)
}
fn text_or_empty_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_14(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method15(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure2();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn process_alive_16(mut v0: u64) -> bool {
    let v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn host_state_of_17(mut v0: u64, mut v1: bool) -> US1 {
    let v2: bool = v0 != 0u64;
    let mut v3: bool = v2 && v1;
    if v3 {
        US1::US1_HostRunning(v0)
    } else {
        US1::US1_HostMissing
    }
}
fn supervisor_executable_18() -> Rc<str> {
    let mut v0: Rc<str> = { let copy = || -> std::io::Result<std::path::PathBuf> { let source = std::env::current_exe()?; let metadata = std::fs::metadata(&source)?; let stamp = metadata.modified().ok().and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok()).map(|elapsed| elapsed.as_secs()).unwrap_or(0); let folder = std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("XDG_CACHE_HOME")).map(std::path::PathBuf::from).or_else(|| std::env::var_os("HOME").map(|home| std::path::PathBuf::from(home).join(".cache"))).unwrap_or_else(std::env::temp_dir).join("kit").join("supervisor"); std::fs::create_dir_all(&folder)?; let stem = source.file_stem().map(|stem| stem.to_string_lossy().into_owned()).unwrap_or_default(); let target = folder.join(format!("{}-{}-{}{}", stem, metadata.len(), stamp, std::env::consts::EXE_SUFFIX)); if !target.is_file() { let temporary = target.with_extension("tmp"); std::fs::copy(&source, &temporary)?; std::fs::rename(&temporary, &target)?; let day = std::time::Duration::from_secs(600); for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let same_stem = path.file_name().map(|name| name.to_string_lossy().starts_with(&format!("{stem}-"))).unwrap_or(false); let stale = entry.metadata().and_then(|metadata| metadata.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age > day).unwrap_or(false); if same_stem && stale && path != target { let _ = std::fs::remove_file(&path); } } } Ok(target) }; std::rc::Rc::<str>::from(copy().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default()) };
    v0
}
fn shell_started_20(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> u64 {
    let v2: u64 = kit_os_launch::kit_shell_started(&*v0, &(v1).borrow());
    v2
}
fn method24(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn decimal_text_22(mut v0: u64) -> Rc<str> {
    format_real_23(v0)
}
fn host_args_21(mut v0: u64, mut v1: u64) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: bool = v0 == 0u64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v3, v4]));
        v5
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = decimal_text_22(v0);
        let v9: Rc<str> = decimal_text_22(v1);
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v6, v7, v8, v9]));
        v10
    }
}
fn owner_started_25(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: u64) -> u64 {
    let mut v3: u64 = kit_os_launch::kit_owner_started(&*v0, &(v1).borrow(), v2);
    v3
}
fn host_started_19(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = host_args_21(v1, v2);
    let mut v4: bool = v1 == 0u64;
    if v4 {
        shell_started_20(v0, v3)
    } else {
        let mut v6: u64 = owner_started_25(v0.clone(), v3.clone(), v1);
        let v7: bool = v6 == 0u64;
        if v7 {
            shell_started_20(v0, v3)
        } else {
            v6
        }
    }
}
fn held_revived_0() -> Rc<str> {
    let mut v0: Rc<RefCell<Vec<Rc<str>>>> = held_names_1();
    let v1: i32 = (v0.borrow().len() as i32);
    let v2: bool = v1 == 0i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let (mut v4, mut v5, mut v6): (Rc<str>, u64, u64) = caller_owner_10();
        let v7: Rc<str> = host_pid_file_11(v4);
        let v8: Rc<str> = text_or_empty_13(v7);
        let v9: u64 = decimal_u64_14(v8);
        let mut v10: bool = process_alive_16(v9);
        let mut v11: US1 = host_state_of_17(v9, v10);
        match v11 {
            US1::US1_HostMissing => {
                let v14: Rc<str> = jobs_root_4();
                let v15: Rc<str> = supervisor_executable_18();
                let v16: u64 = host_started_19(v15, v5, v6);
                let v17: bool = v16 == 0u64;
                if v17 {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: held jobs wait and no job host could be started"); } LIT.with(|lit| lit.clone()) };
                    v18
                } else {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("revived"); } LIT.with(|lit| lit.clone()) };
                    v19
                }
            }
            US1::US1_HostRunning(v12) => {
                let v12: u64 = v12;
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
                v13
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        held_revived_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn requests_directory_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = host_home_directory_12(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("requests"); } LIT.with(|lit| lit.clone()) };
    joined_3(v1, v2)
}
fn job_directory_29(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_4();
    joined_3(v1, v0)
}
fn job_file_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_29(v0);
    joined_3(v2, v1)
}
fn text_written_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2
}
fn request_name_31(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".start"); } LIT.with(|lit| lit.clone()) }));
    v1
}
fn now_ms_32() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn supervisor_wait_of_34(mut v0: u64, mut v1: u64, mut v2: u64) -> US3 {
    let mut v3: bool = v0 != 0u64;
    if v3 {
        let mut v4: US2 = US2::US2_SupervisorUp(v0);
        US3::US3_Some(v4)
    } else {
        let mut v6: bool = v1 >= v2;
        if v6 {
            let v7: US2 = US2::US2_SupervisorLate;
            US3::US3_Some(v7)
        } else {
            US3::US3_None
        }
    }
}
fn held_file_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = held_directory_2();
    joined_3(v1, v0)
}
fn file_exists_36(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn supervisor_awaited_33(mut v0: Rc<str>, mut v1: u64) -> US2 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = job_file_28(v0.clone(), v2);
        let mut v4: Rc<str> = text_or_empty_13(v3);
        let v5: u64 = decimal_u64_14(v4);
        let mut v6: u64 = now_ms_32();
        let v7: US3 = supervisor_wait_of_34(v5, v6, v1);
        match v7 {
            US3::US3_None => {
                let v9: Rc<str> = held_file_35(v0.clone());
                let mut v10: bool = file_exists_36(v9);
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
                    return US2::US2_SupervisorHeld(v11);
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = job_file_28(v0.clone(), v13);
                    let v15: bool = file_exists_36(v14);
                    if v15 {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled"); } LIT.with(|lit| lit.clone()) };
                        return US2::US2_SupervisorHeld(v16);
                    } else {
                        let v18: u64 = 50u64;
                        std::thread::sleep(std::time::Duration::from_millis(v18));
                        continue;
                    }
                }
            }
            US3::US3_Some(v8) => {
                let v8: US2 = v8;
                return v8;
            }
        }
    }
}
fn request_removed_37(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn supervisor_requested_26(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_4();
    let v2: Rc<str> = supervisor_executable_18();
    let (mut v3, mut v4, mut v5): (Rc<str>, u64, u64) = caller_owner_10();
    let mut v6: Rc<str> = requests_directory_27(v3.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v6).err().map(|error| format!("create {}: {error}", &*v6)).unwrap_or_default());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-home"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = job_file_28(v0.clone(), v8);
    let mut v10: Rc<str> = text_written_30(v9, v3.clone());
    let mut v11: Rc<str> = request_name_31(v0.clone());
    let v12: Rc<str> = joined_3(v6, v11);
    let v13: Rc<str> = text_written_30(v12.clone(), v2.clone());
    let v14: Rc<str> = host_pid_file_11(v3.clone());
    let v15: Rc<str> = text_or_empty_13(v14);
    let v16: u64 = decimal_u64_14(v15);
    let v17: bool = process_alive_16(v16);
    let v18: US1 = host_state_of_17(v16, v17);
    let v22: u64 = match v18 {
        US1::US1_HostMissing => {
            host_started_19(v2.clone(), v4, v5)
        }
        US1::US1_HostRunning(v19) => {
            let mut v19: u64 = v19;
            0u64
        }
    };
    let mut v23: u64 = now_ms_32();
    let v24: u64 = v23.wrapping_add(5000u64);
    let v25: US2 = supervisor_awaited_33(v0.clone(), v24);
    match v25 {
        US2::US2_SupervisorHeld(v28) => {
            let mut v28: Rc<str> = v28;
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" host_home="); } LIT.with(|lit| lit.clone()) }));
            let v30: bool = &*v3 == "";
            let v32: Rc<str> = if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shared"); } LIT.with(|lit| lit.clone()) };
                v31
            } else {
                v3
            };
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v32));
            v33
        }
        US2::US2_SupervisorLate => {
            let v34: bool = request_removed_37(v12);
            if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervise"); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v35, v36, v0]));
                let v38: u64 = shell_started_20(v2, v37);
                let v39: bool = v38 == 0u64;
                if v39 {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: neither the job host nor a detached start launched the supervisor"); } LIT.with(|lit| lit.clone()) };
                    v40
                } else {
                    decimal_text_22(v38)
                }
            } else {
                let v43: u64 = now_ms_32();
                let v44: u64 = v43.wrapping_add(20000u64);
                let v45: US2 = supervisor_awaited_33(v0, v44);
                match v45 {
                    US2::US2_SupervisorHeld(v48) => {
                        let v48: Rc<str> = v48;
                        v48
                    }
                    US2::US2_SupervisorLate => {
                        let v49: bool = &*v3 == "";
                        let v51: Rc<str> = if v49 {
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shared"); } LIT.with(|lit| lit.clone()) };
                            v50
                        } else {
                            v3
                        };
                        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("starting host_home="); } LIT.with(|lit| lit.clone()) }, v51));
                        v52
                    }
                    US2::US2_SupervisorUp(v46) => {
                        let v46: u64 = v46;
                        decimal_text_22(v46)
                    }
                }
            }
        }
        US2::US2_SupervisorUp(v26) => {
            let v26: u64 = v26;
            decimal_text_22(v26)
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        supervisor_requested_26(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_host_revive() -> Rc<str> {
    closure0()()
}
pub fn kit_job_host_request(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub type SpiralShape_2617b4e49837 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut2;
