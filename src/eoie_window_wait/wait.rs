#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_LiveGui,
    US2_TestSuite,
    US2_HeavyBuild,
    US2_Compile,
    US2_CompilerInstall,
    US2_TreeRead,
    US2_Bench,
    US2_CompileHold,
}
#[derive(Clone)]
enum US1 {
    US1_ClassKnown(US2),
    US1_ClassUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_KeepWaiting,
    US3_ClearNow,
    US3_GaveUp,
}
#[derive(Clone)]
enum US4 {
    US4_ProcessHolder,
    US4_LeaseHolder,
}
#[derive(Clone)]
enum US5 {
    US5_WindowQueued,
    US5_WindowGranted,
    US5_WindowReleased,
    US5_WindowReleasedWaiting,
    US5_WindowGaveUp,
    US5_WindowPruned,
    US5_WindowDropped,
}
#[derive(Clone)]
enum US6 {
    US6_Quiet,
    US6_Contended(u64),
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn option_value_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn method5(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_4(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method5(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let mut v6: US0 = US0::US0_None;
    let mut v7: US0 = v6.clone();
    let mut v8: US0 = v4.unwrap_or(v7);
    match &v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn class_word_8(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US2::US2_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US2::US2_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US2::US2_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_7(mut v0: Rc<str>) -> US1 {
    let mut v1: US2 = US2::US2_LiveGui;
    let mut v2: Rc<str> = class_word_8(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US2 = US2::US2_LiveGui;
        US1::US1_ClassKnown(v4.clone())
    } else {
        let mut v6: US2 = US2::US2_TestSuite;
        let mut v7: Rc<str> = class_word_8(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US2 = US2::US2_TestSuite;
            US1::US1_ClassKnown(v9.clone())
        } else {
            let mut v11: US2 = US2::US2_HeavyBuild;
            let mut v12: Rc<str> = class_word_8(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US2 = US2::US2_HeavyBuild;
                US1::US1_ClassKnown(v14.clone())
            } else {
                let mut v16: US2 = US2::US2_Compile;
                let mut v17: Rc<str> = class_word_8(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US2 = US2::US2_Compile;
                    US1::US1_ClassKnown(v19.clone())
                } else {
                    let mut v21: US2 = US2::US2_CompilerInstall;
                    let mut v22: Rc<str> = class_word_8(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US2 = US2::US2_CompilerInstall;
                        US1::US1_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US2 = US2::US2_TreeRead;
                        let mut v27: Rc<str> = class_word_8(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US2 = US2::US2_TreeRead;
                            US1::US1_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US2 = US2::US2_Bench;
                            let mut v32: Rc<str> = class_word_8(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US2 = US2::US2_Bench;
                                US1::US1_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US2 = US2::US2_CompileHold;
                                let mut v37: Rc<str> = class_word_8(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US2 = US2::US2_CompileHold;
                                    US1::US1_ClassKnown(v39.clone())
                                } else {
                                    US1::US1_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn loop_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let mut v5: US1 = class_of_7(v4.clone());
            let mut v8: bool = match &v5 {
                US1::US1_ClassKnown(v6) => {
                    let mut v6: US2 = v6.clone();
                    false
                }
                US1::US1_ClassUnknown => {
                    true
                }
            };
            if v8 {
                return true;
            } else {
                let mut v9: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v9);
                continue;
            }
        }
    }
}
fn windows_root_10() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn stored_rows_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn now_ms_12() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn first_row_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_16(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn row_expired_15(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_16(v0.clone(), v2);
    let mut v4: u64 = decimal_of_4(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_18(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_16(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_16(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US4::US4_ProcessHolder
    } else {
        US4::US4_LeaseHolder
    }
}
fn window_pid_19(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_20(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_17(mut v0: Rc<str>) -> bool {
    let mut v1: US4 = holder_of_18(v0.clone());
    match &v1 {
        US4::US4_LeaseHolder => {
            false
        }
        US4::US4_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_16(v0.clone(), v2);
            let mut v4: u64 = window_pid_19(v3.clone());
            let mut v5: bool = process_alive_20(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn window_file_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_22(v0.clone(), v4.clone())
}
fn file_removed_23(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn number_text_25(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn event_logged_24(mut v0: Rc<str>, mut v1: US5, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_22(v0.clone(), v3.clone());
    let mut v5: u64 = now_ms_12();
    let mut v6: Rc<str> = number_text_25(v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v1 {
        US5::US5_WindowDropped => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US5::US5_WindowGaveUp => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US5::US5_WindowGranted => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US5::US5_WindowPruned => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US5::US5_WindowQueued => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_WindowReleased => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US5::US5_WindowReleasedWaiting => {
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
fn other_rows_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_14(v1.clone());
            let mut v6: bool = row_expired_15(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_17(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_16(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_21(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_23(v11.clone());
                let mut v13: US5 = US5::US5_WindowPruned;
                event_logged_24(v0.clone(), v13.clone(), v5.clone());
                let mut v14: Rc<str> = other_rows_26(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_26(v1.clone());
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
fn method28(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rows_busy_27(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    loop {
        let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v2 {
            return false;
        } else {
            let mut v3: Rc<str> = first_row_14(v1.clone());
            let mut v4: i32 = 1i32;
            let mut v5: Rc<str> = tab_field_16(v3.clone(), v4);
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v0));
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v6));
            let mut v11: Option<usize> = v10.find(&*v8);
            let mut v12: Option<usize> = method28(v11.clone());
            let mut v13: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v14: Option<i32> = v12.map(|x| v13(x));
            let mut v16: i32 = v14.unwrap_or(-1i32);
            let mut v17: bool = v16 >= 0i32;
            if v17 {
                return true;
            } else {
                let mut v18: Rc<str> = other_rows_26(v1.clone());
                (v0, v1) = (v0.clone(), v18.clone());
                continue;
            }
        }
    }
}
fn waited_9(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64) -> (US3, u64, u64, u64) {
    loop {
        let mut v7: Rc<str> = windows_root_10();
        let mut v8: Rc<str> = stored_rows_11(v7.clone());
        let mut v9: u64 = now_ms_12();
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = live_rows_13(v7.clone(), v8.clone(), v9, v10.clone());
        let mut v12: bool = rows_busy_27(v0.clone(), v11.clone());
        let mut v13: u64 = v4.wrapping_add(1u64);
        let (mut v16, mut v17, mut v18): (u64, u64, u64) = if v12 {
            let mut v14: u64 = v5.wrapping_add(1u64);
            (0u64, v13, v14)
        } else {
            let mut v15: u64 = v3.wrapping_add(1u64);
            (v15, v13, v5)
        };
        let mut v19: bool = v16 >= v1;
        let mut v25: US3 = if v19 {
            US3::US3_ClearNow
        } else {
            let mut v21: bool = v17 >= v2;
            if v21 {
                US3::US3_GaveUp
            } else {
                US3::US3_KeepWaiting
            }
        };
        match &v25 {
            US3::US3_KeepWaiting => {
                std::thread::sleep(std::time::Duration::from_millis(v6));
                (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1, v2, v16, v17, v18, v6);
                continue;
            }
            _ => {
                return (v25.clone(), v16, v17, v18);
            }
        }
    }
}
fn stdout_line_29(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn window_wait_cleared_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--every"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = option_value_3(v2.clone());
    let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: u64 = if v4 {
        30u64
    } else {
        let mut v5: Rc<str> = option_value_3(v2.clone());
        decimal_of_4(v5.clone())
    };
    let mut v8: u64 = v7.wrapping_mul(1000u64);
    let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: bool = if v9 {
        true
    } else {
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v1.starts_with(&*v10);
        v11
    };
    let mut v18: bool = if v12 {
        true
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v15: i32 = (v14.clone().borrow().len() as i32);
        let mut v16: i32 = 0i32;
        loop_6(v14.clone(), v15, v16)
    };
    if v18 {
        let mut v19: US2 = US2::US2_LiveGui;
        let mut v20: Rc<str> = class_word_8(v19.clone());
        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v20.clone()));
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v21.clone()));
        let mut v23: US2 = US2::US2_TestSuite;
        let mut v24: Rc<str> = class_word_8(v23.clone());
        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v24.clone()));
        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22.clone(), v25.clone()));
        let mut v27: US2 = US2::US2_HeavyBuild;
        let mut v28: Rc<str> = class_word_8(v27.clone());
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v28.clone()));
        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26.clone(), v29.clone()));
        let mut v31: US2 = US2::US2_Compile;
        let mut v32: Rc<str> = class_word_8(v31.clone());
        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v32.clone()));
        let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30.clone(), v33.clone()));
        let mut v35: US2 = US2::US2_CompilerInstall;
        let mut v36: Rc<str> = class_word_8(v35.clone());
        let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v36.clone()));
        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v34.clone(), v37.clone()));
        let mut v39: US2 = US2::US2_TreeRead;
        let mut v40: Rc<str> = class_word_8(v39.clone());
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v40.clone()));
        let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38.clone(), v41.clone()));
        let mut v43: US2 = US2::US2_Bench;
        let mut v44: Rc<str> = class_word_8(v43.clone());
        let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v44.clone()));
        let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42.clone(), v45.clone()));
        let mut v47: US2 = US2::US2_CompileHold;
        let mut v48: Rc<str> = class_word_8(v47.clone());
        let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v48.clone()));
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v46.clone(), v49.clone()));
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-clear: give known window classes separated by commas, e.g. window wait-clear live-gui,compiler-install --settle 2 --max 120; classes "); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v50));
        refused_1(v52.clone())
    } else {
        let mut v54: bool = v8 == 0u64;
        if v54 {
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-clear: --every needs a positive number of seconds"); } LIT.with(|lit| lit.clone()) };
            refused_1(v55.clone())
        } else {
            let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--settle"); } LIT.with(|lit| lit.clone()) };
            let mut v58: Rc<str> = option_value_3(v57.clone());
            let mut v59: bool = v58.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v62: u64 = if v59 {
                2u64
            } else {
                let mut v60: Rc<str> = option_value_3(v57.clone());
                decimal_of_4(v60.clone())
            };
            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
            let mut v64: Rc<str> = option_value_3(v63.clone());
            let mut v65: bool = v64.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v68: u64 = if v65 {
                60u64
            } else {
                let mut v66: Rc<str> = option_value_3(v63.clone());
                decimal_of_4(v66.clone())
            };
            let mut v69: u64 = v68.wrapping_mul(60000u64);
            let mut v70: u64 = v69.wrapping_div(v8);
            let mut v71: u64 = v70.wrapping_add(1u64);
            let mut v72: u64 = 0u64;
            let mut v73: u64 = 0u64;
            let mut v74: u64 = 0u64;
            let (mut v75, mut v76, mut v77, mut v78): (US3, u64, u64, u64) = waited_9(v1.clone(), v62, v71, v72, v73, v74, v8);
            let mut v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" classes="); } LIT.with(|lit| lit.clone()) };
            let mut v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v1));
            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" samples="); } LIT.with(|lit| lit.clone()) };
            let mut v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
            let mut v83: Rc<str> = number_text_25(v77);
            let mut v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
            let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v86: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v85));
            let mut v87: bool = v78 == 0u64;
            let mut v90: US6 = if v87 {
                US6::US6_Quiet
            } else {
                US6::US6_Contended(v78)
            };
            let mut v97: Rc<str> = match &v90 {
                US6::US6_Contended(v92) => {
                    let mut v92: u64 = *v92;
                    let mut v93: Rc<str> = number_text_25(v92);
                    let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("contended busy_samples="); } LIT.with(|lit| lit.clone()) };
                    let mut v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v93));
                    v95.clone()
                }
                US6::US6_Quiet => {
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet"); } LIT.with(|lit| lit.clone()) };
                    v91.clone()
                }
            };
            let mut v98: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v97));
            match &v75 {
                US3::US3_ClearNow => {
                    let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window wait-clear ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v98));
                    stdout_line_29(v100.clone());
                    0i32
                }
                _ => {
                    let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window wait-clear gave-up"); } LIT.with(|lit| lit.clone()) };
                    let mut v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v98));
                    stdout_line_29(v102.clone());
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_wait_cleared_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_wait_run() -> i32 {
    closure0()()
}
