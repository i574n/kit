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
    US1_RowsKept(Rc<str>),
    US1_RowsRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_TestKnown(US0),
    US2_TestUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_PlanAccepted,
    US3_PlanRejected(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US4_TableUnreadable(Rc<str>),
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table test "); } LIT.with(|lit| lit.clone()) }, v0));
    stderr_line_2(v1);
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
fn arms_needed_6(mut v0: US0) -> i32 {
    match v0 {
        US0::US0_WelchTwoSided => {
            2i32
        }
        _ => {
            1i32
        }
    }
}
fn in_arm_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: &Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
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
fn text_before_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v0) < (v1);
    v2
}
fn in_scope_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32) -> bool {
    let mut v7: bool = in_arm_9(v0.clone(), v1, v2, v3, &v4, v5, v6);
    if v7 {
        let v8: bool = v3 < 0i32;
        if v8 {
            true
        } else {
            let v9: Rc<str> = std::rc::Rc::<str>::from((v0).get(v6 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v10: bool = text_before_10(v9, v4);
            let v11: bool = v10 == false;
            v11
        }
    } else {
        false
    }
}
fn number_parsed_11(mut v0: Rc<str>) -> f64 {
    let v1: f64 = (v0).trim().parse::<f64>().unwrap_or(f64::NAN);
    v1
}
fn success_cell_12(mut v0: Rc<str>) -> bool {
    let v1: bool = { let cell = (v0).trim(); !cell.is_empty() && !matches!(cell, "0" | "false" | "no" | "-") };
    v1
}
fn tallied_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: i32, mut v10: f64, mut v11: f64, mut v12: f64, mut v13: i32) -> (i32, f64, f64, f64, i32) {
    let v14: bool = v7 > v8;
    let (mut v49, mut v50, mut v51, mut v52, mut v53): (i32, f64, f64, f64, i32) = if v14 {
        (v9, v10, v11, v12, v13)
    } else {
        let v15: bool = in_scope_8(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
        let (mut v44, mut v45, mut v46, mut v47, mut v48): (i32, f64, f64, f64, i32) = if v15 {
            let v16: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v17: f64 = number_parsed_11(v16.clone());
            let v18: bool = v17 == v17;
            let v19: i32 = v9.wrapping_add(1i32);
            let v20: bool = success_cell_12(v16);
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
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, f64, f64, f64, i32) = tallied_7(v0, v1, v2, v3, v4, v5, v6, v32, v8, v27, v28, v29, v30, v31);
            (v33, v34, v35, v36, v37)
        } else {
            let v38: i32 = v7.wrapping_add(1i32);
            let (mut v39, mut v40, mut v41, mut v42, mut v43): (i32, f64, f64, f64, i32) = tallied_7(v0, v1, v2, v3, v4, v5, v6, v38, v8, v9, v10, v11, v12, v13);
            (v39, v40, v41, v42, v43)
        };
        (v44, v45, v46, v47, v48)
    };
    (v49, v50, v51, v52, v53)
}
fn confidence_given_13(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: Rc<str>, mut v10: &Rc<str>, mut v11: &Rc<str>, mut v12: &Rc<str>, mut v13: &Rc<str>) -> f64 {
    let v14: f64 = number_parsed_11(v9);
    let v15: bool = v14 == v14;
    if v15 {
        v14
    } else {
        0.95f64
    }
}
fn concat5_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn values_of_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: i32, mut v9: Rc<str>) -> Rc<str> {
    loop {
        let v10: bool = v7 > v8;
        if v10 {
            return v9;
        } else {
            let v11: bool = in_scope_8(v0.clone(), v1.clone(), v2, v3, v4.clone(), v5.clone(), v7);
            if v11 {
                let v12: Rc<str> = std::rc::Rc::<str>::from((v0).get(v7 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v13: f64 = number_parsed_11(v12.clone());
                let v14: f64 = number_parsed_11(v12.clone());
                let v15: bool = v13 == v14;
                let v21: Rc<str> = if v15 {
                    let v16: bool = &*v9 == "";
                    if v16 {
                        v12
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        concat5_15(v9, v17, v12, v18.clone(), v18.clone())
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
fn median_terms_between_17(mut v0: Rc<str>, mut v1: f64, mut v2: f64) -> Rc<str> {
    let v3: Rc<str> = kit_experiment_stats::kit_median_terms(&*v0, v1, v2);
    v3
}
fn median_terms_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v2).next().unwrap_or(""));
    let v4: f64 = number_parsed_11(v3);
    let v5: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v2).map(|(_, rest)| rest).unwrap_or(""));
    let v6: f64 = number_parsed_11(v5);
    median_terms_between_17(v0, v4, v6)
}
fn i32_text_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string().as_str());
    v1
}
fn numeric_note_19(mut v0: US0, mut v1: i32, mut v2: f64, mut v3: f64, mut v4: f64, mut v5: i32) -> Rc<str> {
    let v6: i32 = v2 as i32;
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
        let v13: Rc<str> = i32_text_18(v6);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" numeric)"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_15(v12, v13, v14, v15.clone(), v15)
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v17
    }
}
fn stdout_line_20(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn tested_run_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: US0, mut v15: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> i32 {
    let (mut v16, mut v17): (bool, Rc<str>) = kit_table_query::kit_table_rows_kept(&*v0, &*v1);
    let v20: US1 = if v16 {
        US1::US1_RowsKept(v17)
    } else {
        US1::US1_RowsRefused(v17)
    };
    let (mut v27, mut v28): (Rc<str>, Rc<str>) = match v20 {
        US1::US1_RowsKept(v21) => {
            let v21: Rc<str> = v21;
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v21, v22)
        }
        US1::US1_RowsRefused(v23) => {
            let v23: Rc<str> = v23;
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v24, v23)
        }
    };
    let v29: i32 = optional_column_5(v15.clone(), v5.clone());
    let v30: i32 = (v15).first().and_then(|header| header.iter().position(|item| &**item == &*v7)).map(|index| index as i32).unwrap_or(-1);
    let v31: bool = v30 < 0i32;
    let v33: bool = if v31 {
        true
    } else {
        let v32: bool = &*v28 != "";
        v32
    };
    let v37: bool = if v33 {
        true
    } else {
        let v34: bool = &*v5 != "";
        if v34 {
            let v35: bool = v29 < 0i32;
            v35
        } else {
            false
        }
    };
    if v37 {
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns in "); } LIT.with(|lit| lit.clone()) }, v0));
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) }));
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = std::rc::Rc::<str>::from((v15).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v40)).unwrap_or_default().as_str());
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
        refused_1(v43)
    } else {
        let v45: bool = &*v4 == "";
        let v49: Rc<str> = if v45 {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v46
        } else {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = std::rc::Rc::<str>::from((v4).split(&*v47).next().unwrap_or(""));
            v48
        };
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = std::rc::Rc::<str>::from((v4).split_once(&*v50).map(|(_, rest)| rest).unwrap_or(""));
        let v52: i32 = arms_needed_6(v14.clone());
        let v53: bool = v52 == 2i32;
        let v54: i32 = -1i32;
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v56: i32 = 1i32;
        let v57: i32 = ((v15).len().max(1) - 1) as i32;
        let v58: i32 = 0i32;
        let v59: f64 = 0.0f64;
        let v60: f64 = 0.0f64;
        let v61: f64 = 0.0f64;
        let v62: i32 = 0i32;
        let (mut v63, mut v64, mut v65, mut v66, mut v67): (i32, f64, f64, f64, i32) = tallied_7(v15.clone(), v27.clone(), v29, v54, v55.clone(), v49.clone(), v30, v56, v57, v58, v59, v60, v61, v62);
        let (mut v81, mut v82, mut v83, mut v84, mut v85): (i32, f64, f64, f64, i32) = if v53 {
            let v68: i32 = -1i32;
            let v69: i32 = 1i32;
            let v70: i32 = ((v15).len().max(1) - 1) as i32;
            let v71: i32 = 0i32;
            let v72: f64 = 0.0f64;
            let v73: f64 = 0.0f64;
            let v74: f64 = 0.0f64;
            let v75: i32 = 0i32;
            let (mut v76, mut v77, mut v78, mut v79, mut v80): (i32, f64, f64, f64, i32) = tallied_7(v15.clone(), v27.clone(), v29, v68, v55.clone(), v51.clone(), v30, v69, v70, v71, v72, v73, v74, v75);
            (v76, v77, v78, v79, v80)
        } else {
            (v63, v64, v65, v66, v67)
        };
        let v97: Rc<str> = match &v14 {
            US0::US0_BinomialAtMost => {
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial"); } LIT.with(|lit| lit.clone()) };
                v88
            }
            US0::US0_BinomialBand => {
                let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("binomial-band"); } LIT.with(|lit| lit.clone()) };
                v91
            }
            US0::US0_MedianBand => {
                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                v90
            }
            US0::US0_OneSampleT => {
                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one-sample"); } LIT.with(|lit| lit.clone()) };
                v87
            }
            US0::US0_WelchTwoSided => {
                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("welch"); } LIT.with(|lit| lit.clone()) };
                v86
            }
            US0::US0_WilsonBand => {
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wilson"); } LIT.with(|lit| lit.clone()) };
                v89
            }
        };
        let v98: f64 = number_parsed_11(v6.clone());
        let v99: f64 = confidence_given_13(&v0, &v1, &v2, &v3, &v4, &v5, &v6, &v7, &v8, v9, &v10, &v11, &v12, &v13);
        let v100: bool = match &v14 {
            US0::US0_MedianBand => {
                true
            }
            _ => {
                false
            }
        };
        let v107: Rc<str> = if v100 {
            let v101: i32 = -1i32;
            let v102: i32 = 1i32;
            let v103: i32 = ((v15).len().max(1) - 1) as i32;
            let v104: Rc<str> = values_of_14(v15, v27, v29, v101, v55.clone(), v49.clone(), v30, v102, v103, v55.clone());
            median_terms_16(v104, v6)
        } else {
            let v106: Rc<str> = kit_experiment_stats::kit_experiment_terms(&*v97, v63, v64, v65, v66, v67, v81, v82, v83, v84, v85, v98, v99);
            v106
        };
        let v125: Rc<str> = if v53 {
            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v49, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v109: Rc<str> = i32_text_18(v63);
            let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v109));
            let v111: Rc<str> = numeric_note_19(v14.clone(), v63, v64, v65, v66, v67);
            let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v111));
            let v113: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v51));
            let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v113, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v115: Rc<str> = i32_text_18(v81);
            let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
            let v117: Rc<str> = numeric_note_19(v14, v81, v82, v83, v84, v85);
            let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v117));
            let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v118));
            v119
        } else {
            let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v49, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v121: Rc<str> = i32_text_18(v63);
            let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
            let v123: Rc<str> = numeric_note_19(v14, v63, v64, v65, v66, v67);
            let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v123));
            v124
        };
        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table test ok test="); } LIT.with(|lit| lit.clone()) }, v97));
        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" n="); } LIT.with(|lit| lit.clone()) }, v125));
        let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v107));
        let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v129));
        let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v130, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" post_hoc=yes"); } LIT.with(|lit| lit.clone()) }));
        stdout_line_20(v131);
        0i32
    }
}
fn test_of_21(mut v0: Rc<str>) -> US2 {
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
            let mut v7: bool = &*v0 == "binomial";
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
fn comma_count_23(mut v0: Rc<str>) -> i32 {
    let v1: i32 = (v0).split(char::from(44u8)).filter(|item| !item.trim().is_empty()).count() as i32;
    v1
}
fn needs_prediction_24(mut v0: US0) -> bool {
    match v0 {
        US0::US0_WelchTwoSided => {
            false
        }
        _ => {
            true
        }
    }
}
fn plan_checked_22(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> US3 {
    let mut v7: bool = &*v5 == "";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--metric names the primary metric column"); } LIT.with(|lit| lit.clone()) };
        US3::US3_PlanRejected(v8)
    } else {
        let v10: bool = &*v6 == "";
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--decision states the decision rule before any data"); } LIT.with(|lit| lit.clone()) };
            US3::US3_PlanRejected(v11)
        } else {
            let v13: bool = v3 < 2i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--runs is the planned count per arm, at least 2"); } LIT.with(|lit| lit.clone()) };
                US3::US3_PlanRejected(v14)
            } else {
                let v16: bool = &*v1 == "";
                let v19: bool = if v16 {
                    let v17: i32 = arms_needed_6(v0.clone());
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
                    let v36: Rc<str> = concat5_15(v33, v34, v35.clone(), v35.clone(), v35);
                    US3::US3_PlanRejected(v36)
                } else {
                    let v38: i32 = comma_count_23(v1.clone());
                    let v39: i32 = arms_needed_6(v0.clone());
                    let v40: bool = v38 != v39;
                    let v45: bool = if v40 {
                        let v41: bool = &*v1 != "";
                        if v41 {
                            true
                        } else {
                            let v42: i32 = arms_needed_6(v0.clone());
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
                        let v59: i32 = arms_needed_6(v0);
                        let v60: bool = v59 == 2i32;
                        let v63: Rc<str> = if v60 {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("two group values (a,b)"); } LIT.with(|lit| lit.clone()) };
                            v61
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one group value"); } LIT.with(|lit| lit.clone()) };
                            v62
                        };
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = concat5_15(v57, v58, v63, v64.clone(), v64);
                        US3::US3_PlanRejected(v65)
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
                            let v68: i32 = comma_count_23(v4.clone());
                            let v69: bool = v68 != 2i32;
                            v69
                        } else {
                            false
                        };
                        if v70 {
                            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median needs --prediction low,high (the band the median must fall in)"); } LIT.with(|lit| lit.clone()) };
                            US3::US3_PlanRejected(v71)
                        } else {
                            let v73: bool = needs_prediction_24(v0.clone());
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
                                let v90: Rc<str> = concat5_15(v87, v88, v89.clone(), v89.clone(), v89);
                                US3::US3_PlanRejected(v90)
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
fn table_planned_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>) -> i32 {
    let v14: US2 = test_of_21(v2.clone());
    match v14 {
        US2::US2_TestKnown(v17) => {
            let v17: US0 = v17;
            let v18: i32 = 2i32;
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("post hoc"); } LIT.with(|lit| lit.clone()) };
            let v20: US3 = plan_checked_22(v17.clone(), v4.clone(), v5.clone(), v18, v6.clone(), v7.clone(), v19);
            match v20 {
                US3::US3_PlanAccepted => {
                    let v25: Rc<str> = kit_table_text::kit_table_problem(&*v0);
                    let v26: bool = &*v25 != "";
                    let v30: US4 = if v26 {
                        US4::US4_TableUnreadable(v25)
                    } else {
                        let v28: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
                        US4::US4_TableLoaded(v28)
                    };
                    match v30 {
                        US4::US4_TableLoaded(v33) => {
                            let v33: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v33;
                            tested_run_4(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v17, v33)
                        }
                        US4::US4_TableUnreadable(v31) => {
                            let v31: Rc<str> = v31;
                            refused_1(v31)
                        }
                    }
                }
                US3::US3_PlanRejected(v21) => {
                    let v21: Rc<str> = v21;
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) }));
                    refused_1(v23)
                }
            }
        }
        US2::US2_TestUnknown => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --test with a known test; usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) };
            refused_1(v15)
        }
    }
}
fn table_tested_0() -> i32 {
    let v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_experiment_args::kit_experiment_table_test_args(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    let v15: bool = &*v14 != "";
    if v15 {
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] (a one-population test without both measures the whole table) [--prediction x] [--confidence 0.95] [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]..."); } LIT.with(|lit| lit.clone()) }));
        refused_1(v17)
    } else {
        table_planned_3(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        table_tested_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_test_run() -> i32 {
    closure0()()
}
