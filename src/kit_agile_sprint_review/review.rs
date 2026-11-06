#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_SprintOpen,
    US1_SprintClosed,
}
#[derive(Clone)]
enum US2 {
    US2_Committed,
    US2_Dropped,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, i32, US2, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH0>, i32, i32, i32, bool),
    US0_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool, Rc<UH2>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH0>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
pub struct Mut0 { pub l0: Rc<str> }
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US1 = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<UH1> = v8;
                let v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v1));
                (v0, v1) = (v8, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool) = (v0)[v4].clone();
            let v13: i32 = v1.wrapping_sub(1i32);
            let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v2));
            (v1, v2) = (v13, v14);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: US2 = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method7(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method6(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: bool = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let v9: bool = v9;
                let v10: Rc<UH2> = v10;
                let v13: US2 = if v9 {
                    US2::US2_Committed
                } else {
                    US2::US2_Dropped
                };
                let v14: bool = &*v7 == "";
                let mut v18: Rc<UH0> = if v14 {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                } else {
                    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH0::UH0_Cons(v7, v8, v13, v16))
                };
                let v38: Rc<UH1> = match &*v1 {
                    UH1::UH1_Cons(v19, v20, v21, v22, v23, v24, v25) => {
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let mut v23: US1 = v23.clone();
                        let v24: Rc<UH0> = v24.clone();
                        let v25: Rc<UH1> = v25.clone();
                        let v26: bool = &*v19 == &*v2;
                        if v26 {
                            let v27: Rc<UH0> = method7(v24, v18);
                            Rc::new(UH1::UH1_Cons(v19, v20, v21, v22, v23, v27, v25))
                        } else {
                            let v31: US1 = if v6 {
                                US1::US1_SprintOpen
                            } else {
                                US1::US1_SprintClosed
                            };
                            Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v31, v18, v1))
                        }
                    }
                    _ => {
                        let v36: US1 = if v6 {
                            US1::US1_SprintOpen
                        } else {
                            US1::US1_SprintClosed
                        };
                        Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v36, v18, v1))
                    }
                };
                (v0, v1) = (v10, v38);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn sprints_from_3(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>>) -> Rc<UH1> {
    let mut v1: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH2> = method5(v1, v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = method6(v6, v7);
    let v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method4(v8, v9)
}
fn sprints_read_2(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> = kit_agile_sprint_read::kit_agile_sprint_flat_rows(&v0);
    sprints_from_3(v1)
}
fn method10(mut v0: Rc<UH1>, mut v1: US3) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US1 = v6;
                let v7: Rc<UH0> = v7;
                let v8: Rc<UH1> = v8;
                let v20: US3 = match &v1 {
                    US3::US3_None => {
                        let v15: bool = match &v6 {
                            US1::US1_SprintOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v15 {
                            US3::US3_Some(v2, v3, v4, v5, v6, v7)
                        } else {
                            US3::US3_None
                        }
                    }
                    US3::US3_Some(v9, v10, v11, v12, v13, v14) => {
                        let v9: Rc<str> = v9.clone();
                        let v10: Rc<str> = v10.clone();
                        let v11: Rc<str> = v11.clone();
                        let v12: Rc<str> = v12.clone();
                        let v13: US1 = v13.clone();
                        let v14: Rc<UH0> = v14.clone();
                        v1
                    }
                };
                (v0, v1) = (v8, v20);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn first_open_9(mut v0: Rc<UH1>) -> US3 {
    let mut v1: US3 = US3::US3_None;
    method10(v0, v1)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: US1 = v7;
                let v8: Rc<UH0> = v8;
                let v9: Rc<UH1> = v9;
                let v10: bool = &*v3 == &*v0;
                if v10 {
                    return US3::US3_Some(v3, v4, v5, v6, v7, v8);
                } else {
                    v1 = v9;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US3::US3_None;
            }
        }
    }
}
fn chosen_8(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US3 {
    let mut v2: bool = &*v1 == "";
    if v2 {
        first_open_9(v0)
    } else {
        method11(v1, v0)
    }
}
fn task_listed_13(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: bool = &*v5 == &*v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        }
    }
}
fn method12(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: US2 = v5;
                let v6: Rc<UH0> = v6;
                let v7: bool = match v5 {
                    US2::US2_Committed => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v10: bool = if v7 {
                    let v8: i32 = 0i32;
                    task_listed_13(v0, v3, v8)
                } else {
                    false
                };
                let v12: i32 = if v10 {
                    let v11: i32 = v2.wrapping_add(v4);
                    v11
                } else {
                    v2
                };
                (v1, v2) = (v6, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method14(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: US2 = v5;
                let v6: Rc<UH0> = v6;
                let v7: bool = match v5 {
                    US2::US2_Committed => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v11: bool = if v7 {
                    let v8: i32 = 0i32;
                    let v9: bool = task_listed_13(v0, v3, v8);
                    let v10: bool = v9 == false;
                    v10
                } else {
                    false
                };
                let v13: i32 = if v11 {
                    let v12: i32 = v2.wrapping_add(v4);
                    v12
                } else {
                    v2
                };
                (v1, v2) = (v6, v13);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US2 = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = match v4 {
                    US2::US2_Dropped => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v8: i32 = if v6 {
                    let v7: i32 = v1.wrapping_add(v3);
                    v7
                } else {
                    v1
                };
                (v0, v1) = (v5, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn closing_1(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: Rc<UH1> = sprints_read_2(v2);
    let v4: US3 = chosen_8(v3, v1);
    match v4 {
        US3::US3_None => {
            US0::US0_None
        }
        US3::US3_Some(v5, v6, v7, v8, v9, v10) => {
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: Rc<str> = v8;
            let v9: US1 = v9;
            let v10: Rc<UH0> = v10;
            let v11: i32 = 0i32;
            let v12: i32 = method12(v0, v10.clone(), v11);
            let v13: i32 = 0i32;
            let v14: i32 = method14(v0, v10.clone(), v13);
            let v15: i32 = 0i32;
            let v16: i32 = method15(v10.clone(), v15);
            let v17: bool = v14 == 0i32;
            let mut v18: US1 = US1::US1_SprintClosed;
            US0::US0_Some(v5, v6, v7, v8, v18, v10, v12, v14, v16, v17)
        }
    }
}
fn sprint_review_of_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
    let v3: US0 = closing_1(&v2, v1, v0);
    match v3 {
        US0::US0_None => {
            Rc::new(RefCell::new(vec![]))
        }
        US0::US0_Some(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: US1 = v8;
            let v9: Rc<UH0> = v9;
            let v10: i32 = v10;
            let v11: i32 = v11;
            let v12: i32 = v12;
            let v13: bool = v13;
            Rc::new(RefCell::new(vec![(v10, v11, v12)]))
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(i32, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(i32, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
        sprint_review_of_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_row_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: &Rc<UH0>) -> Rc<str> {
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v0.clone()));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v2));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v3));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    ;
    ;
    let v14: bool = match &v4 {
        US1::US1_SprintOpen => {
            true
        }
        _ => {
            false
        }
    };
    let mut v18: US4 = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintOpen"); } LIT.with(|lit| lit.clone()) };
        US4::US4_Some(v15)
    } else {
        US4::US4_None
    };
    let v32: US4 = match v18 {
        US4::US4_None => {
            let v21: bool = match v4 {
                US1::US1_SprintClosed => {
                    true
                }
                _ => {
                    false
                }
            };
            let v25: US4 = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintClosed"); } LIT.with(|lit| lit.clone()) };
                US4::US4_Some(v22)
            } else {
                US4::US4_None
            };
            match v25 {
                US4::US4_None => {
                    US4::US4_None
                }
                US4::US4_Some(v26) => {
                    let v26: Rc<str> = v26;
                    US4::US4_Some(v26)
                }
            }
        }
        US4::US4_Some(v19) => {
            let v19: Rc<str> = v19;
            US4::US4_Some(v19)
        }
    };
    let v36: Rc<str> = match v32 {
        US4::US4_None => {
            std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
        }
        US4::US4_Some(v33) => {
            let v33: Rc<str> = v33;
            v33
        }
    };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(v0.replace(&*v37, &*v38));
    let v40: Rc<str> = Rc::<str>::from(v39.to_lowercase());
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint_"); } LIT.with(|lit| lit.clone()) }, v40));
    let mut v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v42.borrow_mut().push(v7);
    v42.borrow_mut().push(v9);
    v42.borrow_mut().push(v11);
    v42.borrow_mut().push(v13);
    v42.borrow_mut().push(v36);
    let v43: Rc<Vec<Rc<str>>> = Rc::new(v42.borrow().clone());
    let v44: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v43).as_ref().clone()));
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = Rc::<str>::from((*v44.borrow()).join(&*v45));
    let mut v47: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v48);
    v47.borrow_mut().push(v41);
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v49);
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Sprint"); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v50);
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v51);
    v47.borrow_mut().push(v46);
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v52);
    let v53: Rc<Vec<Rc<str>>> = Rc::new(v47.borrow().clone());
    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v55));
    v56
}
fn method20(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_19(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method20(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn sprint_review_row_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: bool) -> Rc<str> {
    let v5: Rc<str> = format_real_19(v3);
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = format_real_19(v2);
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) }));
    let v9: Rc<str> = format_real_19(v1);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) }));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v0.replace(&*v11, &*v12));
    let v14: Rc<str> = Rc::<str>::from(v13.to_lowercase());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint_review_"); } LIT.with(|lit| lit.clone()) }, v14));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v0));
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v18.borrow_mut().push(v17);
    v18.borrow_mut().push(v10);
    v18.borrow_mut().push(v8);
    v18.borrow_mut().push(v6);
    let v19: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v21));
    let mut v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v24);
    v23.borrow_mut().push(v15);
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v25);
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintReview"); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v26);
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v27);
    v23.borrow_mut().push(v22);
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    v23.borrow_mut().push(v28);
    let v29: Rc<Vec<Rc<str>>> = Rc::new(v23.borrow().clone());
    let v30: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v29).as_ref().clone()));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from((*v30.borrow()).join(&*v31));
    v32
}
fn sprint_closed_text_16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: US0 = closing_1(&v0, v1.clone(), v2.clone());
    match v3 {
        US0::US0_None => {
            v2
        }
        US0::US0_Some(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: US1 = v8;
            let v9: Rc<UH0> = v9;
            let v10: i32 = v10;
            let v11: i32 = v11;
            let v12: i32 = v12;
            let v13: bool = v13;
            let v14: bool = &*v2 == "";
            let v17: bool = if v14 {
                true
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v2.ends_with(&*v15);
                v16
            };
            let v19: Rc<str> = if v17 {
                v2
            } else {
                let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                v18
            };
            let v20: Rc<str> = sprint_row_17(v4, v5, v6, v7, v8, &v9);
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let mut v23: Rc<str> = sprint_review_row_18(v1, v10, v11, v12, v13);
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            v25
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<str> {
        sprint_closed_text_16(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_review(v0: &str, v1: &str, v2: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub fn kit_agile_sprint_closed(v0: &str, v1: &str, v2: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
