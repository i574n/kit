#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US1 {
    US1_LabAudit(Rc<str>, Rc<str>),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_LabParsed(US1),
    US0_LabRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Keep,
    US2_Orphan(Rc<str>),
    US2_DoneItem,
    US2_Stale(u64),
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
fn now_ms_1() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_at_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn body_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3
}
fn lab_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let v3: Rc<str> = field_at_5(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("against"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_6(v0.clone(), v1, v4);
    let v6: i32 = 3i32;
    let v7: Rc<str> = body_at_7(v0.clone(), v1, v6);
    let v8: bool = &*v3 == "";
    if v8 {
        let v9: US1 = US1::US1_Blank;
        US0::US0_LabParsed(v9)
    } else {
        let v11: bool = &*v3 != "query";
        let v15: bool = if v11 {
            true
        } else {
            let v12: i32 = 1i32;
            let v13: Rc<str> = field_at_5(v0.clone(), v1, v12);
            let v14: bool = &*v13 != "lab";
            v14
        };
        if v15 {
            let v16: i32 = 1i32;
            let v17: Rc<str> = field_at_5(v0, v1, v16);
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with lab yet; run it as its own call"); } LIT.with(|lit| lit.clone()) }));
            US0::US0_LabRejected(v18)
        } else {
            let v20: i32 = 2i32;
            let v21: Rc<str> = body_at_7(v0.clone(), v1, v20);
            let v22: bool = &*v21 != "audit";
            if v22 {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab: expected lab audit <folder> against <backlog>"); } LIT.with(|lit| lit.clone()) };
                US0::US0_LabRejected(v23)
            } else {
                let v25: bool = &*v7 == "";
                let v27: bool = if v25 {
                    true
                } else {
                    let v26: bool = &*v7 == "against";
                    v26
                };
                if v27 {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: expected the lab folder"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_LabRejected(v28)
                } else {
                    let v30: bool = v5 < 0i32;
                    let v34: bool = if v30 {
                        true
                    } else {
                        let v31: i32 = v5.wrapping_add(1i32);
                        let v32: Rc<str> = body_at_7(v0.clone(), v1, v31);
                        let v33: bool = &*v32 == "";
                        v33
                    };
                    if v34 {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: expected against <backlog root>"); } LIT.with(|lit| lit.clone()) };
                        US0::US0_LabRejected(v35)
                    } else {
                        let v37: i32 = v5.wrapping_add(1i32);
                        let v38: Rc<str> = body_at_7(v0, v1, v37);
                        let v39: US1 = US1::US1_LabAudit(v7, v38);
                        US0::US0_LabParsed(v39)
                    }
                }
            }
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US0 = lab_parsed_4(v0.clone(), v1);
            match v4 {
                US0::US0_LabParsed(v6) => {
                    let v6: US1 = v6;
                    let v7: i32 = v1.wrapping_add(1i32);
                    v1 = v7;
                    continue;
                }
                US0::US0_LabRejected(v5) => {
                    let v5: Rc<str> = v5;
                    return v5;
                }
            }
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v11;
        }
    }
}
fn empty_lab_tally_8() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn backlog_items_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let listed = std::env::current_exe().and_then(|program| std::process::Command::new(program).current_dir(&*v0).args(["agile", "list", &*v1, "all", "--json"]).stdin(std::process::Stdio::null()).output()); std::rc::Rc::<str>::from(match listed { Ok(output) if output.status.success() => serde_json::from_slice::<serde_json::Value>(&output.stdout).ok().and_then(|value| value.get("tasks").and_then(|tasks| tasks.as_array()).map(|tasks| tasks.iter().map(|task| format!("{}{tab}{}{tab}{}{tab}{}", task["id"].as_str().unwrap_or(""), task["status"].as_str().unwrap_or(""), task["progress"].as_u64().unwrap_or(0), task["last_note"].as_str().unwrap_or(""), tab = char::from(9u8))).collect::<Vec<_>>().join("
"))).unwrap_or_else(|| String::from("error: agile list printed no tasks")), Ok(output) => format!("error: {}", String::from_utf8_lossy(&output.stderr).trim().trim_start_matches("kit error: ")), Err(error) => format!("error: agile list: {error}") }) };
    v2
}
fn lines_joined_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1
        } else {
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
            v5
        }
    }
}
fn lab_directories_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let mut names = std::fs::read_dir(std::path::Path::new(&*v0).join(&*v1)).into_iter().flatten().flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).collect::<Vec<_>>(); names.sort(); std::rc::Rc::<str>::from(names.join("
")) };
    v2
}
fn row_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    v4
}
fn go_18(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v5.ends_with(&*v7);
            let v9: bool = v8 == false;
            let v15: Rc<str> = if v9 {
                v5
            } else {
                let v10: bool = v6 == 1i32;
                if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11
                } else {
                    let v12: i32 = v6.wrapping_sub(2i32);
                    let v13: Rc<str> = string_slice(&v5, 0i32 as i64, v12 as i64);
                    v13
                }
            };
            let v16: i32 = v2.wrapping_add(1i32);
            let v17: bool = &*v15 == "";
            let v19: i32 = if v17 {
                v3
            } else {
                let v18: i32 = v3.wrapping_add(1i32);
                v18
            };
            (v2, v3) = (v16, v19);
            continue;
        }
    }
}
fn line_count_17(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let v6: Rc<str> = v2.borrow()[v5 as usize].clone();
        let v7: bool = &*v6 == "";
        v7
    } else {
        false
    };
    let v10: i32 = if v8 {
        let v9: i32 = v3.wrapping_sub(1i32);
        v9
    } else {
        v3
    };
    let v11: i32 = 0i32;
    let v12: i32 = 0i32;
    let v13: i32 = go_18(&v2, v10, v11, v12);
    let v14: u64 = (v13 as u64);
    v14
}
fn go_20(mut v0: i32, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> (i32, Rc<str>) {
    loop {
        let v6: bool = v3 >= v2;
        if v6 {
            return (v4, v5);
        } else {
            let v7: Rc<str> = v1.borrow()[v3 as usize].clone();
            let v8: i32 = (v7.clone().len() as i32);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v7.ends_with(&*v9);
            let v11: bool = v10 == false;
            let v17: Rc<str> = if v11 {
                v7
            } else {
                let v12: bool = v8 == 1i32;
                if v12 {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v13
                } else {
                    let v14: i32 = v8.wrapping_sub(2i32);
                    let v15: Rc<str> = string_slice(&v7, 0i32 as i64, v14 as i64);
                    v15
                }
            };
            let v18: i32 = v3.wrapping_add(1i32);
            let v19: bool = &*v17 == "";
            let (mut v23, mut v24): (i32, Rc<str>) = if v19 {
                (v4, v5)
            } else {
                let v20: i32 = v4.wrapping_add(1i32);
                let v21: bool = v4 == v0;
                let v22: Rc<str> = if v21 {
                    v17
                } else {
                    v5
                };
                (v20, v22)
            };
            (v3, v4, v5) = (v18, v23, v24);
            continue;
        }
    }
}
fn line_at_19(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: i32 = (v1 as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 > 0i32;
    let v10: bool = if v6 {
        let v7: i32 = v5.wrapping_sub(1i32);
        let v8: Rc<str> = v4.borrow()[v7 as usize].clone();
        let v9: bool = &*v8 == "";
        v9
    } else {
        false
    };
    let v12: i32 = if v10 {
        let v11: i32 = v5.wrapping_sub(1i32);
        v11
    } else {
        v5
    };
    let v13: i32 = 0i32;
    let v14: i32 = 0i32;
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v16, mut v17): (i32, Rc<str>) = go_20(v2, &v4, v12, v13, v14, v15);
    v17
}
fn item_id_of_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_uppercase().replace(char::from(95u8), "-"));
    v1
}
fn item_of_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().find(|line| line.split(char::from(9u8)).next() == Some(&*v1)).unwrap_or(""));
    v2
}
fn tab_part_23(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn note_age_days_24(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = { let stamp = (&*v0).trim(); let digits = |from: usize, to: usize| stamp.get(from..to).and_then(|part| part.parse::<i64>().ok()); match (digits(0, 4), digits(5, 7), digits(8, 10)) { (Some(year), Some(month), Some(day)) => { let shifted = if month <= 2 { year - 1 } else { year }; let era = shifted.div_euclid(400); let of_era = shifted - era * 400; let of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1; let days = era * 146097 + of_era * 365 + of_era / 4 - of_era / 100 + of_year - 719468; let today = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64 / 86400).unwrap_or(0); (today - days).max(0) as u64 } _ => 0 } };
    v1
}
fn verdict_of_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> US2 {
    let v3: bool = &*v0 == "";
    if v3 {
        US2::US2_Orphan(v1)
    } else {
        let v5: i32 = 1i32;
        let v6: Rc<str> = tab_part_23(v0, v5);
        let v7: bool = &*v6 == "Done";
        if v7 {
            US2::US2_DoneItem
        } else {
            let v9: bool = v2 > 14u64;
            if v9 {
                US2::US2_Stale(v2)
            } else {
                US2::US2_Keep
            }
        }
    }
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn verdict_text_26(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_DoneItem => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done: promote or delete"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_Keep => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("keep"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_Orphan(v2) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan: no backlog item "); } LIT.with(|lit| lit.clone()) }, v2));
            v3
        }
        US2::US2_Stale(v5) => {
            let v5: u64 = v5;
            let v6: Rc<str> = decimal_text_27(v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale: no note for "); } LIT.with(|lit| lit.clone()) }, v6));
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" days"); } LIT.with(|lit| lit.clone()) }));
            v8
        }
    }
}
fn counted_28(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: US2) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
        US2::US2_Keep => {
            let v10: u64 = v2.wrapping_add(1u64);
            let v11: u64 = v3.wrapping_add(1u64);
            (v0, v1, v10, v11, v4, v5, v6, v7, v8)
        }
        _ => {
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
                US2::US2_Orphan(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: u64 = v2.wrapping_add(1u64);
                    let v14: u64 = v4.wrapping_add(1u64);
                    (v0, v1, v13, v3, v14, v5, v6, v7, v8)
                }
                _ => {
                    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v9 {
                        US2::US2_DoneItem => {
                            let v15: u64 = v2.wrapping_add(1u64);
                            let v16: u64 = v5.wrapping_add(1u64);
                            (v0, v1, v15, v3, v4, v16, v6, v7, v8)
                        }
                        _ => {
                            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match v9 {
                                US2::US2_Stale(v17) => {
                                    let v17: u64 = v17;
                                    let v18: u64 = v2.wrapping_add(1u64);
                                    let v19: u64 = v6.wrapping_add(1u64);
                                    (v0, v1, v18, v3, v4, v5, v19, v7, v8)
                                }
                                _ => unreachable!(),
                            };
                            (v20, v21, v22, v23, v24, v25, v26, v27, v28)
                        }
                    };
                    (v29, v30, v31, v32, v33, v34, v35, v36, v37)
                }
            };
            (v38, v39, v40, v41, v42, v43, v44, v45, v46)
        }
    };
    (v47, v48, v49, v50, v51, v52, v53, v54, v55)
}
fn aligned_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1
}
fn labs_rows_16(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let v15: u64 = line_count_17(v3.clone());
    let v16: bool = v4 < v15;
    let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v16 {
        let v17: Rc<str> = line_at_19(v3.clone(), v4);
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17.clone()));
        let v20: Rc<str> = item_id_of_21(v17);
        let v21: Rc<str> = item_of_22(v2.clone(), v20.clone());
        let v22: i32 = 3i32;
        let v23: Rc<str> = tab_part_23(v21.clone(), v22);
        let v24: u64 = note_age_days_24(v23);
        let v25: US2 = verdict_of_25(v21.clone(), v20.clone(), v24);
        let v26: bool = &*v21 == "";
        let v42: Rc<str> = if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = verdict_text_26(v25.clone());
            let v29: Rc<str> = row_15(v27.clone(), v28);
            let v30: Rc<str> = row_15(v27.clone(), v29);
            let v31: Rc<str> = row_15(v27, v30);
            row_15(v19, v31)
        } else {
            let v33: i32 = 1i32;
            let v34: Rc<str> = tab_part_23(v21.clone(), v33);
            let v35: i32 = 2i32;
            let v36: Rc<str> = tab_part_23(v21, v35);
            let v37: Rc<str> = verdict_text_26(v25.clone());
            let v38: Rc<str> = row_15(v36, v37);
            let v39: Rc<str> = row_15(v34, v38);
            let v40: Rc<str> = row_15(v20, v39);
            row_15(v19, v40)
        };
        let v43: u64 = v4.wrapping_add(1u64);
        let v44: Rc<str> = lines_joined_13(v5, v42);
        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = counted_28(v6, v7, v8, v9, v10, v11, v12, v13, v14, v25);
        let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_rows_16(v0, v1, v2, v3, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53);
        (v54, v55, v56, v57, v58, v59, v60, v61, v62)
    } else {
        let v63: Rc<str> = aligned_29(v5);
        let v64: Rc<str> = lines_joined_13(v6, v63);
        (v64, v7, v8, v9, v10, v11, v12, v13, v14)
    };
    (v65, v66, v67, v68, v69, v70, v71, v72, v73)
}
fn non_portable_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let (b, q, a) = (char::from(92u8), char::from(34u8), char::from(39u8)); let pattern = regex::Regex::new(&format!("[A-Za-z]:[{b}{b}/]Users[{b}{b}/][^{b}s{q}{a}<>|]+|(?:^|[{b}s{q}{a}=(])/home/[a-z_][a-z0-9_-]*/[^{b}s{q}{a}<>|]+")).unwrap(); fn files(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) { if path.is_dir() { let mut entries = std::fs::read_dir(path).into_iter().flatten().flatten().map(|entry| entry.path()).collect::<Vec<_>>(); entries.sort(); for entry in entries { files(&entry, found); } } else { found.push(path.to_path_buf()); } } let root = std::path::Path::new(&*v0); let mut found = Vec::new(); files(&root.join(&*v1), &mut found); let mut rows = Vec::new(); for path in found { let text = String::from_utf8_lossy(&std::fs::read(&path).unwrap_or_default()).into_owned(); let shown = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace(b, "/"); for (index, line) in text.lines().enumerate() { for hit in pattern.find_iter(line) { let text = hit.as_str(); let text = if text.get(1..2) == Some(":") { text } else { text.trim_start_matches(|c: char| c != char::from(47u8)) }; rows.push(format!("{}:{}  non-portable path {}", shown, index + 1, text)); } } } std::rc::Rc::<str>::from(rows.join("
")) };
    v2
}
fn ignored_paths_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let marker = format!("{0}{0} ", char::from(33u8)); let listed = std::process::Command::new("git").current_dir(&*v0).args(["status", "--porcelain", "--ignored=matching", "--untracked-files=all", "--"]).arg(&*v1).stdin(std::process::Stdio::null()).output(); std::rc::Rc::<str>::from(match listed { Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).lines().filter_map(|line| line.strip_prefix(marker.as_str())).map(|path| path.trim_matches(char::from(34u8)).trim_end_matches(char::from(47u8)).to_owned()).filter(|path| !path.split(char::from(47u8)).any(|part| part == ".runs")).map(|path| format!("{path}  ignored by .gitignore (run output outside .runs)")).collect::<Vec<_>>().join("
"), _ => String::new() }) };
    v2
}
fn audit_run_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let v12: Rc<str> = backlog_items_12(v0.clone(), v2);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let v14: bool = v12.starts_with(&*v13);
    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v14 {
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab audit: "); } LIT.with(|lit| lit.clone()) }, v12));
        let v16: Rc<str> = lines_joined_13(v3, v15);
        (v16, true, v5, v6, v7, v8, v9, v10, v11)
    } else {
        let v17: Rc<str> = lab_directories_14(v0.clone(), v1.clone());
        let v18: u64 = 0u64;
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = row_15(v22, v23);
        let v25: Rc<str> = row_15(v21, v24);
        let v26: Rc<str> = row_15(v20, v25);
        let v27: Rc<str> = row_15(v19, v26);
        let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_rows_16(&v0, v1.clone(), v12, v17, v18, v27, v3, v4, v5, v6, v7, v8, v9, v10, v11);
        let v37: Rc<str> = non_portable_30(v0.clone(), v1.clone());
        let v38: Rc<str> = ignored_paths_31(v0, v1);
        let v39: Rc<str> = lines_joined_13(v28, v37.clone());
        let v40: Rc<str> = lines_joined_13(v39, v38.clone());
        let v41: u64 = line_count_17(v37);
        let v42: u64 = v35.wrapping_add(v41);
        let v43: u64 = line_count_17(v38);
        let v44: u64 = v36.wrapping_add(v43);
        (v40, v29, v30, v31, v32, v33, v34, v42, v44)
    };
    (v45, v46, v47, v48, v49, v50, v51, v52, v53)
}
fn step_run_10(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
        US1::US1_LabAudit(v11, v12) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            audit_run_11(v0, v11, v12, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match v1 {
                US1::US1_Blank => {
                    (v2, v3, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => unreachable!(),
            };
            (v22, v23, v24, v25, v26, v27, v28, v29, v30)
        }
    };
    (v31, v32, v33, v34, v35, v36, v37, v38, v39)
}
fn labs_run_9(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) {
    let v12: i32 = v1.len() as i32;
    let v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = if v13 {
        let v14: US0 = lab_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match &v14 {
            US0::US0_LabParsed(v15) => {
                let v15: US1 = v15.clone();
                let v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = step_run_10(v0.clone(), v15, v3, v4, v5, v6, v7, v8, v9, v10, v11);
                labs_run_9(v0, v1, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = match v14 {
                    US0::US0_LabRejected(v35) => {
                        let v35: Rc<str> = v35;
                        let v36: i32 = v2.wrapping_add(1i32);
                        labs_run_9(v0, v1, v36, v3, v4, v5, v6, v7, v8, v9, v10, v11)
                    }
                    _ => unreachable!(),
                };
                (v46, v47, v48, v49, v50, v51, v52, v53, v54)
            }
        };
        (v55, v56, v57, v58, v59, v60, v61, v62, v63)
    } else {
        (v3, v4, v5, v6, v7, v8, v9, v10, v11)
    };
    (v64, v65, v66, v67, v68, v69, v70, v71, v72)
}
fn receipt_counts_32(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64) -> Rc<str> {
    let v9: Rc<str> = decimal_text_27(v2);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs "); } LIT.with(|lit| lit.clone()) }, v9));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  keep "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = decimal_text_27(v3);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = decimal_text_27(v4);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  orphan "); } LIT.with(|lit| lit.clone()) }, v14));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  done "); } LIT.with(|lit| lit.clone()) }));
    let v17: Rc<str> = decimal_text_27(v5);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stale "); } LIT.with(|lit| lit.clone()) }));
    let v21: Rc<str> = decimal_text_27(v6);
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  non-portable "); } LIT.with(|lit| lit.clone()) }));
    let v24: Rc<str> = decimal_text_27(v7);
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: bool = v8 == 0u64;
    if v26 {
        v25
    } else {
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  ignored "); } LIT.with(|lit| lit.clone()) }));
        let v28: Rc<str> = decimal_text_27(v8);
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
        v29
    }
}
fn elapsed_text_34(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_36() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_35() -> Rc<str> {
    let v0: i64 = local_offset_minutes_36();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_33(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US3::US3_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US3::US3_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_34(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_35();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn lab_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = empty_lab_tally_8();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64) = labs_run_9(v0, v3, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
        let v26: bool = &*v17 == "";
        if v26 {
            ()
        } else {
            println!("{}", v17);
            ()
        };
        let v32: bool = if v18 {
            true
        } else {
            let v27: u64 = v21.wrapping_add(v22);
            let v28: u64 = v27.wrapping_add(v23);
            let v29: u64 = v28.wrapping_add(v24);
            let v30: u64 = v29.wrapping_add(v25);
            let v31: bool = v30 > 0u64;
            v31
        };
        if v32 {
            let v33: US3 = US3::US3_Failed;
            let v34: Rc<str> = receipt_counts_32(&v17, v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_33(v33, v34, v2);
            1i32
        } else {
            let v35: US3 = US3::US3_Succeeded;
            let v36: Rc<str> = receipt_counts_32(&v17, v18, v19, v20, v21, v22, v23, v24, v25);
            receipt_line_33(v35, v36, v2);
            0i32
        }
    } else {
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) }, v5));
        println!("{}", v38);
        let v39: US3 = US3::US3_Failed;
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("labs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_33(v39, v40, v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        lab_program_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_lab_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
