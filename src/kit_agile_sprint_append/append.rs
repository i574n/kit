#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_SprintOpen,
    US0_SprintClosed,
}
#[derive(Clone)]
enum US1 {
    US1_Committed,
    US1_Dropped,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, i32, US1, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn sprint_row_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US0, mut v5: &Rc<UH0>) -> Rc<str> {
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
    let mut v14: bool = match &v4 {
        US0::US0_SprintOpen => {
            true
        }
        _ => {
            false
        }
    };
    let v18: US2 = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintOpen"); } LIT.with(|lit| lit.clone()) };
        US2::US2_Some(v15)
    } else {
        US2::US2_None
    };
    let v32: US2 = match v18 {
        US2::US2_None => {
            let v21: bool = match v4 {
                US0::US0_SprintClosed => {
                    true
                }
                _ => {
                    false
                }
            };
            let v25: US2 = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintClosed"); } LIT.with(|lit| lit.clone()) };
                US2::US2_Some(v22)
            } else {
                US2::US2_None
            };
            match v25 {
                US2::US2_None => {
                    US2::US2_None
                }
                US2::US2_Some(v26) => {
                    let v26: Rc<str> = v26;
                    US2::US2_Some(v26)
                }
            }
        }
        US2::US2_Some(v19) => {
            let v19: Rc<str> = v19;
            US2::US2_Some(v19)
        }
    };
    let v36: Rc<str> = match v32 {
        US2::US2_None => {
            std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
        }
        US2::US2_Some(v33) => {
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
    let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v49);
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Sprint"); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v50);
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v51);
    v47.borrow_mut().push(v46);
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    v47.borrow_mut().push(v52);
    let v53: Rc<Vec<Rc<str>>> = Rc::new(v47.borrow().clone());
    let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v53).as_ref().clone()));
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = Rc::<str>::from((*v54.borrow()).join(&*v55));
    v56
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method3(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: US1 = v5;
            let v6: Rc<UH0> = v6;
            let v7: Rc<UH1> = method3(v0.clone(), v6, v2);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v0.replace(&*v8, &*v9));
            let v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
            let v12: Rc<str> = Rc::<str>::from(v3.replace(&*v8, &*v9));
            let v13: Rc<str> = Rc::<str>::from(v12.to_lowercase());
            let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint_item_"); } LIT.with(|lit| lit.clone()) };
            v14.borrow_mut().push(v15);
            v14.borrow_mut().push(v11);
            v14.borrow_mut().push(v9);
            v14.borrow_mut().push(v13);
            let v16: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
            let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v18));
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v0));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v3));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            let v24: Rc<str> = format_real_4(v4);
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) }));
            ;
            ;
            let v26: bool = match &v5 {
                US1::US1_Committed => {
                    true
                }
                _ => {
                    false
                }
            };
            let v30: US2 = if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Committed"); } LIT.with(|lit| lit.clone()) };
                US2::US2_Some(v27)
            } else {
                US2::US2_None
            };
            let v44: US2 = match v30 {
                US2::US2_None => {
                    let v33: bool = match v5 {
                        US1::US1_Dropped => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v37: US2 = if v33 {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
                        US2::US2_Some(v34)
                    } else {
                        US2::US2_None
                    };
                    match v37 {
                        US2::US2_None => {
                            US2::US2_None
                        }
                        US2::US2_Some(v38) => {
                            let v38: Rc<str> = v38;
                            US2::US2_Some(v38)
                        }
                    }
                }
                US2::US2_Some(v31) => {
                    let v31: Rc<str> = v31;
                    US2::US2_Some(v31)
                }
            };
            let v48: Rc<str> = match v44 {
                US2::US2_None => {
                    std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
                }
                US2::US2_Some(v45) => {
                    let v45: Rc<str> = v45;
                    v45
                }
            };
            let mut v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v49.borrow_mut().push(v21);
            v49.borrow_mut().push(v23);
            v49.borrow_mut().push(v25);
            v49.borrow_mut().push(v48);
            let v50: Rc<Vec<Rc<str>>> = Rc::new(v49.borrow().clone());
            let v51: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v50).as_ref().clone()));
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = Rc::<str>::from((*v51.borrow()).join(&*v52));
            let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            v54.borrow_mut().push(v55);
            v54.borrow_mut().push(v19);
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            v54.borrow_mut().push(v56);
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintItem"); } LIT.with(|lit| lit.clone()) };
            v54.borrow_mut().push(v57);
            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
            v54.borrow_mut().push(v58);
            v54.borrow_mut().push(v53);
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            v54.borrow_mut().push(v59);
            let mut v60: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
            let v61: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v60).as_ref().clone()));
            let v62: Rc<str> = Rc::<str>::from((*v61.borrow()).join(&*v18));
            Rc::new(UH1::UH1_Cons(v62, v7))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn sprints_appended_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US0, mut v6: Rc<UH0>) -> Rc<str> {
    let v7: bool = &*v0 == "";
    let v13: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion sprint_status_row =\n    | SprintOpen :: sprint_status_row\n    | SprintClosed :: sprint_status_row\n\nunion item_fate_row =\n    | Committed :: item_fate_row\n    | Dropped :: item_fate_row\n\nunion sprint_row =\n    | Sprint :: string * string * string * string * sprint_status_row -> sprint_row\n    | SprintItem :: string * string * i32 * item_fate_row -> sprint_row\n    | SprintReview :: string * i32 * i32 * i32 -> sprint_row\n\n"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v0.ends_with(&*v9);
        if v10 {
            v0
        } else {
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            v11
        }
    };
    let mut v14: Rc<str> = sprint_row_2(v1.clone(), v2, v3, v4, v5, &v6);
    let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH1> = method3(v1, v6, v15);
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v17.borrow_mut().push(v14);
    let v18: i32 = 1i32;
    let v19: i32 = method6(v17.clone(), v16, v18);
    let mut v20: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
    let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v22));
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v23));
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    v25
}
fn items_from_7(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5): (Rc<str>, i32) = v0.borrow()[v1 as usize].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: US1 = US1::US1_Committed;
            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5, v7, v2));
            (v1, v2) = (v6, v8);
            continue;
        }
    }
}
fn sprint_appended_text_0(mut v0: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> Rc<str> {
    let v6: i32 = (v0.clone().borrow().len() as i32);
    let v7: i32 = v6.wrapping_sub(1i32);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = items_from_7(&v0, v7, v8);
    let v10: US0 = US0::US0_SprintOpen;
    sprints_appended_1(v5, v4, v3, v2, v1, v10, v9)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<RefCell<Vec<(Rc<str>, i32)>>>| -> Rc<str> {
        sprint_appended_text_0(v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_item_appended_8(mut v0: bool, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v7: US1 = if v0 {
        US1::US1_Dropped
    } else {
        US1::US1_Committed
    };
    let v8: bool = &*v4 == "";
    let v11: bool = if v8 {
        true
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v4.ends_with(&*v9);
        v10
    };
    let v13: Rc<str> = if v11 {
        v4
    } else {
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        v12
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v3.replace(&*v14, &*v15));
    let mut v17: Rc<str> = Rc::<str>::from(v16.to_lowercase());
    let v18: Rc<str> = Rc::<str>::from(v2.replace(&*v14, &*v15));
    let v19: Rc<str> = Rc::<str>::from(v18.to_lowercase());
    let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint_item_"); } LIT.with(|lit| lit.clone()) };
    v20.borrow_mut().push(v21);
    v20.borrow_mut().push(v17);
    v20.borrow_mut().push(v15);
    v20.borrow_mut().push(v19);
    let v22: Rc<Vec<Rc<str>>> = Rc::new(v20.borrow().clone());
    let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v22).as_ref().clone()));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from((*v23.borrow()).join(&*v24));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v3));
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v2));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v30: Rc<str> = format_real_4(v1);
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) }));
    ;
    ;
    let v32: bool = match &v7 {
        US1::US1_Committed => {
            true
        }
        _ => {
            false
        }
    };
    let v36: US2 = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Committed"); } LIT.with(|lit| lit.clone()) };
        US2::US2_Some(v33)
    } else {
        US2::US2_None
    };
    let v50: US2 = match v36 {
        US2::US2_None => {
            let v39: bool = match v7 {
                US1::US1_Dropped => {
                    true
                }
                _ => {
                    false
                }
            };
            let v43: US2 = if v39 {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
                US2::US2_Some(v40)
            } else {
                US2::US2_None
            };
            match v43 {
                US2::US2_None => {
                    US2::US2_None
                }
                US2::US2_Some(v44) => {
                    let v44: Rc<str> = v44;
                    US2::US2_Some(v44)
                }
            }
        }
        US2::US2_Some(v37) => {
            let v37: Rc<str> = v37;
            US2::US2_Some(v37)
        }
    };
    let mut v54: Rc<str> = match v50 {
        US2::US2_None => {
            std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
        }
        US2::US2_Some(v51) => {
            let v51: Rc<str> = v51;
            v51
        }
    };
    let mut v55: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v55.borrow_mut().push(v27);
    v55.borrow_mut().push(v29);
    v55.borrow_mut().push(v31);
    v55.borrow_mut().push(v54);
    let v56: Rc<Vec<Rc<str>>> = Rc::new(v55.borrow().clone());
    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v56).as_ref().clone()));
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v59: Rc<str> = Rc::<str>::from((*v57.borrow()).join(&*v58));
    let mut v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    v60.borrow_mut().push(v61);
    v60.borrow_mut().push(v25);
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
    v60.borrow_mut().push(v62);
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintItem"); } LIT.with(|lit| lit.clone()) };
    v60.borrow_mut().push(v63);
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    v60.borrow_mut().push(v64);
    v60.borrow_mut().push(v59);
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    v60.borrow_mut().push(v65);
    let v66: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
    let v67: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v66).as_ref().clone()));
    let v68: Rc<str> = Rc::<str>::from((*v67.borrow()).join(&*v24));
    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v68));
    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    v70
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: bool| -> Rc<str> {
        sprint_item_appended_8(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_appended(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), v5)
}
pub fn kit_agile_sprint_item_appended(v0: &str, v1: &str, v2: &str, v3: i32, v4: bool) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
