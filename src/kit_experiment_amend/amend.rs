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
    US1_AmendAccepted,
    US1_AmendRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_WelchTwoSided,
    US3_OneSampleT,
    US3_BinomialAtMost,
    US3_WilsonBand,
    US3_MedianBand,
    US3_BinomialBand,
}
#[derive(Clone)]
enum US2 {
    US2_TestKnown(US3),
    US2_TestUnknown,
}
#[derive(Clone)]
enum US4 {
    US4_PlanAccepted,
    US4_PlanRejected(Rc<str>),
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
fn joined_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn file_text_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn plan_text_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let v6: Rc<str> = joined_7(v0.clone(), v5.clone());
    let v7: Rc<str> = file_text_8(v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
    let v10: Rc<str> = joined_7(v0.clone(), v9.clone());
    let v11: Rc<str> = file_text_8(v10.clone());
    let v12: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v11, &*v4, &*v4, &*v4].concat());
    v12.clone()
}
fn option_value_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|arg| arg != &*v0).nth(1).unwrap_or_default());
    v1.clone()
}
fn path_exists_11(mut v0: Rc<str>) -> bool {
    let v1: bool = std::path::Path::new(&*v0).exists();
    v1
}
fn looked_at_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v2, &*v4, &*v4].concat());
    let v6: Rc<str> = joined_7(v0.clone(), v5.clone());
    let v7: bool = path_exists_11(v6.clone());
    if v7 {
        true
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v8, &*v4, &*v4].concat());
        let v10: Rc<str> = joined_7(v0.clone(), v9.clone());
        path_exists_11(v10.clone())
    }
}
fn amendable_named_13(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 == "question";
    if v1 {
        true
    } else {
        let v2: bool = &*v0 == "test";
        if v2 {
            true
        } else {
            let v3: bool = &*v0 == "metric";
            if v3 {
                true
            } else {
                let v4: bool = &*v0 == "group";
                if v4 {
                    true
                } else {
                    let v5: bool = &*v0 == "arms";
                    if v5 {
                        true
                    } else {
                        let v6: bool = &*v0 == "runs";
                        if v6 {
                            true
                        } else {
                            let v7: bool = &*v0 == "prediction";
                            if v7 {
                                true
                            } else {
                                let v8: bool = &*v0 == "confidence";
                                if v8 {
                                    true
                                } else {
                                    let v9: bool = &*v0 == "decision";
                                    if v9 {
                                        true
                                    } else {
                                        let v10: bool = &*v0 == "time";
                                        v10
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn amend_checked_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US1 {
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the metric was already read (an interim look or the verdict); an amendment must precede the runs it changes, so record an addendum instead"); } LIT.with(|lit| lit.clone()) };
        US1::US1_AmendRefused(v3.clone())
    } else {
        let v5: bool = amendable_named_13(v0.clone());
        let v6: bool = v5 == false;
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field must be one of "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("question|test|metric|group|arms|runs|prediction|confidence|decision|time"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v8, &*v9, &*v0, &*v10].concat());
            US1::US1_AmendRefused(v11.clone())
        } else {
            let v13: bool = &*v1 == "";
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--reason says why the plan changes"); } LIT.with(|lit| lit.clone()) };
                US1::US1_AmendRefused(v14.clone())
            } else {
                US1::US1_AmendAccepted
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
fn local_stamp_18() -> Rc<str> {
    let v0: Rc<str> = { #[cfg(windows)] fn local() -> String { #[repr(C)] struct Clock { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(clock: *mut Clock); } let mut clock = Clock { year: 0, month: 0, weekday: 0, day: 0, hour: 0, minute: 0, second: 0, millis: 0 }; unsafe { GetLocalTime(&mut clock); } let _ = clock.weekday; let _ = clock.millis; format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", clock.year, clock.month, clock.day, clock.hour, clock.minute, clock.second) } #[cfg(not(windows))] fn local() -> String { std::process::Command::new("date").arg("+%Y-%m-%dT%H:%M:%S").output().ok().map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned()).unwrap_or_default() } std::rc::Rc::<str>::from(local().as_str()) };
    v0.clone()
}
fn field_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2.clone()
}
fn test_of_20(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "welch";
    if v1 {
        let v2: US3 = US3::US3_WelchTwoSided;
        US2::US2_TestKnown(v2)
    } else {
        let v4: bool = &*v0 == "one-sample";
        if v4 {
            let v5: US3 = US3::US3_OneSampleT;
            US2::US2_TestKnown(v5)
        } else {
            let v7: bool = &*v0 == "binomial";
            if v7 {
                let v8: US3 = US3::US3_BinomialAtMost;
                US2::US2_TestKnown(v8)
            } else {
                let v10: bool = &*v0 == "wilson";
                if v10 {
                    let v11: US3 = US3::US3_WilsonBand;
                    US2::US2_TestKnown(v11)
                } else {
                    let v13: bool = &*v0 == "median";
                    if v13 {
                        let v14: US3 = US3::US3_MedianBand;
                        US2::US2_TestKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "binomial-band";
                        if v16 {
                            let v17: US3 = US3::US3_BinomialBand;
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
fn decimal_i32_21(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn arms_needed_23(mut v0: US3) -> i32 {
    match &v0 {
        US3::US3_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn comma_count_24(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_25(mut v0: US3) -> bool {
    match &v0 {
        US3::US3_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_22(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US4 {
    let v7: bool = &*v5 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US4::US4_PlanRejected(v8.clone())
    } else {
        let v10: bool = &*v6 == "";
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US4::US4_PlanRejected(v11.clone())
        } else {
            let v13: bool = v3 < 2i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US4::US4_PlanRejected(v14.clone())
            } else {
                let v16: bool = &*v1 == "";
                let v19: bool = if v16 {
                    let v17: i32 = arms_needed_23(v0.clone());
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
                        US3::US3_BinomialAtMost => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                            v24.clone()
                        }
                        US3::US3_BinomialBand => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                            v27.clone()
                        }
                        US3::US3_MedianBand => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US3::US3_OneSampleT => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US3::US3_WelchTwoSided => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                        US3::US3_WilsonBand => {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                    };
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with --group needs --arms naming the group value (without both it measures the whole table)"); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                    US4::US4_PlanRejected(v36.clone())
                } else {
                    let v38: i32 = comma_count_24(v1.clone());
                    let v39: i32 = arms_needed_23(v0.clone());
                    let v40: bool = v38 != v39;
                    let v45: bool = if v40 {
                        let v41: bool = &*v1 != "";
                        if v41 {
                            true
                        } else {
                            let v42: i32 = arms_needed_23(v0.clone());
                            let v43: bool = v42 == 2i32;
                            v43
                        }
                    } else {
                        false
                    };
                    if v45 {
                        let v57: Rc<str> = match &v0 {
                            US3::US3_BinomialAtMost => {
                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                v48.clone()
                            }
                            US3::US3_BinomialBand => {
                                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                v51.clone()
                            }
                            US3::US3_MedianBand => {
                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                v50.clone()
                            }
                            US3::US3_OneSampleT => {
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                v47.clone()
                            }
                            US3::US3_WelchTwoSided => {
                                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                v46.clone()
                            }
                            US3::US3_WilsonBand => {
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                v49.clone()
                            }
                        };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --arms with "); } LIT.with(|lit| lit.clone()) };
                        let v59: i32 = arms_needed_23(v0.clone());
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
                        US4::US4_PlanRejected(v65.clone())
                    } else {
                        let v67: bool = match &v0 {
                            US3::US3_MedianBand => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v70: bool = if v67 {
                            let v68: i32 = comma_count_24(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_PlanRejected(v71.clone())
                        } else {
                            let v73: bool = needs_prediction_25(v0.clone());
                            let v75: bool = if v73 {
                                let v74: bool = &*v4 == "";
                                v74
                            } else {
                                false
                            };
                            if v75 {
                                let v87: Rc<str> = match &v0 {
                                    US3::US3_BinomialAtMost => {
                                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                                        v78.clone()
                                    }
                                    US3::US3_BinomialBand => {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                                        v81.clone()
                                    }
                                    US3::US3_MedianBand => {
                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                                        v80.clone()
                                    }
                                    US3::US3_OneSampleT => {
                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                                        v77.clone()
                                    }
                                    US3::US3_WelchTwoSided => {
                                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                                        v76.clone()
                                    }
                                    US3::US3_WilsonBand => {
                                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                                        v79.clone()
                                    }
                                };
                                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs --prediction (the model value or rate it is tested against)"); } LIT.with(|lit| lit.clone()) };
                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v90: Rc<str> = std::rc::Rc::<str>::from([&*v87, &*v88, &*v89, &*v89, &*v89].concat());
                                US4::US4_PlanRejected(v90.clone())
                            } else {
                                US4::US4_PlanAccepted
                            }
                        }
                    }
                }
            }
        }
    }
}
fn text_appended_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(&*v0).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }).err().map(|error| format!("append {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2.clone()
}
fn stdout_line_27(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn amended_0() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = argument_3(v0);
    let v2: i32 = 4i32;
    let v3: Rc<str> = argument_3(v2);
    let v4: Rc<str> = plan_text_6(v1.clone(), v3.clone());
    let v5: bool = &*v4 == "";
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: no registration for "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v8)), v1));
        failed_1(v10.clone())
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = option_value_9(v12.clone());
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--reason"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = option_value_9(v14.clone());
        let v16: bool = looked_at_10(v1.clone(), v3.clone());
        let v17: US1 = amend_checked_12(v13.clone(), v15.clone(), v16);
        match &v17 {
            US1::US1_AmendAccepted => {
                let v22: u64 = now_ms_14();
                let v23: Rc<str> = utc_stamp_15(v22);
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amended"); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = field_line_17(v24.clone(), v23.clone());
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = option_value_9(v14.clone());
                let v28: Rc<str> = field_line_17(v26.clone(), v27.clone());
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v28));
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--value"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = option_value_9(v30.clone());
                let v32: Rc<str> = field_line_17(v13.clone(), v31.clone());
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v32));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = local_stamp_18();
                let v36: Rc<str> = field_line_17(v34.clone(), v35.clone());
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v36));
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v37));
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = field_of_19(v38.clone(), v39.clone());
                let v41: US2 = test_of_20(v40.clone());
                match &v41 {
                    US2::US2_TestKnown(v52) => {
                        let v52: US3 = v52.clone();
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
                        let v54: Rc<str> = field_of_19(v38.clone(), v53.clone());
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = field_of_19(v38.clone(), v55.clone());
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = field_of_19(v38.clone(), v57.clone());
                        let v59: i32 = decimal_i32_21(v58.clone());
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = field_of_19(v38.clone(), v60.clone());
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = field_of_19(v38.clone(), v62.clone());
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = field_of_19(v38.clone(), v64.clone());
                        let v66: US4 = plan_checked_22(v52.clone(), v54.clone(), v56.clone(), v59, v61.clone(), v63.clone(), v65.clone());
                        match &v66 {
                            US4::US4_PlanAccepted => {
                                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
                                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v74: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v72, &*v71, &*v73, &*v73].concat());
                                let v75: Rc<str> = joined_7(v1.clone(), v74.clone());
                                let v76: Rc<str> = text_appended_26(v75.clone(), v37.clone());
                                let v77: bool = &*v76 != "";
                                if v77 {
                                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: "); } LIT.with(|lit| lit.clone()) };
                                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v76));
                                    failed_1(v79.clone())
                                } else {
                                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment amend ok id="); } LIT.with(|lit| lit.clone()) };
                                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v3));
                                    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v82, v83)), v13));
                                    let v86: Rc<str> = option_value_9(v30.clone());
                                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" value="); } LIT.with(|lit| lit.clone()) };
                                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v86));
                                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" amended="); } LIT.with(|lit| lit.clone()) };
                                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v85, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v88, v89)), v23))));
                                    stdout_line_27(v92.clone());
                                    0i32
                                }
                            }
                            US4::US4_PlanRejected(v67) => {
                                let v67: Rc<str> = v67.clone();
                                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: the amended plan is incomplete: "); } LIT.with(|lit| lit.clone()) };
                                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v67));
                                failed_1(v69.clone())
                            }
                        }
                    }
                    US2::US2_TestUnknown => {
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: --test must be "); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: --test must be welch|one-sample|binomial|wilson|median|binomial-band"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", received '"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: --test must be welch|one-sample|binomial|wilson|median|binomial-band, received '"); } LIT.with(|lit| lit.clone()) };
                        let v47: Rc<str> = field_of_19(v38.clone(), v39.clone());
                        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
                        failed_1(v50.clone())
                    }
                }
            }
            US1::US1_AmendRefused(v18) => {
                let v18: Rc<str> = v18.clone();
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amend: "); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
                failed_1(v20.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        amended_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn addended_28() -> i32 {
    let v0: i32 = 3i32;
    let v1: Rc<str> = argument_3(v0);
    let v2: i32 = 4i32;
    let v3: Rc<str> = argument_3(v2);
    let v4: i32 = 5i32;
    let v5: Rc<str> = argument_3(v4);
    let v6: Rc<str> = plan_text_6(v1.clone(), v3.clone());
    let v7: bool = &*v6 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: no registration for "); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v3));
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v1));
        failed_1(v12.clone())
    } else {
        let v14: bool = &*v5 == "";
        if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: the text is the post-hoc note, kept apart from the verdict"); } LIT.with(|lit| lit.clone()) };
            failed_1(v15.clone())
        } else {
            let v17: u64 = now_ms_14();
            let v18: Rc<str> = utc_stamp_15(v17);
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addenda"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v20, &*v19, &*v21, &*v21].concat());
            let v23: Rc<str> = joined_7(v1.clone(), v22.clone());
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v24));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = field_line_17(v26.clone(), v5.clone());
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v27));
            let v29: Rc<str> = text_appended_26(v23.clone(), v28.clone());
            let v30: bool = &*v29 != "";
            if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("addendum: "); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v29));
                failed_1(v32.clone())
            } else {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment addendum ok id="); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v3));
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v18));
                let v39: bool = looked_at_10(v1.clone(), v3.clone());
                let v42: Rc<str> = if v39 {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_look=yes"); } LIT.with(|lit| lit.clone()) };
                    v40.clone()
                } else {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_look=no"); } LIT.with(|lit| lit.clone()) };
                    v41.clone()
                };
                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v42));
                stdout_line_27(v43.clone());
                0i32
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        addended_28()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_amend_run() -> i32 {
    closure0()()
}
pub fn kit_experiment_addendum_run() -> i32 {
    closure1()()
}
