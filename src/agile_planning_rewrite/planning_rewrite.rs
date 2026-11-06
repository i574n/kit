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
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_planning ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\nunion task_assignee =\n    | Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee\n\nunion planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\nunion planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\nunion task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning\n\n"); } LIT.with(|lit| lit.clone()) }, v0));
            v7
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v9: i32 = v0.get(v5 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
            let v10: bool = v9 < 0i32;
            let v12: i32 = if v10 {
                let mut v11: i32 = (v0.clone().len() as i32);
                v11
            } else {
                v9
            };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\nunion task_load =\n    | Light :: task_load | Medium :: task_load | Heavy :: task_load\n\nunion task_assignee =\n    | Agent :: task_assignee | Pair :: task_assignee | User :: task_assignee\n\nunion planned_estimate =\n    | Unestimated :: planned_estimate | Minutes :: u32 -> planned_estimate\n\nunion planned_load =\n    | LoadUnset :: planned_load | Load :: task_load -> planned_load\n\nunion planned_assignee =\n    | Unassigned :: planned_assignee | Assigned :: task_assignee -> planned_assignee\n\nunion task_planning =\n    | Planning :: string * planned_estimate * planned_load * planned_assignee -> task_planning"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v12 as usize).unwrap_or(""));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
            let v16: i32 = (v0.clone().len() as i32);
            let mut v17: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v16 as usize).unwrap_or(""));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v17));
            v18
        }
    } else {
        v0
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
                v2 = v9;
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
                let v25: US1 = match v3 {
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
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
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
            let mut v6: i32 = field_end_6(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn planning_tokens_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) }));
    let mut v4: i32 = 0i32;
    let v5: i32 = declared_from_4(v0.clone(), v3, v4);
    let mut v6: bool = v5 < 0i32;
    let (mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>) = if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
        (v7, v8, v9)
    } else {
        let v10: i32 = v5.wrapping_add(10i32);
        let mut v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = 1i32;
        let v13: i32 = field_start_5(v0.clone(), v10, v11, v12);
        let v14: US1 = US1::US1_ScanPlain;
        let v15: i32 = field_end_6(v0.clone(), v13, v11, v14);
        let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
        let mut v17: i32 = 2i32;
        let v18: i32 = field_start_5(v0.clone(), v10, v11, v17);
        let v19: US1 = US1::US1_ScanPlain;
        let v20: i32 = field_end_6(v0.clone(), v18, v11, v19);
        let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v18 as usize..v20 as usize).unwrap_or(""));
        let v22: i32 = 3i32;
        let v23: i32 = field_start_5(v0.clone(), v10, v11, v22);
        let v24: US1 = US1::US1_ScanPlain;
        let v25: i32 = field_end_6(v0.clone(), v23, v11, v24);
        let v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v23 as usize..v25 as usize).unwrap_or(""));
        (v16, v21, v26)
    };
    (v27, v28, v29)
}
fn concat5_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn planning_row_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = std::rc::Rc::<str>::from(v6.to_ascii_lowercase());
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) }, v7));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v8));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Planning ("); } LIT.with(|lit| lit.clone()) }));
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(v0.replace(&*v12, &*v13));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v14.replace(&*v11, &*v15));
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v16.replace(&*v17, &*v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(v22.replace(&*v23, &*v24));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = concat5_10(v11.clone(), v25, v11, v26.clone(), v26);
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v27));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v2));
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v3));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v33));
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
    v35
}
fn planning_upserted_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> US2 {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) }));
    let v7: i32 = v0.find(&*v6).map(|offset| offset as i32).unwrap_or(-1);
    let v8: bool = v7 < 0i32;
    if v8 {
        US2::US2_Invalid
    } else {
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) }, v1.clone()));
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) }));
        let v12: i32 = 0i32;
        let v13: i32 = declared_from_4(v0.clone(), v11, v12);
        let v14: bool = v13 < 0i32;
        if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = v0.get(v7 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v7 as usize) as i32).unwrap_or(-1);
            let mut v17: bool = v16 < 0i32;
            let v19: i32 = if v17 {
                let v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let v20: Rc<str> = planning_row_9(v1, v2, v3, v4);
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v20));
            let v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
            let v24: i32 = (v0.clone().len() as i32);
            let v25: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v24 as usize).unwrap_or(""));
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v25));
            US2::US2_Valid(v26)
        } else {
            let v28: i32 = v0.get(..v13 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v30: i32 = v0.get(v13 as usize..).and_then(|rest| rest.find(&*v29)).map(|found| (found + v13 as usize) as i32).unwrap_or(-1);
            let v31: bool = v30 < 0i32;
            let v33: i32 = if v31 {
                let v32: i32 = (v0.clone().len() as i32);
                v32
            } else {
                v30
            };
            let v34: Rc<str> = planning_row_9(v1, v2, v3, v4);
            let v35: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v28 as usize).unwrap_or(""));
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            let v37: i32 = (v0.clone().len() as i32);
            let v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v33 as usize..v37 as usize).unwrap_or(""));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
            US2::US2_Valid(v39)
        }
    }
}
fn field_token_rewrite_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = planning_schema_ensured_2(v0.clone());
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = planning_tokens_of_3(v0, v1.clone());
    let (mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
        US0::US0_FieldEstimate => {
            (v3, v6, v7)
        }
        _ => {
            let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = match &v2 {
                US0::US0_FieldLoad => {
                    (v5, v3, v7)
                }
                _ => {
                    let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = match v2 {
                        US0::US0_FieldAssignee => {
                            (v5, v6, v3)
                        }
                        _ => unreachable!(),
                    };
                    (v8, v9, v10)
                }
            };
            (v11, v12, v13)
        }
    };
    let mut v17: US2 = planning_upserted_8(v4, v1, v14, v15, v16);
    match v17 {
        US2::US2_Invalid => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
            (2i32, v19)
        }
        US2::US2_Valid(v18) => {
            let v18: Rc<str> = v18;
            (0i32, v18)
        }
    }
}
fn planning_rewrite_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from(v4.to_ascii_lowercase());
    let mut v6: bool = &*v2 == &*v5;
    let v10: US3 = if v6 {
        let v7: US0 = US0::US0_FieldEstimate;
        US3::US3_Valid(v7)
    } else {
        US3::US3_Invalid
    };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = std::rc::Rc::<str>::from(v11.to_ascii_lowercase());
    let v13: bool = &*v2 == &*v12;
    let v16: US3 = if v13 {
        let v14: US0 = US0::US0_FieldLoad;
        US3::US3_Valid(v14)
    } else {
        v10
    };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from(v17.to_ascii_lowercase());
    let v19: bool = &*v2 == &*v18;
    let v22: US3 = if v19 {
        let v20: US0 = US0::US0_FieldAssignee;
        US3::US3_Valid(v20)
    } else {
        v16
    };
    match v22 {
        US3::US3_Invalid => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v26)
        }
        US3::US3_Valid(v23) => {
            let v23: US0 = v23;
            field_token_rewrite_1(v0, v1, v23, v3)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        planning_rewrite_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_rewrite(v0: &str, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
