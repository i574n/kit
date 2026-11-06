#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_VerbNext,
    US1_VerbClaim,
    US1_VerbRelease,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_RequestRead(US1, Rc<str>, Rc<str>, i32, Rc<UH0>, bool),
    US2_RequestRefused(Rc<str>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US4 {
    US4_FlagAny,
    US4_FlagAs,
    US4_FlagTtl,
    US4_FlagPaths,
    US4_FlagClaim,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(i32),
    US5_None,
}
pub struct Mut1 { pub l0: i32 }
fn listed_from_1(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn verb_of_2(mut v0: Rc<str>) -> US0 {
    let v1: bool = "next" == &*v0;
    if v1 {
        let v2: US1 = US1::US1_VerbNext;
        US0::US0_Some(v2)
    } else {
        let mut v4: bool = "claim" == &*v0;
        if v4 {
            let v5: US1 = US1::US1_VerbClaim;
            US0::US0_Some(v5)
        } else {
            let v7: bool = "release" == &*v0;
            if v7 {
                let mut v8: US1 = US1::US1_VerbRelease;
                US0::US0_Some(v8)
            } else {
                US0::US0_None
            }
        }
    }
}
fn refused_request_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v1, false, false, false, v0, v3.clone(), v3, 0i32, v2)
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method7(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn globs_of_flag_6(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_8(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method7(v2, v3)
}
fn flag_of_11(mut v0: Rc<str>) -> US3 {
    let v1: bool = "--any" == &*v0;
    if v1 {
        let v2: US4 = US4::US4_FlagAny;
        US3::US3_Some(v2)
    } else {
        let mut v4: bool = "--as" == &*v0;
        if v4 {
            let v5: US4 = US4::US4_FlagAs;
            US3::US3_Some(v5)
        } else {
            let v7: bool = "--ttl" == &*v0;
            if v7 {
                let mut v8: US4 = US4::US4_FlagTtl;
                US3::US3_Some(v8)
            } else {
                let v10: bool = "--paths" == &*v0;
                if v10 {
                    let v11: US4 = US4::US4_FlagPaths;
                    US3::US3_Some(v11)
                } else {
                    let v13: bool = "--claim" == &*v0;
                    if v13 {
                        let v14: US4 = US4::US4_FlagClaim;
                        US3::US3_Some(v14)
                    } else {
                        US3::US3_None
                    }
                }
            }
        }
    }
}
fn plain_text_12(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    let v5: bool = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v3: bool = v0.contains(&*v2);
        let mut v4: bool = v3 == false;
        v4
    } else {
        false
    };
    let mut v9: bool = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v0.contains(&*v6);
        let mut v8: bool = v7 == false;
        v8
    } else {
        false
    };
    let v13: bool = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v0.contains(&*v10);
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    let v17: bool = if v13 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v0.contains(&*v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    };
    let v21: bool = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v0.contains(&*v18);
        let v20: bool = v19 == false;
        v20
    } else {
        false
    };
    let v25: bool = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v0.contains(&*v22);
        let v24: bool = v23 == false;
        v24
    } else {
        false
    };
    let v29: bool = if v25 {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v27: bool = v0.contains(&*v26);
        let v28: bool = v27 == false;
        v28
    } else {
        false
    };
    if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v31: bool = v0.contains(&*v30);
        let v32: bool = v31 == false;
        v32
    } else {
        false
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v5: bool = if v1 {
                    plain_text_12(v2)
                } else {
                    false
                };
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn paths_refusal_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--paths expects comma-separated globs without spaces, quotes or parentheses, got '"); } LIT.with(|lit| lit.clone()) }, v0));
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn method16(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US5> = Rc::new(move |mut v0: (i32)| -> US5 {
        let v1: i32 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ttl_read_15(mut v0: Rc<str>) -> US5 {
    let v1: Option<i32> = (v0).trim().parse::<i32>().ok();
    let v2: Option<i32> = method16(v1);
    let mut v3: Rc<dyn Fn((i32)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let v7: US5 = v6.clone();
    let mut v8: US5 = v4.unwrap_or(v7);
    match v8 {
        US5::US5_None => {
            US5::US5_None
        }
        US5::US5_Some(v9) => {
            let mut v9: i32 = v9;
            let v10: bool = v9 > 0i32;
            if v10 {
                US5::US5_Some(v9)
            } else {
                US5::US5_None
            }
        }
    }
}
fn flags_read_5(mut v0: Rc<UH0>, mut v1: US1, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<UH0>, mut v6: bool) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v12, v13) => {
                let v12: Rc<str> = v12;
                let v13: Rc<UH0> = v13;
                let (mut v20, mut v21): (Rc<str>, Rc<UH0>) = match &*v13 {
                    UH0::UH0_Cons(v14, v15) => {
                        let v14: Rc<str> = v14.clone();
                        let v15: Rc<UH0> = v15.clone();
                        (v14, v15)
                    }
                    UH0::UH0_Nil => {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        (v16, v17)
                    }
                };
                let v22: Rc<UH0> = globs_of_flag_6(v20.clone());
                let v23: US3 = flag_of_11(v12.clone());
                match v23 {
                    US3::US3_None => {
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected argument "); } LIT.with(|lit| lit.clone()) }, v12));
                        return US2::US2_RequestRefused(v24);
                    }
                    US3::US3_Some(v26) => {
                        let v26: US4 = v26;
                        match v26 {
                            US4::US4_FlagAny => {
                                v0 = v13;
                                continue;
                            }
                            US4::US4_FlagAs => {
                                let v34: bool = plain_text_12(v20.clone());
                                if v34 {
                                    (v0, v3) = (v21, v20);
                                    continue;
                                } else {
                                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as expects a lane name, got '"); } LIT.with(|lit| lit.clone()) }, v20));
                                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
                                    return US2::US2_RequestRefused(v37);
                                }
                            }
                            US4::US4_FlagClaim => {
                                let v28: bool = match &v1 {
                                    US1::US1_VerbNext => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                if v28 {
                                    let v29: bool = true;
                                    (v0, v6) = (v13, v29);
                                    continue;
                                } else {
                                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--claim belongs to next; claim takes an item ID"); } LIT.with(|lit| lit.clone()) };
                                    return US2::US2_RequestRefused(v31);
                                }
                            }
                            US4::US4_FlagPaths => {
                                let v48: bool = true;
                                let v49: bool = method13(v22.clone(), v48);
                                let v50: bool = match &v1 {
                                    US1::US1_VerbClaim => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let v53: bool = if v50 {
                                    let v51: bool = match &*v22 {
                                        UH0::UH0_Nil => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v52: bool = v51 != true;
                                    v52
                                } else {
                                    false
                                };
                                let v54: bool = v53 && v49;
                                if v54 {
                                    (v0, v5) = (v21, v22);
                                    continue;
                                } else {
                                    let v56: Rc<str> = paths_refusal_14(v20);
                                    return US2::US2_RequestRefused(v56);
                                }
                            }
                            US4::US4_FlagTtl => {
                                let v40: US5 = ttl_read_15(v20.clone());
                                match v40 {
                                    US5::US5_None => {
                                        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ttl expects positive minutes, got '"); } LIT.with(|lit| lit.clone()) }, v20));
                                        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
                                        return US2::US2_RequestRefused(v44);
                                    }
                                    US5::US5_Some(v41) => {
                                        let v41: i32 = v41;
                                        (v0, v4) = (v21, v41);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v7: bool = &*v3 == "";
                if v7 {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> is required"); } LIT.with(|lit| lit.clone()) };
                    return US2::US2_RequestRefused(v8);
                } else {
                    return US2::US2_RequestRead(v1, v2, v3, v4, v5, v6);
                }
            }
        }
    }
}
fn closure2(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    Rc::new(move |mut v1: Rc<UH0>| -> US5 {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == "--any";
                if v5 {
                    US5::US5_Some(v0)
                } else {
                    let v7: i32 = v0.wrapping_add(1i32);
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US5> = method17(v7);
                    v8(v4)
                }
            }
            UH0::UH0_Nil => {
                US5::US5_None
            }
        }
    })
}
fn method17(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    closure2(v0)
}
fn request_of_4(mut v0: US1, mut v1: Rc<UH0>) -> US2 {
    let v2: bool = match &v0 {
        US1::US1_VerbNext => {
            true
        }
        _ => {
            false
        }
    };
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: Rc<dyn Fn(Rc<UH0>) -> US5> = method17(v3);
        let v5: US5 = v4(v1.clone());
        let v6: bool = match v5 {
            US5::US5_None => {
                true
            }
            _ => {
                false
            }
        };
        let v7: bool = v6 != true;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = 90i32;
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v11: bool = false;
            flags_read_5(v1, v0, v8.clone(), v8, v9, v10, v11)
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next claims only with --any"); } LIT.with(|lit| lit.clone()) };
            US2::US2_RequestRefused(v13)
        }
    } else {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v16, v17) => {
                let v16: Rc<str> = v16;
                let v17: Rc<UH0> = v17;
                let v18: bool = plain_text_12(v16.clone());
                let v22: bool = if v18 {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v20: bool = v16.starts_with(&*v19);
                    let v21: bool = v20 == false;
                    v21
                } else {
                    false
                };
                if v22 {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v24: i32 = 90i32;
                    let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v26: bool = false;
                    flags_read_5(v17, v0, v16, v23, v24, v25, v26)
                } else {
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID, got '"); } LIT.with(|lit| lit.clone()) }, v16));
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
                    US2::US2_RequestRefused(v29)
                }
            }
            UH0::UH0_Nil => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID"); } LIT.with(|lit| lit.clone()) };
                US2::US2_RequestRefused(v32)
            }
        }
    }
}
fn method20(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method20(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn read_request_18(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH0>, mut v5: bool) -> (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) {
    let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v7: Rc<RefCell<Vec<Rc<str>>>> = method19(v4, v6);
    let mut v8: bool = match &v0 {
        US1::US1_VerbNext => {
            true
        }
        _ => {
            false
        }
    };
    let mut v9: bool = match &v0 {
        US1::US1_VerbRelease => {
            true
        }
        _ => {
            false
        }
    };
    let v13: Rc<str> = match v0 {
        US1::US1_VerbClaim => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claim"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US1::US1_VerbNext => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US1::US1_VerbRelease => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release"); } LIT.with(|lit| lit.clone()) };
            v12
        }
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v14, v8, v5, v9, v13, v1, v2, v3, v7)
}
fn request_parsed_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = listed_from_1(&v0, v3, v4);
    let v6: US0 = verb_of_2(v1.clone());
    let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match &v6 {
        US0::US0_None => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not a claim verb"); } LIT.with(|lit| lit.clone()) };
            refused_request_3(v1, v7)
        }
        _ => {
            let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match v6 {
                US0::US0_Some(v17) => {
                    let v17: US1 = v17;
                    let v18: US2 = request_of_4(v17, v5);
                    match &v18 {
                        US2::US2_RequestRefused(v19) => {
                            let v19: Rc<str> = v19.clone();
                            refused_request_3(v1, v19)
                        }
                        _ => {
                            let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match v18 {
                                US2::US2_RequestRead(v29, v30, v31, v32, v33, v34) => {
                                    let v29: US1 = v29;
                                    let v30: Rc<str> = v30;
                                    let v31: Rc<str> = v31;
                                    let v32: i32 = v32;
                                    let v33: Rc<UH0> = v33;
                                    let v34: bool = v34;
                                    read_request_18(v29, v30, v31, v32, v33, v34)
                                }
                                _ => unreachable!(),
                            };
                            (v44, v45, v46, v47, v48, v49, v50, v51, v52)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v62, v63, v64, v65, v66, v67, v68, v69, v70)
        }
    };
    Rc::new(RefCell::new(vec![(v71, v72, v73, v74, v75, v76, v77, v78, v79)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        request_parsed_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn agile_claim_request_parsed(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0), v1)
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
