#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_MainAgent,
    US0_Subagent,
}
#[derive(Clone)]
enum US1 {
    US1_NextText,
    US1_NextJson,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_ScanPlain,
    US3_ScanQuoted,
    US3_ScanEscaped,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoTasks,
    UH0_TaskThen(Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRanked,
    UH1_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<UH1>),
}
fn field_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US3) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US3::US3_ScanPlain => {
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
                let v25: US3 = match &v3 {
                    US3::US3_ScanEscaped => {
                        US3::US3_ScanQuoted
                    }
                    US3::US3_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US3::US3_ScanQuoted
                        } else {
                            US3::US3_ScanPlain
                        }
                    }
                    US3::US3_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US3::US3_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US3::US3_ScanPlain
                            } else {
                                US3::US3_ScanQuoted
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
fn skip_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn top_level_field_count_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: US3 = US3::US3_ScanPlain;
        let v5: i32 = field_end_4(v0.clone(), v1, v2, v4.clone());
        let v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v8: bool = v7 == 41i32;
            if v8 {
                let v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let v10: i32 = v5.wrapping_add(1i32);
                let v11: i32 = skip_spaces_5(v0.clone(), v10);
                let v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v8.starts_with(&*v9);
            if v10 {
                let v11: i32 = 0i32;
                let v12: i32 = top_level_field_count_3(v0.clone(), v6, v2, v11);
                let v13: bool = v12 == 11i32;
                if v13 {
                    (v0, v1, v2) = (v0.clone(), v6, v2);
                    continue;
                } else {
                    return v4;
                }
            } else {
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        }
    }
}
fn field_start_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US3 = US3::US3_ScanPlain;
            let v6: i32 = field_end_4(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_5(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn backlog_task_at_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) {
    let v3: i32 = 0i32;
    let v4: i32 = field_start_8(v0.clone(), v1, v2, v3);
    let v5: US3 = US3::US3_ScanPlain;
    let v6: i32 = field_end_4(v0.clone(), v4, v2, v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_8(v0.clone(), v1, v2, v11);
    let v13: US3 = US3::US3_ScanPlain;
    let v14: i32 = field_end_4(v0.clone(), v12, v2, v13.clone());
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 3i32;
    let v20: i32 = field_start_8(v0.clone(), v1, v2, v19);
    let v21: US3 = US3::US3_ScanPlain;
    let v22: i32 = field_end_4(v0.clone(), v20, v2, v21.clone());
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = (v23).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v25: i32 = 4i32;
    let v26: i32 = field_start_8(v0.clone(), v1, v2, v25);
    let v27: US3 = US3::US3_ScanPlain;
    let v28: i32 = field_end_4(v0.clone(), v26, v2, v27.clone());
    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
    let v30: i32 = (v29).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v31: i32 = 6i32;
    let v32: i32 = field_start_8(v0.clone(), v1, v2, v31);
    let v33: US3 = US3::US3_ScanPlain;
    let v34: i32 = field_end_4(v0.clone(), v32, v2, v33.clone());
    let v35: Rc<str> = std::rc::Rc::<str>::from(v0.get(v32 as usize..v34 as usize).unwrap_or(""));
    let v36: i32 = 7i32;
    let v37: i32 = field_start_8(v0.clone(), v1, v2, v36);
    let v38: US3 = US3::US3_ScanPlain;
    let v39: i32 = field_end_4(v0.clone(), v37, v2, v38.clone());
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = (v40.clone().len() as i32);
    let v42: i32 = v41.wrapping_sub(1i32);
    let v43: Rc<str> = std::rc::Rc::<str>::from(v40.get(1i32 as usize..v42 as usize).unwrap_or(""));
    let v44: i32 = 8i32;
    let v45: i32 = field_start_8(v0.clone(), v1, v2, v44);
    let v46: US3 = US3::US3_ScanPlain;
    let v47: i32 = field_end_4(v0.clone(), v45, v2, v46.clone());
    let v48: Rc<str> = std::rc::Rc::<str>::from(v0.get(v45 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = (v48.clone().len() as i32);
    let v50: i32 = v49.wrapping_sub(1i32);
    let v51: Rc<str> = std::rc::Rc::<str>::from(v48.get(1i32 as usize..v50 as usize).unwrap_or(""));
    let v52: i32 = agile_planning_fields::kit_agile_planning_minutes_near(&*v0, &*v10, v1);
    let v53: i32 = 5i32;
    let v54: i32 = field_start_8(v0.clone(), v1, v2, v53);
    let v55: US3 = US3::US3_ScanPlain;
    let v56: i32 = field_end_4(v0.clone(), v54, v2, v55.clone());
    let v57: Rc<str> = std::rc::Rc::<str>::from(v0.get(v54 as usize..v56 as usize).unwrap_or(""));
    let v58: i32 = (v57).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    (v10.clone(), v18.clone(), v24, v30, v35.clone(), v43.clone(), v51.clone(), v52, v58)
}
fn tasks_from_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
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
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) = backlog_task_at_7(v0.clone(), v7, v2);
                let v21: Rc<UH0> = Rc::new(UH0::UH0_TaskThen(v12.clone(), v13.clone(), v14, v15, v16.clone(), v17.clone(), v18.clone(), v19, v20, v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v21.clone());
                continue;
            } else {
                (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3.clone());
                continue;
            }
        }
    }
}
fn load_words_9(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2.clone()
}
fn loads_pass_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32) -> bool {
    loop {
        let v4: i32 = (v2.clone().borrow().len() as i32);
        let v5: bool = v4 <= v3;
        if v5 {
            return false;
        } else {
            let v6: Rc<str> = v2.clone().borrow()[v3 as usize].clone();
            let v7: bool = agile_planning_filters::kit_agile_planning_selected(&*v0, &*v1, &*v6, "all");
            if v7 {
                return true;
            } else {
                let v8: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v8);
                continue;
            }
        }
    }
}
fn closure1(mut v0: US0, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> {
    Rc::new(move |mut v6: Rc<str>, mut v7: Rc<str>, mut v8: i32, mut v9: i32, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: i32, mut v14: i32| -> bool {
        let v15: bool = &*v2 == "";
        let v17: bool = if v15 {
            true
        } else {
            let v16: bool = agile_task_query::kit_query_passes(&*v2, &*v6, &*v7, &*v12, &*v10, true);
            v16
        };
        if v17 {
            let v18: Rc<RefCell<Vec<Rc<str>>>> = load_words_9(v3.clone());
            let v19: i32 = 0i32;
            loads_pass_10(v5.clone(), v6.clone(), v18.clone(), v19)
        } else {
            false
        }
    })
}
fn task_open_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: i32) -> bool {
    let v9: bool = &*v4 == "Planned";
    if v9 {
        true
    } else {
        let v10: bool = &*v4 == "Active";
        if v10 {
            true
        } else {
            let v11: bool = &*v4 == "Blocked";
            if v11 {
                false
            } else {
                let v12: bool = &*v4 == "Paused";
                if v12 {
                    false
                } else {
                    false
                }
            }
        }
    }
}
fn status_of_14(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2.clone();
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let v6: i32 = *v6;
                let v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let v10: i32 = *v10;
                let v11: i32 = *v11;
                let v12: Rc<UH0> = v12.clone();
                let v13: bool = &*v3 == &*v1;
                if v13 {
                    return v7.clone();
                } else {
                    (v0, v1) = (v12.clone(), v1.clone());
                    continue;
                }
            }
        }
    }
}
fn deps_settled_13(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = std::rc::Rc::<str>::from(v9.trim());
            let v11: Rc<str> = status_of_14(v0.clone(), v10.clone());
            let v12: bool = &*v11 == "";
            let v14: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                v13
            };
            if v14 {
                let v15: i32 = v8.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v15);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v7: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let v9: i32 = if v8 {
                v4
            } else {
                v7
            };
            let v10: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v9 as usize).unwrap_or(""));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
            let v12: Rc<str> = status_of_14(v0.clone(), v11.clone());
            let v13: i32 = v9.wrapping_add(1i32);
            let v14: bool = &*v12 == "";
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = &*v12 == "Done";
                v15
            };
            let v18: i32 = if v16 {
                v3
            } else {
                let v17: i32 = v3.wrapping_add(1i32);
                v17
            };
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v13, v18);
            continue;
        }
    }
}
fn unblocked_weight_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let v6: i32 = *v6;
                let v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let v10: i32 = *v10;
                let v11: i32 = *v11;
                let v12: Rc<UH0> = v12.clone();
                let v13: bool = task_open_12(v3.clone(), v4.clone(), v5, v6, v7.clone(), v8.clone(), v9.clone(), v10, v11);
                let v20: bool = if v13 {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = std::rc::Rc::<str>::from(v8.replace(&*v14, &v15));
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v17, &v15));
                    let v19: bool = v18.split("|").any(|item| item == &*v1);
                    v19
                } else {
                    false
                };
                let v28: i32 = if v20 {
                    let v21: i32 = 0i32;
                    let v22: i32 = 0i32;
                    let v23: i32 = open_deps_17(v0.clone(), v8.clone(), v21, v22);
                    let v24: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
                    let v25: bool = v23 < 1i32;
                    let v26: i32 = if v25 {
                        1i32
                    } else {
                        v23
                    };
                    let v27: i32 = v24.wrapping_div(v26);
                    v27
                } else {
                    0i32
                };
                let v29: i32 = v2.wrapping_add(v28);
                (v0, v1, v2) = (v12.clone(), v1.clone(), v29);
                continue;
            }
        }
    }
}
fn ranked_of_15(mut v0: US0, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) {
    let v11: bool = 0i32 < v9;
    let v29: i32 = if v11 {
        v9
    } else {
        let v12: bool = 0i32 == v5;
        let v27: i32 = if v12 {
            15i32
        } else {
            let v13: bool = 1i32 == v5;
            if v13 {
                30i32
            } else {
                let v14: bool = 2i32 == v5;
                if v14 {
                    60i32
                } else {
                    let v15: bool = 3i32 == v5;
                    if v15 {
                        180i32
                    } else {
                        let v16: bool = 4i32 == v5;
                        if v16 {
                            300i32
                        } else {
                            let v17: bool = 5i32 == v5;
                            if v17 {
                                600i32
                            } else {
                                let v18: bool = 6i32 == v5;
                                if v18 {
                                    1200i32
                                } else {
                                    let v19: bool = 7i32 == v5;
                                    if v19 {
                                        2400i32
                                    } else {
                                        3600i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let v28: i32 = ((v27 as i64) * (100 - (v10).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v28
    };
    let v30: i32 = 0i32;
    let v31: i32 = unblocked_weight_16(v1, v2.clone(), v30);
    let v32: i32 = 1i32 << (2 * (5 - (v4).clamp(0, 5)));
    let v33: i32 = v32.wrapping_add(v31);
    let v39: i32 = match &v0 {
        US0::US0_MainAgent => {
            let v34: i32 = ((v33 as i64) * 6000 / (v29).max(1) as i64).min(i32::MAX as i64) as i32;
            v34
        }
        US0::US0_Subagent => {
            let v35: i32 = v29.wrapping_mul(8i32);
            let v36: i32 = v35.wrapping_add(5i32);
            let v37: i32 = v36.wrapping_sub(v4);
            v37
        }
    };
    (v2, v3, v4, v5, v29, v11, v31, v39)
}
fn inserted_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<UH1>) -> Rc<UH1> {
    match &*v8 {
        UH1::UH1_NoRanked => {
            let v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_RankedThen(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v9.clone()))
        }
        UH1::UH1_RankedThen(v11, v12, v13, v14, v15, v16, v17, v18, v19) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: i32 = *v13;
            let v14: i32 = *v14;
            let v15: i32 = *v15;
            let v16: bool = *v16;
            let v17: i32 = *v17;
            let v18: i32 = *v18;
            let v19: Rc<UH1> = v19.clone();
            let v20: bool = v18 < v7;
            if v20 {
                Rc::new(UH1::UH1_RankedThen(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v8.clone()))
            } else {
                let v22: Rc<UH1> = inserted_18(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v19.clone());
                Rc::new(UH1::UH1_RankedThen(v11.clone(), v12.clone(), v13, v14, v15, v16, v17, v18, v22.clone()))
            }
        }
    }
}
fn ranked_from_11(mut v0: US0, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v3 {
            UH0::UH0_NoTasks => {
                return v4.clone();
            }
            UH0::UH0_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: i32 = *v7;
                let v8: i32 = *v8;
                let v9: Rc<str> = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: Rc<str> = v11.clone();
                let v12: i32 = *v12;
                let v13: i32 = *v13;
                let v14: Rc<UH0> = v14.clone();
                let v15: bool = task_open_12(v5.clone(), v6.clone(), v7, v8, v9.clone(), v10.clone(), v11.clone(), v12, v13);
                let v18: bool = if v15 {
                    let v16: i32 = 0i32;
                    deps_settled_13(v2.clone(), v10.clone(), v16)
                } else {
                    false
                };
                let v20: bool = if v18 {
                    v1(v5.clone(), v6.clone(), v7, v8, v9.clone(), v10.clone(), v11.clone(), v12, v13)
                } else {
                    false
                };
                if v20 {
                    let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) = ranked_of_15(v0.clone(), v2.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10.clone(), v11.clone(), v12, v13);
                    let v29: Rc<UH1> = inserted_18(v21.clone(), v22.clone(), v23, v24, v25, v26, v27, v28, v4.clone());
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v14.clone(), v29.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v14.clone(), v4.clone());
                    continue;
                }
            }
        }
    }
}
fn ranked_json_20(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: bool, mut v7: i32, mut v8: i32) -> Rc<str> {
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"rank\":"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"id\":\""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v1, &*v12].concat());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"priority\":"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"difficulty\":"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"minutes\":"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"estimated\":"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = if v6 {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("true"); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    } else {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("false"); } LIT.with(|lit| lit.clone()) };
        v22.clone()
    };
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v23, &*v24].concat());
    let v26: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v17, &*v25].concat());
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"wsjf\":"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unblocks\":"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"title\":\""); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"}"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v30, &*v31, &*v2, &*v32].concat());
    let v34: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v26, &*v27, &*v28, &*v33].concat());
    v34.clone()
}
fn json_items_19(mut v0: Rc<UH1>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32) {
    loop {
        match &*v0 {
            UH1::UH1_NoRanked => {
                let v4: i32 = v1.wrapping_sub(1i32);
                return (v3.clone(), v4);
            }
            UH1::UH1_RankedThen(v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: i32 = *v7;
                let v8: i32 = *v8;
                let v9: i32 = *v9;
                let v10: bool = *v10;
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: Rc<UH1> = v13.clone();
                let v14: bool = v2 < v1;
                if v14 {
                    let v15: i32 = v1.wrapping_sub(1i32);
                    return (v3.clone(), v15);
                } else {
                    let v16: i32 = v1.wrapping_add(1i32);
                    let v17: bool = &*v3 == "";
                    let v23: Rc<str> = if v17 {
                        ranked_json_20(v1, v5.clone(), v6.clone(), v7, v8, v9, v10, v11, v12)
                    } else {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v20: Rc<str> = ranked_json_20(v1, v5.clone(), v6.clone(), v7, v8, v9, v10, v11, v12);
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v22: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v19, &*v20, &*v21, &*v21].concat());
                        v22.clone()
                    };
                    (v0, v1, v2, v3) = (v13.clone(), v16, v2, v23.clone());
                    continue;
                }
            }
        }
    }
}
fn ranked_count_21(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_NoRanked => {
                return v1;
            }
            UH1::UH1_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: i32 = *v4;
                let v5: i32 = *v5;
                let v6: i32 = *v6;
                let v7: bool = *v7;
                let v8: i32 = *v8;
                let v9: i32 = *v9;
                let v10: Rc<UH1> = v10.clone();
                let v11: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v10.clone(), v11);
                continue;
            }
        }
    }
}
fn ranked_line_23(mut v0: US0, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: bool, mut v8: i32, mut v9: i32) -> Rc<str> {
    let v10: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = if v7 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        v12.clone()
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v14, &*v15, &*v15].concat());
    let v22: Rc<str> = match &v0 {
        US0::US0_MainAgent => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  wsjf="); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
            let v19: Rc<str> = { let text = v18; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v19, &*v15, &*v15, &*v15].concat());
            v20.clone()
        }
        US0::US0_Subagent => {
            v15.clone()
        }
    };
    let v23: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v24: Rc<str> = { let text = v23; let filler = (3i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = { let text = v2; let filler = (28i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v29: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v27, &*v28].concat());
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" D"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let v32: Rc<str> = { let text = v16; let filler = (7i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v25, &*v32, &*v22].concat());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unblocks="); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v36: Rc<str> = { let text = v35; let filler = (5i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let v37: Rc<str> = { let text = v3; if text.chars().count() <= 72i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((72i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let v38: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v37, &*v15, &*v15, &*v15].concat());
    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v33, &*v34, &*v36, &*v38].concat());
    v39.clone()
}
fn printed_22(mut v0: US0, mut v1: Rc<UH1>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_NoRanked => {
                let v4: i32 = v2.wrapping_sub(1i32);
                return v4;
            }
            UH1::UH1_RankedThen(v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: i32 = *v7;
                let v8: i32 = *v8;
                let v9: i32 = *v9;
                let v10: bool = *v10;
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: Rc<UH1> = v13.clone();
                let v14: bool = v3 < v2;
                if v14 {
                    let v15: i32 = v2.wrapping_sub(1i32);
                    return v15;
                } else {
                    let v16: Rc<str> = ranked_line_23(v0.clone(), v2, v5.clone(), v6.clone(), v7, v8, v9, v10, v11, v12);
                    let v17: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v16.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    let v18: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v13.clone(), v18, v3);
                    continue;
                }
            }
        }
    }
}
fn agile_next_1(mut v0: Rc<str>, mut v1: US0, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1) -> i32 {
    let v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v8, &*v8].concat());
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v13: US2 = if v10 {
        US2::US2_FileReadable(v9.clone())
    } else {
        US2::US2_FileMissing(v9.clone())
    };
    match &v13 {
        US2::US2_FileMissing(v14) => {
            let v14: Rc<str> = v14.clone();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v8, &*v8, &*v8].concat());
            let v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US2::US2_FileReadable(v19) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let v21: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v20);
            let v22: i32 = (v21.clone().len() as i32);
            let v23: i32 = 0i32;
            let v24: i32 = first_malformed_task_2(v21.clone(), v23, v22);
            let v25: bool = v24 < 0i32;
            if v25 {
                let v26: i32 = 0i32;
                let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) };
                let v28: Rc<UH0> = tasks_from_6(v21.clone(), v26, v22, v27.clone());
                let v29: Rc<dyn Fn(Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> = closure1(v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v21.clone());
                let v30: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
                let v31: Rc<UH1> = ranked_from_11(v1.clone(), v29.clone(), v28.clone(), v28.clone(), v30.clone());
                match &v5 {
                    US1::US1_NextJson => {
                        let v32: i32 = 1i32;
                        let (mut v33, mut v34): (Rc<str>, i32) = json_items_19(v31.clone(), v32, v2, v8.clone());
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"role\":\""); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = match &v1 {
                            US0::US0_MainAgent => {
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
                                v36.clone()
                            }
                            US0::US0_Subagent => {
                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subagent"); } LIT.with(|lit| lit.clone()) };
                                v37.clone()
                            }
                        };
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\",\"ready\":"); } LIT.with(|lit| lit.clone()) };
                        let v41: i32 = 0i32;
                        let v42: i32 = ranked_count_21(v31.clone(), v41);
                        let v43: Rc<str> = std::rc::Rc::<str>::from((v42).to_string());
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"shown\":"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"items\":["); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v45, &*v46, &*v33, &*v47].concat());
                        let v49: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v39, &*v40, &*v43, &*v48].concat());
                        let v50: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v49.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v50
                    }
                    US1::US1_NextText => {
                        let v51: i32 = 1i32;
                        let v52: i32 = printed_22(v1.clone(), v31.clone(), v51, v2);
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ready="); } LIT.with(|lit| lit.clone()) };
                        let v54: i32 = 0i32;
                        let v55: i32 = ranked_count_21(v31.clone(), v54);
                        let v56: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = std::rc::Rc::<str>::from((v52).to_string());
                        let v59: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v56, &*v57, &*v58, &*v8].concat());
                        let v60: bool = &*v4 == "all";
                        let v63: Rc<str> = if v60 {
                            v8.clone()
                        } else {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v4));
                            v62.clone()
                        };
                        let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v63));
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile next ok role="); } LIT.with(|lit| lit.clone()) };
                        let v69: Rc<str> = match &v1 {
                            US0::US0_MainAgent => {
                                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
                                v66.clone()
                            }
                            US0::US0_Subagent => {
                                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subagent"); } LIT.with(|lit| lit.clone()) };
                                v67.clone()
                            }
                        };
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let v71: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
                        let v72: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                        let v73: Rc<str> = std::rc::Rc::<str>::from([&*v65, &*v69, &*v64, &*v70, &*v72].concat());
                        let v74: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v73.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v74
                    }
                }
            } else {
                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let v78: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
                let v79: Rc<str> = std::rc::Rc::<str>::from([&*v77, &*v78, &*v8, &*v8, &*v8].concat());
                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
                let v81: Rc<str> = std::rc::Rc::<str>::from([&*v80, &*v79, &*v8, &*v8, &*v8].concat());
                let v82: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v81 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v82
            }
        }
    }
}
fn agile_next_entry_0() -> i32 {
    let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) = kit_agile_next_args::kit_agile_next_request();
    if v0 {
        let v7: bool = &*v2 == "subagent";
        let v10: US0 = if v7 {
            US0::US0_Subagent
        } else {
            US0::US0_MainAgent
        };
        let v13: US1 = if v6 {
            US1::US1_NextJson
        } else {
            US1::US1_NextText
        };
        let v14: i32 = 1i32;
        let v15: Rc<str> = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        agile_next_1(v15.clone(), v10.clone(), v3, v4.clone(), v5.clone(), v13.clone())
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile next rejected: "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v1, &*v18, &*v18, &*v18].concat());
        let v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v20
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_next_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_next_run() -> i32 {
    closure0()()
}
