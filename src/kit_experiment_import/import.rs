#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_WelchTwoSided,
    US2_OneSampleT,
    US2_BinomialAtMost,
    US2_WilsonBand,
    US2_MedianBand,
    US2_BinomialBand,
}
#[derive(Clone)]
enum US1 {
    US1_TestKnown(US2),
    US1_TestUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_PlanAccepted,
    US3_PlanRejected(Rc<str>),
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn failed_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: experiment "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_2(v3.clone());
    2i32
}
fn command_line_argument_count_4() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_4();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_5(v1);
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
fn option_value_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn identifier_ok_7(mut v0: Rc<str>) -> bool {
    let v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8));
    v1
}
fn test_of_8(mut v0: Rc<str>) -> US1 {
    let v1: bool = &*v0 == "welch";
    if v1 {
        let v2: US2 = US2::US2_WelchTwoSided;
        US1::US1_TestKnown(v2)
    } else {
        let v4: bool = &*v0 == "one-sample";
        if v4 {
            let v5: US2 = US2::US2_OneSampleT;
            US1::US1_TestKnown(v5)
        } else {
            let v7: bool = &*v0 == "binomial";
            if v7 {
                let v8: US2 = US2::US2_BinomialAtMost;
                US1::US1_TestKnown(v8)
            } else {
                let v10: bool = &*v0 == "wilson";
                if v10 {
                    let v11: US2 = US2::US2_WilsonBand;
                    US1::US1_TestKnown(v11)
                } else {
                    let v13: bool = &*v0 == "median";
                    if v13 {
                        let v14: US2 = US2::US2_MedianBand;
                        US1::US1_TestKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "binomial-band";
                        if v16 {
                            let v17: US2 = US2::US2_BinomialBand;
                            US1::US1_TestKnown(v17)
                        } else {
                            US1::US1_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn decimal_i32_9(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_11(mut v0: US2) -> i32 {
    match &v0 {
        US2::US2_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_12(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_13(mut v0: US2) -> bool {
    match &v0 {
        US2::US2_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_10(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US3 {
    let v7: bool = &*v5 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US3::US3_PlanRejected(v8.clone())
    } else {
        let v10: bool = &*v6 == "";
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US3::US3_PlanRejected(v11.clone())
        } else {
            let v13: bool = v3 < 2i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US3::US3_PlanRejected(v14.clone())
            } else {
                let v16: bool = &*v1 == "";
                let v19: bool = if v16 {
                    let v17: i32 = arms_needed_11(v0.clone());
                    let v18: bool = v17 == 1i32;
                    v18
                } else {
                    false
                };
                let v21: bool = if v19 {
                    let v20: bool = &*v2 != "";
                    v20
                } else {
                    false
                };
                if v21 {
                    let v33: Rc<str> = match &v0 {
                        US2::US2_BinomialAtMost => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US2::US2_BinomialBand => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                        US2::US2_MedianBand => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US2::US2_OneSampleT => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US2::US2_WelchTwoSided => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US2::US2_WilsonBand => {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                    };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                    US3::US3_PlanRejected(v36.clone())
                } else {
                    let v38: i32 = comma_count_12(v1.clone());
                    let v39: i32 = arms_needed_11(v0.clone());
                    let v40: bool = v38 != v39;
                    let v45: bool = if v40 {
                        let v41: bool = &*v1 != "";
                        if v41 {
                            true
                        } else {
                            let v42: i32 = arms_needed_11(v0.clone());
                            let v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let v57: Rc<str> = match &v0 {
                            US2::US2_BinomialAtMost => {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            }
                            US2::US2_BinomialBand => {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US2::US2_MedianBand => {
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            US2::US2_OneSampleT => {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47.clone()
                            }
                            US2::US2_WelchTwoSided => {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46.clone()
                            }
                            US2::US2_WilsonBand => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                        };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let v59: i32 = arms_needed_11(v0.clone());
                        let v60: bool = v59 == 2i32;
                        let v63: Rc<str> = if v60 {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("two group values (a,b)"); } LIT.with(|lit| lit.clone()) };
                            v61.clone()
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one group value"); } LIT.with(|lit| lit.clone()) };
                            v62.clone()
                        };
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = std::rc::Rc::<str>::from([&*v57, &*v58, &*v63, &*v64, &*v64].concat());
                        US3::US3_PlanRejected(v65.clone())
                    } else {
                        let v67: bool = match &v0 {
                            US2::US2_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v70: bool = if v67 {
                            let v68: i32 = comma_count_12(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US3::US3_PlanRejected(v71.clone())
                        } else {
                            let v73: bool = needs_prediction_13(v0.clone());
                            let v75: bool = if v73 {
                                let v74: bool = &*v4 == "";
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let v87: Rc<str> = match &v0 {
                                    US2::US2_BinomialAtMost => {
                                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78.clone()
                                    }
                                    US2::US2_BinomialBand => {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81.clone()
                                    }
                                    US2::US2_MedianBand => {
                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80.clone()
                                    }
                                    US2::US2_OneSampleT => {
                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77.clone()
                                    }
                                    US2::US2_WelchTwoSided => {
                                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76.clone()
                                    }
                                    US2::US2_WilsonBand => {
                                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79.clone()
                                    }
                                };
                                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v90: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v88, &*v89, &*v89, &*v89].concat());
                                US3::US3_PlanRejected(v90.clone())
                            } else {
                                US3::US3_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn now_ms_14() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn civil_of_days_16(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
    let v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let v11: u64 = v10.wrapping_div(365u64);
    let v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let v25: u64 = v24.wrapping_add(1u64);
    let v26: bool = v20 < 10u64;
    let v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_15(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_16(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
        v14.clone()
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15.clone()
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        v21.clone()
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22.clone()
    };
    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v16, &*v9, &*v23].concat());
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
        v30.clone()
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31.clone()
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v38, &*v38].concat());
        v39.clone()
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40.clone()
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v41, &*v42, &*v43].concat());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v45, &*v44, &*v43, &*v43].concat());
    v46.clone()
}
fn field_line_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2.clone()
}
fn joined_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn text_created_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2.clone()
}
fn stdout_line_20(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn imported_0() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = argument_3(v0);
    let v2: i32 = 4i32;
    let v3: Rc<str> = argument_3(v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--registered-at"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = option_value_6(v4.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--evidence"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = option_value_6(v6.clone());
    let v8: bool = identifier_ok_7(v3.clone());
    let v9: bool = v8 == false;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: the ID must be letters, digits, '-' or '_', received '"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v3));
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
        failed_1(v13.clone())
    } else {
        let v15: bool = { let bytes = (v5).as_bytes(); bytes.len() >= 10 && bytes[..4].iter().all(u8::is_ascii_digit) && bytes[4] == 45u8 && bytes[5..7].iter().all(u8::is_ascii_digit) && bytes[7] == 45u8 && bytes[8..10].iter().all(u8::is_ascii_digit) };
        let v16: bool = v15 == false;
        if v16 {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --registered-at needs the source's registration date or time (YYYY-MM-DD...), as the source states it"); } LIT.with(|lit| lit.clone()) };
            failed_1(v17.clone())
        } else {
            let v19: bool = &*v7 == "";
            if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --evidence names what attests the registration (a git commit or file@line)"); } LIT.with(|lit| lit.clone()) };
                failed_1(v20.clone())
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = option_value_6(v22.clone());
                let v24: US1 = test_of_8(v23.clone());
                match &v24 {
                    US1::US1_TestKnown(v35) => {
                        let v35: US2 = v35.clone();
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--arms"); } LIT.with(|lit| lit.clone()) };
                        let v37: Rc<str> = option_value_6(v36.clone());
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = option_value_6(v38.clone());
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = option_value_6(v40.clone());
                        let v42: i32 = decimal_i32_9(v41.clone());
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prediction"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = option_value_6(v43.clone());
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = option_value_6(v45.clone());
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = option_value_6(v47.clone());
                        let v49: US3 = plan_checked_10(v35.clone(), v37.clone(), v39.clone(), v42, v44.clone(), v46.clone(), v48.clone());
                        match &v49 {
                            US3::US3_PlanAccepted => {
                                let v54: u64 = now_ms_14();
                                let v55: Rc<str> = utc_stamp_15(v54);
                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--confidence"); } LIT.with(|lit| lit.clone()) };
                                let v57: Rc<str> = option_value_6(v56.clone());
                                let v58: bool = &*v57 == "";
                                let v61: Rc<str> = if v58 {
                                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0.95"); } LIT.with(|lit| lit.clone()) };
                                    v59.clone()
                                } else {
                                    option_value_6(v56.clone())
                                };
                                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
                                let v63: Rc<str> = field_line_17(v62.clone(), v3.clone());
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered"); } LIT.with(|lit| lit.clone()) };
                                let v65: Rc<str> = field_line_17(v64.clone(), v5.clone());
                                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v65));
                                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported"); } LIT.with(|lit| lit.clone()) };
                                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                                let v69: Rc<str> = field_line_17(v67.clone(), v68.clone());
                                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v69));
                                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported_at"); } LIT.with(|lit| lit.clone()) };
                                let v72: Rc<str> = field_line_17(v71.clone(), v55.clone());
                                let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v72));
                                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                                let v75: Rc<str> = field_line_17(v74.clone(), v7.clone());
                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v75));
                                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
                                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--question"); } LIT.with(|lit| lit.clone()) };
                                let v79: Rc<str> = option_value_6(v78.clone());
                                let v80: Rc<str> = field_line_17(v77.clone(), v79.clone());
                                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                                let v93: Rc<str> = match &v35 {
                                    US2::US2_BinomialAtMost => {
                                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v84.clone()
                                    }
                                    US2::US2_BinomialBand => {
                                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v87.clone()
                                    }
                                    US2::US2_MedianBand => {
                                        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v86.clone()
                                    }
                                    US2::US2_OneSampleT => {
                                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v83.clone()
                                    }
                                    US2::US2_WelchTwoSided => {
                                        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v82.clone()
                                    }
                                    US2::US2_WilsonBand => {
                                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v85.clone()
                                    }
                                };
                                let v94: Rc<str> = field_line_17(v81.clone(), v93.clone());
                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v94));
                                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                                let v97: Rc<str> = option_value_6(v45.clone());
                                let v98: Rc<str> = field_line_17(v96.clone(), v97.clone());
                                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v98));
                                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                                let v101: Rc<str> = option_value_6(v38.clone());
                                let v102: Rc<str> = field_line_17(v100.clone(), v101.clone());
                                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v102));
                                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                                let v105: Rc<str> = option_value_6(v36.clone());
                                let v106: Rc<str> = field_line_17(v104.clone(), v105.clone());
                                let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v106));
                                let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                                let v109: Rc<str> = option_value_6(v40.clone());
                                let v110: Rc<str> = field_line_17(v108.clone(), v109.clone());
                                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                                let v112: Rc<str> = option_value_6(v43.clone());
                                let v113: Rc<str> = field_line_17(v111.clone(), v112.clone());
                                let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v113));
                                let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
                                let v116: Rc<str> = field_line_17(v115.clone(), v61.clone());
                                let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v116));
                                let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                                let v119: Rc<str> = option_value_6(v47.clone());
                                let v120: Rc<str> = field_line_17(v118.clone(), v119.clone());
                                let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v120));
                                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
                                let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
                                let v124: Rc<str> = option_value_6(v123.clone());
                                let v125: Rc<str> = field_line_17(v122.clone(), v124.clone());
                                let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v125));
                                let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
                                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v130: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v128, &*v127, &*v129, &*v129].concat());
                                let v131: Rc<str> = joined_18(v1.clone(), v130.clone());
                                let v133: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v76, v107)), v126));
                                let v134: Rc<str> = text_created_19(v131.clone(), v133.clone());
                                let v135: bool = &*v134 != "";
                                if v135 {
                                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: "); } LIT.with(|lit| lit.clone()) };
                                    let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v3));
                                    let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already in the ledger or the ledger is unwritable ("); } LIT.with(|lit| lit.clone()) };
                                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v137, v138)), v134));
                                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                    let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
                                    failed_1(v142.clone())
                                } else {
                                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment import ok id="); } LIT.with(|lit| lit.clone()) };
                                    let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v3));
                                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported=yes registered="); } LIT.with(|lit| lit.clone()) };
                                    let v148: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v145, v146)), v5));
                                    let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" evidence="); } LIT.with(|lit| lit.clone()) };
                                    let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v7));
                                    let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported_at="); } LIT.with(|lit| lit.clone()) };
                                    let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v148, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v150, v151)), v55))));
                                    stdout_line_20(v154.clone());
                                    0i32
                                }
                            }
                            US3::US3_PlanRejected(v50) => {
                                let v50: Rc<str> = v50.clone();
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: "); } LIT.with(|lit| lit.clone()) };
                                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v50));
                                failed_1(v52.clone())
                            }
                        }
                    }
                    US1::US1_TestUnknown => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --test must be "); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --test must be welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --test must be welch|one-sample|binomial|wilson|median|binomial-band, received '"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = option_value_6(v22.clone());
                        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                        failed_1(v33.clone())
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        imported_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_text_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn plan_text_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let v6: Rc<str> = joined_18(v0.clone(), v5.clone());
    let v7: Rc<str> = file_text_23(v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
    let v10: Rc<str> = joined_18(v0.clone(), v9.clone());
    let v11: Rc<str> = file_text_23(v10.clone());
    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v11, &*v4, &*v4, &*v4].concat());
    v12.clone()
}
fn field_of_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2.clone()
}
fn verdict_recorded_21() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = argument_3(v0);
    let v2: i32 = 4i32;
    let v3: Rc<str> = argument_3(v2);
    let v4: Rc<str> = plan_text_22(v1.clone(), v3.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--verdict"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = option_value_6(v5.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--at"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = option_value_6(v7.clone());
    let v9: bool = &*v4 == "";
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: no registration for "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v3));
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v1));
        failed_1(v14.clone())
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported"); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = field_of_24(v4.clone(), v16.clone());
        let v18: bool = &*v17 != "yes";
        if v18 {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: only imported registrations take a recorded verdict; a live registration is analyzed (experiment analyze)"); } LIT.with(|lit| lit.clone()) };
            failed_1(v19.clone())
        } else {
            let v21: bool = &*v6 == "";
            if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: --verdict is the historical result as the source states it"); } LIT.with(|lit| lit.clone()) };
                failed_1(v22.clone())
            } else {
                let v24: bool = { let bytes = (v8).as_bytes(); bytes.len() >= 10 && bytes[..4].iter().all(u8::is_ascii_digit) && bytes[4] == 45u8 && bytes[5..7].iter().all(u8::is_ascii_digit) && bytes[7] == 45u8 && bytes[8..10].iter().all(u8::is_ascii_digit) };
                let v25: bool = v24 == false;
                if v25 {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: --at needs the source's verdict date or time (YYYY-MM-DD...)"); } LIT.with(|lit| lit.clone()) };
                    failed_1(v26.clone())
                } else {
                    let v28: u64 = now_ms_14();
                    let v29: Rc<str> = utc_stamp_15(v28);
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--evidence"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = option_value_6(v30.clone());
                    let v32: bool = &*v31 == "";
                    let v36: Rc<str> = if v32 {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                        field_of_24(v4.clone(), v33.clone())
                    } else {
                        option_value_6(v30.clone())
                    };
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v40: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v38, &*v37, &*v39, &*v39].concat());
                    let v41: Rc<str> = joined_18(v1.clone(), v40.clone());
                    let v42: Rc<str> = field_line_17(v37.clone(), v6.clone());
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at"); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = field_line_17(v43.clone(), v8.clone());
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v44));
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = field_line_17(v16.clone(), v46.clone());
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported_at"); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = field_line_17(v49.clone(), v29.clone());
                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v50));
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                    let v53: Rc<str> = field_line_17(v52.clone(), v36.clone());
                    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v53));
                    let v55: Rc<str> = text_created_19(v41.clone(), v54.clone());
                    let v56: bool = &*v55 != "";
                    if v56 {
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: "); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v3));
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" already has a verdict ("); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v58, v59)), v55));
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                        failed_1(v63.clone())
                    } else {
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment record-verdict ok id="); } LIT.with(|lit| lit.clone()) };
                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v3));
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v66, v67)), v8));
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported=yes"); } LIT.with(|lit| lit.clone()) };
                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                        stdout_line_20(v71.clone());
                        0i32
                    }
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        verdict_recorded_21()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_import_run() -> i32 {
    closure0()()
}
pub fn kit_experiment_verdict_run() -> i32 {
    closure1()()
}
