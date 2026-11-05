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
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
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
fn identifier_ok_6(mut v0: Rc<str>) -> bool {
    let v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(45u8) || c == char::from(95u8));
    v1
}
fn option_value_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn test_of_8(mut v0: Rc<str>) -> US0 {
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
fn decimal_i32_9(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_11(mut v0: US1) -> i32 {
    match &v0 {
        US1::US1_WelchTwoSided => {
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
fn needs_prediction_13(mut v0: US1) -> bool {
    match &v0 {
        US1::US1_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_10(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US2 {
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
                        let v59: i32 = arms_needed_11(v0.clone());
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
                            let v68: i32 = comma_count_12(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US2::US2_PlanRejected(v71)
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
fn field_line_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2.clone()
}
fn joined_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn local_stamp_19() -> Rc<str> {
    let v0: Rc<str> = { #[cfg(windows)] fn local() -> String { #[repr(C)] struct Clock { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(clock: *mut Clock); } let mut clock = Clock { year: 0, month: 0, weekday: 0, day: 0, hour: 0, minute: 0, second: 0, millis: 0 }; unsafe { GetLocalTime(&mut clock); } let _ = clock.weekday; let _ = clock.millis; format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", clock.year, clock.month, clock.day, clock.hour, clock.minute, clock.second) } #[cfg(not(windows))] fn local() -> String { std::process::Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output().ok().map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned()).unwrap_or_default() } std::rc::Rc::<str>::from(local().as_str()) };
    v0.clone()
}
fn text_created_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2.clone()
}
fn stdout_line_21(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn registered_5(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: bool = identifier_ok_6(v1.clone());
    let v3: bool = v2 == false;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: the ID must be letters, digits, '-' or '_', received '"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = concat5_2(v6.clone(), v7.clone(), v8.clone(), v9.clone(), v9.clone());
        let v11: Rc<str> = concat5_2(v4.clone(), v1.clone(), v5.clone(), v10.clone(), v9.clone());
        failed_1(v11.clone())
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = option_value_7(v13.clone());
        let v15: US0 = test_of_8(v14.clone());
        match &v15 {
            US0::US0_TestKnown(v23) => {
                let v23: US1 = v23.clone();
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs"); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = option_value_7(v24.clone());
                let v26: i32 = decimal_i32_9(v25.clone());
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--arms"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = option_value_7(v27.clone());
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--group"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = option_value_7(v29.clone());
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--prediction"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = option_value_7(v31.clone());
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = option_value_7(v33.clone());
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = option_value_7(v35.clone());
                let v37: US2 = plan_checked_10(v23.clone(), v28.clone(), v30.clone(), v26, v32.clone(), v34.clone(), v36.clone());
                match &v37 {
                    US2::US2_PlanAccepted => {
                        let v43: u64 = now_ms_14();
                        let v44: Rc<str> = utc_stamp_15(v43);
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--confidence"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = option_value_7(v45.clone());
                        let v47: bool = &*v46 == "";
                        let v50: Rc<str> = if v47 {
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0.95"); } LIT.with(|lit| lit.clone()) };
                            v48.clone()
                        } else {
                            option_value_7(v45.clone())
                        };
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
                        let v52: Rc<str> = field_line_17(v51.clone(), v1.clone());
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered"); } LIT.with(|lit| lit.clone()) };
                        let v54: Rc<str> = field_line_17(v53.clone(), v44.clone());
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--question"); } LIT.with(|lit| lit.clone()) };
                        let v57: Rc<str> = option_value_7(v56.clone());
                        let v58: Rc<str> = field_line_17(v55.clone(), v57.clone());
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                        let v71: Rc<str> = match &v23 {
                            US1::US1_BinomialAtMost => {
                                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v62.clone()
                            }
                            US1::US1_BinomialBand => {
                                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v65.clone()
                            }
                            US1::US1_MedianBand => {
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v64.clone()
                            }
                            US1::US1_OneSampleT => {
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v61.clone()
                            }
                            US1::US1_WelchTwoSided => {
                                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v60.clone()
                            }
                            US1::US1_WilsonBand => {
                                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v63.clone()
                            }
                        };
                        let v72: Rc<str> = field_line_17(v59.clone(), v71.clone());
                        let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                        let v74: Rc<str> = option_value_7(v33.clone());
                        let v75: Rc<str> = field_line_17(v73.clone(), v74.clone());
                        let v76: Rc<str> = concat5_2(v52.clone(), v54.clone(), v58.clone(), v72.clone(), v75.clone());
                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                        let v78: Rc<str> = option_value_7(v29.clone());
                        let v79: Rc<str> = field_line_17(v77.clone(), v78.clone());
                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                        let v81: Rc<str> = option_value_7(v27.clone());
                        let v82: Rc<str> = field_line_17(v80.clone(), v81.clone());
                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                        let v84: Rc<str> = option_value_7(v24.clone());
                        let v85: Rc<str> = field_line_17(v83.clone(), v84.clone());
                        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                        let v87: Rc<str> = option_value_7(v31.clone());
                        let v88: Rc<str> = field_line_17(v86.clone(), v87.clone());
                        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
                        let v90: Rc<str> = field_line_17(v89.clone(), v50.clone());
                        let v91: Rc<str> = concat5_2(v79.clone(), v82.clone(), v85.clone(), v88.clone(), v90.clone());
                        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
                        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v95: Rc<str> = concat5_2(v1.clone(), v93.clone(), v92.clone(), v94.clone(), v94.clone());
                        let v96: Rc<str> = joined_18(v0.clone(), v95.clone());
                        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let v98: Rc<str> = option_value_7(v35.clone());
                        let v99: Rc<str> = field_line_17(v97.clone(), v98.clone());
                        let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
                        let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
                        let v102: Rc<str> = option_value_7(v101.clone());
                        let v103: Rc<str> = field_line_17(v100.clone(), v102.clone());
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
                        let v105: Rc<str> = local_stamp_19();
                        let v106: Rc<str> = field_line_17(v104.clone(), v105.clone());
                        let v107: Rc<str> = concat5_2(v76.clone(), v91.clone(), v99.clone(), v103.clone(), v106.clone());
                        let v108: Rc<str> = text_created_20(v96.clone(), v107.clone());
                        let v109: bool = &*v108 != "";
                        if v109 {
                            let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) };
                            let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already registered or the ledger is unwritable ("); } LIT.with(|lit| lit.clone()) };
                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                            let v113: Rc<str> = concat5_2(v110.clone(), v1.clone(), v111.clone(), v108.clone(), v112.clone());
                            failed_1(v113.clone())
                        } else {
                            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment register ok id="); } LIT.with(|lit| lit.clone()) };
                            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" test="); } LIT.with(|lit| lit.clone()) };
                            let v128: Rc<str> = match &v23 {
                                US1::US1_BinomialAtMost => {
                                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                    v119.clone()
                                }
                                US1::US1_BinomialBand => {
                                    let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                    v122.clone()
                                }
                                US1::US1_MedianBand => {
                                    let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                    v121.clone()
                                }
                                US1::US1_OneSampleT => {
                                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                    v118.clone()
                                }
                                US1::US1_WelchTwoSided => {
                                    let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                    v117.clone()
                                }
                                US1::US1_WilsonBand => {
                                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                    v120.clone()
                                }
                            };
                            let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" runs="); } LIT.with(|lit| lit.clone()) };
                            let v130: Rc<str> = option_value_7(v24.clone());
                            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" registered="); } LIT.with(|lit| lit.clone()) };
                            let v132: Rc<str> = concat5_2(v129.clone(), v130.clone(), v131.clone(), v44.clone(), v94.clone());
                            let v133: Rc<str> = concat5_2(v115.clone(), v1.clone(), v116.clone(), v128.clone(), v132.clone());
                            stdout_line_21(v133.clone());
                            0i32
                        }
                    }
                    US2::US2_PlanRejected(v38) => {
                        let v38: Rc<str> = v38.clone();
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: "); } LIT.with(|lit| lit.clone()) };
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = concat5_2(v39.clone(), v38.clone(), v40.clone(), v40.clone(), v40.clone());
                        failed_1(v41.clone())
                    }
                }
            }
            US0::US0_TestUnknown => {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("register: --test must be "); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = option_value_7(v13.clone());
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = concat5_2(v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone());
                failed_1(v21.clone())
            }
        }
    }
}
fn file_text_23(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn path_exists_24(mut v0: Rc<str>) -> bool {
    let v1: bool = std::path::Path::new(&*v0).exists();
    v1
}
fn shown_22(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v1.clone(), v3.clone(), v2, v4.clone(), v4.clone());
    let v6: Rc<str> = joined_18(v0.clone(), v5);
    let v7: Rc<str> = file_text_23(v6);
    let v8: bool = &*v7 == "";
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: no registration for "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = concat5_2(v9, v1.clone(), v10, v0.clone(), v4.clone());
        failed_1(v11)
    } else {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration\n"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments\n"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_2(v1.clone(), v3.clone(), v15, v4.clone(), v4.clone());
        let v17: Rc<str> = joined_18(v0.clone(), v16);
        let v18: Rc<str> = file_text_23(v17);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures\n"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = concat5_2(v1.clone(), v3.clone(), v20, v4.clone(), v4.clone());
        let v22: Rc<str> = joined_18(v0.clone(), v21);
        let v23: Rc<str> = file_text_23(v22);
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict\n"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = concat5_2(v1.clone(), v3.clone(), v25.clone(), v4.clone(), v4.clone());
        let v27: Rc<str> = joined_18(v0.clone(), v26);
        let v28: Rc<str> = file_text_23(v27);
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda\n"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = concat5_2(v1.clone(), v3.clone(), v30, v4.clone(), v4.clone());
        let v32: Rc<str> = joined_18(v0.clone(), v31);
        let v33: Rc<str> = file_text_23(v32);
        let v34: Rc<str> = concat5_2(v29, v33, v4.clone(), v4.clone(), v4.clone());
        let v35: Rc<str> = concat5_2(v19, v23, v24, v28, v34);
        let v36: Rc<str> = concat5_2(v13, v7, v14, v18, v35);
        stdout_line_21(v36);
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment show ok id="); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = concat5_2(v1.clone(), v3, v25, v4.clone(), v4.clone());
        let v39: Rc<str> = joined_18(v0, v38);
        let v40: bool = path_exists_24(v39);
        let v43: Rc<str> = if v40 {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=recorded"); } LIT.with(|lit| lit.clone()) };
            v41
        } else {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verdict=pending"); } LIT.with(|lit| lit.clone()) };
            v42
        };
        let v44: Rc<str> = concat5_2(v37, v1, v43, v4.clone(), v4);
        stdout_line_21(v44);
        0i32
    }
}
fn verb_ran_4(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    match &v0 {
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
            registered_5(v1.clone(), v2.clone())
        }
        US3::US3_VerbShow => {
            shown_22(v1, v2)
        }
    }
}
fn command_line_argument_count_26() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_27(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_25(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_26();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US4 = if v4 {
        let v5: Rc<str> = command_line_argument_27(v0);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    };
    match &v8 {
        US4::US4_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US4::US4_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_25(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn experiment_verb_of_28(mut v0: Rc<str>) -> US5 {
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
    let v1: Rc<UH0> = arguments_from_25(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    let v6: US5 = experiment_verb_of_28(v4.clone());
                    match &v6 {
                        US5::US5_None => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb '"); } LIT.with(|lit| lit.clone()) };
                            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v4));
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; "); } LIT.with(|lit| lit.clone()) };
                            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v29: Rc<str> = concat5_2(v25.clone(), v26.clone(), v27.clone(), v28.clone(), v28.clone());
                            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v29));
                            failed_1(v30.clone())
                        }
                        US5::US5_Some(v7) => {
                            let v7: US3 = v7.clone();
                            match &*v5 {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8.clone();
                                    let v9: Rc<UH0> = v9.clone();
                                    match &*v9 {
                                        UH0::UH0_Cons(v10, v11) => {
                                            let v10: Rc<str> = v10.clone();
                                            let v11: Rc<UH0> = v11.clone();
                                            verb_ran_4(v7.clone(), v8.clone(), v10.clone())
                                        }
                                        UH0::UH0_Nil => {
                                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            verb_ran_4(v7.clone(), v8.clone(), v13.clone())
                                        }
                                    }
                                }
                                UH0::UH0_Nil => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    verb_ran_4(v7.clone(), v17.clone(), v17.clone())
                                }
                            }
                        }
                    }
                }
                _ => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v38: Rc<str> = concat5_2(v34.clone(), v35.clone(), v36.clone(), v37.clone(), v37.clone());
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) };
                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
                    failed_1(v40.clone())
                }
            }
        }
        _ => {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: experiment register <ledger> <ID> --test "); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --metric <col> --runs <n per arm> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | experiment analyze <ledger> <ID> <table.tsv> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--interim <reason>] | experiment amend <ledger> <ID> --field <name> --value <v> --reason <text> | experiment addendum <ledger> <ID> <text> | experiment show <ledger> <ID> | experiment import <ledger> <ID> --registered-at <iso> --evidence <commit|file@line> <register flags> | experiment record-verdict <ledger> <ID> --verdict <text> --at <iso> [--evidence <ref>]"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = concat5_2(v43.clone(), v44.clone(), v45.clone(), v46.clone(), v46.clone());
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown verb ''; "); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v47));
            failed_1(v49.clone())
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
