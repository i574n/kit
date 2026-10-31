#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_ExceptionBoundary,
    US1_UnionBridge,
    US1_UnionBridgeHeader,
    US1_PlatformBoundary,
    US1_TypeDefinition,
    US1_SequenceComprehension,
    US1_CPointerArrow,
    US1_HostTypeText,
    US1_ZigCapture,
    US1_Comment,
    US1_LibraryBinding,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US1, Rc<UH0>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US3 {
    US3_WordLetters,
    US3_Blanks,
    US3_AnyLetter,
    US3_NoneOf(Rc<str>),
    US3_AnyOf(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Text(Rc<str>),
    US2_Texts(Rc<UH3>),
    US2_Words(Rc<UH3>),
    US2_Run(US3, i32),
    US2_Maybe(Rc<str>),
    US2_Edge,
    US2_Start,
    US2_End,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, Rc<UH2>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH2>, Rc<UH1>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<UH2>),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
fn method4(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match &*v0 {
        UH3::UH3_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH3> = v3.clone();
            let v4: Rc<UH3> = method4(v3, v1.clone());
            let v5: Rc<str> = Rc::<str>::from(v2.to_lowercase());
            Rc::new(UH3::UH3_Cons(v5, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method3(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH2::UH2_Cons(v2, v3) => {
            let v2: US2 = v2.clone();
            let v3: Rc<UH2> = v3.clone();
            let v4: Rc<UH2> = method3(v3, v1.clone());
            let v22: US2 = match &v2 {
                US2::US2_Maybe(v16) => {
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = Rc::<str>::from(v16.to_lowercase());
                    US2::US2_Maybe(v17)
                }
                US2::US2_Text(v5) => {
                    let v5: Rc<str> = v5.clone();
                    let v6: Rc<str> = Rc::<str>::from(v5.to_lowercase());
                    US2::US2_Text(v6)
                }
                US2::US2_Texts(v8) => {
                    let v8: Rc<UH3> = v8.clone();
                    let v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                    let v10: Rc<UH3> = method4(v8, v9);
                    US2::US2_Texts(v10)
                }
                US2::US2_Words(v12) => {
                    let v12: Rc<UH3> = v12.clone();
                    let v13: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                    let v14: Rc<UH3> = method4(v12, v13);
                    US2::US2_Words(v14)
                }
                _ => {
                    v2
                }
            };
            Rc::new(UH2::UH2_Cons(v22, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method2(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH2> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH1> = method2(v3, v1.clone());
            let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH2> = method3(v2, v5);
            Rc::new(UH1::UH1_Cons(v6, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn code_at_10(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> i32 {
    let v2: bool = v1 < 0i32;
    let v5: bool = if v2 {
        true
    } else {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        v4
    };
    if v5 {
        -1i32
    } else {
        let v6: u8 = v0.borrow()[v1 as usize].clone();
        let v7: i32 = (v6 as i32);
        v7
    }
}
fn word_code_11(mut v0: i32) -> bool {
    let v1: bool = v0 >= 97i32;
    let v3: bool = if v1 {
        let v2: bool = v0 <= 122i32;
        v2
    } else {
        false
    };
    let v7: bool = if v3 {
        true
    } else {
        let v4: bool = v0 >= 65i32;
        if v4 {
            let v5: bool = v0 <= 90i32;
            v5
        } else {
            false
        }
    };
    let v11: bool = if v7 {
        true
    } else {
        let v8: bool = v0 >= 48i32;
        if v8 {
            let v9: bool = v0 <= 57i32;
            v9
        } else {
            false
        }
    };
    if v11 {
        true
    } else {
        let v12: bool = v0 == 95i32;
        v12
    }
}
fn edge_at_9(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32) -> bool {
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: i32 = code_at_10(v0.clone(), v2);
    let v4: bool = word_code_11(v3);
    let v5: i32 = code_at_10(v0, v1);
    let v6: bool = word_code_11(v5);
    let v7: bool = v4 != v6;
    v7
}
fn bytes_from_13(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<RefCell<Vec<u8>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v3 >= v4;
        if v5 {
            return true;
        } else {
            let v6: i32 = v2.wrapping_add(v3);
            let v7: i32 = code_at_10(v0.clone(), v6);
            let v8: i32 = code_at_10(v1.clone(), v3);
            let v9: bool = v7 == v8;
            if v9 {
                let v10: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v10);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn text_at_12(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> bool {
    let v3: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v2.as_bytes().to_vec()));
    let v4: i32 = 0i32;
    bytes_from_13(v0, v3, v1, v4)
}
fn code_in_16(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 < v3;
        if v4 {
            let v5: i32 = code_at_10(v0.clone(), v2);
            let v6: bool = v5 == v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v7);
                continue;
            }
        } else {
            return false;
        }
    }
}
fn fits_15(mut v0: US3, mut v1: i32) -> bool {
    match &v0 {
        US3::US3_AnyLetter => {
            true
        }
        US3::US3_AnyOf(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v11.as_bytes().to_vec()));
            let v13: i32 = 0i32;
            code_in_16(v12, v1, v13)
        }
        US3::US3_Blanks => {
            let v3: bool = v1 == 32i32;
            if v3 {
                true
            } else {
                let v4: bool = v1 == 9i32;
                v4
            }
        }
        US3::US3_NoneOf(v6) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v6.as_bytes().to_vec()));
            let v8: i32 = 0i32;
            let v9: bool = code_in_16(v7, v1, v8);
            let v10: bool = v9 == false;
            v10
        }
        US3::US3_WordLetters => {
            word_code_11(v1)
        }
    }
}
fn run_length_14(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: US3, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = v2.wrapping_add(v3);
        let v5: i32 = (v0.clone().borrow().len() as i32);
        let v6: bool = v4 < v5;
        let v9: bool = if v6 {
            let v7: i32 = code_at_10(v0.clone(), v4);
            fits_15(v1.clone(), v7)
        } else {
            false
        };
        if v9 {
            let v10: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v10);
            continue;
        } else {
            return v3;
        }
    }
}
fn attempt_17(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let v5: bool = v4 < v3;
        if v5 {
            return false;
        } else {
            let v6: i32 = v1.wrapping_add(v4);
            let v7: bool = matches_at_8(v0.clone(), v2.clone(), v6);
            if v7 {
                return true;
            } else {
                let v8: i32 = v4.wrapping_sub(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2.clone(), v3, v8);
                continue;
            }
        }
    }
}
fn method18(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> US5 {
    loop {
        match &*v3 {
            UH3::UH3_Cons(v5, v6) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<UH3> = v6.clone();
                let v7: US2 = US2::US2_Text(v5.clone());
                let v8: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7.clone(), v2.clone()));
                let v9: bool = matches_at_8(v0.clone(), v8.clone(), v1);
                if v9 {
                    return US5::US5_Some(v5.clone());
                } else {
                    (v0, v1, v2, v3) = (v0.clone(), v1, v2.clone(), v6.clone());
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US5::US5_None;
            }
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> US5 {
    loop {
        match &*v3 {
            UH3::UH3_Cons(v5, v6) => {
                let v5: Rc<str> = v5.clone();
                let v6: Rc<UH3> = v6.clone();
                let v7: US2 = US2::US2_Edge;
                let v8: US2 = US2::US2_Text(v5.clone());
                let v9: US2 = US2::US2_Edge;
                let v10: Rc<UH2> = Rc::new(UH2::UH2_Cons(v9.clone(), v2.clone()));
                let v11: Rc<UH2> = Rc::new(UH2::UH2_Cons(v8.clone(), v10.clone()));
                let v12: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7.clone(), v11.clone()));
                let v13: bool = matches_at_8(v0.clone(), v12.clone(), v1);
                if v13 {
                    return US5::US5_Some(v5.clone());
                } else {
                    (v0, v1, v2, v3) = (v0.clone(), v1, v2.clone(), v6.clone());
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US5::US5_None;
            }
        }
    }
}
fn matches_at_8(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH2>, mut v2: i32) -> bool {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let v3: US2 = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                match &v3 {
                    US2::US2_Edge => {
                        let v27: bool = edge_at_9(v0.clone(), v2);
                        if v27 {
                            (v0, v1, v2) = (v0.clone(), v4.clone(), v2);
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_End => {
                        let v33: i32 = (v0.clone().borrow().len() as i32);
                        let v34: bool = v2 == v33;
                        if v34 {
                            (v0, v1, v2) = (v0.clone(), v4.clone(), v2);
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Maybe(v19) => {
                        let v19: Rc<str> = v19.clone();
                        let v20: bool = text_at_12(v0.clone(), v2, v19.clone());
                        let v24: bool = if v20 {
                            let v21: i32 = (v19.clone().len() as i32);
                            let v22: i32 = v2.wrapping_add(v21);
                            matches_at_8(v0.clone(), v4.clone(), v22)
                        } else {
                            false
                        };
                        if v24 {
                            return true;
                        } else {
                            (v0, v1, v2) = (v0.clone(), v4.clone(), v2);
                            continue;
                        }
                    }
                    US2::US2_Run(v37, v38) => {
                        let v37: US3 = v37.clone();
                        let v38: i32 = *v38;
                        let v39: i32 = 0i32;
                        let v40: i32 = run_length_14(v0.clone(), v37.clone(), v2, v39);
                        return attempt_17(v0.clone(), v2, v4.clone(), v38, v40);
                    }
                    US2::US2_Start => {
                        let v30: bool = v2 == 0i32;
                        if v30 {
                            (v0, v1, v2) = (v0.clone(), v4.clone(), v2);
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Text(v5) => {
                        let v5: Rc<str> = v5.clone();
                        let v6: bool = text_at_12(v0.clone(), v2, v5.clone());
                        if v6 {
                            let v7: i32 = (v5.clone().len() as i32);
                            let v8: i32 = v2.wrapping_add(v7);
                            (v0, v1, v2) = (v0.clone(), v4.clone(), v8);
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Texts(v11) => {
                        let v11: Rc<UH3> = v11.clone();
                        let v12: US5 = method18(v0.clone(), v2, v4.clone(), v11.clone());
                        let v13: bool = match &v12 {
                            US5::US5_None => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v14: bool = v13 != true;
                        return v14;
                    }
                    US2::US2_Words(v15) => {
                        let v15: Rc<UH3> = v15.clone();
                        let v16: US5 = method19(v0.clone(), v2, v4.clone(), v15.clone());
                        let v17: bool = match &v16 {
                            US5::US5_None => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v18: bool = v17 != true;
                        return v18;
                    }
                }
            }
            UH2::UH2_Nil => {
                return true;
            }
        }
    }
}
fn found_from_7(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH2>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 > v3;
        if v4 {
            return false;
        } else {
            let v5: bool = matches_at_8(v0.clone(), v1.clone(), v2);
            if v5 {
                return true;
            } else {
                let v6: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v6);
                continue;
            }
        }
    }
}
fn method6(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH1>) -> US4 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<UH2> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: i32 = 0i32;
                let v6: bool = found_from_7(v0.clone(), v3.clone(), v5);
                if v6 {
                    return US4::US4_Some(v3.clone());
                } else {
                    (v0, v1) = (v0.clone(), v4.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn found_5(mut v0: Rc<str>, mut v1: Rc<UH1>) -> bool {
    let v2: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v0.as_bytes().to_vec()));
    let v3: US4 = method6(v2, v1);
    let v4: bool = match &v3 {
        US4::US4_None => {
            true
        }
        _ => {
            false
        }
    };
    let v5: bool = v4 != true;
    v5
}
fn method1(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: US1 = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v278: Rc<UH1> = match &v3 {
                    US1::US1_CPointerArrow => {
                        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")->"); } LIT.with(|lit| lit.clone()) };
                        let v179: US2 = US2::US2_Text(v178.clone());
                        let v180: US3 = US3::US3_WordLetters;
                        let v181: US2 = US2::US2_Run(v180.clone(), 1i32);
                        let v182: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v183: Rc<UH2> = Rc::new(UH2::UH2_Cons(v181.clone(), v182.clone()));
                        let v184: Rc<UH2> = Rc::new(UH2::UH2_Cons(v179.clone(), v183.clone()));
                        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->ptr"); } LIT.with(|lit| lit.clone()) };
                        let v186: US2 = US2::US2_Text(v185.clone());
                        let v187: US2 = US2::US2_Edge;
                        let v188: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v189: Rc<UH2> = Rc::new(UH2::UH2_Cons(v187.clone(), v188.clone()));
                        let v190: Rc<UH2> = Rc::new(UH2::UH2_Cons(v186.clone(), v189.clone()));
                        let v191: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v192: Rc<UH1> = Rc::new(UH1::UH1_Cons(v190.clone(), v191.clone()));
                        Rc::new(UH1::UH1_Cons(v184.clone(), v192.clone()))
                    }
                    US1::US1_Comment => {
                        let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v243: US2 = US2::US2_Text(v242.clone());
                        let v244: US3 = US3::US3_Blanks;
                        let v245: US2 = US2::US2_Run(v244.clone(), 0i32);
                        let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                        let v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                        let v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("##"); } LIT.with(|lit| lit.clone()) };
                        let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/*"); } LIT.with(|lit| lit.clone()) };
                        let v250: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v251: Rc<UH3> = Rc::new(UH3::UH3_Cons(v249.clone(), v250.clone()));
                        let v252: Rc<UH3> = Rc::new(UH3::UH3_Cons(v248.clone(), v251.clone()));
                        let v253: Rc<UH3> = Rc::new(UH3::UH3_Cons(v247.clone(), v252.clone()));
                        let v254: Rc<UH3> = Rc::new(UH3::UH3_Cons(v246.clone(), v253.clone()));
                        let v255: US2 = US2::US2_Texts(v254.clone());
                        let v256: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v257: Rc<UH2> = Rc::new(UH2::UH2_Cons(v255.clone(), v256.clone()));
                        let v258: Rc<UH2> = Rc::new(UH2::UH2_Cons(v245.clone(), v257.clone()));
                        let v259: Rc<UH2> = Rc::new(UH2::UH2_Cons(v243.clone(), v258.clone()));
                        let v260: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v259.clone(), v260.clone()))
                    }
                    US1::US1_ExceptionBoundary => {
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("try"); } LIT.with(|lit| lit.clone()) };
                        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("with ex"); } LIT.with(|lit| lit.clone()) };
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catch"); } LIT.with(|lit| lit.clone()) };
                        let v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v10: Rc<UH3> = Rc::new(UH3::UH3_Cons(v8.clone(), v9.clone()));
                        let v11: Rc<UH3> = Rc::new(UH3::UH3_Cons(v7.clone(), v10.clone()));
                        let v12: Rc<UH3> = Rc::new(UH3::UH3_Cons(v6.clone(), v11.clone()));
                        let v13: Rc<UH3> = Rc::new(UH3::UH3_Cons(v5.clone(), v12.clone()));
                        let v14: US2 = US2::US2_Words(v13.clone());
                        let v15: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14.clone(), v15.clone()));
                        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v16.clone(), v17.clone()))
                    }
                    US1::US1_HostTypeText => {
                        let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Fn"); } LIT.with(|lit| lit.clone()) };
                        let v195: US2 = US2::US2_Text(v194.clone());
                        let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Once"); } LIT.with(|lit| lit.clone()) };
                        let v197: US2 = US2::US2_Maybe(v196.clone());
                        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() ->"); } LIT.with(|lit| lit.clone()) };
                        let v199: US2 = US2::US2_Text(v198.clone());
                        let v200: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v201: Rc<UH2> = Rc::new(UH2::UH2_Cons(v199.clone(), v200.clone()));
                        let v202: Rc<UH2> = Rc::new(UH2::UH2_Cons(v197.clone(), v201.clone()));
                        let v203: Rc<UH2> = Rc::new(UH2::UH2_Cons(v195.clone(), v202.clone()));
                        let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() => void"); } LIT.with(|lit| lit.clone()) };
                        let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("regex::Match"); } LIT.with(|lit| lit.clone()) };
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leptos::prelude::"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v208: Rc<UH3> = Rc::new(UH3::UH3_Cons(v206.clone(), v207.clone()));
                        let v209: Rc<UH3> = Rc::new(UH3::UH3_Cons(v205.clone(), v208.clone()));
                        let v210: Rc<UH3> = Rc::new(UH3::UH3_Cons(v204.clone(), v209.clone()));
                        let v211: US2 = US2::US2_Texts(v210.clone());
                        let v212: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v213: Rc<UH2> = Rc::new(UH2::UH2_Cons(v211.clone(), v212.clone()));
                        let v214: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v215: Rc<UH1> = Rc::new(UH1::UH1_Cons(v213.clone(), v214.clone()));
                        Rc::new(UH1::UH1_Cons(v203.clone(), v215.clone()))
                    }
                    US1::US1_LibraryBinding => {
                        let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("FParsec."); } LIT.with(|lit| lit.clone()) };
                        let v263: US2 = US2::US2_Text(v262.clone());
                        let v264: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v265: Rc<UH2> = Rc::new(UH2::UH2_Cons(v263.clone(), v264.clone()));
                        let v266: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v265.clone(), v266.clone()))
                    }
                    US1::US1_PlatformBoundary => {
                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cfg"); } LIT.with(|lit| lit.clone()) };
                        let v86: US2 = US2::US2_Text(v85.clone());
                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let v88: US2 = US2::US2_Maybe(v87.clone());
                        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
                        let v90: US2 = US2::US2_Text(v89.clone());
                        let v91: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v92: Rc<UH2> = Rc::new(UH2::UH2_Cons(v90.clone(), v91.clone()));
                        let v93: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88.clone(), v92.clone()));
                        let v94: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86.clone(), v93.clone()));
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[cfg"); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("builtin.os.tag"); } LIT.with(|lit| lit.clone()) };
                        let v97: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v98: Rc<UH3> = Rc::new(UH3::UH3_Cons(v96.clone(), v97.clone()));
                        let v99: Rc<UH3> = Rc::new(UH3::UH3_Cons(v95.clone(), v98.clone()));
                        let v100: US2 = US2::US2_Texts(v99.clone());
                        let v101: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v102: Rc<UH2> = Rc::new(UH2::UH2_Cons(v100.clone(), v101.clone()));
                        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#if"); } LIT.with(|lit| lit.clone()) };
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#endif"); } LIT.with(|lit| lit.clone()) };
                        let v105: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v106: Rc<UH3> = Rc::new(UH3::UH3_Cons(v104.clone(), v105.clone()));
                        let v107: Rc<UH3> = Rc::new(UH3::UH3_Cons(v103.clone(), v106.clone()));
                        let v108: US2 = US2::US2_Texts(v107.clone());
                        let v109: US2 = US2::US2_Edge;
                        let v110: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v111: Rc<UH2> = Rc::new(UH2::UH2_Cons(v109.clone(), v110.clone()));
                        let v112: Rc<UH2> = Rc::new(UH2::UH2_Cons(v108.clone(), v111.clone()));
                        let v113: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v114: Rc<UH1> = Rc::new(UH1::UH1_Cons(v112.clone(), v113.clone()));
                        let v115: Rc<UH1> = Rc::new(UH1::UH1_Cons(v102.clone(), v114.clone()));
                        Rc::new(UH1::UH1_Cons(v94.clone(), v115.clone()))
                    }
                    US1::US1_SequenceComprehension => {
                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yield"); } LIT.with(|lit| lit.clone()) };
                        let v143: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v144: Rc<UH3> = Rc::new(UH3::UH3_Cons(v142.clone(), v143.clone()));
                        let v145: US2 = US2::US2_Words(v144.clone());
                        let v146: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v147: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145.clone(), v146.clone()));
                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                        let v149: US2 = US2::US2_Text(v148.clone());
                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                        let v151: US3 = US3::US3_NoneOf(v150.clone());
                        let v152: US2 = US2::US2_Run(v151.clone(), 0i32);
                        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("for"); } LIT.with(|lit| lit.clone()) };
                        let v154: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v155: Rc<UH3> = Rc::new(UH3::UH3_Cons(v153.clone(), v154.clone()));
                        let v156: US2 = US2::US2_Words(v155.clone());
                        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v158: US2 = US2::US2_Text(v157.clone());
                        let v159: US3 = US3::US3_WordLetters;
                        let v160: US2 = US2::US2_Run(v159.clone(), 1i32);
                        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
                        let v162: US2 = US2::US2_Text(v161.clone());
                        let v163: US3 = US3::US3_NoneOf(v150.clone());
                        let v164: US2 = US2::US2_Run(v163.clone(), 0i32);
                        let v165: US2 = US2::US2_Text(v150.clone());
                        let v166: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v167: Rc<UH2> = Rc::new(UH2::UH2_Cons(v165.clone(), v166.clone()));
                        let v168: Rc<UH2> = Rc::new(UH2::UH2_Cons(v164.clone(), v167.clone()));
                        let v169: Rc<UH2> = Rc::new(UH2::UH2_Cons(v162.clone(), v168.clone()));
                        let v170: Rc<UH2> = Rc::new(UH2::UH2_Cons(v160.clone(), v169.clone()));
                        let v171: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158.clone(), v170.clone()));
                        let v172: Rc<UH2> = Rc::new(UH2::UH2_Cons(v156.clone(), v171.clone()));
                        let v173: Rc<UH2> = Rc::new(UH2::UH2_Cons(v152.clone(), v172.clone()));
                        let v174: Rc<UH2> = Rc::new(UH2::UH2_Cons(v149.clone(), v173.clone()));
                        let v175: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v176: Rc<UH1> = Rc::new(UH1::UH1_Cons(v174.clone(), v175.clone()));
                        Rc::new(UH1::UH1_Cons(v147.clone(), v176.clone()))
                    }
                    US1::US1_TypeDefinition => {
                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v118: US2 = US2::US2_Text(v117.clone());
                        let v119: US3 = US3::US3_Blanks;
                        let v120: US2 = US2::US2_Run(v119.clone(), 0i32);
                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("class "); } LIT.with(|lit| lit.clone()) };
                        let v122: US2 = US2::US2_Text(v121.clone());
                        let v123: US3 = US3::US3_WordLetters;
                        let v124: US2 = US2::US2_Run(v123.clone(), 1i32);
                        let v125: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v126: Rc<UH2> = Rc::new(UH2::UH2_Cons(v124.clone(), v125.clone()));
                        let v127: Rc<UH2> = Rc::new(UH2::UH2_Cons(v122.clone(), v126.clone()));
                        let v128: Rc<UH2> = Rc::new(UH2::UH2_Cons(v120.clone(), v127.clone()));
                        let v129: Rc<UH2> = Rc::new(UH2::UH2_Cons(v118.clone(), v128.clone()));
                        let v130: US2 = US2::US2_Text(v117.clone());
                        let v131: US3 = US3::US3_Blanks;
                        let v132: US2 = US2::US2_Run(v131.clone(), 1i32);
                        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def __"); } LIT.with(|lit| lit.clone()) };
                        let v134: US2 = US2::US2_Text(v133.clone());
                        let v135: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v136: Rc<UH2> = Rc::new(UH2::UH2_Cons(v134.clone(), v135.clone()));
                        let v137: Rc<UH2> = Rc::new(UH2::UH2_Cons(v132.clone(), v136.clone()));
                        let v138: Rc<UH2> = Rc::new(UH2::UH2_Cons(v130.clone(), v137.clone()));
                        let v139: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v140: Rc<UH1> = Rc::new(UH1::UH1_Cons(v138.clone(), v139.clone()));
                        Rc::new(UH1::UH1_Cons(v129.clone(), v140.clone()))
                    }
                    US1::US1_UnionBridge => {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("match"); } LIT.with(|lit| lit.clone()) };
                        let v20: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v21: Rc<UH3> = Rc::new(UH3::UH3_Cons(v19.clone(), v20.clone()));
                        let v22: US2 = US2::US2_Words(v21.clone());
                        let v23: US3 = US3::US3_AnyLetter;
                        let v24: US2 = US2::US2_Run(v23.clone(), 0i32);
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ok"); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Error"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Err"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Some"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Choice1Of2"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Choice2Of2"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Poll"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v34: Rc<UH3> = Rc::new(UH3::UH3_Cons(v32.clone(), v33.clone()));
                        let v35: Rc<UH3> = Rc::new(UH3::UH3_Cons(v31.clone(), v34.clone()));
                        let v36: Rc<UH3> = Rc::new(UH3::UH3_Cons(v30.clone(), v35.clone()));
                        let v37: Rc<UH3> = Rc::new(UH3::UH3_Cons(v29.clone(), v36.clone()));
                        let v38: Rc<UH3> = Rc::new(UH3::UH3_Cons(v28.clone(), v37.clone()));
                        let v39: Rc<UH3> = Rc::new(UH3::UH3_Cons(v27.clone(), v38.clone()));
                        let v40: Rc<UH3> = Rc::new(UH3::UH3_Cons(v26.clone(), v39.clone()));
                        let v41: Rc<UH3> = Rc::new(UH3::UH3_Cons(v25.clone(), v40.clone()));
                        let v42: US2 = US2::US2_Words(v41.clone());
                        let v43: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v44: Rc<UH2> = Rc::new(UH2::UH2_Cons(v42.clone(), v43.clone()));
                        let v45: Rc<UH2> = Rc::new(UH2::UH2_Cons(v24.clone(), v44.clone()));
                        let v46: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22.clone(), v45.clone()));
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("case"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v49: Rc<UH3> = Rc::new(UH3::UH3_Cons(v47.clone(), v48.clone()));
                        let v50: US2 = US2::US2_Words(v49.clone());
                        let v51: US3 = US3::US3_AnyLetter;
                        let v52: US2 = US2::US2_Run(v51.clone(), 0i32);
                        let v53: US2 = US2::US2_Edge;
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ok("); } LIT.with(|lit| lit.clone()) };
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Error("); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v57: Rc<UH3> = Rc::new(UH3::UH3_Cons(v55.clone(), v56.clone()));
                        let v58: Rc<UH3> = Rc::new(UH3::UH3_Cons(v54.clone(), v57.clone()));
                        let v59: US2 = US2::US2_Texts(v58.clone());
                        let v60: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v61: Rc<UH2> = Rc::new(UH2::UH2_Cons(v59.clone(), v60.clone()));
                        let v62: Rc<UH2> = Rc::new(UH2::UH2_Cons(v53.clone(), v61.clone()));
                        let v63: Rc<UH2> = Rc::new(UH2::UH2_Cons(v52.clone(), v62.clone()));
                        let v64: Rc<UH2> = Rc::new(UH2::UH2_Cons(v50.clone(), v63.clone()));
                        let v65: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v66: Rc<UH1> = Rc::new(UH1::UH1_Cons(v64.clone(), v65.clone()));
                        Rc::new(UH1::UH1_Cons(v46.clone(), v66.clone()))
                    }
                    US1::US1_UnionBridgeHeader => {
                        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'match !"); } LIT.with(|lit| lit.clone()) };
                        let v69: US2 = US2::US2_Text(v68.clone());
                        let v70: US3 = US3::US3_WordLetters;
                        let v71: US2 = US2::US2_Run(v70.clone(), 1i32);
                        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with'"); } LIT.with(|lit| lit.clone()) };
                        let v73: US2 = US2::US2_Text(v72.clone());
                        let v74: US3 = US3::US3_Blanks;
                        let v75: US2 = US2::US2_Run(v74.clone(), 0i32);
                        let v76: US2 = US2::US2_End;
                        let v77: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v78: Rc<UH2> = Rc::new(UH2::UH2_Cons(v76.clone(), v77.clone()));
                        let v79: Rc<UH2> = Rc::new(UH2::UH2_Cons(v75.clone(), v78.clone()));
                        let v80: Rc<UH2> = Rc::new(UH2::UH2_Cons(v73.clone(), v79.clone()));
                        let v81: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71.clone(), v80.clone()));
                        let v82: Rc<UH2> = Rc::new(UH2::UH2_Cons(v69.clone(), v81.clone()));
                        let v83: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v82.clone(), v83.clone()))
                    }
                    US1::US1_ZigCapture => {
                        let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("if"); } LIT.with(|lit| lit.clone()) };
                        let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("while"); } LIT.with(|lit| lit.clone()) };
                        let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catch"); } LIT.with(|lit| lit.clone()) };
                        let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orelse"); } LIT.with(|lit| lit.clone()) };
                        let v221: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v222: Rc<UH3> = Rc::new(UH3::UH3_Cons(v220.clone(), v221.clone()));
                        let v223: Rc<UH3> = Rc::new(UH3::UH3_Cons(v219.clone(), v222.clone()));
                        let v224: Rc<UH3> = Rc::new(UH3::UH3_Cons(v218.clone(), v223.clone()));
                        let v225: Rc<UH3> = Rc::new(UH3::UH3_Cons(v217.clone(), v224.clone()));
                        let v226: US2 = US2::US2_Words(v225.clone());
                        let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                        let v228: US3 = US3::US3_NoneOf(v227.clone());
                        let v229: US2 = US2::US2_Run(v228.clone(), 0i32);
                        let v230: US2 = US2::US2_Text(v227.clone());
                        let v231: US3 = US3::US3_WordLetters;
                        let v232: US2 = US2::US2_Run(v231.clone(), 1i32);
                        let v233: US2 = US2::US2_Text(v227.clone());
                        let v234: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v235: Rc<UH2> = Rc::new(UH2::UH2_Cons(v233.clone(), v234.clone()));
                        let v236: Rc<UH2> = Rc::new(UH2::UH2_Cons(v232.clone(), v235.clone()));
                        let v237: Rc<UH2> = Rc::new(UH2::UH2_Cons(v230.clone(), v236.clone()));
                        let v238: Rc<UH2> = Rc::new(UH2::UH2_Cons(v229.clone(), v237.clone()));
                        let v239: Rc<UH2> = Rc::new(UH2::UH2_Cons(v226.clone(), v238.clone()));
                        let v240: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v239.clone(), v240.clone()))
                    }
                };
                let v279: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v280: Rc<UH1> = method2(v278.clone(), v279.clone());
                let v281: bool = found_5(v0.clone(), v280.clone());
                if v281 {
                    return US0::US0_Some(v3.clone());
                } else {
                    (v0, v1) = (v0.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn construct_of_0(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v2: US1 = US1::US1_ExceptionBoundary;
    let v3: US1 = US1::US1_UnionBridge;
    let v4: US1 = US1::US1_UnionBridgeHeader;
    let v5: US1 = US1::US1_PlatformBoundary;
    let v6: US1 = US1::US1_TypeDefinition;
    let v7: US1 = US1::US1_SequenceComprehension;
    let v8: US1 = US1::US1_CPointerArrow;
    let v9: US1 = US1::US1_HostTypeText;
    let v10: US1 = US1::US1_ZigCapture;
    let v11: US1 = US1::US1_Comment;
    let v12: US1 = US1::US1_LibraryBinding;
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v14));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v16));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v22));
    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v23));
    method1(v1, v24)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let v1: US0 = construct_of_0(v0.clone());
        let v2: bool = match &v1 {
            US0::US0_None => {
                true
            }
            _ => {
                false
            }
        };
        let v3: bool = v2 != true;
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_flow_construct(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
