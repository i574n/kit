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
struct Mut0 { l0: Rc<str> }
fn class_word_3(mut v0: US1) -> Rc<str> {
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
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_2(mut v0: Rc<str>) -> US0 {
    let v1: US1 = US1::US1_LiveGui;
    let v2: Rc<str> = class_word_3(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US1 = US1::US1_LiveGui;
        US0::US0_ClassKnown(v4)
    } else {
        let v6: US1 = US1::US1_TestSuite;
        let v7: Rc<str> = class_word_3(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US1 = US1::US1_TestSuite;
            US0::US0_ClassKnown(v9)
        } else {
            let v11: US1 = US1::US1_HeavyBuild;
            let v12: Rc<str> = class_word_3(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US1 = US1::US1_HeavyBuild;
                US0::US0_ClassKnown(v14)
            } else {
                let v16: US1 = US1::US1_Compile;
                let v17: Rc<str> = class_word_3(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US1 = US1::US1_Compile;
                    US0::US0_ClassKnown(v19)
                } else {
                    let v21: US1 = US1::US1_CompilerInstall;
                    let v22: Rc<str> = class_word_3(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US1 = US1::US1_CompilerInstall;
                        US0::US0_ClassKnown(v24)
                    } else {
                        let mut v26: US1 = US1::US1_TreeRead;
                        let v27: Rc<str> = class_word_3(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let mut v29: US1 = US1::US1_TreeRead;
                            US0::US0_ClassKnown(v29)
                        } else {
                            let v31: US1 = US1::US1_Bench;
                            let v32: Rc<str> = class_word_3(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US1 = US1::US1_Bench;
                                US0::US0_ClassKnown(v34)
                            } else {
                                let v36: US1 = US1::US1_CompileHold;
                                let v37: Rc<str> = class_word_3(v36);
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
fn windows_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn now_ms_5() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn stored_rows_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_10(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_11(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method12(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let v6: US2 = US2::US2_None;
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
fn row_expired_9(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_10(v0, v2);
    let mut v4: u64 = decimal_of_11(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_14(mut v0: Rc<str>) -> US3 {
    let v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_10(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let v6: Rc<str> = tab_field_10(v0.clone(), v5);
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
fn window_pid_15(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_16(mut v0: u64) -> bool {
    let v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_13(mut v0: Rc<str>) -> bool {
    let v1: US3 = holder_of_14(v0.clone());
    match &v1 {
        US3::US3_LeaseHolder => {
            false
        }
        US3::US3_ProcessHolder => {
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_10(v0, v2);
            let mut v4: u64 = window_pid_15(v3);
            let v5: bool = process_alive_16(v4);
            let v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_18(v0.clone(), v4.clone())
}
fn file_removed_19(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method23(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn number_text_21(mut v0: u64) -> Rc<str> {
    format_real_22(v0)
}
fn event_logged_20(mut v0: Rc<str>, mut v1: US4, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_18(v0.clone(), v3.clone());
    let v5: u64 = now_ms_5();
    let v6: Rc<str> = number_text_21(v5);
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
fn other_rows_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_8(v1.clone());
            let v6: bool = row_expired_9(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_13(v5.clone())
            };
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_10(v5.clone(), v9);
                let v11: Rc<str> = window_file_17(v0.clone(), v10.clone());
                let v12: bool = file_removed_19(v11.clone());
                let v13: US4 = US4::US4_WindowPruned;
                event_logged_20(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_24(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let v16: Rc<str> = other_rows_24(v1.clone());
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
fn free_mb_25() -> u64 {
    let mut v0: u64 = { #[cfg(windows)] fn free() -> u64 { #[repr(C)] struct Status { length: u32, load: u32, total: u64, available: u64, total_page: u64, available_page: u64, total_virtual: u64, available_virtual: u64, extended: u64 } unsafe extern "system" { fn GlobalMemoryStatusEx(status: *mut Status) -> i32; } unsafe { let mut status: Status = std::mem::zeroed(); status.length = std::mem::size_of::<Status>() as u32; if GlobalMemoryStatusEx(&mut status) == 0 { u64::MAX } else { status.available / 1048576 } } } #[cfg(not(windows))] fn free() -> u64 { std::fs::read_to_string("/proc/meminfo").ok().and_then(|text| text.lines().find(|line| line.starts_with("MemAvailable:")).and_then(|line| line.split_whitespace().nth(1)).and_then(|kb| kb.parse::<u64>().ok())).map(|kb| kb / 1024).unwrap_or(u64::MAX) } std::env::var("KIT_WINDOWS_FREE_MB").ok().and_then(|text| text.trim().parse::<u64>().ok()).unwrap_or_else(free) };
    v0
}
fn verdict_text_26(mut v0: US1, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64) -> Rc<str> {
    let v8: Rc<str> = class_word_3(v0);
    let v9: Rc<str> = kit_window_admission::kit_window_verdict(&*v8, &*v1, v2, v3, &*v4, v5, v6, v7);
    v9
}
fn class_admitted_1(mut v0: Rc<str>) -> bool {
    let v1: US0 = class_of_2(v0.clone());
    match &v1 {
        US0::US0_ClassKnown(v2) => {
            let v2: US1 = v2.clone();
            let mut v3: Rc<str> = windows_root_4();
            let mut v4: u64 = now_ms_5();
            let v5: Rc<str> = stored_rows_6(v3.clone());
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = live_rows_7(v3.clone(), v5.clone(), v4, v6.clone());
            let v8: u64 = 0u64;
            let v9: u64 = free_mb_25();
            let v10: u64 = 0u64;
            let v11: Rc<str> = verdict_text_26(v2.clone(), v6.clone(), v4, v8, v7.clone(), v9, v4, v10);
            let v12: bool = &*v11 == "";
            v12
        }
        US0::US0_ClassUnknown => {
            false
        }
    }
}
fn fitting_loads_0() -> Rc<str> {
    let mut v0: Rc<str> = kit_window_run::kit_heavy_holders_text();
    let v1: u64 = (v0).lines().count() as u64;
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = class_admitted_1(v2.clone());
    let v6: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",medium"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    };
    let v7: bool = v1 < 2u64;
    let v10: bool = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
        class_admitted_1(v8.clone())
    } else {
        false
    };
    let v13: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",heavy"); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12.clone()
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light,none"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v6)), v13));
    v16.clone()
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        fitting_loads_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn minutes_left_29(mut v0: u64, mut v1: u64) -> u64 {
    let v2: bool = v0 <= v1;
    if v2 {
        0u64
    } else {
        let mut v3: u64 = v0.wrapping_sub(v1);
        let mut v4: u64 = v3.wrapping_add(59999u64);
        let v5: u64 = v4.wrapping_div(60000u64);
        v5
    }
}
fn stdout_line_30(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn shown_rows_28(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> u64 {
    loop {
        let mut v3: bool = &*v0 == "";
        if v3 {
            return v2;
        } else {
            let mut v4: Rc<str> = first_row_8(v0.clone());
            let v5: i32 = 2i32;
            let v6: Rc<str> = tab_field_10(v4.clone(), v5);
            let v7: bool = &*v6 == "waiting";
            let v25: Rc<str> = if v7 {
                let v8: i32 = 4i32;
                let v9: Rc<str> = tab_field_10(v4.clone(), v8);
                let v10: u64 = decimal_of_11(v9.clone());
                let v11: u64 = v1.wrapping_sub(v10);
                let v12: u64 = v11.wrapping_div(60000u64);
                let v13: Rc<str> = number_text_21(v12);
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waited "); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                v17.clone()
            } else {
                let v18: i32 = 5i32;
                let v19: Rc<str> = tab_field_10(v4.clone(), v18);
                let v20: u64 = decimal_of_11(v19.clone());
                let v21: u64 = minutes_left_29(v20, v1);
                let v22: Rc<str> = number_text_21(v21);
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m left"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                v24.clone()
            };
            let mut v26: i32 = 0i32;
            let v27: Rc<str> = tab_field_10(v4.clone(), v26);
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            let mut v30: i32 = 1i32;
            let v31: Rc<str> = tab_field_10(v4.clone(), v30);
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v31)), v28));
            let v34: i32 = 2i32;
            let v35: Rc<str> = tab_field_10(v4.clone(), v34);
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v28)), v25)), v28));
            let v39: i32 = 3i32;
            let v40: Rc<str> = tab_field_10(v4.clone(), v39);
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v33, Rc::<str>::from(format!("{}{}", v38, v40))));
            stdout_line_30(v42.clone());
            let v43: Rc<str> = other_rows_24(v0.clone());
            let v44: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2) = (v43.clone(), v1, v44);
            continue;
        }
    }
}
fn window_listed_27() -> i32 {
    let mut v0: Rc<str> = windows_root_4();
    let v1: u64 = now_ms_5();
    let v2: Rc<str> = stored_rows_6(v0.clone());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = live_rows_7(v0.clone(), v2.clone(), v1, v3.clone());
    let v5: u64 = 0u64;
    let v6: u64 = shown_rows_28(v4.clone(), v1, v5);
    let v7: Rc<str> = kit_window_run::kit_heavy_holders_text();
    let v8: bool = &*v7 == "";
    if v8 {
        ()
    } else {
        stdout_line_30(v7.clone())
    };
    let v9: u64 = (v7).lines().count() as u64;
    let v10: Rc<str> = number_text_21(v6);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window list ok windows="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free_mb="); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: u64 = free_mb_25();
    let v16: Rc<str> = number_text_21(v15);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v16));
    let v18: Rc<str> = number_text_21(v9);
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" heavy_slots="); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17, Rc::<str>::from(format!("{}{}", v19, v18))));
    stdout_line_30(v21.clone());
    0i32
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_listed_27()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stderr_line_33(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_32(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_33(v3.clone());
    2i32
}
fn window_checked_31() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: US0 = class_of_2(v1.clone());
    match &v2 {
        US0::US0_ClassKnown(v46) => {
            let v46: US1 = v46.clone();
            let v47: Rc<str> = windows_root_4();
            let v48: u64 = now_ms_5();
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v50: u64 = 0u64;
            let v51: Rc<str> = stored_rows_6(v47.clone());
            let v52: Rc<str> = live_rows_7(v47.clone(), v51.clone(), v48, v49.clone());
            let v53: u64 = free_mb_25();
            let v54: u64 = 0u64;
            let v55: Rc<str> = verdict_text_26(v46.clone(), v49.clone(), v48, v50, v52.clone(), v53, v48, v54);
            let v56: bool = &*v55 == "";
            if v56 {
                let v57: Rc<str> = class_word_3(v46.clone());
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window check ok class="); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" free"); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                stdout_line_30(v61.clone());
                0i32
            } else {
                let v62: Rc<str> = class_word_3(v46.clone());
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window check blocked class="); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v62));
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v64, v65)), v55));
                stdout_line_30(v67.clone());
                1i32
            }
        }
        US0::US0_ClassUnknown => {
            let mut v3: i32 = 3i32;
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v3 as usize).unwrap_or_default());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check: unknown class '"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...]; classes "); } LIT.with(|lit| lit.clone()) };
            let v10: US1 = US1::US1_LiveGui;
            let v11: Rc<str> = class_word_3(v10.clone());
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v11.clone()));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v12.clone()));
            let v14: US1 = US1::US1_TestSuite;
            let v15: Rc<str> = class_word_3(v14.clone());
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v15.clone()));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), v16.clone()));
            let v18: US1 = US1::US1_HeavyBuild;
            let v19: Rc<str> = class_word_3(v18.clone());
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v19.clone()));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17.clone(), v20.clone()));
            let v22: US1 = US1::US1_Compile;
            let v23: Rc<str> = class_word_3(v22.clone());
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v24.clone()));
            let mut v26: US1 = US1::US1_CompilerInstall;
            let v27: Rc<str> = class_word_3(v26.clone());
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v27.clone()));
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25.clone(), v28.clone()));
            let mut v30: US1 = US1::US1_TreeRead;
            let v31: Rc<str> = class_word_3(v30.clone());
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v31.clone()));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), v32.clone()));
            let v34: US1 = US1::US1_Bench;
            let v35: Rc<str> = class_word_3(v34.clone());
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v35.clone()));
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33.clone(), v36.clone()));
            let v38: US1 = US1::US1_CompileHold;
            let v39: Rc<str> = class_word_3(v38.clone());
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v39.clone()));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37.clone(), v40.clone()));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v41, &*v42, &*v42, &*v42].concat());
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v43));
            refused_32(v44.clone())
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_checked_31()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_fitting_loads() -> Rc<str> {
    closure0()()
}
pub fn kit_window_list_run() -> i32 {
    closure2()()
}
pub fn kit_window_check_run() -> i32 {
    closure3()()
}
