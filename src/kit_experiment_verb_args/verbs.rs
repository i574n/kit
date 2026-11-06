#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_AmendField,
    US2_AmendReason,
    US2_AmendValue,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_AnalyzeInterim,
    US4_AnalyzeWhere,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US6 {
    US6_RecordVerdict,
    US6_RecordAt,
    US6_RecordEvidence,
}
#[derive(Clone)]
enum US5 {
    US5_Some(US6),
    US5_None,
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn dropped_4(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn amend_applied_7(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US2::US2_AmendField => {
            (v2, v3, v4, v1, v6, v7, v8)
        }
        _ => {
            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                US2::US2_AmendReason => {
                    (v2, v3, v4, v5, v1, v7, v8)
                }
                _ => {
                    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                        US2::US2_AmendValue => {
                            (v2, v3, v4, v5, v6, v1, v8)
                        }
                        _ => unreachable!(),
                    };
                    (v9, v10, v11, v12, v13, v14, v15)
                }
            };
            (v16, v17, v18, v19, v20, v21, v22)
        }
    };
    (v23, v24, v25, v26, v27, v28, v29)
}
fn placed_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v8: bool = &*v1 == "";
    let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
        (v0, v2, v3, v4, v5, v6, v7)
    } else {
        let v9: bool = &*v2 == "";
        let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v9 {
            (v1, v0, v3, v4, v5, v6, v7)
        } else {
            let v10: bool = &*v3 == "";
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v10 {
                (v1, v2, v0, v4, v5, v6, v7)
            } else {
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word "); } LIT.with(|lit| lit.clone()) }, v0));
                (v1, v2, v3, v4, v5, v6, v11)
            };
            (v12, v13, v14, v15, v16, v17, v18)
        };
        (v19, v20, v21, v22, v23, v24, v25)
    };
    (v26, v27, v28, v29, v30, v31, v32)
}
fn amend_read_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6, v7)
        }
        _ => {
            let v8: bool = &*v7 != "";
            let (mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
                (v1, v2, v3, v4, v5, v6, v7)
            } else {
                let (mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v9, v10) => {
                        let v9: Rc<str> = v9;
                        let v10: Rc<UH0> = v10;
                        let v11: bool = &*v9 == "--field";
                        let v23: US1 = if v11 {
                            let v12: US2 = US2::US2_AmendField;
                            US1::US1_Some(v12)
                        } else {
                            let v14: bool = &*v9 == "--reason";
                            if v14 {
                                let v15: US2 = US2::US2_AmendReason;
                                US1::US1_Some(v15)
                            } else {
                                let v17: bool = &*v9 == "--value";
                                if v17 {
                                    let v18: US2 = US2::US2_AmendValue;
                                    US1::US1_Some(v18)
                                } else {
                                    US1::US1_None
                                }
                            }
                        };
                        match &v23 {
                            US1::US1_Some(v24) => {
                                let v24: US2 = v24.clone();
                                match &*v10 {
                                    UH0::UH0_Nil => {
                                        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v9));
                                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                        (v1, v2, v3, v4, v5, v6, v26)
                                    }
                                    _ => {
                                        let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v10) {
                                            UH0::UH0_Cons(v27, v28) => {
                                                let v27: Rc<str> = v27;
                                                let v28: Rc<UH0> = v28;
                                                let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = amend_applied_7(v24, v27, v1, v2, v3, v4, v5, v6, v7);
                                                amend_read_6(v28, v29, v30, v31, v32, v33, v34, v35)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v43, v44, v45, v46, v47, v48, v49)
                                    }
                                }
                            }
                            _ => {
                                let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v23 {
                                    US1::US1_None => {
                                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                        let v58: bool = v9.starts_with(&*v57);
                                        if v58 {
                                            let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v9));
                                            (v1, v2, v3, v4, v5, v6, v59)
                                        } else {
                                            let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = placed_8(v9, v1, v2, v3, v4, v5, v6, v7);
                                            let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = amend_read_6(v10, v60, v61, v62, v63, v64, v65, v66);
                                            (v67, v68, v69, v70, v71, v72, v73)
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                (v81, v82, v83, v84, v85, v86, v87)
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                (v95, v96, v97, v98, v99, v100, v101)
            };
            (v102, v103, v104, v105, v106, v107, v108)
        }
    };
    (v109, v110, v111, v112, v113, v114, v115)
}
fn amend_of_5(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = amend_read_6(v0, v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1);
    (v2, v3, v4, v5, v6, v7, v8)
}
fn amend_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = amend_of_5(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        amend_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn analyze_applied_12(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US4::US4_AnalyzeInterim => {
            (v2, v3, v4, v1, v6, v7)
        }
        _ => {
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                US4::US4_AnalyzeWhere => {
                    let v8: bool = &*v6 == "";
                    let v11: Rc<str> = if v8 {
                        v1
                    } else {
                        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v1));
                        v10
                    };
                    (v2, v3, v4, v5, v11, v7)
                }
                _ => unreachable!(),
            };
            (v12, v13, v14, v15, v16, v17)
        }
    };
    (v18, v19, v20, v21, v22, v23)
}
fn analyze_placed_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v7: bool = &*v1 == "";
    let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v7 {
        (v0, v2, v3, v4, v5, v6)
    } else {
        let v8: bool = &*v2 == "";
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
            (v1, v0, v3, v4, v5, v6)
        } else {
            let v9: bool = &*v3 == "";
            let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v9 {
                (v1, v2, v0, v4, v5, v6)
            } else {
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word "); } LIT.with(|lit| lit.clone()) }, v0));
                (v1, v2, v3, v4, v5, v10)
            };
            (v11, v12, v13, v14, v15, v16)
        };
        (v17, v18, v19, v20, v21, v22)
    };
    (v23, v24, v25, v26, v27, v28)
}
fn analyze_read_11(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6)
        }
        _ => {
            let v7: bool = &*v6 != "";
            let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v7 {
                (v1, v2, v3, v4, v5, v6)
            } else {
                let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v8, v9) => {
                        let v8: Rc<str> = v8;
                        let v9: Rc<UH0> = v9;
                        let v10: bool = &*v8 == "--interim";
                        let v18: US3 = if v10 {
                            let v11: US4 = US4::US4_AnalyzeInterim;
                            US3::US3_Some(v11)
                        } else {
                            let v13: bool = &*v8 == "--where";
                            if v13 {
                                let v14: US4 = US4::US4_AnalyzeWhere;
                                US3::US3_Some(v14)
                            } else {
                                US3::US3_None
                            }
                        };
                        match &v18 {
                            US3::US3_Some(v19) => {
                                let v19: US4 = v19.clone();
                                match &*v9 {
                                    UH0::UH0_Nil => {
                                        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v8));
                                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                        (v1, v2, v3, v4, v5, v21)
                                    }
                                    _ => {
                                        let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v9) {
                                            UH0::UH0_Cons(v22, v23) => {
                                                let v22: Rc<str> = v22;
                                                let v23: Rc<UH0> = v23;
                                                let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = analyze_applied_12(v19, v22, v1, v2, v3, v4, v5, v6);
                                                analyze_read_11(v23, v24, v25, v26, v27, v28, v29)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v36, v37, v38, v39, v40, v41)
                                    }
                                }
                            }
                            _ => {
                                let (mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v18 {
                                    US3::US3_None => {
                                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                        let v49: bool = v8.starts_with(&*v48);
                                        if v49 {
                                            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v8));
                                            (v1, v2, v3, v4, v5, v50)
                                        } else {
                                            let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = analyze_placed_13(v8, v1, v2, v3, v4, v5, v6);
                                            let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = analyze_read_11(v9, v51, v52, v53, v54, v55, v56);
                                            (v57, v58, v59, v60, v61, v62)
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                (v69, v70, v71, v72, v73, v74)
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                (v81, v82, v83, v84, v85, v86)
            };
            (v87, v88, v89, v90, v91, v92)
        }
    };
    (v93, v94, v95, v96, v97, v98)
}
fn analyze_of_10(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = analyze_read_11(v0, v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1);
    (v2, v3, v4, v5, v6, v7)
}
fn analyze_args_read_9(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = analyze_of_10(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9)]))
}
fn closure1() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        analyze_args_read_9(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn record_applied_17(mut v0: US6, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US6::US6_RecordVerdict => {
            (v2, v3, v1, v5, v6, v7)
        }
        _ => {
            let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                US6::US6_RecordAt => {
                    (v2, v3, v4, v1, v6, v7)
                }
                _ => {
                    let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                        US6::US6_RecordEvidence => {
                            (v2, v3, v4, v5, v1, v7)
                        }
                        _ => unreachable!(),
                    };
                    (v8, v9, v10, v11, v12, v13)
                }
            };
            (v14, v15, v16, v17, v18, v19)
        }
    };
    (v20, v21, v22, v23, v24, v25)
}
fn verdict_read_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6)
        }
        _ => {
            let v7: bool = &*v6 != "";
            let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v7 {
                (v1, v2, v3, v4, v5, v6)
            } else {
                let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                    UH0::UH0_Cons(v8, v9) => {
                        let v8: Rc<str> = v8;
                        let v9: Rc<UH0> = v9;
                        let v10: bool = &*v8 == "--verdict";
                        let v22: US5 = if v10 {
                            let v11: US6 = US6::US6_RecordVerdict;
                            US5::US5_Some(v11)
                        } else {
                            let v13: bool = &*v8 == "--at";
                            if v13 {
                                let v14: US6 = US6::US6_RecordAt;
                                US5::US5_Some(v14)
                            } else {
                                let v16: bool = &*v8 == "--evidence";
                                if v16 {
                                    let v17: US6 = US6::US6_RecordEvidence;
                                    US5::US5_Some(v17)
                                } else {
                                    US5::US5_None
                                }
                            }
                        };
                        match &v22 {
                            US5::US5_Some(v23) => {
                                let v23: US6 = v23.clone();
                                match &*v9 {
                                    UH0::UH0_Nil => {
                                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v8));
                                        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                        (v1, v2, v3, v4, v5, v25)
                                    }
                                    _ => {
                                        let (mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v9) {
                                            UH0::UH0_Cons(v26, v27) => {
                                                let v26: Rc<str> = v26;
                                                let v27: Rc<UH0> = v27;
                                                let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = record_applied_17(v23, v26, v1, v2, v3, v4, v5, v6);
                                                verdict_read_16(v27, v28, v29, v30, v31, v32, v33)
                                            }
                                            _ => unreachable!(),
                                        };
                                        (v40, v41, v42, v43, v44, v45)
                                    }
                                }
                            }
                            _ => {
                                let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v22 {
                                    US5::US5_None => {
                                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                        let v53: bool = v8.starts_with(&*v52);
                                        if v53 {
                                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v8));
                                            (v1, v2, v3, v4, v5, v54)
                                        } else {
                                            let v55: bool = &*v1 == "";
                                            let (mut v76, mut v77, mut v78, mut v79, mut v80, mut v81): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v55 {
                                                let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = verdict_read_16(v9, v8, v2, v3, v4, v5, v6);
                                                (v56, v57, v58, v59, v60, v61)
                                            } else {
                                                let v62: bool = &*v2 == "";
                                                let (mut v70, mut v71, mut v72, mut v73, mut v74, mut v75): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v62 {
                                                    let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = verdict_read_16(v9, v1, v8, v3, v4, v5, v6);
                                                    (v63, v64, v65, v66, v67, v68)
                                                } else {
                                                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word "); } LIT.with(|lit| lit.clone()) }, v8));
                                                    (v1, v2, v3, v4, v5, v69)
                                                };
                                                (v70, v71, v72, v73, v74, v75)
                                            };
                                            (v76, v77, v78, v79, v80, v81)
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                (v88, v89, v90, v91, v92, v93)
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                (v100, v101, v102, v103, v104, v105)
            };
            (v106, v107, v108, v109, v110, v111)
        }
    };
    (v112, v113, v114, v115, v116, v117)
}
fn verdict_of_15(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = verdict_read_16(v0, v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1);
    (v2, v3, v4, v5, v6, v7)
}
fn verdict_args_read_14(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = verdict_of_15(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9)]))
}
fn closure2() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        verdict_args_read_14(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_amend_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(v0)
}
pub fn kit_experiment_analyze_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure1()(v0)
}
pub fn kit_experiment_verdict_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure2()(v0)
}
