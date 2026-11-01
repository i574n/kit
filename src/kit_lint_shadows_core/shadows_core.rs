#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_TypeTwice,
    US0_ShadowedUnused,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, i32, i32, US0, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(i32, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_TypeSpace,
    US1_TermSpace,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, US1, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, i32, US1),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, i32, i32, US0),
    US3_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
struct Mut0 { l0: i32, l1: Rc<UH3> }
#[derive(Clone)]
enum US4 {
    US4_Some(US1, Rc<str>),
    US4_None,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: i32 = *v4;
                let v5: US0 = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3, v4, v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn same_binding_5(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<str>, mut v4: i32, mut v5: US1) -> bool {
    let mut v6: bool = &*v0 == &*v3;
    if v6 {
        let v8: i32 = match &v2 {
            US1::US1_TermSpace => {
                1i32
            }
            US1::US1_TypeSpace => {
                0i32
            }
        };
        let v10: i32 = match &v5 {
            US1::US1_TermSpace => {
                1i32
            }
            US1::US1_TypeSpace => {
                0i32
            }
        };
        let v11: bool = v8 == v10;
        v11
    } else {
        false
    }
}
fn next_same_4(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<UH2>) -> US2 {
    loop {
        match &*v3 {
            UH2::UH2_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: US1 = v7.clone();
                let v8: Rc<UH2> = v8.clone();
                let mut v9: bool = same_binding_5(v0.clone(), v1, v2.clone(), v5.clone(), v6, v7.clone());
                if v9 {
                    return US2::US2_Some(v5.clone(), v6, v7.clone());
                } else {
                    (v0, v1, v2, v3) = (v0.clone(), v1, v2.clone(), v8.clone());
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn word_count_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> i32 {
    loop {
        let v4: Option<usize> = v1.find(&*v0);
        let v5: Option<usize> = method10(v4.clone());
        let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v7: Option<i32> = v5.map(|x| v6(x));
        let mut v9: i32 = v7.unwrap_or(-1i32);
        let v10: bool = v9 < 0i32;
        let v12: bool = if v10 {
            true
        } else {
            let v11: bool = &*v0 == "";
            v11
        };
        if v12 {
            return v3;
        } else {
            let mut v13: bool = v9 == 0i32;
            let v19: Rc<str> = if v13 {
                v2.clone()
            } else {
                let mut v14: i32 = v9.wrapping_sub(1i32);
                let v15: usize = ((v14) as usize);
                let v16: i32 = v14.wrapping_add(1i32);
                let v17: usize = ((v16) as usize);
                let v18: Rc<str> = Rc::<str>::from(&(v1)[v15..v17]);
                v18.clone()
            };
            let v20: i32 = (v0.clone().len() as i32);
            let v21: i32 = v9.wrapping_add(v20);
            let v22: i32 = (v1.clone().len() as i32);
            let v23: bool = v21 >= v22;
            let v29: Rc<str> = if v23 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            } else {
                let v25: usize = ((v21) as usize);
                let v26: i32 = v21.wrapping_add(1i32);
                let v27: usize = ((v26) as usize);
                let v28: Rc<str> = Rc::<str>::from(&(v1)[v25..v27]);
                v28.clone()
            };
            let v30: i32 = (v19.clone().len() as i32);
            let v31: bool = v30 == 1i32;
            let v39: bool = if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
                let v33: Option<usize> = v32.find(&*v19);
                let v34: Option<usize> = method10(v33.clone());
                let mut v35: Option<i32> = v34.map(|x| v6(x));
                let v37: i32 = v35.unwrap_or(-1i32);
                let v38: bool = v37 >= 0i32;
                v38
            } else {
                false
            };
            let v40: bool = v39 == false;
            let v42: bool = if v40 {
                let v41: bool = &*v19 != ".";
                v41
            } else {
                false
            };
            let v54: bool = if v42 {
                let v43: i32 = (v29.clone().len() as i32);
                let v44: bool = v43 == 1i32;
                let v52: bool = if v44 {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
                    let v46: Option<usize> = v45.find(&*v29);
                    let v47: Option<usize> = method10(v46.clone());
                    let mut v48: Option<i32> = v47.map(|x| v6(x));
                    let v50: i32 = v48.unwrap_or(-1i32);
                    let v51: bool = v50 >= 0i32;
                    v51
                } else {
                    false
                };
                let v53: bool = v52 == false;
                v53
            } else {
                false
            };
            let v55: i32 = v21.wrapping_sub(1i32);
            let v56: usize = ((v55) as usize);
            let v57: i32 = v55.wrapping_add(1i32);
            let v58: usize = ((v57) as usize);
            let v59: Rc<str> = Rc::<str>::from(&(v1)[v56..v58]);
            let v60: i32 = v22.wrapping_sub(v21);
            let v61: usize = ((v21) as usize);
            let v62: i32 = v21.wrapping_add(v60);
            let v63: usize = ((v62) as usize);
            let v64: Rc<str> = Rc::<str>::from(&(v1)[v61..v63]);
            let v66: i32 = if v54 {
                let v65: i32 = v3.wrapping_add(1i32);
                v65
            } else {
                v3
            };
            (v0, v1, v2, v3) = (v0.clone(), v64.clone(), v59.clone(), v66);
            continue;
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<str>, mut v4: i32, mut v5: US1, mut v6: Rc<UH1>, mut v7: i32) -> i32 {
    loop {
        match &*v6 {
            UH1::UH1_Cons(v8, v9, v10) => {
                let v8: i32 = *v8;
                let mut v9: Rc<str> = v9.clone();
                let v10: Rc<UH1> = v10.clone();
                let v11: bool = v8 > v1;
                let mut v13: bool = if v11 {
                    let v12: bool = v8 <= v4;
                    v12
                } else {
                    false
                };
                let v16: i32 = if v13 {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    word_count_9(v0.clone(), v9.clone(), v14.clone(), v7)
                } else {
                    v7
                };
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1, v2.clone(), v3.clone(), v4, v5.clone(), v10.clone(), v16);
                continue;
            }
            UH1::UH1_Nil => {
                return v7;
            }
        }
    }
}
fn uses_between_7(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: i32, mut v6: US1) -> i32 {
    let mut v7: i32 = 0i32;
    let v8: i32 = method8(v1, v2, v3, v4, v5, v6, v0, v7);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    v9
}
fn shadow_of_6(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: i32, mut v6: US1) -> US3 {
    match &v3 {
        US1::US1_TermSpace => {
            let mut v9: i32 = uses_between_7(v0, v1.clone(), v2, v3, v4, v5, v6);
            let v10: bool = v9 == 0i32;
            if v10 {
                let v11: US0 = US0::US0_ShadowedUnused;
                US3::US3_Some(v1.clone(), v2, v5, v11)
            } else {
                US3::US3_None
            }
        }
        US1::US1_TypeSpace => {
            let mut v7: US0 = US0::US0_TypeTwice;
            US3::US3_Some(v1, v2, v5, v7)
        }
    }
}
fn shadows_in_2(mut v0: Rc<UH1>, mut v1: Rc<UH2>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: US1 = v7.clone();
                let v8: Rc<UH2> = v8.clone();
                let mut v9: US2 = next_same_4(v5.clone(), v6, v7.clone(), v8.clone());
                let v16: US3 = match &v9 {
                    US2::US2_None => {
                        US3::US3_None
                    }
                    US2::US2_Some(v10, v11, v12) => {
                        let v10: Rc<str> = v10.clone();
                        let v11: i32 = *v11;
                        let v12: US1 = v12.clone();
                        shadow_of_6(v0.clone(), v5.clone(), v6, v7.clone(), v10.clone(), v11, v12.clone())
                    }
                };
                match &v16 {
                    US3::US3_None => {
                        (v0, v1, v2) = (v0.clone(), v8.clone(), v2.clone());
                        continue;
                    }
                    US3::US3_Some(v17, v18, v19, v20) => {
                        let v17: Rc<str> = v17.clone();
                        let v18: i32 = *v18;
                        let v19: i32 = *v19;
                        let v20: US0 = v20.clone();
                        let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v18, v19, v20.clone(), v2.clone()));
                        (v0, v1, v2) = (v0.clone(), v8.clone(), v21.clone());
                        continue;
                    }
                }
            }
            UH2::UH2_Nil => {
                let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method3(v2.clone(), v3.clone());
            }
        }
    }
}
fn method12(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH3> = v3.clone();
                let v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH3> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method13(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH3> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.clone().borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH3> = v9.borrow().l1.clone();
        let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method12(v16.clone(), v17.clone())
    }
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method16(mut v0: Rc<UH3>, mut v1: i32, mut v2: Rc<UH1>) -> (i32, Rc<UH1>) {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH3> = v4.clone();
                let v5: i32 = v1.wrapping_add(1i32);
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v3.clone(), v2.clone()));
                (v0, v1, v2) = (v4.clone(), v5, v6.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return (v1, v2.clone());
            }
        }
    }
}
fn numbered_14(mut v0: Rc<UH3>) -> Rc<UH1> {
    let v1: i32 = 0i32;
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v3, mut v4): (i32, Rc<UH1>) = method16(v0, v1, v2);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method15(v4, v5)
}
fn method18(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let mut v6: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2.clone(), v3, v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn ident_length_20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 < v2;
        let v19: bool = if v3 {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let mut v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let v8: i32 = (v7.clone().len() as i32);
            let mut v9: bool = v8 == 1i32;
            if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
                let v11: Option<usize> = v10.find(&*v7);
                let v12: Option<usize> = method10(v11.clone());
                let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v14: Option<i32> = v12.map(|x| v13(x));
                let v16: i32 = v14.unwrap_or(-1i32);
                let v17: bool = v16 >= 0i32;
                v17
            } else {
                false
            }
        } else {
            false
        };
        if v19 {
            let v20: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v20);
            continue;
        } else {
            return v1;
        }
    }
}
fn method19(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v60: US4 = if v6 {
                    let mut v7: i32 = (v3.clone().len() as i32);
                    let v8: i32 = v7.wrapping_sub(4i32);
                    let mut v9: usize = ((4i32) as usize);
                    let v10: i32 = 4i32.wrapping_add(v8);
                    let v11: usize = ((v10) as usize);
                    let v12: Rc<str> = Rc::<str>::from(&(v3)[v9..v11]);
                    let mut v13: US1 = US1::US1_TypeSpace;
                    US4::US4_Some(v13.clone(), v12.clone())
                } else {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":! "); } LIT.with(|lit| lit.clone()) };
                    let v16: bool = v3.starts_with(&*v15);
                    if v16 {
                        let v17: i32 = (v3.clone().len() as i32);
                        let v18: i32 = v17.wrapping_sub(3i32);
                        let v19: usize = ((3i32) as usize);
                        let v20: i32 = 3i32.wrapping_add(v18);
                        let v21: usize = ((v20) as usize);
                        let v22: Rc<str> = Rc::<str>::from(&(v3)[v19..v21]);
                        let v23: US1 = US1::US1_TypeSpace;
                        US4::US4_Some(v23.clone(), v22.clone())
                    } else {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v26: bool = v3.starts_with(&*v25);
                        if v26 {
                            let v27: i32 = (v3.clone().len() as i32);
                            let v28: i32 = v27.wrapping_sub(3i32);
                            let v29: usize = ((3i32) as usize);
                            let v30: i32 = 3i32.wrapping_add(v28);
                            let v31: usize = ((v30) as usize);
                            let v32: Rc<str> = Rc::<str>::from(&(v3)[v29..v31]);
                            let v33: US1 = US1::US1_TypeSpace;
                            US4::US4_Some(v33.clone(), v32.clone())
                        } else {
                            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            let v36: bool = v3.starts_with(&*v35);
                            if v36 {
                                let v37: i32 = (v3.clone().len() as i32);
                                let v38: i32 = v37.wrapping_sub(3i32);
                                let v39: usize = ((3i32) as usize);
                                let v40: i32 = 3i32.wrapping_add(v38);
                                let v41: usize = ((v40) as usize);
                                let v42: Rc<str> = Rc::<str>::from(&(v3)[v39..v41]);
                                let v43: US1 = US1::US1_TermSpace;
                                US4::US4_Some(v43.clone(), v42.clone())
                            } else {
                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
                                let v46: bool = v3.starts_with(&*v45);
                                if v46 {
                                    let v47: i32 = (v3.clone().len() as i32);
                                    let mut v48: i32 = v47.wrapping_sub(3i32);
                                    let v49: usize = ((3i32) as usize);
                                    let v50: i32 = 3i32.wrapping_add(v48);
                                    let v51: usize = ((v50) as usize);
                                    let v52: Rc<str> = Rc::<str>::from(&(v3)[v49..v51]);
                                    let v53: US1 = US1::US1_TermSpace;
                                    US4::US4_Some(v53.clone(), v52.clone())
                                } else {
                                    US4::US4_None
                                }
                            }
                        }
                    }
                };
                let v83: US2 = match &v60 {
                    US4::US4_None => {
                        US2::US2_None
                    }
                    US4::US4_Some(v61, v62) => {
                        let v61: US1 = v61.clone();
                        let v62: Rc<str> = v62.clone();
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^ "); } LIT.with(|lit| lit.clone()) };
                        let v64: bool = v62.starts_with(&*v63);
                        let v71: Rc<str> = if v64 {
                            let v65: i32 = (v62.clone().len() as i32);
                            let v66: i32 = v65.wrapping_sub(2i32);
                            let v67: usize = ((2i32) as usize);
                            let v68: i32 = 2i32.wrapping_add(v66);
                            let v69: usize = ((v68) as usize);
                            let v70: Rc<str> = Rc::<str>::from(&(v62)[v67..v69]);
                            v70.clone()
                        } else {
                            v62.clone()
                        };
                        let v72: i32 = 0i32;
                        let v73: i32 = ident_length_20(v71.clone(), v72);
                        let v74: usize = ((0i32) as usize);
                        let v75: usize = ((v73) as usize);
                        let v76: Rc<str> = Rc::<str>::from(&(v71)[v74..v75]);
                        let v77: bool = &*v76 == "";
                        if v77 {
                            US2::US2_None
                        } else {
                            US2::US2_Some(v76.clone(), v2, v61.clone())
                        }
                    }
                };
                let v89: Rc<UH2> = match &v83 {
                    US2::US2_None => {
                        v1.clone()
                    }
                    US2::US2_Some(v84, v85, v86) => {
                        let v84: Rc<str> = v84.clone();
                        let v85: i32 = *v85;
                        let v86: US1 = v86.clone();
                        Rc::new(UH2::UH2_Cons(v84.clone(), v85, v86.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v4.clone(), v89.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn definitions_17(mut v0: Rc<UH1>) -> Rc<UH2> {
    let v1: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH2> = method19(v0, v1);
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method18(v2, v3)
}
fn file_shadows_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH3> = split_list_11(v1, v4);
    let mut v6: Rc<UH1> = numbered_14(v5);
    let mut v7: Rc<UH2> = definitions_17(v6.clone());
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    shadows_in_2(v6, v7, v8)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i32, i32, US0) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0| -> Rc<str> {
        v0.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: i32 = *v4;
                let v5: US0 = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6.clone(), v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method23(mut v0: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> Rc<str>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6, v7, v8) => {
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let mut v6: i32 = *v6;
                let mut v7: US0 = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let mut v9: Rc<str> = v0(v4.clone(), v5, v6, v7.clone());
                v1.clone().borrow_mut()[v3 as usize] = v9.clone();
                let v10: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v8.clone(), v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_21(mut v0: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> Rc<str>>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: i32 = 0i32;
    let v3: i32 = method22(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method23(v0, v4.clone(), v1, v5);
    v4
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0| -> i32 {
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6, v7, v8) => {
                let v4: Rc<str> = v4.clone();
                let v5: i32 = *v5;
                let mut v6: i32 = *v6;
                let mut v7: US0 = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let mut v9: i32 = v0(v4.clone(), v5, v6, v7.clone());
                v1.clone().borrow_mut()[v3 as usize] = v9;
                let v10: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v8.clone(), v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_24(mut v0: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<i32>>> {
    let v2: i32 = 0i32;
    let v3: i32 = method22(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method25(v0, v4.clone(), v1, v5);
    v4
}
fn closure4() -> Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0| -> i32 {
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0| -> i32 {
        match &v3 {
            US0::US0_ShadowedUnused => {
                1i32
            }
            US0::US0_TypeTwice => {
                0i32
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn scanned_0(mut v0: Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let v1: Rc<UH0> = file_shadows_1(v0);
    let v2: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> Rc<str>> = closure2();
    let v3: Rc<RefCell<Vec<Rc<str>>>> = filled_21(v2, v1.clone());
    let v4: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = closure3();
    let v5: Rc<RefCell<Vec<i32>>> = filled_24(v4, v1.clone());
    let mut v6: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = closure4();
    let mut v7: Rc<RefCell<Vec<i32>>> = filled_24(v6, v1.clone());
    let v8: Rc<dyn Fn(Rc<str>, i32, i32, US0) -> i32> = closure5();
    let mut v9: Rc<RefCell<Vec<i32>>> = filled_24(v8, v1);
    (v3, v5, v7, v9)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
        scanned_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_shadows_scan(v0: &str) -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    closure0()(Rc::<str>::from(v0))
}
