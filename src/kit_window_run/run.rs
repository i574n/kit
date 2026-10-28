#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
enum US0 {
    US0_ClassKnown(US1),
    US0_ClassUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_ProcessHolder,
    US3_LeaseHolder,
}
#[derive(Clone)]
enum US4 {
    US4_WindowQueued,
    US4_WindowGranted,
    US4_WindowReleased,
    US4_WindowReleasedWaiting,
    US4_WindowGaveUp,
    US4_WindowPruned,
    US4_WindowDropped,
}
#[derive(Clone)]
enum US5 {
    US5_Granted,
    US5_ReleasedWhileWaiting,
    US5_WaitExpired,
    US5_KeepWaiting,
}
#[derive(Clone)]
enum US6 {
    US6_Held,
    US6_Waiting,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn class_word_4(mut v0: US1) -> Rc<str> {
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
fn class_of_3(mut v0: Rc<str>) -> US0 {
    let v1: US1 = US1::US1_LiveGui;
    let v2: Rc<str> = class_word_4(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US1 = US1::US1_LiveGui;
        US0::US0_ClassKnown(v4)
    } else {
        let mut v6: US1 = US1::US1_TestSuite;
        let v7: Rc<str> = class_word_4(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US1 = US1::US1_TestSuite;
            US0::US0_ClassKnown(v9)
        } else {
            let v11: US1 = US1::US1_HeavyBuild;
            let v12: Rc<str> = class_word_4(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US1 = US1::US1_HeavyBuild;
                US0::US0_ClassKnown(v14)
            } else {
                let v16: US1 = US1::US1_Compile;
                let v17: Rc<str> = class_word_4(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US1 = US1::US1_Compile;
                    US0::US0_ClassKnown(v19)
                } else {
                    let v21: US1 = US1::US1_CompilerInstall;
                    let v22: Rc<str> = class_word_4(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US1 = US1::US1_CompilerInstall;
                        US0::US0_ClassKnown(v24)
                    } else {
                        let mut v26: US1 = US1::US1_TreeRead;
                        let v27: Rc<str> = class_word_4(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let mut v29: US1 = US1::US1_TreeRead;
                            US0::US0_ClassKnown(v29)
                        } else {
                            let v31: US1 = US1::US1_Bench;
                            let v32: Rc<str> = class_word_4(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US1 = US1::US1_Bench;
                                US0::US0_ClassKnown(v34)
                            } else {
                                let v36: US1 = US1::US1_CompileHold;
                                let v37: Rc<str> = class_word_4(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US1 = US1::US1_CompileHold;
                                    US0::US0_ClassKnown(v39)
                                } else {
                                    US0::US0_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_5(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method6(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
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
fn command_after_separator_7() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != "--").skip(1).collect::<Vec<_>>().join(&char::from(31u8).to_string()).as_str());
    v0.clone()
}
fn now_ms_8() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method11(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn number_text_9(mut v0: u64) -> Rc<str> {
    format_real_10(v0)
}
fn process_number_12() -> u64 {
    let mut v0: u64 = std::process::id() as u64;
    v0
}
fn own_option_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().take_while(|arg| arg != "--").skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default().as_str());
    v1.clone()
}
fn command_head_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(31u8)).next().unwrap_or(""));
    v1
}
fn windows_root_16() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn lock_taken_17(mut v0: Rc<str>) -> bool {
    let v1: bool = { let lock = std::path::Path::new(&*v0).join("lock"); let _ = std::fs::create_dir_all(&*v0); let mut taken = false; for _ in 0..400 { if std::fs::create_dir(&lock).is_ok() { taken = true; break; } let stale = std::fs::metadata(&lock).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() > 10).unwrap_or(false); if stale { let _ = std::fs::remove_dir(&lock); } else { std::thread::sleep(std::time::Duration::from_millis(50)); } } taken };
    v1
}
fn stored_rows_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_22(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn row_expired_21(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_22(v0, v2);
    let mut v4: u64 = decimal_of_5(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_24(mut v0: Rc<str>) -> US3 {
    let v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_22(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_22(v0.clone(), v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US3::US3_ProcessHolder
    } else {
        US3::US3_LeaseHolder
    }
}
fn window_pid_25(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_26(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_23(mut v0: Rc<str>) -> bool {
    let v1: US3 = holder_of_24(v0.clone());
    match &v1 {
        US3::US3_LeaseHolder => {
            false
        }
        US3::US3_ProcessHolder => {
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_22(v0, v2);
            let mut v4: u64 = window_pid_25(v3);
            let v5: bool = process_alive_26(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_28(v0.clone(), v4.clone())
}
fn file_removed_29(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn event_logged_30(mut v0: Rc<str>, mut v1: US4, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_28(v0.clone(), v3.clone());
    let v5: u64 = now_ms_8();
    let mut v6: Rc<str> = number_text_9(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = match &v1 {
        US4::US4_WindowDropped => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US4::US4_WindowGaveUp => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US4::US4_WindowGranted => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US4::US4_WindowPruned => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US4::US4_WindowQueued => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US4::US4_WindowReleased => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US4::US4_WindowReleasedWaiting => {
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
fn other_rows_31(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_20(v1.clone());
            let mut v6: bool = row_expired_21(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_23(v5.clone())
            };
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_22(v5.clone(), v9);
                let v11: Rc<str> = window_file_27(v0.clone(), v10.clone());
                let v12: bool = file_removed_29(v11.clone());
                let v13: US4 = US4::US4_WindowPruned;
                event_logged_30(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_31(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let v16: Rc<str> = other_rows_31(v1.clone());
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
fn free_mb_32() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn verdict_text_33(mut v0: US1, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> Rc<str> {
    let v8: Rc<str> = class_word_4(v0);
    let v9: Rc<str> = kit_window_admission::kit_window_verdict(&*v8, &*v1, v2, v3, &*v4, v5, v6, v7);
    v9
}
fn released_mark_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_28(v0.clone(), v4.clone())
}
fn file_exists_35(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn wait_step_of_36(mut v0: bool, mut v1: bool, mut v2: u64, mut v3: u64) -> US5 {
    if v1 {
        US5::US5_ReleasedWhileWaiting
    } else {
        if v0 {
            US5::US5_Granted
        } else {
            let mut v6: bool = v2 >= v3;
            if v6 {
                US5::US5_WaitExpired
            } else {
                US5::US5_KeepWaiting
            }
        }
    }
}
fn one_line_39(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>());
    v1.clone()
}
fn row_text_38(mut v0: Rc<str>, mut v1: US1, mut v2: US6, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = class_word_4(v1.clone());
    let v11: Rc<str> = match &v2 {
        US6::US6_Held => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US6::US6_Waiting => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
    };
    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v6, &*v7, &*v6, &*v11].concat());
    let v13: Rc<str> = one_line_39(v3.clone());
    let v14: Rc<str> = number_text_9(v4);
    let v15: Rc<str> = number_text_9(v5);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v6, &*v15, &*v16, &*v16].concat());
    let v18: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v6, &*v13, &*v6, &*v17].concat());
    v18.clone()
}
fn request_row_37(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: US6, mut v8: u64) -> Rc<str> {
    row_text_38(v3, v0, v7, v2, v4, v8)
}
fn text_written_40(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn lock_released_41(mut v0: Rc<str>) -> () {
    { let _ = std::fs::remove_dir(std::path::Path::new(&*v0).join("lock")); };
    ()
}
fn request_head_42(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window request "); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" class="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = class_word_4(v0.clone());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
    let v14: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v7, &*v9, &*v3, &*v13].concat());
    v14.clone()
}
fn stdout_line_43(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn requested_15(mut v0: US1, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> i32 {
    loop {
        let v8: Rc<str> = windows_root_16();
        let v9: bool = lock_taken_17(v8.clone());
        let v10: bool = v9 == false;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("request: the registry lock stayed taken for 20 s"); } LIT.with(|lit| lit.clone()) };
            return refused_1(v11.clone());
        } else {
            let v13: u64 = now_ms_8();
            let v14: Rc<str> = stored_rows_18(v8.clone());
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = live_rows_19(v8.clone(), v14.clone(), v13, v15.clone());
            let v17: u64 = free_mb_32();
            let v18: Rc<str> = verdict_text_33(v0.clone(), v3.clone(), v4, v1, v16.clone(), v17, v13, v6);
            let v19: bool = &*v18 == "";
            let v20: Rc<str> = released_mark_34(v8.clone(), v3.clone());
            let v21: bool = file_exists_35(v20.clone());
            let v22: US5 = wait_step_of_36(v19, v21, v13, v5);
            match &v22 {
                US5::US5_Granted => {
                    let v36: u64 = v1.wrapping_mul(60000u64);
                    let v37: u64 = v13.wrapping_add(v36);
                    let v38: Rc<str> = window_file_27(v8.clone(), v3.clone());
                    let v39: US6 = US6::US6_Held;
                    let v40: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v39.clone(), v37);
                    let v41: Rc<str> = text_written_40(v38.clone(), v40.clone());
                    let v42: US4 = US4::US4_WindowGranted;
                    let v43: US6 = US6::US6_Held;
                    let v44: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v13, v5, v6, v43.clone(), v37);
                    event_logged_30(v8.clone(), v42.clone(), v44.clone());
                    lock_released_41(v8.clone());
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = request_head_42(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v45.clone());
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = number_text_9(v1);
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = number_text_9(v17);
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v52: u64 = v13.wrapping_sub(v4);
                    let v53: Rc<str> = number_text_9(v52);
                    let v54: Rc<str> = std::rc::Rc::<str>::from([&*v50, &*v51, &*v53, &*v15, &*v15].concat());
                    let v55: Rc<str> = std::rc::Rc::<str>::from([&*v46, &*v47, &*v48, &*v49, &*v54].concat());
                    stdout_line_43(v55.clone());
                    return 0i32;
                }
                US5::US5_KeepWaiting => {
                    let v65: Rc<str> = window_file_27(v8.clone(), v3.clone());
                    let v66: US6 = US6::US6_Waiting;
                    let v67: u64 = v13.wrapping_add(30000u64);
                    let v68: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v66.clone(), v67);
                    let v69: Rc<str> = text_written_40(v65.clone(), v68.clone());
                    if v7 {
                        ()
                    } else {
                        let v70: US4 = US4::US4_WindowQueued;
                        let v71: US6 = US6::US6_Waiting;
                        let v72: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v71.clone(), v67);
                        event_logged_30(v8.clone(), v70.clone(), v72.clone())
                    };
                    lock_released_41(v8.clone());
                    if v7 {
                        ()
                    } else {
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = request_head_42(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v73.clone());
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = std::rc::Rc::<str>::from([&*v74, &*v75, &*v18, &*v15, &*v15].concat());
                        stdout_line_43(v76.clone())
                    };
                    let v77: u64 = 500u64;
                    std::thread::sleep(std::time::Duration::from_millis(v77));
                    let v78: bool = true;
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v78);
                    continue;
                }
                US5::US5_ReleasedWhileWaiting => {
                    let v23: Rc<str> = released_mark_34(v8.clone(), v3.clone());
                    let v24: bool = file_removed_29(v23.clone());
                    let v25: Rc<str> = window_file_27(v8.clone(), v3.clone());
                    let mut v26: bool = file_removed_29(v25.clone());
                    let v27: US4 = US4::US4_WindowReleasedWaiting;
                    let v28: US6 = US6::US6_Waiting;
                    let mut v29: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v28.clone(), v13);
                    event_logged_30(v8.clone(), v27.clone(), v29.clone());
                    lock_released_41(v8.clone());
                    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = request_head_42(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v30.clone());
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" while waiting waited_ms="); } LIT.with(|lit| lit.clone()) };
                    let v33: u64 = v13.wrapping_sub(v4);
                    let v34: Rc<str> = number_text_9(v33);
                    let v35: Rc<str> = std::rc::Rc::<str>::from([&*v31, &*v32, &*v34, &*v15, &*v15].concat());
                    stdout_line_43(v35.clone());
                    return 1i32;
                }
                US5::US5_WaitExpired => {
                    let v56: Rc<str> = window_file_27(v8.clone(), v3.clone());
                    let v57: bool = file_removed_29(v56.clone());
                    let v58: US4 = US4::US4_WindowGaveUp;
                    let v59: US6 = US6::US6_Waiting;
                    let v60: Rc<str> = request_row_37(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v59.clone(), v13);
                    event_logged_30(v8.clone(), v58.clone(), v60.clone());
                    lock_released_41(v8.clone());
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = request_head_42(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5, v6, v61.clone());
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = std::rc::Rc::<str>::from([&*v62, &*v63, &*v18, &*v15, &*v15].concat());
                    stdout_line_43(v64.clone());
                    return 1i32;
                }
            }
        }
    }
}
fn child_status_44(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = { let mut parts = (v0).split(char::from(31u8)); let program = parts.next().unwrap_or_default().to_owned(); match std::process::Command::new(&program).args(parts).env("KIT_WINDOW_HELD", &*v1).status() { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("kit error: window run: cannot start {program}: {error}"); 127 } } };
    v2
}
fn format_real_46(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn exit_text_45(mut v0: i32) -> Rc<str> {
    format_real_46(v0)
}
fn window_ran_0() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: US0 = class_of_3(v1.clone());
    match &v2 {
        US0::US0_ClassKnown(v46) => {
            let v46: US1 = v46.clone();
            let v47: i32 = 4i32;
            let v48: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v47 as usize).unwrap_or_default());
            let v49: u64 = decimal_of_5(v48.clone());
            let v50: Rc<str> = command_after_separator_7();
            let v51: bool = v49 == 0u64;
            if v51 {
                let v52: i32 = 4i32;
                let v53: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v52 as usize).unwrap_or_default());
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: minutes must be a whole number above 0, received '"); } LIT.with(|lit| lit.clone()) };
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v53));
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
                let v59: US1 = US1::US1_LiveGui;
                let v60: Rc<str> = class_word_4(v59.clone());
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v60.clone()));
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v61.clone()));
                let v63: US1 = US1::US1_TestSuite;
                let v64: Rc<str> = class_word_4(v63.clone());
                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v64.clone()));
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v62.clone(), v65.clone()));
                let v67: US1 = US1::US1_HeavyBuild;
                let v68: Rc<str> = class_word_4(v67.clone());
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v68.clone()));
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v66.clone(), v69.clone()));
                let v71: US1 = US1::US1_Compile;
                let v72: Rc<str> = class_word_4(v71.clone());
                let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v72.clone()));
                let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v70.clone(), v73.clone()));
                let v75: US1 = US1::US1_CompilerInstall;
                let v76: Rc<str> = class_word_4(v75.clone());
                let v77: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v76.clone()));
                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v74.clone(), v77.clone()));
                let v79: US1 = US1::US1_TreeRead;
                let v80: Rc<str> = class_word_4(v79.clone());
                let v81: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v80.clone()));
                let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v78.clone(), v81.clone()));
                let v83: US1 = US1::US1_Bench;
                let v84: Rc<str> = class_word_4(v83.clone());
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v84.clone()));
                let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v82.clone(), v85.clone()));
                let v87: US1 = US1::US1_CompileHold;
                let v88: Rc<str> = class_word_4(v87.clone());
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v88.clone()));
                let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v86.clone(), v89.clone()));
                let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v92: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v90, &*v91, &*v91, &*v91].concat());
                let v93: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v92));
                refused_1(v93.clone())
            } else {
                let v95: bool = &*v50 == "";
                if v95 {
                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: give the command after --, e.g. window run compile 5 --wait 10 -- pwsh -File build.ps1"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v96.clone())
                } else {
                    let v98: u64 = now_ms_8();
                    let v99: Rc<str> = number_text_9(v98);
                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
                    let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v99));
                    let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                    let v104: u64 = process_number_12();
                    let v105: Rc<str> = number_text_9(v104);
                    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v105));
                    let v107: Rc<str> = class_word_4(v46.clone());
                    let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v107));
                    let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
                    let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v106, Rc::<str>::from(format!("{}{}", v108, v109))));
                    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--label"); } LIT.with(|lit| lit.clone()) };
                    let v113: Rc<str> = own_option_13(v112.clone());
                    let v114: bool = &*v113 == "";
                    let v117: Rc<str> = if v114 {
                        command_head_14(v50.clone())
                    } else {
                        own_option_13(v112.clone())
                    };
                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--wait"); } LIT.with(|lit| lit.clone()) };
                    let v119: Rc<str> = own_option_13(v118.clone());
                    let v120: u64 = decimal_of_5(v119.clone());
                    let v121: u64 = v120.wrapping_mul(60000u64);
                    let v122: u64 = v98.wrapping_add(v121);
                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
                    let v124: Rc<str> = own_option_13(v123.clone());
                    let v125: u64 = decimal_of_5(v124.clone());
                    let v126: bool = false;
                    let v127: i32 = requested_15(v46.clone(), v49, v117.clone(), v111.clone(), v98, v122, v125, v126);
                    let v128: bool = v127 != 0i32;
                    if v128 {
                        v127
                    } else {
                        let v129: Rc<str> = class_word_4(v46.clone());
                        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v132: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v129, v130)), v111));
                        let v133: i32 = child_status_44(v50.clone(), v132.clone());
                        let v134: Rc<str> = windows_root_16();
                        let v135: Rc<str> = window_file_27(v134.clone(), v111.clone());
                        let v136: bool = file_removed_29(v135.clone());
                        let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window run ok id="); } LIT.with(|lit| lit.clone()) };
                        let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v111));
                        let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
                        let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                        let v141: Rc<str> = exit_text_45(v133);
                        let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
                        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" released=yes"); } LIT.with(|lit| lit.clone()) };
                        let v144: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v143));
                        stdout_line_43(v144.clone());
                        v133
                    }
                }
            }
        }
        US0::US0_ClassUnknown => {
            let mut v3: i32 = 3i32;
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
            let v10: US1 = US1::US1_LiveGui;
            let v11: Rc<str> = class_word_4(v10.clone());
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v11.clone()));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v12.clone()));
            let v14: US1 = US1::US1_TestSuite;
            let v15: Rc<str> = class_word_4(v14.clone());
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v15.clone()));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
            let v18: US1 = US1::US1_HeavyBuild;
            let v19: Rc<str> = class_word_4(v18.clone());
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v19.clone()));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
            let v22: US1 = US1::US1_Compile;
            let v23: Rc<str> = class_word_4(v22.clone());
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v24.clone()));
            let mut v26: US1 = US1::US1_CompilerInstall;
            let v27: Rc<str> = class_word_4(v26.clone());
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v27.clone()));
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v28.clone()));
            let mut v30: US1 = US1::US1_TreeRead;
            let v31: Rc<str> = class_word_4(v30.clone());
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v31.clone()));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), v32.clone()));
            let v34: US1 = US1::US1_Bench;
            let v35: Rc<str> = class_word_4(v34.clone());
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v35.clone()));
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33.clone(), v36.clone()));
            let v38: US1 = US1::US1_CompileHold;
            let v39: Rc<str> = class_word_4(v38.clone());
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v39.clone()));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37.clone(), v40.clone()));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v41, &*v42, &*v42, &*v42].concat());
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v43));
            refused_1(v44.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_ran_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn text_or_empty_49(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn decimal_u64_50(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method6(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
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
fn file_minutes_51(mut v0: Rc<str>) -> u64 {
    let v1: u64 = std::fs::metadata(&*v0).and_then(|meta| meta.modified()).ok().and_then(|time| time.elapsed().ok()).map(|age| age.as_secs() / 60).unwrap_or(0);
    v1
}
fn field_rest_52(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).trim().splitn((v1 as usize) + 1, char::from(9u8)).nth(v1 as usize).unwrap_or("").replace(char::from(9u8), " ").as_str());
    v2
}
fn holder_lines_48(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let mut v3: Rc<str> = first_row_20(v0.clone());
            let mut v4: Rc<str> = text_or_empty_49(v3.clone());
            let v5: i32 = 1i32;
            let mut v6: Rc<str> = tab_field_22(v4.clone(), v5);
            let v7: u64 = decimal_u64_50(v6.clone());
            let v8: bool = process_alive_26(v7);
            let v9: bool = v8 == false;
            if v9 {
                let v10: bool = file_removed_29(v3.clone());
                let v11: Rc<str> = other_rows_31(v0.clone());
                (v0, v1) = (v11.clone(), v1.clone());
                continue;
            } else {
                let v13: i32 = 0i32;
                let v14: Rc<str> = tab_field_22(v4.clone(), v13);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-slot-"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                let v19: i32 = 1i32;
                let v20: Rc<str> = tab_field_22(v4.clone(), v19);
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
                let v22: u64 = file_minutes_51(v3.clone());
                let v23: Rc<str> = number_text_9(v22);
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  held "); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m  "); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
                let v28: i32 = 3i32;
                let mut v29: Rc<str> = field_rest_52(v4.clone(), v28);
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v21, Rc::<str>::from(format!("{}{}", v27, v29))));
                let v32: Rc<str> = other_rows_31(v0.clone());
                let v33: bool = &*v1 == "";
                let v37: Rc<str> = if v33 {
                    v31.clone()
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v34)), v31));
                    v36.clone()
                };
                (v0, v1) = (v32.clone(), v37.clone());
                continue;
            }
        }
    }
}
fn heavy_queue_dir_53() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_QUEUE").or_else(|| std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("HOME")).map(|base| std::path::PathBuf::from(base).join("spiral-bin").join("heavy-queue").into_os_string())).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default().as_str());
    v0.clone()
}
fn holder_paths_54(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let mut paths = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().is_some_and(|ext| ext == "h")).map(|path| path.to_string_lossy().into_owned()).collect::<Vec<_>>(); paths.sort(); paths.join("
") }.as_str());
    v1.clone()
}
fn heavy_holders_text_47() -> Rc<str> {
    let mut v0: Rc<str> = heavy_queue_dir_53();
    let v1: Rc<str> = holder_paths_54(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    holder_lines_48(v1.clone(), v2.clone())
}
fn closure2() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        heavy_holders_text_47()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_run_run() -> i32 {
    closure0()()
}
pub fn kit_heavy_holders_text() -> Rc<str> {
    closure2()()
}
