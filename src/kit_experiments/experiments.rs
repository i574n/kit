#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WelchTwoSided,
    US0_OneSampleT,
    US0_BinomialAtMost,
    US0_WilsonBand,
    US0_MedianBand,
    US0_BinomialBand,
}
#[derive(Clone)]
enum US1 {
    US1_PlanAccepted,
    US1_PlanRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_TestKnown(US0),
    US2_TestUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_VerbRegister,
    US3_VerbAnalyze,
    US3_VerbShow,
    US3_VerbAmend,
    US3_VerbAddendum,
    US3_VerbImport,
    US3_VerbRecordVerdict,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(US3),
    US5_None,
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
fn decimal_i32_7(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_9(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_10(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_11(mut v0: US0) -> bool {
    match v0 {
        US0::US0_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_8(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US1 {
    let v7: bool = &*v5 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US1::US1_PlanRejected(v8)
    } else {
        let v10: bool = &*v6 == "";
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US1::US1_PlanRejected(v11)
        } else {
            let v13: bool = v3 < 2i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US1::US1_PlanRejected(v14)
            } else {
                let v16: bool = &*v1 == "";
                let v19: bool = if v16 {
                    let v17: i32 = arms_needed_9(v0.clone());
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
                        US0::US0_BinomialAtMost => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24
                        }
                        US0::US0_BinomialBand => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27
                        }
                        US0::US0_MedianBand => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26
                        }
                        US0::US0_OneSampleT => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23
                        }
                        US0::US0_WelchTwoSided => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22
                        }
                        US0::US0_WilsonBand => {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25
                        }
                    };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = concat5_2(v33, v34, v35.clone(), v35.clone(), v35);
                    US1::US1_PlanRejected(v36)
                } else {
                    let v38: i32 = comma_count_10(v1.clone());
                    let v39: i32 = arms_needed_9(v0.clone());
                    let v40: bool = v38 != v39;
                    let v45: bool = if v40 {
                        let v41: bool = &*v1 != "";
                        if v41 {
                            true
                        } else {
                            let v42: i32 = arms_needed_9(v0.clone());
                            let v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let v57: Rc<str> = match &v0 {
                            US0::US0_BinomialAtMost => {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48
                            }
                            US0::US0_BinomialBand => {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51
                            }
                            US0::US0_MedianBand => {
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50
                            }
                            US0::US0_OneSampleT => {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47
                            }
                            US0::US0_WelchTwoSided => {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46
                            }
                            US0::US0_WilsonBand => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49
                            }
                        };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let v59: i32 = arms_needed_9(v0);
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
                        US1::US1_PlanRejected(v65)
                    } else {
                        let v67: bool = match &v0 {
                            US0::US0_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v70: bool = if v67 {
                            let v68: i32 = comma_count_10(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US1::US1_PlanRejected(v71)
                        } else {
                            let v73: bool = needs_prediction_11(v0.clone());
                            let v75: bool = if v73 {
                                let v74: bool = &*v4 == "";
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let v87: Rc<str> = match v0 {
                                    US0::US0_BinomialAtMost => {
                                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78
                                    }
                                    US0::US0_BinomialBand => {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81
                                    }
                                    US0::US0_MedianBand => {
                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80
                                    }
                                    US0::US0_OneSampleT => {
                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77
                                    }
                                    US0::US0_WelchTwoSided => {
                                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76
                                    }
                                    US0::US0_WilsonBand => {
                                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79
                                    }
                                };
                                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v90: Rc<str> = concat5_2(v87, v88, v89.clone(), v89.clone(), v89);
                                US1::US1_PlanRejected(v90)
                            } else {
                                US1::US1_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn now_ms_12() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn civil_of_days_14(mut v0: u64) -> (u64, u64, u64) {
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
fn utc_stamp_13(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_14(v2);
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
fn joined_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn field_line_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2
}
fn local_stamp_18() -> Rc<str> {
    let v0: Rc<str> = { #[cfg(windows)] fn local() -> String { #[repr(C)] struct Clock { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(clock: *mut Clock); } let mut clock = Clock { year: 0, month: 0, weekday: 0, day: 0, hour: 0, minute: 0, second: 0, millis: 0 }; unsafe { GetLocalTime(&mut clock); } let _ = clock.weekday; let _ = clock.millis; format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", clock.year, clock.month, clock.day, clock.hour, clock.minute, clock.second) } #[cfg(not(windows))] fn local() -> String { std::process::Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output().ok().map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned()).unwrap_or_default() } std::rc::Rc::<str>::from(local().as_str()) };
    v0
}
fn registration_text_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0, mut v3: &Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: &Rc<str>) -> Rc<str> {
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = field_line_17(v14, v0);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = field_line_17(v16, v1);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = field_line_17(v19, v11);
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = match v2 {
        US0::US0_BinomialAtMost => {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
            v25
        }
        US0::US0_BinomialBand => {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
            v28
        }
        US0::US0_MedianBand => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
            v27
        }
        US0::US0_OneSampleT => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
            v24
        }
        US0::US0_WelchTwoSided => {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
            v23
        }
        US0::US0_WilsonBand => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
            v26
        }
    };
    let v35: Rc<str> = field_line_17(v22, v34);
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v35));
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = field_line_17(v37, v8);
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = field_line_17(v40, v6);
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = field_line_17(v43, v5);
    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v44));
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = field_line_17(v46, v4);
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = field_line_17(v49, v7);
    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v50));
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
    let v53: Rc<str> = field_line_17(v52, v10);
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v53));
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = field_line_17(v55, v9);
    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v56));
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
    let v59: Rc<str> = field_line_17(v58, v12);
    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v59));
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
    let v62: Rc<str> = local_stamp_18();
    let v63: Rc<str> = field_line_17(v61, v62);
    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v63));
    v64
}
fn text_created_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2
}
fn stdout_line_20(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn registered_as_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>) -> i32 {
    let v14: i32 = decimal_i32_7(v4.clone());
    let v15: US1 = plan_checked_8(v2.clone(), v5.clone(), v6.clone(), v14, v7.clone(), v8.clone(), v9.clone());
    match v15 {
        US1::US1_PlanAccepted => {
            let v19: u64 = now_ms_12();
            let v20: Rc<str> = utc_stamp_13(v19);
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = concat5_2(v1.clone(), v22, v21, v23.clone(), v23);
            let v25: Rc<str> = joined_15(v0, v24);
            let v26: Rc<str> = registration_text_16(v1.clone(), v20.clone(), v2.clone(), &v3, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, &v13);
            let v27: Rc<str> = text_created_19(v25, v26);
            let v28: bool = &*v27 != "";
            if v28 {
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) }, v1));
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already registered or the ledger is unwritable ("); } LIT.with(|lit| lit.clone()) }));
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v27));
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
                failed_1(v32)
            } else {
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment register ok id="); } LIT.with(|lit| lit.clone()) }, v1));
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" test="); } LIT.with(|lit| lit.clone()) }));
                let v47: Rc<str> = match v2 {
                    US0::US0_BinomialAtMost => {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                        v38
                    }
                    US0::US0_BinomialBand => {
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        v41
                    }
                    US0::US0_MedianBand => {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                        v40
                    }
                    US0::US0_OneSampleT => {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                        v37
                    }
                    US0::US0_WelchTwoSided => {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                        v36
                    }
                    US0::US0_WilsonBand => {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                        v39
                    }
                };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v47));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" runs="); } LIT.with(|lit| lit.clone()) }));
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v4));
                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" registered="); } LIT.with(|lit| lit.clone()) }));
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v20));
                stdout_line_20(v52);
                0i32
            }
        }
        US1::US1_PlanRejected(v16) => {
            let v16: Rc<str> = v16;
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) }, v16));
            failed_1(v17)
        }
    }
}
fn identifier_ok_21(mut v0: Rc<str>) -> bool {
    let v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8));
    v1
}
fn test_of_22(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "welch";
    if v1 {
        let v2: US0 = US0::US0_WelchTwoSided;
        US2::US2_TestKnown(v2)
    } else {
        let v4: bool = &*v0 == "one-sample";
        if v4 {
            let v5: US0 = US0::US0_OneSampleT;
            US2::US2_TestKnown(v5)
        } else {
            let v7: bool = &*v0 == "binomial";
            if v7 {
                let v8: US0 = US0::US0_BinomialAtMost;
                US2::US2_TestKnown(v8)
            } else {
                let v10: bool = &*v0 == "wilson";
                if v10 {
                    let v11: US0 = US0::US0_WilsonBand;
                    US2::US2_TestKnown(v11)
                } else {
                    let v13: bool = &*v0 == "median";
                    if v13 {
                        let v14: US0 = US0::US0_MedianBand;
                        US2::US2_TestKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "binomial-band";
                        if v16 {
                            let v17: US0 = US0::US0_BinomialBand;
                            US2::US2_TestKnown(v17)
                        } else {
                            US2::US2_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn registered_5(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_experiment_args::kit_experiment_register_args(4i32);
    let (mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v2.borrow()[0i32 as usize].clone();
    let v14: bool = identifier_ok_21(v1.clone());
    let v15: bool = v14 == false;
    if v15 {
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: the ID must be letters, digits, '-' or '_', received '"); } LIT.with(|lit| lit.clone()) }, v1));
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) }));
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = concat5_2(v18, v19, v20, v21.clone(), v21.clone());
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v22));
        failed_1(v23)
    } else {
        let v25: bool = &*v13 != "";
        if v25 {
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) }, v13));
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = concat5_2(v28, v29, v30, v31.clone(), v31.clone());
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v32));
            failed_1(v33)
        } else {
            let v35: US2 = test_of_22(v3.clone());
            match v35 {
                US2::US2_TestKnown(v39) => {
                    let v39: US0 = v39;
                    registered_as_6(v0, v1, v39, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
                }
                US2::US2_TestUnknown => {
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: --test must be welch|one-sample|binomial|wilson|median|binomial-band, received '"); } LIT.with(|lit| lit.clone()) }, v3));
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
                    failed_1(v37)
                }
            }
        }
    }
}
fn file_text_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn path_exists_25(mut v0: Rc<str>) -> bool {
    let v1: bool = std::path::Path::new(&*v0).exists();
    v1
}
fn shown_23(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v1.clone(), v3.clone(), v2, v4.clone(), v4.clone());
    let v6: Rc<str> = joined_15(v0.clone(), v5);
    let v7: Rc<str> = file_text_24(v6);
    let v8: bool = &*v7 == "";
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: no registration for "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = concat5_2(v9, v1, v10, v0, v4);
        failed_1(v11)
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration\n"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments\n"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_2(v1.clone(), v3.clone(), v15, v4.clone(), v4.clone());
        let v17: Rc<str> = joined_15(v0.clone(), v16);
        let v18: Rc<str> = file_text_24(v17);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures\n"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = concat5_2(v1.clone(), v3.clone(), v20, v4.clone(), v4.clone());
        let v22: Rc<str> = joined_15(v0.clone(), v21);
        let v23: Rc<str> = file_text_24(v22);
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict\n"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = concat5_2(v1.clone(), v3.clone(), v25.clone(), v4.clone(), v4.clone());
        let v27: Rc<str> = joined_15(v0.clone(), v26);
        let v28: Rc<str> = file_text_24(v27);
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda\n"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = concat5_2(v1.clone(), v3.clone(), v30, v4.clone(), v4.clone());
        let v32: Rc<str> = joined_15(v0.clone(), v31);
        let v33: Rc<str> = file_text_24(v32);
        let v34: Rc<str> = concat5_2(v29, v33, v4.clone(), v4.clone(), v4.clone());
        let v35: Rc<str> = concat5_2(v19, v23, v24, v28, v34);
        let v36: Rc<str> = concat5_2(v13, v7, v14, v18, v35);
        stdout_line_20(v36);
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment show ok id="); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = concat5_2(v1.clone(), v3, v25, v4.clone(), v4.clone());
        let v39: Rc<str> = joined_15(v0, v38);
        let v40: bool = path_exists_25(v39);
        let v43: Rc<str> = if v40 {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=recorded"); } LIT.with(|lit| lit.clone()) };
            v41
        } else {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=pending"); } LIT.with(|lit| lit.clone()) };
            v42
        };
        let v44: Rc<str> = concat5_2(v37, v1, v43, v4.clone(), v4);
        stdout_line_20(v44);
        0i32
    }
}
fn verb_ran_4(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match v0 {
        US3::US3_VerbAddendum => {
            let v7: i32 = kit_experiment_amend::kit_experiment_addendum_run();
            v7
        }
        US3::US3_VerbAmend => {
            let v6: i32 = kit_experiment_amend::kit_experiment_amend_run();
            v6
        }
        US3::US3_VerbAnalyze => {
            let v4: i32 = kit_experiment_analysis::kit_experiment_analyze_run();
            v4
        }
        US3::US3_VerbImport => {
            let v8: i32 = kit_experiment_import::kit_experiment_import_run();
            v8
        }
        US3::US3_VerbRecordVerdict => {
            let v9: i32 = kit_experiment_import::kit_experiment_verdict_run();
            v9
        }
        US3::US3_VerbRegister => {
            registered_5(v1, v2)
        }
        US3::US3_VerbShow => {
            shown_23(v1, v2)
        }
    }
}
fn command_line_argument_count_27() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_28(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_26(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_27();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US4 = if v4 {
        let v5: Rc<str> = command_line_argument_28(v0);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    };
    match v8 {
        US4::US4_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US4::US4_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_26(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn experiment_verb_of_29(mut v0: Rc<str>) -> US5 {
    let v1: bool = &*v0 == "register";
    if v1 {
        let v2: US3 = US3::US3_VerbRegister;
        US5::US5_Some(v2)
    } else {
        let v4: bool = &*v0 == "analyze";
        if v4 {
            let v5: US3 = US3::US3_VerbAnalyze;
            US5::US5_Some(v5)
        } else {
            let v7: bool = &*v0 == "show";
            if v7 {
                let v8: US3 = US3::US3_VerbShow;
                US5::US5_Some(v8)
            } else {
                let v10: bool = &*v0 == "amend";
                if v10 {
                    let v11: US3 = US3::US3_VerbAmend;
                    US5::US5_Some(v11)
                } else {
                    let v13: bool = &*v0 == "addendum";
                    if v13 {
                        let v14: US3 = US3::US3_VerbAddendum;
                        US5::US5_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "import";
                        if v16 {
                            let v17: US3 = US3::US3_VerbImport;
                            US5::US5_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "record-verdict";
                            if v19 {
                                let v20: US3 = US3::US3_VerbRecordVerdict;
                                US5::US5_Some(v20)
                            } else {
                                US5::US5_None
                            }
                        }
                    }
                }
            }
        }
    }
}
fn experiment_run_0() -> i32 {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_26(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: US5 = experiment_verb_of_29(v4.clone());
                    match v6 {
                        US5::US5_None => {
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) }, v4));
                            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) }));
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v27: Rc<str> = concat5_2(v23, v24, v25, v26.clone(), v26.clone());
                            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v27));
                            failed_1(v28)
                        }
                        US5::US5_Some(v7) => {
                            let v7: US3 = v7;
                            match Rc::unwrap_or_clone(v5) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let v9: Rc<UH0> = v9;
                                    match Rc::unwrap_or_clone(v9) {
                                        UH0::UH0_Cons(v10, v11) => {
                                            let v10: Rc<str> = v10;
                                            let v11: Rc<UH0> = v11;
                                            verb_ran_4(v7, v8, v10)
                                        }
                                        UH0::UH0_Nil => {
                                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            verb_ran_4(v7, v8, v13)
                                        }
                                    }
                                }
                                UH0::UH0_Nil => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    verb_ran_4(v7, v17.clone(), v17.clone())
                                }
                            }
                        }
                    }
                }
                _ => {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = concat5_2(v32, v33, v34, v35.clone(), v35.clone());
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) }, v36));
                    failed_1(v37)
                }
            }
        }
        _ => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = concat5_2(v40, v41, v42, v43.clone(), v43.clone());
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) }, v44));
            failed_1(v45)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        experiment_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_run() -> i32 {
    closure0()()
}
