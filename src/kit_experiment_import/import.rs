#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_WelchTwoSided,
    US1_OneSampleT,
    US1_BinomialAtMost,
    US1_WilsonBand,
    US1_MedianBand,
    US1_BinomialBand,
}
#[derive(Clone)]
enum US0 {
    US0_TestKnown(US1),
    US0_TestUnknown,
}
#[derive(Clone)]
enum US2 {
    US2_PlanAccepted,
    US2_PlanRejected(Rc<str>),
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn stderr_line_3(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn failed_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: experiment "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = concat5_2(v1, v0, v2.clone(), v2.clone(), v2);
    stderr_line_3(v3);
    2i32
}
fn identifier_ok_4(mut v0: Rc<str>) -> bool {
    let v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8));
    v1
}
fn test_of_5(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "welch";
    if v1 {
        let v2: US1 = US1::US1_WelchTwoSided;
        US0::US0_TestKnown(v2)
    } else {
        let v4: bool = &*v0 == "one-sample";
        if v4 {
            let v5: US1 = US1::US1_OneSampleT;
            US0::US0_TestKnown(v5)
        } else {
            let v7: bool = &*v0 == "binomial";
            if v7 {
                let v8: US1 = US1::US1_BinomialAtMost;
                US0::US0_TestKnown(v8)
            } else {
                let v10: bool = &*v0 == "wilson";
                if v10 {
                    let v11: US1 = US1::US1_WilsonBand;
                    US0::US0_TestKnown(v11)
                } else {
                    let v13: bool = &*v0 == "median";
                    if v13 {
                        let v14: US1 = US1::US1_MedianBand;
                        US0::US0_TestKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "binomial-band";
                        if v16 {
                            let v17: US1 = US1::US1_BinomialBand;
                            US0::US0_TestKnown(v17)
                        } else {
                            US0::US0_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn decimal_i32_6(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_8(mut v0: US1) -> i32 {
    match v0 {
        US1::US1_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_9(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_10(mut v0: US1) -> bool {
    match v0 {
        US1::US1_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_7(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US2 {
    let v7: bool = &*v5 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US2::US2_PlanRejected(v8)
    } else {
        let v10: bool = &*v6 == "";
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US2::US2_PlanRejected(v11)
        } else {
            let v13: bool = v3 < 2i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US2::US2_PlanRejected(v14)
            } else {
                let v16: bool = &*v1 == "";
                let v19: bool = if v16 {
                    let v17: i32 = arms_needed_8(v0.clone());
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
                    let v33: Rc<str> = match v0 {
                        US1::US1_BinomialAtMost => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24
                        }
                        US1::US1_BinomialBand => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27
                        }
                        US1::US1_MedianBand => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26
                        }
                        US1::US1_OneSampleT => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23
                        }
                        US1::US1_WelchTwoSided => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22
                        }
                        US1::US1_WilsonBand => {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25
                        }
                    };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = concat5_2(v33, v34, v35.clone(), v35.clone(), v35);
                    US2::US2_PlanRejected(v36)
                } else {
                    let v38: i32 = comma_count_9(v1.clone());
                    let v39: i32 = arms_needed_8(v0.clone());
                    let v40: bool = v38 != v39;
                    let v45: bool = if v40 {
                        let v41: bool = &*v1 != "";
                        if v41 {
                            true
                        } else {
                            let v42: i32 = arms_needed_8(v0.clone());
                            let v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let v57: Rc<str> = match &v0 {
                            US1::US1_BinomialAtMost => {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48
                            }
                            US1::US1_BinomialBand => {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51
                            }
                            US1::US1_MedianBand => {
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50
                            }
                            US1::US1_OneSampleT => {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47
                            }
                            US1::US1_WelchTwoSided => {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46
                            }
                            US1::US1_WilsonBand => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49
                            }
                        };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let v59: i32 = arms_needed_8(v0);
                        let v60: bool = v59 == 2i32;
                        let v63: Rc<str> = if v60 {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("two group values (a,b)"); } LIT.with(|lit| lit.clone()) };
                            v61
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one group value"); } LIT.with(|lit| lit.clone()) };
                            v62
                        };
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = concat5_2(v57, v58, v63, v64.clone(), v64);
                        US2::US2_PlanRejected(v65)
                    } else {
                        let v67: bool = match &v0 {
                            US1::US1_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v70: bool = if v67 {
                            let v68: i32 = comma_count_9(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US2::US2_PlanRejected(v71)
                        } else {
                            let v73: bool = needs_prediction_10(v0.clone());
                            let v75: bool = if v73 {
                                let v74: bool = &*v4 == "";
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let v87: Rc<str> = match v0 {
                                    US1::US1_BinomialAtMost => {
                                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78
                                    }
                                    US1::US1_BinomialBand => {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81
                                    }
                                    US1::US1_MedianBand => {
                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80
                                    }
                                    US1::US1_OneSampleT => {
                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77
                                    }
                                    US1::US1_WelchTwoSided => {
                                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76
                                    }
                                    US1::US1_WilsonBand => {
                                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79
                                    }
                                };
                                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v90: Rc<str> = concat5_2(v87, v88, v89.clone(), v89.clone(), v89);
                                US2::US2_PlanRejected(v90)
                            } else {
                                US2::US2_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn now_ms_11() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn civil_of_days_13(mut v0: u64) -> (u64, u64, u64) {
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
fn utc_stamp_12(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_13(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_2(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_2(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v24, v45, v44, v43.clone(), v43)
}
fn field_line_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2
}
fn joined_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn text_created_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2
}
fn stdout_line_17(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn imported_0() -> i32 {
    let v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_experiment_args::kit_experiment_import_args(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    let v17: bool = &*v16 != "";
    if v17 {
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: "); } LIT.with(|lit| lit.clone()) }, v16));
        failed_1(v18)
    } else {
        let v20: bool = identifier_ok_4(v2.clone());
        let v21: bool = v20 == false;
        if v21 {
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: the ID must be letters, digits, '-' or '_', received '"); } LIT.with(|lit| lit.clone()) }, v2));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
            failed_1(v23)
        } else {
            let v25: bool = { let bytes = (v3).as_bytes(); bytes.len() >= 10 && bytes[..4].iter().all(u8::is_ascii_digit) && bytes[4] == 45u8 && bytes[5..7].iter().all(u8::is_ascii_digit) && bytes[7] == 45u8 && bytes[8..10].iter().all(u8::is_ascii_digit) };
            let v26: bool = v25 == false;
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --registered-at needs the source's registration date or time (YYYY-MM-DD...), as the source states it"); } LIT.with(|lit| lit.clone()) };
                failed_1(v27)
            } else {
                let v29: bool = &*v4 == "";
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --evidence names what attests the registration (a git commit or file@line)"); } LIT.with(|lit| lit.clone()) };
                    failed_1(v30)
                } else {
                    let v32: US0 = test_of_5(v5.clone());
                    match v32 {
                        US0::US0_TestKnown(v36) => {
                            let v36: US1 = v36;
                            let v37: i32 = decimal_i32_6(v6.clone());
                            let v38: US2 = plan_checked_7(v36.clone(), v7.clone(), v8.clone(), v37, v9.clone(), v10.clone(), v11.clone());
                            match v38 {
                                US2::US2_PlanAccepted => {
                                    let v42: u64 = now_ms_11();
                                    let v43: Rc<str> = utc_stamp_12(v42);
                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
                                    let v45: Rc<str> = field_line_14(v44, v2.clone());
                                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered"); } LIT.with(|lit| lit.clone()) };
                                    let v47: Rc<str> = field_line_14(v46, v3.clone());
                                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported"); } LIT.with(|lit| lit.clone()) };
                                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                                    let v51: Rc<str> = field_line_14(v49, v50);
                                    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v51));
                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported_at"); } LIT.with(|lit| lit.clone()) };
                                    let v54: Rc<str> = field_line_14(v53, v43.clone());
                                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v54));
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                                    let v57: Rc<str> = field_line_14(v56, v4.clone());
                                    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v57));
                                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
                                    let v60: Rc<str> = field_line_14(v59, v13);
                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                                    let v73: Rc<str> = match v36 {
                                        US1::US1_BinomialAtMost => {
                                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                            v64
                                        }
                                        US1::US1_BinomialBand => {
                                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                            v67
                                        }
                                        US1::US1_MedianBand => {
                                            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                            v66
                                        }
                                        US1::US1_OneSampleT => {
                                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                            v63
                                        }
                                        US1::US1_WelchTwoSided => {
                                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                            v62
                                        }
                                        US1::US1_WilsonBand => {
                                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                            v65
                                        }
                                    };
                                    let v74: Rc<str> = field_line_14(v61, v73);
                                    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v74));
                                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                                    let v77: Rc<str> = field_line_14(v76, v10);
                                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v77));
                                    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                                    let v80: Rc<str> = field_line_14(v79, v8);
                                    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v80));
                                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                                    let v83: Rc<str> = field_line_14(v82, v7);
                                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v83));
                                    let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                                    let v86: Rc<str> = field_line_14(v85, v6);
                                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                                    let v88: Rc<str> = field_line_14(v87, v9);
                                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v88));
                                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
                                    let v91: Rc<str> = field_line_14(v90, v12);
                                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v91));
                                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                                    let v94: Rc<str> = field_line_14(v93, v11);
                                    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v94));
                                    let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
                                    let v97: Rc<str> = field_line_14(v96, v14);
                                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v97));
                                    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
                                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v102: Rc<str> = concat5_2(v2.clone(), v100, v99, v101.clone(), v101.clone());
                                    let v103: Rc<str> = joined_15(v1, v102);
                                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v84));
                                    let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v98));
                                    let v106: Rc<str> = text_created_16(v103, v105);
                                    let v107: bool = &*v106 != "";
                                    if v107 {
                                        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: "); } LIT.with(|lit| lit.clone()) }, v2));
                                        let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v108, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already in the ledger or the ledger is unwritable ("); } LIT.with(|lit| lit.clone()) }));
                                        let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v106));
                                        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
                                        failed_1(v111)
                                    } else {
                                        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment import ok id="); } LIT.with(|lit| lit.clone()) }, v2));
                                        let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported=yes registered="); } LIT.with(|lit| lit.clone()) }));
                                        let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v3));
                                        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" evidence="); } LIT.with(|lit| lit.clone()) }, v4));
                                        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported_at="); } LIT.with(|lit| lit.clone()) }));
                                        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v43));
                                        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v118));
                                        stdout_line_17(v119);
                                        0i32
                                    }
                                }
                                US2::US2_PlanRejected(v39) => {
                                    let v39: Rc<str> = v39;
                                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: "); } LIT.with(|lit| lit.clone()) }, v39));
                                    failed_1(v40)
                                }
                            }
                        }
                        US0::US0_TestUnknown => {
                            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import: --test must be welch|one-sample|binomial|wilson|median|binomial-band, received '"); } LIT.with(|lit| lit.clone()) }, v5));
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
                            failed_1(v34)
                        }
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
fn file_text_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn plan_text_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v1.clone(), v3.clone(), v2, v4.clone(), v4.clone());
    let v6: Rc<str> = joined_15(v0.clone(), v5);
    let v7: Rc<str> = file_text_20(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = concat5_2(v1, v3, v8, v4.clone(), v4.clone());
    let v10: Rc<str> = joined_15(v0, v9);
    let v11: Rc<str> = file_text_20(v10);
    concat5_2(v7, v11, v4.clone(), v4.clone(), v4)
}
fn field_of_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2
}
fn verdict_recorded_18() -> i32 {
    let v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_experiment_verb_args::kit_experiment_verdict_args(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    let v7: Rc<str> = plan_text_19(v1.clone(), v2.clone());
    let v8: bool = &*v6 != "";
    if v8 {
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: "); } LIT.with(|lit| lit.clone()) }, v6));
        failed_1(v9)
    } else {
        let v11: bool = &*v7 == "";
        if v11 {
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: no registration for "); } LIT.with(|lit| lit.clone()) }, v2));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            failed_1(v14)
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = field_of_21(v7.clone(), v16.clone());
            let v18: bool = &*v17 != "yes";
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: only imported registrations take a recorded verdict; a live registration is analyzed (experiment analyze)"); } LIT.with(|lit| lit.clone()) };
                failed_1(v19)
            } else {
                let v21: bool = &*v3 == "";
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: --verdict is the historical result as the source states it"); } LIT.with(|lit| lit.clone()) };
                    failed_1(v22)
                } else {
                    let v24: bool = { let bytes = (v4).as_bytes(); bytes.len() >= 10 && bytes[..4].iter().all(u8::is_ascii_digit) && bytes[4] == 45u8 && bytes[5..7].iter().all(u8::is_ascii_digit) && bytes[7] == 45u8 && bytes[8..10].iter().all(u8::is_ascii_digit) };
                    let v25: bool = v24 == false;
                    if v25 {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: --at needs the source's verdict date or time (YYYY-MM-DD...)"); } LIT.with(|lit| lit.clone()) };
                        failed_1(v26)
                    } else {
                        let v28: u64 = now_ms_11();
                        let v29: Rc<str> = utc_stamp_12(v28);
                        let v30: bool = &*v5 == "";
                        let v33: Rc<str> = if v30 {
                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                            field_of_21(v7, v31)
                        } else {
                            v5
                        };
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v37: Rc<str> = concat5_2(v2.clone(), v35, v34.clone(), v36.clone(), v36);
                        let v38: Rc<str> = joined_15(v1, v37);
                        let v39: Rc<str> = field_line_14(v34, v3);
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = field_line_14(v40, v4.clone());
                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = field_line_14(v16, v43);
                        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v44));
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("imported_at"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = field_line_14(v46, v29);
                        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence"); } LIT.with(|lit| lit.clone()) };
                        let v50: Rc<str> = field_line_14(v49, v33);
                        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v50));
                        let v52: Rc<str> = text_created_16(v38, v51);
                        let v53: bool = &*v52 != "";
                        if v53 {
                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record-verdict: "); } LIT.with(|lit| lit.clone()) }, v2));
                            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" already has a verdict ("); } LIT.with(|lit| lit.clone()) }));
                            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v52));
                            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
                            failed_1(v57)
                        } else {
                            let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment record-verdict ok id="); } LIT.with(|lit| lit.clone()) }, v2));
                            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) }));
                            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v4));
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" imported=yes"); } LIT.with(|lit| lit.clone()) }));
                            stdout_line_17(v62);
                            0i32
                        }
                    }
                }
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        verdict_recorded_18()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_import_run() -> i32 {
    closure0()()
}
pub fn kit_experiment_verdict_run() -> i32 {
    closure1()()
}
