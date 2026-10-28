#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ByDay,
    US2_ByHabit,
}
#[derive(Clone)]
enum US3 {
    US3_ClaudeCode,
    US3_Codex,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoDays,
    UH0_DayThen(Rc<str>, u64, u64, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_KitCall,
    US4_HabitCall,
    US4_OtherCall,
}
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_4(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn argument_count_5() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn option_value_1(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v1.wrapping_add(1i32);
        let mut v3: i32 = argument_count_5();
        let mut v4: bool = v2 >= v3;
        if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let v6: Rc<str> = argument_2(v1);
            let v7: bool = &*v6 == &*v0;
            if v7 {
                return argument_2(v2);
            } else {
                (v0, v1) = (v0.clone(), v2);
                continue;
            }
        }
    }
}
fn method7(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_6(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method7(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v4: Option<US1> = v2.map(|x| v3(x));
    let v6: US1 = US1::US1_None;
    let v7: US1 = v6.clone();
    let v8: US1 = v4.unwrap_or(v7);
    match &v8 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn flag_present_8(mut v0: Rc<str>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = argument_count_5();
        let mut v3: bool = v1 >= v2;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = argument_2(v1);
            let v5: bool = &*v4 == &*v0;
            if v5 {
                return true;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            }
        }
    }
}
fn session_files_9(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { let since = std::time::SystemTime::now() - std::time::Duration::from_secs(v1 * 86400); let mut found = Vec::new(); let mut stack = vec![std::path::PathBuf::from(&*v0)]; while let Some(dir) = stack.pop() { let Ok(entries) = std::fs::read_dir(&dir) else { continue }; for entry in entries.flatten() { let path = entry.path(); if path.is_dir() { stack.push(path); } else if path.extension().map(|extension| extension == "jsonl").unwrap_or(false) && entry.metadata().and_then(|meta| meta.modified()).map(|modified| modified >= since).unwrap_or(false) { found.push(path.to_string_lossy().into_owned()); } } } found.sort(); std::rc::Rc::<str>::from(found.join("
").as_str()) };
    v2.clone()
}
fn cutoff_day_10(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).saturating_sub(v0 * 86400) as i64; let z = secs.div_euclid(86400) + 719468; let era = z.div_euclid(146097); let doe = z - era * 146097; let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); let mp = (5 * doy + 2) / 153; let day = doy - (153 * mp + 2) / 5 + 1; let month = if mp < 10 { mp + 3 } else { mp - 9 }; let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 }; std::rc::Rc::<str>::from(format!("{year:04}-{month:02}-{day:02}").as_str()) };
    v1.clone()
}
fn file_calls_12(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<str>) -> Rc<str> {
    let v6: Rc<str> = match &v1 {
        US3::US3_ClaudeCode => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tool_use"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US3::US3_Codex => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("function_call"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
    };
    let v10: Rc<str> = match &v1 {
        US3::US3_ClaudeCode => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22tool_use\\x22,\\x22id\\x22:\\x22[^\\x22]*\\x22,\\x22name\\x22:\\x22(?:PowerShell|Bash)\\x22,\\x22input\\x22:\\{\\x22command\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US3::US3_Codex => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22type\\x22:\\x22function_call\\x22,\\x22name\\x22:\\x22(?:shell|exec_command|local_shell)\\x22,\\x22arguments\\x22:\\x22((?:[^\\x22\\\\]|\\\\.)*)\\x22"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x22timestamp\\x22:\\x22(\\d{4}-\\d\\d-\\d\\d)"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { let quote = char::from(34u8); let backslash = char::from(92u8); let stamp = regex::Regex::new(&*v11).unwrap(); let command = regex::Regex::new(&*v10).unwrap(); let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let mut out = String::new(); for line in text.lines().filter(|line| line.contains(&*v6)) { let Some(day) = stamp.captures(line).map(|found| found[1].to_owned()) else { continue }; if day.as_str() < &*v2 { continue; } for found in command.captures_iter(line) { let decoded = found[1].replace(&format!("{backslash}{quote}"), &quote.to_string()).replace(&format!("{backslash}{backslash}"), &backslash.to_string()).replace(char::from(10u8), " ").replace(char::from(9u8), " "); out.push_str(&format!("{day}	{decoded}
")); } } std::rc::Rc::<str>::from(out.as_str()) };
    v12.clone()
}
fn tally_added_15(mut v0: Rc<str>, mut v1: US4, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v2 {
        UH0::UH0_DayThen(v14, v15, v16, v17, v18) => {
            let v14: Rc<str> = v14.clone();
            let v15: u64 = *v15;
            let v16: u64 = *v16;
            let v17: u64 = *v17;
            let v18: Rc<UH0> = v18.clone();
            let v19: bool = &*v14 == &*v0;
            if v19 {
                let (mut v31, mut v32, mut v33): (u64, u64, u64) = match &v1 {
                    US4::US4_KitCall => {
                        let v20: u64 = v15.wrapping_add(1u64);
                        let v21: u64 = v16.wrapping_add(1u64);
                        (v20, v21, v17)
                    }
                    _ => {
                        let (mut v28, mut v29, mut v30): (u64, u64, u64) = match &v1 {
                            US4::US4_HabitCall => {
                                let v22: u64 = v15.wrapping_add(1u64);
                                let v23: u64 = v17.wrapping_add(1u64);
                                (v22, v16, v23)
                            }
                            _ => {
                                let (mut v25, mut v26, mut v27): (u64, u64, u64) = match &v1 {
                                    US4::US4_OtherCall => {
                                        let v24: u64 = v15.wrapping_add(1u64);
                                        (v24, v16, v17)
                                    }
                                    _ => unreachable!(),
                                };
                                (v25, v26, v27)
                            }
                        };
                        (v28, v29, v30)
                    }
                };
                Rc::new(UH0::UH0_DayThen(v14.clone(), v31, v32, v33, v18.clone()))
            } else {
                let v35: Rc<UH0> = tally_added_15(v0.clone(), v1.clone(), v18.clone());
                Rc::new(UH0::UH0_DayThen(v14.clone(), v15, v16, v17, v35.clone()))
            }
        }
        UH0::UH0_NoDays => {
            let (mut v9, mut v10, mut v11): (u64, u64, u64) = match &v1 {
                US4::US4_KitCall => {
                    (1u64, 1u64, 0u64)
                }
                _ => {
                    let (mut v6, mut v7, mut v8): (u64, u64, u64) = match &v1 {
                        US4::US4_HabitCall => {
                            (1u64, 0u64, 1u64)
                        }
                        _ => {
                            let (mut v3, mut v4, mut v5): (u64, u64, u64) = match &v1 {
                                US4::US4_OtherCall => {
                                    (1u64, 0u64, 0u64)
                                }
                                _ => unreachable!(),
                            };
                            (v3, v4, v5)
                        }
                    };
                    (v6, v7, v8)
                }
            };
            let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_DayThen(v0.clone(), v9, v10, v11, v12.clone()))
        }
    }
}
fn words_added_16(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: u64 = (&*v0).len() as u64;
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: u64 = (&*v0)[v1 as usize..].find(char::from(10u8)).map(|found| v1 + found as u64).unwrap_or((&*v0).len() as u64);
            let v6: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[v1 as usize..v5 as usize]);
            let v7: u64 = v5.wrapping_add(1u64);
            let v8: bool = &*v6 == "";
            let v11: Rc<UH0> = if v8 {
                v2.clone()
            } else {
                let v9: US4 = US4::US4_HabitCall;
                tally_added_15(v6.clone(), v9.clone(), v2.clone())
            };
            (v0, v1, v2) = (v0.clone(), v7, v11.clone());
            continue;
        }
    }
}
fn habit_any_17(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_adoption_habits::kit_adoption_habit_any(&*v0);
    v1
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        habit_any_17(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn kit_named_19(mut v0: Rc<str>) -> bool {
    let v1: bool = { let text = &*v0; let bytes = text.as_bytes(); let separators = [32u8, 9, 13, 10, 38, 39, 34, 92, 47, 40, 59, 124]; text.match_indices("kit").any(|(at, found)| { let before = at == 0 || separators.contains(&bytes[at - 1]); let mut rest = &text[at + found.len()..]; if let Some(stripped) = rest.strip_prefix(".exe") { rest = stripped; } if let Some(stripped) = rest.strip_prefix(|c: char| c == char::from(39u8) || c == char::from(34u8)) { rest = stripped; } let trimmed = rest.trim_start(); before && trimmed.len() < rest.len() && trimmed.chars().next().is_some_and(|c| c.is_ascii_lowercase()) }) };
    v1
}
fn call_of_command_18(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<str>) -> US4 {
    let v2: bool = kit_named_19(v1.clone());
    if v2 {
        US4::US4_KitCall
    } else {
        let mut v4: bool = v0(v1);
        if v4 {
            US4::US4_HabitCall
        } else {
            US4::US4_OtherCall
        }
    }
}
fn habit_words_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_adoption_habits::kit_adoption_habit_words(&*v0);
    v1
}
fn row_tallied_14(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match &v0 {
        US2::US2_ByDay => {
            let mut v4: Rc<dyn Fn(Rc<str>) -> bool> = closure2();
            let v5: US4 = call_of_command_18(v4, v2.clone());
            tally_added_15(v1, v5, v3.clone())
        }
        US2::US2_ByHabit => {
            let v7: Rc<str> = habit_words_20(v2);
            let v8: u64 = 0u64;
            words_added_16(v7, v8, v3)
        }
    }
}
fn calls_tallied_13(mut v0: US2, mut v1: US3, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v5: u64 = (&*v2).len() as u64;
        let v6: bool = v3 >= v5;
        if v6 {
            return v4.clone();
        } else {
            let v7: u64 = (&*v2)[v3 as usize..].find(char::from(10u8)).map(|found| v3 + found as u64).unwrap_or((&*v2).len() as u64);
            let v8: Rc<str> = std::rc::Rc::<str>::from(&(&*v2)[v3 as usize..v7 as usize]);
            let v12: Rc<str> = match &v1 {
                US3::US3_ClaudeCode => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
                US3::US3_Codex => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("codex"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
            };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            let v15: Rc<str> = std::rc::Rc::<str>::from((&*v8).split(char::from(9u8)).nth(0i32 as usize).unwrap_or(""));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
            let v17: u64 = v7.wrapping_add(1u64);
            let v18: bool = &*v8 == "";
            let v21: Rc<UH0> = if v18 {
                v4.clone()
            } else {
                let v19: Rc<str> = std::rc::Rc::<str>::from((&*v8).split(char::from(9u8)).nth(1i32 as usize).unwrap_or(""));
                row_tallied_14(v0.clone(), v16.clone(), v19.clone(), v4.clone())
            };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v17, v21.clone());
            continue;
        }
    }
}
fn files_tallied_11(mut v0: US2, mut v1: US3, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v6: u64 = (&*v2).len() as u64;
        let v7: bool = v4 >= v6;
        if v7 {
            return v5.clone();
        } else {
            let v8: u64 = (&*v2)[v4 as usize..].find(char::from(10u8)).map(|found| v4 + found as u64).unwrap_or((&*v2).len() as u64);
            let v9: Rc<str> = std::rc::Rc::<str>::from(&(&*v2)[v4 as usize..v8 as usize]);
            let v10: u64 = v8.wrapping_add(1u64);
            let v11: bool = &*v9 == "";
            let v15: Rc<UH0> = if v11 {
                v5.clone()
            } else {
                let v12: Rc<str> = file_calls_12(v9.clone(), v1.clone(), v3.clone());
                let v13: u64 = 0u64;
                calls_tallied_13(v0.clone(), v1.clone(), v12.clone(), v13, v5.clone())
            };
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v10, v15.clone());
            continue;
        }
    }
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
fn kit_share_24(mut v0: u64, mut v1: u64, mut v2: u64) -> f64 {
    let mut v3: bool = v0 == 0u64;
    if v3 {
        0.0f64
    } else {
        let mut v4: f64 = (v1 as f64) / (v0 as f64);
        v4
    }
}
fn rows_of_21(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> (Rc<str>, u64, u64, u64) {
    loop {
        match &*v0 {
            UH0::UH0_DayThen(v5, v6, v7, v8, v9) => {
                let v5: Rc<str> = v5.clone();
                let v6: u64 = *v6;
                let v7: u64 = *v7;
                let v8: u64 = *v8;
                let v9: Rc<UH0> = v9.clone();
                let v10: bool = &*v1 == "";
                let v42: Rc<str> = if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v11));
                    let v13: Rc<str> = format_real_22(v6);
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v11));
                    let v16: Rc<str> = format_real_22(v7);
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v11));
                    let v18: Rc<str> = format_real_22(v8);
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v18)), v11));
                    let v21: f64 = kit_share_24(v6, v7, v8);
                    let v22: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v21));
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v15, Rc::<str>::from(format!("{}{}", v20, v22))));
                    v24.clone()
                } else {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v25));
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v27));
                    let v29: Rc<str> = format_real_22(v6);
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v29)), v27));
                    let v32: Rc<str> = format_real_22(v7);
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v27));
                    let v34: Rc<str> = format_real_22(v8);
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v27));
                    let v37: f64 = kit_share_24(v6, v7, v8);
                    let v38: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v37));
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v26, Rc::<str>::from(format!("{}{}", v31, Rc::<str>::from(format!("{}{}", v36, v38))))));
                    v41.clone()
                };
                let v43: u64 = v2.wrapping_add(v6);
                let v44: u64 = v3.wrapping_add(v7);
                let v45: u64 = v4.wrapping_add(v8);
                (v0, v1, v2, v3, v4) = (v9.clone(), v42.clone(), v43, v44, v45);
                continue;
            }
            UH0::UH0_NoDays => {
                return (v1.clone(), v2, v3, v4);
            }
        }
    }
}
fn max_habit_26(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        match &*v0 {
            UH0::UH0_DayThen(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: u64 = *v4;
                let v5: u64 = *v5;
                let v6: u64 = *v6;
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = v6 > v2;
                if v8 {
                    (v0, v1, v2) = (v7.clone(), v3.clone(), v6);
                    continue;
                } else {
                    (v0, v1, v2) = (v7.clone(), v1.clone(), v2);
                    continue;
                }
            }
            UH0::UH0_NoDays => {
                return (v1.clone(), v2);
            }
        }
    }
}
fn without_key_27(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_DayThen(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: u64 = *v4;
            let v5: u64 = *v5;
            let v6: u64 = *v6;
            let v7: Rc<UH0> = v7.clone();
            let v8: bool = &*v3 == &*v0;
            if v8 {
                v7.clone()
            } else {
                let v9: Rc<UH0> = without_key_27(v0.clone(), v7.clone());
                Rc::new(UH0::UH0_DayThen(v3.clone(), v4, v5, v6, v9.clone()))
            }
        }
        UH0::UH0_NoDays => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) }
        }
    }
}
fn habit_lines_25(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v4: u64 = 0u64;
        let (mut v5, mut v6): (Rc<str>, u64) = max_habit_26(v0.clone(), v3.clone(), v4);
        let v7: bool = &*v5 == "";
        if v7 {
            return (v1.clone(), v2);
        } else {
            let v8: Rc<UH0> = without_key_27(v5.clone(), v0.clone());
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v5));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = format_real_22(v6);
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2) = (v8.clone(), v15.clone(), v16);
            continue;
        }
    }
}
fn adoption_run_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--days-back"); } LIT.with(|lit| lit.clone()) };
    let v1: i32 = 2i32;
    let v2: Rc<str> = option_value_1(v0.clone(), v1);
    let mut v3: bool = &*v2 != "";
    let v7: Rc<str> = if v3 {
        v2.clone()
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--days"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = 2i32;
        option_value_1(v4.clone(), v5)
    };
    let v8: u64 = decimal_u64_6(v7.clone());
    let v9: bool = v8 == 0u64;
    let v10: u64 = if v9 {
        7u64
    } else {
        v8
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
    let v12: i32 = 2i32;
    let v13: Rc<str> = option_value_1(v11.clone(), v12);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--habits"); } LIT.with(|lit| lit.clone()) };
    let v15: i32 = 2i32;
    let v16: bool = flag_present_8(v14.clone(), v15);
    let v19: US2 = if v16 {
        US2::US2_ByHabit
    } else {
        US2::US2_ByDay
    };
    let v20: US3 = US3::US3_ClaudeCode;
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".claude/projects"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v21).to_string_lossy().as_ref());
    let v23: Rc<str> = session_files_9(v22.clone(), v10);
    let v24: Rc<str> = cutoff_day_10(v10);
    let v25: u64 = 0u64;
    let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoDays); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH0> = files_tallied_11(v19.clone(), v20.clone(), v23.clone(), v24.clone(), v25, v26.clone());
    let v28: US3 = US3::US3_Codex;
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".codex/sessions"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(std::path::PathBuf::from).unwrap_or_default().join(&*v29).to_string_lossy().as_ref());
    let v31: Rc<str> = session_files_9(v30.clone(), v10);
    let v32: Rc<str> = cutoff_day_10(v10);
    let v33: u64 = 0u64;
    let v34: Rc<UH0> = files_tallied_11(v19.clone(), v28.clone(), v31.clone(), v32.clone(), v33, v27.clone());
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v36: u64 = 0u64;
    let v37: u64 = 0u64;
    let v38: u64 = 0u64;
    let (mut v39, mut v40, mut v41, mut v42): (Rc<str>, u64, u64, u64) = rows_of_21(v34.clone(), v35.clone(), v36, v37, v38);
    let v43: u64 = 0u64;
    let (mut v44, mut v45): (Rc<str>, u64) = habit_lines_25(v34.clone(), v35.clone(), v43);
    let v54: Rc<str> = match &v19 {
        US2::US2_ByDay => {
            let v46: Rc<str> = { let mut lines = (&*v39).lines().filter(|line| !line.is_empty()).collect::<Vec<_>>(); lines.sort(); std::rc::Rc::<str>::from(lines.join("
")) };
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("client\tday\tshell_calls\tkit_calls\thabit_calls\tkit_share\n"); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v46));
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            v50.clone()
        }
        US2::US2_ByHabit => {
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("habit\tcalls\n"); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v44));
            v52.clone()
        }
    };
    let v55: bool = &*v13 == "";
    let v57: Rc<str> = if v55 {
        v35.clone()
    } else {
        let v56: Rc<str> = std::rc::Rc::<str>::from(std::fs::write(&*v13, (&*v54).as_bytes()).err().map(|error| format!("write {}: {error}", &*v13)).unwrap_or_default().as_str());
        v56.clone()
    };
    let v58: bool = &*v57 != "";
    if v58 {
        eprintln!("kit error: adoption: {}", &*v57);
        2i32
    } else {
        if v55 {
            print!("{}", &*v54);
            ()
        };
        let v95: Rc<str> = match &v19 {
            US2::US2_ByDay => {
                let v59: Rc<str> = format_real_22(v40);
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shell_calls="); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kit_calls="); } LIT.with(|lit| lit.clone()) };
                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                let v64: Rc<str> = format_real_22(v41);
                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                let v66: Rc<str> = format_real_22(v42);
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" habit_calls="); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v66));
                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kit_share="); } LIT.with(|lit| lit.clone()) };
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
                let v71: f64 = kit_share_24(v40, v41, v42);
                let v72: Rc<str> = std::rc::Rc::<str>::from(format!("{:.3}", v71));
                let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v65, Rc::<str>::from(format!("{}{}", v70, v72))));
                let v75: Rc<str> = format_real_22(v10);
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit adoption ok days_back="); } LIT.with(|lit| lit.clone()) };
                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v76, v75)), v74));
                let v80: Rc<str> = if v55 {
                    v35.clone()
                } else {
                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                    v79.clone()
                };
                let v82: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v78, v80)), v13));
                v82.clone()
            }
            US2::US2_ByHabit => {
                let v83: Rc<str> = format_real_22(v10);
                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit adoption ok mode=habits days="); } LIT.with(|lit| lit.clone()) };
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v83));
                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" habits="); } LIT.with(|lit| lit.clone()) };
                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
                let v88: Rc<str> = format_real_22(v45);
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                let v91: Rc<str> = if v55 {
                    v35.clone()
                } else {
                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
                    v90.clone()
                };
                let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v89, v91)), v13));
                v93.clone()
            }
        };
        println!("{}", v95);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        adoption_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_adoption_run() -> i32 {
    closure0()()
}
