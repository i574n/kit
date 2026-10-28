#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Apply,
    US0_Preview,
}
#[derive(Clone)]
enum US1 {
    US1_Succeeded,
    US1_Failed,
}
#[derive(Clone)]
enum US2 {
    US2_Edits,
    US2_Reads,
    US2_Facts,
    US2_Runs,
    US2_Writes,
    US2_Git,
    US2_Jobs,
    US2_Labs,
}
#[derive(Clone)]
enum US3 {
    US3_CompileRejected(Rc<str>),
    US3_Delegated(US2, Rc<str>),
    US3_Split(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(US2),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
fn elapsed_text_4(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_6() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let v0: i64 = local_offset_minutes_6();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_3(mut v0: US1, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US1::US1_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_4(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_5();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn rejected_2(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    println!("{}", v3);
    let v4: US1 = US1::US1_Failed;
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
    receipt_line_3(v4.clone(), v5.clone(), v1);
    2i32
}
fn delegated_run_7(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match &v0 {
        US2::US2_Edits => {
            let v3: i32 = kit_do_edit::kit_do_edit_run(&*v1, &*v2);
            v3
        }
        US2::US2_Facts => {
            let v5: i32 = kit_do_facts::kit_do_facts_run(&*v1, &*v2);
            v5
        }
        US2::US2_Git => {
            let v8: i32 = kit_do_git::kit_do_git_run(&*v1, &*v2);
            v8
        }
        US2::US2_Jobs => {
            let v9: i32 = kit_do_jobs::kit_do_jobs_run(&*v1, &*v2);
            v9
        }
        US2::US2_Labs => {
            let v10: i32 = kit_do_lab::kit_do_lab_run(&*v1, &*v2);
            v10
        }
        US2::US2_Reads => {
            let v4: i32 = kit_do_read::kit_do_read_run(&*v1, &*v2);
            v4
        }
        US2::US2_Runs => {
            let mut v6: i32 = kit_do_process::kit_do_process_run(&*v1, &*v2);
            v6
        }
        US2::US2_Writes => {
            let v7: i32 = kit_do_write::kit_do_write_run(&*v1, &*v2);
            v7
        }
    }
}
fn dry_headed_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(2i32);
    let v3: usize = ((2i32) as usize);
    let v4: i32 = 2i32.wrapping_add(v2);
    let v5: usize = ((v4) as usize);
    let mut v6: Rc<str> = Rc::<str>::from(&(v0)[v3..v5]);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    v8.clone()
}
fn previewed_run_8(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    match &v0 {
        US2::US2_Edits => {
            let v4: US2 = US2::US2_Edits;
            let v5: Rc<str> = dry_headed_9(v2.clone());
            delegated_run_7(v4.clone(), v1.clone(), v5.clone())
        }
        US2::US2_Facts => {
            let v9: US2 = US2::US2_Facts;
            delegated_run_7(v9.clone(), v1.clone(), v2.clone())
        }
        US2::US2_Reads => {
            let v7: US2 = US2::US2_Reads;
            delegated_run_7(v7.clone(), v1.clone(), v2.clone())
        }
        _ => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run covers edit, read and fact programs; this one writes, runs or changes state"); } LIT.with(|lit| lit.clone()) };
            rejected_2(v11.clone(), v3)
        }
    }
}
fn now_ms_10() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn compiled_text_12(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_do_program::kit_do_program_compiled(&*v0);
    v1
}
fn records_of_13(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn program_family_named_16(mut v0: Rc<str>) -> US4 {
    let v1: bool = &*v0 == "edits";
    if v1 {
        let v2: US2 = US2::US2_Edits;
        US4::US4_Some(v2)
    } else {
        let v4: bool = &*v0 == "reads";
        if v4 {
            let v5: US2 = US2::US2_Reads;
            US4::US4_Some(v5)
        } else {
            let v7: bool = &*v0 == "facts";
            if v7 {
                let v8: US2 = US2::US2_Facts;
                US4::US4_Some(v8)
            } else {
                let v10: bool = &*v0 == "runs";
                if v10 {
                    let v11: US2 = US2::US2_Runs;
                    US4::US4_Some(v11)
                } else {
                    let v13: bool = &*v0 == "writes";
                    if v13 {
                        let v14: US2 = US2::US2_Writes;
                        US4::US4_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "git";
                        if v16 {
                            let v17: US2 = US2::US2_Git;
                            US4::US4_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "jobs";
                            if v19 {
                                let v20: US2 = US2::US2_Jobs;
                                US4::US4_Some(v20)
                            } else {
                                let v22: bool = &*v0 == "labs";
                                if v22 {
                                    let v23: US2 = US2::US2_Labs;
                                    US4::US4_Some(v23)
                                } else {
                                    US4::US4_None
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn program_family_of_15(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = kit_do_verbs::kit_do_program_family(&*v0);
    let v2: US4 = program_family_named_16(v1);
    match &v2 {
        US4::US4_None => {
            US2::US2_Edits
        }
        US4::US4_Some(v3) => {
            let v3: US2 = v3.clone();
            v3
        }
    }
}
fn records_by_kinds_17(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: bool) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.iter().enumerate().filter(|(index, fields)| *index == 0 || fields.first().map(|first| (&*v1).split(char::from(124u8)).any(|kind| kind == &**first)).unwrap_or(false) == v2).map(|(_, fields)| fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(&char::from(31u8).to_string())).collect::<Vec<_>>().join(&char::from(30u8).to_string()).as_str());
    v3.clone()
}
fn program_decoded_11(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = compiled_text_12(v0.clone());
    let v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_13(v1.clone());
    let v3: i32 = 0i32;
    let v4: i32 = 0i32;
    let v5: Rc<str> = field_at_14(v2.clone(), v3, v4);
    let mut v6: bool = &*v5 == "ok";
    if v6 {
        let v7: US2 = program_family_of_15(v1.clone());
        match &v7 {
            US2::US2_Runs => {
                let v8: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
                let v11: US2 = if v8 {
                    US2::US2_Edits
                } else {
                    US2::US2_Runs
                };
                US3::US3_Delegated(v11.clone(), v1.clone())
            }
            US2::US2_Writes => {
                let v13: bool = v2.iter().skip(1).any(|fields| fields.first().map(|kind| &**kind == "edit").unwrap_or(false));
                if v13 {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit|except|replace|pattern|show"); } LIT.with(|lit| lit.clone()) };
                    let v15: bool = true;
                    let v16: Rc<str> = records_by_kinds_17(v2.clone(), v14.clone(), v15);
                    let v17: bool = false;
                    let v18: Rc<str> = records_by_kinds_17(v2.clone(), v14.clone(), v17);
                    US3::US3_Split(v16.clone(), v18.clone())
                } else {
                    let v20: US2 = US2::US2_Writes;
                    US3::US3_Delegated(v20.clone(), v1.clone())
                }
            }
            _ => {
                US3::US3_Delegated(v7.clone(), v1.clone())
            }
        }
    } else {
        let v26: i32 = 1i32;
        let v27: i32 = 0i32;
        let v28: Rc<str> = field_at_14(v2.clone(), v26, v27);
        US3::US3_CompileRejected(v28.clone())
    }
}
fn program_run_1(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: u64 = now_ms_10();
    let v4: US3 = program_decoded_11(v2);
    match &v4 {
        US3::US3_CompileRejected(v5) => {
            let v5: Rc<str> = v5.clone();
            rejected_2(v5, v3)
        }
        US3::US3_Delegated(v7, v8) => {
            let v7: US2 = v7.clone();
            let v8: Rc<str> = v8.clone();
            match &v0 {
                US0::US0_Apply => {
                    delegated_run_7(v7.clone(), v1.clone(), v8.clone())
                }
                US0::US0_Preview => {
                    previewed_run_8(v7, v1.clone(), v8, v3)
                }
            }
        }
        US3::US3_Split(v13, v14) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = v14.clone();
            match &v0 {
                US0::US0_Apply => {
                    let v15: US2 = US2::US2_Edits;
                    let v16: i32 = delegated_run_7(v15, v1.clone(), v13);
                    let v17: bool = v16 != 0i32;
                    if v17 {
                        v16
                    } else {
                        let v18: US2 = US2::US2_Writes;
                        delegated_run_7(v18, v1.clone(), v14.clone())
                    }
                }
                US0::US0_Preview => {
                    let v21: US2 = US2::US2_Writes;
                    previewed_run_8(v21, v1, v14, v3)
                }
            }
        }
    }
}
fn command_line_argument_count_19() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_18() -> i32 {
    let v0: i32 = command_line_argument_count_19();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_21(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_20(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_19();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US5 = if v5 {
        let mut v6: Rc<str> = command_line_argument_21(v1);
        US5::US5_Some(v6.clone())
    } else {
        US5::US5_None
    };
    match &v9 {
        US5::US5_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US5::US5_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn directory_exists_22(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn lines_joined_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
fn program_from_23(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = v0 < v1;
        if v3 {
            let v4: i32 = v0.wrapping_add(1i32);
            let v5: Rc<str> = argument_20(v0);
            let mut v6: Rc<str> = lines_joined_24(v2.clone(), v5.clone());
            (v0, v1, v2) = (v4, v1, v6.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn do_run_0() -> i32 {
    let v0: i32 = argument_count_18();
    let v1: i32 = 2i32;
    let v2: Rc<str> = argument_20(v1);
    let v3: i32 = 3i32;
    let v4: Rc<str> = argument_20(v3);
    let v5: bool = &*v4 == "--dry-run";
    let v8: US0 = if v5 {
        US0::US0_Preview
    } else {
        US0::US0_Apply
    };
    let v10: i32 = match &v8 {
        US0::US0_Apply => {
            3i32
        }
        US0::US0_Preview => {
            4i32
        }
    };
    let v11: bool = v0 <= v10;
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> [--dry-run] <program>... | do <root> [--dry-run] - (program on stdin; --dry-run chains every edit in memory and writes nothing); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit do <root> [--dry-run] <program>... | do <root> [--dry-run] - (program on stdin; --dry-run chains every edit in memory and writes nothing); statements: edit <path|glob> [dry-run] [expect files <n> replacements <n>], replace [pattern] once|all|<n> times \"<old>\" with \"<new>\" (pattern: a regex, $1 groups in the replacement; either takes raw lines between <<< and a >>> line), patch <<< apply_patch hunks (@@, then lines starting with space, - or +) >>>, show <path>"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v14);
        2i32
    } else {
        let v15: bool = directory_exists_22(v2.clone());
        let v16: bool = v15 == false;
        if v16 {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: root is not a directory: "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v2));
            println!("{}", v18);
            2i32
        } else {
            let v19: Rc<str> = argument_20(v10);
            let v20: bool = &*v19 == "-";
            if v20 {
                let v21: Rc<str> = { let mut text = String::new(); let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut text); std::rc::Rc::<str>::from(text) };
                program_run_1(v8.clone(), v2.clone(), v21.clone())
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = program_from_23(v10, v0, v23.clone());
                program_run_1(v8.clone(), v2.clone(), v24.clone())
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
