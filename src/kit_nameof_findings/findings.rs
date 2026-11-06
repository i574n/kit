#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Keyword,
    US0_Variable,
    US0_Operator,
    US0_Str,
    US0_Paren,
    US0_OtherClass,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(i32, Rc<str>, US0, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32, Rc<str>, US0),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ToName,
    US2_OfName,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US2, i32, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US2, i32, Rc<str>, Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, i32, i32, Rc<UH3>, Rc<UH2>),
}
pub struct Mut1 { pub l0: i32 }
impl Default for US0 {
    fn default() -> Self {
        US0::US0_Keyword
    }
}
fn method1(mut v0: &Rc<RefCell<Mut0>>) -> bool {
    let v1: i32 = v0.borrow().l0;
    let v2: bool = v1 > 0i32;
    v2
}
fn method2(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: i32 = v2;
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method3(mut v0: &Rc<RefCell<Vec<(i32, Rc<str>, US0)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: i32 = v3;
                let v4: Rc<str> = v4;
                let v5: US0 = v5;
                let mut v6: Rc<UH0> = v6;
                v0.borrow_mut()[v2 as usize] = (v3, v4, v5);
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: US2 = v2;
                let v3: i32 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: i32 = v2;
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn named_10(mut v0: US2, mut v1: i32, mut v2: US3, mut v3: i32, mut v4: Rc<str>, mut v5: US0) -> US4 {
    match v2 {
        US3::US3_None => {
            US4::US4_None
        }
        US3::US3_Some(v6) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v9));
            let v13: Rc<str> = Rc::<str>::from(v4.to_lowercase());
            let v14: Rc<str> = Rc::<str>::from(v13.replace(&*v8, &*v9));
            let mut v15: Rc<str> = Rc::<str>::from(v14.replace(&*v11, &*v9));
            let mut v16: bool = &*v12 == &*v15;
            if v16 {
                US4::US4_Some(v0, v1, v6, v4)
            } else {
                US4::US4_None
            }
        }
    }
}
fn of_name_arm_9(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: US0, mut v4: i32, mut v5: Rc<str>, mut v6: US0, mut v7: i32, mut v8: Rc<str>, mut v9: US0, mut v10: i32, mut v11: Rc<str>, mut v12: US0, mut v13: i32, mut v14: Rc<str>, mut v15: US0, mut v16: i32, mut v17: Rc<str>, mut v18: US0) -> US4 {
    let v20: bool = match v3 {
        US0::US0_Operator => {
            let v19: bool = &*v2 == "|";
            v19
        }
        _ => {
            false
        }
    };
    let v23: bool = if v20 {
        match v15 {
            US0::US0_Operator => {
                let v21: bool = &*v14 == "=>";
                v21
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v25: bool = if v23 {
        match v6 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v27: bool = if v25 {
        match &v9 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v29: bool = if v27 {
        match v12 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    if v29 {
        let v30: US2 = US2::US2_OfName;
        let v39: US3 = match v18 {
            US0::US0_Variable => {
                let v31: u8 = v17.clone().as_bytes()[0i32 as usize];
                let v32: bool = v31 >= b'A';
                let v34: bool = if v32 {
                    let v33: bool = v31 <= b'Z';
                    v33
                } else {
                    false
                };
                if v34 {
                    US3::US3_Some(v17)
                } else {
                    US3::US3_None
                }
            }
            _ => {
                US3::US3_None
            }
        };
        named_10(v30, v0, v39, v7, v8, v9)
    } else {
        US4::US4_None
    }
}
fn to_name_arm_11(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: US0, mut v4: i32, mut v5: Rc<str>, mut v6: US0, mut v7: i32, mut v8: Rc<str>, mut v9: US0, mut v10: i32, mut v11: &Rc<str>, mut v12: US0, mut v13: i32, mut v14: Rc<str>, mut v15: US0, mut v16: i32, mut v17: &Rc<str>, mut v18: US0) -> US4 {
    let v20: bool = match v3 {
        US0::US0_Operator => {
            let v19: bool = &*v2 == "|";
            v19
        }
        _ => {
            false
        }
    };
    let v23: bool = if v20 {
        match v9 {
            US0::US0_Operator => {
                let v21: bool = &*v8 == "=>";
                v21
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v25: bool = if v23 {
        match v12 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v27: bool = if v25 {
        match &v15 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    let v29: bool = if v27 {
        match v18 {
            US0::US0_Str => {
                true
            }
            _ => {
                false
            }
        }
    } else {
        false
    };
    if v29 {
        let v30: US2 = US2::US2_ToName;
        let v39: US3 = match v6 {
            US0::US0_Variable => {
                let v31: u8 = v5.clone().as_bytes()[0i32 as usize];
                let v32: bool = v31 >= b'A';
                let v34: bool = if v32 {
                    let v33: bool = v31 <= b'Z';
                    v33
                } else {
                    false
                };
                if v34 {
                    US3::US3_Some(v5)
                } else {
                    US3::US3_None
                }
            }
            _ => {
                US3::US3_None
            }
        };
        named_10(v30, v0, v39, v13, v14, v15)
    } else {
        US4::US4_None
    }
}
fn arm_of_8(mut v0: i32, mut v1: &Rc<UH0>) -> US4 {
    match &**v1 {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let v2: i32 = *v2;
            let v3: Rc<str> = v3.clone();
            let v4: US0 = v4.clone();
            let v5: Rc<UH0> = v5.clone();
            match Rc::unwrap_or_clone(v5) {
                UH0::UH0_Cons(v6, v7, v8, v9) => {
                    let mut v6: i32 = v6;
                    let v7: Rc<str> = v7;
                    let v8: US0 = v8;
                    let v9: Rc<UH0> = v9;
                    match Rc::unwrap_or_clone(v9) {
                        UH0::UH0_Cons(v10, v11, v12, v13) => {
                            let v10: i32 = v10;
                            let v11: Rc<str> = v11;
                            let v12: US0 = v12;
                            let v13: Rc<UH0> = v13;
                            match Rc::unwrap_or_clone(v13) {
                                UH0::UH0_Cons(v14, v15, v16, v17) => {
                                    let v14: i32 = v14;
                                    let mut v15: Rc<str> = v15;
                                    let mut v16: US0 = v16;
                                    let v17: Rc<UH0> = v17;
                                    match Rc::unwrap_or_clone(v17) {
                                        UH0::UH0_Cons(v18, v19, v20, v21) => {
                                            let v18: i32 = v18;
                                            let v19: Rc<str> = v19;
                                            let v20: US0 = v20;
                                            let v21: Rc<UH0> = v21;
                                            match Rc::unwrap_or_clone(v21) {
                                                UH0::UH0_Cons(v22, v23, v24, v25) => {
                                                    let v22: i32 = v22;
                                                    let v23: Rc<str> = v23;
                                                    let v24: US0 = v24;
                                                    let v25: Rc<UH0> = v25;
                                                    match Rc::unwrap_or_clone(v25) {
                                                        UH0::UH0_Cons(v35, v36, v37, v38) => {
                                                            let v35: i32 = v35;
                                                            let v36: Rc<str> = v36;
                                                            let v37: US0 = v37;
                                                            let v38: Rc<UH0> = v38;
                                                            match Rc::unwrap_or_clone(v38) {
                                                                UH0::UH0_Nil => {
                                                                    let v39: US4 = to_name_arm_11(v0, v2, v3.clone(), v4.clone(), v6, v7.clone(), v8.clone(), v14, v15.clone(), v16.clone(), v18, &v19, v20.clone(), v22, v23, v24, v35, &v36, v37.clone());
                                                                    match v39 {
                                                                        US4::US4_None => {
                                                                            of_name_arm_9(v0, v2, v3, v4, v6, v7, v8, v10, v11, v12, v14, v15, v16, v18, v19, v20, v35, v36, v37)
                                                                        }
                                                                        US4::US4_Some(v40, v41, v42, v43) => {
                                                                            let v40: US2 = v40;
                                                                            let v41: i32 = v41;
                                                                            let v42: Rc<str> = v42;
                                                                            let v43: Rc<str> = v43;
                                                                            US4::US4_Some(v40, v41, v42, v43)
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    US4::US4_None
                                                                }
                                                            }
                                                        }
                                                        UH0::UH0_Nil => {
                                                            let v26: US4 = to_name_arm_11(v0, v2, v3.clone(), v4.clone(), v6, v7.clone(), v8.clone(), v10, v11.clone(), v12.clone(), v14, &v15, v16.clone(), v18, v19.clone(), v20.clone(), v22, &v23, v24.clone());
                                                            match v26 {
                                                                US4::US4_None => {
                                                                    of_name_arm_9(v0, v2, v3, v4, v6, v7, v8, v10, v11, v12, v14, v15, v16, v18, v19, v20, v22, v23, v24)
                                                                }
                                                                US4::US4_Some(v27, v28, v29, v30) => {
                                                                    let v27: US2 = v27;
                                                                    let v28: i32 = v28;
                                                                    let v29: Rc<str> = v29;
                                                                    let v30: Rc<str> = v30;
                                                                    US4::US4_Some(v27, v28, v29, v30)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    US4::US4_None
                                                }
                                            }
                                        }
                                        _ => {
                                            US4::US4_None
                                        }
                                    }
                                }
                                _ => {
                                    US4::US4_None
                                }
                            }
                        }
                        _ => {
                            US4::US4_None
                        }
                    }
                }
                _ => {
                    US4::US4_None
                }
            }
        }
        _ => {
            US4::US4_None
        }
    }
}
fn push_arm_6(mut v0: i32, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = method7(v1, v3);
    let v5: US4 = arm_of_8(v0, &v4);
    match v5 {
        US4::US4_None => {
            v2
        }
        US4::US4_Some(v6, v7, v8, v9) => {
            let mut v6: US2 = v6;
            let v7: i32 = v7;
            let v8: Rc<str> = v8;
            let v9: Rc<str> = v9;
            Rc::new(UH1::UH1_Cons(v6, v7, v8, v9, v2))
        }
    }
}
fn arms_from_4(mut v0: &Rc<RefCell<Vec<(i32, Rc<str>, US0)>>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v5: i32 = (v0.clone().borrow().len() as i32);
        let mut v6: bool = v1 >= v5;
        if v6 {
            let v7: Rc<UH1> = push_arm_6(v2, v3, v4);
            let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method5(v7, v8);
        } else {
            let (mut v10, mut v11, mut v12): (i32, Rc<str>, US0) = v0.borrow()[v1 as usize].clone();
            let v13: bool = v10 == v2;
            if v13 {
                let v14: i32 = v1.wrapping_add(1i32);
                let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v11, v12, v3));
                (v1, v3) = (v14, v15);
                continue;
            } else {
                let v17: i32 = v1.wrapping_add(1i32);
                let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v11, v12, v18));
                let v20: Rc<UH1> = push_arm_6(v2, v3, v4);
                (v1, v2, v3, v4) = (v17, v10, v19, v20);
                continue;
            }
        }
    }
}
fn method13(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6) => {
                let v2: US2 = v2;
                let v3: i32 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: Rc<UH2> = v6;
                let v7: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: US2 = v2;
                let v3: i32 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let v2: US2 = v2;
            let v3: i32 = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: Rc<UH1> = v6;
            let v7: Rc<UH3> = method15(v6, v1);
            Rc::new(UH3::UH3_Cons(v4, v7))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn runs_from_12(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v23, v24, v25, v26, v27) => {
                let v23: US2 = v23;
                let v24: i32 = v24;
                let v25: Rc<str> = v25;
                let v26: Rc<str> = v26;
                let v27: Rc<UH1> = v27;
                match &*v1 {
                    UH1::UH1_Cons(v28, v29, v30, v31, v32) => {
                        let v28: US2 = v28.clone();
                        let v29: i32 = *v29;
                        let v30: Rc<str> = v30.clone();
                        let v31: Rc<str> = v31.clone();
                        let v32: Rc<UH1> = v32.clone();
                        let v36: bool = match v28 {
                            US2::US2_OfName => {
                                match &v23 {
                                    US2::US2_OfName => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }
                            }
                            US2::US2_ToName => {
                                match &v23 {
                                    US2::US2_ToName => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }
                            }
                        };
                        let v39: bool = if v36 {
                            let v37: i32 = v29.wrapping_add(1i32);
                            let v38: bool = v24 == v37;
                            v38
                        } else {
                            false
                        };
                        if v39 {
                            let v40: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v24, v25, v26, v1));
                            (v0, v1) = (v27, v40);
                            continue;
                        } else {
                            let v42: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                            let v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v24, v25, v26, v42));
                            let v44: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                            let v45: Rc<UH1> = method5(v1.clone(), v44);
                            let v61: Rc<UH2> = match Rc::unwrap_or_clone(v45) {
                                UH1::UH1_Cons(v46, v47, v48, v49, v50) => {
                                    let v46: US2 = v46;
                                    let v47: i32 = v47;
                                    let v48: Rc<str> = v48;
                                    let v49: Rc<str> = v49;
                                    let v50: Rc<UH1> = v50;
                                    let v51: i32 = 0i32;
                                    let v52: i32 = method14(v1.clone(), v51);
                                    let v53: bool = v52 >= 3i32;
                                    if v53 {
                                        let v54: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                        let v55: Rc<UH1> = method5(v1, v54);
                                        let v56: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                                        let v57: Rc<UH3> = method15(v55, v56);
                                        Rc::new(UH2::UH2_Cons(v46, v47, v52, v57, v2))
                                    } else {
                                        v2
                                    }
                                }
                                UH1::UH1_Nil => {
                                    v2
                                }
                            };
                            (v0, v1, v2) = (v27, v43, v61);
                            continue;
                        }
                    }
                    UH1::UH1_Nil => {
                        let v64: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v65: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v24, v25, v26, v64));
                        (v0, v1) = (v27, v65);
                        continue;
                    }
                }
            }
            UH1::UH1_Nil => {
                let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v4: Rc<UH1> = method5(v1.clone(), v3);
                let v20: Rc<UH2> = match Rc::unwrap_or_clone(v4) {
                    UH1::UH1_Cons(v5, v6, v7, v8, v9) => {
                        let v5: US2 = v5;
                        let mut v6: i32 = v6;
                        let v7: Rc<str> = v7;
                        let v8: Rc<str> = v8;
                        let v9: Rc<UH1> = v9;
                        let v10: i32 = 0i32;
                        let v11: i32 = method14(v1.clone(), v10);
                        let v12: bool = v11 >= 3i32;
                        if v12 {
                            let v13: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                            let v14: Rc<UH1> = method5(v1, v13);
                            let mut v15: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                            let mut v16: Rc<UH3> = method15(v14, v15);
                            Rc::new(UH2::UH2_Cons(v5, v6, v11, v16, v2))
                        } else {
                            v2
                        }
                    }
                    UH1::UH1_Nil => {
                        v2
                    }
                };
                let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                return method13(v20, v21);
            }
        }
    }
}
fn method18(mut v0: Rc<UH3>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH3> = v3;
                let v4: bool = &*v1 == "";
                let v7: Rc<str> = if v4 {
                    v2
                } else {
                    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
                    v6
                };
                (v0, v1) = (v3, v7);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn row_of_17(mut v0: US2, mut v1: i32, mut v2: i32, mut v3: Rc<UH3>) -> (Rc<str>, i32, i32, Rc<str>) {
    let mut v6: Rc<str> = match v0 {
        US2::US2_OfName => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("of-name"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_ToName => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to-name"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = method18(v3, v7);
    (v6, v1, v2, v8)
}
fn method19(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method16(mut v0: Rc<UH2>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6) => {
                let v2: US2 = v2;
                let v3: i32 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH3> = v5;
                let mut v6: Rc<UH2> = v6;
                let (mut v7, mut v8, mut v9, mut v10): (Rc<str>, i32, i32, Rc<str>) = row_of_17(v2, v3, v4, v5);
                let v11: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v7, v8, v9, v10)]));
                let v12: i32 = (v1.clone().borrow().len() as i32);
                let v13: i32 = (v11.clone().borrow().len() as i32);
                let v14: i32 = v12.wrapping_add(v13);
                let mut v15: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32, i32, Rc<str>)>::default(); v14 as usize]));
                let mut v16: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method19(v14, &v16) {
                    let v18: i32 = v16.borrow().l0;
                    let v19: bool = v18 < v12;
                    let (mut v29, mut v30, mut v31, mut v32): (Rc<str>, i32, i32, Rc<str>) = if v19 {
                        let (mut v20, mut v21, mut v22, mut v23): (Rc<str>, i32, i32, Rc<str>) = v1.borrow()[v18 as usize].clone();
                        (v20, v21, v22, v23)
                    } else {
                        let v24: i32 = v18.wrapping_sub(v12);
                        let (mut v25, mut v26, mut v27, mut v28): (Rc<str>, i32, i32, Rc<str>) = v11.borrow()[v24 as usize].clone();
                        (v25, v26, v27, v28)
                    };
                    v15.borrow_mut()[v18 as usize] = (v29, v30, v31, v32);
                    let v33: i32 = v18.wrapping_add(1i32);
                    v16.borrow_mut().l0 = v33;
                    ()
                };
                (v0, v1) = (v6, v15);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn finding_rows_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v4: i32 = (v3.clone().borrow().len() as i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v4, l1: v5 }));
    while method1(&v6) {
        let v8: i32 = v6.borrow().l0;
        let v9: i32 = v8.wrapping_sub(1i32);
        let v10: Rc<UH0> = v6.borrow().l1.clone();
        let v11: Rc<str> = v3.borrow()[v9 as usize].clone();
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v14: i32 = (v13.clone().borrow().len() as i32);
        let mut v15: bool = v14 < 4i32;
        let v51: US1 = if v15 {
            US1::US1_None
        } else {
            let v17: Rc<str> = v13.borrow()[0i32 as usize].clone();
            let v18: i32 = i32::from_str_radix(&*(v17), 10u32).unwrap();
            let v19: Rc<str> = v13.borrow()[1i32 as usize].clone();
            let v20: i32 = i32::from_str_radix(&*(v19), 10u32).unwrap();
            let v21: Rc<str> = v13.borrow()[2i32 as usize].clone();
            let v22: i32 = i32::from_str_radix(&*(v21), 10u32).unwrap();
            let v23: i32 = (v2.clone().borrow().len() as i32);
            let v24: bool = v18 >= v23;
            if v24 {
                US1::US1_None
            } else {
                let v26: i32 = v22.wrapping_sub(v20);
                let v27: Rc<str> = v2.borrow()[v18 as usize].clone();
                let v28: usize = ((v20) as usize);
                let v29: i32 = v20.wrapping_add(v26);
                let v30: usize = ((v29) as usize);
                let v31: Rc<str> = Rc::<str>::from(&(v27)[v28..v30]);
                let v32: Rc<str> = v13.borrow()[3i32 as usize].clone();
                let v33: bool = "keyword" == &*v32;
                let v48: US0 = if v33 {
                    US0::US0_Keyword
                } else {
                    let v35: bool = "variable" == &*v32;
                    if v35 {
                        US0::US0_Variable
                    } else {
                        let v37: bool = "operator" == &*v32;
                        if v37 {
                            US0::US0_Operator
                        } else {
                            let v39: bool = "string" == &*v32;
                            if v39 {
                                US0::US0_Str
                            } else {
                                let v41: bool = "parenthesis" == &*v32;
                                if v41 {
                                    US0::US0_Paren
                                } else {
                                    US0::US0_OtherClass
                                }
                            }
                        }
                    }
                };
                US1::US1_Some(v18, v31, v48)
            }
        };
        let v57: Rc<UH0> = match v51 {
            US1::US1_None => {
                v10
            }
            US1::US1_Some(v52, v53, v54) => {
                let v52: i32 = v52;
                let v53: Rc<str> = v53;
                let v54: US0 = v54;
                Rc::new(UH0::UH0_Cons(v52, v53, v54, v10))
            }
        };
        v6.borrow_mut().l0 = v9;
        v6.borrow_mut().l1 = v57;
        ()
    };
    let v58: Rc<UH0> = v6.borrow().l1.clone();
    let v59: i32 = 0i32;
    let v60: i32 = method2(v58.clone(), v59);
    let v61: Rc<RefCell<Vec<(i32, Rc<str>, US0)>>> = Rc::new(RefCell::new(vec![<(i32, Rc<str>, US0)>::default(); v60 as usize]));
    let v62: i32 = 0i32;
    let v63: i32 = method3(&v61, v58, v62);
    let v64: i32 = 0i32;
    let v65: i32 = -1i32;
    let v66: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v67: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v68: Rc<UH1> = arms_from_4(&v61, v64, v65, v66, v67);
    let v69: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v70: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v71: Rc<UH2> = runs_from_12(v68, v69, v70);
    let v72: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
    let v73: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> = method16(v71, v72);
    v73
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> {
        finding_rows_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_nameof_finding_rows(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
