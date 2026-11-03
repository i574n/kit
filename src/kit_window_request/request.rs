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
    US0_TreeWrite,
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
struct Mut0 { l0: Rc<str> }
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
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_2(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_3(v3);
    2i32
}
fn windows_root_5() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
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
    v1
}
fn tab_field_12(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_13(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method14(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let mut v6: US1 = US1::US1_None;
    let mut v7: US1 = v6.clone();
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
fn row_expired_11(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_12(v0, v2);
    let mut v4: u64 = decimal_of_13(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_16(mut v0: Rc<str>) -> US2 {
    let mut v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_12(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_12(v0, v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
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
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_12(v0, v2);
            let mut v4: u64 = window_pid_17(v3);
            let v5: bool = process_alive_18(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_2(v1, v2, v3.clone(), v3.clone(), v3);
    joined_20(v0, v4)
}
fn file_removed_21(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method25(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_24(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method25(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_23(mut v0: u64) -> Rc<str> {
    format_real_24(v0)
}
fn event_logged_22(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_20(v0, v3);
    let v5: u64 = now_ms_7();
    let mut v6: Rc<str> = number_text_23(v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = match &v1 {
        US3::US3_WindowDropped => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14
        }
        US3::US3_WindowGaveUp => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US3::US3_WindowGranted => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US3::US3_WindowPruned => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13
        }
        US3::US3_WindowQueued => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US3::US3_WindowReleased => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US3::US3_WindowReleasedWaiting => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v11
        }
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = concat5_2(v2, v22, v23.clone(), v23.clone(), v23);
    let v25: Rc<str> = concat5_2(v6, v7.clone(), v21, v7, v24);
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
fn other_rows_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_10(v1.clone());
            let mut v6: bool = row_expired_11(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_15(v5.clone())
            };
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_12(v5.clone(), v9);
                let v11: Rc<str> = window_file_19(v0.clone(), v10.clone());
                let v12: bool = file_removed_21(v11.clone());
                let v13: US3 = US3::US3_WindowPruned;
                event_logged_22(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_26(v1.clone());
                v1 = v14.clone();
                continue;
            } else {
                let v16: Rc<str> = other_rows_26(v1.clone());
                let v17: bool = &*v3 == "";
                let v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_2(v3.clone(), v18.clone(), v5.clone(), v19.clone(), v19.clone())
                };
                (v1, v3) = (v16.clone(), v21.clone());
                continue;
            }
        }
    }
}
fn free_mb_27() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn class_word_29(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn verdict_text_28(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let v9: Rc<str> = class_word_29(v0);
    let v10: Rc<str> = kit_window_admission::kit_window_verdict(&*v9, &*v1, &*v2, v3, v4, &*v5, v6, v7, v8);
    v10
}
fn released_mark_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_2(v1, v2, v3.clone(), v3.clone(), v3);
    joined_20(v0, v4)
}
fn file_exists_31(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn wait_step_of_32(mut v0: bool, mut v1: bool, mut v2: u64, mut v3: u64) -> US4 {
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
fn one_line_35(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_34(mut v0: Rc<str>, mut v1: US0, mut v2: US5, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = class_word_29(v1);
    let v11: Rc<str> = match &v2 {
        US5::US5_Held => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US5::US5_Waiting => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    };
    let v12: Rc<str> = concat5_2(v0, v6.clone(), v7, v6.clone(), v11);
    let v13: Rc<str> = one_line_35(v3);
    let v14: Rc<str> = number_text_23(v4);
    let v15: Rc<str> = number_text_23(v5);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = concat5_2(v14, v6.clone(), v15, v16.clone(), v16);
    concat5_2(v12, v6.clone(), v13, v6, v17)
}
fn request_row_33(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: US5, mut v8: u64) -> Rc<str> {
    row_text_34(v3, v0, v7, v2, v4, v8)
}
fn text_written_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_released_37(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn request_head_38(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window request "); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" class="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = class_word_29(v0);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_2(v10, v11, v12.clone(), v12.clone(), v12);
    concat5_2(v8, v7, v9, v3, v13)
}
fn stdout_line_39(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn requested_4(mut v0: US0, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> i32 {
    loop {
        let v8: Rc<str> = windows_root_5();
        let v9: bool = lock_taken_6(v8.clone());
        let v10: bool = v9 == false;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: the registry lock stayed taken for 20 s"); } LIT.with(|lit| lit.clone()) };
            return refused_1(v11.clone());
        } else {
            let v13: u64 = now_ms_7();
            let v14: Rc<str> = stored_rows_8(v8.clone());
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = live_rows_9(v8.clone(), v14.clone(), v13, v15.clone());
            let v17: u64 = free_mb_27();
            let v18: Rc<str> = verdict_text_28(v0.clone(), v2.clone(), v3.clone(), v4, v1, v16.clone(), v17, v13, v6);
            let v19: bool = &*v18 == "";
            let v20: Rc<str> = released_mark_30(v8.clone(), v3.clone());
            let v21: bool = file_exists_31(v20.clone());
            let v22: US4 = wait_step_of_32(v19, v21, v13, v5);
            match &v22 {
                US4::US4_Granted => {
                    let v36: u64 = v1.wrapping_mul(60000u64);
                    let v37: u64 = v13.wrapping_add(v36);
                    let v38: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let v39: US5 = US5::US5_Held;
                    let v40: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v39.clone(), v37);
                    let v41: Rc<str> = text_written_36(v38.clone(), v40.clone());
                    let v42: US3 = US3::US3_WindowGranted;
                    let v43: US5 = US5::US5_Held;
                    let v44: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v43.clone(), v37);
                    event_logged_22(v8.clone(), v42.clone(), v44.clone());
                    lock_released_37(v8.clone());
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = request_head_38(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v45.clone());
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = number_text_23(v1);
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = number_text_23(v17);
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v52: u64 = v13.wrapping_sub(v4);
                    let v53: Rc<str> = number_text_23(v52);
                    let v54: Rc<str> = concat5_2(v50.clone(), v51.clone(), v53.clone(), v15.clone(), v15.clone());
                    let v55: Rc<str> = concat5_2(v46.clone(), v47.clone(), v48.clone(), v49.clone(), v54.clone());
                    stdout_line_39(v55.clone());
                    return 0i32;
                }
                US4::US4_KeepWaiting => {
                    let v65: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let v66: US5 = US5::US5_Waiting;
                    let v67: u64 = v13.wrapping_add(30000u64);
                    let v68: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v66.clone(), v67);
                    let v69: Rc<str> = text_written_36(v65.clone(), v68.clone());
                    if v7 {
                        ()
                    } else {
                        let v70: US3 = US3::US3_WindowQueued;
                        let v71: US5 = US5::US5_Waiting;
                        let v72: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v71.clone(), v67);
                        event_logged_22(v8.clone(), v70.clone(), v72.clone())
                    };
                    lock_released_37(v8.clone());
                    if v7 {
                        ()
                    } else {
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = request_head_38(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v73.clone());
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = concat5_2(v74.clone(), v75.clone(), v18.clone(), v15.clone(), v15.clone());
                        stdout_line_39(v76.clone())
                    };
                    let v77: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v77));
                    let v78: bool = true;
                    v7 = v78;
                    continue;
                }
                US4::US4_ReleasedWhileWaiting => {
                    let v23: Rc<str> = released_mark_30(v8.clone(), v3.clone());
                    let v24: bool = file_removed_21(v23.clone());
                    let v25: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let mut v26: bool = file_removed_21(v25.clone());
                    let v27: US3 = US3::US3_WindowReleasedWaiting;
                    let v28: US5 = US5::US5_Waiting;
                    let mut v29: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v28.clone(), v13);
                    event_logged_22(v8.clone(), v27.clone(), v29.clone());
                    lock_released_37(v8.clone());
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = request_head_38(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v30.clone());
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" while waiting waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v33: u64 = v13.wrapping_sub(v4);
                    let v34: Rc<str> = number_text_23(v33);
                    let v35: Rc<str> = concat5_2(v31.clone(), v32.clone(), v34.clone(), v15.clone(), v15.clone());
                    stdout_line_39(v35.clone());
                    return 1i32;
                }
                US4::US4_WaitExpired => {
                    let v56: Rc<str> = window_file_19(v8.clone(), v3.clone());
                    let v57: bool = file_removed_21(v56.clone());
                    let v58: US3 = US3::US3_WindowGaveUp;
                    let v59: US5 = US5::US5_Waiting;
                    let v60: Rc<str> = request_row_33(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v59.clone(), v13);
                    event_logged_22(v8.clone(), v58.clone(), v60.clone());
                    lock_released_37(v8.clone());
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = request_head_38(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v61.clone());
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = concat5_2(v62.clone(), v63.clone(), v18.clone(), v15.clone(), v15.clone());
                    stdout_line_39(v64.clone());
                    return 1i32;
                }
            }
        }
    }
}
fn class_of_40(mut v0: Rc<str>) -> US6 {
    let mut v1: US0 = US0::US0_LiveGui;
    let v2: Rc<str> = class_word_29(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US0 = US0::US0_LiveGui;
        US6::US6_ClassKnown(v4)
    } else {
        let mut v6: US0 = US0::US0_TestSuite;
        let mut v7: Rc<str> = class_word_29(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US0 = US0::US0_TestSuite;
            US6::US6_ClassKnown(v9)
        } else {
            let v11: US0 = US0::US0_HeavyBuild;
            let v12: Rc<str> = class_word_29(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US0 = US0::US0_HeavyBuild;
                US6::US6_ClassKnown(v14)
            } else {
                let v16: US0 = US0::US0_Compile;
                let v17: Rc<str> = class_word_29(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US0 = US0::US0_Compile;
                    US6::US6_ClassKnown(v19)
                } else {
                    let v21: US0 = US0::US0_CompilerInstall;
                    let v22: Rc<str> = class_word_29(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US0 = US0::US0_CompilerInstall;
                        US6::US6_ClassKnown(v24)
                    } else {
                        let mut v26: US0 = US0::US0_TreeRead;
                        let v27: Rc<str> = class_word_29(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let mut v29: US0 = US0::US0_TreeRead;
                            US6::US6_ClassKnown(v29)
                        } else {
                            let v31: US0 = US0::US0_Bench;
                            let v32: Rc<str> = class_word_29(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US0 = US0::US0_Bench;
                                US6::US6_ClassKnown(v34)
                            } else {
                                let v36: US0 = US0::US0_CompileHold;
                                let v37: Rc<str> = class_word_29(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US0 = US0::US0_CompileHold;
                                    US6::US6_ClassKnown(v39)
                                } else {
                                    let v41: US0 = US0::US0_TreeWrite;
                                    let v42: Rc<str> = class_word_29(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US0 = US0::US0_TreeWrite;
                                        US6::US6_ClassKnown(v44)
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
}
fn announce_refusal_41(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = windows_root_5();
    let mut v3: Rc<str> = kit_window_announce_check::kit_window_announced_refusal(&*v2, &*v0, v1);
    v3
}
fn process_number_42() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn option_value_43(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn window_requested_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: US6 = class_of_40(v1.clone());
    match &v2 {
        US6::US6_ClassKnown(v48) => {
            let v48: US0 = v48.clone();
            let v49: i32 = 4i32;
            let v50: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v49 as usize).unwrap_or_default());
            let v51: u64 = decimal_of_13(v50.clone());
            let v52: bool = v51 == 0u64;
            if v52 {
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: minutes must be a whole number above 0, received '"); } LIT.with(|lit| lit.clone()) };
                let v54: i32 = 4i32;
                let v55: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v54 as usize).unwrap_or_default());
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                let v58: US0 = US0::US0_LiveGui;
                let v59: Rc<str> = class_word_29(v58.clone());
                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v59.clone()));
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v60.clone()));
                let v62: US0 = US0::US0_TestSuite;
                let v63: Rc<str> = class_word_29(v62.clone());
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v63.clone()));
                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v61.clone(), v64.clone()));
                let v66: US0 = US0::US0_HeavyBuild;
                let v67: Rc<str> = class_word_29(v66.clone());
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v67.clone()));
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v65.clone(), v68.clone()));
                let v70: US0 = US0::US0_Compile;
                let v71: Rc<str> = class_word_29(v70.clone());
                let v72: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v71.clone()));
                let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v72.clone()));
                let v74: US0 = US0::US0_CompilerInstall;
                let v75: Rc<str> = class_word_29(v74.clone());
                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v75.clone()));
                let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v73.clone(), v76.clone()));
                let v78: US0 = US0::US0_TreeRead;
                let v79: Rc<str> = class_word_29(v78.clone());
                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v79.clone()));
                let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v77.clone(), v80.clone()));
                let v82: US0 = US0::US0_Bench;
                let v83: Rc<str> = class_word_29(v82.clone());
                let v84: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v83.clone()));
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v81.clone(), v84.clone()));
                let v86: US0 = US0::US0_CompileHold;
                let v87: Rc<str> = class_word_29(v86.clone());
                let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v87.clone()));
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v85.clone(), v88.clone()));
                let v90: US0 = US0::US0_TreeWrite;
                let v91: Rc<str> = class_word_29(v90.clone());
                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v91.clone()));
                let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v89.clone(), v92.clone()));
                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v95: Rc<str> = concat5_2(v57.clone(), v93.clone(), v94.clone(), v94.clone(), v94.clone());
                let v96: Rc<str> = concat5_2(v53.clone(), v55.clone(), v56.clone(), v95.clone(), v94.clone());
                refused_1(v96.clone())
            } else {
                let v98: i32 = 3i32;
                let v99: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v98 as usize).unwrap_or_default());
                let v100: Rc<str> = announce_refusal_41(v99.clone(), v51);
                let v101: bool = &*v100 != "";
                if v101 {
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: "); } LIT.with(|lit| lit.clone()) };
                    let v103: i32 = 3i32;
                    let v104: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v103 as usize).unwrap_or_default());
                    let v105: Rc<str> = announce_refusal_41(v104.clone(), v51);
                    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v107: Rc<str> = concat5_2(v102.clone(), v105.clone(), v106.clone(), v106.clone(), v106.clone());
                    refused_1(v107.clone())
                } else {
                    let v109: u64 = now_ms_7();
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
                    let v111: Rc<str> = number_text_23(v109);
                    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v113: u64 = process_number_42();
                    let v114: Rc<str> = number_text_23(v113);
                    let v115: Rc<str> = class_word_29(v48.clone());
                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v117: Rc<str> = concat5_2(v112.clone(), v115.clone(), v116.clone(), v116.clone(), v116.clone());
                    let v118: Rc<str> = concat5_2(v110.clone(), v111.clone(), v112.clone(), v114.clone(), v117.clone());
                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                    let v120: Rc<str> = option_value_43(v119.clone());
                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
                    let v122: Rc<str> = option_value_43(v121.clone());
                    let v123: bool = &*v122 == "until-granted";
                    let v128: u64 = if v123 {
                        9223372036854775807u64
                    } else {
                        let v124: Rc<str> = option_value_43(v121.clone());
                        let v125: u64 = decimal_of_13(v124.clone());
                        let v126: u64 = v125.wrapping_mul(60000u64);
                        let v127: u64 = v109.wrapping_add(v126);
                        v127
                    };
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = option_value_43(v129.clone());
                    let v131: u64 = decimal_of_13(v130.clone());
                    let v132: bool = false;
                    requested_4(v48.clone(), v51, v120.clone(), v118.clone(), v109, v128, v131, v132)
                }
            }
        }
        US6::US6_ClassUnknown => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v4: i32 = 3i32;
            let v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v4 as usize).unwrap_or_default());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
            let v8: US0 = US0::US0_LiveGui;
            let v9: Rc<str> = class_word_29(v8.clone());
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v9.clone()));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v10.clone()));
            let v12: US0 = US0::US0_TestSuite;
            let v13: Rc<str> = class_word_29(v12.clone());
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v13.clone()));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), v14.clone()));
            let v16: US0 = US0::US0_HeavyBuild;
            let v17: Rc<str> = class_word_29(v16.clone());
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v17.clone()));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15.clone(), v18.clone()));
            let v20: US0 = US0::US0_Compile;
            let v21: Rc<str> = class_word_29(v20.clone());
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v21.clone()));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19.clone(), v22.clone()));
            let v24: US0 = US0::US0_CompilerInstall;
            let v25: Rc<str> = class_word_29(v24.clone());
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v25.clone()));
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), v26.clone()));
            let v28: US0 = US0::US0_TreeRead;
            let mut v29: Rc<str> = class_word_29(v28.clone());
            let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v29.clone()));
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v27.clone(), v30.clone()));
            let v32: US0 = US0::US0_Bench;
            let v33: Rc<str> = class_word_29(v32.clone());
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v33.clone()));
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v31.clone(), v34.clone()));
            let v36: US0 = US0::US0_CompileHold;
            let v37: Rc<str> = class_word_29(v36.clone());
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v37.clone()));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v35.clone(), v38.clone()));
            let v40: US0 = US0::US0_TreeWrite;
            let v41: Rc<str> = class_word_29(v40.clone());
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v41.clone()));
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v39.clone(), v42.clone()));
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = concat5_2(v7.clone(), v43.clone(), v44.clone(), v44.clone(), v44.clone());
            let v46: Rc<str> = concat5_2(v3.clone(), v5.clone(), v6.clone(), v45.clone(), v44.clone());
            refused_1(v46.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_requested_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_request_run() -> i32 {
    closure0()()
}
