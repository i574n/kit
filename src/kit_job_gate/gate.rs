#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LiveGui,
    US1_TestSuite,
    US1_HeavyBuild,
    US1_Compile,
    US1_CompilerInstall,
    US1_TreeRead,
    US1_Bench,
    US1_CompileHold,
}
#[derive(Clone)]
enum US2 {
    US2_ProcessHolder,
    US2_LeaseHolder,
}
#[derive(Clone)]
enum US3 {
    US3_WindowQueued,
    US3_WindowGranted,
    US3_WindowReleased,
    US3_WindowReleasedWaiting,
    US3_WindowGaveUp,
    US3_WindowPruned,
    US3_WindowDropped,
}
#[derive(Clone)]
enum US4 {
    US4_Held,
    US4_Waiting,
}
#[derive(Clone)]
enum US5 {
    US5_TreeStopped,
    US5_SurvivorsLeft(u64),
}
#[derive(Clone)]
enum US6 {
    US6_JobFinished(i32),
    US6_JobTimedOut,
    US6_JobStopped,
    US6_JobSpawnFailed(Rc<str>),
    US6_JobSkipped(Rc<str>),
    US6_JobLost,
    US6_JobMemoryCapped,
    US6_JobExpiredInQueue,
}
fn joined_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_5() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_5();
    joined_3(v1, v0)
}
fn job_file_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_4(v0);
    joined_3(v2, v1)
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_7(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method8(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match &v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn now_ms_9() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn number_text_10(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn process_number_11() -> u64 {
    let v0: u64 = std::process::id() as u64;
    v0
}
fn file_exists_13(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn windows_root_14() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn window_file_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_3(v0.clone(), v4.clone())
}
fn file_removed_16(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn text_written_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_taken_20(mut v0: Rc<str>) -> bool {
    let v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
}
fn stored_rows_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_25(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn decimal_of_26(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method8(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match &v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn row_expired_24(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_25(v0, v2);
    let mut v4: u64 = decimal_of_26(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_28(mut v0: Rc<str>) -> US2 {
    let v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_25(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_25(v0.clone(), v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US2::US2_ProcessHolder
    } else {
        US2::US2_LeaseHolder
    }
}
fn window_pid_29(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_30(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_27(mut v0: Rc<str>) -> bool {
    let v1: US2 = holder_of_28(v0.clone());
    match &v1 {
        US2::US2_LeaseHolder => {
            false
        }
        US2::US2_ProcessHolder => {
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_25(v0, v2);
            let mut v4: u64 = window_pid_29(v3);
            let v5: bool = process_alive_30(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn event_logged_31(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_3(v0.clone(), v3.clone());
    let v5: u64 = now_ms_9();
    let mut v6: Rc<str> = number_text_10(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = match &v1 {
        US3::US3_WindowDropped => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US3::US3_WindowGaveUp => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US3::US3_WindowGranted => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_WindowPruned => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US3::US3_WindowQueued => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_WindowReleased => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US3::US3_WindowReleasedWaiting => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v22, &*v23, &*v23, &*v23].concat());
    let v25: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v21, &*v7, &*v24].concat());
    let mut v26: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let v27: bool = v26.is_ok();
    let v28: bool = v27 == false;
    if v28 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v29: std::fs::File = v26.unwrap();
    let mut v30: std::io::Result<()> = std::io::Write::write_all(&mut v29, (v25).as_bytes());
    let v31: bool = v30.is_ok();
    let v32: bool = v31 == false;
    if v32 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    }
}
fn other_rows_32(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_23(v1.clone());
            let mut v6: bool = row_expired_24(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_27(v5.clone())
            };
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_25(v5.clone(), v9);
                let v11: Rc<str> = window_file_15(v0.clone(), v10.clone());
                let v12: bool = file_removed_16(v11.clone());
                let v13: US3 = US3::US3_WindowPruned;
                event_logged_31(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_32(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let v16: Rc<str> = other_rows_32(v1.clone());
                let v17: bool = &*v3 == "";
                let v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v18, &*v5, &*v19, &*v19].concat());
                    v20.clone()
                };
                (v0, v1, v2, v3) = (v0.clone(), v16.clone(), v2, v21.clone());
                continue;
            }
        }
    }
}
fn free_mb_33() -> u64 {
    let v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_35(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US1::US1_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_LiveGui => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US1::US1_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn verdict_text_34(mut v0: US1, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> Rc<str> {
    let v8: Rc<str> = class_word_35(v0);
    let v9: Rc<str> = kit_window_admission::kit_window_verdict(&*v8, &*v1, v2, v3, &*v4, v5, v6, v7);
    v9
}
fn one_line_38(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_37(mut v0: Rc<str>, mut v1: US1, mut v2: US4, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = class_word_35(v1.clone());
    let v11: Rc<str> = match &v2 {
        US4::US4_Held => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US4::US4_Waiting => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v6, &*v11].concat());
    let v13: Rc<str> = one_line_38(v3.clone());
    let v14: Rc<str> = number_text_10(v4);
    let v15: Rc<str> = number_text_10(v5);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v15, &*v16, &*v16].concat());
    let v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v13, &*v6, &*v17].concat());
    v18.clone()
}
fn request_row_36(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: US4, mut v8: u64) -> Rc<str> {
    row_text_37(v3, v0, v7, v2, v4, v8)
}
fn lock_released_39(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn window_blocker_19(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64) -> Rc<str> {
    let v7: Rc<str> = windows_root_14();
    let v8: bool = lock_taken_20(v7.clone());
    let v9: bool = v8 == false;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason=registry-lock-busy"); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    } else {
        let v11: u64 = now_ms_9();
        let v12: Rc<str> = stored_rows_21(v7.clone());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = live_rows_22(v7.clone(), v12.clone(), v11, v13.clone());
        let v15: u64 = free_mb_33();
        let v16: Rc<str> = verdict_text_34(v0.clone(), v3.clone(), v4, v1, v14.clone(), v15, v11, v6);
        let v17: bool = &*v16 == "";
        if v17 {
            let v18: Rc<str> = window_file_15(v7.clone(), v3.clone());
            let v19: US4 = US4::US4_Held;
            let v20: u64 = v1.wrapping_mul(60000u64);
            let v21: u64 = v11.wrapping_add(v20);
            let v22: Rc<str> = request_row_36(v0.clone(), v1, v2.clone(), v3.clone(), v11, v5, v6, v19.clone(), v21);
            let v23: Rc<str> = text_written_18(v18.clone(), v22.clone());
            let v24: US3 = US3::US3_WindowGranted;
            let v25: US4 = US4::US4_Held;
            let mut v26: Rc<str> = request_row_36(v0.clone(), v1, v2.clone(), v3.clone(), v11, v5, v6, v25.clone(), v21);
            event_logged_31(v7.clone(), v24.clone(), v26.clone());
            lock_released_39(v7.clone());
            v13.clone()
        } else {
            let v27: Rc<str> = window_file_15(v7.clone(), v3.clone());
            let v28: US4 = US4::US4_Waiting;
            let mut v29: u64 = v11.wrapping_add(30000u64);
            let mut v30: Rc<str> = request_row_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v28.clone(), v29);
            let v31: Rc<str> = text_written_18(v27.clone(), v30.clone());
            lock_released_39(v7.clone());
            v16.clone()
        }
    }
}
fn compile_hold_loop_12(mut v0: Rc<str>, mut v1: US1, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> () {
    loop {
        let v9: u64 = now_ms_9();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = job_file_2(v0.clone(), v10.clone());
        let v12: bool = file_exists_13(v11.clone());
        if v12 {
            return ();
        } else {
            let v13: bool = v9 > v8;
            if v13 {
                let v14: Rc<str> = windows_root_14();
                let v15: Rc<str> = window_file_15(v14.clone(), v4.clone());
                let v16: bool = file_removed_16(v15.clone());
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = job_file_2(v0.clone(), v17.clone());
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started beside compile windows after "); } LIT.with(|lit| lit.clone()) };
                let v20: u64 = v9.wrapping_sub(v5);
                let v21: u64 = v20.wrapping_div(60000u64);
                let v22: Rc<str> = decimal_text_17(v21);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min of --quiet-wait"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v22, &*v23, &*v24, &*v24].concat());
                let mut v26: Rc<str> = text_written_18(v18.clone(), v25.clone());
                return ();
            } else {
                let v27: Rc<str> = window_blocker_19(v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7);
                let v28: bool = &*v27 == "";
                if v28 {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: Rc<str> = job_file_2(v0.clone(), v29.clone());
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("holding compiles id="); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v33: u64 = v9.wrapping_sub(v5);
                    let v34: Rc<str> = decimal_text_17(v33);
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v4, &*v32, &*v34, &*v35].concat());
                    let v37: Rc<str> = text_written_18(v30.clone(), v36.clone());
                    return ();
                } else {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = job_file_2(v0.clone(), v38.clone());
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting for compiles to end id="); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v4, &*v41, &*v27, &*v42].concat());
                    let v44: Rc<str> = text_written_18(v39.clone(), v43.clone());
                    let v45: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v45));
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7, v8);
                    continue;
                }
            }
        }
        return ();
    }
}
fn compile_hold_taken_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_2(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_or_empty_6(v2.clone());
    let mut v4: u64 = decimal_u64_7(v3.clone());
    let v5: bool = v4 == 0u64;
    if v5 {
        0i32
    } else {
        let mut v6: u64 = now_ms_9();
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = job_file_2(v0.clone(), v7.clone());
        let v9: Rc<str> = text_or_empty_6(v8.clone());
        let v10: u64 = decimal_u64_7(v9.clone());
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = job_file_2(v0.clone(), v11.clone());
        let v13: Rc<str> = text_or_empty_6(v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (kit job "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v0, &*v15, &*v16].concat());
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = number_text_10(v6);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v21: u64 = process_number_11();
        let v22: Rc<str> = number_text_10(v21);
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-compile-hold"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v22, &*v23].concat());
        let v25: u64 = v10.wrapping_mul(60000u64);
        let mut v26: u64 = v6.wrapping_add(v25);
        let v27: US1 = US1::US1_CompileHold;
        let v28: u64 = 0u64;
        compile_hold_loop_12(v0.clone(), v27.clone(), v4, v17.clone(), v24.clone(), v6, v26, v28, v26);
        0i32
    }
}
fn settled_word_42(mut v0: US6) -> Rc<str> {
    match &v0 {
        US6::US6_JobExpiredInQueue => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US6::US6_JobFinished(v1) => {
            let v1: i32 = *v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US6::US6_JobLost => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US6::US6_JobMemoryCapped => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US6::US6_JobSkipped(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US6::US6_JobSpawnFailed(v5) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spawn-failed"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US6::US6_JobStopped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US6::US6_JobTimedOut => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn released_mark_43(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_3(v0.clone(), v4.clone())
}
fn gate_loop_41(mut v0: Rc<str>, mut v1: US1, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> i32 {
    loop {
        let v9: u64 = now_ms_9();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heartbeat_ms"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = job_file_2(v0.clone(), v10.clone());
        let v12: Rc<str> = decimal_text_17(v9);
        let v13: Rc<str> = text_written_18(v11.clone(), v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = job_file_2(v0.clone(), v14.clone());
        let v16: bool = file_exists_13(v15.clone());
        if v16 {
            let v17: Rc<str> = windows_root_14();
            let v18: Rc<str> = window_file_15(v17.clone(), v4.clone());
            let v19: bool = file_removed_16(v18.clone());
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = job_file_2(v0.clone(), v20.clone());
            let v22: u64 = 0u64;
            let v23: bool = v22 == 0u64;
            let mut v26: US5 = if v23 {
                US5::US5_TreeStopped
            } else {
                US5::US5_SurvivorsLeft(v22)
            };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped on request"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = match &v26 {
                US5::US5_SurvivorsLeft(v31) => {
                    let v31: u64 = *v31;
                    let v32: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes left alive"); } LIT.with(|lit| lit.clone()) };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v34, &*v34, &*v34].concat());
                    v35.clone()
                }
                US5::US5_TreeStopped => {
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
                    v30.clone()
                }
            };
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v29, &*v37, &*v38, &*v38].concat());
            let v40: Rc<str> = std::rc::Rc::<str>::from((143i32).to_string());
            let v41: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v42, &*v39, &*v43, &*v38].concat());
            let v45: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v42, &*v41, &*v42, &*v44].concat());
            let v46: Rc<str> = text_written_18(v21.clone(), v45.clone());
            return 143i32;
        } else {
            let v47: bool = v9 > v8;
            if v47 {
                let v48: Rc<str> = windows_root_14();
                let v49: Rc<str> = window_file_15(v48.clone(), v4.clone());
                let v50: bool = file_removed_16(v49.clone());
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = job_file_2(v0.clone(), v51.clone());
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no "); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = class_word_35(v1.clone());
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window before the deadline (24 hours, or --slot-within); the job never started"); } LIT.with(|lit| lit.clone()) };
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v54, &*v55, &*v56, &*v56].concat());
                let v58: US6 = US6::US6_JobExpiredInQueue;
                let v59: Rc<str> = settled_word_42(v58.clone());
                let v60: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                let v61: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = std::rc::Rc::<str>::from([&*v59, &*v62, &*v57, &*v63, &*v56].concat());
                let v65: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v62, &*v61, &*v62, &*v64].concat());
                let v66: Rc<str> = text_written_18(v52.clone(), v65.clone());
                return 75i32;
            } else {
                let v67: Rc<str> = windows_root_14();
                let v68: Rc<str> = released_mark_43(v67.clone(), v4.clone());
                let v69: bool = file_exists_13(v68.clone());
                if v69 {
                    let v70: Rc<str> = windows_root_14();
                    let v71: Rc<str> = released_mark_43(v70.clone(), v4.clone());
                    let v72: bool = file_removed_16(v71.clone());
                    let v73: Rc<str> = window_file_15(v70.clone(), v4.clone());
                    let v74: bool = file_removed_16(v73.clone());
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = job_file_2(v0.clone(), v75.clone());
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("its waiting "); } LIT.with(|lit| lit.clone()) };
                    let v78: Rc<str> = class_word_35(v1.clone());
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window was released (kit window release); the job never started"); } LIT.with(|lit| lit.clone()) };
                    let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v81: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v78, &*v79, &*v80, &*v80].concat());
                    let v82: US6 = US6::US6_JobExpiredInQueue;
                    let v83: Rc<str> = settled_word_42(v82.clone());
                    let v84: Rc<str> = std::rc::Rc::<str>::from((75i32).to_string());
                    let v85: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<str> = std::rc::Rc::<str>::from([&*v83, &*v86, &*v81, &*v87, &*v80].concat());
                    let v89: Rc<str> = std::rc::Rc::<str>::from([&*v84, &*v86, &*v85, &*v86, &*v88].concat());
                    let v90: Rc<str> = text_written_18(v76.clone(), v89.clone());
                    return 75i32;
                } else {
                    let v91: Rc<str> = window_blocker_19(v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7);
                    let v92: bool = &*v91 == "";
                    if v92 {
                        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
                        let v94: Rc<str> = job_file_2(v0.clone(), v93.clone());
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted id="); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                        let v97: u64 = v9.wrapping_sub(v5);
                        let v98: Rc<str> = decimal_text_17(v97);
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v100: Rc<str> = std::rc::Rc::<str>::from([&*v95, &*v4, &*v96, &*v98, &*v99].concat());
                        let v101: Rc<str> = text_written_18(v94.clone(), v100.clone());
                        return 0i32;
                    } else {
                        let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
                        let v103: Rc<str> = job_file_2(v0.clone(), v102.clone());
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting id="); } LIT.with(|lit| lit.clone()) };
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v107: Rc<str> = std::rc::Rc::<str>::from([&*v104, &*v4, &*v105, &*v91, &*v106].concat());
                        let v108: Rc<str> = text_written_18(v103.clone(), v107.clone());
                        let v109: u64 = 500u64;
                        std::thread::sleep(std::time::Duration::from_millis(v109));
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5, v6, v7, v8);
                        continue;
                    }
                }
            }
        }
    }
}
fn windowed_gate_40(mut v0: Rc<str>, mut v1: US1) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = job_file_2(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_6(v3.clone());
    let v5: u64 = decimal_u64_7(v4.clone());
    let mut v6: u64 = now_ms_9();
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = job_file_2(v0.clone(), v7.clone());
    let v9: Rc<str> = text_or_empty_6(v8.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (kit job "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v0, &*v11, &*v12].concat());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = number_text_10(v6);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v17: u64 = process_number_11();
    let v18: Rc<str> = number_text_10(v17);
    let v19: Rc<str> = class_word_35(v1.clone());
    let v20: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v19, &*v12, &*v12, &*v12].concat());
    let v21: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v18, &*v20].concat());
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = job_file_2(v0.clone(), v22.clone());
    let v24: Rc<str> = text_or_empty_6(v23.clone());
    let v25: u64 = decimal_u64_7(v24.clone());
    let mut v26: bool = v25 != 0u64;
    let mut v29: bool = if v26 {
        let v27: u64 = v6.wrapping_add(86400000u64);
        let v28: bool = v25 < v27;
        v28
    } else {
        false
    };
    let v31: u64 = if v29 {
        v25
    } else {
        let mut v30: u64 = v6.wrapping_add(86400000u64);
        v30
    };
    let v32: u64 = v5.wrapping_div(60000u64);
    let v33: u64 = v32.wrapping_add(1u64);
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = job_file_2(v0.clone(), v34.clone());
    let v36: Rc<str> = text_or_empty_6(v35.clone());
    let v37: u64 = decimal_u64_7(v36.clone());
    gate_loop_41(v0.clone(), v1.clone(), v33, v13.clone(), v21.clone(), v6, v31, v37, v31)
}
fn job_window_gate_0(mut v0: Rc<str>) -> i32 {
    let v1: i32 = kit_job_slot::kit_job_slot_gate(&*v0);
    let v2: bool = v1 != 0i32;
    if v2 {
        v1
    } else {
        let mut v3: US1 = US1::US1_HeavyBuild;
        let mut v4: i32 = windowed_gate_40(v0.clone(), v3);
        let v5: bool = v4 != 0i32;
        if v5 {
            v4
        } else {
            compile_hold_taken_1(v0)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        job_window_gate_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_compile_gate_44(mut v0: Rc<str>) -> i32 {
    let v1: US1 = US1::US1_Compile;
    windowed_gate_40(v0, v1)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        job_compile_gate_44(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_window_gate(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_job_compile_gate(v0: &str) -> i32 {
    closure2()(Rc::<str>::from(v0))
}
