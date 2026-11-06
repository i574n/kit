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
enum US2 {
    US2_WelchPlan(Rc<str>),
    US2_OneSamplePlan(f64),
}
#[derive(Clone)]
enum US3 {
    US3_BinomialPlan(f64),
    US3_WilsonPlan(f64),
    US3_BinomialBandPlan(f64),
}
#[derive(Clone)]
enum US4 {
    US4_MedianPlan(f64, f64),
}
#[derive(Clone)]
enum US1 {
    US1_TestedMean(US2),
    US1_TestedRate(US3),
    US1_TestedMedian(US4),
}
#[derive(Clone)]
enum US5 {
    US5_RowsKept(Rc<str>),
    US5_RowsRefused(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_OneArm,
    US6_AgainstArm(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_FullLook,
    US7_InterimLook(Rc<str>),
    US7_LookRefused(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_NoBand,
    US8_Band(f64, f64),
}
#[derive(Clone)]
enum US9 {
    US9_PlanRead(Rc<str>, Rc<str>, Rc<str>, US0, Rc<str>, i32, f64, Rc<str>, Rc<str>, Rc<str>, US1),
    US9_PlanBroken(Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_TestKnown(US0),
    US10_TestUnknown,
}
#[derive(Clone)]
enum US11 {
    US11_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US11_TableUnreadable(Rc<str>),
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
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
fn optional_column_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> i32 {
    let v2: bool = &*v1 == "";
    if v2 {
        -1i32
    } else {
        let v3: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v1)).map(|index| index as i32).unwrap_or(-1);
        v3
    }
}
fn in_arm_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: &Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = (v1).as_bytes().get(v6 as usize) == Some(&49u8);
    if v7 {
        let v8: bool = v2 < 0i32;
        if v8 {
            true
        } else {
            let v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v10: bool = &*v9 == &*v5;
            v10
        }
    } else {
        false
    }
}
fn text_before_9(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v0) < (v1);
    v2
}
fn in_scope_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = in_arm_8(v0.clone(), v1, v2, v3, &v4, v5, v6);
    if v7 {
        let v8: bool = v3 < 0i32;
        if v8 {
            true
        } else {
            let v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v10: bool = text_before_9(v9, v4);
            let v11: bool = v10 == false;
            v11
        }
    } else {
        false
    }
}
fn number_parsed_10(mut v0: Rc<str>) -> f64 {
    let v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn success_cell_11(mut v0: Rc<str>) -> bool {
    let v1: bool = { let cell = (v0).trim(); !cell.is_empty() && !matches!(cell, "0" | "false" | "no" | "-") };
    v1
}
fn tallied_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: f64, mut v11: f64, mut v12: f64, mut v13: i32) -> (i32, f64, f64, f64, i32) {
    let v14: bool = v7 > v8;
    let (mut v49, mut v50, mut v51, mut v52, mut v53): (i32, f64, f64, f64, i32) = if v14 {
        (v9, v10, v11, v12, v13)
    } else {
        let v15: bool = in_scope_7(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
        let (mut v44, mut v45, mut v46, mut v47, mut v48): (i32, f64, f64, f64, i32) = if v15 {
            let v16: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v17: f64 = number_parsed_10(v16.clone());
            let v18: bool = v17 == v17;
            let v19: i32 = v9.wrapping_add(1i32);
            let v20: bool = success_cell_11(v16);
            let v21: i32 = if v20 {
                1i32
            } else {
                0i32
            };
            let v22: i32 = v13.wrapping_add(v21);
            let (mut v27, mut v28, mut v29, mut v30, mut v31): (i32, f64, f64, f64, i32) = if v18 {
                let v23: f64 = v10 + 1.0f64;
                let v24: f64 = v11 + v17;
                let v25: f64 = v17 * v17;
                let v26: f64 = v12 + v25;
                (v19, v23, v24, v26, v22)
            } else {
                (v19, v10, v11, v12, v22)
            };
            let v32: i32 = v7.wrapping_add(1i32);
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, f64, f64, f64, i32) = tallied_6(v0, v1, v2, v3, v4, v5, v6, v32, v8, v27, v28, v29, v30, v31);
            (v33, v34, v35, v36, v37)
        } else {
            let v38: i32 = v7.wrapping_add(1i32);
            let (mut v39, mut v40, mut v41, mut v42, mut v43): (i32, f64, f64, f64, i32) = tallied_6(v0, v1, v2, v3, v4, v5, v6, v38, v8, v9, v10, v11, v12, v13);
            (v39, v40, v41, v42, v43)
        };
        (v44, v45, v46, v47, v48)
    };
    (v49, v50, v51, v52, v53)
}
fn second_arm_of_12(mut v0: US1) -> US6 {
    match v0 {
        US1::US1_TestedMean(v1) => {
            let v1: US2 = v1;
            match v1 {
                US2::US2_WelchPlan(v2) => {
                    let v2: Rc<str> = v2;
                    US6::US6_AgainstArm(v2)
                }
                _ => {
                    US6::US6_OneArm
                }
            }
        }
        _ => {
            US6::US6_OneArm
        }
    }
}
fn early_rows_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32) -> i32 {
    loop {
        let v9: bool = v6 > v7;
        let v11: bool = if v9 {
            true
        } else {
            let v10: bool = v3 < 0i32;
            v10
        };
        if v11 {
            return v8;
        } else {
            let v12: bool = in_arm_8(v0.clone(), v1.clone(), v2, v3, &v4, v5.clone(), v6);
            let v15: bool = if v12 {
                let v13: bool = in_scope_7(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v6);
                let v14: bool = v13 == false;
                v14
            } else {
                false
            };
            if v15 {
                let v16: i32 = v6.wrapping_add(1i32);
                let v17: i32 = v8.wrapping_add(1i32);
                (v6, v8) = (v16, v17);
                continue;
            } else {
                let v19: i32 = v6.wrapping_add(1i32);
                v6 = v19;
                continue;
            }
        }
    }
}
fn i32_text_15(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1
}
fn numeric_note_16(mut v0: US0, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32) -> Rc<str> {
    let mut v6: i32 = v2 as i32;
    let v9: bool = match v0 {
        US0::US0_MedianBand => {
            true
        }
        US0::US0_OneSampleT => {
            true
        }
        US0::US0_WelchTwoSided => {
            true
        }
        _ => {
            false
        }
    };
    let v11: bool = if v9 {
        let v10: bool = v6 < v1;
        v10
    } else {
        false
    };
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = i32_text_15(v6);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" numeric)"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v12, v13, v14, v15.clone(), v15)
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    }
}
fn arm_counts_14(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: &Rc<str>, mut v5: i32, mut v6: f64, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: &Rc<str>, mut v10: US1, mut v11: i32, mut v12: f64, mut v13: f64, mut v14: f64, mut v15: i32, mut v16: i32, mut v17: f64, mut v18: f64, mut v19: f64, mut v20: i32) -> Rc<str> {
    let v21: Rc<str> = i32_text_15(v5);
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = i32_text_15(v11);
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = numeric_note_16(v3.clone(), v11, v12, v13, v14, v15);
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = concat5_2(v21.clone(), v25, v26.clone(), v26.clone(), v26.clone());
    let v28: Rc<str> = concat5_2(v2, v22.clone(), v23, v24.clone(), v27);
    let v29: US6 = second_arm_of_12(v10);
    match v29 {
        US6::US6_AgainstArm(v30) => {
            let v30: Rc<str> = v30;
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = i32_text_15(v16);
            let v33: Rc<str> = numeric_note_16(v3, v16, v17, v18, v19, v20);
            let v34: Rc<str> = concat5_2(v32, v24, v21, v33, v26);
            concat5_2(v28, v31, v30, v22, v34)
        }
        US6::US6_OneArm => {
            v28
        }
    }
}
fn joined_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn file_text_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn field_of_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().rev().find_map(|line| line.split_once(char::from(61u8)).filter(|(name, _)| *name == &*v1).map(|(_, value)| value)).unwrap_or(""));
    v2
}
fn look_of_20(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> US7 {
    let v4: bool = &*v3 != "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the verdict was already recorded at "); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; an experiment is analyzed once (experiment show prints it)"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = concat5_2(v5, v3, v6, v7.clone(), v7);
        US7::US7_LookRefused(v8)
    } else {
        let v10: bool = v1 == false;
        if v10 {
            US7::US7_FullLook
        } else {
            let v12: bool = &*v2 == "";
            if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("interim look refused: "); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows of the planned count; pass --interim '<reason>' to look anyway and record a disclosure"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = concat5_2(v13, v0, v14, v15.clone(), v15);
                US7::US7_LookRefused(v16)
            } else {
                US7::US7_InterimLook(v2)
            }
        }
    }
}
fn stdout_line_21(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn now_ms_22() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn median_terms_between_24(mut v0: Rc<str>, mut v1: f64, mut v2: f64) -> Rc<str> {
    let v3: Rc<str> = kit_experiment_stats::kit_median_terms(&*v0, v1, v2);
    v3
}
fn band_of_25(mut v0: US1) -> US8 {
    match v0 {
        US1::US1_TestedMedian(v1) => {
            let v1: US4 = v1;
            match v1 {
                US4::US4_MedianPlan(v2, v3) => {
                    let v2: f64 = v2;
                    let v3: f64 = v3;
                    US8::US8_Band(v2, v3)
                }
            }
        }
        _ => {
            US8::US8_NoBand
        }
    }
}
fn values_of_26(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> Rc<str> {
    loop {
        let v10: bool = v7 > v8;
        if v10 {
            return v9;
        } else {
            let v11: bool = in_scope_7(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
            if v11 {
                let v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v13: f64 = number_parsed_10(v12.clone());
                let v14: f64 = number_parsed_10(v12.clone());
                let v15: bool = v13 == v14;
                let v21: Rc<str> = if v15 {
                    let v16: bool = &*v9 == "";
                    if v16 {
                        v12
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        concat5_2(v9, v17, v12, v18.clone(), v18.clone())
                    }
                } else {
                    v9
                };
                let v22: i32 = v7.wrapping_add(1i32);
                (v7, v9) = (v22, v21);
                continue;
            } else {
                let v24: i32 = v7.wrapping_add(1i32);
                v7 = v24;
                continue;
            }
        }
    }
}
fn prediction_of_27(mut v0: US1) -> f64 {
    match v0 {
        US1::US1_TestedMean(v1) => {
            let v1: US2 = v1;
            match v1 {
                US2::US2_OneSamplePlan(v2) => {
                    let v2: f64 = v2;
                    v2
                }
                _ => {
                    0.0f64
                }
            }
        }
        US1::US1_TestedRate(v4) => {
            let v4: US3 = v4;
            match v4 {
                US3::US3_BinomialBandPlan(v7) => {
                    let mut v7: f64 = v7;
                    v7
                }
                US3::US3_BinomialPlan(v5) => {
                    let v5: f64 = v5;
                    v5
                }
                US3::US3_WilsonPlan(v6) => {
                    let mut v6: f64 = v6;
                    v6
                }
            }
        }
        _ => {
            0.0f64
        }
    }
}
fn terms_of_23(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: Rc<str>, mut v9: US0, mut v10: &Rc<str>, mut v11: i32, mut v12: f64, mut v13: &Rc<str>, mut v14: &Rc<str>, mut v15: &Rc<str>, mut v16: US1, mut v17: i32, mut v18: f64, mut v19: f64, mut v20: f64, mut v21: i32, mut v22: i32, mut v23: f64, mut v24: f64, mut v25: f64, mut v26: i32) -> Rc<str> {
    let v27: US8 = band_of_25(v16.clone());
    match v27 {
        US8::US8_Band(v28, v29) => {
            let v28: f64 = v28;
            let v29: f64 = v29;
            let v30: i32 = 1i32;
            let v31: i32 = ((v0).len().max(1) - 1) as i32;
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = values_of_26(v0, v1, v2, v3, v4, v8, v5, v30, v31, v32);
            median_terms_between_24(v33, v28, v29)
        }
        US8::US8_NoBand => {
            let v46: Rc<str> = match v9 {
                US0::US0_BinomialAtMost => {
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                    v37
                }
                US0::US0_BinomialBand => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                    v40
                }
                US0::US0_MedianBand => {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    v39
                }
                US0::US0_OneSampleT => {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                    v36
                }
                US0::US0_WelchTwoSided => {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                    v35
                }
                US0::US0_WilsonBand => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                    v38
                }
            };
            let v47: f64 = prediction_of_27(v16);
            let v48: Rc<str> = kit_experiment_stats::kit_experiment_terms(&*v46, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v47, v12);
            v48
        }
    }
}
fn field_line_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("{}={}
", &*v0, (v1).chars().map(|c| if c.is_control() { char::from(32u8) } else { c }).collect::<String>()).as_str());
    v2
}
fn text_appended_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(&*v0).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }).err().map(|error| format!("append {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2
}
fn text_created_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let path = std::path::Path::new(&*v0); let created = path.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::OpenOptions::new().write(true).create_new(true).open(path)).and_then(|mut file| { use std::io::Write; file.write_all((v1).as_bytes()) }); created.err().map(|error| format!("write {}: {error}", path.display())).unwrap_or_default() }.as_str());
    v2
}
fn analyzed_plan_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: US0, mut v11: Rc<str>, mut v12: i32, mut v13: f64, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: US1, mut v18: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> i32 {
    let (mut v19, mut v20): (bool, Rc<str>) = kit_table_query::kit_table_rows_kept(&*v3, &*v5);
    let v23: US5 = if v19 {
        US5::US5_RowsKept(v20)
    } else {
        US5::US5_RowsRefused(v20)
    };
    let (mut v30, mut v31): (Rc<str>, Rc<str>) = match v23 {
        US5::US5_RowsKept(v24) => {
            let v24: Rc<str> = v24;
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v24, v25)
        }
        US5::US5_RowsRefused(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v27, v26)
        }
    };
    let v32: i32 = optional_column_5(v18.clone(), v11.clone());
    let v33: i32 = optional_column_5(v18.clone(), v15.clone());
    let v34: i32 = (v18).first().and_then(|header| header.iter().position(|item| &**item == &*v8)).map(|index| index as i32).unwrap_or(-1);
    let v35: bool = v34 < 0i32;
    let v37: bool = if v35 {
        true
    } else {
        let v36: bool = &*v31 != "";
        v36
    };
    let v41: bool = if v37 {
        true
    } else {
        let v38: bool = &*v11 != "";
        if v38 {
            let v39: bool = v32 < 0i32;
            v39
        } else {
            false
        }
    };
    let v45: bool = if v41 {
        true
    } else {
        let v42: bool = &*v15 != "";
        if v42 {
            let v43: bool = v33 < 0i32;
            v43
        } else {
            false
        }
    };
    if v45 {
        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: unknown columns in "); } LIT.with(|lit| lit.clone()) }, v3));
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) }));
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = std::rc::Rc::<str>::from((v18).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v48)).unwrap_or_default().as_str());
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v49));
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
        failed_1(v51)
    } else {
        let v53: i32 = 1i32;
        let v54: i32 = ((v18).len().max(1) - 1) as i32;
        let v55: i32 = 0i32;
        let v56: f64 = 0.0f64;
        let v57: f64 = 0.0f64;
        let v58: f64 = 0.0f64;
        let v59: i32 = 0i32;
        let (mut v60, mut v61, mut v62, mut v63, mut v64): (i32, f64, f64, f64, i32) = tallied_6(v18.clone(), v30.clone(), v32, v33, v16.clone(), v9.clone(), v34, v53, v54, v55, v56, v57, v58, v59);
        let v65: US6 = second_arm_of_12(v17.clone());
        let (mut v89, mut v90, mut v91, mut v92, mut v93, mut v94): (i32, f64, f64, f64, i32, i32) = match v65 {
            US6::US6_AgainstArm(v66) => {
                let v66: Rc<str> = v66;
                let v67: i32 = 1i32;
                let v68: i32 = ((v18).len().max(1) - 1) as i32;
                let v69: i32 = 0i32;
                let v70: f64 = 0.0f64;
                let v71: f64 = 0.0f64;
                let v72: f64 = 0.0f64;
                let v73: i32 = 0i32;
                let (mut v74, mut v75, mut v76, mut v77, mut v78): (i32, f64, f64, f64, i32) = tallied_6(v18.clone(), v30.clone(), v32, v33, v16.clone(), v66.clone(), v34, v67, v68, v69, v70, v71, v72, v73);
                let v79: i32 = 1i32;
                let v80: i32 = ((v18).len().max(1) - 1) as i32;
                let v81: i32 = 0i32;
                let v82: i32 = early_rows_13(v18.clone(), v30.clone(), v32, v33, v16.clone(), v66, v79, v80, v81);
                (v74, v75, v76, v77, v78, v82)
            }
            US6::US6_OneArm => {
                (v60, v61, v62, v63, v64, 0i32)
            }
        };
        let v95: i32 = 1i32;
        let v96: i32 = ((v18).len().max(1) - 1) as i32;
        let v97: i32 = 0i32;
        let v98: i32 = early_rows_13(v18.clone(), v30.clone(), v32, v33, v16.clone(), v9.clone(), v95, v96, v97);
        let v99: i32 = v98.wrapping_add(v94);
        let v100: Rc<str> = arm_counts_14(&v7, &v8, v9.clone(), v10.clone(), &v11, v12, v13, &v14, &v15, &v16, v17.clone(), v60, v61, v62, v63, v64, v89, v90, v91, v92, v93);
        let v101: bool = v60 < v12;
        let v103: bool = if v101 {
            true
        } else {
            let v102: bool = v89 < v12;
            v102
        };
        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict"); } LIT.with(|lit| lit.clone()) };
        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v107: Rc<str> = concat5_2(v7.clone(), v105.clone(), v104.clone(), v106.clone(), v106.clone());
        let v108: Rc<str> = joined_17(v0.clone(), v107);
        let v109: Rc<str> = file_text_18(v108);
        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyzed"); } LIT.with(|lit| lit.clone()) };
        let v111: Rc<str> = field_of_19(v109, v110.clone());
        let v112: US7 = look_of_20(v100.clone(), v103, v4, v111);
        match &v112 {
            US7::US7_LookRefused(v113) => {
                let v113: Rc<str> = v113.clone();
                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment analyze refused id="); } LIT.with(|lit| lit.clone()) };
                let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v116: Rc<str> = concat5_2(v114, v7, v115, v113, v106);
                stdout_line_21(v116);
                1i32
            }
            _ => {
                let v117: u64 = now_ms_22();
                let v118: Rc<str> = agile_task_notes::kit_utc_stamp(v117);
                let v119: bool = v33 < 0i32;
                let v123: Rc<str> = if v119 {
                    v106.clone()
                } else {
                    let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" before_registration="); } LIT.with(|lit| lit.clone()) };
                    let v121: Rc<str> = i32_text_15(v99);
                    concat5_2(v120, v121, v106.clone(), v106.clone(), v106.clone())
                };
                let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id="); } LIT.with(|lit| lit.clone()) };
                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" test="); } LIT.with(|lit| lit.clone()) };
                let v137: Rc<str> = match &v10 {
                    US0::US0_BinomialAtMost => {
                        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                        v128
                    }
                    US0::US0_BinomialBand => {
                        let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                        v131
                    }
                    US0::US0_MedianBand => {
                        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                        v130
                    }
                    US0::US0_OneSampleT => {
                        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                        v127
                    }
                    US0::US0_WelchTwoSided => {
                        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                        v126
                    }
                    US0::US0_WilsonBand => {
                        let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                        v129
                    }
                };
                let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" n="); } LIT.with(|lit| lit.clone()) };
                let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v140: Rc<str> = terms_of_23(v18, v30, v32, v33, v16.clone(), v34, &v7, &v8, v9, v10, &v11, v12, v13, &v14, &v15, &v16, v17, v60, v61, v62, v63, v64, v89, v90, v91, v92, v93);
                let v141: Rc<str> = concat5_2(v138, v100, v139, v140, v123);
                let v142: Rc<str> = concat5_2(v124, v7.clone(), v125, v137, v141);
                let v164: Rc<str> = match &v112 {
                    US7::US7_InterimLook(v143) => {
                        let v143: Rc<str> = v143.clone();
                        let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosures"); } LIT.with(|lit| lit.clone()) };
                        let v145: Rc<str> = concat5_2(v7, v105, v144, v106.clone(), v106.clone());
                        let v146: Rc<str> = joined_17(v0, v145);
                        let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disclosed"); } LIT.with(|lit| lit.clone()) };
                        let v148: Rc<str> = field_line_28(v147, v118);
                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reason"); } LIT.with(|lit| lit.clone()) };
                        let v150: Rc<str> = field_line_28(v149, v143);
                        let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result"); } LIT.with(|lit| lit.clone()) };
                        let v152: Rc<str> = field_line_28(v151, v142.clone());
                        let v153: Rc<str> = concat5_2(v148, v150, v152, v106.clone(), v106.clone());
                        text_appended_29(v146, v153)
                    }
                    _ => {
                        let v155: Rc<str> = concat5_2(v7, v105, v104, v106.clone(), v106.clone());
                        let v156: Rc<str> = joined_17(v0, v155);
                        let v157: Rc<str> = field_line_28(v110, v118);
                        let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("result"); } LIT.with(|lit| lit.clone()) };
                        let v159: Rc<str> = field_line_28(v158, v142.clone());
                        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
                        let v161: Rc<str> = field_line_28(v160, v14);
                        let v162: Rc<str> = concat5_2(v157, v159, v161, v106.clone(), v106.clone());
                        text_created_30(v156, v162)
                    }
                };
                let v165: bool = &*v164 != "";
                if v165 {
                    let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: "); } LIT.with(|lit| lit.clone()) };
                    let v167: Rc<str> = concat5_2(v166, v164, v106.clone(), v106.clone(), v106.clone());
                    failed_1(v167)
                } else {
                    let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit experiment analyze ok "); } LIT.with(|lit| lit.clone()) };
                    let v173: Rc<str> = match v112 {
                        US7::US7_InterimLook(v170) => {
                            let v170: Rc<str> = v170;
                            let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" look=interim disclosed=yes"); } LIT.with(|lit| lit.clone()) };
                            v171
                        }
                        _ => {
                            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" look=full verdict=recorded"); } LIT.with(|lit| lit.clone()) };
                            v172
                        }
                    };
                    let v174: Rc<str> = concat5_2(v169, v142, v173, v106.clone(), v106.clone());
                    stdout_line_21(v174);
                    0i32
                }
            }
        }
    }
}
fn plan_text_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registration"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v1.clone(), v3.clone(), v2, v4.clone(), v4.clone());
    let mut v6: Rc<str> = joined_17(v0.clone(), v5);
    let mut v7: Rc<str> = file_text_18(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("amendments"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = concat5_2(v1, v3, v8, v4.clone(), v4.clone());
    let v10: Rc<str> = joined_17(v0, v9);
    let v11: Rc<str> = file_text_18(v10);
    concat5_2(v7, v11, v4.clone(), v4.clone(), v4)
}
fn test_of_33(mut v0: Rc<str>) -> US10 {
    let v1: bool = &*v0 == "welch";
    if v1 {
        let v2: US0 = US0::US0_WelchTwoSided;
        US10::US10_TestKnown(v2)
    } else {
        let v4: bool = &*v0 == "one-sample";
        if v4 {
            let v5: US0 = US0::US0_OneSampleT;
            US10::US10_TestKnown(v5)
        } else {
            let mut v7: bool = &*v0 == "binomial";
            if v7 {
                let v8: US0 = US0::US0_BinomialAtMost;
                US10::US10_TestKnown(v8)
            } else {
                let v10: bool = &*v0 == "wilson";
                if v10 {
                    let v11: US0 = US0::US0_WilsonBand;
                    US10::US10_TestKnown(v11)
                } else {
                    let v13: bool = &*v0 == "median";
                    if v13 {
                        let v14: US0 = US0::US0_MedianBand;
                        US10::US10_TestKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "binomial-band";
                        if v16 {
                            let v17: US0 = US0::US0_BinomialBand;
                            US10::US10_TestKnown(v17)
                        } else {
                            US10::US10_TestUnknown
                        }
                    }
                }
            }
        }
    }
}
fn whole_of_34(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).trim().parse::<i32>().unwrap_or(-1);
    v1
}
fn number_of_35(mut v0: Rc<str>) -> f64 {
    let v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn first_item_36(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(44u8)).next().unwrap_or("").trim());
    v1
}
fn second_item_38(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(44u8)).nth(1).unwrap_or("").trim());
    v1
}
fn test_plan_of_37(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    match v0 {
        US0::US0_BinomialAtMost => {
            let v9: f64 = number_of_35(v2);
            let v10: US3 = US3::US3_BinomialPlan(v9);
            US1::US1_TestedRate(v10)
        }
        US0::US0_BinomialBand => {
            let v15: f64 = number_of_35(v2);
            let v16: US3 = US3::US3_BinomialBandPlan(v15);
            US1::US1_TestedRate(v16)
        }
        US0::US0_MedianBand => {
            let v18: Rc<str> = first_item_36(v2.clone());
            let v19: f64 = number_of_35(v18);
            let v20: Rc<str> = second_item_38(v2);
            let v21: f64 = number_of_35(v20);
            let v22: US4 = US4::US4_MedianPlan(v19, v21);
            US1::US1_TestedMedian(v22)
        }
        US0::US0_OneSampleT => {
            let mut v6: f64 = number_of_35(v2);
            let mut v7: US2 = US2::US2_OneSamplePlan(v6);
            US1::US1_TestedMean(v7)
        }
        US0::US0_WelchTwoSided => {
            let v3: Rc<str> = second_item_38(v1);
            let v4: US2 = US2::US2_WelchPlan(v3);
            US1::US1_TestedMean(v4)
        }
        US0::US0_WilsonBand => {
            let v12: f64 = number_of_35(v2);
            let v13: US3 = US3::US3_WilsonPlan(v12);
            US1::US1_TestedRate(v13)
        }
    }
}
fn plan_trusted_32(mut v0: Rc<str>) -> US9 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = field_of_19(v0.clone(), v1);
    let v3: US10 = test_of_33(v2);
    match v3 {
        US10::US10_TestKnown(v6) => {
            let mut v6: US0 = v6;
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("runs"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = field_of_19(v0.clone(), v7);
            let v9: i32 = whole_of_34(v8);
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("confidence"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = field_of_19(v0.clone(), v10);
            let v12: f64 = number_of_35(v11);
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = field_of_19(v0.clone(), v13);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("metric"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = field_of_19(v0.clone(), v15);
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arms"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = field_of_19(v0.clone(), v17.clone());
            let v19: bool = &*v18 == "";
            let v23: Rc<str> = if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                v20
            } else {
                let v21: Rc<str> = field_of_19(v0.clone(), v17.clone());
                first_item_36(v21)
            };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = field_of_19(v0.clone(), v24);
            let v26: bool = v12 == v12;
            let v27: f64 = if v26 {
                v12
            } else {
                0.95f64
            };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("decision"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = field_of_19(v0.clone(), v28);
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = field_of_19(v0.clone(), v30);
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("registered_local"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = field_of_19(v0.clone(), v32);
            let v34: Rc<str> = field_of_19(v0.clone(), v17);
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prediction"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = field_of_19(v0, v35);
            let v37: US1 = test_plan_of_37(v6.clone(), v34, v36);
            US9::US9_PlanRead(v14, v16, v23, v6, v25, v9, v27, v29, v31, v33, v37)
        }
        US10::US10_TestUnknown => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the registration names an unknown test"); } LIT.with(|lit| lit.clone()) };
            US9::US9_PlanBroken(v4)
        }
    }
}
fn analyzed_0() -> i32 {
    let v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_experiment_verb_args::kit_experiment_analyze_args(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    let mut v7: Rc<str> = plan_text_31(v1.clone(), v2.clone());
    let v8: bool = &*v6 != "";
    if v8 {
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: "); } LIT.with(|lit| lit.clone()) }, v6));
        failed_1(v9)
    } else {
        let v11: bool = &*v7 == "";
        if v11 {
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: no registration for "); } LIT.with(|lit| lit.clone()) }, v2));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            failed_1(v14)
        } else {
            let v16: US9 = plan_trusted_32(v7);
            match v16 {
                US9::US9_PlanBroken(v17) => {
                    let v17: Rc<str> = v17;
                    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: the registration of "); } LIT.with(|lit| lit.clone()) }, v2));
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is unusable: "); } LIT.with(|lit| lit.clone()) }));
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v17));
                    failed_1(v20)
                }
                US9::US9_PlanRead(v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32) => {
                    let v22: Rc<str> = v22;
                    let v23: Rc<str> = v23;
                    let v24: Rc<str> = v24;
                    let v25: US0 = v25;
                    let v26: Rc<str> = v26;
                    let v27: i32 = v27;
                    let v28: f64 = v28;
                    let v29: Rc<str> = v29;
                    let v30: Rc<str> = v30;
                    let v31: Rc<str> = v31;
                    let v32: US1 = v32;
                    let v33: Rc<str> = kit_table_text::kit_table_problem(&*v3);
                    let v34: bool = &*v33 != "";
                    let v38: US11 = if v34 {
                        US11::US11_TableUnreadable(v33)
                    } else {
                        let v36: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v3).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
                        US11::US11_TableLoaded(v36)
                    };
                    match v38 {
                        US11::US11_TableLoaded(v42) => {
                            let v42: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v42;
                            analyzed_plan_4(v1.clone(), v1.clone(), v2, v3, v4, v5, v6, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v42)
                        }
                        US11::US11_TableUnreadable(v39) => {
                            let v39: Rc<str> = v39;
                            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("analyze: "); } LIT.with(|lit| lit.clone()) }, v39));
                            failed_1(v40)
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        analyzed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_analyze_run() -> i32 {
    closure0()()
}
