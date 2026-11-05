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
struct Mut0 { l0: Rc<str> }
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
    US5_TreeWrite,
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
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US11 {
    US11_Some(Rc<str>),
    US11_None,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn now_ms_3() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn windows_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn stored_rows_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn first_row_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn tab_field_9(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_10(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method11(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let v6: US0 = US0::US0_None;
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
fn row_expired_8(mut v0: Rc<str>, mut v1: u64) -> bool {
    let v2: i32 = 5i32;
    let mut v3: Rc<str> = tab_field_9(v0, v2);
    let mut v4: u64 = decimal_of_10(v3);
    let v5: bool = v4 < v1;
    v5
}
fn holder_of_13(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = 3i32;
    let v2: Rc<str> = tab_field_9(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v2.contains(&*v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: i32 = 0i32;
        let v6: Rc<str> = tab_field_9(v0, v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-run"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v6.ends_with(&*v7);
        v8
    };
    if v9 {
        US1::US1_ProcessHolder
    } else {
        US1::US1_LeaseHolder
    }
}
fn window_pid_14(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).split(char::from(45u8)).nth(2).and_then(|pid| pid.parse::<u64>().ok()).unwrap_or(0);
    v1
}
fn process_alive_15(mut v0: u64) -> bool {
    let mut v1: bool = kit_process_alive::kit_process_alive(v0);
    v1
}
fn orphaned_window_12(mut v0: Rc<str>) -> bool {
    let mut v1: US1 = holder_of_13(v0.clone());
    match &v1 {
        US1::US1_LeaseHolder => {
            false
        }
        US1::US1_ProcessHolder => {
            let v2: i32 = 0i32;
            let mut v3: Rc<str> = tab_field_9(v0, v2);
            let mut v4: u64 = window_pid_14(v3);
            let v5: bool = process_alive_15(v4);
            let v6: bool = v5 == false;
            v6
        }
    }
}
fn joined_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn concat5_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn window_file_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = concat5_18(v1, v2, v3.clone(), v3.clone(), v3);
    joined_17(v0, v4)
}
fn file_removed_19(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn method23(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_21(mut v0: u64) -> Rc<str> {
    format_real_22(v0)
}
fn window_event_word_24(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_WindowDropped => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_WindowGaveUp => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_WindowGranted => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_WindowPruned => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_WindowQueued => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_WindowReleased => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_WindowReleasedWaiting => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn event_logged_20(mut v0: Rc<str>, mut v1: US2, mut v2: Rc<str>) -> () {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = joined_17(v0, v3);
    let v5: u64 = now_ms_3();
    let v6: Rc<str> = number_text_21(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = window_event_word_24(v1);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_18(v2, v9, v10.clone(), v10.clone(), v10);
    let v12: Rc<str> = concat5_18(v6, v7.clone(), v8, v7, v11);
    let mut v13: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v4);
    let v14: bool = v13.is_ok();
    let v15: bool = v14 == false;
    if v15 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v16: std::fs::File = v13.unwrap();
    let mut v17: std::io::Result<()> = std::io::Write::write_all(&mut v16, (v12).as_bytes());
    let v18: bool = v17.is_ok();
    let v19: bool = v18 == false;
    if v19 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    }
}
fn other_rows_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn live_rows_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v1 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = first_row_7(v1.clone());
            let v6: bool = row_expired_8(v5.clone(), v2);
            let v8: bool = if v6 {
                true
            } else {
                orphaned_window_12(v5.clone())
            };
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_9(v5.clone(), v9);
                let v11: Rc<str> = window_file_16(v0.clone(), v10.clone());
                let v12: bool = file_removed_19(v11.clone());
                let mut v13: US2 = US2::US2_WindowPruned;
                event_logged_20(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_25(v1.clone());
                v1 = v14.clone();
                continue;
            } else {
                let mut v16: Rc<str> = other_rows_25(v1.clone());
                let mut v17: bool = &*v3 == "";
                let v21: Rc<str> = if v17 {
                    v5.clone()
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_18(v3.clone(), v18.clone(), v5.clone(), v19.clone(), v19.clone())
                };
                (v1, v3) = (v16.clone(), v21.clone());
                continue;
            }
        }
    }
}
fn class_word_29(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US5::US5_Compile => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US5::US5_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US5::US5_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US5::US5_HeavyBuild => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US5::US5_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US5::US5_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US5::US5_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US5::US5_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn class_of_28(mut v0: Rc<str>) -> US7 {
    let mut v1: US5 = US5::US5_LiveGui;
    let v2: Rc<str> = class_word_29(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US5 = US5::US5_LiveGui;
        US7::US7_ClassKnown(v4)
    } else {
        let v6: US5 = US5::US5_TestSuite;
        let v7: Rc<str> = class_word_29(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US5 = US5::US5_TestSuite;
            US7::US7_ClassKnown(v9)
        } else {
            let v11: US5 = US5::US5_HeavyBuild;
            let v12: Rc<str> = class_word_29(v11);
            let mut v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US5 = US5::US5_HeavyBuild;
                US7::US7_ClassKnown(v14)
            } else {
                let mut v16: US5 = US5::US5_Compile;
                let mut v17: Rc<str> = class_word_29(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US5 = US5::US5_Compile;
                    US7::US7_ClassKnown(v19)
                } else {
                    let v21: US5 = US5::US5_CompilerInstall;
                    let v22: Rc<str> = class_word_29(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US5 = US5::US5_CompilerInstall;
                        US7::US7_ClassKnown(v24)
                    } else {
                        let v26: US5 = US5::US5_TreeRead;
                        let v27: Rc<str> = class_word_29(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US5 = US5::US5_TreeRead;
                            US7::US7_ClassKnown(v29)
                        } else {
                            let v31: US5 = US5::US5_Bench;
                            let v32: Rc<str> = class_word_29(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US5 = US5::US5_Bench;
                                US7::US7_ClassKnown(v34)
                            } else {
                                let v36: US5 = US5::US5_CompileHold;
                                let v37: Rc<str> = class_word_29(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US5 = US5::US5_CompileHold;
                                    US7::US7_ClassKnown(v39)
                                } else {
                                    let v41: US5 = US5::US5_TreeWrite;
                                    let v42: Rc<str> = class_word_29(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US5 = US5::US5_TreeWrite;
                                        US7::US7_ClassKnown(v44)
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
}
fn row_of_27(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 1i32;
    let v2: Rc<str> = tab_field_9(v0.clone(), v1);
    let mut v3: US7 = class_of_28(v2);
    match &v3 {
        US7::US7_ClassKnown(v5) => {
            let v5: US5 = v5.clone();
            let v6: i32 = 2i32;
            let v7: Rc<str> = tab_field_9(v0.clone(), v6);
            let v8: bool = &*v7 == "held";
            let v11: US6 = if v8 {
                US6::US6_Held
            } else {
                US6::US6_Waiting
            };
            let v12: i32 = 0i32;
            let mut v13: Rc<str> = tab_field_9(v0.clone(), v12);
            let v14: i32 = 3i32;
            let v15: Rc<str> = tab_field_9(v0.clone(), v14);
            let mut v16: i32 = 4i32;
            let mut v17: Rc<str> = tab_field_9(v0.clone(), v16);
            let v18: u64 = decimal_of_10(v17);
            let v19: i32 = 5i32;
            let v20: Rc<str> = tab_field_9(v0, v19);
            let v21: u64 = decimal_of_10(v20);
            US4::US4_RowKnown(v13, v5, v11, v15, v18, v21)
        }
        US7::US7_ClassUnknown => {
            US4::US4_RowUnknown
        }
    }
}
fn leases_of_rows_26(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = other_rows_25(v0.clone());
        let mut v4: Rc<UH0> = leases_of_rows_26(v3, v1);
        let v5: Rc<str> = first_row_7(v0);
        let v6: US4 = row_of_27(v5);
        match &v6 {
            US4::US4_RowKnown(v7, v8, v9, v10, v11, v12) => {
                let v7: Rc<str> = v7.clone();
                let v8: US5 = v8.clone();
                let v9: US6 = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: u64 = *v11;
                let v12: u64 = *v12;
                let mut v13: bool = match &v8 {
                    US5::US5_TreeRead => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v13 {
                    let v14: US3 = US3::US3_StoredRead(v10, v7, v12);
                    Rc::new(UH0::UH0_LeaseThen(v14, v4.clone()))
                } else {
                    v4.clone()
                }
            }
            US4::US4_RowUnknown => {
                v4
            }
        }
    }
}
fn claim_of_30(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> US8 {
    loop {
        match &*v0 {
            UH0::UH0_LeaseThen(v4, v5) => {
                let mut v4: US3 = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v15: US9 = match &v4 {
                    US3::US3_StoredRead(v6, v7, v8) => {
                        let v6: Rc<str> = v6.clone();
                        let v7: Rc<str> = v7.clone();
                        let v8: u64 = *v8;
                        US9::US9_Some(v6.clone(), v7.clone(), v8)
                    }
                    US3::US3_StoredShape(v10, v11, v12) => {
                        let v10: Rc<str> = v10.clone();
                        let v11: Rc<str> = v11.clone();
                        let v12: u64 = *v12;
                        US9::US9_None
                    }
                };
                match &v15 {
                    US9::US9_None => {
                        v0 = v5.clone();
                        continue;
                    }
                    US9::US9_Some(v16, v17, v18) => {
                        let mut v16: Rc<str> = v16.clone();
                        let mut v17: Rc<str> = v17.clone();
                        let v18: u64 = *v18;
                        let v19: bool = v2 < v18;
                        let v38: bool = if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v22: Rc<str> = Rc::<str>::from(v1.replace(&*v20, &*v21));
                            let v23: Rc<str> = Rc::<str>::from(v22.to_lowercase());
                            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v21));
                            let v25: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                            let v26: Rc<str> = Rc::<str>::from(v25.to_lowercase());
                            let v27: bool = v26.ends_with(&*v21);
                            let v29: Rc<str> = if v27 {
                                v26.clone()
                            } else {
                                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v21));
                                v28.clone()
                            };
                            let v30: bool = &*v24 == &*v29;
                            if v30 {
                                true
                            } else {
                                let v31: Rc<str> = Rc::<str>::from(v16.replace(&*v20, &*v21));
                                let v32: Rc<str> = Rc::<str>::from(v31.to_lowercase());
                                let v33: bool = v32.ends_with(&*v21);
                                let v35: Rc<str> = if v33 {
                                    v32.clone()
                                } else {
                                    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v21));
                                    v34.clone()
                                };
                                let v36: bool = v23.starts_with(&*v35);
                                v36
                            }
                        } else {
                            false
                        };
                        if v38 {
                            return US8::US8_TreeHeld(v16.clone(), v17.clone(), v18);
                        } else {
                            v0 = v5.clone();
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
fn stdout_line_31(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn window_claimed_1(mut v0: Rc<str>) -> i32 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window claim <path> (refused while a tree-read window covers the path)"); } LIT.with(|lit| lit.clone()) };
        stderr_line_2(v2.clone());
        2i32
    } else {
        let mut v3: u64 = now_ms_3();
        let mut v4: Rc<str> = windows_root_4();
        let v5: Rc<str> = stored_rows_5(v4.clone());
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = live_rows_6(v4.clone(), v5.clone(), v3, v6.clone());
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoLeases); } CASE.with(|case| case.clone()) };
        let v9: Rc<UH0> = leases_of_rows_26(v7.clone(), v8.clone());
        let v10: US8 = claim_of_30(v9.clone(), v0.clone(), v3);
        let mut v17: US10 = match &v10 {
            US8::US8_TreeFree => {
                US10::US10_EditAllowed
            }
            US8::US8_TreeHeld(v12, v13, v14) => {
                let v12: Rc<str> = v12.clone();
                let mut v13: Rc<str> = v13.clone();
                let v14: u64 = *v14;
                US10::US10_EditRefused(v12.clone(), v13.clone(), v14)
            }
        };
        match &v17 {
            US10::US10_EditAllowed => {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window claim ok path="); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v0));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit allowed"); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                stdout_line_31(v23.clone());
                0i32
            }
            US10::US10_EditRefused(v24, v25, v26) => {
                let v24: Rc<str> = v24.clone();
                let v25: Rc<str> = v25.clone();
                let v26: u64 = *v26;
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window claim refused path="); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v0));
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit refused: "); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v24));
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is read by "); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v33)), v25));
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" for "); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                let v38: u64 = v26.wrapping_sub(v3);
                let v39: u64 = v38.wrapping_add(59999u64);
                let v40: u64 = v39.wrapping_div(60000u64);
                let v41: Rc<str> = format_real_22(v40);
                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v41));
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" more min"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v30, Rc::<str>::from(format!("{}{}", v42, v43))));
                stdout_line_31(v45.clone());
                1i32
            }
        }
    }
}
fn command_line_argument_count_33() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_34(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_32(mut v0: i32) -> Rc<UH1> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_33();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US11 = if v4 {
        let v5: Rc<str> = command_line_argument_34(v0);
        US11::US11_Some(v5)
    } else {
        US11::US11_None
    };
    match &v8 {
        US11::US11_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US11::US11_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH1> = arguments_from_32(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn claim_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH1> = arguments_from_32(v0);
    match &*v1 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH1> = v3.clone();
            match &*v3 {
                UH1::UH1_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let v5: Rc<UH1> = v5.clone();
                    match &*v5 {
                        UH1::UH1_Cons(v6, v7) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<UH1> = v7.clone();
                            window_claimed_1(v6)
                        }
                        _ => {
                            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            window_claimed_1(v9)
                        }
                    }
                }
                _ => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    window_claimed_1(v12)
                }
            }
        }
        _ => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            window_claimed_1(v15)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        claim_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_claim_run() -> i32 {
    closure0()()
}
