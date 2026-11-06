#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Unestimated,
    US0_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US2 {
    US2_LoadLight,
    US2_LoadMedium,
    US2_LoadHeavy,
}
#[derive(Clone)]
enum US1 {
    US1_LoadUnset,
    US1_PlannedLoad(US2),
}
#[derive(Clone)]
enum US4 {
    US4_AssigneeAgent,
    US4_AssigneePair,
    US4_AssigneeUser,
}
#[derive(Clone)]
enum US3 {
    US3_Unassigned,
    US3_PlannedAssignee(US4),
}
#[derive(Clone)]
enum US5 {
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum US6 {
    US6_Valid(i32),
    US6_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_Valid(US1),
    US7_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(US2),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_Valid(US3),
    US9_Invalid,
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US4),
    US10_Invalid,
}
fn declared_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
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
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US5::US5_ScanPlain => {
                    let v6: bool = v5 == 44i32;
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
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US5 = match v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
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
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US5 = US5::US5_ScanPlain;
            let v6: i32 = field_end_6(v0.clone(), v1, v2, v5);
            let mut v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn decimal_digits_value_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v6: bool = 48i32 <= v5;
            let v8: bool = if v6 {
                let mut v7: bool = v5 <= 57i32;
                v7
            } else {
                false
            };
            if v8 {
                let v9: i32 = v1.wrapping_add(1i32);
                let v10: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v11: i32 = v3 * 10 + v10 - 48;
                (v1, v3) = (v9, v11);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn decimal_value_8(mut v0: Rc<str>) -> US6 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US6::US6_Invalid
    } else {
        let v4: bool = v1 <= 6i32;
        if v4 {
            let v5: i32 = 0i32;
            let v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_9(v0, v5, v1, v6);
            let v8: bool = v7 < 0i32;
            if v8 {
                US6::US6_Invalid
            } else {
                US6::US6_Valid(v7)
            }
        } else {
            US6::US6_Invalid
        }
    }
}
fn parse_planned_load_10(mut v0: Rc<str>) -> US7 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US1 = US1::US1_LoadUnset;
        US7::US7_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let v9: bool = &*v1 == &*v8;
        let v13: US8 = if v9 {
            let v10: US2 = US2::US2_LoadLight;
            US8::US8_Valid(v10)
        } else {
            US8::US8_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US8 = if v16 {
            let v17: US2 = US2::US2_LoadMedium;
            US8::US8_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US8 = if v22 {
            let v23: US2 = US2::US2_LoadHeavy;
            US8::US8_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US8::US8_Invalid => {
                US7::US7_Invalid
            }
            US8::US8_Valid(v26) => {
                let v26: US2 = v26;
                let v27: US1 = US1::US1_PlannedLoad(v26);
                US7::US7_Valid(v27)
            }
        }
    }
}
fn parse_planned_assignee_11(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US3 = US3::US3_Unassigned;
        US9::US9_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let v9: bool = &*v1 == &*v8;
        let v13: US10 = if v9 {
            let v10: US4 = US4::US4_AssigneeAgent;
            US10::US10_Valid(v10)
        } else {
            US10::US10_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US10 = if v16 {
            let v17: US4 = US4::US4_AssigneePair;
            US10::US10_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US10 = if v22 {
            let v23: US4 = US4::US4_AssigneeUser;
            US10::US10_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US10::US10_Invalid => {
                US9::US9_Invalid
            }
            US10::US10_Valid(v26) => {
                let v26: US4 = v26;
                let v27: US3 = US3::US3_PlannedAssignee(v26);
                US9::US9_Valid(v27)
            }
        }
    }
}
fn planning_at_4(mut v0: Rc<str>, mut v1: i32) -> (US0, US1, US3) {
    let mut v2: i32 = v1.wrapping_add(10i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 1i32;
    let v5: i32 = field_start_5(v0.clone(), v2, v3, v4);
    let v6: US5 = US5::US5_ScanPlain;
    let mut v7: i32 = field_end_6(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v8.starts_with(&*v9);
    let v23: US0 = if v10 {
        let v11: i32 = (v8.clone().len() as i32);
        let v12: Rc<str> = std::rc::Rc::<str>::from(v8.get(8i32 as usize..v11 as usize).unwrap_or(""));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
        let v16: US6 = decimal_value_8(v15);
        match v16 {
            US6::US6_Invalid => {
                US0::US0_Unestimated
            }
            US6::US6_Valid(v17) => {
                let v17: i32 = v17;
                US0::US0_EstimateMinutes(v17)
            }
        }
    } else {
        US0::US0_Unestimated
    };
    let v24: i32 = 2i32;
    let v25: i32 = field_start_5(v0.clone(), v2, v3, v24);
    let v26: US5 = US5::US5_ScanPlain;
    let v27: i32 = field_end_6(v0.clone(), v25, v3, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
    let v30: bool = v28.starts_with(&*v29);
    let v39: US1 = if v30 {
        let v31: i32 = (v28.clone().len() as i32);
        let v32: Rc<str> = std::rc::Rc::<str>::from(v28.get(5i32 as usize..v31 as usize).unwrap_or(""));
        let v33: US7 = parse_planned_load_10(v32);
        match v33 {
            US7::US7_Invalid => {
                US1::US1_LoadUnset
            }
            US7::US7_Valid(v34) => {
                let v34: US1 = v34;
                v34
            }
        }
    } else {
        US1::US1_LoadUnset
    };
    let v40: i32 = 3i32;
    let v41: i32 = field_start_5(v0.clone(), v2, v3, v40);
    let v42: US5 = US5::US5_ScanPlain;
    let v43: i32 = field_end_6(v0.clone(), v41, v3, v42);
    let v44: Rc<str> = std::rc::Rc::<str>::from(v0.get(v41 as usize..v43 as usize).unwrap_or(""));
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
    let v46: bool = v44.starts_with(&*v45);
    let v55: US3 = if v46 {
        let v47: i32 = (v44.clone().len() as i32);
        let v48: Rc<str> = std::rc::Rc::<str>::from(v44.get(9i32 as usize..v47 as usize).unwrap_or(""));
        let v49: US9 = parse_planned_assignee_11(v48);
        match v49 {
            US9::US9_Invalid => {
                US3::US3_Unassigned
            }
            US9::US9_Valid(v50) => {
                let v50: US3 = v50;
                v50
            }
        }
    } else {
        US3::US3_Unassigned
    };
    (v23, v39, v55)
}
fn planning_of_2(mut v0: Rc<str>, mut v1: Rc<str>) -> (US0, US1, US3) {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning (\""); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) }));
    let v4: i32 = 0i32;
    let v5: i32 = declared_from_3(v0.clone(), v3, v4);
    let v6: bool = v5 < 0i32;
    let (mut v13, mut v14, mut v15): (US0, US1, US3) = if v6 {
        let mut v7: US0 = US0::US0_Unestimated;
        let v8: US1 = US1::US1_LoadUnset;
        let v9: US3 = US3::US3_Unassigned;
        (v7, v8, v9)
    } else {
        let (mut v10, mut v11, mut v12): (US0, US1, US3) = planning_at_4(v0, v5);
        (v10, v11, v12)
    };
    (v13, v14, v15)
}
fn gaps_from_1(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let mut v7: bool = &*v5 == "Planned";
                let v9: bool = if v7 {
                    true
                } else {
                    let v8: bool = &*v5 == "Active";
                    v8
                };
                let v11: bool = if v9 {
                    let v10: bool = &*v4 != "Epic";
                    v10
                } else {
                    false
                };
                let v41: Rc<str> = if v11 {
                    let (mut v12, mut v13, mut v14): (US0, US1, US3) = planning_of_2(v0.clone(), v3.clone());
                    let v15: bool = match v12 {
                        US0::US0_Unestimated => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v18: Rc<str> = if v15 {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
                        v16
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v17
                    };
                    let v19: bool = match v13 {
                        US1::US1_LoadUnset => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v25: Rc<str> = if v19 {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
                        let v21: bool = &*v18 == "";
                        if v21 {
                            v20
                        } else {
                            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v20));
                            v23
                        }
                    } else {
                        v18
                    };
                    let v26: bool = match v14 {
                        US3::US3_Unassigned => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v32: Rc<str> = if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
                        let v28: bool = &*v25 == "";
                        if v28 {
                            v27
                        } else {
                            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v27));
                            v30
                        }
                    } else {
                        v25
                    };
                    let v33: bool = &*v32 == "";
                    if v33 {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v34
                    } else {
                        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile check warn missing="); } LIT.with(|lit| lit.clone()) }, v32));
                        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" id="); } LIT.with(|lit| lit.clone()) }));
                        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v3));
                        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        v38
                    }
                } else {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v40
                };
                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v2));
                (v1, v2) = (v6, v42);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn gap_tasks_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v4)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        if v6 {
            return v3;
        } else {
            let mut v7: i32 = v5.wrapping_add(6i32);
            let v8: i32 = v0.get(..v5 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v5 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v9.starts_with(&*v10);
            if v11 {
                let v12: i32 = 0i32;
                let v13: i32 = field_start_5(v0.clone(), v7, v2, v12);
                let mut v14: US5 = US5::US5_ScanPlain;
                let v15: i32 = field_end_6(v0.clone(), v13, v2, v14);
                let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v15 as usize).unwrap_or(""));
                let v17: i32 = (v16.clone().len() as i32);
                let v18: i32 = v17.wrapping_sub(1i32);
                let v19: Rc<str> = std::rc::Rc::<str>::from(v16.get(1i32 as usize..v18 as usize).unwrap_or(""));
                let mut v20: i32 = 2i32;
                let v21: i32 = field_start_5(v0.clone(), v7, v2, v20);
                let v22: US5 = US5::US5_ScanPlain;
                let v23: i32 = field_end_6(v0.clone(), v21, v2, v22);
                let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v23 as usize).unwrap_or(""));
                let v25: i32 = 6i32;
                let v26: i32 = field_start_5(v0.clone(), v7, v2, v25);
                let v27: US5 = US5::US5_ScanPlain;
                let v28: i32 = field_end_6(v0.clone(), v26, v2, v27);
                let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
                let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v24, v29, v3));
                (v1, v3) = (v7, v30);
                continue;
            } else {
                v1 = v7;
                continue;
            }
        }
    }
}
fn planning_gaps_text_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = gap_tasks_from_12(v0.clone(), v1, v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    gaps_from_1(v0, v4, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        planning_gaps_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_gaps_text(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
