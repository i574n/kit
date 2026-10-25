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
    US1_ProcessHolder,
    US1_LeaseHolder,
}
#[derive(Clone)]
enum US2 {
    US2_WindowQueued,
    US2_WindowGranted,
    US2_WindowReleased,
    US2_WindowReleasedWaiting,
    US2_WindowGaveUp,
    US2_WindowPruned,
    US2_WindowDropped,
}
#[derive(Clone)]
enum US3 {
    US3_StoredRead(Rc<str>, Rc<str>, u64),
    US3_StoredShape(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoLeases,
    UH0_LeaseThen(US3, Rc<UH0>),
}
#[derive(Clone)]
enum US5 {
    US5_LiveGui,
    US5_TestSuite,
    US5_HeavyBuild,
    US5_Compile,
    US5_CompilerInstall,
    US5_TreeRead,
    US5_Bench,
    US5_CompileHold,
}
#[derive(Clone)]
enum US6 {
    US6_Held,
    US6_Waiting,
}
#[derive(Clone)]
enum US4 {
    US4_RowKnown(Rc<str>, US5, US6, Rc<str>, u64, u64),
    US4_RowUnknown,
}
#[derive(Clone)]
enum US7 {
    US7_ClassKnown(US5),
    US7_ClassUnknown,
}
#[derive(Clone)]
enum US8 {
    US8_TreeFree,
    US8_TreeHeld(Rc<str>, Rc<str>, u64),
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>, Rc<str>, u64),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_EditAllowed,
    US10_EditRefused(Rc<str>, Rc<str>, u64),
}
struct Mut0 { l0: Rc<str> }
fn stderr_line_1(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn now_ms_2() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn windows_root_3() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn stored_rows_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1.clone()
}
fn tab_field_8(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_9(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method10(v1.clone());
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
fn row_expired_7(mut v0: Rc<str>, mut v1: u64) -> bool {
    let mut v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_8(v0.clone(), v2);
    let mut v4: u64 = decimal_of_9(v3.clone());
    let mut v5: bool = v4 < v1;
    v5
}
fn holder_of_12(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = tab_field_8(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(eoie job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let mut v9: bool = if v4 {
        true
    } else {
        let mut v5: i32 = 0i32;
        let mut v6: Rc<str> = tab_field_8(v0.clone(), v5);
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let mut v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US1::US1_ProcessHolder
    } else {
        US1::US1_LeaseHolder
    }
}
fn window_pid_13(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_14(mut v0: u64) -> bool {
    let mut v1: bool = { #[cfg(windows)] fn alive(pid: u64) -> bool { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; fn GetLastError() -> u32; } if pid == 0 { return true; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid as u32); if handle.is_null() { return GetLastError() == 5; } let running = WaitForSingleObject(handle, 0) == 258; CloseHandle(handle); running } } #[cfg(not(windows))] fn alive(pid: u64) -> bool { pid == 0 || std::os::unix::process::parent_id() as u64 == pid } alive(v0) };
    v1
}
fn orphaned_window_11(mut v0: Rc<str>) -> bool {
    let mut v1: US1 = holder_of_12(v0.clone());
    match &v1 {
        US1::US1_LeaseHolder => {
            false
        }
        US1::US1_ProcessHolder => {
            let mut v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_8(v0.clone(), v2);
            let mut v4: u64 = window_pid_13(v3.clone());
            let mut v5: bool = process_alive_14(v4);
            let mut v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn window_file_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_16(v0.clone(), v4.clone())
}
fn file_removed_17(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn number_text_19(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn event_logged_18(mut v0: Rc<str>, mut v1: US2, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_16(v0.clone(), v3.clone());
    let mut v5: u64 = now_ms_2();
    let mut v6: Rc<str> = number_text_19(v5);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = match &v1 {
        US2::US2_WindowDropped => {
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        }
        US2::US2_WindowGaveUp => {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        }
        US2::US2_WindowGranted => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US2::US2_WindowPruned => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US2::US2_WindowQueued => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_WindowReleased => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US2::US2_WindowReleasedWaiting => {
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
fn other_rows_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1.clone()
}
fn live_rows_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = first_row_6(v1.clone());
            let mut v6: bool = row_expired_7(v5.clone(), v2);
            let mut v8: bool = if v6 {
                true
            } else {
                orphaned_window_11(v5.clone())
            };
            if v8 {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<str> = tab_field_8(v5.clone(), v9);
                let mut v11: Rc<str> = window_file_15(v0.clone(), v10.clone());
                let mut v12: bool = file_removed_17(v11.clone());
                let mut v13: US2 = US2::US2_WindowPruned;
                event_logged_18(v0.clone(), v13.clone(), v5.clone());
                let mut v14: Rc<str> = other_rows_20(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2, v3.clone());
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_20(v1.clone());
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
fn class_word_24(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_Bench => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US5::US5_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_CompileHold => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US5::US5_CompilerInstall => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US5::US5_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US5::US5_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US5::US5_TestSuite => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US5::US5_TreeRead => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
    }
}
fn class_of_23(mut v0: Rc<str>) -> US7 {
    let mut v1: US5 = US5::US5_LiveGui;
    let mut v2: Rc<str> = class_word_24(v1.clone());
    let mut v3: bool = v0.clone() == v2.clone();
    if v3 {
        let mut v4: US5 = US5::US5_LiveGui;
        US7::US7_ClassKnown(v4.clone())
    } else {
        let mut v6: US5 = US5::US5_TestSuite;
        let mut v7: Rc<str> = class_word_24(v6.clone());
        let mut v8: bool = v0.clone() == v7.clone();
        if v8 {
            let mut v9: US5 = US5::US5_TestSuite;
            US7::US7_ClassKnown(v9.clone())
        } else {
            let mut v11: US5 = US5::US5_HeavyBuild;
            let mut v12: Rc<str> = class_word_24(v11.clone());
            let mut v13: bool = v0.clone() == v12.clone();
            if v13 {
                let mut v14: US5 = US5::US5_HeavyBuild;
                US7::US7_ClassKnown(v14.clone())
            } else {
                let mut v16: US5 = US5::US5_Compile;
                let mut v17: Rc<str> = class_word_24(v16.clone());
                let mut v18: bool = v0.clone() == v17.clone();
                if v18 {
                    let mut v19: US5 = US5::US5_Compile;
                    US7::US7_ClassKnown(v19.clone())
                } else {
                    let mut v21: US5 = US5::US5_CompilerInstall;
                    let mut v22: Rc<str> = class_word_24(v21.clone());
                    let mut v23: bool = v0.clone() == v22.clone();
                    if v23 {
                        let mut v24: US5 = US5::US5_CompilerInstall;
                        US7::US7_ClassKnown(v24.clone())
                    } else {
                        let mut v26: US5 = US5::US5_TreeRead;
                        let mut v27: Rc<str> = class_word_24(v26.clone());
                        let mut v28: bool = v0.clone() == v27.clone();
                        if v28 {
                            let mut v29: US5 = US5::US5_TreeRead;
                            US7::US7_ClassKnown(v29.clone())
                        } else {
                            let mut v31: US5 = US5::US5_Bench;
                            let mut v32: Rc<str> = class_word_24(v31.clone());
                            let mut v33: bool = v0.clone() == v32.clone();
                            if v33 {
                                let mut v34: US5 = US5::US5_Bench;
                                US7::US7_ClassKnown(v34.clone())
                            } else {
                                let mut v36: US5 = US5::US5_CompileHold;
                                let mut v37: Rc<str> = class_word_24(v36.clone());
                                let mut v38: bool = v0.clone() == v37.clone();
                                if v38 {
                                    let mut v39: US5 = US5::US5_CompileHold;
                                    US7::US7_ClassKnown(v39.clone())
                                } else {
                                    US7::US7_ClassUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn row_of_22(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = tab_field_8(v0.clone(), v1);
    let mut v3: US7 = class_of_23(v2.clone());
    match &v3 {
        US7::US7_ClassKnown(v5) => {
            let mut v5: US5 = v5.clone();
            let mut v6: i32 = 2i32;
            let mut v7: Rc<str> = tab_field_8(v0.clone(), v6);
            let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("held"); } LIT.with(|lit| lit.clone()) };
            let mut v11: US6 = if v8 {
                US6::US6_Held
            } else {
                US6::US6_Waiting
            };
            let mut v12: i32 = 0i32;
            let mut v13: Rc<str> = tab_field_8(v0.clone(), v12);
            let mut v14: i32 = 3i32;
            let mut v15: Rc<str> = tab_field_8(v0.clone(), v14);
            let mut v16: i32 = 4i32;
            let mut v17: Rc<str> = tab_field_8(v0.clone(), v16);
            let mut v18: u64 = decimal_of_9(v17.clone());
            let mut v19: i32 = 5i32;
            let mut v20: Rc<str> = tab_field_8(v0.clone(), v19);
            let mut v21: u64 = decimal_of_9(v20.clone());
            US4::US4_RowKnown(v13.clone(), v5.clone(), v11.clone(), v15.clone(), v18, v21)
        }
        US7::US7_ClassUnknown => {
            US4::US4_RowUnknown
        }
    }
}
fn leases_of_rows_21(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let mut v2: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = other_rows_20(v0.clone());
        let mut v4: Rc<UH0> = leases_of_rows_21(v3.clone(), v1.clone());
        let mut v5: Rc<str> = first_row_6(v0.clone());
        let mut v6: US4 = row_of_22(v5.clone());
        match &v6 {
            US4::US4_RowKnown(v7, v8, v9, v10, v11, v12) => {
                let mut v7: Rc<str> = v7.clone();
                let mut v8: US5 = v8.clone();
                let mut v9: US6 = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let mut v11: u64 = *v11;
                let mut v12: u64 = *v12;
                let mut v13: bool = match &v8 {
                    US5::US5_TreeRead => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v13 {
                    let mut v14: US3 = US3::US3_StoredRead(v10.clone(), v7.clone(), v12);
                    Rc::new(UH0::UH0_LeaseThen(v14.clone(), v4.clone()))
                } else {
                    v4.clone()
                }
            }
            US4::US4_RowUnknown => {
                v4.clone()
            }
        }
    }
}
fn claim_of_25(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> US8 {
    loop {
        match &*v0 {
            UH0::UH0_LeaseThen(v4, v5) => {
                let mut v4: US3 = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v15: US9 = match &v4 {
                    US3::US3_StoredRead(v6, v7, v8) => {
                        let mut v6: Rc<str> = v6.clone();
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: u64 = *v8;
                        US9::US9_Some(v6.clone(), v7.clone(), v8)
                    }
                    US3::US3_StoredShape(v10, v11, v12) => {
                        let mut v10: Rc<str> = v10.clone();
                        let mut v11: Rc<str> = v11.clone();
                        let mut v12: u64 = *v12;
                        US9::US9_None
                    }
                };
                match &v15 {
                    US9::US9_None => {
                        (v0, v1, v2) = (v5.clone(), v1.clone(), v2);
                        continue;
                    }
                    US9::US9_Some(v16, v17, v18) => {
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: Rc<str> = v17.clone();
                        let mut v18: u64 = *v18;
                        let mut v19: bool = v2 < v18;
                        let mut v38: bool = if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let mut v22: Rc<str> = Rc::<str>::from(v1.replace(&*v20, &*v21));
                            let mut v23: Rc<str> = Rc::<str>::from(v22.to_lowercase());
                            let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v21));
                            let mut v25: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                            let mut v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                            let mut v27: bool = v26.ends_with(&*v21);
                            let mut v29: Rc<str> = if v27 {
                                v26.clone()
                            } else {
                                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v21));
                                v28.clone()
                            };
                            let mut v30: bool = v24.clone() == v29.clone();
                            if v30 {
                                true
                            } else {
                                let mut v31: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                                let mut v32: Rc<str> = Rc::<str>::from(v31.to_lowercase());
                                let mut v33: bool = v32.ends_with(&*v21);
                                let mut v35: Rc<str> = if v33 {
                                    v32.clone()
                                } else {
                                    let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v21));
                                    v34.clone()
                                };
                                let mut v36: bool = v23.starts_with(&*v35);
                                v36
                            }
                        } else {
                            false
                        };
                        if v38 {
                            return US8::US8_TreeHeld(v16.clone(), v17.clone(), v18);
                        } else {
                            (v0, v1, v2) = (v5.clone(), v1.clone(), v2);
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_NoLeases => {
                return US8::US8_TreeFree;
            }
        }
    }
}
fn stdout_line_26(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn method28(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method28(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn window_claim_run_0() -> i32 {
    let mut v0: i32 = 3i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: window claim <path> (refused while a tree-read window covers the path)"); } LIT.with(|lit| lit.clone()) };
        stderr_line_1(v3.clone());
        2i32
    } else {
        let mut v4: u64 = now_ms_2();
        let mut v5: Rc<str> = windows_root_3();
        let mut v6: Rc<str> = stored_rows_4(v5.clone());
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = live_rows_5(v5.clone(), v6.clone(), v4, v7.clone());
        let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoLeases); } CASE.with(|case| case.clone()) };
        let mut v10: Rc<UH0> = leases_of_rows_21(v8.clone(), v9.clone());
        let mut v11: US8 = claim_of_25(v10.clone(), v1.clone(), v4);
        let mut v18: US10 = match &v11 {
            US8::US8_TreeFree => {
                US10::US10_EditAllowed
            }
            US8::US8_TreeHeld(v13, v14, v15) => {
                let mut v13: Rc<str> = v13.clone();
                let mut v14: Rc<str> = v14.clone();
                let mut v15: u64 = *v15;
                US10::US10_EditRefused(v13.clone(), v14.clone(), v15)
            }
        };
        match &v18 {
            US10::US10_EditAllowed => {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window claim ok path="); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v1));
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit allowed"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                stdout_line_26(v24.clone());
                0i32
            }
            US10::US10_EditRefused(v25, v26, v27) => {
                let mut v25: Rc<str> = v25.clone();
                let mut v26: Rc<str> = v26.clone();
                let mut v27: u64 = *v27;
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie window claim refused path="); } LIT.with(|lit| lit.clone()) };
                let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v1));
                let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit refused: "); } LIT.with(|lit| lit.clone()) };
                let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v25));
                let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is read by "); } LIT.with(|lit| lit.clone()) };
                let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v26));
                let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" for "); } LIT.with(|lit| lit.clone()) };
                let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                let mut v39: u64 = v27.wrapping_sub(v4);
                let mut v40: u64 = v39.wrapping_add(59999u64);
                let mut v41: u64 = v40.wrapping_div(60000u64);
                let mut v42: Rc<str> = format_real_27(v41);
                let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v42));
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" more min"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v45));
                stdout_line_26(v46.clone());
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_claim_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_window_claim_run() -> i32 {
    closure0()()
}
