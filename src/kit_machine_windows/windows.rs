#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WindowQueued,
    US0_WindowGranted,
    US0_WindowReleased,
    US0_WindowReleasedWaiting,
    US0_WindowGaveUp,
    US0_WindowPruned,
    US0_WindowDropped,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_VerbRequest,
    US1_VerbRelease,
    US1_VerbList,
    US1_VerbCheck,
    US1_VerbRun,
    US1_VerbClaim,
    US1_VerbWaitClear,
    US1_VerbAnnounce,
    US1_VerbRemote,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US1),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_LiveGui,
    US4_TestSuite,
    US4_HeavyBuild,
    US4_Compile,
    US4_CompilerInstall,
    US4_TreeRead,
    US4_Bench,
    US4_CompileHold,
    US4_TreeWrite,
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
    let v3: Rc<str> = concat5_2(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_3(v3);
    2i32
}
fn windows_root_6() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn announce_verb_5() -> i32 {
    let mut v0: Rc<str> = windows_root_6();
    let mut v1: i32 = kit_window_announce_verb::kit_window_announce_run(&*v0);
    v1
}
fn stdout_line_8(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn listed_with_announced_7() -> i32 {
    let mut v0: Rc<str> = windows_root_6();
    let mut v1: Rc<str> = kit_window_announce_check::kit_window_announced_lines(&*v0);
    let v2: bool = &*v1 == "";
    if v2 {
        ()
    } else {
        stdout_line_8(v1)
    };
    let v3: i32 = kit_window_query::kit_window_list_run();
    v3
}
fn joined_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = concat5_2(v1, v2, v3.clone(), v3.clone(), v3);
    joined_11(v0, v4)
}
fn text_or_empty_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn file_removed_13(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn tab_field_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method19(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_17(mut v0: u64) -> Rc<str> {
    format_real_18(v0)
}
fn window_event_word_20(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_WindowDropped => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US0::US0_WindowGaveUp => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_WindowGranted => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_WindowPruned => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US0::US0_WindowQueued => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_WindowReleased => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_WindowReleasedWaiting => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn event_logged_15(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>) -> () {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = joined_11(v0, v3);
    let v5: u64 = now_ms_16();
    let v6: Rc<str> = number_text_17(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = window_event_word_20(v1);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_2(v2, v9, v10.clone(), v10.clone(), v10);
    let v12: Rc<str> = concat5_2(v6, v7.clone(), v8, v7, v11);
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
fn released_mark_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = concat5_2(v1, v2, v3.clone(), v3.clone(), v3);
    joined_11(v0, v4)
}
fn text_written_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn window_released_9(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = windows_root_6();
    let v2: Rc<str> = window_file_10(v1.clone(), v0.clone());
    let v3: Rc<str> = text_or_empty_12(v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v0.starts_with(&*v4);
    let v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = concat5_2(v7, v0.clone(), v8, v9.clone(), v9);
        refused_1(v10)
    } else {
        let v12: Rc<str> = window_file_10(v1.clone(), v0.clone());
        let mut v13: bool = file_removed_13(v12);
        if v13 {
            let v14: i32 = 2i32;
            let v15: Rc<str> = tab_field_14(v3.clone(), v14);
            let mut v16: bool = &*v15 == "waiting";
            let mut v17: US0 = US0::US0_WindowReleased;
            event_logged_15(v1.clone(), v17, v3);
            let v22: Rc<str> = if v16 {
                let v18: Rc<str> = released_mark_21(v1, v0.clone());
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                text_written_22(v18, v19)
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v21
            };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window release ok id="); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = if v16 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" requester=told"); } LIT.with(|lit| lit.clone()) };
                v24
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v25
            };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = concat5_2(v23, v0.clone(), v26, v27.clone(), v27);
            stdout_line_8(v28);
            0i32
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; it was released or expired"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = concat5_2(v29, v0, v30, v31.clone(), v31);
            refused_1(v32)
        }
    }
}
fn remote_status_23(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = kit_remote_status::kit_remote_status_run(&*v0, &*v1);
    v2
}
fn first_word_24(mut v0: Rc<UH0>) -> Rc<str> {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            v1
        }
        UH0::UH0_Nil => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn option_value_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn verb_ran_4(mut v0: US1, mut v1: Rc<UH0>) -> i32 {
    match &v0 {
        US1::US1_VerbAnnounce => {
            announce_verb_5()
        }
        US1::US1_VerbCheck => {
            let v6: i32 = kit_window_query::kit_window_check_run();
            v6
        }
        US1::US1_VerbClaim => {
            let v8: i32 = kit_window_claim::kit_window_claim_run();
            v8
        }
        US1::US1_VerbList => {
            listed_with_announced_7()
        }
        US1::US1_VerbRelease => {
            let v3: Rc<str> = first_word_24(v1.clone());
            window_released_9(v3)
        }
        US1::US1_VerbRemote => {
            let v11: Rc<str> = first_word_24(v1);
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--bin"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = option_value_25(v12.clone());
            let v14: bool = &*v13 == "";
            let mut v17: Rc<str> = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                option_value_25(v12)
            };
            remote_status_23(v11, v17)
        }
        US1::US1_VerbRequest => {
            let v2: i32 = kit_window_request::kit_window_request_run();
            v2
        }
        US1::US1_VerbRun => {
            let v7: i32 = kit_window_run::kit_window_run_run();
            v7
        }
        US1::US1_VerbWaitClear => {
            let v9: i32 = kit_window_wait::kit_window_wait_run();
            v9
        }
    }
}
fn command_line_argument_count_27() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_28(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_26(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_27();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_28(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match &v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_26(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn class_word_29(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US4::US4_Compile => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US4::US4_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US4::US4_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US4::US4_HeavyBuild => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US4::US4_LiveGui => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US4::US4_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US4::US4_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn window_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_26(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    let v6: bool = "request" == &*v4;
                    let v42: US3 = if v6 {
                        let v7: US1 = US1::US1_VerbRequest;
                        US3::US3_Some(v7.clone())
                    } else {
                        let v9: bool = "release" == &*v4;
                        if v9 {
                            let v10: US1 = US1::US1_VerbRelease;
                            US3::US3_Some(v10.clone())
                        } else {
                            let v12: bool = "list" == &*v4;
                            if v12 {
                                let mut v13: US1 = US1::US1_VerbList;
                                US3::US3_Some(v13.clone())
                            } else {
                                let v15: bool = "check" == &*v4;
                                if v15 {
                                    let mut v16: US1 = US1::US1_VerbCheck;
                                    US3::US3_Some(v16.clone())
                                } else {
                                    let v18: bool = "run" == &*v4;
                                    if v18 {
                                        let v19: US1 = US1::US1_VerbRun;
                                        US3::US3_Some(v19.clone())
                                    } else {
                                        let v21: bool = "claim" == &*v4;
                                        if v21 {
                                            let v22: US1 = US1::US1_VerbClaim;
                                            US3::US3_Some(v22.clone())
                                        } else {
                                            let v24: bool = "wait-clear" == &*v4;
                                            if v24 {
                                                let v25: US1 = US1::US1_VerbWaitClear;
                                                US3::US3_Some(v25.clone())
                                            } else {
                                                let v27: bool = "announce" == &*v4;
                                                if v27 {
                                                    let v28: US1 = US1::US1_VerbAnnounce;
                                                    US3::US3_Some(v28.clone())
                                                } else {
                                                    let v30: bool = "remote" == &*v4;
                                                    if v30 {
                                                        let v31: US1 = US1::US1_VerbRemote;
                                                        US3::US3_Some(v31.clone())
                                                    } else {
                                                        US3::US3_None
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match &v42 {
                        US3::US3_None => {
                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) };
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v4));
                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                            let v50: US4 = US4::US4_LiveGui;
                            let v51: Rc<str> = class_word_29(v50.clone());
                            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v51.clone()));
                            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v52.clone()));
                            let v54: US4 = US4::US4_TestSuite;
                            let v55: Rc<str> = class_word_29(v54.clone());
                            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v55.clone()));
                            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v53.clone(), v56.clone()));
                            let v58: US4 = US4::US4_HeavyBuild;
                            let v59: Rc<str> = class_word_29(v58.clone());
                            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v59.clone()));
                            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v57.clone(), v60.clone()));
                            let v62: US4 = US4::US4_Compile;
                            let v63: Rc<str> = class_word_29(v62.clone());
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v63.clone()));
                            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v61.clone(), v64.clone()));
                            let v66: US4 = US4::US4_CompilerInstall;
                            let v67: Rc<str> = class_word_29(v66.clone());
                            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v67.clone()));
                            let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v65.clone(), v68.clone()));
                            let v70: US4 = US4::US4_TreeRead;
                            let v71: Rc<str> = class_word_29(v70.clone());
                            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v71.clone()));
                            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v72.clone()));
                            let v74: US4 = US4::US4_Bench;
                            let v75: Rc<str> = class_word_29(v74.clone());
                            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v75.clone()));
                            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v73.clone(), v76.clone()));
                            let v78: US4 = US4::US4_CompileHold;
                            let v79: Rc<str> = class_word_29(v78.clone());
                            let v80: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v79.clone()));
                            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v77.clone(), v80.clone()));
                            let v82: US4 = US4::US4_TreeWrite;
                            let v83: Rc<str> = class_word_29(v82.clone());
                            let v84: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v83.clone()));
                            let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v81.clone(), v84.clone()));
                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v87: Rc<str> = concat5_2(v49.clone(), v85.clone(), v86.clone(), v86.clone(), v86.clone());
                            let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v87));
                            refused_1(v88.clone())
                        }
                        US3::US3_Some(v43) => {
                            let v43: US1 = v43.clone();
                            verb_ran_4(v43.clone(), v5.clone())
                        }
                    }
                }
                _ => {
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                    let v93: US4 = US4::US4_LiveGui;
                    let v94: Rc<str> = class_word_29(v93.clone());
                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v94.clone()));
                    let v96: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v95.clone()));
                    let v97: US4 = US4::US4_TestSuite;
                    let v98: Rc<str> = class_word_29(v97.clone());
                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v98.clone()));
                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v96.clone(), v99.clone()));
                    let v101: US4 = US4::US4_HeavyBuild;
                    let v102: Rc<str> = class_word_29(v101.clone());
                    let v103: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v102.clone()));
                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v100.clone(), v103.clone()));
                    let v105: US4 = US4::US4_Compile;
                    let v106: Rc<str> = class_word_29(v105.clone());
                    let v107: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v106.clone()));
                    let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v104.clone(), v107.clone()));
                    let v109: US4 = US4::US4_CompilerInstall;
                    let v110: Rc<str> = class_word_29(v109.clone());
                    let v111: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v110.clone()));
                    let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v108.clone(), v111.clone()));
                    let v113: US4 = US4::US4_TreeRead;
                    let v114: Rc<str> = class_word_29(v113.clone());
                    let v115: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v114.clone()));
                    let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v112.clone(), v115.clone()));
                    let v117: US4 = US4::US4_Bench;
                    let v118: Rc<str> = class_word_29(v117.clone());
                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v118.clone()));
                    let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v116.clone(), v119.clone()));
                    let v121: US4 = US4::US4_CompileHold;
                    let v122: Rc<str> = class_word_29(v121.clone());
                    let v123: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v122.clone()));
                    let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v120.clone(), v123.clone()));
                    let v125: US4 = US4::US4_TreeWrite;
                    let v126: Rc<str> = class_word_29(v125.clone());
                    let v127: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v126.clone()));
                    let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v124.clone(), v127.clone()));
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = concat5_2(v92.clone(), v128.clone(), v129.clone(), v129.clone(), v129.clone());
                    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) };
                    let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v130));
                    refused_1(v132.clone())
                }
            }
        }
        _ => {
            let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
            let v136: US4 = US4::US4_LiveGui;
            let v137: Rc<str> = class_word_29(v136.clone());
            let v138: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v137.clone()));
            let v139: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v138.clone()));
            let v140: US4 = US4::US4_TestSuite;
            let v141: Rc<str> = class_word_29(v140.clone());
            let v142: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v141.clone()));
            let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v139.clone(), v142.clone()));
            let v144: US4 = US4::US4_HeavyBuild;
            let v145: Rc<str> = class_word_29(v144.clone());
            let v146: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v145.clone()));
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v143.clone(), v146.clone()));
            let v148: US4 = US4::US4_Compile;
            let v149: Rc<str> = class_word_29(v148.clone());
            let v150: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v149.clone()));
            let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v147.clone(), v150.clone()));
            let v152: US4 = US4::US4_CompilerInstall;
            let v153: Rc<str> = class_word_29(v152.clone());
            let v154: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v153.clone()));
            let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v151.clone(), v154.clone()));
            let v156: US4 = US4::US4_TreeRead;
            let v157: Rc<str> = class_word_29(v156.clone());
            let v158: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v157.clone()));
            let v159: Rc<str> = Rc::<str>::from(format!("{}{}", v155.clone(), v158.clone()));
            let v160: US4 = US4::US4_Bench;
            let v161: Rc<str> = class_word_29(v160.clone());
            let v162: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v161.clone()));
            let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v159.clone(), v162.clone()));
            let v164: US4 = US4::US4_CompileHold;
            let v165: Rc<str> = class_word_29(v164.clone());
            let v166: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v165.clone()));
            let v167: Rc<str> = Rc::<str>::from(format!("{}{}", v163.clone(), v166.clone()));
            let v168: US4 = US4::US4_TreeWrite;
            let v169: Rc<str> = class_word_29(v168.clone());
            let v170: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v169.clone()));
            let v171: Rc<str> = Rc::<str>::from(format!("{}{}", v167.clone(), v170.clone()));
            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v173: Rc<str> = concat5_2(v135.clone(), v171.clone(), v172.clone(), v172.clone(), v172.clone());
            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) };
            let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v174, v173));
            refused_1(v175.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        window_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn first_row_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn other_rows_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn dropped_tagged_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> u64 {
    loop {
        let v4: bool = &*v1 == "";
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = first_row_32(v1.clone());
            let v6: i32 = 3i32;
            let v7: Rc<str> = tab_field_14(v5.clone(), v6);
            let v8: bool = v7.contains(&*v2);
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_14(v5.clone(), v9);
                let v11: Rc<str> = window_file_10(v0.clone(), v10.clone());
                let v12: bool = file_removed_13(v11.clone());
                let mut v13: US0 = US0::US0_WindowDropped;
                event_logged_15(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_33(v1.clone());
                let v15: u64 = v3.wrapping_add(1u64);
                (v1, v3) = (v14.clone(), v15);
                continue;
            } else {
                let mut v17: Rc<str> = other_rows_33(v1.clone());
                v1 = v17.clone();
                continue;
            }
        }
    }
}
fn stored_rows_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn job_windows_dropped_30(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = windows_root_6();
    let v2: Rc<str> = stored_rows_34(v1.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v3, v0, v4, v5.clone(), v5);
    let v7: u64 = 0u64;
    dropped_tagged_31(v1, v2, v6, v7)
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        job_windows_dropped_30(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn heavy_wrapper_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (kit job "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = concat5_2(v2, v5, v3, v6, v7.clone());
    let v9: u64 = v4.wrapping_div(60000u64);
    let v10: u64 = v9.wrapping_add(1u64);
    let v11: Rc<str> = number_text_17(v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$argv = @(Get-Content -LiteralPath '"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("')\n. '"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = concat5_2(v12, v1, v13, v0, v14);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy -Label '"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("' -MaxMinutes "); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = concat5_2(v16, v8, v17, v11, v18);
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ & $argv[0] @($argv | Select-Object -Skip 1) }\n"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit $LASTEXITCODE\n"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v15, v19, v20, v21, v7)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64| -> Rc<str> {
        heavy_wrapper_35(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_window_run() -> i32 {
    closure0()()
}
pub fn kit_job_windows_dropped(v0: &str) -> u64 {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_heavy_wrapper(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
