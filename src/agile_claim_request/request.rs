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
    US2_RequestRead(US1, Rc<str>, Rc<str>, i32, Rc<UH0>),
    US2_RequestRefused(Rc<str>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US4 {
    US4_FlagAny,
    US4_FlagAs,
    US4_FlagTtl,
    US4_FlagPaths,
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
struct Mut1 { l0: i32 }
fn listed_from_1(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v1, v2) = (v4, v6.clone());
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
            let mut v7: bool = "release" == &*v0;
            if v7 {
                let mut v8: US1 = US1::US1_VerbRelease;
                US0::US0_Some(v8)
            } else {
                US0::US0_None
            }
        }
    }
}
fn refused_request_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v1, false, false, v0, v3.clone(), v3, 0i32, v2)
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method7(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
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
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method10(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method10(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16.clone(), v17.clone())
    }
}
fn globs_of_flag_6(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_8(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method7(v2, v3)
}
fn flag_of_11(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = &*v3 == &*v0;
    if v4 {
        let v5: US4 = US4::US4_FlagAny;
        US3::US3_Some(v5)
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = &*v8 == &*v0;
        if v9 {
            let v10: US4 = US4::US4_FlagAs;
            US3::US3_Some(v10)
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ttl"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ttl"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = &*v13 == &*v0;
            if v14 {
                let v15: US4 = US4::US4_FlagTtl;
                US3::US3_Some(v15)
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--paths"); } LIT.with(|lit| lit.clone()) };
                let v19: bool = &*v18 == &*v0;
                if v19 {
                    let v20: US4 = US4::US4_FlagPaths;
                    US3::US3_Some(v20)
                } else {
                    US3::US3_None
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
        let mut v7: bool = v0.contains(&*v6);
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
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v5: bool = if v1 {
                    plain_text_12(v2.clone())
                } else {
                    false
                };
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn paths_refusal_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--paths expects comma-separated globs without spaces, quotes or parentheses, got '"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    v4
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
    let v2: Option<i32> = method16(v1.clone());
    let mut v3: Rc<dyn Fn((i32)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let mut v7: US5 = v6.clone();
    let mut v8: US5 = v4.unwrap_or(v7);
    match &v8 {
        US5::US5_None => {
            US5::US5_None
        }
        US5::US5_Some(v9) => {
            let mut v9: i32 = *v9;
            let v10: bool = v9 > 0i32;
            if v10 {
                US5::US5_Some(v9)
            } else {
                US5::US5_None
            }
        }
    }
}
fn flags_read_5(mut v0: Rc<UH0>, mut v1: US1, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<UH0>) -> US2 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v11, v12) => {
                let v11: Rc<str> = v11.clone();
                let v12: Rc<UH0> = v12.clone();
                let (mut v19, mut v20): (Rc<str>, Rc<UH0>) = match &*v12 {
                    UH0::UH0_Cons(v13, v14) => {
                        let v13: Rc<str> = v13.clone();
                        let v14: Rc<UH0> = v14.clone();
                        (v13.clone(), v14.clone())
                    }
                    UH0::UH0_Nil => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        (v15.clone(), v16.clone())
                    }
                };
                let v21: Rc<UH0> = globs_of_flag_6(v19.clone());
                let v22: US3 = flag_of_11(v11.clone());
                match &v22 {
                    US3::US3_None => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected argument "); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v11));
                        return US2::US2_RequestRefused(v24.clone());
                    }
                    US3::US3_Some(v26) => {
                        let v26: US4 = v26.clone();
                        match &v26 {
                            US4::US4_FlagAny => {
                                v0 = v12.clone();
                                continue;
                            }
                            US4::US4_FlagAs => {
                                let v28: bool = plain_text_12(v19.clone());
                                if v28 {
                                    (v0, v3) = (v20.clone(), v19.clone());
                                    continue;
                                } else {
                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as expects a lane name, got '"); } LIT.with(|lit| lit.clone()) };
                                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v19));
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                                    return US2::US2_RequestRefused(v33.clone());
                                }
                            }
                            US4::US4_FlagPaths => {
                                let v46: bool = true;
                                let v47: bool = method13(v21.clone(), v46);
                                let v48: bool = match &v1 {
                                    US1::US1_VerbClaim => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let v51: bool = if v48 {
                                    let v49: bool = match &*v21 {
                                        UH0::UH0_Nil => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v50: bool = v49 != true;
                                    v50
                                } else {
                                    false
                                };
                                let v52: bool = v51 && v47;
                                if v52 {
                                    (v0, v5) = (v20.clone(), v21.clone());
                                    continue;
                                } else {
                                    let v54: Rc<str> = paths_refusal_14(v19.clone());
                                    return US2::US2_RequestRefused(v54.clone());
                                }
                            }
                            US4::US4_FlagTtl => {
                                let v36: US5 = ttl_read_15(v19.clone());
                                match &v36 {
                                    US5::US5_None => {
                                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ttl expects positive minutes, got '"); } LIT.with(|lit| lit.clone()) };
                                        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v19));
                                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                                        return US2::US2_RequestRefused(v42.clone());
                                    }
                                    US5::US5_Some(v37) => {
                                        let v37: i32 = *v37;
                                        (v0, v4) = (v20.clone(), v37);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v6: bool = &*v3 == "";
                if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> is required"); } LIT.with(|lit| lit.clone()) };
                    return US2::US2_RequestRefused(v7.clone());
                } else {
                    return US2::US2_RequestRead(v1.clone(), v2.clone(), v3.clone(), v4, v5.clone());
                }
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    Rc::new(move |mut v2: Rc<UH0>| -> US5 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US5::US5_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US5> = method17(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US5::US5_None
            }
        }
    })
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US5> {
    closure2(v0, v1)
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
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--any"); } LIT.with(|lit| lit.clone()) };
        let v6: i32 = 0i32;
        let mut v7: Rc<dyn Fn(Rc<UH0>) -> US5> = method17(v5, v6);
        let mut v8: US5 = v7(v1.clone());
        let mut v9: bool = match &v8 {
            US5::US5_None => {
                true
            }
            _ => {
                false
            }
        };
        let v10: bool = v9 != true;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v12: i32 = 90i32;
            let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            flags_read_5(v1.clone(), v0.clone(), v11.clone(), v11, v12, v13)
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next claims only with --any"); } LIT.with(|lit| lit.clone()) };
            US2::US2_RequestRefused(v15)
        }
    } else {
        match &*v1 {
            UH0::UH0_Cons(v18, v19) => {
                let v18: Rc<str> = v18.clone();
                let v19: Rc<UH0> = v19.clone();
                let v20: bool = plain_text_12(v18.clone());
                let v24: bool = if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v22: bool = v18.starts_with(&*v21);
                    let v23: bool = v22 == false;
                    v23
                } else {
                    false
                };
                if v24 {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v26: i32 = 90i32;
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    flags_read_5(v19, v0, v18.clone(), v25, v26, v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID, got '"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v18));
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                    US2::US2_RequestRefused(v32)
                }
            }
            UH0::UH0_Nil => {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID"); } LIT.with(|lit| lit.clone()) };
                US2::US2_RequestRefused(v35)
            }
        }
    }
}
fn method20(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let mut v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method20(v7, v9.clone()) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn read_request_18(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH0>) -> (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) {
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v6: Rc<RefCell<Vec<Rc<str>>>> = method19(v4, v5);
    let mut v7: bool = match &v0 {
        US1::US1_VerbNext => {
            true
        }
        _ => {
            false
        }
    };
    let mut v8: bool = match &v0 {
        US1::US1_VerbRelease => {
            true
        }
        _ => {
            false
        }
    };
    let v12: Rc<str> = match &v0 {
        US1::US1_VerbClaim => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claim"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US1::US1_VerbNext => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US1::US1_VerbRelease => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release"); } LIT.with(|lit| lit.clone()) };
            v11
        }
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v13, v7, v8, v12, v1, v2, v3, v6)
}
fn request_parsed_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = listed_from_1(v0, v3, v4);
    let v6: US0 = verb_of_2(v1.clone());
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match &v6 {
        US0::US0_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("not a claim verb"); } LIT.with(|lit| lit.clone()) };
            refused_request_3(v1.clone(), v7)
        }
        _ => {
            let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match &v6 {
                US0::US0_Some(v16) => {
                    let v16: US1 = v16.clone();
                    let v17: US2 = request_of_4(v16, v5);
                    match &v17 {
                        US2::US2_RequestRefused(v18) => {
                            let v18: Rc<str> = v18.clone();
                            refused_request_3(v1, v18)
                        }
                        _ => {
                            let (mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = match &v17 {
                                US2::US2_RequestRead(v27, v28, v29, v30, v31) => {
                                    let v27: US1 = v27.clone();
                                    let v28: Rc<str> = v28.clone();
                                    let v29: Rc<str> = v29.clone();
                                    let v30: i32 = *v30;
                                    let v31: Rc<UH0> = v31.clone();
                                    read_request_18(v27, v28, v29, v30, v31)
                                }
                                _ => unreachable!(),
                            };
                            (v40, v41, v42, v43, v44, v45, v46, v47)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v56, v57, v58, v59, v60, v61, v62, v63)
        }
    };
    Rc::new(RefCell::new(vec![(v64, v65, v66, v67, v68, v69, v70, v71)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        request_parsed_0(v1.clone(), v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn agile_claim_request_parsed(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure0()(Rc::<str>::from(v0), v1)
}
