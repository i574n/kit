#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FieldEstimate,
    US0_FieldLoad,
    US0_FieldAssignee,
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_Valid(Rc<str>),
    US2_Invalid,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US0),
    US3_Invalid,
}
fn planning_schema_ensured_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning ="); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light :: task_load | Medium :: task_load | Heavy :: task_load"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_assignee =\n    | "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v11, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v10))));
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\n"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v21)), v10)), v0));
            v24.clone()
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v26: i32 = v0.get(v5 as usize..).and_then(|rest| rest.find(&*v25)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let v27: bool = v26 < 0i32;
            let v29: i32 = if v27 {
                let v28: i32 = (v0.clone().len() as i32);
                v28
            } else {
                v26
            };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light :: task_load | Medium :: task_load | Heavy :: task_load"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\n"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_assignee =\n    | "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v34, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v33))));
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\n"); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\n"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v33, Rc::<str>::from(format!("{}{}", v43, v44))));
            let v47: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v29 as usize).unwrap_or(""));
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v46));
            let v49: i32 = (v0.clone().len() as i32);
            let v50: Rc<str> = std::rc::Rc::<str>::from(v0.get(v29 as usize..v49 as usize).unwrap_or(""));
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v50));
            v51.clone()
        }
    } else {
        v0.clone()
    }
}
fn declared_from_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
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
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_6(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn planning_tokens_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: i32 = 0i32;
    let v7: i32 = declared_from_4(v0.clone(), v5.clone(), v6);
    let v8: bool = v7 < 0i32;
    let (mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, Rc<str>) = if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
        (v9.clone(), v10.clone(), v11.clone())
    } else {
        let v12: i32 = v7.wrapping_add(10i32);
        let v13: i32 = (v0.clone().len() as i32);
        let v14: i32 = 1i32;
        let v15: i32 = field_start_5(v0.clone(), v12, v13, v14);
        let v16: US1 = US1::US1_ScanPlain;
        let mut v17: i32 = field_end_6(v0.clone(), v15, v13, v16.clone());
        let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v15 as usize..v17 as usize).unwrap_or(""));
        let v19: i32 = 2i32;
        let v20: i32 = field_start_5(v0.clone(), v12, v13, v19);
        let v21: US1 = US1::US1_ScanPlain;
        let v22: i32 = field_end_6(v0.clone(), v20, v13, v21.clone());
        let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
        let v24: i32 = 3i32;
        let v25: i32 = field_start_5(v0.clone(), v12, v13, v24);
        let v26: US1 = US1::US1_ScanPlain;
        let v27: i32 = field_end_6(v0.clone(), v25, v13, v26.clone());
        let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
        (v18.clone(), v23.clone(), v28.clone())
    };
    (v29.clone(), v30.clone(), v31.clone())
}
fn planning_row_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Planning ("); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v15, &v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from(v17.replace(&*v14, &v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = std::rc::Rc::<str>::from(v19.replace(&*v20, &v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = std::rc::Rc::<str>::from(v22.replace(&*v23, &v24));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = std::rc::Rc::<str>::from(v25.replace(&*v26, &v27));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v28, &*v14, &*v29, &*v29].concat());
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v30));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v33, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v32)), v2)), v32)), v3))));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
    v40.clone()
}
fn planning_upserted_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> US2 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v1));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: i32 = v0.find(&*v8).map(|offset| offset as i32).unwrap_or(-1);
    let v10: bool = v9 < 0i32;
    if v10 {
        US2::US2_Invalid
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v1)), v7));
        let v15: i32 = 0i32;
        let v16: i32 = declared_from_4(v0.clone(), v14.clone(), v15);
        let mut v17: bool = v16 < 0i32;
        if v17 {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v19: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v18)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let v20: bool = v19 < 0i32;
            let v22: i32 = if v20 {
                let v21: i32 = (v0.clone().len() as i32);
                v21
            } else {
                v19
            };
            let v23: Rc<str> = planning_row_9(v1.clone(), v2.clone(), v3.clone(), v4.clone());
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v23));
            let v25: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v22 as usize).unwrap_or(""));
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
            let v27: i32 = (v0.clone().len() as i32);
            let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v22 as usize..v27 as usize).unwrap_or(""));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v28));
            US2::US2_Valid(v29.clone())
        } else {
            let v31: i32 = v0.get(..v16 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v33: i32 = v0.get(v16 as usize..).and_then(|rest| rest.find(&*v32)).map(|found| (found + v16 as usize) as i32).unwrap_or(-1);
            let v34: bool = v33 < 0i32;
            let v36: i32 = if v34 {
                let v35: i32 = (v0.clone().len() as i32);
                v35
            } else {
                v33
            };
            let v37: Rc<str> = planning_row_9(v1.clone(), v2.clone(), v3.clone(), v4.clone());
            let v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v31 as usize).unwrap_or(""));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
            let v40: i32 = (v0.clone().len() as i32);
            let v41: Rc<str> = std::rc::Rc::<str>::from(v0.get(v36 as usize..v40 as usize).unwrap_or(""));
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
            US2::US2_Valid(v42.clone())
        }
    }
}
fn field_token_rewrite_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = planning_schema_ensured_2(v0.clone());
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = planning_tokens_of_3(v0.clone(), v1.clone());
    let (mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
        US0::US0_FieldEstimate => {
            (v3.clone(), v6.clone(), v7.clone())
        }
        _ => {
            let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
                US0::US0_FieldLoad => {
                    (v5.clone(), v3.clone(), v7.clone())
                }
                _ => {
                    let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
                        US0::US0_FieldAssignee => {
                            (v5.clone(), v6.clone(), v3.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v8.clone(), v9.clone(), v10.clone())
                }
            };
            (v11.clone(), v12.clone(), v13.clone())
        }
    };
    let mut v17: US2 = planning_upserted_8(v4.clone(), v1.clone(), v14.clone(), v15.clone(), v16.clone());
    match &v17 {
        US2::US2_Invalid => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
            (2i32, v19.clone())
        }
        US2::US2_Valid(v18) => {
            let v18: Rc<str> = v18.clone();
            (0i32, v18.clone())
        }
    }
}
fn planning_rewrite_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
    let mut v6: bool = &*v2 == &*v5;
    let v10: US3 = if v6 {
        let v7: US0 = US0::US0_FieldEstimate;
        US3::US3_Valid(v7.clone())
    } else {
        US3::US3_Invalid
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = std::rc::Rc::<str>::from(v11.to_ascii_lowercase());
    let v13: bool = &*v2 == &*v12;
    let v16: US3 = if v13 {
        let v14: US0 = US0::US0_FieldLoad;
        US3::US3_Valid(v14.clone())
    } else {
        v10.clone()
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from(v17.to_ascii_lowercase());
    let v19: bool = &*v2 == &*v18;
    let v22: US3 = if v19 {
        let v20: US0 = US0::US0_FieldAssignee;
        US3::US3_Valid(v20.clone())
    } else {
        v16.clone()
    };
    match &v22 {
        US3::US3_Invalid => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v26.clone())
        }
        US3::US3_Valid(v23) => {
            let v23: US0 = v23.clone();
            field_token_rewrite_1(v0.clone(), v1.clone(), v23.clone(), v3.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        planning_rewrite_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_rewrite(v0: &str, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
