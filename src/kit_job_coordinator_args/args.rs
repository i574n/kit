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
    US2_StepBuild,
    US2_StepStart,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedSpiral,
    US4_ValuedKit,
    US4_ValuedSpool,
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
fn stepped_6(mut v0: US2, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US2::US2_StepBuild => {
            (true, v2, v3, v4, v5, v6)
        }
        _ => {
            let (mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                US2::US2_StepStart => {
                    (v1, true, v3, v4, v5, v6)
                }
                _ => unreachable!(),
            };
            (v7, v8, v9, v10, v11, v12)
        }
    };
    (v13, v14, v15, v16, v17, v18)
}
fn coordinator_applied_8(mut v0: US4, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
        US4::US4_ValuedSpiral => {
            (v2, v3, v1, v5, v6, v7)
        }
        _ => {
            let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v0 {
                US4::US4_ValuedKit => {
                    (v2, v3, v4, v1, v6, v7)
                }
                _ => {
                    let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v0 {
                        US4::US4_ValuedSpool => {
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
fn flags_read_7(mut v0: Rc<UH0>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v70, mut v71, mut v72, mut v73, mut v74, mut v75): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4, v5, v6)
        }
        _ => {
            let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<UH0> = v8;
                    let v9: bool = &*v7 == "--spiral";
                    let v21: US3 = if v9 {
                        let v10: US4 = US4::US4_ValuedSpiral;
                        US3::US3_Some(v10)
                    } else {
                        let v12: bool = &*v7 == "--kit";
                        if v12 {
                            let v13: US4 = US4::US4_ValuedKit;
                            US3::US3_Some(v13)
                        } else {
                            let v15: bool = &*v7 == "--spool";
                            if v15 {
                                let v16: US4 = US4::US4_ValuedSpool;
                                US3::US3_Some(v16)
                            } else {
                                US3::US3_None
                            }
                        }
                    };
                    match &v21 {
                        US3::US3_Some(v22) => {
                            let v22: US4 = v22.clone();
                            match &*v8 {
                                UH0::UH0_Nil => {
                                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v7));
                                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                    (v1, v2, v3, v4, v5, v24)
                                }
                                _ => {
                                    let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v8) {
                                        UH0::UH0_Cons(v25, v26) => {
                                            let v25: Rc<str> = v25;
                                            let v26: Rc<UH0> = v26;
                                            let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = coordinator_applied_8(v22, v25, v1, v2, v3, v4, v5, v6);
                                            flags_read_7(v26, v27, v28, v29, v30, v31, v32)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v39, v40, v41, v42, v43, v44)
                                }
                            }
                        }
                        _ => {
                            let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v21 {
                                US3::US3_None => {
                                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v7));
                                    (v1, v2, v3, v4, v5, v51)
                                }
                                _ => unreachable!(),
                            };
                            (v52, v53, v54, v55, v56, v57)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v64, v65, v66, v67, v68, v69)
        }
    };
    (v70, v71, v72, v73, v74, v75)
}
fn coordinator_of_5(mut v0: &Rc<UH0>) -> (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            let v3: bool = &*v1 == "build";
            let v11: US1 = if v3 {
                let v4: US2 = US2::US2_StepBuild;
                US1::US1_Some(v4)
            } else {
                let v6: bool = &*v1 == "start";
                if v6 {
                    let v7: US2 = US2::US2_StepStart;
                    US1::US1_Some(v7)
                } else {
                    US1::US1_None
                }
            };
            match &v11 {
                US1::US1_Some(v12) => {
                    let v12: US2 = v12.clone();
                    let v13: bool = false;
                    let v14: bool = false;
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = stepped_6(v12, v13, v14, v15.clone(), v15.clone(), v15.clone(), v15);
                    flags_read_7(v2, v16, v17, v18, v19, v20, v21)
                }
                _ => {
                    let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v11 {
                        US1::US1_None => {
                            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected build or start, received "); } LIT.with(|lit| lit.clone()) }, v1));
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (false, false, v29.clone(), v29.clone(), v29, v28)
                        }
                        _ => unreachable!(),
                    };
                    (v30, v31, v32, v33, v34, v35)
                }
            }
        }
        _ => {
            let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &**v0 {
                UH0::UH0_Nil => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected build or start"); } LIT.with(|lit| lit.clone()) };
                    (false, false, v42.clone(), v42.clone(), v42, v43)
                }
                _ => unreachable!(),
            };
            (v44, v45, v46, v47, v48, v49)
        }
    };
    (v50, v51, v52, v53, v54, v55)
}
fn coordinator_args_read_0(mut v0: i32) -> Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    let v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = dropped_4(v0, v2);
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = coordinator_of_5(&v3);
    Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v8, v9)]))
}
fn closure0() -> Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: i32| -> Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
        coordinator_args_read_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_coordinator_args_read(v0: i32) -> Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(v0)
}
