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
pub struct Mut0 { pub l0: Rc<str> }
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
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
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
    v0
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
fn method19(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_17(mut v0: u64) -> Rc<str> {
    format_real_18(v0)
}
fn window_event_word_20(mut v0: US0) -> Rc<str> {
    match v0 {
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
    v2
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
        let v10: Rc<str> = concat5_2(v7, v0, v8, v9.clone(), v9);
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
            let v28: Rc<str> = concat5_2(v23, v0, v26, v27.clone(), v27);
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
fn first_word_24(mut v0: &Rc<UH0>) -> Rc<str> {
    match &**v0 {
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
    v1
}
fn verb_ran_4(mut v0: US1, mut v1: Rc<UH0>) -> i32 {
    match v0 {
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
            let v3: Rc<str> = first_word_24(&v1);
            window_released_9(v3)
        }
        US1::US1_VerbRemote => {
            let v11: Rc<str> = first_word_24(&v1);
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
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_26(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn class_word_29(mut v0: US4) -> Rc<str> {
    match v0 {
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
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: bool = "request" == &*v4;
                    let v42: US3 = if v6 {
                        let v7: US1 = US1::US1_VerbRequest;
                        US3::US3_Some(v7)
                    } else {
                        let v9: bool = "release" == &*v4;
                        if v9 {
                            let v10: US1 = US1::US1_VerbRelease;
                            US3::US3_Some(v10)
                        } else {
                            let v12: bool = "list" == &*v4;
                            if v12 {
                                let mut v13: US1 = US1::US1_VerbList;
                                US3::US3_Some(v13)
                            } else {
                                let v15: bool = "check" == &*v4;
                                if v15 {
                                    let mut v16: US1 = US1::US1_VerbCheck;
                                    US3::US3_Some(v16)
                                } else {
                                    let v18: bool = "run" == &*v4;
                                    if v18 {
                                        let v19: US1 = US1::US1_VerbRun;
                                        US3::US3_Some(v19)
                                    } else {
                                        let v21: bool = "claim" == &*v4;
                                        if v21 {
                                            let v22: US1 = US1::US1_VerbClaim;
                                            US3::US3_Some(v22)
                                        } else {
                                            let v24: bool = "wait-clear" == &*v4;
                                            if v24 {
                                                let v25: US1 = US1::US1_VerbWaitClear;
                                                US3::US3_Some(v25)
                                            } else {
                                                let v27: bool = "announce" == &*v4;
                                                if v27 {
                                                    let v28: US1 = US1::US1_VerbAnnounce;
                                                    US3::US3_Some(v28)
                                                } else {
                                                    let v30: bool = "remote" == &*v4;
                                                    if v30 {
                                                        let v31: US1 = US1::US1_VerbRemote;
                                                        US3::US3_Some(v31)
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
                    match v42 {
                        US3::US3_None => {
                            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) }, v4));
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) }));
                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                            let v48: US4 = US4::US4_LiveGui;
                            let v49: Rc<str> = class_word_29(v48);
                            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v49));
                            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v50));
                            let v52: US4 = US4::US4_TestSuite;
                            let v53: Rc<str> = class_word_29(v52);
                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v53));
                            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v54));
                            let v56: US4 = US4::US4_HeavyBuild;
                            let v57: Rc<str> = class_word_29(v56);
                            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v57));
                            let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v58));
                            let v60: US4 = US4::US4_Compile;
                            let v61: Rc<str> = class_word_29(v60);
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v61));
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v62));
                            let v64: US4 = US4::US4_CompilerInstall;
                            let v65: Rc<str> = class_word_29(v64);
                            let v66: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v65));
                            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v66));
                            let v68: US4 = US4::US4_TreeRead;
                            let v69: Rc<str> = class_word_29(v68);
                            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v69));
                            let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v70));
                            let v72: US4 = US4::US4_Bench;
                            let v73: Rc<str> = class_word_29(v72);
                            let v74: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v73));
                            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v74));
                            let v76: US4 = US4::US4_CompileHold;
                            let v77: Rc<str> = class_word_29(v76);
                            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v77));
                            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v78));
                            let v80: US4 = US4::US4_TreeWrite;
                            let v81: Rc<str> = class_word_29(v80);
                            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v81));
                            let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v82));
                            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v85: Rc<str> = concat5_2(v47, v83, v84.clone(), v84.clone(), v84.clone());
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v85));
                            refused_1(v86)
                        }
                        US3::US3_Some(v43) => {
                            let v43: US1 = v43;
                            verb_ran_4(v43, v5)
                        }
                    }
                }
                _ => {
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                    let v91: US4 = US4::US4_LiveGui;
                    let v92: Rc<str> = class_word_29(v91);
                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v92));
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v93));
                    let v95: US4 = US4::US4_TestSuite;
                    let v96: Rc<str> = class_word_29(v95);
                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v96));
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v97));
                    let v99: US4 = US4::US4_HeavyBuild;
                    let v100: Rc<str> = class_word_29(v99);
                    let v101: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v100));
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v101));
                    let v103: US4 = US4::US4_Compile;
                    let v104: Rc<str> = class_word_29(v103);
                    let v105: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v104));
                    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v105));
                    let v107: US4 = US4::US4_CompilerInstall;
                    let v108: Rc<str> = class_word_29(v107);
                    let v109: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v108));
                    let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v109));
                    let v111: US4 = US4::US4_TreeRead;
                    let v112: Rc<str> = class_word_29(v111);
                    let v113: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v112));
                    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v113));
                    let v115: US4 = US4::US4_Bench;
                    let v116: Rc<str> = class_word_29(v115);
                    let v117: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v116));
                    let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v117));
                    let v119: US4 = US4::US4_CompileHold;
                    let v120: Rc<str> = class_word_29(v119);
                    let v121: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v120));
                    let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v121));
                    let v123: US4 = US4::US4_TreeWrite;
                    let v124: Rc<str> = class_word_29(v123);
                    let v125: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v124));
                    let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v125));
                    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v128: Rc<str> = concat5_2(v90, v126, v127.clone(), v127.clone(), v127.clone());
                    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) }, v128));
                    refused_1(v129)
                }
            }
        }
        _ => {
            let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
            let v133: US4 = US4::US4_LiveGui;
            let v134: Rc<str> = class_word_29(v133);
            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v134));
            let v136: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v135));
            let v137: US4 = US4::US4_TestSuite;
            let v138: Rc<str> = class_word_29(v137);
            let v139: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v138));
            let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v139));
            let v141: US4 = US4::US4_HeavyBuild;
            let v142: Rc<str> = class_word_29(v141);
            let v143: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v142));
            let v144: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v143));
            let v145: US4 = US4::US4_Compile;
            let v146: Rc<str> = class_word_29(v145);
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v146));
            let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v147));
            let v149: US4 = US4::US4_CompilerInstall;
            let v150: Rc<str> = class_word_29(v149);
            let v151: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v150));
            let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v151));
            let v153: US4 = US4::US4_TreeRead;
            let v154: Rc<str> = class_word_29(v153);
            let v155: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v154));
            let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v155));
            let v157: US4 = US4::US4_Bench;
            let v158: Rc<str> = class_word_29(v157);
            let v159: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v158));
            let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v159));
            let v161: US4 = US4::US4_CompileHold;
            let v162: Rc<str> = class_word_29(v161);
            let v163: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v162));
            let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v163));
            let v165: US4 = US4::US4_TreeWrite;
            let v166: Rc<str> = class_word_29(v165);
            let v167: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v166));
            let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v167));
            let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v170: Rc<str> = concat5_2(v132, v168, v169.clone(), v169.clone(), v169.clone());
            let v171: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) }, v170));
            refused_1(v171)
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
                let v11: Rc<str> = window_file_10(v0.clone(), v10);
                let v12: bool = file_removed_13(v11);
                let mut v13: US0 = US0::US0_WindowDropped;
                event_logged_15(v0.clone(), v13, v5);
                let v14: Rc<str> = other_rows_33(v1);
                let v15: u64 = v3.wrapping_add(1u64);
                (v1, v3) = (v14, v15);
                continue;
            } else {
                let mut v17: Rc<str> = other_rows_33(v1);
                v1 = v17;
                continue;
            }
        }
    }
}
fn stored_rows_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1
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
        job_windows_dropped_30(v0)
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
        heavy_wrapper_35(v0, v1, v2, v3, v4)
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
pub type SpiralShape_97df63ebabc3 = Mut0;
