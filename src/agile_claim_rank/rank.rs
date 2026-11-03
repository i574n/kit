#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_NoTasks,
    UH0_TaskThen(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_MainAgent,
    US2_Subagent,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRanked,
    UH1_RankedThen(Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
struct Mut0 { l0: i32, l1: Rc<UH2> }
enum US3 {
    US3_Ok(Vec<std::string::String>),
    US3_Error(std::io::Error),
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US1::US1_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
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
                let mut v10: i32 = v1.wrapping_add(1i32);
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
                (v1, v3) = (v10, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn backlog_task_at_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) {
    let mut v3: i32 = 0i32;
    let v4: i32 = field_start_4(v0.clone(), v1, v2, v3);
    let v5: US1 = US1::US1_ScanPlain;
    let mut v6: i32 = field_end_5(v0.clone(), v4, v2, v5.clone());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let mut v8: i32 = (v7.clone().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_4(v0.clone(), v1, v2, v11);
    let v13: US1 = US1::US1_ScanPlain;
    let v14: i32 = field_end_5(v0.clone(), v12, v2, v13.clone());
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_4(v0.clone(), v1, v2, v19);
    let v21: US1 = US1::US1_ScanPlain;
    let v22: i32 = field_end_5(v0.clone(), v20, v2, v21.clone());
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let v25: i32 = field_start_4(v0.clone(), v1, v2, v24);
    let v26: US1 = US1::US1_ScanPlain;
    let v27: i32 = field_end_5(v0.clone(), v25, v2, v26.clone());
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = (v28).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v30: i32 = 4i32;
    let v31: i32 = field_start_4(v0.clone(), v1, v2, v30);
    let v32: US1 = US1::US1_ScanPlain;
    let v33: i32 = field_end_5(v0.clone(), v31, v2, v32.clone());
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: i32 = (v34).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
    let v36: i32 = 6i32;
    let v37: i32 = field_start_4(v0.clone(), v1, v2, v36);
    let v38: US1 = US1::US1_ScanPlain;
    let v39: i32 = field_end_5(v0.clone(), v37, v2, v38.clone());
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: i32 = 7i32;
    let v42: i32 = field_start_4(v0.clone(), v1, v2, v41);
    let v43: US1 = US1::US1_ScanPlain;
    let v44: i32 = field_end_5(v0.clone(), v42, v2, v43.clone());
    let v45: Rc<str> = std::rc::Rc::<str>::from(v0.get(v42 as usize..v44 as usize).unwrap_or(""));
    let v46: i32 = (v45.clone().len() as i32);
    let v47: i32 = v46.wrapping_sub(1i32);
    let v48: Rc<str> = std::rc::Rc::<str>::from(v45.get(1i32 as usize..v47 as usize).unwrap_or(""));
    let v49: i32 = 8i32;
    let v50: i32 = field_start_4(v0.clone(), v1, v2, v49);
    let v51: US1 = US1::US1_ScanPlain;
    let v52: i32 = field_end_5(v0.clone(), v50, v2, v51.clone());
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: i32 = (v53.clone().len() as i32);
    let v55: i32 = v54.wrapping_sub(1i32);
    let v56: Rc<str> = std::rc::Rc::<str>::from(v53.get(1i32 as usize..v55 as usize).unwrap_or(""));
    let v57: i32 = agile_planning_fields::kit_agile_planning_minutes_near(&v0, &v10, v1);
    let v58: i32 = 5i32;
    let v59: i32 = field_start_4(v0.clone(), v1, v2, v58);
    let v60: US1 = US1::US1_ScanPlain;
    let v61: i32 = field_end_5(v0.clone(), v59, v2, v60.clone());
    let v62: Rc<str> = std::rc::Rc::<str>::from(v0.get(v59 as usize..v61 as usize).unwrap_or(""));
    let v63: i32 = (v62).trim().trim_end_matches("u32").parse::<i32>().unwrap_or(0);
    (v10.clone(), v18.clone(), v23.clone(), v29, v35, v40.clone(), v48.clone(), v56.clone(), v57, v63)
}
fn tasks_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v3.clone();
        } else {
            let mut v7: i32 = v5.wrapping_add(6i32);
            let mut v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) = backlog_task_at_3(v0.clone(), v7, v2);
                let v22: Rc<UH0> = Rc::new(UH0::UH0_TaskThen(v12.clone(), v13.clone(), v14.clone(), v15, v16, v17.clone(), v18.clone(), v19.clone(), v20, v21, v3.clone()));
                (v1, v3) = (v7, v22.clone());
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn agents_take_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32) -> bool {
    let v11: bool = &*v3 != "Epic";
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
        let v14: bool = agile_planning_filters::kit_agile_planning_selected(&v0, &v1, &v12, &v13);
        let v15: bool = v14 == false;
        v15
    } else {
        false
    }
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> {
    Rc::new(move |mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: i32| -> bool {
        agents_take_8(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5, v6.clone(), v7.clone(), v8.clone(), v9, v10)
    })
}
fn task_open_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: i32, mut v9: i32) -> bool {
    let mut v10: bool = &*v5 == "Planned";
    if v10 {
        true
    } else {
        let v11: bool = &*v5 == "Active";
        if v11 {
            true
        } else {
            let v12: bool = &*v5 == "Blocked";
            if v12 {
                false
            } else {
                let v13: bool = &*v5 == "Paused";
                if v13 {
                    false
                } else {
                    false
                }
            }
        }
    }
}
fn status_of_12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2.clone();
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: i32 = *v7;
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: Rc<UH0> = v13.clone();
                let v14: bool = &*v3 == &*v1;
                if v14 {
                    return v8.clone();
                } else {
                    v0 = v13.clone();
                    continue;
                }
            }
        }
    }
}
fn deps_settled_11(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v1.clone().len() as i32);
        let v4: bool = v3 <= v2;
        if v4 {
            return true;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v6: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let mut v7: bool = v6 < 0i32;
            let mut v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v8 as usize).unwrap_or(""));
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.trim());
            let v11: Rc<str> = status_of_12(v0.clone(), v10.clone());
            let v12: bool = &*v11 == "";
            let v14: bool = if v12 {
                true
            } else {
                let v13: bool = &*v11 == "Done";
                v13
            };
            if v14 {
                let v15: i32 = v8.wrapping_add(1i32);
                v2 = v15;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn open_deps_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v4 <= v2;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v7: i32 = v1.get(v2 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
            let mut v8: bool = v7 < 0i32;
            let mut v9: i32 = if v8 {
                v4
            } else {
                v7
            };
            let mut v10: Rc<str> = std::rc::Rc::<str>::from(v1.get(v2 as usize..v9 as usize).unwrap_or(""));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
            let v12: Rc<str> = status_of_12(v0.clone(), v11.clone());
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
            (v2, v3) = (v13, v18);
            continue;
        }
    }
}
fn unblocked_weight_14(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                return v2;
            }
            UH0::UH0_TaskThen(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: i32 = *v7;
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: Rc<str> = v10.clone();
                let v11: i32 = *v11;
                let v12: i32 = *v12;
                let v13: Rc<UH0> = v13.clone();
                let v14: bool = task_open_10(v3.clone(), v4.clone(), v5.clone(), v6, v7, v8.clone(), v9.clone(), v10.clone(), v11, v12);
                let v21: bool = if v14 {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v17: Rc<str> = Rc::<str>::from(v9.replace(&*v15, &*v16));
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v16));
                    let v20: bool = v19.split("|").any(|item| item == &*v1);
                    v20
                } else {
                    false
                };
                let v29: i32 = if v21 {
                    let v22: i32 = 0i32;
                    let v23: i32 = 0i32;
                    let v24: i32 = open_deps_15(v0.clone(), v9.clone(), v22, v23);
                    let v25: i32 = 1i32 << (2 * (5 - (v6).clamp(0, 5)));
                    let v26: bool = v24 < 1i32;
                    let v27: i32 = if v26 {
                        1i32
                    } else {
                        v24
                    };
                    let v28: i32 = v25.wrapping_div(v27);
                    v28
                } else {
                    0i32
                };
                let v30: i32 = v2.wrapping_add(v29);
                (v0, v2) = (v13.clone(), v30);
                continue;
            }
        }
    }
}
fn ranked_of_13(mut v0: US2, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: i32) -> (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) {
    let v12: bool = 0i32 < v10;
    let v30: i32 = if v12 {
        v10
    } else {
        let v13: bool = 0i32 == v6;
        let v28: i32 = if v13 {
            15i32
        } else {
            let v14: bool = 1i32 == v6;
            if v14 {
                30i32
            } else {
                let v15: bool = 2i32 == v6;
                if v15 {
                    60i32
                } else {
                    let v16: bool = 3i32 == v6;
                    if v16 {
                        180i32
                    } else {
                        let v17: bool = 4i32 == v6;
                        if v17 {
                            300i32
                        } else {
                            let v18: bool = 5i32 == v6;
                            if v18 {
                                600i32
                            } else {
                                let v19: bool = 6i32 == v6;
                                if v19 {
                                    1200i32
                                } else {
                                    let v20: bool = 7i32 == v6;
                                    if v20 {
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
        let v29: i32 = ((v28 as i64) * (100 - (v11).clamp(0, 100)) as i64 / 100).max(15) as i32;
        v29
    };
    let v31: i32 = 0i32;
    let v32: i32 = unblocked_weight_14(v1, v2.clone(), v31);
    let v33: i32 = 1i32 << (2 * (5 - (v5).clamp(0, 5)));
    let v34: i32 = v33.wrapping_add(v32);
    let v40: i32 = match &v0 {
        US2::US2_MainAgent => {
            let v35: i32 = ((v34 as i64) * 6000 / (v30).max(1) as i64).min(i32::MAX as i64) as i32;
            v35
        }
        US2::US2_Subagent => {
            let v36: i32 = v30.wrapping_mul(8i32);
            let v37: i32 = v36.wrapping_add(5i32);
            let v38: i32 = v37.wrapping_sub(v5);
            v38
        }
    };
    (v2, v3, v5, v6, v30, v12, v32, v40)
}
fn inserted_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: bool, mut v6: i32, mut v7: i32, mut v8: Rc<UH1>) -> Rc<UH1> {
    match &*v8 {
        UH1::UH1_NoRanked => {
            let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_RankedThen(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v9))
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
                Rc::new(UH1::UH1_RankedThen(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7, v8))
            } else {
                let v22: Rc<UH1> = inserted_16(v0, v1, v2, v3, v4, v5, v6, v7, v19);
                Rc::new(UH1::UH1_RankedThen(v11, v12, v13, v14, v15, v16, v17, v18, v22))
            }
        }
    }
}
fn ranked_from_9(mut v0: US2, mut v1: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v3 {
            UH0::UH0_NoTasks => {
                return v4.clone();
            }
            UH0::UH0_TaskThen(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let mut v8: i32 = *v8;
                let mut v9: i32 = *v9;
                let mut v10: Rc<str> = v10.clone();
                let v11: Rc<str> = v11.clone();
                let v12: Rc<str> = v12.clone();
                let v13: i32 = *v13;
                let v14: i32 = *v14;
                let v15: Rc<UH0> = v15.clone();
                let v16: bool = task_open_10(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14);
                let v20: bool = if v16 {
                    match &v0 {
                        US2::US2_MainAgent => {
                            true
                        }
                        US2::US2_Subagent => {
                            let v17: bool = &*v7 != "Epic";
                            v17
                        }
                    }
                } else {
                    false
                };
                let v29: bool = if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user-batch"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(v12.replace(&*v22, &*v23));
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v23));
                    let v27: bool = v26.split("|").any(|item| item == &*v21);
                    let v28: bool = v27 == false;
                    v28
                } else {
                    false
                };
                let v32: bool = if v29 {
                    let v30: i32 = 0i32;
                    deps_settled_11(v2.clone(), v11.clone(), v30)
                } else {
                    false
                };
                let v34: bool = if v32 {
                    v1(v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14)
                } else {
                    false
                };
                if v34 {
                    let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, Rc<str>, i32, i32, i32, bool, i32, i32) = ranked_of_13(v0.clone(), v2.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v9, v10.clone(), v11.clone(), v12.clone(), v13, v14);
                    let v43: Rc<UH1> = inserted_16(v35.clone(), v36.clone(), v37, v38, v39, v40, v41, v42, v4.clone());
                    (v3, v4) = (v15.clone(), v43.clone());
                    continue;
                } else {
                    v3 = v15.clone();
                    continue;
                }
            }
        }
    }
}
fn ranked_words_17(mut v0: Rc<UH1>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_NoRanked => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        UH1::UH1_RankedThen(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: i32 = *v5;
            let mut v6: i32 = *v6;
            let mut v7: bool = *v7;
            let mut v8: i32 = *v8;
            let mut v9: i32 = *v9;
            let mut v10: Rc<UH1> = v10.clone();
            let v11: Rc<UH2> = ranked_words_17(v10);
            Rc::new(UH2::UH2_Cons(v2, v11))
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<UH2> = v4.clone();
            let v5: Rc<UH2> = method18(v0.clone(), v4, v2.clone());
            let mut v6: Rc<str> = status_of_12(v0, v3.clone());
            let mut v7: bool = &*v6 == "Active";
            if v7 {
                Rc::new(UH2::UH2_Cons(v3, v5.clone()))
            } else {
                v5
            }
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn open_words_20(mut v0: Rc<UH0>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH0::UH0_NoTasks => {
                return { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            }
            UH0::UH0_TaskThen(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let mut v6: i32 = *v6;
                let mut v7: Rc<str> = v7.clone();
                let mut v8: Rc<str> = v8.clone();
                let mut v9: Rc<str> = v9.clone();
                let mut v10: i32 = *v10;
                let v11: i32 = *v11;
                let v12: Rc<UH0> = v12.clone();
                let v13: bool = &*v7 == "Done";
                if v13 {
                    v0 = v12.clone();
                    continue;
                } else {
                    let v15: Rc<UH2> = open_words_20(v12.clone());
                    return Rc::new(UH2::UH2_Cons(v2.clone(), v15.clone()));
                }
            }
        }
    }
}
fn ranking_of_7(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: US2 = US2::US2_MainAgent;
    let mut v3: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, i32, Rc<str>, Rc<str>, Rc<str>, i32, i32) -> bool> = closure1(v0);
    let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRanked); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH1> = ranked_from_9(v2, v3, v1.clone(), v1.clone(), v4);
    let mut v6: Rc<UH2> = ranked_words_17(v5);
    let mut v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH2> = method18(v1.clone(), v6.clone(), v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v10: i32 = 0i32;
    let v11: i32 = method19(v9.clone(), v6, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v17: i32 = 0i32;
    let v18: i32 = method19(v16.clone(), v8, v17);
    let v19: Rc<Vec<Rc<str>>> = Rc::new(v16.borrow().clone());
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
    let v21: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v14));
    let v22: Rc<UH2> = open_words_20(v1);
    let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v24: i32 = 0i32;
    let v25: i32 = method19(v23.clone(), v22, v24);
    let v26: Rc<Vec<Rc<str>>> = Rc::new(v23.borrow().clone());
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
    let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v14));
    (v15, v21, v28)
}
fn method22(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH2> = v3.clone();
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method23(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method23(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method22(v16.clone(), v17.clone())
    }
}
fn jobs_root_25() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn ok_c2() -> Rc<dyn Fn(Vec<std::string::String>) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US3> = Rc::new(move |mut v0: Vec<std::string::String>| -> US3 {
        US3::US3_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method26() -> Rc<dyn Fn(Vec<std::string::String>) -> US3> {
    ok_c2()
}
fn error_c3() -> Rc<dyn Fn(std::io::Error) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US3> = Rc::new(move |mut v0: std::io::Error| -> US3 {
        US3::US3_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method27() -> Rc<dyn Fn(std::io::Error) -> US3> {
    error_c3()
}
fn file_exists_28(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn finished_c4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v2)), v1));
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/exit"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
        file_exists_28(v6.clone())
    })
}
fn method31(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<str>, mut v2: Rc<UH2>, mut v3: bool) -> bool {
    loop {
        match &*v2 {
            UH2::UH2_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let v17: bool = if v3 {
                    true
                } else {
                    let mut v6: Rc<str> = Rc::<str>::from(v4.to_lowercase());
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v8));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v8));
                    let v13: bool = v12.contains(&*v1);
                    if v13 {
                        let v14: bool = v0(v4.clone());
                        let v15: bool = v14 == false;
                        v15
                    } else {
                        false
                    }
                };
                (v2, v3) = (v5.clone(), v17);
                continue;
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method30(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH2>, mut v2: Rc<UH2>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH2::UH2_Cons(v4, v5) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<UH2> = v5.clone();
            let mut v6: Rc<UH2> = method30(v0.clone(), v1.clone(), v5, v3.clone());
            let mut v7: Rc<str> = Rc::<str>::from(v4.to_lowercase());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v7)), v8));
            let v11: bool = false;
            let v12: bool = method31(v0, v10, v1, v11);
            if v12 {
                Rc::new(UH2::UH2_Cons(v4, v6.clone()))
            } else {
                v6
            }
        }
        UH2::UH2_Nil => {
            v3
        }
    }
}
fn method34(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure5() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn started_ms_of_33(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH2> = split_list_21(v1.clone(), v0.clone());
    match &*v2 {
        UH2::UH2_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<UH2> = v4.clone();
            match &*v4 {
                UH2::UH2_Cons(v5, v6) => {
                    let v5: Rc<str> = v5.clone();
                    let mut v6: Rc<UH2> = v6.clone();
                    let mut v7: Option<u64> = (v5).trim().parse::<u64>().ok();
                    let mut v8: Option<u64> = method34(v7.clone());
                    let mut v9: Rc<dyn Fn((u64)) -> US4> = closure5();
                    let mut v10: Option<US4> = v8.map(|x| v9(x));
                    let v12: US4 = US4::US4_None;
                    let v13: US4 = v12.clone();
                    let v14: US4 = v10.unwrap_or(v13);
                    match &v14 {
                        US4::US4_None => {
                            0u64
                        }
                        US4::US4_Some(v15) => {
                            let v15: u64 = *v15;
                            v15
                        }
                    }
                }
                _ => {
                    0u64
                }
            }
        }
        _ => {
            0u64
        }
    }
}
fn method32(mut v0: u64, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<UH2> = v4.clone();
            let v5: Rc<UH2> = method32(v0, v4, v2.clone());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v3.starts_with(&*v6);
            let v11: bool = if v7 {
                let mut v8: u64 = started_ms_of_33(v3.clone());
                let mut v9: u64 = v8.wrapping_add(43200000u64);
                let mut v10: bool = v0 < v9;
                v10
            } else {
                false
            };
            if v11 {
                Rc::new(UH2::UH2_Cons(v3, v5.clone()))
            } else {
                v5
            }
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn live_of_names_29(mut v0: Rc<UH2>, mut v1: Rc<UH2>, mut v2: u64, mut v3: Rc<dyn Fn(Rc<str>) -> bool>) -> Rc<UH2> {
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH2> = method32(v2, v0, v4);
    let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method30(v3, v5, v1, v6)
}
fn live_items_24(mut v0: Rc<UH2>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_25();
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v6: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v7: Rc<dyn Fn(Vec<std::string::String>) -> US3> = method26();
    let mut v8: Rc<dyn Fn(std::io::Error) -> US3> = method27();
    let mut v9: US3 = match v6 { Ok(x) => v7(x), Err(e) => v8(e) };
    let v30: Rc<UH2> = match v9 {
        US3::US3_Error(v27) => {
            let v27: std::io::Error = v27;
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Ok(v10) => {
            let mut v10: Vec<std::string::String> = v10.clone();
            let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v13: bool = true; let _vec_fold_ = v10.into_iter().fold(v11, |acc, x| { //;
            let v15: Rc<UH2> = acc;
            let v17: std::string::String = x;
            let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
            let v21: Rc<UH2> = Rc::new(UH2::UH2_Cons(v19.clone(), v15.clone()));
            let v22: bool = true; v21 });
            let v24: Rc<UH2> = _vec_fold_;
            let v25: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            method22(v24.clone(), v25.clone())
        }
    };
    let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v32: Rc<dyn Fn(Rc<str>) -> bool> = finished_c4(v4.clone());
    let v33: Rc<UH2> = live_of_names_29(v30.clone(), v0.clone(), v31, v32.clone());
    let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v35: i32 = 0i32;
    let v36: i32 = method19(v34.clone(), v33.clone(), v35);
    let v37: Rc<Vec<Rc<str>>> = Rc::new(v34.borrow().clone());
    let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v37).as_ref().clone()));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = Rc::<str>::from((*v38.borrow()).join(&*v39));
    v40.clone()
}
fn claim_ranking_0(mut v0: Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = concat5_1(v0, v1, v2.clone(), v2.clone(), v2.clone());
    let v4: bool = std::path::Path::new(&*v3).is_file();
    let mut v7: US0 = if v4 {
        US0::US0_FileReadable(v3.clone())
    } else {
        US0::US0_FileMissing(v3)
    };
    match &v7 {
        US0::US0_FileMissing(v8) => {
            let mut v8: Rc<str> = v8.clone();
            (false, v2.clone(), v2.clone(), v2.clone(), v2)
        }
        US0::US0_FileReadable(v9) => {
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = std::fs::read_to_string(&*v9).unwrap_or_default().into();
            let v11: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v10);
            let v12: i32 = 0i32;
            let v13: i32 = (v11.clone().len() as i32);
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoTasks); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = tasks_from_2(v11.clone(), v12, v13, v14);
            let (mut v16, mut v17, mut v18): (Rc<str>, Rc<str>, Rc<str>) = ranking_of_7(v11, v15);
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<UH2> = split_list_21(v19, v16.clone());
            let v21: Rc<str> = live_items_24(v20);
            (true, v16, v17, v18, v21)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
        claim_ranking_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_ranking(v0: &str) -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
