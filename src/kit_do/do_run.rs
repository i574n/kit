#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Edits,
    US0_Reads,
    US0_Facts,
    US0_Runs,
    US0_Writes,
    US0_Git,
    US0_Jobs,
    US0_Labs,
}
#[derive(Clone)]
enum US1 {
    US1_CompileRejected(Rc<str>),
    US1_Delegated(US0, Rc<str>),
    US1_Split(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
fn delegated_run_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match &v0 {
        US0::US0_Edits => {
            let v3: i32 = kit_do_edit::kit_do_edit_run(&*v1, &*v2);
            v3
        }
        US0::US0_Facts => {
            let v5: i32 = kit_do_facts::kit_do_facts_run(&*v1, &*v2);
            v5
        }
        US0::US0_Git => {
            let v8: i32 = kit_do_git::kit_do_git_run(&*v1, &*v2);
            v8
        }
        US0::US0_Jobs => {
            let v9: i32 = kit_do_jobs::kit_do_jobs_run(&*v1, &*v2);
            v9
        }
        US0::US0_Labs => {
            let v10: i32 = kit_do_lab::kit_do_lab_run(&*v1, &*v2);
            v10
        }
        US0::US0_Reads => {
            let v4: i32 = kit_do_read::kit_do_read_run(&*v1, &*v2);
            v4
        }
        US0::US0_Runs => {
            let mut v6: i32 = kit_do_process::kit_do_process_run(&*v1, &*v2);
            v6
        }
        US0::US0_Writes => {
            let v7: i32 = kit_do_write::kit_do_write_run(&*v1, &*v2);
            v7
        }
    }
}
fn now_ms_3() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn compiled_text_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_do_program::kit_do_program_compiled(&*v0);
    v1
}
fn records_of_6(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn program_family_named_9(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "edits";
    if v1 {
        let v2: US0 = US0::US0_Edits;
        US2::US2_Some(v2)
    } else {
        let v4: bool = &*v0 == "reads";
        if v4 {
            let v5: US0 = US0::US0_Reads;
            US2::US2_Some(v5)
        } else {
            let v7: bool = &*v0 == "facts";
            if v7 {
                let v8: US0 = US0::US0_Facts;
                US2::US2_Some(v8)
            } else {
                let v10: bool = &*v0 == "runs";
                if v10 {
                    let v11: US0 = US0::US0_Runs;
                    US2::US2_Some(v11)
                } else {
                    let v13: bool = &*v0 == "writes";
                    if v13 {
                        let v14: US0 = US0::US0_Writes;
                        US2::US2_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "git";
                        if v16 {
                            let v17: US0 = US0::US0_Git;
                            US2::US2_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "jobs";
                            if v19 {
                                let v20: US0 = US0::US0_Jobs;
                                US2::US2_Some(v20)
                            } else {
                                let v22: bool = &*v0 == "labs";
                                if v22 {
                                    let v23: US0 = US0::US0_Labs;
                                    US2::US2_Some(v23)
                                } else {
                                    US2::US2_None
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn program_family_of_8(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = kit_do_verbs::kit_do_program_family(&*v0);
    let v2: US2 = program_family_named_9(v1);
    match &v2 {
        US2::US2_None => {
            US0::US0_Edits
        }
        US2::US2_Some(v3) => {
            let v3: US0 = v3.clone();
            v3
        }
    }
}
fn records_by_kinds_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: bool) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.iter().enumerate().filter(|(index, fields)| *index == 0 || fields.first().map(|first| (&*v1).split(char::from(124u8)).any(|kind| kind == &**first)).unwrap_or(false) == v2).map(|(_, fields)| fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(&char::from(31u8).to_string())).collect::<Vec<_>>().join(&char::from(30u8).to_string()).as_str());
    v3.clone()
}
fn program_decoded_4(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = compiled_text_5(v0.clone());
    let v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_6(v1.clone());
    let v3: i32 = 0i32;
    let v4: i32 = 0i32;
    let v5: Rc<str> = field_at_7(v2.clone(), v3, v4);
    let mut v6: bool = &*v5 == "ok";
    if v6 {
        let v7: US0 = program_family_of_8(v1.clone());
        match &v7 {
            US0::US0_Runs => {
                let v8: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
                let v11: US0 = if v8 {
                    US0::US0_Edits
                } else {
                    US0::US0_Runs
                };
                US1::US1_Delegated(v11.clone(), v1.clone())
            }
            US0::US0_Writes => {
                let v13: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
                if v13 {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit|except|replace|pattern|show"); } LIT.with(|lit| lit.clone()) };
                    let v15: bool = true;
                    let v16: Rc<str> = records_by_kinds_10(v2.clone(), v14.clone(), v15);
                    let v17: bool = false;
                    let v18: Rc<str> = records_by_kinds_10(v2.clone(), v14.clone(), v17);
                    US1::US1_Split(v16.clone(), v18.clone())
                } else {
                    let v20: US0 = US0::US0_Writes;
                    US1::US1_Delegated(v20.clone(), v1.clone())
                }
            }
            _ => {
                US1::US1_Delegated(v7.clone(), v1.clone())
            }
        }
    } else {
        let v26: i32 = 1i32;
        let v27: i32 = 0i32;
        let v28: Rc<str> = field_at_7(v2.clone(), v26, v27);
        US1::US1_CompileRejected(v28.clone())
    }
}
fn elapsed_text_12(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_14() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_13() -> Rc<str> {
    let v0: i64 = local_offset_minutes_14();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_11(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US3::US3_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_12(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_13();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn program_run_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_3();
    let v3: US1 = program_decoded_4(v1.clone());
    match &v3 {
        US1::US1_CompileRejected(v4) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            println!("{}", &*v6);
            let v7: US3 = US3::US3_Failed;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
            receipt_line_11(v7.clone(), v8.clone(), v2);
            2i32
        }
        US1::US1_Delegated(v9, v10) => {
            let v9: US0 = v9.clone();
            let v10: Rc<str> = v10.clone();
            delegated_run_2(v9.clone(), v0.clone(), v10.clone())
        }
        US1::US1_Split(v12, v13) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let v14: US0 = US0::US0_Edits;
            let v15: i32 = delegated_run_2(v14.clone(), v0.clone(), v12.clone());
            let v16: bool = v15 != 0i32;
            if v16 {
                v15
            } else {
                let v17: US0 = US0::US0_Writes;
                delegated_run_2(v17.clone(), v0.clone(), v13.clone())
            }
        }
    }
}
fn command_line_argument_count_16() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_15() -> i32 {
    let v0: i32 = command_line_argument_count_16();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_18(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_17(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_16();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US4 = if v5 {
        let mut v6: Rc<str> = command_line_argument_18(v1);
        US4::US4_Some(v6.clone())
    } else {
        US4::US4_None
    };
    match &v9 {
        US4::US4_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US4::US4_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn directory_exists_19(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn lines_joined_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0.clone()
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v4)), v1));
            v6.clone()
        }
    }
}
fn program_from_20(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = v0 < v1;
        if v3 {
            let v4: i32 = v0.wrapping_add(1i32);
            let v5: Rc<str> = argument_17(v0);
            let mut v6: Rc<str> = lines_joined_21(v2.clone(), v5.clone());
            (v0, v1, v2) = (v4, v1, v6.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn do_run_0() -> i32 {
    let v0: i32 = argument_count_15();
    let v1: i32 = 2i32;
    let v2: Rc<str> = argument_17(v1);
    let v3: bool = v0 < 4i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> <program>... | do <root> - (program on stdin); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit do <root> <program>... | do <root> - (program on stdin); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v6);
        2i32
    } else {
        let v7: bool = directory_exists_19(v2.clone());
        let v8: bool = v7 == false;
        if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: root is not a directory: "); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
            println!("{}", &*v10);
            2i32
        } else {
            let v11: i32 = 3i32;
            let v12: Rc<str> = argument_17(v11);
            let v13: bool = &*v12 == "-";
            if v13 {
                let v14: Rc<str> = { let mut text = String::new(); let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut text); std::rc::Rc::<str>::from(text) };
                program_run_1(v2.clone(), v14.clone())
            } else {
                let v16: i32 = 3i32;
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = program_from_20(v16, v0, v17.clone());
                program_run_1(v2.clone(), v18.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        do_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_run() -> i32 {
    closure0()()
}
