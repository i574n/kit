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
    US2_ValuedTest,
    US2_ValuedRuns,
    US2_ValuedArms,
    US2_ValuedGroup,
    US2_ValuedPrediction,
    US2_ValuedMetric,
    US2_ValuedDecision,
    US2_ValuedConfidence,
    US2_ValuedQuestion,
    US2_ValuedTime,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ImportRegisteredAt,
    US4_ImportEvidence,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
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
fn valued_applied_7(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US2::US2_ValuedTest => {
            (v1, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
        }
        _ => {
            let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                US2::US2_ValuedRuns => {
                    (v2, v1, v4, v5, v6, v7, v8, v9, v10, v11, v12)
                }
                _ => {
                    let (mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                        US2::US2_ValuedArms => {
                            (v2, v3, v1, v5, v6, v7, v8, v9, v10, v11, v12)
                        }
                        _ => {
                            let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                US2::US2_ValuedGroup => {
                                    (v2, v3, v4, v1, v6, v7, v8, v9, v10, v11, v12)
                                }
                                _ => {
                                    let (mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                        US2::US2_ValuedPrediction => {
                                            (v2, v3, v4, v5, v1, v7, v8, v9, v10, v11, v12)
                                        }
                                        _ => {
                                            let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                                US2::US2_ValuedMetric => {
                                                    (v2, v3, v4, v5, v6, v1, v8, v9, v10, v11, v12)
                                                }
                                                _ => {
                                                    let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                                        US2::US2_ValuedDecision => {
                                                            (v2, v3, v4, v5, v6, v7, v1, v9, v10, v11, v12)
                                                        }
                                                        _ => {
                                                            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                                                US2::US2_ValuedConfidence => {
                                                                    (v2, v3, v4, v5, v6, v7, v8, v1, v10, v11, v12)
                                                                }
                                                                _ => {
                                                                    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                                                                        US2::US2_ValuedQuestion => {
                                                                            (v2, v3, v4, v5, v6, v7, v8, v9, v1, v11, v12)
                                                                        }
                                                                        _ => {
                                                                            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                                                                                US2::US2_ValuedTime => {
                                                                                    (v2, v3, v4, v5, v6, v7, v8, v9, v10, v1, v12)
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            (v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
                                                                        }
                                                                    };
                                                                    (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34)
                                                                }
                                                            };
                                                            (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45)
                                                        }
                                                    };
                                                    (v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56)
                                                }
                                            };
                                            (v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67)
                                        }
                                    };
                                    (v68, v69, v70, v71, v72, v73, v74, v75, v76, v77, v78)
                                }
                            };
                            (v79, v80, v81, v82, v83, v84, v85, v86, v87, v88, v89)
                        }
                    };
                    (v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100)
                }
            };
            (v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111)
        }
    };
    (v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122)
}
fn words_read_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
        }
        _ => {
            let (mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v12, v13) => {
                    let v12: Rc<str> = v12;
                    let v13: Rc<UH0> = v13;
                    let v14: bool = &*v12 == "--test";
                    let v54: US1 = if v14 {
                        let v15: US2 = US2::US2_ValuedTest;
                        US1::US1_Some(v15)
                    } else {
                        let v17: bool = &*v12 == "--runs";
                        if v17 {
                            let v18: US2 = US2::US2_ValuedRuns;
                            US1::US1_Some(v18)
                        } else {
                            let v20: bool = &*v12 == "--arms";
                            if v20 {
                                let v21: US2 = US2::US2_ValuedArms;
                                US1::US1_Some(v21)
                            } else {
                                let v23: bool = &*v12 == "--group";
                                if v23 {
                                    let v24: US2 = US2::US2_ValuedGroup;
                                    US1::US1_Some(v24)
                                } else {
                                    let v26: bool = &*v12 == "--prediction";
                                    if v26 {
                                        let v27: US2 = US2::US2_ValuedPrediction;
                                        US1::US1_Some(v27)
                                    } else {
                                        let v29: bool = &*v12 == "--metric";
                                        if v29 {
                                            let v30: US2 = US2::US2_ValuedMetric;
                                            US1::US1_Some(v30)
                                        } else {
                                            let v32: bool = &*v12 == "--decision";
                                            if v32 {
                                                let v33: US2 = US2::US2_ValuedDecision;
                                                US1::US1_Some(v33)
                                            } else {
                                                let v35: bool = &*v12 == "--confidence";
                                                if v35 {
                                                    let v36: US2 = US2::US2_ValuedConfidence;
                                                    US1::US1_Some(v36)
                                                } else {
                                                    let v38: bool = &*v12 == "--question";
                                                    if v38 {
                                                        let v39: US2 = US2::US2_ValuedQuestion;
                                                        US1::US1_Some(v39)
                                                    } else {
                                                        let v41: bool = &*v12 == "--time";
                                                        if v41 {
                                                            let v42: US2 = US2::US2_ValuedTime;
                                                            US1::US1_Some(v42)
                                                        } else {
                                                            US1::US1_None
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match &v54 {
                        US1::US1_Some(v55) => {
                            let v55: US2 = v55.clone();
                            match &*v13 {
                                UH0::UH0_Nil => {
                                    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v12));
                                    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                    (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v57)
                                }
                                _ => {
                                    let (mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v13) {
                                        UH0::UH0_Cons(v58, v59) => {
                                            let v58: Rc<str> = v58;
                                            let v59: Rc<UH0> = v59;
                                            let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = valued_applied_7(v55, v58, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11);
                                            words_read_6(v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v82, v83, v84, v85, v86, v87, v88, v89, v90, v91, v92)
                                }
                            }
                        }
                        _ => {
                            let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v54 {
                                US1::US1_None => {
                                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v12));
                                    (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v104)
                                }
                                _ => unreachable!(),
                            };
                            (v105, v106, v107, v108, v109, v110, v111, v112, v113, v114, v115)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v127, v128, v129, v130, v131, v132, v133, v134, v135, v136, v137)
        }
    };
    (v138, v139, v140, v141, v142, v143, v144, v145, v146, v147, v148)
}
fn register_of_5(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0.95"); } LIT.with(|lit| lit.clone()) };
    let (mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = words_read_6(v0, v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v1.clone(), v2, v1.clone(), v1.clone(), v1);
    (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
}
fn register_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = register_of_5(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        register_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn import_applied_11(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US4::US4_ImportRegisteredAt => {
            (v2, v3, v1, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
        }
        _ => {
            let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                US4::US4_ImportEvidence => {
                    (v2, v3, v4, v1, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
                }
                _ => unreachable!(),
            };
            (v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
        }
    };
    (v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49)
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn import_split_10(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v20, v21) => {
                let v20: Rc<str> = v20;
                let v21: Rc<UH0> = v21;
                let v22: bool = &*v20 == "--registered-at";
                let v30: US3 = if v22 {
                    let v23: US4 = US4::US4_ImportRegisteredAt;
                    US3::US3_Some(v23)
                } else {
                    let v25: bool = &*v20 == "--evidence";
                    if v25 {
                        let v26: US4 = US4::US4_ImportEvidence;
                        US3::US3_Some(v26)
                    } else {
                        US3::US3_None
                    }
                };
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                let v32: bool = v20.starts_with(&*v31);
                let v33: bool = v32 == false;
                match v30 {
                    US3::US3_None => {
                        let v109: bool = if v33 {
                            let v108: bool = &*v1 == "";
                            v108
                        } else {
                            false
                        };
                        if v109 {
                            (v0, v1) = (v21, v20);
                            continue;
                        } else {
                            let v128: bool = if v33 {
                                let v127: bool = &*v2 == "";
                                v127
                            } else {
                                false
                            };
                            if v128 {
                                (v0, v2) = (v21, v20);
                                continue;
                            } else {
                                let v146: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v17));
                                (v0, v17) = (v21, v146);
                                continue;
                            }
                        }
                    }
                    US3::US3_Some(v34) => {
                        let v34: US4 = v34;
                        match Rc::unwrap_or_clone(v21) {
                            UH0::UH0_Cons(v39, v40) => {
                                let v39: Rc<str> = v39;
                                let v40: Rc<UH0> = v40;
                                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = import_applied_11(v34, v39, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
                                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = (v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56);
                                continue;
                            }
                            UH0::UH0_Nil => {
                                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v20));
                                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                let v37: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v38: Rc<UH0> = method12(v17, v37);
                                return (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v36, v38);
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH0> = method12(v17, v18);
                return (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v19);
            }
        }
    }
}
fn import_of_9(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = register_of_5(v1);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>) = import_split_10(v0, v13.clone(), v13.clone(), v13.clone(), v13.clone(), v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = register_of_5(v31);
    let v43: bool = &*v30 != "";
    let v44: Rc<str> = if v43 {
        v30
    } else {
        v42.clone()
    };
    (v15, v16, v17, v18, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v44)
}
fn import_args_read_8(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = import_of_9(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)]))
}
fn closure1() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        import_args_read_8(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn table_split_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v18, v19) => {
                let v18: Rc<str> = v18;
                let v19: Rc<UH0> = v19;
                match &*v19 {
                    UH0::UH0_Cons(v91, v92) => {
                        let v91: Rc<str> = v91.clone();
                        let v92: Rc<UH0> = v92.clone();
                        let v93: bool = &*v18 == "--where";
                        if v93 {
                            let v94: bool = &*v2 == "";
                            let v97: Rc<str> = if v94 {
                                v91
                            } else {
                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v91));
                                v96
                            };
                            (v0, v2) = (v92, v97);
                            continue;
                        } else {
                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                            let v114: bool = v18.starts_with(&*v113);
                            let v115: bool = v114 == false;
                            let v117: bool = if v115 {
                                let v116: bool = &*v1 == "";
                                v116
                            } else {
                                false
                            };
                            if v117 {
                                (v0, v1) = (v19, v18);
                                continue;
                            } else {
                                let v133: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v15));
                                (v0, v15) = (v19, v133);
                                continue;
                            }
                        }
                    }
                    UH0::UH0_Nil => {
                        let v20: bool = &*v18 == "--where";
                        if v20 {
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v18));
                            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v24: Rc<UH0> = method12(v15, v23);
                            return (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v22, v24);
                        } else {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                            let v26: bool = v18.starts_with(&*v25);
                            let v27: bool = v26 == false;
                            let v29: bool = if v27 {
                                let v28: bool = &*v1 == "";
                                v28
                            } else {
                                false
                            };
                            if v29 {
                                (v0, v1) = (v19, v18);
                                continue;
                            } else {
                                let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v15));
                                (v0, v15) = (v19, v45);
                                continue;
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v17: Rc<UH0> = method12(v15, v16);
                return (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v17);
            }
        }
    }
}
fn table_test_of_14(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = register_of_5(v1);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>) = table_split_15(v0, v13.clone(), v13.clone(), v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = register_of_5(v29);
    let v41: bool = &*v28 != "";
    let v42: Rc<str> = if v41 {
        v28
    } else {
        v40.clone()
    };
    (v15, v16, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v42)
}
fn table_test_args_read_13(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = table_test_of_14(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)]))
}
fn closure2() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        table_test_args_read_13(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_experiment_register_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(v0)
}
pub fn kit_experiment_import_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure1()(v0)
}
pub fn kit_experiment_table_test_args(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure2()(v0)
}
