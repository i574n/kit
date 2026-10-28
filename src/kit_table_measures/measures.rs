#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_WhereEqual,
    US0_WhereNotEqual,
    US0_WhereGreater,
    US0_WhereGreaterEqual,
    US0_WhereLess,
    US0_WhereLessEqual,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(i32, US0, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, US0, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_MeanOf,
    US2_SdOf,
    US2_SumOf,
    US2_MinOf,
    US2_MaxOf,
    US2_CountOf,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoMeasures,
    UH1_MeasureThen(US2, Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_NoMeasureStats,
    UH2_MeasureStatsThen(i32, i32, f64, f64, f64, f64, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_NoGroups,
    UH3_GroupThen(Rc<str>, i32, Rc<UH2>, Rc<UH3>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(f64),
    US3_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_NoCells,
    UH4_CellThen(Rc<str>, Rc<UH4>),
}
#[derive(Clone)]
enum US4 {
    US4_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US4_TableUnreadable(Rc<str>),
}
fn operator_from_4(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return -1i32;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
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
fn filters_from_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v7, mut v8): (Rc<UH0>, Rc<str>) = filters_from_3(v0.clone(), v6.clone());
        let v9: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v10: i32 = 0i32;
        let v11: i32 = operator_from_4(v9.clone(), v10);
        let v12: bool = v11 < 0i32;
        let v54: US1 = if v12 {
            US1::US1_None
        } else {
            let v14: i32 = v11.wrapping_add(1i32);
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
                let v20: u8 = v9.clone().as_bytes()[v11 as usize];
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
            let v28: i32 = v11.wrapping_add(v23);
            let v29: usize = ((v28) as usize);
            let v30: Rc<str> = Rc::<str>::from(&(v9)[v27..v29]);
            let v31: bool = &*v30 == "!=";
            let v46: US0 = if v31 {
                US0::US0_WhereNotEqual
            } else {
                let v33: bool = &*v30 == ">";
                if v33 {
                    US0::US0_WhereGreater
                } else {
                    let v35: bool = &*v30 == ">=";
                    if v35 {
                        US0::US0_WhereGreaterEqual
                    } else {
                        let v37: bool = &*v30 == "<";
                        if v37 {
                            US0::US0_WhereLess
                        } else {
                            let v39: bool = &*v30 == "<=";
                            if v39 {
                                US0::US0_WhereLessEqual
                            } else {
                                US0::US0_WhereEqual
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
            US1::US1_Some(v26.clone(), v46.clone(), v52.clone())
        };
        match &v54 {
            US1::US1_None => {
                (v7.clone(), v8.clone())
            }
            US1::US1_Some(v55, v56, v57) => {
                let v55: Rc<str> = v55.clone();
                let v56: US0 = v56.clone();
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
fn measures_from_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH1>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoMeasures); } CASE.with(|case| case.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from((v6).split(&*v7).next().unwrap_or(""));
        let v9: Rc<str> = std::rc::Rc::<str>::from((v6).split_once(&*v7).map(|(_, rest)| rest).unwrap_or(""));
        let v10: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v9)).map(|index| index as i32).unwrap_or(-1);
        let v11: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v12, mut v13): (Rc<UH1>, Rc<str>) = measures_from_5(v0.clone(), v11.clone());
        let v14: bool = v10 < 0i32;
        if v14 {
            let v15: bool = &*v9 == "";
            let v20: Rc<str> = if v15 {
                v13.clone()
            } else {
                let v16: bool = &*v13 == "";
                if v16 {
                    v9.clone()
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v5, &*v13, &*v17, &*v17].concat());
                    v18.clone()
                }
            };
            (v12.clone(), v20.clone())
        } else {
            let v21: bool = &*v8 == "mean";
            let v24: Rc<UH1> = if v21 {
                let v22: US2 = US2::US2_MeanOf;
                Rc::new(UH1::UH1_MeasureThen(v22.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v12.clone()
            };
            let v25: bool = &*v8 == "sd";
            let v28: Rc<UH1> = if v25 {
                let v26: US2 = US2::US2_SdOf;
                Rc::new(UH1::UH1_MeasureThen(v26.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v24.clone()
            };
            let v29: bool = &*v8 == "sum";
            let v32: Rc<UH1> = if v29 {
                let v30: US2 = US2::US2_SumOf;
                Rc::new(UH1::UH1_MeasureThen(v30.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v28.clone()
            };
            let v33: bool = &*v8 == "min";
            let v36: Rc<UH1> = if v33 {
                let v34: US2 = US2::US2_MinOf;
                Rc::new(UH1::UH1_MeasureThen(v34.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v32.clone()
            };
            let v37: bool = &*v8 == "max";
            let v40: Rc<UH1> = if v37 {
                let v38: US2 = US2::US2_MaxOf;
                Rc::new(UH1::UH1_MeasureThen(v38.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v36.clone()
            };
            let v41: bool = &*v8 == "count";
            let v44: Rc<UH1> = if v41 {
                let v42: US2 = US2::US2_CountOf;
                Rc::new(UH1::UH1_MeasureThen(v42.clone(), v9.clone(), v10, v12.clone()))
            } else {
                v40.clone()
            };
            (v44.clone(), v13.clone())
        }
    }
}
fn stats_empty_6(mut v0: Rc<UH1>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let v2: US2 = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH2> = stats_empty_6(v5.clone());
            Rc::new(UH2::UH2_MeasureStatsThen(0i32, 0i32, 0.0f64, 0.0f64, 0.0f64, 0.0f64, v6.clone()))
        }
        UH1::UH1_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_NoMeasureStats); } CASE.with(|case| case.clone()) }
        }
    }
}
fn row_passes_8(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let v3: i32 = *v3;
                let v4: US0 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v33: bool = match &v4 {
                    US0::US0_WhereEqual => {
                        let v8: bool = &*v7 == &*v5;
                        v8
                    }
                    US0::US0_WhereNotEqual => {
                        let v9: bool = &*v7 != &*v5;
                        v9
                    }
                    _ => {
                        let v10: bool = (v7).trim().parse::<f64>().is_ok();
                        let v14: US3 = if v10 {
                            let v11: f64 = (v7).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v11)
                        } else {
                            US3::US3_None
                        };
                        let v15: bool = (v5).trim().parse::<f64>().is_ok();
                        let v19: US3 = if v15 {
                            let v16: f64 = (v5).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v16)
                        } else {
                            US3::US3_None
                        };
                        match &v14 {
                            US3::US3_Some(v20) => {
                                let v20: f64 = *v20;
                                match &v19 {
                                    US3::US3_Some(v21) => {
                                        let v21: f64 = *v21;
                                        match &v4 {
                                            US0::US0_WhereGreater => {
                                                let v22: bool = v20 > v21;
                                                v22
                                            }
                                            US0::US0_WhereGreaterEqual => {
                                                let v23: bool = v20 >= v21;
                                                v23
                                            }
                                            US0::US0_WhereLess => {
                                                let v24: bool = v20 < v21;
                                                v24
                                            }
                                            US0::US0_WhereLessEqual => {
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
fn row_cells_9(mut v0: Rc<UH1>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> Rc<UH4> {
    match &*v0 {
        UH1::UH1_MeasureThen(v4, v5, v6, v7) => {
            let v4: US2 = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: i32 = *v6;
            let v7: Rc<UH1> = v7.clone();
            let v8: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v6 as usize)).map(|cell| &**cell).unwrap_or(""));
            let v9: Rc<UH4> = row_cells_9(v7.clone(), v1.clone(), v2);
            Rc::new(UH4::UH4_CellThen(v8.clone(), v9.clone()))
        }
        UH1::UH1_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_NoCells); } CASE.with(|case| case.clone()) }
        }
    }
}
fn stats_added_11(mut v0: Rc<UH2>, mut v1: Rc<UH4>) -> Rc<UH2> {
    match &*v0 {
        UH2::UH2_MeasureStatsThen(v2, v3, v4, v5, v6, v7, v8) => {
            let v2: i32 = *v2;
            let v3: i32 = *v3;
            let v4: f64 = *v4;
            let v5: f64 = *v5;
            let v6: f64 = *v6;
            let v7: f64 = *v7;
            let v8: Rc<UH2> = v8.clone();
            match &*v1 {
                UH4::UH4_CellThen(v9, v10) => {
                    let v9: Rc<str> = v9.clone();
                    let v10: Rc<UH4> = v10.clone();
                    let v11: bool = (v9).trim().is_empty();
                    let v13: i32 = if v11 {
                        v3
                    } else {
                        let v12: i32 = v3.wrapping_add(1i32);
                        v12
                    };
                    let v14: bool = (v9).trim().parse::<f64>().map(|value| value.is_finite()).unwrap_or(false);
                    if v14 {
                        let v15: f64 = (v9).trim().parse::<f64>().unwrap_or(0.0);
                        let v16: bool = v2 == 0i32;
                        let v18: bool = if v16 {
                            true
                        } else {
                            let v17: bool = v15 < v6;
                            v17
                        };
                        let v19: f64 = if v18 {
                            v15
                        } else {
                            v6
                        };
                        let v21: bool = if v16 {
                            true
                        } else {
                            let v20: bool = v15 > v7;
                            v20
                        };
                        let v22: f64 = if v21 {
                            v15
                        } else {
                            v7
                        };
                        let v23: i32 = v2.wrapping_add(1i32);
                        let v24: f64 = v4 + v15;
                        let v25: f64 = v15 * v15;
                        let v26: f64 = v5 + v25;
                        let v27: Rc<UH2> = stats_added_11(v8.clone(), v10.clone());
                        Rc::new(UH2::UH2_MeasureStatsThen(v23, v13, v24, v26, v19, v22, v27.clone()))
                    } else {
                        let v29: Rc<UH2> = stats_added_11(v8.clone(), v10.clone());
                        Rc::new(UH2::UH2_MeasureStatsThen(v2, v13, v4, v5, v6, v7, v29.clone()))
                    }
                }
                _ => {
                    v0.clone()
                }
            }
        }
        _ => {
            v0.clone()
        }
    }
}
fn group_added_10(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<UH2>, mut v3: Rc<UH4>) -> Rc<UH3> {
    match &*v0 {
        UH3::UH3_GroupThen(v7, v8, v9, v10) => {
            let v7: Rc<str> = v7.clone();
            let v8: i32 = *v8;
            let v9: Rc<UH2> = v9.clone();
            let v10: Rc<UH3> = v10.clone();
            let v11: bool = &*v7 == &*v1;
            if v11 {
                let v12: i32 = v8.wrapping_add(1i32);
                let v13: Rc<UH2> = stats_added_11(v9.clone(), v3.clone());
                Rc::new(UH3::UH3_GroupThen(v7.clone(), v12, v13.clone(), v10.clone()))
            } else {
                let v15: Rc<UH3> = group_added_10(v10.clone(), v1.clone(), v2.clone(), v3.clone());
                Rc::new(UH3::UH3_GroupThen(v7.clone(), v8, v9.clone(), v15.clone()))
            }
        }
        UH3::UH3_NoGroups => {
            let v4: Rc<UH2> = stats_added_11(v2.clone(), v3.clone());
            let v5: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v1.clone(), 1i32, v4.clone(), v5.clone()))
        }
    }
}
fn grouped_from_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<UH2>, mut v5: i32, mut v6: i32, mut v7: Rc<UH3>, mut v8: i32) -> (Rc<UH3>, i32) {
    loop {
        let v9: bool = v6 < v5;
        if v9 {
            return (v7.clone(), v8);
        } else {
            let v10: bool = row_passes_8(v1.clone(), v0.clone(), v5);
            if v10 {
                let v11: i32 = v5.wrapping_add(1i32);
                let v12: bool = v2 < 0i32;
                let v15: Rc<str> = if v12 {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                } else {
                    let v14: Rc<str> = std::rc::Rc::<str>::from((v0).get(v5 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                    v14.clone()
                };
                let v16: Rc<UH4> = row_cells_9(v3.clone(), v0.clone(), v5);
                let v17: Rc<UH3> = group_added_10(v7.clone(), v15.clone(), v4.clone(), v16.clone());
                let v18: i32 = v8.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v11, v6, v17.clone(), v18);
                continue;
            } else {
                let v21: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v21, v6, v7.clone(), v8);
                continue;
            }
        }
    }
}
fn measure_headers_12(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let v2: US2 = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = match &v2 {
                US2::US2_CountOf => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v12.clone()
                }
                US2::US2_MaxOf => {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                US2::US2_MeanOf => {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mean"); } LIT.with(|lit| lit.clone()) };
                    v7.clone()
                }
                US2::US2_MinOf => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US2::US2_SdOf => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sd"); } LIT.with(|lit| lit.clone()) };
                    v8.clone()
                }
                US2::US2_SumOf => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sum"); } LIT.with(|lit| lit.clone()) };
                    v9.clone()
                }
            };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = measure_headers_12(v5.clone());
            let v21: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v18, &*v19, &*v3, &*v20].concat());
            v21.clone()
        }
        UH1::UH1_NoMeasures => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn group_inserted_14(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> Rc<UH3> {
    match &*v3 {
        UH3::UH3_GroupThen(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let v7: i32 = *v7;
            let v8: Rc<UH2> = v8.clone();
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
                Rc::new(UH3::UH3_GroupThen(v0.clone(), v1, v2.clone(), v3.clone()))
            } else {
                let v15: Rc<UH3> = group_inserted_14(v0.clone(), v1, v2.clone(), v9.clone());
                Rc::new(UH3::UH3_GroupThen(v6.clone(), v7, v8.clone(), v15.clone()))
            }
        }
        UH3::UH3_NoGroups => {
            let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
            Rc::new(UH3::UH3_GroupThen(v0.clone(), v1, v2.clone(), v4.clone()))
        }
    }
}
fn groups_sorted_13(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH2> = v4.clone();
                let v5: Rc<UH3> = v5.clone();
                let v6: Rc<UH3> = group_inserted_14(v2.clone(), v3, v4.clone(), v1.clone());
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1.clone();
            }
        }
    }
}
fn decimal_text_18(mut v0: f64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let text = format!("{:.3}", v0); let trimmed = text.trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)); if trimmed == "-0" { "0".to_owned() } else { trimmed.to_owned() } }.as_str());
    v1
}
fn measure_value_17(mut v0: US2, mut v1: i32, mut v2: i32, mut v3: f64, mut v4: f64, mut v5: f64, mut v6: f64) -> Rc<str> {
    match &v0 {
        US2::US2_CountOf => {
            let v7: f64 = (v2 as f64);
            decimal_text_18(v7)
        }
        US2::US2_MaxOf => {
            let v13: bool = v1 == 0i32;
            if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14.clone()
            } else {
                decimal_text_18(v6)
            }
        }
        US2::US2_MeanOf => {
            let v18: bool = v1 == 0i32;
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v19.clone()
            } else {
                let v20: f64 = (v1 as f64);
                let v21: f64 = v3 / v20;
                decimal_text_18(v21)
            }
        }
        US2::US2_MinOf => {
            let v9: bool = v1 == 0i32;
            if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v10.clone()
            } else {
                decimal_text_18(v5)
            }
        }
        US2::US2_SdOf => {
            let v24: bool = v1 < 2i32;
            if v24 {
                let v25: bool = v1 == 0i32;
                if v25 {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                } else {
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    v27.clone()
                }
            } else {
                let v29: f64 = v3 * v3;
                let v30: f64 = (v1 as f64);
                let v31: f64 = v29 / v30;
                let v32: f64 = v4 - v31;
                let v33: i32 = v1.wrapping_sub(1i32);
                let v34: f64 = (v33 as f64);
                let v35: f64 = v32 / v34;
                let v36: f64 = (v35).max(0.0).sqrt();
                decimal_text_18(v36)
            }
        }
        US2::US2_SumOf => {
            decimal_text_18(v3)
        }
    }
}
fn measure_cells_16(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_MeasureThen(v2, v3, v4, v5) => {
            let v2: US2 = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            match &*v1 {
                UH2::UH2_MeasureStatsThen(v6, v7, v8, v9, v10, v11, v12) => {
                    let v6: i32 = *v6;
                    let v7: i32 = *v7;
                    let v8: f64 = *v8;
                    let v9: f64 = *v9;
                    let v10: f64 = *v10;
                    let v11: f64 = *v11;
                    let v12: Rc<UH2> = v12.clone();
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = measure_value_17(v2.clone(), v6, v7, v8, v9, v10, v11);
                    let v15: Rc<str> = measure_cells_16(v5.clone(), v12.clone());
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v16, &*v16].concat());
                    v17.clone()
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
            }
        }
        _ => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
    }
}
fn groups_printed_15(mut v0: Rc<UH1>, mut v1: Rc<UH3>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match &*v1 {
            UH3::UH3_GroupThen(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let v6: Rc<UH2> = v6.clone();
                let v7: Rc<UH3> = v7.clone();
                let v8: bool = (v3 as u64) < v2;
                if v8 {
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v5).to_string().as_str());
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v4));
                    let v13: Rc<str> = measure_cells_16(v0.clone(), v6.clone());
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    println!("{}", v14);
                    let v15: i32 = v3.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v7.clone(), v2, v15);
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
fn group_count_19(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH3::UH3_GroupThen(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<UH2> = v4.clone();
                let v5: Rc<UH3> = v5.clone();
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH3::UH3_NoGroups => {
                return v1;
            }
        }
    }
}
fn grouped_measures_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: bool = &*v3 == "";
    let v11: i32 = if v9 {
        -1i32
    } else {
        let v10: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v3)).map(|index| index as i32).unwrap_or(-1);
        v10
    };
    let (mut v12, mut v13): (Rc<UH0>, Rc<str>) = filters_from_3(v0.clone(), v1.clone());
    let (mut v14, mut v15): (Rc<UH1>, Rc<str>) = measures_from_5(v0.clone(), v6.clone());
    let v16: bool = v9 == false;
    let v18: bool = if v16 {
        let v17: bool = v11 < 0i32;
        v17
    } else {
        false
    };
    let v20: Rc<str> = if v18 {
        v3.clone()
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    };
    let v21: bool = &*v13 == "";
    let v27: Rc<str> = if v21 {
        v20.clone()
    } else {
        let v22: bool = &*v20 == "";
        if v22 {
            v13.clone()
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v23, &*v20, &*v24, &*v24].concat());
            v25.clone()
        }
    };
    let v28: bool = &*v27 == "";
    let v34: Rc<str> = if v28 {
        v15.clone()
    } else {
        let v29: bool = &*v15 == "";
        if v29 {
            v27.clone()
        } else {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v30, &*v15, &*v31, &*v31].concat());
            v32.clone()
        }
    };
    let v35: bool = &*v34 != "";
    if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v34));
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = std::rc::Rc::<str>::from((v0).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v40)).unwrap_or_default().as_str());
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v41));
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = std::rc::Rc::<str>::from([&*v45, &*v44, &*v46, &*v46, &*v46].concat());
        eprintln!("{}", v47);
        2i32
    } else {
        let v48: Rc<UH2> = stats_empty_6(v14.clone());
        let v49: i32 = 1i32;
        let v50: i32 = ((v0).len().max(1) - 1) as i32;
        let v51: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let v52: i32 = 0i32;
        let (mut v53, mut v54): (Rc<UH3>, i32) = grouped_from_7(v0.clone(), v12.clone(), v11, v14.clone(), v48.clone(), v49, v50, v51.clone(), v52);
        let v56: Rc<str> = if v9 {
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v55.clone()
        } else {
            v3.clone()
        };
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count\t"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v56));
        let v59: Rc<str> = measure_headers_12(v14.clone());
        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
        println!("{}", v60);
        let v61: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_NoGroups); } CASE.with(|case| case.clone()) };
        let v62: Rc<UH3> = groups_sorted_13(v53.clone(), v61.clone());
        let v63: i32 = 0i32;
        let v64: i32 = groups_printed_15(v14.clone(), v62.clone(), v5, v63);
        let v65: Rc<str> = std::rc::Rc::<str>::from((v54).to_string().as_str());
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v65));
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
        let v70: i32 = 0i32;
        let v71: i32 = group_count_19(v53.clone(), v70);
        let v72: Rc<str> = std::rc::Rc::<str>::from((v71).to_string().as_str());
        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v72));
        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
        let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
        if v8 {
            ()
        } else {
            let v76: i32 = ((v0).len().max(1) - 1) as i32;
            let v77: Rc<str> = std::rc::Rc::<str>::from((v76).to_string().as_str());
            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) };
            let v80: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v78, v77)), v75));
            let v81: Rc<str> = std::rc::Rc::<str>::from((v64).to_string().as_str());
            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
            println!("{}", v82);
            ()
        };
        0i32
    }
}
fn measured_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let v10: bool = &*v9 != "";
    let v14: US4 = if v10 {
        US4::US4_TableUnreadable(v9.clone())
    } else {
        let v12: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US4::US4_TableLoaded(v12.clone())
    };
    match &v14 {
        US4::US4_TableLoaded(v19) => {
            let v19: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v19.clone();
            grouped_measures_2(v19.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6.clone(), v7.clone(), v8)
        }
        US4::US4_TableUnreadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v17, &*v17, &*v17].concat());
            eprintln!("{}", v18);
            2i32
        }
    }
}
fn no_request_20() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn measured_entry_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_20();
    measured_query_1(v0, v1, v7, v2, v9, v4, v3, v12, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool| -> i32 {
        measured_entry_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_measured(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64, v5: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
