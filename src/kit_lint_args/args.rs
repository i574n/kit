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
struct Mut0 { l0: i32 }
#[derive(Clone)]
enum US2 {
    US2_SwitchTsv,
    US2_SwitchOpens,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedBaseline,
    US4_ValuedWriteBaseline,
    US4_ValuedLib,
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
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let mut v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_1(v11);
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
                    (v0, v1) = (v5, v3.clone());
                    continue;
                } else {
                    return v1.clone();
                }
            }
            _ => {
                return v1.clone();
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn leading_paths_6(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                let v5: bool = v2.starts_with(&*v4);
                let v6: bool = v5 == false;
                if v6 {
                    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                    let v8: i32 = (v1.clone().borrow().len() as i32);
                    let v9: i32 = (v7.clone().borrow().len() as i32);
                    let v10: i32 = v8.wrapping_add(v9);
                    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v10 as usize]));
                    let mut v12: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                    while method7(v10, v12.clone()) {
                        let v14: i32 = v12.borrow().l0;
                        let v15: bool = v14 < v8;
                        let v19: Rc<str> = if v15 {
                            let v16: Rc<str> = v1.borrow()[v14 as usize].clone();
                            v16.clone()
                        } else {
                            let v17: i32 = v14.wrapping_sub(v8);
                            let v18: Rc<str> = v7.borrow()[v17 as usize].clone();
                            v18.clone()
                        };
                        v11.borrow_mut()[v14 as usize] = v19.clone();
                        let v20: i32 = v14.wrapping_add(1i32);
                        v12.borrow_mut().l0 = v20;
                        ()
                    };
                    (v0, v1) = (v3.clone(), v11.clone());
                    continue;
                } else {
                    return v1.clone();
                }
            }
            _ => {
                return v1.clone();
            }
        }
    }
}
fn switch_applied_9(mut v0: US2, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: bool, mut v7: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
        US2::US2_SwitchTsv => {
            (v1.clone(), v2.clone(), v3.clone(), v4.clone(), true, v6, v7.clone())
        }
        _ => {
            let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
                US2::US2_SwitchOpens => {
                    (v1, v2, v3, v4, v5, true, v7)
                }
                _ => unreachable!(),
            };
            (v8, v9, v10, v11, v12, v13, v14)
        }
    };
    (v15, v16, v17, v18, v19, v20, v21)
}
fn valued_applied_10(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool, mut v7: bool, mut v8: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
        US4::US4_ValuedBaseline => {
            (v2.clone(), v1.clone(), v4.clone(), v5.clone(), v6, v7, v8.clone())
        }
        _ => {
            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
                US4::US4_ValuedWriteBaseline => {
                    (v2.clone(), v3.clone(), v1.clone(), v5, v6, v7, v8.clone())
                }
                _ => {
                    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v0 {
                        US4::US4_ValuedLib => {
                            (v2, v3, v4, v1, v6, v7, v8)
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
fn flags_read_8(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: bool, mut v7: Rc<RefCell<Vec<Rc<str>>>>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) {
    let (mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7.clone())
        }
        _ => {
            let (mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &*v0 {
                UH0::UH0_Cons(v8, v9) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<UH0> = v9.clone();
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tsv"); } LIT.with(|lit| lit.clone()) };
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tsv"); } LIT.with(|lit| lit.clone()) };
                    let v13: bool = &*v8 == &*v12;
                    let v23: US1 = if v13 {
                        let v14: US2 = US2::US2_SwitchTsv;
                        US1::US1_Some(v14)
                    } else {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("opens"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--opens"); } LIT.with(|lit| lit.clone()) };
                        let v18: bool = &*v8 == &*v17;
                        if v18 {
                            let v19: US2 = US2::US2_SwitchOpens;
                            US1::US1_Some(v19)
                        } else {
                            US1::US1_None
                        }
                    };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("baseline"); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--baseline"); } LIT.with(|lit| lit.clone()) };
                    let v26: bool = &*v8 == &*v25;
                    let v42: US3 = if v26 {
                        let v27: US4 = US4::US4_ValuedBaseline;
                        US3::US3_Some(v27)
                    } else {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write-baseline"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--write-baseline"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = &*v8 == &*v30;
                        if v31 {
                            let v32: US4 = US4::US4_ValuedWriteBaseline;
                            US3::US3_Some(v32)
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lib"); } LIT.with(|lit| lit.clone()) };
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--lib"); } LIT.with(|lit| lit.clone()) };
                            let v36: bool = &*v8 == &*v35;
                            if v36 {
                                let v37: US4 = US4::US4_ValuedLib;
                                US3::US3_Some(v37)
                            } else {
                                US3::US3_None
                            }
                        }
                    };
                    match &v23 {
                        US1::US1_Some(v43) => {
                            let v43: US2 = v43.clone();
                            let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = switch_applied_9(v43, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7.clone());
                            flags_read_8(v9.clone(), v44, v45, v46, v47, v48, v49, v50)
                        }
                        _ => {
                            let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &v42 {
                                US3::US3_Some(v58) => {
                                    let v58: US4 = v58.clone();
                                    match &*v9 {
                                        UH0::UH0_Cons(v59, v60) => {
                                            let v59: Rc<str> = v59.clone();
                                            let v60: Rc<UH0> = v60.clone();
                                            let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = valued_applied_10(v58, v59, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7.clone());
                                            flags_read_8(v60, v61, v62, v63, v64, v65, v66, v67)
                                        }
                                        _ => {
                                            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = flags_read_8(v9.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7.clone());
                                            (v75, v76, v77, v78, v79, v80, v81)
                                        }
                                    }
                                }
                                _ => {
                                    let (mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = flags_read_8(v9, v1, v2, v3, v4, v5, v6, v7);
                                    (v89, v90, v91, v92, v93, v94, v95)
                                }
                            };
                            (v96, v97, v98, v99, v100, v101, v102)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v110, v111, v112, v113, v114, v115, v116)
        }
    };
    (v117, v118, v119, v120, v121, v122, v123)
}
fn args_of_5(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
            let v5: Rc<RefCell<Vec<Rc<str>>>> = leading_paths_6(v3.clone(), v4);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: bool = false;
            let v8: bool = false;
            flags_read_8(v3, v2, v6.clone(), v6.clone(), v6, v7, v8, v5)
        }
        _ => {
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = match &*v0 {
                UH0::UH0_Nil => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (v16.clone(), v16.clone(), v16.clone(), v16, false, false, v1)
                }
                _ => unreachable!(),
            };
            (v17, v18, v19, v20, v21, v22, v23)
        }
    };
    (v24, v25, v26, v27, v28, v29, v30)
}
fn lint_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = args_of_5(v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9, v10)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        lint_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_args_read(v0: i32) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(v0)
}
