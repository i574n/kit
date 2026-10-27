#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RollupText,
    US0_RollupJson,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US0),
    US3_Invalid,
}
fn tally_of_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (i32, i32, i32, i32, i32) {
    let (mut v3, mut v4, mut v5, mut v6, mut v7): (i32, i32, i32, i32, i32) = agile_planning_tally::kit_agile_planning_tally(&*v0, &*v1, &*v2);
    (v3, v4, v5, v6, v7)
}
fn tally_json_6(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tasks"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"tasks"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"tasks\":"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_tasks"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v12)), v8));
    let v15: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v11, Rc::<str>::from(format!("{}{}", v14, v15))));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimated"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v18)), v8));
    let v21: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17, Rc::<str>::from(format!("{}{}", v20, v21))));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("minutes"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v24)), v8));
    let v27: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v23, Rc::<str>::from(format!("{}{}", v26, v27))));
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open_minutes"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v30)), v8));
    let v33: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v29, Rc::<str>::from(format!("{}{}", v32, v33))));
    v35.clone()
}
fn tally_columns_7(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { let text = (v0).to_string(); let filler = (5i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v6: Rc<str> = { let text = (v1).to_string(); let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let v8: Rc<str> = { let text = (v2).to_string(); let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = { let text = (v3).to_string(); let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = { let text = (v4).to_string(); let filler = (14i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &text) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    v13
}
fn rollup_line_5(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: bool) -> Rc<str> {
    match &v0 {
        US0::US0_RollupJson => {
            let v17: Rc<str> = if v9 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                v16.clone()
            };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"group\":\""); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v18)), v2));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"value\":\""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v21)), v3));
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            let v26: Rc<str> = tally_json_6(v4, v5, v6, v7, v8);
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            v29.clone()
        }
        US0::US0_RollupText => {
            let v10: Rc<str> = { let text = v2; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v11: Rc<str> = { let text = v3; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
            let v13: Rc<str> = tally_columns_7(v4, v5, v6, v7, v8);
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            v14.clone()
        }
    }
}
fn print_group_rows_3(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: bool) -> i32 {
    loop {
        let v7: i32 = (v4.clone().len() as i32);
        let v8: bool = v7 <= v5;
        if v8 {
            return 0i32;
        } else {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v10: i32 = v4.get(v5 as usize..).and_then(|rest| rest.find(&*v9)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let v11: bool = v10 < 0i32;
            let v12: i32 = if v11 {
                v7
            } else {
                v10
            };
            let v13: Rc<str> = std::rc::Rc::<str>::from(v4.get(v5 as usize..v12 as usize).unwrap_or(""));
            let (mut v14, mut v15, mut v16, mut v17, mut v18): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v2.clone(), v13.clone());
            let v19: Rc<str> = rollup_line_5(v1.clone(), v2.clone(), v3.clone(), v13.clone(), v14, v15, v16, v17, v18, v6);
            let v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let v21: i32 = v12.wrapping_add(1i32);
            let v22: bool = false;
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v21, v22);
            continue;
        }
    }
}
fn field_end_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US1 = match &v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let v6: i32 = field_end_11(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_12(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn epic_ids_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3.clone();
        } else {
            let v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let v12: i32 = 2i32;
                let v13: i32 = field_start_10(v0.clone(), v7, v2, v12);
                let v14: US1 = US1::US1_ScanPlain;
                let v15: i32 = field_end_11(v0.clone(), v13, v2, v14.clone());
                let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
                let v17: bool = &*v16 == "Epic";
                if v17 {
                    let v18: i32 = 0i32;
                    let v19: i32 = field_start_10(v0.clone(), v7, v2, v18);
                    let v20: US1 = US1::US1_ScanPlain;
                    let v21: i32 = field_end_11(v0.clone(), v19, v2, v20.clone());
                    let v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
                    let v23: i32 = (v22.clone().len() as i32);
                    let v24: i32 = v23.wrapping_sub(1i32);
                    let v25: Rc<str> = std::rc::Rc::<str>::from(v22.get(1i32 as usize..v24 as usize).unwrap_or(""));
                    let v26: bool = &*v3 == "";
                    let v30: Rc<str> = if v26 {
                        v25.clone()
                    } else {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v27)), v25));
                        v29.clone()
                    };
                    (v0, v1, v2, v3) = (v0.clone(), v7, v2, v30.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3.clone());
                continue;
            }
        }
    }
}
fn planning_epic_ids_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = (v0.clone().len() as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    epic_ids_from_9(v0.clone(), v1, v2, v3.clone())
}
fn print_rollup_2(mut v0: Rc<str>, mut v1: US0, mut v2: u64) -> i32 {
    let v12: Rc<str> = match &v1 {
        US0::US0_RollupJson => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"groups\":["); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_RollupText => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("GROUP"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = { let text = v3; let filler = (11i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("VALUE"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { let text = v5; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TASKS   OPEN  ESTIMATED  MINUTES  OPEN_MINUTES"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            v9.clone()
        }
    };
    let v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planned,Active,Blocked,Paused,Done"); } LIT.with(|lit| lit.clone()) };
    let v16: i32 = 0i32;
    let v17: bool = true;
    let v18: i32 = print_group_rows_3(v0.clone(), v1.clone(), v14.clone(), v14.clone(), v15.clone(), v16, v17);
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("light,medium,heavy,none"); } LIT.with(|lit| lit.clone()) };
    let v21: i32 = 0i32;
    let v22: bool = false;
    let v23: i32 = print_group_rows_3(v0.clone(), v1.clone(), v19.clone(), v19.clone(), v20.clone(), v21, v22);
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent,pair,user,none"); } LIT.with(|lit| lit.clone()) };
    let v26: i32 = 0i32;
    let v27: bool = false;
    let v28: i32 = print_group_rows_3(v0.clone(), v1.clone(), v24.clone(), v24.clone(), v25.clone(), v26, v27);
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("epic+deps"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = planning_epic_ids_8(v0.clone());
    let v32: i32 = 0i32;
    let v33: bool = false;
    let v34: i32 = print_group_rows_3(v0.clone(), v1.clone(), v29.clone(), v30.clone(), v31.clone(), v32, v33);
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v37, mut v38, mut v39, mut v40, mut v41): (i32, i32, i32, i32, i32) = tally_of_4(v0.clone(), v35.clone(), v36.clone());
    let v77: Rc<str> = match &v1 {
        US0::US0_RollupJson => {
            let v66: Rc<str> = tally_json_6(v37, v38, v39, v40, v41);
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v66));
            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
            let v71: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let v72: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v72));
            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
            v75.clone()
        }
        US0::US0_RollupText => {
            let v42: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile plan ok tasks="); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" open="); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
            let v47: Rc<str> = std::rc::Rc::<str>::from((v38).to_string());
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimated="); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            let v51: Rc<str> = std::rc::Rc::<str>::from((v39).to_string());
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
            let v54: Rc<str> = std::rc::Rc::<str>::from((v40).to_string());
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" open_minutes="); } LIT.with(|lit| lit.clone()) };
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            let v58: Rc<str> = std::rc::Rc::<str>::from((v41).to_string());
            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v50, Rc::<str>::from(format!("{}{}", v57, v58))));
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
            let v63: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v2);
            let v64: Rc<str> = std::rc::Rc::<str>::from((v63).to_string());
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v64));
            v65.clone()
        }
    };
    let v78: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v77.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v78
}
fn planning_rollup_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let v9: US2 = if v6 {
        US2::US2_FileReadable(v5.clone())
    } else {
        US2::US2_FileMissing(v5.clone())
    };
    match &v9 {
        US2::US2_FileMissing(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US2::US2_FileReadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            let v17: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v16);
            print_rollup_2(v17.clone(), v1.clone(), v2)
        }
    }
}
fn command_line_argument_count_14() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_13() -> i32 {
    let v0: i32 = command_line_argument_count_14();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn planning_rollup_entry_0() -> i32 {
    let v0: i32 = argument_count_13();
    let v1: bool = v0 == 2i32;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: US0 = US0::US0_RollupText;
        planning_rollup_1(v3.clone(), v4.clone())
    } else {
        let v6: bool = v0 == 3i32;
        if v6 {
            let v7: i32 = 2i32;
            let v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v9: bool = &*v8 == "--json";
            let v13: US3 = if v9 {
                let v10: US0 = US0::US0_RollupJson;
                US3::US3_Valid(v10.clone())
            } else {
                US3::US3_Invalid
            };
            match &v13 {
                US3::US3_Invalid => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
                    let v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v21
                }
                US3::US3_Valid(v14) => {
                    let v14: US0 = v14.clone();
                    let v15: i32 = 1i32;
                    let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    planning_rollup_1(v16.clone(), v14.clone())
                }
            }
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile plan rejected: plan expects <root> [--json]"); } LIT.with(|lit| lit.clone()) };
            let v27: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v26 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v27
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        planning_rollup_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_rollup_run() -> i32 {
    closure0()()
}
