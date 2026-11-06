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
    US1_Valid(Rc<str>),
    US1_Invalid,
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
enum US2 {
    US2_Valid(US3),
    US2_Invalid,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US4),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_Unestimated,
    US7_EstimateMinutes(i32),
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US7),
    US6_Invalid,
}
#[derive(Clone)]
enum US8 {
    US8_Valid(i32),
    US8_Invalid,
}
#[derive(Clone)]
enum US9 {
    US9_MinuteUnit,
    US9_HourUnit,
}
#[derive(Clone)]
enum US12 {
    US12_LoadLight,
    US12_LoadMedium,
    US12_LoadHeavy,
}
#[derive(Clone)]
enum US11 {
    US11_LoadUnset,
    US11_PlannedLoad(US12),
}
#[derive(Clone)]
enum US10 {
    US10_Valid(US11),
    US10_Invalid,
}
#[derive(Clone)]
enum US13 {
    US13_Valid(US12),
    US13_Invalid,
}
#[derive(Clone)]
enum US14 {
    US14_Valid(US0),
    US14_Invalid,
}
fn parse_planned_assignee_3(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US3 = US3::US3_Unassigned;
        US2::US2_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let mut v9: bool = &*v1 == &*v8;
        let v13: US5 = if v9 {
            let v10: US4 = US4::US4_AssigneeAgent;
            US5::US5_Valid(v10)
        } else {
            US5::US5_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US5 = if v16 {
            let v17: US4 = US4::US4_AssigneePair;
            US5::US5_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US5 = if v22 {
            let v23: US4 = US4::US4_AssigneeUser;
            US5::US5_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US5::US5_Invalid => {
                US2::US2_Invalid
            }
            US5::US5_Valid(v26) => {
                let v26: US4 = v26;
                let v27: US3 = US3::US3_PlannedAssignee(v26);
                US2::US2_Valid(v27)
            }
        }
    }
}
fn decimal_digits_value_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
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
                let mut v9: i32 = v1.wrapping_add(1i32);
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
fn decimal_value_5(mut v0: Rc<str>) -> US8 {
    let v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 <= 0i32;
    if v2 {
        US8::US8_Invalid
    } else {
        let v4: bool = v1 <= 6i32;
        if v4 {
            let v5: i32 = 0i32;
            let v6: i32 = 0i32;
            let mut v7: i32 = decimal_digits_value_6(v0, v5, v1, v6);
            let v8: bool = v7 < 0i32;
            if v8 {
                US8::US8_Invalid
            } else {
                US8::US8_Valid(v7)
            }
        } else {
            US8::US8_Invalid
        }
    }
}
fn parse_planned_estimate_4(mut v0: Rc<str>) -> US6 {
    let v1: bool = &*v0 == "none";
    let v3: bool = if v1 {
        true
    } else {
        let mut v2: bool = &*v0 == "-";
        v2
    };
    if v3 {
        let v4: US7 = US7::US7_Unestimated;
        US6::US6_Valid(v4)
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.ends_with(&*v6);
        let v17: Rc<str> = if v7 {
            let v8: i32 = (v0.clone().len() as i32);
            let mut v9: i32 = v8.wrapping_sub(1i32);
            let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v9 as usize).unwrap_or(""));
            v10
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v0.ends_with(&*v11);
            if v12 {
                let v13: i32 = (v0.clone().len() as i32);
                let mut v14: i32 = v13.wrapping_sub(1i32);
                let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v14 as usize).unwrap_or(""));
                v15
            } else {
                v0.clone()
            }
        };
        let v18: US8 = decimal_value_5(v17);
        match v18 {
            US8::US8_Invalid => {
                US6::US6_Invalid
            }
            US8::US8_Valid(v19) => {
                let v19: i32 = v19;
                let mut v20: bool = v0.ends_with(&*v6);
                let v23: US9 = if v20 {
                    US9::US9_HourUnit
                } else {
                    US9::US9_MinuteUnit
                };
                let v25: i32 = match v23 {
                    US9::US9_HourUnit => {
                        60i32
                    }
                    US9::US9_MinuteUnit => {
                        1i32
                    }
                };
                let v26: i32 = v19.wrapping_mul(v25);
                let v27: bool = v26 <= 999999i32;
                if v27 {
                    let v28: US7 = US7::US7_EstimateMinutes(v26);
                    US6::US6_Valid(v28)
                } else {
                    US6::US6_Invalid
                }
            }
        }
    }
}
fn parse_planned_load_7(mut v0: Rc<str>) -> US10 {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.to_ascii_lowercase());
    let mut v2: bool = &*v1 == "none";
    let v4: bool = if v2 {
        true
    } else {
        let v3: bool = &*v1 == "-";
        v3
    };
    if v4 {
        let v5: US11 = US11::US11_LoadUnset;
        US10::US10_Valid(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v7.to_ascii_lowercase());
        let mut v9: bool = &*v1 == &*v8;
        let v13: US13 = if v9 {
            let v10: US12 = US12::US12_LoadLight;
            US13::US13_Valid(v10)
        } else {
            US13::US13_Invalid
        };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = std::rc::Rc::<str>::from(v14.to_ascii_lowercase());
        let v16: bool = &*v1 == &*v15;
        let v19: US13 = if v16 {
            let v17: US12 = US12::US12_LoadMedium;
            US13::US13_Valid(v17)
        } else {
            v13
        };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v20.to_ascii_lowercase());
        let v22: bool = &*v1 == &*v21;
        let v25: US13 = if v22 {
            let v23: US12 = US12::US12_LoadHeavy;
            US13::US13_Valid(v23)
        } else {
            v19
        };
        match v25 {
            US13::US13_Invalid => {
                US10::US10_Invalid
            }
            US13::US13_Valid(v26) => {
                let v26: US12 = v26;
                let v27: US11 = US11::US11_PlannedLoad(v26);
                US10::US10_Valid(v27)
            }
        }
    }
}
fn planned_value_token_2(mut v0: US0, mut v1: Rc<str>) -> US1 {
    match v0 {
        US0::US0_FieldAssignee => {
            let v30: US2 = parse_planned_assignee_3(v1);
            match v30 {
                US2::US2_Invalid => {
                    US1::US1_Invalid
                }
                US2::US2_Valid(v31) => {
                    let v31: US3 = v31;
                    let v40: Rc<str> = match v31 {
                        US3::US3_PlannedAssignee(v33) => {
                            let v33: US4 = v33;
                            let v37: Rc<str> = match v33 {
                                US4::US4_AssigneeAgent => {
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Agent"); } LIT.with(|lit| lit.clone()) };
                                    v34
                                }
                                US4::US4_AssigneePair => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Pair"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                                US4::US4_AssigneeUser => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                            };
                            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) }, v37));
                            v38
                        }
                        US3::US3_Unassigned => {
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unassigned"); } LIT.with(|lit| lit.clone()) };
                            v32
                        }
                    };
                    US1::US1_Valid(v40)
                }
            }
        }
        US0::US0_FieldEstimate => {
            let mut v2: US6 = parse_planned_estimate_4(v1);
            match v2 {
                US6::US6_Invalid => {
                    US1::US1_Invalid
                }
                US6::US6_Valid(v3) => {
                    let v3: US7 = v3;
                    let v10: Rc<str> = match v3 {
                        US7::US7_EstimateMinutes(v5) => {
                            let v5: i32 = v5;
                            let v6: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
                            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) }, v6));
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) }));
                            v8
                        }
                        US7::US7_Unestimated => {
                            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unestimated"); } LIT.with(|lit| lit.clone()) };
                            v4
                        }
                    };
                    US1::US1_Valid(v10)
                }
            }
        }
        US0::US0_FieldLoad => {
            let mut v15: US10 = parse_planned_load_7(v1);
            match v15 {
                US10::US10_Invalid => {
                    US1::US1_Invalid
                }
                US10::US10_Valid(v16) => {
                    let v16: US11 = v16;
                    let v25: Rc<str> = match v16 {
                        US11::US11_LoadUnset => {
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LoadUnset"); } LIT.with(|lit| lit.clone()) };
                            v17
                        }
                        US11::US11_PlannedLoad(v18) => {
                            let v18: US12 = v18;
                            let v22: Rc<str> = match v18 {
                                US12::US12_LoadHeavy => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Heavy"); } LIT.with(|lit| lit.clone()) };
                                    v21
                                }
                                US12::US12_LoadLight => {
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Light"); } LIT.with(|lit| lit.clone()) };
                                    v19
                                }
                                US12::US12_LoadMedium => {
                                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Medium"); } LIT.with(|lit| lit.clone()) };
                                    v20
                                }
                            };
                            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) }, v22));
                            v23
                        }
                    };
                    US1::US1_Valid(v25)
                }
            }
        }
    }
}
fn field_value_token_1(mut v0: US0, mut v1: Rc<str>) -> (i32, Rc<str>) {
    let mut v2: US1 = planned_value_token_2(v0.clone(), v1);
    match v2 {
        US1::US1_Invalid => {
            let mut v9: Rc<str> = match v0 {
                US0::US0_FieldAssignee => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee must be agent, pair, user or none"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
                US0::US0_FieldEstimate => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate must be whole minutes (360, 360m) or hours (6h), at most 999999 minutes, or none"); } LIT.with(|lit| lit.clone()) };
                    v4
                }
                US0::US0_FieldLoad => {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load must be light, medium, heavy or none"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
            };
            (1i32, v9)
        }
        US1::US1_Valid(v3) => {
            let v3: Rc<str> = v3;
            (0i32, v3)
        }
    }
}
fn planning_value_token_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, Rc<str>) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let v4: bool = &*v0 == &*v3;
    let v8: US14 = if v4 {
        let v5: US0 = US0::US0_FieldEstimate;
        US14::US14_Valid(v5)
    } else {
        US14::US14_Invalid
    };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let v11: bool = &*v0 == &*v10;
    let mut v14: US14 = if v11 {
        let v12: US0 = US0::US0_FieldLoad;
        US14::US14_Valid(v12)
    } else {
        v8
    };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
    let v17: bool = &*v0 == &*v16;
    let mut v20: US14 = if v17 {
        let v18: US0 = US0::US0_FieldAssignee;
        US14::US14_Valid(v18)
    } else {
        v14
    };
    match v20 {
        US14::US14_Invalid => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("field must be estimate, load or assignee"); } LIT.with(|lit| lit.clone()) };
            (1i32, v24)
        }
        US14::US14_Valid(v21) => {
            let v21: US0 = v21;
            field_value_token_1(v21, v1)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (i32, Rc<str>) {
        planning_value_token_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn legacy_agent_value_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 == "yes";
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: bool = &*v0 == "pair";
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pair"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let v5: bool = &*v0 == "no";
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        legacy_agent_value_8(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_value_token(v0: &str, v1: &str) -> (i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_planning_legacy_agent_value(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
