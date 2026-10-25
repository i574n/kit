#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_LiveGui,
    US0_TestSuite,
    US0_HeavyBuild,
    US0_Compile,
    US0_CompilerInstall,
    US0_TreeRead,
    US0_Bench,
    US0_CompileHold,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
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
    US4_Granted,
    US4_ReleasedWhileWaiting,
    US4_WaitExpired,
    US4_KeepWaiting,
}
#[derive(Clone)]
enum US5 {
    US5_Held,
    US5_Waiting,
}
#[derive(Clone)]
enum US6 {
    US6_ClassKnown(US0),
    US6_ClassUnknown,
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_3(v3.clone());
    2i32
}
fn windows_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn lock_taken_6(mut v0: Rc<str>) -> bool {
    let mut v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
}
fn now_ms_7() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn stored_rows_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_12(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_13(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method14(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let mut v6: US1 = US1::US1_None;
    let mut v7: US1 = v6.clone();
    let mut v8: US1 = v4.unwrap_or(v7);
    match &v8 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn row_expired_11(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_12(v0.clone(), v2);
    let mut v4: u64 = decimal_of_13(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_16(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_12(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_12(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US2::US2_ProcessHolder
    } else {
        US2::US2_LeaseHolder
    }
}
fn window_pid_17(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_18(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_15(mut v0: Rc<str>) -> bool {
    let mut v1: US2 = holder_of_16(v0.clone());
    match &v1 {
        US2::US2_LeaseHolder => {
            false
        }
        US2::US2_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_12(v0.clone(), v2);
            let mut v4: u64 = window_pid_17(v3.clone());
            let mut v5: bool = process_alive_18(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn window_file_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_20(v0.clone(), v4.clone())
}
fn file_removed_21(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn number_text_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn event_logged_22(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_20(v0.clone(), v3.clone());
    let mut v5: u64 = now_ms_7();
    let mut v6: Rc<str> = number_text_23(v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v1 {
        US3::US3_WindowDropped => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US3::US3_WindowGaveUp => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US3::US3_WindowGranted => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_WindowPruned => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US3::US3_WindowQueued => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US3::US3_WindowReleased => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US3::US3_WindowReleasedWaiting => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v22, &*v23, &*v23, &*v23].concat());
    let mut v25: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v7, &*v21, &*v7, &*v24].concat());
    let mut v26: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let mut v27: bool = v26.is_ok();
    let mut v28: bool = v27 == false;
    if v28 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v29: std::fs::File = v26.unwrap();
    let mut v30: std::io::Result<()> = std::io::Write::write_all(&mut v29, (v25).as_bytes());
    let mut v31: bool = v30.is_ok();
    let mut v32: bool = v31 == false;
    if v32 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    }
}
fn other_rows_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_10(v1.clone());
            let mut v6: bool = row_expired_11(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_15(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_12(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_19(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_21(v11.clone());
                let mut v13: US3 = US3::US3_WindowPruned;
                event_logged_22(v0.clone(), v13.clone(), v5.clone());
                let mut v14: Rc<str> = other_rows_24(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_24(v1.clone());
                let mut v17: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v20: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v18, &*v5, &*v19, &*v19].concat());
                    v20.clone()
                };
                (v0, v1, v2, v3) = (v0.clone(), v16.clone(), v2, v21.clone());
                continue;
            }
        }
    }
}
fn free_mb_25() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("EOIE_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_27(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US0::US0_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US0::US0_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US0::US0_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US0::US0_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US0::US0_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn verdict_text_26(mut v0: US0, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> Rc<str> {
    let mut v8: Rc<str> = class_word_27(v0.clone());
    let mut v9: Rc<str> = eoie_window_admission::eoie_window_verdict(&*v8, &*v1, v2, v3, &*v4, v5, v6, v7);
    v9.clone()
}
fn released_mark_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_20(v0.clone(), v4.clone())
}
fn file_exists_29(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_file();
    v8
}
fn wait_step_of_30(mut v0: bool, mut v1: bool, mut v2: u64, mut v3: u64) -> US4 {
    if v1 {
        US4::US4_ReleasedWhileWaiting
    } else {
        if v0 {
            US4::US4_Granted
        } else {
            let mut v6: bool = v2 >= v3;
            if v6 {
                US4::US4_WaitExpired
            } else {
                US4::US4_KeepWaiting
            }
        }
    }
}
fn one_line_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_32(mut v0: Rc<str>, mut v1: US0, mut v2: US5, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = class_word_27(v1.clone());
    let mut v11: Rc<str> = match &v2 {
        US5::US5_Held => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_Waiting => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let mut v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v6, &*v11].concat());
    let mut v13: Rc<str> = one_line_33(v3.clone());
    let mut v14: Rc<str> = number_text_23(v4);
    let mut v15: Rc<str> = number_text_23(v5);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v15, &*v16, &*v16].concat());
    let mut v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v13, &*v6, &*v17].concat());
    v18.clone()
}
fn request_row_31(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: US5, mut v8: u64) -> Rc<str> {
    row_text_32(v3.clone(), v0.clone(), v7.clone(), v2.clone(), v4, v8)
}
fn text_written_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_released_35(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn request_head_36(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window request "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" class="); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = class_word_27(v0.clone());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v3, &*v13].concat());
    v14.clone()
}
fn stdout_line_37(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn requested_4(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> i32 {
    loop {
        let mut v8: Rc<str> = windows_root_5();
        let mut v9: bool = lock_taken_6(v8.clone());
        let mut v10: bool = v9 == false;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: the registry lock stayed taken for 20 s"); } LIT.with(|lit| lit.clone()) };
            return refused_2(v11.clone());
        } else {
            let mut v13: u64 = now_ms_7();
            let mut v14: Rc<str> = stored_rows_8(v8.clone());
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = live_rows_9(v8.clone(), v14.clone(), v13, v15.clone());
            let mut v17: u64 = free_mb_25();
            let mut v18: Rc<str> = verdict_text_26(v0.clone(), v3.clone(), v4, v1, v16.clone(), v17, v13, v6);
            let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<str> = released_mark_28(v8.clone(), v3.clone());
            let mut v21: bool = file_exists_29(v20.clone());
            let mut v22: US4 = wait_step_of_30(v19, v21, v13, v5);
            match &v22 {
                US4::US4_Granted => {
                    let mut v36: u64 = v1.wrapping_mul(60000u64);
                    let mut v37: u64 = v13.wrapping_add(v36);
                    let mut v38: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let mut v39: US5 = US5::US5_Held;
                    let mut v40: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v39.clone(), v37);
                    let mut v41: Rc<str> = text_written_34(v38.clone(), v40.clone());
                    let mut v42: US3 = US3::US3_WindowGranted;
                    let mut v43: US5 = US5::US5_Held;
                    let mut v44: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v43.clone(), v37);
                    event_logged_22(v8.clone(), v42.clone(), v44.clone());
                    lock_released_35(v8.clone());
                    let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v46: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v45.clone());
                    let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                    let mut v48: Rc<str> = number_text_23(v1);
                    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
                    let mut v50: Rc<str> = number_text_23(v17);
                    let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v52: u64 = v13.wrapping_sub(v4);
                    let mut v53: Rc<str> = number_text_23(v52);
                    let mut v54: Rc<str> = std::rc::Rc::<str>::from([&*v50, &*v51, &*v53, &*v15, &*v15].concat());
                    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v47, &*v48, &*v49, &*v54].concat());
                    stdout_line_37(v55.clone());
                    return 0i32;
                }
                US4::US4_KeepWaiting => {
                    let mut v65: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let mut v66: US5 = US5::US5_Waiting;
                    let mut v67: u64 = v13.wrapping_add(30000u64);
                    let mut v68: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v66.clone(), v67);
                    let mut v69: Rc<str> = text_written_34(v65.clone(), v68.clone());
                    if v7 {
                        ()
                    } else {
                        let mut v70: US3 = US3::US3_WindowQueued;
                        let mut v71: US5 = US5::US5_Waiting;
                        let mut v72: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v71.clone(), v67);
                        event_logged_22(v8.clone(), v70.clone(), v72.clone())
                    };
                    lock_released_35(v8.clone());
                    if v7 {
                        ()
                    } else {
                        let mut v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
                        let mut v74: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v73.clone());
                        let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let mut v76: Rc<str> = std::rc::Rc::<str>::from([&*v74, &*v75, &*v18, &*v15, &*v15].concat());
                        stdout_line_37(v76.clone())
                    };
                    let mut v77: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v77));
                    let mut v78: bool = true;
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v78);
                    continue;
                }
                US4::US4_ReleasedWhileWaiting => {
                    let mut v23: Rc<str> = released_mark_28(v8.clone(), v3.clone());
                    let mut v24: bool = file_removed_21(v23.clone());
                    let mut v25: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let mut v26: bool = file_removed_21(v25.clone());
                    let mut v27: US3 = US3::US3_WindowReleasedWaiting;
                    let mut v28: US5 = US5::US5_Waiting;
                    let mut v29: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v28.clone(), v13);
                    event_logged_22(v8.clone(), v27.clone(), v29.clone());
                    lock_released_35(v8.clone());
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                    let mut v31: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v30.clone());
                    let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" while waiting waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let mut v33: u64 = v13.wrapping_sub(v4);
                    let mut v34: Rc<str> = number_text_23(v33);
                    let mut v35: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v32, &*v34, &*v15, &*v15].concat());
                    stdout_line_37(v35.clone());
                    return 1i32;
                }
                US4::US4_WaitExpired => {
                    let mut v56: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let mut v57: bool = file_removed_21(v56.clone());
                    let mut v58: US3 = US3::US3_WindowGaveUp;
                    let mut v59: US5 = US5::US5_Waiting;
                    let mut v60: Rc<str> = request_row_31(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v59.clone(), v13);
                    event_logged_22(v8.clone(), v58.clone(), v60.clone());
                    lock_released_35(v8.clone());
                    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                    let mut v62: Rc<str> = request_head_36(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v61.clone());
                    let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v63, &*v18, &*v15, &*v15].concat());
                    stdout_line_37(v64.clone());
                    return 1i32;
                }
            }
        }
    }
}
fn class_of_38(mut v0: Rc<str>) -> US6 {
    let mut v1: US0 = US0::US0_LiveGui;
    let mut v2: Rc<str> = class_word_27(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US0 = US0::US0_LiveGui;
        US6::US6_ClassKnown(v4.clone())
    } else {
        let mut v6: US0 = US0::US0_TestSuite;
        let mut v7: Rc<str> = class_word_27(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US0 = US0::US0_TestSuite;
            US6::US6_ClassKnown(v9.clone())
        } else {
            let mut v11: US0 = US0::US0_HeavyBuild;
            let mut v12: Rc<str> = class_word_27(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US0 = US0::US0_HeavyBuild;
                US6::US6_ClassKnown(v14.clone())
            } else {
                let mut v16: US0 = US0::US0_Compile;
                let mut v17: Rc<str> = class_word_27(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US0 = US0::US0_Compile;
                    US6::US6_ClassKnown(v19.clone())
                } else {
                    let mut v21: US0 = US0::US0_CompilerInstall;
                    let mut v22: Rc<str> = class_word_27(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US0 = US0::US0_CompilerInstall;
                        US6::US6_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US0 = US0::US0_TreeRead;
                        let mut v27: Rc<str> = class_word_27(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US0 = US0::US0_TreeRead;
                            US6::US6_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US0 = US0::US0_Bench;
                            let mut v32: Rc<str> = class_word_27(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US0 = US0::US0_Bench;
                                US6::US6_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US0 = US0::US0_CompileHold;
                                let mut v37: Rc<str> = class_word_27(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US0 = US0::US0_CompileHold;
                                    US6::US6_ClassKnown(v39.clone())
                                } else {
                                    US6::US6_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn process_number_39() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn option_value_40(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn window_requested_1() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: US6 = class_of_38(v1.clone());
    match &v2 {
        US6::US6_ClassKnown(v44) => {
            let mut v44: US0 = v44.clone();
            let mut v45: i32 = 4i32;
            let mut v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v45 as usize).unwrap_or_default());
            let mut v47: u64 = decimal_of_13(v46.clone());
            let mut v48: bool = v47 == 0u64;
            if v48 {
                let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: minutes must be a whole number above 0, received '"); } LIT.with(|lit| lit.clone()) };
                let mut v50: i32 = 4i32;
                let mut v51: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v50 as usize).unwrap_or_default());
                let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
                let mut v54: US0 = US0::US0_LiveGui;
                let mut v55: Rc<str> = class_word_27(v54.clone());
                let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v55.clone()));
                let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v56.clone()));
                let mut v58: US0 = US0::US0_TestSuite;
                let mut v59: Rc<str> = class_word_27(v58.clone());
                let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v59.clone()));
                let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v57.clone(), v60.clone()));
                let mut v62: US0 = US0::US0_HeavyBuild;
                let mut v63: Rc<str> = class_word_27(v62.clone());
                let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v63.clone()));
                let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v61.clone(), v64.clone()));
                let mut v66: US0 = US0::US0_Compile;
                let mut v67: Rc<str> = class_word_27(v66.clone());
                let mut v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v67.clone()));
                let mut v69: Rc<str> = Rc::<str>::from(format!("{}{}", v65.clone(), v68.clone()));
                let mut v70: US0 = US0::US0_CompilerInstall;
                let mut v71: Rc<str> = class_word_27(v70.clone());
                let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v71.clone()));
                let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v72.clone()));
                let mut v74: US0 = US0::US0_TreeRead;
                let mut v75: Rc<str> = class_word_27(v74.clone());
                let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v75.clone()));
                let mut v77: Rc<str> = Rc::<str>::from(format!("{}{}", v73.clone(), v76.clone()));
                let mut v78: US0 = US0::US0_Bench;
                let mut v79: Rc<str> = class_word_27(v78.clone());
                let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v79.clone()));
                let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v77.clone(), v80.clone()));
                let mut v82: US0 = US0::US0_CompileHold;
                let mut v83: Rc<str> = class_word_27(v82.clone());
                let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v83.clone()));
                let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v81.clone(), v84.clone()));
                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v87: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v85, &*v86, &*v86, &*v86].concat());
                let mut v88: Rc<str> = std::rc::Rc::<str>::from([&*v49, &*v51, &*v52, &*v87, &*v86].concat());
                refused_2(v88.clone())
            } else {
                let mut v90: u64 = now_ms_7();
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
                let mut v92: Rc<str> = number_text_23(v90);
                let mut v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v94: u64 = process_number_39();
                let mut v95: Rc<str> = number_text_23(v94);
                let mut v96: Rc<str> = class_word_27(v44.clone());
                let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v98: Rc<str> = std::rc::Rc::<str>::from([&*v93, &*v96, &*v97, &*v97, &*v97].concat());
                let mut v99: Rc<str> = std::rc::Rc::<str>::from([&*v91, &*v92, &*v93, &*v95, &*v98].concat());
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                let mut v101: Rc<str> = option_value_40(v100.clone());
                let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
                let mut v103: Rc<str> = option_value_40(v102.clone());
                let mut v104: bool = v103.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("until-granted"); } LIT.with(|lit| lit.clone()) };
                let mut v109: u64 = if v104 {
                    9223372036854775807u64
                } else {
                    let mut v105: Rc<str> = option_value_40(v102.clone());
                    let mut v106: u64 = decimal_of_13(v105.clone());
                    let mut v107: u64 = v106.wrapping_mul(60000u64);
                    let mut v108: u64 = v90.wrapping_add(v107);
                    v108
                };
                let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
                let mut v111: Rc<str> = option_value_40(v110.clone());
                let mut v112: u64 = decimal_of_13(v111.clone());
                let mut v113: bool = false;
                requested_4(v44.clone(), v47, v101.clone(), v99.clone(), v90, v109, v112, v113)
            }
        }
        US6::US6_ClassUnknown => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v4: i32 = 3i32;
            let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v4 as usize).unwrap_or_default());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
            let mut v8: US0 = US0::US0_LiveGui;
            let mut v9: Rc<str> = class_word_27(v8.clone());
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v9.clone()));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v10.clone()));
            let mut v12: US0 = US0::US0_TestSuite;
            let mut v13: Rc<str> = class_word_27(v12.clone());
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v13.clone()));
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), v14.clone()));
            let mut v16: US0 = US0::US0_HeavyBuild;
            let mut v17: Rc<str> = class_word_27(v16.clone());
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v17.clone()));
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15.clone(), v18.clone()));
            let mut v20: US0 = US0::US0_Compile;
            let mut v21: Rc<str> = class_word_27(v20.clone());
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v21.clone()));
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19.clone(), v22.clone()));
            let mut v24: US0 = US0::US0_CompilerInstall;
            let mut v25: Rc<str> = class_word_27(v24.clone());
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v25.clone()));
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), v26.clone()));
            let mut v28: US0 = US0::US0_TreeRead;
            let mut v29: Rc<str> = class_word_27(v28.clone());
            let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v29.clone()));
            let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v27.clone(), v30.clone()));
            let mut v32: US0 = US0::US0_Bench;
            let mut v33: Rc<str> = class_word_27(v32.clone());
            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v33.clone()));
            let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v31.clone(), v34.clone()));
            let mut v36: US0 = US0::US0_CompileHold;
            let mut v37: Rc<str> = class_word_27(v36.clone());
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v37.clone()));
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v35.clone(), v38.clone()));
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v39, &*v40, &*v40, &*v40].concat());
            let mut v42: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v5, &*v6, &*v41, &*v40].concat());
            refused_2(v42.clone())
        }
    }
}
fn text_or_empty_42(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn window_released_41() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = windows_root_5();
    let mut v3: Rc<str> = window_file_19(v2.clone(), v1.clone());
    let mut v4: Rc<str> = text_or_empty_42(v3.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = v1.starts_with(&*v5);
    let mut v7: bool = v6 == false;
    if v7 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v1, &*v9, &*v10, &*v10].concat());
        refused_2(v11.clone())
    } else {
        let mut v13: Rc<str> = window_file_19(v2.clone(), v1.clone());
        let mut v14: bool = file_removed_21(v13.clone());
        if v14 {
            let mut v15: i32 = 2i32;
            let mut v16: Rc<str> = tab_field_12(v4.clone(), v15);
            let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            let mut v18: US3 = US3::US3_WindowReleased;
            event_logged_22(v2.clone(), v18.clone(), v4.clone());
            let mut v23: Rc<str> = if v17 {
                let mut v19: Rc<str> = released_mark_28(v2.clone(), v1.clone());
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                text_written_34(v19.clone(), v20.clone())
            } else {
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22.clone()
            };
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window release ok id="); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = if v17 {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" requester=told"); } LIT.with(|lit| lit.clone()) };
                v25.clone()
            } else {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26.clone()
            };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v1, &*v27, &*v28, &*v28].concat());
            stdout_line_37(v29.clone());
            0i32
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; it was released or expired"); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v1, &*v31, &*v32, &*v32].concat());
            refused_2(v33.clone())
        }
    }
}
fn window_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request"); } LIT.with(|lit| lit.clone()) };
    if v2 {
        window_requested_1()
    } else {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            window_released_41()
        } else {
            let mut v6: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
            if v6 {
                let mut v7: i32 = eoie_window_query::eoie_window_list_run();
                v7
            } else {
                let mut v8: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                if v8 {
                    let mut v9: i32 = eoie_window_query::eoie_window_check_run();
                    v9
                } else {
                    let mut v10: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                    if v10 {
                        let mut v11: i32 = eoie_window_run::eoie_window_run_run();
                        v11
                    } else {
                        let mut v12: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claim"); } LIT.with(|lit| lit.clone()) };
                        if v12 {
                            let mut v13: i32 = eoie_window_claim::eoie_window_claim_run();
                            v13
                        } else {
                            let mut v14: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-clear"); } LIT.with(|lit| lit.clone()) };
                            if v14 {
                                let mut v15: i32 = eoie_window_wait::eoie_window_wait_run();
                                v15
                            } else {
                                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) };
                                let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                                let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
                                let mut v19: US0 = US0::US0_LiveGui;
                                let mut v20: Rc<str> = class_word_27(v19.clone());
                                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v20.clone()));
                                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v21.clone()));
                                let mut v23: US0 = US0::US0_TestSuite;
                                let mut v24: Rc<str> = class_word_27(v23.clone());
                                let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v24.clone()));
                                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22.clone(), v25.clone()));
                                let mut v27: US0 = US0::US0_HeavyBuild;
                                let mut v28: Rc<str> = class_word_27(v27.clone());
                                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v28.clone()));
                                let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26.clone(), v29.clone()));
                                let mut v31: US0 = US0::US0_Compile;
                                let mut v32: Rc<str> = class_word_27(v31.clone());
                                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v32.clone()));
                                let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30.clone(), v33.clone()));
                                let mut v35: US0 = US0::US0_CompilerInstall;
                                let mut v36: Rc<str> = class_word_27(v35.clone());
                                let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v36.clone()));
                                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v34.clone(), v37.clone()));
                                let mut v39: US0 = US0::US0_TreeRead;
                                let mut v40: Rc<str> = class_word_27(v39.clone());
                                let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v40.clone()));
                                let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38.clone(), v41.clone()));
                                let mut v43: US0 = US0::US0_Bench;
                                let mut v44: Rc<str> = class_word_27(v43.clone());
                                let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v44.clone()));
                                let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42.clone(), v45.clone()));
                                let mut v47: US0 = US0::US0_CompileHold;
                                let mut v48: Rc<str> = class_word_27(v47.clone());
                                let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v48.clone()));
                                let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v46.clone(), v49.clone()));
                                let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let mut v52: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v50, &*v51, &*v51, &*v51].concat());
                                let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v1, &*v17, &*v52, &*v51].concat());
                                refused_2(v53.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn dropped_tagged_44(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> u64 {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3;
        } else {
            let mut v5: Rc<str> = first_row_10(v1.clone());
            let mut v6: i32 = 3i32;
            let mut v7: Rc<str> = tab_field_12(v5.clone(), v6);
            let mut v8: bool = v7.contains(&*v2);
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_12(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_19(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_21(v11.clone());
                let mut v13: US3 = US3::US3_WindowDropped;
                event_logged_22(v0.clone(), v13.clone(), v5.clone());
                let mut v14: Rc<str> = other_rows_24(v1.clone());
                let mut v15: u64 = v3.wrapping_add(1u64);
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2.clone(), v15);
                continue;
            } else {
                let mut v17: Rc<str> = other_rows_24(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v17.clone(), v2.clone(), v3);
                continue;
            }
        }
    }
}
fn job_windows_dropped_43(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = windows_root_5();
    let mut v2: Rc<str> = stored_rows_8(v1.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v5, &*v5].concat());
    let mut v7: u64 = 0u64;
    dropped_tagged_44(v1.clone(), v2.clone(), v6.clone(), v7)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        job_windows_dropped_43(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn heavy_wrapper_45(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v5, &*v3, &*v6, &*v7].concat());
    let mut v9: u64 = v4.wrapping_div(60000u64);
    let mut v10: u64 = v9.wrapping_add(1u64);
    let mut v11: Rc<str> = number_text_23(v10);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$argv = @(Get-Content -LiteralPath '"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("')\n. '"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\n"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v1, &*v13, &*v0, &*v14].concat());
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy -Label '"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("' -MaxMinutes "); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v8, &*v17, &*v11, &*v18].concat());
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ & $argv[0] @($argv | Select-Object -Skip 1) }\n"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit $LASTEXITCODE\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v19, &*v20, &*v21, &*v7].concat());
    v22.clone()
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64| -> Rc<str> {
        heavy_wrapper_45(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_run() -> i32 {
    closure0()()
}
pub fn eoie_job_windows_dropped(v0: &str) -> u64 {
    closure2()(Rc::<str>::from(v0))
}
pub fn eoie_heavy_wrapper(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
