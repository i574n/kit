#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, i32, Rc<str>, bool),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_RecSite(Rc<str>, i32, Rc<UH0>, bool),
    US1_PlainSite(Rc<str>, i32, Rc<UH0>, bool),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, Rc<UH0>, bool, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_InCycle,
    US2_NoCycle,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method3(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
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
        method2(v16, v17)
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US1 = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn header_rest_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":& "); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let mut v9: Rc<str> = if v2 {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: i32 = v3.wrapping_sub(3i32);
        let v5: usize = ((3i32) as usize);
        let v6: i32 = 3i32.wrapping_add(v4);
        let v7: usize = ((v6) as usize);
        let mut v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
        v8
    } else {
        v0
    };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
    let mut v11: bool = v9.starts_with(&*v10);
    let v14: bool = if v11 {
        true
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
        let v13: bool = v9.starts_with(&*v12);
        v13
    };
    if v14 {
        let v15: i32 = (v9.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(3i32);
        let v17: usize = ((3i32) as usize);
        let v18: i32 = 3i32.wrapping_add(v16);
        let v19: usize = ((v18) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v9)[v17..v19]);
        v20
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v21
    }
}
fn method9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
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
fn words_of_10(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v2));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v2));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v2));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v2));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v2));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v2));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v2));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v2));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v2));
    let v26: Rc<UH0> = split_list_1(v2, v25);
    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method11(v26, v27)
}
fn site_of_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: bool) -> US1 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^ "); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v0.starts_with(&*v4);
    let v12: Rc<str> = if v5 {
        let v6: i32 = (v0.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(2i32);
        let mut v8: usize = ((2i32) as usize);
        let mut v9: i32 = 2i32.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v0)[v8..v10]);
        v11
    } else {
        v0
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<UH0> = split_list_1(v13.clone(), v12);
    let (mut v26, mut v27): (Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v14) {
        UH0::UH0_Cons(v15, v16) => {
            let v15: Rc<str> = v15;
            let v16: Rc<UH0> = v16;
            let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v18: i32 = 0i32;
            let v19: i32 = method9(v17.clone(), v16, v18);
            let v20: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
            let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
            let v22: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v13));
            (v15, v22)
        }
        UH0::UH0_Nil => {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v23.clone(), v23)
        }
    };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2));
    let v30: Rc<UH0> = words_of_10(v29);
    if v5 {
        US1::US1_RecSite(v26, v1, v30, v3)
    } else {
        US1::US1_PlainSite(v26, v1, v30, v3)
    }
}
fn closed_7(mut v0: US0, mut v1: Rc<UH1>) -> Rc<UH1> {
    match v0 {
        US0::US0_None => {
            v1
        }
        US0::US0_Some(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: Rc<str> = v4;
            let v5: bool = v5;
            let v6: US1 = site_of_8(v2, v3, v4, v5);
            Rc::new(UH1::UH1_Cons(v6, v1))
        }
    }
}
fn sites_from_4(mut v0: Rc<UH0>, mut v1: i32, mut v2: US0, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v7, v8) => {
                let v7: Rc<str> = v7;
                let mut v8: Rc<UH0> = v8;
                let mut v9: Rc<str> = header_rest_6(v7.clone());
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":& "); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v7.starts_with(&*v10);
                let v20: US0 = if v11 {
                    match &v2 {
                        US0::US0_None => {
                            US0::US0_None
                        }
                        US0::US0_Some(v12, v13, v14, v15) => {
                            let v12: Rc<str> = v12.clone();
                            let v13: i32 = *v13;
                            let v14: Rc<str> = v14.clone();
                            let v15: bool = *v15;
                            US0::US0_Some(v12, v13, v14, true)
                        }
                    }
                } else {
                    v2.clone()
                };
                let v21: bool = &*v9 != "";
                if v21 {
                    let v22: i32 = v1.wrapping_add(1i32);
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v24: bool = false;
                    let v25: US0 = US0::US0_Some(v9, v1, v23, v24);
                    let v26: Rc<UH1> = closed_7(v20, v3);
                    (v0, v1, v2, v3) = (v8, v22, v25, v26);
                    continue;
                } else {
                    let v28: bool = &*v7 != "";
                    let v32: bool = if v28 {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v30: bool = v7.starts_with(&*v29);
                        let v31: bool = v30 == false;
                        v31
                    } else {
                        false
                    };
                    if v32 {
                        let v33: i32 = v1.wrapping_add(1i32);
                        let v34: US0 = US0::US0_None;
                        let v35: Rc<UH1> = closed_7(v2, v3);
                        (v0, v1, v2, v3) = (v8, v33, v34, v35);
                        continue;
                    } else {
                        let v46: US0 = match v2 {
                            US0::US0_None => {
                                US0::US0_None
                            }
                            US0::US0_Some(v37, v38, v39, v40) => {
                                let v37: Rc<str> = v37;
                                let v38: i32 = v38;
                                let v39: Rc<str> = v39;
                                let v40: bool = v40;
                                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v7));
                                US0::US0_Some(v37, v38, v42, v40)
                            }
                        };
                        let v47: i32 = v1.wrapping_add(1i32);
                        (v0, v1, v2) = (v8, v47, v46);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH1> = closed_7(v2, v3);
                let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method5(v4, v5);
            }
        }
    }
}
fn site_definition_13(mut v0: US1) -> (Rc<str>, i32, Rc<UH0>, bool) {
    let (mut v13, mut v14, mut v15, mut v16): (Rc<str>, i32, Rc<UH0>, bool) = match &v0 {
        US1::US1_RecSite(v1, v2, v3, v4) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: i32 = *v2;
            let v3: Rc<UH0> = v3.clone();
            let v4: bool = *v4;
            (v1, v2, v3, v4)
        }
        _ => {
            let (mut v9, mut v10, mut v11, mut v12): (Rc<str>, i32, Rc<UH0>, bool) = match v0 {
                US1::US1_PlainSite(v5, v6, v7, v8) => {
                    let v5: Rc<str> = v5;
                    let v6: i32 = v6;
                    let v7: Rc<UH0> = v7;
                    let mut v8: bool = v8;
                    (v5, v6, v7, v8)
                }
                _ => unreachable!(),
            };
            (v9, v10, v11, v12)
        }
    };
    (v13, v14, v15, v16)
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: US1 = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH2> = method12(v3, v1);
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, i32, Rc<UH0>, bool) = site_definition_13(v2);
            Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method17(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: Rc<UH0> = v4;
            let v5: bool = v5;
            let v6: Rc<UH2> = v6;
            let v7: Rc<UH0> = method17(v6, v1);
            Rc::new(UH0::UH0_Cons(v2, v7))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v2: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US3> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure1(v0, v1)
}
fn method18(mut v0: Rc<str>, mut v1: i32, mut v2: &Rc<UH0>, mut v3: bool, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v5) {
        UH0::UH0_Cons(v7, v8) => {
            let v7: Rc<str> = v7;
            let mut v8: Rc<UH0> = v8;
            let mut v9: Rc<UH0> = method18(v0.clone(), v1, v2, v3, v4.clone(), v8, v6);
            let mut v10: i32 = 0i32;
            let mut v11: Rc<dyn Fn(Rc<UH0>) -> US3> = method19(v7.clone(), v10);
            let v12: US3 = v11(v4);
            let v13: bool = match v12 {
                US3::US3_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v14: bool = v13 != true;
            let v16: bool = if v14 {
                let v15: bool = &*v7 != &*v0;
                v15
            } else {
                false
            };
            if v16 {
                Rc::new(UH0::UH0_Cons(v7, v9))
            } else {
                v9
            }
        }
        UH0::UH0_Nil => {
            v6
        }
    }
}
fn edges_of_21(mut v0: Rc<str>, mut v1: Rc<UH2>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = v6;
                let v7: Rc<UH2> = v7;
                let mut v8: bool = &*v3 == &*v0;
                if v8 {
                    return v5;
                } else {
                    v1 = v7;
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method22(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn reaches_20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    return true;
                } else {
                    let v7: i32 = 0i32;
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US3> = method19(v4.clone(), v7);
                    let mut v9: US3 = v8(v2.clone());
                    let mut v10: bool = match v9 {
                        US3::US3_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v11: bool = v10 != true;
                    if v11 {
                        v1 = v5;
                        continue;
                    } else {
                        let v13: Rc<UH0> = edges_of_21(v4.clone(), v3.clone());
                        let v14: Rc<UH0> = method22(v13, v5);
                        let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v2));
                        (v1, v2) = (v14, v15);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn cycle_of_16(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH0>, mut v3: bool, mut v4: Rc<UH2>) -> US2 {
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method17(v4.clone(), v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method18(v0.clone(), v1, &v2, v3, v6, v2.clone(), v7);
    let mut v9: i32 = 0i32;
    let mut v10: Rc<dyn Fn(Rc<UH0>) -> US3> = method19(v0.clone(), v9);
    let mut v11: US3 = v10(v2);
    let v12: bool = match v11 {
        US3::US3_None => {
            true
        }
        _ => {
            false
        }
    };
    let v13: bool = v12 != true;
    let v16: bool = if v13 {
        true
    } else {
        let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        reaches_20(v0, v8, v14, v4)
    };
    if v16 {
        US2::US2_InCycle
    } else {
        US2::US2_NoCycle
    }
}
fn method24(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn finding_of_15(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: US1) -> Rc<str> {
    match v2 {
        US1::US1_PlainSite(v19, v20, v21, v22) => {
            let v19: Rc<str> = v19;
            let v20: i32 = v20;
            let v21: Rc<UH0> = v21;
            let v22: bool = v22;
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23
        }
        US1::US1_RecSite(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: Rc<UH0> = v5;
            let v6: bool = v6;
            if v6 {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7
            } else {
                let mut v8: US2 = cycle_of_16(v3.clone(), v4, v5, v6, v1);
                match v8 {
                    US2::US2_InCycle => {
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v9
                    }
                    US2::US2_NoCycle => {
                        let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                        let mut v11: Rc<str> = format_real_23(v4);
                        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  rec-outside-cycle  ^ on "); } LIT.with(|lit| lit.clone()) }));
                        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v3));
                        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" but it is in no recursion cycle; drop the ^ (it means rec, not join point)\n"); } LIT.with(|lit| lit.clone()) }));
                        v15
                    }
                }
            }
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: Rc<UH1>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5) => {
            let v4: US1 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH0> = method14(v0.clone(), v1.clone(), v5, v3);
            let v7: Rc<str> = finding_of_15(v0, v1, v4);
            Rc::new(UH0::UH0_Cons(v7, v6))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn rec_findings_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v1.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_1(v2, v5);
    let v7: i32 = 1i32;
    let mut v8: US0 = US0::US0_None;
    let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH1> = sites_from_4(v6, v7, v8, v9);
    let mut v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH2> = method12(v10.clone(), v11);
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = method14(v0, v12, v10, v13);
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v16: i32 = 0i32;
    let v17: i32 = method9(v15.clone(), v14, v16);
    let v18: Rc<Vec<Rc<str>>> = Rc::new(v15.borrow().clone());
    let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v18).as_ref().clone()));
    let v20: Rc<str> = Rc::<str>::from((*v19.borrow()).join(&*v4));
    v20
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        rec_findings_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_rec_cycle_findings(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
