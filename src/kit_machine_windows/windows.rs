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
    US1_LiveGui,
    US1_TestSuite,
    US1_HeavyBuild,
    US1_Compile,
    US1_CompilerInstall,
    US1_TreeRead,
    US1_Bench,
    US1_CompileHold,
    US1_TreeWrite,
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_2(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: window "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_3(v3);
    2i32
}
fn windows_root_4() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn joined_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn window_file_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".window"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_6(v0, v4)
}
fn text_or_empty_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn file_removed_8(mut v0: Rc<str>) -> bool {
    let v1: bool = std::fs::remove_file(&*v0).is_ok();
    v1
}
fn tab_field_9(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn now_ms_11() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method14(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn number_text_12(mut v0: u64) -> Rc<str> {
    format_real_13(v0)
}
fn event_logged_10(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>) -> () {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("events.log"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = joined_6(v0, v3);
    let v5: u64 = now_ms_11();
    let v6: Rc<str> = number_text_12(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = match &v1 {
        US0::US0_WindowDropped => {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dropped"); } LIT.with(|lit| lit.clone()) };
            v14
        }
        US0::US0_WindowGaveUp => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gave-up"); } LIT.with(|lit| lit.clone()) };
            v12
        }
        US0::US0_WindowGranted => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("granted"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US0::US0_WindowPruned => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pruned"); } LIT.with(|lit| lit.clone()) };
            v13
        }
        US0::US0_WindowQueued => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("queued"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US0::US0_WindowReleased => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_WindowReleasedWaiting => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released-waiting"); } LIT.with(|lit| lit.clone()) };
            v11
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
fn released_mark_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".released"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v2, &*v3, &*v3, &*v3].concat());
    joined_6(v0, v4)
}
fn text_written_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn stdout_line_17(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn window_released_1() -> i32 {
    let mut v0: i32 = 3i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: Rc<str> = windows_root_4();
    let v3: Rc<str> = window_file_5(v2.clone(), v1.clone());
    let v4: Rc<str> = text_or_empty_7(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("win-"); } LIT.with(|lit| lit.clone()) };
    let v6: bool = v1.starts_with(&*v5);
    let v7: bool = v6 == false;
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v1, &*v9, &*v10, &*v10].concat());
        refused_2(v11)
    } else {
        let v13: Rc<str> = window_file_5(v2.clone(), v1.clone());
        let v14: bool = file_removed_8(v13);
        if v14 {
            let v15: i32 = 2i32;
            let v16: Rc<str> = tab_field_9(v4.clone(), v15);
            let v17: bool = &*v16 == "waiting";
            let v18: US0 = US0::US0_WindowReleased;
            event_logged_10(v2.clone(), v18, v4);
            let v23: Rc<str> = if v17 {
                let v19: Rc<str> = released_mark_15(v2, v1.clone());
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("released"); } LIT.with(|lit| lit.clone()) };
                text_written_16(v19, v20)
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit window release ok id="); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = if v17 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" requester=told"); } LIT.with(|lit| lit.clone()) };
                v25
            } else {
                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26
            };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v1, &*v27, &*v28, &*v28].concat());
            stdout_line_17(v29);
            0i32
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release: unknown window id '"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; it was released or expired"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v1, &*v31, &*v32, &*v32].concat());
            refused_2(v33)
        }
    }
}
fn remote_status_18(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = kit_remote_status::kit_remote_status_run(&*v0, &*v1);
    v2
}
fn option_value_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn class_word_20(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Bench => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bench"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US1::US1_Compile => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US1::US1_CompileHold => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile-hold"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US1::US1_CompilerInstall => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compiler-install"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US1::US1_HeavyBuild => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-build"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_LiveGui => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("live-gui"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US1::US1_TestSuite => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-suite"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_TreeRead => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-read"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US1::US1_TreeWrite => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree-write"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn window_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: bool = &*v1 == "request";
    if v2 {
        let v3: i32 = kit_window_request::kit_window_request_run();
        v3
    } else {
        let v4: bool = &*v1 == "release";
        if v4 {
            window_released_1()
        } else {
            let v6: bool = &*v1 == "list";
            if v6 {
                let v7: i32 = kit_window_query::kit_window_list_run();
                v7
            } else {
                let v8: bool = &*v1 == "check";
                if v8 {
                    let v9: i32 = kit_window_query::kit_window_check_run();
                    v9
                } else {
                    let v10: bool = &*v1 == "run";
                    if v10 {
                        let v11: i32 = kit_window_run::kit_window_run_run();
                        v11
                    } else {
                        let v12: bool = &*v1 == "claim";
                        if v12 {
                            let v13: i32 = kit_window_claim::kit_window_claim_run();
                            v13
                        } else {
                            let v14: bool = &*v1 == "wait-clear";
                            if v14 {
                                let v15: i32 = kit_window_wait::kit_window_wait_run();
                                v15
                            } else {
                                let v16: bool = &*v1 == "remote";
                                if v16 {
                                    let v17: i32 = 3i32;
                                    let v18: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v17 as usize).unwrap_or_default());
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--bin"); } LIT.with(|lit| lit.clone()) };
                                    let v20: Rc<str> = option_value_19(v19.clone());
                                    let v21: bool = &*v20 == "";
                                    let v24: Rc<str> = if v21 {
                                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                                        v22.clone()
                                    } else {
                                        option_value_19(v19.clone())
                                    };
                                    remote_status_18(v18.clone(), v24.clone())
                                } else {
                                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) };
                                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: window request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | window release <id> | window list | window check <class> | window claim <path> | window wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | window run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] | window remote <ssh-alias> [--bin <remote kit path>]; classes "); } LIT.with(|lit| lit.clone()) };
                                    let mut v29: US1 = US1::US1_LiveGui;
                                    let mut v30: Rc<str> = class_word_20(v29.clone());
                                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v30.clone()));
                                    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v31.clone()));
                                    let v33: US1 = US1::US1_TestSuite;
                                    let v34: Rc<str> = class_word_20(v33.clone());
                                    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v34.clone()));
                                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v32.clone(), v35.clone()));
                                    let v37: US1 = US1::US1_HeavyBuild;
                                    let v38: Rc<str> = class_word_20(v37.clone());
                                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v38.clone()));
                                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v36.clone(), v39.clone()));
                                    let v41: US1 = US1::US1_Compile;
                                    let v42: Rc<str> = class_word_20(v41.clone());
                                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v42.clone()));
                                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40.clone(), v43.clone()));
                                    let v45: US1 = US1::US1_CompilerInstall;
                                    let v46: Rc<str> = class_word_20(v45.clone());
                                    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v46.clone()));
                                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v44.clone(), v47.clone()));
                                    let v49: US1 = US1::US1_TreeRead;
                                    let v50: Rc<str> = class_word_20(v49.clone());
                                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v50.clone()));
                                    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v48.clone(), v51.clone()));
                                    let v53: US1 = US1::US1_Bench;
                                    let v54: Rc<str> = class_word_20(v53.clone());
                                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v54.clone()));
                                    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v52.clone(), v55.clone()));
                                    let v57: US1 = US1::US1_CompileHold;
                                    let v58: Rc<str> = class_word_20(v57.clone());
                                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v58.clone()));
                                    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v56.clone(), v59.clone()));
                                    let v61: US1 = US1::US1_TreeWrite;
                                    let v62: Rc<str> = class_word_20(v61.clone());
                                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) }, v62.clone()));
                                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v60.clone(), v63.clone()));
                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v66: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v64, &*v65, &*v65, &*v65].concat());
                                    let v67: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v1, &*v27, &*v66, &*v65].concat());
                                    refused_2(v67.clone())
                                }
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
fn first_row_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().next().unwrap_or(""));
    v1
}
fn other_rows_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.lines().skip(1).collect::<Vec<_>>().join("
"));
    v1
}
fn dropped_tagged_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> u64 {
    loop {
        let v4: bool = &*v1 == "";
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = first_row_23(v1.clone());
            let v6: i32 = 3i32;
            let v7: Rc<str> = tab_field_9(v5.clone(), v6);
            let v8: bool = v7.contains(&*v2);
            if v8 {
                let v9: i32 = 0i32;
                let v10: Rc<str> = tab_field_9(v5.clone(), v9);
                let v11: Rc<str> = window_file_5(v0.clone(), v10.clone());
                let v12: bool = file_removed_8(v11.clone());
                let v13: US0 = US0::US0_WindowDropped;
                event_logged_10(v0.clone(), v13.clone(), v5.clone());
                let v14: Rc<str> = other_rows_24(v1.clone());
                let v15: u64 = v3.wrapping_add(1u64);
                (v0, v1, v2, v3) = (v0.clone(), v14.clone(), v2.clone(), v15);
                continue;
            } else {
                let v17: Rc<str> = other_rows_24(v1.clone());
                (v0, v1, v2, v3) = (v0.clone(), v17.clone(), v2.clone(), v3);
                continue;
            }
        }
    }
}
fn stored_rows_25(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut rows = std::fs::read_dir(&*v0).into_iter().flatten().flatten().map(|entry| entry.path()).filter(|path| path.extension().map(|ext| ext == "window").unwrap_or(false)).filter_map(|path| std::fs::read_to_string(&path).ok()).map(|text| text.trim().to_owned()).filter(|text| !text.is_empty()).collect::<Vec<_>>(); rows.sort_by_key(|row| row.split(char::from(9u8)).nth(4).and_then(|start| start.parse::<u64>().ok()).unwrap_or(0)); std::rc::Rc::<str>::from(rows.join("
")) };
    v1.clone()
}
fn job_windows_dropped_21(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = windows_root_4();
    let v2: Rc<str> = stored_rows_25(v1.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(kit job "); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v0, &*v4, &*v5, &*v5].concat());
    let v7: u64 = 0u64;
    dropped_tagged_22(v1, v2, v6, v7)
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> u64> = Rc::new(move |mut v0: Rc<str>| -> u64 {
        job_windows_dropped_21(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn heavy_wrapper_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (kit job "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v5, &*v3, &*v6, &*v7].concat());
    let v9: u64 = v4.wrapping_div(60000u64);
    let v10: u64 = v9.wrapping_add(1u64);
    let v11: Rc<str> = number_text_12(v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$argv = @(Get-Content -LiteralPath '"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("')\n. '"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v1, &*v13, &*v0, &*v14].concat());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Invoke-Heavy -Label '"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("' -MaxMinutes "); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v8, &*v17, &*v11, &*v18].concat());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{ & $argv[0] @($argv | Select-Object -Skip 1) }\n"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit $LASTEXITCODE\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v19, &*v20, &*v21, &*v7].concat());
    v22
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64| -> Rc<str> {
        heavy_wrapper_26(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
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
