#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_MeanOf,
    US0_SdOf,
    US0_SumOf,
    US0_MinOf,
    US0_MaxOf,
    US0_CountOf,
    US0_MedianOf,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMeasures,
    UH0_MeasureThen(US0, Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(f64, Rc<UH2>),
}
#[derive(Clone)]
enum UH1 {
    UH1_NoMeasureStats,
    UH1_MeasureStatsThen(i32, i32, f64, f64, f64, f64, Rc<UH2>, Rc<UH1>),
}
#[derive(Clone)]
enum UH3 {
    UH3_NoGroups,
    UH3_GroupThen(Rc<str>, i32, Rc<UH1>, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_NoCells,
    UH4_CellThen(Rc<str>, Rc<UH4>),
}
#[derive(Clone)]
enum US1 {
    US1_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US1_TableUnreadable(Rc<str>),
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn measures_from_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3, v4)
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from((v6).split(&*v7).next().unwrap_or(""));
        let v9: Rc<str> = std::rc::Rc::<str>::from((v6).split_once(&*v7).map(|(_, rest)| rest).unwrap_or(""));
        let v10: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v9)).map(|index| index as i32).unwrap_or(-1);
        let v11: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v12, mut v13): (Rc<UH0>, Rc<str>) = measures_from_3(v0, v11);
        let v14: bool = v10 < 0i32;
        if v14 {
            let v15: bool = &*v9 == "";
            let v20: Rc<str> = if v15 {
                v13
            } else {
                let v16: bool = &*v13 == "";
                if v16 {
                    v9
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v9, v5, v13, v17.clone(), v17.clone())
                }
            };
            (v12, v20)
        } else {
            let v21: bool = &*v8 == "mean";
            let v24: Rc<UH0> = if v21 {
                let v22: US0 = US0::US0_MeanOf;
                Rc::new(UH0::UH0_MeasureThen(v22, v9.clone(), v10, v12.clone()))
            } else {
                v12.clone()
            };
            let v25: bool = &*v8 == "sd";
            let v28: Rc<UH0> = if v25 {
                let v26: US0 = US0::US0_SdOf;
                Rc::new(UH0::UH0_MeasureThen(v26, v9.clone(), v10, v12.clone()))
            } else {
                v24
            };
            let v29: bool = &*v8 == "sum";
            let v32: Rc<UH0> = if v29 {
                let v30: US0 = US0::US0_SumOf;
                Rc::new(UH0::UH0_MeasureThen(v30, v9.clone(), v10, v12.clone()))
            } else {
                v28
            };
            let v33: bool = &*v8 == "min";
            let v36: Rc<UH0> = if v33 {
                let v34: US0 = US0::US0_MinOf;
                Rc::new(UH0::UH0_MeasureThen(v34, v9.clone(), v10, v12.clone()))
            } else {
                v32
            };
            let v37: bool = &*v8 == "max";
            let v40: Rc<UH0> = if v37 {
                let v38: US0 = US0::US0_MaxOf;
                Rc::new(UH0::UH0_MeasureThen(v38, v9.clone(), v10, v12.clone()))
            } else {
                v36
            };
            let v41: bool = &*v8 == "count";
            let v44: Rc<UH0> = if v41 {
                let v42: US0 = US0::US0_CountOf;
                Rc::new(UH0::UH0_MeasureThen(v42, v9.clone(), v10, v12.clone()))
            } else {
                v40
            };
            let v45: bool = &*v8 == "median";
            let v48: Rc<UH0> = if v45 {
                let v46: US0 = US0::US0_MedianOf;
                Rc::new(UH0::UH0_MeasureThen(v46, v9, v10, v12))
            } else {
                v44
            };
            (v48, v13)
        }
    }
}
fn stats_empty_5(mut v0: Rc<UH0>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v2, v3, v4, v5) => {
            let v2: US0 = v2;
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let mut v5: Rc<UH0> = v5;
            let v6: Rc<UH1> = stats_empty_5(v5);
            let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_MeasureStatsThen(0i32, 0i32, 0.0f64, 0.0f64, 0.0f64, 0.0f64, v7, v6))
        }
        UH0::UH0_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoMeasureStats); } CASE.with(|case| case.clone()) }
        }
    }
}
fn row_cells_7(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v4, v5, v6, v7) => {
            let v4: US0 = v4;
            let mut v5: Rc<str> = v5;
            let v6: i32 = v6;
            let v7: Rc<UH0> = v7;
            let v8: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v9: Rc<UH4> = row_cells_7(v7, v1, v2);
            Rc::new(UH4::UH4_CellThen(v8, v9))
        }
        UH0::UH0_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_NoCells); } CASE.with(|case| case.clone()) }
        }
    }
}
fn stats_added_9(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_MeasureStatsThen(v2, v3, v4, v5, v6, v7, v8, v9) => {
            let v2: i32 = *v2;
            let v3: i32 = *v3;
            let v4: f64 = *v4;
            let mut v5: f64 = *v5;
            let v6: f64 = *v6;
            let v7: f64 = *v7;
            let v8: Rc<UH2> = v8.clone();
            let v9: Rc<UH1> = v9.clone();
            match Rc::unwrap_or_clone(v1) {
                UH4::UH4_CellThen(v10, v11) => {
                    let v10: Rc<str> = v10;
                    let v11: Rc<UH4> = v11;
                    let v12: bool = (v10).trim().is_empty();
                    let v14: i32 = if v12 {
                        v3
                    } else {
                        let v13: i32 = v3.wrapping_add(1i32);
                        v13
                    };
                    let v15: bool = (v10).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                    if v15 {
                        let v16: f64 = (v10).trim().parse::<f64>().unwrap_or(0.0);
                        let v17: bool = v2 == 0i32;
                        let v19: bool = if v17 {
                            true
                        } else {
                            let v18: bool = v16 < v6;
                            v18
                        };
                        let v20: f64 = if v19 {
                            v16
                        } else {
                            v6
                        };
                        let v22: bool = if v17 {
                            true
                        } else {
                            let v21: bool = v16 > v7;
                            v21
                        };
                        let v23: f64 = if v22 {
                            v16
                        } else {
                            v7
                        };
                        let v24: i32 = v2.wrapping_add(1i32);
                        let v25: f64 = v4 + v16;
                        let v26: f64 = v16 * v16;
                        let v27: f64 = v5 + v26;
                        let v28: Rc<UH1> = stats_added_9(v9, v11);
                        let v29: Rc<UH2> = Rc::new(UH2::UH2_Cons(v16, v8));
                        Rc::new(UH1::UH1_MeasureStatsThen(v24, v14, v25, v27, v20, v23, v29, v28))
                    } else {
                        let v31: Rc<UH1> = stats_added_9(v9, v11);
                        Rc::new(UH1::UH1_MeasureStatsThen(v2, v14, v4, v5, v6, v7, v8, v31))
                    }
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn group_added_8(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<UH1>, mut v3: Rc<UH4>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_GroupThen(v7, v8, v9, v10) => {
            let v7: Rc<str> = v7;
            let v8: i32 = v8;
            let v9: Rc<UH1> = v9;
            let v10: Rc<UH3> = v10;
            let v11: bool = &*v7 == &*v1;
            if v11 {
                let v12: i32 = v8.wrapping_add(1i32);
                let v13: Rc<UH1> = stats_added_9(v9, v3);
                Rc::new(UH3::UH3_GroupThen(v7, v12, v13, v10))
            } else {
                let v15: Rc<UH3> = group_added_8(v10, v1, v2, v3);
                Rc::new(UH3::UH3_GroupThen(v7, v8, v9, v15))
            }
        }
        UH3::UH3_NoGroups => {
            let v4: Rc<UH1> = stats_added_9(v2, v3);
            let mut v5: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v1, 1i32, v4, v5))
        }
    }
}
fn grouped_from_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: &Rc<RefCell<Vec<u8>>>, mut v2: i32, mut v3: Rc<UH0>, mut v4: Rc<UH1>, mut v5: i32, mut v6: i32, mut v7: Rc<UH3>, mut v8: i32) -> (Rc<UH3>, i32) {
    loop {
        let v9: bool = v6 < v5;
        if v9 {
            return (v7, v8);
        } else {
            let v10: i32 = (v1.clone().borrow().len() as i32);
            let v11: bool = v5 >= v10;
            let v14: bool = if v11 {
                false
            } else {
                let v12: u8 = v1.borrow()[v5 as usize];
                let v13: bool = v12 == 49u8;
                v13
            };
            if v14 {
                let v15: i32 = v5.wrapping_add(1i32);
                let v16: bool = v2 < 0i32;
                let v19: Rc<str> = if v16 {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                    v17
                } else {
                    let v18: Rc<str> = std::rc::Rc::<str>::from((v0).get(v5 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                    v18
                };
                let v20: Rc<UH4> = row_cells_7(v3.clone(), v0.clone(), v5);
                let v21: Rc<UH3> = group_added_8(v7, v19, v4.clone(), v20);
                let v22: i32 = v8.wrapping_add(1i32);
                (v5, v7, v8) = (v15, v21, v22);
                continue;
            } else {
                let v25: i32 = v5.wrapping_add(1i32);
                v5 = v25;
                continue;
            }
        }
    }
}
fn measure_headers_10(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v2, v3, v4, v5) => {
            let v2: US0 = v2;
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let mut v5: Rc<UH0> = v5;
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = match v2 {
                US0::US0_CountOf => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
                US0::US0_MaxOf => {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max"); } LIT.with(|lit| lit.clone()) };
                    v11
                }
                US0::US0_MeanOf => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mean"); } LIT.with(|lit| lit.clone()) };
                    v7
                }
                US0::US0_MedianOf => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("median"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US0::US0_MinOf => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                US0::US0_SdOf => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sd"); } LIT.with(|lit| lit.clone()) };
                    v8
                }
                US0::US0_SumOf => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sum"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
            };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = measure_headers_10(v5);
            concat5_4(v6, v20, v21, v3, v22)
        }
        UH0::UH0_NoMeasures => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn group_inserted_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH3>) -> Rc<UH3> {
    match &*v3 {
        UH3::UH3_GroupThen(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let v7: i32 = *v7;
            let v8: Rc<UH1> = v8.clone();
            let v9: Rc<UH3> = v9.clone();
            let v10: bool = v1 == v7;
            let v13: bool = if v10 {
                let v11: bool = (v0.clone()) < (v6.clone());
                v11
            } else {
                let v12: bool = v7 < v1;
                v12
            };
            if v13 {
                Rc::new(UH3::UH3_GroupThen(v0, v1, v2, v3))
            } else {
                let v15: Rc<UH3> = group_inserted_12(v0, v1, v2, v9);
                Rc::new(UH3::UH3_GroupThen(v6, v7, v8, v15))
            }
        }
        UH3::UH3_NoGroups => {
            let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v0, v1, v2, v4))
        }
    }
}
fn groups_sorted_11(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: Rc<UH3> = v5;
                let v6: Rc<UH3> = group_inserted_12(v2, v3, v4, v1);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1;
            }
        }
    }
}
fn decimal_text_16(mut v0: f64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let text = format!("{:.3}", v0); let trimmed = text.trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)); if trimmed == "-0" { "0".to_owned() } else { trimmed.to_owned() } }.as_str());
    v1
}
fn reversed_onto_20(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: f64 = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn values_merged_19(mut v0: Rc<UH2>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH2::UH2_Nil => {
                return reversed_onto_20(v2, v1);
            }
            _ => {
                match &*v1 {
                    UH2::UH2_Nil => {
                        return reversed_onto_20(v2, v0);
                    }
                    _ => {
                        match &*v0 {
                            UH2::UH2_Cons(v5, v6) => {
                                let mut v5: f64 = *v5;
                                let v6: Rc<UH2> = v6.clone();
                                match &*v1 {
                                    UH2::UH2_Cons(v7, v8) => {
                                        let v7: f64 = *v7;
                                        let v8: Rc<UH2> = v8.clone();
                                        let v9: bool = v5 <= v7;
                                        if v9 {
                                            let v10: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v2));
                                            (v0, v2) = (v6, v10);
                                            continue;
                                        } else {
                                            let v12: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v2));
                                            (v1, v2) = (v8, v12);
                                            continue;
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }
}
fn values_split_21(mut v0: Rc<UH2>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> (Rc<UH2>, Rc<UH2>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: f64 = v3;
                let v4: Rc<UH2> = v4;
                let mut v5: Rc<UH2> = Rc::new(UH2::UH2_Cons(v3, v1));
                (v0, v1, v2) = (v4, v2, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn values_sorted_18(mut v0: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH2::UH2_Cons(v2, v3) => {
            let v2: f64 = *v2;
            let v3: Rc<UH2> = v3.clone();
            match Rc::unwrap_or_clone(v3) {
                UH2::UH2_Nil => {
                    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH2::UH2_Cons(v2, v4))
                }
                _ => {
                    let v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let (mut v8, mut v9): (Rc<UH2>, Rc<UH2>) = values_split_21(v0, v6, v7);
                    let v10: Rc<UH2> = values_sorted_18(v8);
                    let v11: Rc<UH2> = values_sorted_18(v9);
                    let v12: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    values_merged_19(v10, v11, v12)
                }
            }
        }
        UH2::UH2_Nil => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn value_at_22(mut v0: Rc<UH2>, mut v1: i32) -> f64 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: f64 = v2;
                let v3: Rc<UH2> = v3;
                let v4: bool = v1 <= 0i32;
                if v4 {
                    return v2;
                } else {
                    let mut v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3, v5);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return 0.0f64;
            }
        }
    }
}
fn median_of_17(mut v0: Rc<UH2>, mut v1: i32) -> f64 {
    let v2: Rc<UH2> = values_sorted_18(v0);
    let v3: i32 = v1.wrapping_div(2i32);
    let v4: f64 = value_at_22(v2.clone(), v3);
    let mut v5: i32 = v1.wrapping_rem(2i32);
    let v6: bool = v5 == 1i32;
    if v6 {
        v4
    } else {
        let v7: i32 = v3.wrapping_sub(1i32);
        let v8: f64 = value_at_22(v2, v7);
        let v9: f64 = v8 + v4;
        let v10: f64 = v9 / 2.0f64;
        v10
    }
}
fn measure_value_15(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64, mut v7: Rc<UH2>) -> Rc<str> {
    match v0 {
        US0::US0_CountOf => {
            let v13: f64 = (v2 as f64);
            decimal_text_16(v13)
        }
        US0::US0_MaxOf => {
            let v19: bool = v1 == 0i32;
            if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v20
            } else {
                decimal_text_16(v6)
            }
        }
        US0::US0_MeanOf => {
            let v24: bool = v1 == 0i32;
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v25
            } else {
                let v26: f64 = (v1 as f64);
                let v27: f64 = v3 / v26;
                decimal_text_16(v27)
            }
        }
        US0::US0_MedianOf => {
            let v8: bool = v1 == 0i32;
            if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9
            } else {
                let v10: f64 = median_of_17(v7, v1);
                decimal_text_16(v10)
            }
        }
        US0::US0_MinOf => {
            let v15: bool = v1 == 0i32;
            if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v16
            } else {
                decimal_text_16(v5)
            }
        }
        US0::US0_SdOf => {
            let v30: bool = v1 < 2i32;
            if v30 {
                let v31: bool = v1 == 0i32;
                if v31 {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v32
                } else {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    v33
                }
            } else {
                let v35: f64 = v3 * v3;
                let v36: f64 = (v1 as f64);
                let v37: f64 = v35 / v36;
                let v38: f64 = v4 - v37;
                let v39: i32 = v1.wrapping_sub(1i32);
                let v40: f64 = (v39 as f64);
                let v41: f64 = v38 / v40;
                let v42: f64 = (v41).max(0.0).sqrt();
                decimal_text_16(v42)
            }
        }
        US0::US0_SumOf => {
            decimal_text_16(v3)
        }
    }
}
fn measure_cells_14(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v2, v3, v4, v5) => {
            let v2: US0 = v2;
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let mut v5: Rc<UH0> = v5;
            match Rc::unwrap_or_clone(v1) {
                UH1::UH1_MeasureStatsThen(v6, v7, v8, v9, v10, v11, v12, v13) => {
                    let v6: i32 = v6;
                    let v7: i32 = v7;
                    let v8: f64 = v8;
                    let v9: f64 = v9;
                    let v10: f64 = v10;
                    let v11: f64 = v11;
                    let v12: Rc<UH2> = v12;
                    let v13: Rc<UH1> = v13;
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = measure_value_15(v2, v6, v7, v8, v9, v10, v11, v12);
                    let v16: Rc<str> = measure_cells_14(v5, v13);
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v14, v15, v16, v17.clone(), v17)
                }
                _ => {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v19
                }
            }
        }
        _ => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v21
        }
    }
}
fn groups_printed_13(mut v0: Rc<UH0>, mut v1: Rc<UH3>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_GroupThen(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4;
                let mut v5: i32 = v5;
                let v6: Rc<UH1> = v6;
                let v7: Rc<UH3> = v7;
                let v8: bool = (v3 as u64) < v2;
                if v8 {
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string().as_str());
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v4));
                    let v12: Rc<str> = measure_cells_14(v0.clone(), v6);
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    println!("{}", v13);
                    let v14: i32 = v3.wrapping_add(1i32);
                    (v1, v3) = (v7, v14);
                    continue;
                } else {
                    return v3;
                }
            }
            UH3::UH3_NoGroups => {
                return v3;
            }
        }
    }
}
fn group_count_23(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: Rc<UH1> = v4;
                let mut v5: Rc<UH3> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1;
            }
        }
    }
}
fn grouped_measures_2(mut v0: bool, mut v1: Rc<str>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: bool) -> i32 {
    let v11: bool = &*v5 == "";
    let v13: i32 = if v11 {
        -1i32
    } else {
        let v12: i32 = (v2).first().and_then(|header| header.iter().position(|item| &**item == &*v5)).map(|index| index as i32).unwrap_or(-1);
        v12
    };
    let (mut v16, mut v17): (Rc<str>, Rc<str>) = if v0 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v1, v14)
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v15, v1)
    };
    let v18: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v16.as_bytes().to_vec()));
    let (mut v19, mut v20): (Rc<UH0>, Rc<str>) = measures_from_3(v2.clone(), v8);
    let v21: bool = v11 == false;
    let v23: bool = if v21 {
        let v22: bool = v13 < 0i32;
        v22
    } else {
        false
    };
    let v25: Rc<str> = if v23 {
        v5.clone()
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v24
    };
    let v26: bool = &*v17 == "";
    let v32: Rc<str> = if v26 {
        v25
    } else {
        let v27: bool = &*v25 == "";
        if v27 {
            v17
        } else {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_4(v17, v28, v25, v29.clone(), v29.clone())
        }
    };
    let v33: bool = &*v32 == "";
    let v39: Rc<str> = if v33 {
        v20
    } else {
        let v34: bool = &*v20 == "";
        if v34 {
            v32
        } else {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_4(v32, v35, v20, v36.clone(), v36.clone())
        }
    };
    let v40: bool = &*v39 != "";
    if v40 {
        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) }, v39));
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) }));
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<str> = std::rc::Rc::<str>::from((v2).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v43)).unwrap_or_default().as_str());
        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v44));
        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v46));
        eprintln!("{}", v47);
        2i32
    } else {
        let v48: Rc<UH1> = stats_empty_5(v19.clone());
        let v49: i32 = 1i32;
        let v50: i32 = ((v2).len().max(1) - 1) as i32;
        let v51: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let v52: i32 = 0i32;
        let (mut v53, mut v54): (Rc<UH3>, i32) = grouped_from_6(v2.clone(), &v18, v13, v19.clone(), v48, v49, v50, v51, v52);
        let v56: Rc<str> = if v11 {
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v55
        } else {
            v5
        };
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count\t"); } LIT.with(|lit| lit.clone()) }, v56));
        let v58: Rc<str> = measure_headers_10(v19.clone());
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
        println!("{}", v59);
        let v60: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let v61: Rc<UH3> = groups_sorted_11(v53.clone(), v60);
        let v62: i32 = 0i32;
        let v63: i32 = groups_printed_13(v19, v61, v7, v62);
        let v64: Rc<str> = std::rc::Rc::<str>::from((v54).to_string().as_str());
        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) }, v64));
        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) }));
        let v67: i32 = 0i32;
        let v68: i32 = group_count_23(v53, v67);
        let v69: Rc<str> = std::rc::Rc::<str>::from((v68).to_string().as_str());
        let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v69));
        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) }));
        if v10 {
            ()
        } else {
            let v72: i32 = ((v2).len().max(1) - 1) as i32;
            let v73: Rc<str> = std::rc::Rc::<str>::from((v72).to_string().as_str());
            let v74: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) }, v73));
            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v71));
            let v76: Rc<str> = std::rc::Rc::<str>::from((v63).to_string().as_str());
            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
            println!("{}", v77);
            ()
        };
        0i32
    }
}
fn measured_query_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: bool) -> i32 {
    let v11: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let v12: bool = &*v11 != "";
    let v16: US1 = if v12 {
        US1::US1_TableUnreadable(v11)
    } else {
        let v14: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US1::US1_TableLoaded(v14)
    };
    match v16 {
        US1::US1_TableLoaded(v19) => {
            let v19: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v19;
            grouped_measures_2(v1, v2, v19, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        US1::US1_TableUnreadable(v17) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v17));
            eprintln!("{}", v18);
            2i32
        }
    }
}
fn no_request_24() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn measured_entry_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: bool, mut v5: bool, mut v6: Rc<str>) -> i32 {
    let (mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_24();
    measured_query_1(v0, v5, v6, v7, v8, v1, v10, v3, v2, v13, v4)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, u64, bool, bool, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, u64, bool, bool, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: bool, mut v5: bool, mut v6: Rc<str>| -> i32 {
        measured_entry_0(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_measured(v0: &str, v1: &str, v2: &str, v3: u64, v4: bool, v5: bool, v6: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, v5, Rc::<str>::from(v6))
}
