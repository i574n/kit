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
    US1_TableQuery,
    US1_TableDiff,
    US1_TableUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_StepTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool),
    US2_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US4_TableUnreadable(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_WhereEqual,
    US5_WhereNotEqual,
    US5_WhereGreater,
    US5_WhereGreaterEqual,
    US5_WhereLess,
    US5_WhereLessEqual,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(i32, US5, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>, US5, Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(f64),
    US7_None,
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: i32 = kit_table_measures::kit_table_measured(&*v0, &*v1, &*v3, &*v6, v5, v8);
    v9
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
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
    let v0: i32 = command_line_argument_count_3();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn no_request_6() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn each_prefixed_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).map(|item| format!("{}{}", &*v0, item.trim())).collect::<Vec<_>>().join(",").as_str());
    v2.clone()
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_10(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method11(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn option_applied_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> US2 {
    let v10: bool = &*v8 == "--where";
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let v12: bool = v9.contains(&*v11);
        let v15: bool = if v12 {
            true
        } else {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = v9.contains(&*v13);
            v14
        };
        let v18: bool = if v15 {
            true
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v9.contains(&*v16);
            v17
        };
        if v18 {
            let v19: bool = &*v0 == "";
            let v23: Rc<str> = if v19 {
                v9.clone()
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v20, &*v9, &*v21, &*v21].concat());
                v22.clone()
            };
            US2::US2_StepTaken(v23.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--where expects col=value (or col!=value, col>value, col>=value, col<value, col<=value), got "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v9, &*v26, &*v26, &*v26].concat());
            US2::US2_StepRejected(v27.clone())
        }
    } else {
        let v30: bool = &*v8 == "--cols";
        if v30 {
            US2::US2_StepTaken(v0.clone(), v9.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
        } else {
            let v32: bool = &*v8 == "--group";
            if v32 {
                US2::US2_StepTaken(v0.clone(), v1.clone(), v9.clone(), v3.clone(), v4, v5.clone(), v6.clone(), v7)
            } else {
                let v34: bool = &*v8 == "--key";
                if v34 {
                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v9.clone(), v4, v5.clone(), v6.clone(), v7)
                } else {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mean|--sd|--sum|--min|--max|--count"); } LIT.with(|lit| lit.clone()) };
                    let v37: bool = v36.split("|").any(|item| item == &*v8);
                    if v37 {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from((v8).trim_start_matches(&*v38));
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v40, &*v41, &*v41, &*v41].concat());
                        let v43: Rc<str> = each_prefixed_9(v42.clone(), v9.clone());
                        let v44: bool = &*v5 == "";
                        let v49: Rc<str> = if v44 {
                            v43.clone()
                        } else {
                            let v45: bool = &*v43 == "";
                            if v45 {
                                v5.clone()
                            } else {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                let v47: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v46, &*v43, &*v41, &*v41].concat());
                                v47.clone()
                            }
                        };
                        US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v49.clone(), v6.clone(), v7)
                    } else {
                        let v51: bool = &*v8 == "--latest-by";
                        if v51 {
                            US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v9.clone(), v7)
                        } else {
                            let v53: bool = &*v8 == "--limit";
                            if v53 {
                                let v54: u64 = decimal_u64_10(v9.clone());
                                let v55: bool = v54 == 0u64;
                                if v55 {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit expects a positive integer"); } LIT.with(|lit| lit.clone()) };
                                    US2::US2_StepRejected(v56.clone())
                                } else {
                                    US2::US2_StepTaken(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v54, v5.clone(), v6.clone(), v7)
                                }
                            } else {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v62: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v8, &*v61, &*v61, &*v61].concat());
                                US2::US2_StepRejected(v62.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn options_from_7(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: bool) -> US2 {
    loop {
        let v10: bool = v0 < v1;
        if v10 {
            let v11: Rc<str> = argument_2(v0);
            let v12: bool = &*v11 == "--quiet";
            if v12 {
                let mut v13: i32 = v0.wrapping_add(1i32);
                let mut v14: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v13, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v14);
                continue;
            } else {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = v16 < v1;
                if v17 {
                    let v18: Rc<str> = argument_2(v0);
                    let v19: Rc<str> = argument_2(v16);
                    let mut v20: US2 = option_applied_8(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9, v18.clone(), v19.clone());
                    match &v20 {
                        US2::US2_StepRejected(v31) => {
                            let v31: Rc<str> = v31.clone();
                            return US2::US2_StepRejected(v31.clone());
                        }
                        US2::US2_StepTaken(v21, v22, v23, v24, v25, v26, v27, v28) => {
                            let v21: Rc<str> = v21.clone();
                            let v22: Rc<str> = v22.clone();
                            let v23: Rc<str> = v23.clone();
                            let v24: Rc<str> = v24.clone();
                            let v25: u64 = *v25;
                            let v26: Rc<str> = v26.clone();
                            let v27: Rc<str> = v27.clone();
                            let mut v28: bool = *v28;
                            let mut v29: i32 = v0.wrapping_add(2i32);
                            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v29, v1, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25, v26.clone(), v27.clone(), v28);
                            continue;
                        }
                    }
                } else {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = argument_2(v0);
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v35, &*v36, &*v37, &*v38, &*v38].concat());
                    return US2::US2_StepRejected(v39.clone());
                }
            }
        } else {
            return US2::US2_StepTaken(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone(), v9);
        }
    }
}
fn operator_from_13(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 >= v2;
        if v3 {
            return -1i32;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'<';
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'>';
                v6
            };
            let v9: bool = if v7 {
                true
            } else {
                let v8: bool = v4 == b'=';
                v8
            };
            let v11: bool = if v9 {
                true
            } else {
                let v10: bool = v4 == b'!';
                v10
            };
            if v11 {
                return v1;
            } else {
                let v12: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v12);
                continue;
            }
        }
    }
}
fn filters_from_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v7, mut v8): (Rc<UH0>, Rc<str>) = filters_from_12(v0.clone(), v6.clone());
        let v9: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v10: i32 = 0i32;
        let v11: i32 = operator_from_13(v9.clone(), v10);
        let v12: bool = v11 < 0i32;
        let v54: US6 = if v12 {
            US6::US6_None
        } else {
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: i32 = (v9.clone().len() as i32);
            let v16: bool = v14 < v15;
            let v19: bool = if v16 {
                let v17: u8 = v9.clone().as_bytes()[v14 as usize];
                let v18: bool = v17 == b'=';
                v18
            } else {
                false
            };
            let v22: bool = if v19 {
                let mut v20: u8 = v9.clone().as_bytes()[v11 as usize];
                let v21: bool = v20 != b'=';
                v21
            } else {
                false
            };
            let v23: i32 = if v22 {
                2i32
            } else {
                1i32
            };
            let v24: usize = ((0i32) as usize);
            let v25: usize = ((v11) as usize);
            let v26: Rc<str> = Rc::<str>::from(&(v9)[v24..v25]);
            let v27: usize = ((v11) as usize);
            let mut v28: i32 = v11.wrapping_add(v23);
            let mut v29: usize = ((v28) as usize);
            let v30: Rc<str> = Rc::<str>::from(&(v9)[v27..v29]);
            let v31: bool = &*v30 == "!=";
            let v46: US5 = if v31 {
                US5::US5_WhereNotEqual
            } else {
                let v33: bool = &*v30 == ">";
                if v33 {
                    US5::US5_WhereGreater
                } else {
                    let v35: bool = &*v30 == ">=";
                    if v35 {
                        US5::US5_WhereGreaterEqual
                    } else {
                        let v37: bool = &*v30 == "<";
                        if v37 {
                            US5::US5_WhereLess
                        } else {
                            let v39: bool = &*v30 == "<=";
                            if v39 {
                                US5::US5_WhereLessEqual
                            } else {
                                US5::US5_WhereEqual
                            }
                        }
                    }
                }
            };
            let v47: i32 = v15.wrapping_sub(v11);
            let v48: i32 = v47.wrapping_sub(v23);
            let v49: usize = ((v28) as usize);
            let v50: i32 = v28.wrapping_add(v48);
            let v51: usize = ((v50) as usize);
            let v52: Rc<str> = Rc::<str>::from(&(v9)[v49..v51]);
            US6::US6_Some(v26.clone(), v46.clone(), v52.clone())
        };
        match &v54 {
            US6::US6_None => {
                (v7.clone(), v8.clone())
            }
            US6::US6_Some(v55, v56, v57) => {
                let v55: Rc<str> = v55.clone();
                let v56: US5 = v56.clone();
                let v57: Rc<str> = v57.clone();
                let v58: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v55)).map(|index| index as i32).unwrap_or(-1);
                let v59: bool = v58 < 0i32;
                if v59 {
                    let v60: bool = &*v55 == "";
                    let v66: Rc<str> = if v60 {
                        v8.clone()
                    } else {
                        let v61: bool = &*v8 == "";
                        if v61 {
                            v55.clone()
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v62, &*v8, &*v63, &*v63].concat());
                            v64.clone()
                        }
                    };
                    (v7.clone(), v66.clone())
                } else {
                    let v67: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v58, v56.clone(), v57.clone(), v7.clone()));
                    (v67.clone(), v8.clone())
                }
            }
        }
    }
}
fn all_columns_14(mut v0: i32, mut v1: i32) -> Rc<UH1> {
    let v2: bool = v1 >= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: i32 = v1.wrapping_add(1i32);
        let v5: Rc<UH1> = all_columns_14(v0, v4);
        Rc::new(UH1::UH1_Cons(v1, v5.clone()))
    }
}
fn named_columns_15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<UH1>, Rc<str>) {
    let mut v3: bool = &*v1 == "";
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        (v4.clone(), v2.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v7: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let v8: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let v9: bool = v7 < 0i32;
        let v17: Rc<str> = if v9 {
            let v10: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
            let v11: bool = &*v2 == "";
            if v11 {
                v10.clone()
            } else {
                let v12: bool = &*v10 == "";
                if v12 {
                    v2.clone()
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v5, &*v10, &*v13, &*v13].concat());
                    v14.clone()
                }
            }
        } else {
            v2.clone()
        };
        let (mut v18, mut v19): (Rc<UH1>, Rc<str>) = named_columns_15(v0.clone(), v8.clone(), v17.clone());
        if v9 {
            (v18.clone(), v19.clone())
        } else {
            let mut v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7, v18.clone()));
            (v20.clone(), v19.clone())
        }
    }
}
fn row_passes_17(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let mut v3: i32 = *v3;
                let mut v4: US5 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v33: bool = match &v4 {
                    US5::US5_WhereEqual => {
                        let v8: bool = &*v7 == &*v5;
                        v8
                    }
                    US5::US5_WhereNotEqual => {
                        let v9: bool = &*v7 != &*v5;
                        v9
                    }
                    _ => {
                        let v10: bool = (v7).trim().parse::<f64>().is_ok();
                        let mut v14: US7 = if v10 {
                            let v11: f64 = (v7).trim().parse::<f64>().unwrap_or(0.0);
                            US7::US7_Some(v11)
                        } else {
                            US7::US7_None
                        };
                        let v15: bool = (v5).trim().parse::<f64>().is_ok();
                        let v19: US7 = if v15 {
                            let v16: f64 = (v5).trim().parse::<f64>().unwrap_or(0.0);
                            US7::US7_Some(v16)
                        } else {
                            US7::US7_None
                        };
                        match &v14 {
                            US7::US7_Some(v20) => {
                                let mut v20: f64 = *v20;
                                match &v19 {
                                    US7::US7_Some(v21) => {
                                        let v21: f64 = *v21;
                                        match &v4 {
                                            US5::US5_WhereGreater => {
                                                let v22: bool = v20 > v21;
                                                v22
                                            }
                                            US5::US5_WhereGreaterEqual => {
                                                let v23: bool = v20 >= v21;
                                                v23
                                            }
                                            US5::US5_WhereLess => {
                                                let v24: bool = v20 < v21;
                                                v24
                                            }
                                            US5::US5_WhereLessEqual => {
                                                let v25: bool = v20 <= v21;
                                                v25
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => {
                                        false
                                    }
                                }
                            }
                            _ => {
                                false
                            }
                        }
                    }
                };
                if v33 {
                    (v0, v1, v2) = (v6.clone(), v1.clone(), v2);
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn grouped_rows_16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32, mut v3: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v4: i32, mut v5: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v6: i32 = ((v0).len().max(1) - 1) as i32;
        let v7: bool = v4 > v6;
        if v7 {
            return v5.clone();
        } else {
            let v8: bool = row_passes_17(v1.clone(), v0.clone(), v4);
            if v8 {
                let v9: i32 = v4.wrapping_add(1i32);
                let v10: Rc<str> = std::rc::Rc::<str>::from((v0).get(v4 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v11: i32 = (v3).borrow().get(&*v10).copied().unwrap_or(-1);
                let v12: bool = v11 < 0i32;
                let mut v14: i32 = if v12 {
                    1i32
                } else {
                    let mut v13: i32 = v11.wrapping_add(1i32);
                    v13
                };
                { (v3).borrow_mut().insert(v10.clone(), v14); };
                let v16: Rc<UH2> = if v12 {
                    Rc::new(UH2::UH2_Cons(v10.clone(), v5.clone()))
                } else {
                    v5.clone()
                };
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v3.clone(), v9, v16.clone());
                continue;
            } else {
                let v18: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v3.clone(), v18, v5.clone());
                continue;
            }
        }
    }
}
fn method21(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure2() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn((char)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> i32> = Rc::new(move |mut v0: (char)| -> i32 {
        let mut v1: char = (v0);
        let v2: usize = v1.len_utf8();
        let mut v3: i32 = ((v2) as i32);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn text_order_from_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v2 >= v3;
        let v7: bool = if v4 {
            let v5: i32 = (v1.clone().len() as i32);
            let v6: bool = v2 >= v5;
            v6
        } else {
            false
        };
        if v7 {
            return 0i32;
        } else {
            if v4 {
                return -1i32;
            } else {
                let v8: i32 = (v1.clone().len() as i32);
                let v9: bool = v2 >= v8;
                if v9 {
                    return 1i32;
                } else {
                    let v10: usize = ((v2) as usize);
                    let v11: Option<char> = (&*v0)[v10..].chars().next();
                    let v12: Option<char> = method21(v11.clone());
                    let mut v13: Rc<dyn Fn((char)) -> u32> = closure2();
                    let mut v14: Option<u32> = v12.map(|x| v13(x));
                    let v16: u32 = v14.unwrap_or(0u32);
                    let v17: usize = ((v2) as usize);
                    let v18: Option<char> = (&*v1)[v17..].chars().next();
                    let v19: Option<char> = method21(v18.clone());
                    let mut v20: Option<u32> = v19.map(|x| v13(x));
                    let v22: u32 = v20.unwrap_or(0u32);
                    let v23: bool = v16 < v22;
                    if v23 {
                        return -1i32;
                    } else {
                        let v24: bool = v16 > v22;
                        if v24 {
                            return 1i32;
                        } else {
                            let v25: usize = ((v2) as usize);
                            let v26: Option<char> = (&*v0)[v25..].chars().next();
                            let v27: Option<char> = method21(v26.clone());
                            let mut v28: Rc<dyn Fn((char)) -> i32> = closure3();
                            let mut v29: Option<i32> = v27.map(|x| v28(x));
                            let v31: i32 = v29.unwrap_or(1i32);
                            let v32: i32 = v2.wrapping_add(v31);
                            (v0, v1, v2) = (v0.clone(), v1.clone(), v32);
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn ranked_19(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<str>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH2::UH2_Cons(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<UH2> = v6.clone();
            let v7: i32 = (v0).borrow().get(&*v1).copied().unwrap_or(-1);
            let v8: i32 = (v0).borrow().get(&*v5).copied().unwrap_or(-1);
            let v9: bool = v7 > v8;
            let v15: bool = if v9 {
                true
            } else {
                let v10: bool = v7 == v8;
                if v10 {
                    let v11: i32 = 0i32;
                    let v12: i32 = text_order_from_20(v1.clone(), v5.clone(), v11);
                    let mut v13: bool = v12 < 0i32;
                    v13
                } else {
                    false
                }
            };
            if v15 {
                Rc::new(UH2::UH2_Cons(v1.clone(), v2.clone()))
            } else {
                let v17: Rc<UH2> = ranked_19(v0.clone(), v1.clone(), v6.clone());
                Rc::new(UH2::UH2_Cons(v5.clone(), v17.clone()))
            }
        }
        UH2::UH2_Nil => {
            let mut v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v1.clone(), v3.clone()))
        }
    }
}
fn method18(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH2> = v4.clone();
                let v5: Rc<UH2> = ranked_19(v0.clone(), v3.clone(), v2.clone());
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v2.clone();
            }
        }
    }
}
fn printed_groups_22(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let v6: bool = (v3 as u64) < v2;
                let v7: bool = v6 == false;
                if v7 {
                    return v3;
                } else {
                    let v8: i32 = (v0).borrow().get(&*v4).copied().unwrap_or(-1);
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string().as_str());
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v4, &*v11, &*v11].concat());
                    println!("{}", &*v12);
                    let mut v13: i32 = v3.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v5.clone(), v2, v13);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method23(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH2> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method24(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH2> = v4.clone();
                let v5: i32 = (v0).borrow().get(&*v3).copied().unwrap_or(-1);
                let v6: i32 = v2.wrapping_add(v5);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn row_text_25(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<str> {
    match &*v2 {
        UH1::UH1_Cons(v4, v5) => {
            let mut v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            match &*v5 {
                UH1::UH1_Nil => {
                    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v4 as usize)).map(|cell| &**cell).unwrap_or(""));
                    v6.clone()
                }
                _ => {
                    let v7: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v4 as usize)).map(|cell| &**cell).unwrap_or(""));
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v9: Rc<str> = row_text_25(v0.clone(), v1, v5.clone());
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v10, &*v10].concat());
                    v11.clone()
                }
            }
        }
        UH1::UH1_Nil => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    }
}
fn listed_rows_26(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32) -> (i32, i32, i32) {
    let v8: i32 = ((v0).len().max(1) - 1) as i32;
    let v9: bool = v4 > v8;
    let (mut v30, mut v31, mut v32): (i32, i32, i32) = if v9 {
        (v5, v6, v7)
    } else {
        let v10: bool = row_passes_17(v1.clone(), v0.clone(), v4);
        let v11: bool = v10 == false;
        let (mut v27, mut v28, mut v29): (i32, i32, i32) = if v11 {
            let v12: i32 = v4.wrapping_add(1i32);
            let mut v13: i32 = v5.wrapping_add(1i32);
            let (mut v14, mut v15, mut v16): (i32, i32, i32) = listed_rows_26(v0.clone(), v1.clone(), v2.clone(), v3, v12, v13, v6, v7);
            (v14, v15, v16)
        } else {
            let v17: bool = (v7 as u64) < v3;
            if v17 {
                let v18: Rc<str> = row_text_25(v0.clone(), v4, v2.clone());
                println!("{}", &*v18);
                ()
            };
            let v19: i32 = v5.wrapping_add(1i32);
            let mut v20: i32 = v6.wrapping_add(1i32);
            let v21: i32 = if v17 {
                1i32
            } else {
                0i32
            };
            let v22: i32 = v7.wrapping_add(v21);
            let v23: i32 = v4.wrapping_add(1i32);
            let (mut v24, mut v25, mut v26): (i32, i32, i32) = listed_rows_26(v0, v1, v2, v3, v23, v19, v20, v22);
            (v24, v25, v26)
        };
        (v27, v28, v29)
    };
    (v30, v31, v32)
}
fn table_run_0() -> i32 {
    let v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_2(v0);
    let v2: bool = &*v1 == "query";
    let v8: US1 = if v2 {
        US1::US1_TableQuery
    } else {
        let mut v4: bool = &*v1 == "diff";
        if v4 {
            US1::US1_TableDiff
        } else {
            US1::US1_TableUnknown
        }
    };
    match &v8 {
        US1::US1_TableDiff => {
            let v144: i32 = argument_count_5();
            let v145: bool = v144 < 5i32;
            if v145 {
                let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v149: Rc<str> = std::rc::Rc::<str>::from([&*v147, &*v146, &*v148, &*v148, &*v148].concat());
                eprintln!("{}", &*v149);
                2i32
            } else {
                let v150: i32 = 5i32;
                let v151: i32 = argument_count_5();
                let (mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_6();
                let v160: US2 = options_from_7(v150, v151, v152.clone(), v153.clone(), v154.clone(), v155.clone(), v156, v157.clone(), v158.clone(), v159);
                match &v160 {
                    US2::US2_StepRejected(v174) => {
                        let v174: Rc<str> = v174.clone();
                        let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v177: Rc<str> = std::rc::Rc::<str>::from([&*v175, &*v174, &*v176, &*v176, &*v176].concat());
                        eprintln!("{}", &*v177);
                        2i32
                    }
                    US2::US2_StepTaken(v161, v162, v163, v164, v165, v166, v167, v168) => {
                        let v161: Rc<str> = v161.clone();
                        let v162: Rc<str> = v162.clone();
                        let v163: Rc<str> = v163.clone();
                        let v164: Rc<str> = v164.clone();
                        let v165: u64 = *v165;
                        let v166: Rc<str> = v166.clone();
                        let v167: Rc<str> = v167.clone();
                        let v168: bool = *v168;
                        let v169: i32 = 3i32;
                        let v170: Rc<str> = argument_2(v169);
                        let v171: i32 = 4i32;
                        let v172: Rc<str> = argument_2(v171);
                        let v173: i32 = kit_table_diff::kit_table_diff_run(&*v170, &*v172, &*v164, &*v162, &*v167, v165);
                        v173
                    }
                }
            }
        }
        US1::US1_TableQuery => {
            let v9: i32 = argument_count_5();
            let v10: bool = v9 < 4i32;
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v13, &*v13, &*v13].concat());
                eprintln!("{}", &*v14);
                2i32
            } else {
                let v15: i32 = 4i32;
                let v16: i32 = argument_count_5();
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_6();
                let v25: US2 = options_from_7(v15, v16, v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21, v22.clone(), v23.clone(), v24);
                match &v25 {
                    US2::US2_StepRejected(v137) => {
                        let v137: Rc<str> = v137.clone();
                        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                        let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v140: Rc<str> = std::rc::Rc::<str>::from([&*v138, &*v137, &*v139, &*v139, &*v139].concat());
                        eprintln!("{}", &*v140);
                        2i32
                    }
                    US2::US2_StepTaken(v26, v27, v28, v29, v30, v31, v32, v33) => {
                        let v26: Rc<str> = v26.clone();
                        let v27: Rc<str> = v27.clone();
                        let mut v28: Rc<str> = v28.clone();
                        let mut v29: Rc<str> = v29.clone();
                        let v30: u64 = *v30;
                        let v31: Rc<str> = v31.clone();
                        let v32: Rc<str> = v32.clone();
                        let v33: bool = *v33;
                        let v34: bool = &*v31 != "";
                        if v34 {
                            let v35: i32 = 3i32;
                            let v36: Rc<str> = argument_2(v35);
                            measured_query_1(v36.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30, v31.clone(), v32.clone(), v33)
                        } else {
                            let v38: i32 = 3i32;
                            let v39: Rc<str> = argument_2(v38);
                            let v40: Rc<str> = kit_table_text::kit_table_problem(&*v39);
                            let v41: bool = &*v40 != "";
                            let v45: US4 = if v41 {
                                US4::US4_TableUnreadable(v40.clone())
                            } else {
                                let v43: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v39).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
                                US4::US4_TableLoaded(v43.clone())
                            };
                            match &v45 {
                                US4::US4_TableLoaded(v50) => {
                                    let v50: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v50.clone();
                                    let (mut v51, mut v52): (Rc<UH0>, Rc<str>) = filters_from_12(v50.clone(), v26.clone());
                                    let v53: bool = &*v27 == "";
                                    let (mut v59, mut v60): (Rc<UH1>, Rc<str>) = if v53 {
                                        let v54: i32 = (v50).get(0i32 as usize).map_or(0, |r| r.len()) as i32;
                                        let v55: i32 = 0i32;
                                        let v56: Rc<UH1> = all_columns_14(v54, v55);
                                        (v56.clone(), v52.clone())
                                    } else {
                                        named_columns_15(v50.clone(), v27.clone(), v52.clone())
                                    };
                                    let v61: bool = &*v28 == "";
                                    let v63: i32 = if v61 {
                                        -1i32
                                    } else {
                                        let v62: i32 = (v50).first().and_then(|header| header.iter().position(|item| &**item == &*v28)).map(|index| index as i32).unwrap_or(-1);
                                        v62
                                    };
                                    let v64: bool = &*v28 != "";
                                    let v66: bool = if v64 {
                                        let v65: bool = v63 < 0i32;
                                        v65
                                    } else {
                                        false
                                    };
                                    let v73: Rc<str> = if v66 {
                                        let v67: bool = &*v60 == "";
                                        if v67 {
                                            v28.clone()
                                        } else {
                                            if v61 {
                                                v60.clone()
                                            } else {
                                                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let v70: Rc<str> = std::rc::Rc::<str>::from([&*v60, &*v68, &*v28, &*v69, &*v69].concat());
                                                v70.clone()
                                            }
                                        }
                                    } else {
                                        v60.clone()
                                    };
                                    let v74: bool = &*v73 != "";
                                    if v74 {
                                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) };
                                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                        let v78: Rc<str> = std::rc::Rc::<str>::from((v50).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v77)).unwrap_or_default().as_str());
                                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                        let v80: Rc<str> = std::rc::Rc::<str>::from([&*v75, &*v73, &*v76, &*v78, &*v79].concat());
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                                        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v83: Rc<str> = std::rc::Rc::<str>::from([&*v81, &*v80, &*v82, &*v82, &*v82].concat());
                                        eprintln!("{}", &*v83);
                                        2i32
                                    } else {
                                        let v84: bool = v63 >= 0i32;
                                        if v84 {
                                            let v85: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
                                            let v86: i32 = 1i32;
                                            let v87: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                                            let v88: Rc<UH2> = grouped_rows_16(v50.clone(), v51.clone(), v63, v85.clone(), v86, v87.clone());
                                            let v89: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                                            let v90: Rc<UH2> = method18(v85.clone(), v88.clone(), v89.clone());
                                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count\t"); } LIT.with(|lit| lit.clone()) };
                                            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v93: Rc<str> = std::rc::Rc::<str>::from([&*v91, &*v28, &*v92, &*v92, &*v92].concat());
                                            println!("{}", &*v93);
                                            let v94: i32 = 0i32;
                                            let v95: i32 = printed_groups_22(v85.clone(), v90.clone(), v30, v94);
                                            let v96: bool = v33 == false;
                                            if v96 {
                                                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) };
                                                let v98: i32 = 0i32;
                                                let v99: i32 = method23(v88.clone(), v98);
                                                let v100: Rc<str> = std::rc::Rc::<str>::from((v99).to_string().as_str());
                                                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
                                                let v102: Rc<str> = std::rc::Rc::<str>::from((v95).to_string().as_str());
                                                let v103: Rc<str> = std::rc::Rc::<str>::from([&*v97, &*v100, &*v101, &*v102, &*v92].concat());
                                                let v104: i32 = ((v50).len().max(1) - 1) as i32;
                                                let v105: i32 = 0i32;
                                                let v106: i32 = method24(v85.clone(), v88.clone(), v105);
                                                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) };
                                                let v108: Rc<str> = std::rc::Rc::<str>::from((v104).to_string().as_str());
                                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
                                                let v110: Rc<str> = std::rc::Rc::<str>::from((v106).to_string().as_str());
                                                let v111: Rc<str> = std::rc::Rc::<str>::from([&*v107, &*v108, &*v109, &*v110, &*v92].concat());
                                                let v112: Rc<str> = std::rc::Rc::<str>::from([&*v111, &*v103, &*v92, &*v92, &*v92].concat());
                                                println!("{}", &*v112);
                                                ()
                                            };
                                            0i32
                                        } else {
                                            let v113: i32 = 0i32;
                                            let v114: Rc<str> = row_text_25(v50.clone(), v113, v59.clone());
                                            println!("{}", &*v114);
                                            let v115: i32 = 1i32;
                                            let v116: i32 = 0i32;
                                            let v117: i32 = 0i32;
                                            let v118: i32 = 0i32;
                                            let (mut v119, mut v120, mut v121): (i32, i32, i32) = listed_rows_26(v50.clone(), v51.clone(), v59.clone(), v30, v115, v116, v117, v118);
                                            let v122: bool = v33 == false;
                                            if v122 {
                                                let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) };
                                                let v124: Rc<str> = std::rc::Rc::<str>::from((v119).to_string().as_str());
                                                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
                                                let v126: Rc<str> = std::rc::Rc::<str>::from((v120).to_string().as_str());
                                                let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                let v128: Rc<str> = std::rc::Rc::<str>::from([&*v123, &*v124, &*v125, &*v126, &*v127].concat());
                                                let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
                                                let v130: Rc<str> = std::rc::Rc::<str>::from((v121).to_string().as_str());
                                                let v131: Rc<str> = std::rc::Rc::<str>::from([&*v128, &*v129, &*v130, &*v127, &*v127].concat());
                                                println!("{}", &*v131);
                                                ()
                                            };
                                            0i32
                                        }
                                    }
                                }
                                US4::US4_TableUnreadable(v46) => {
                                    let v46: Rc<str> = v46.clone();
                                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v49: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v46, &*v48, &*v48, &*v48].concat());
                                    eprintln!("{}", &*v49);
                                    2i32
                                }
                            }
                        }
                    }
                }
            }
        }
        US1::US1_TableUnknown => {
            let v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | table diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n]"); } LIT.with(|lit| lit.clone()) };
            let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v184: Rc<str> = std::rc::Rc::<str>::from([&*v182, &*v181, &*v183, &*v183, &*v183].concat());
            eprintln!("{}", &*v184);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn kept_mask_28(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v2 > v3;
        if v5 {
            return v4.clone();
        } else {
            let v6: i32 = v2.wrapping_add(1i32);
            let v7: bool = row_passes_17(v0.clone(), v1.clone(), v2);
            let v10: Rc<str> = if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v10, &*v11, &*v11, &*v11].concat());
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v6, v3, v12.clone());
            continue;
        }
    }
}
fn rows_kept_27(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let v2: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let mut v3: bool = &*v2 != "";
    let v7: US4 = if v3 {
        US4::US4_TableUnreadable(v2.clone())
    } else {
        let v5: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US4::US4_TableLoaded(v5.clone())
    };
    match &v7 {
        US4::US4_TableLoaded(v9) => {
            let v9: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v9.clone();
            let (mut v10, mut v11): (Rc<UH0>, Rc<str>) = filters_from_12(v9.clone(), v1.clone());
            let v12: bool = &*v11 != "";
            if v12 {
                (false, v11.clone())
            } else {
                let mut v13: i32 = 1i32;
                let mut v14: i32 = ((v9).len().max(1) - 1) as i32;
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = kept_mask_28(v10.clone(), v9.clone(), v13, v14, v15.clone());
                (true, v16.clone())
            }
        }
        US4::US4_TableUnreadable(v8) => {
            let v8: Rc<str> = v8.clone();
            (false, v8.clone())
        }
    }
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        rows_kept_27(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_run() -> i32 {
    closure0()()
}
pub fn kit_table_rows_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
