#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ExceptionBoundary,
    US0_UnionBridge,
    US0_UnionBridgeHeader,
    US0_PlatformBoundary,
    US0_TypeDefinition,
    US0_SequenceComprehension,
    US0_CPointerArrow,
    US0_HostTypeText,
    US0_ZigCapture,
    US0_Comment,
    US0_LibraryBinding,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US0),
    US1_None,
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
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH3> = v3;
            let mut v4: Rc<UH3> = method4(v3, v1);
            let v5: Rc<str> = Rc::<str>::from(v2.to_lowercase());
            Rc::new(UH3::UH3_Cons(v5, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method3(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US2 = v2;
            let mut v3: Rc<UH2> = v3;
            let mut v4: Rc<UH2> = method3(v3, v1);
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
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<UH2> = v2;
            let mut v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = method2(v3, v1);
            let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH2> = method3(v2, v5);
            Rc::new(UH1::UH1_Cons(v6, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn attempt_9(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: i32, mut v4: i32) -> bool {
    loop {
        let v5: bool = v4 < v3;
        if v5 {
            return false;
        } else {
            let v6: i32 = v1.wrapping_add(v4);
            let v7: bool = matches_at_8(v0, v2.clone(), v6);
            if v7 {
                return true;
            } else {
                let v8: i32 = v4.wrapping_sub(1i32);
                v4 = v8;
                continue;
            }
        }
    }
}
fn code_at_11(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    let v5: bool = if v2 {
        true
    } else {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        v4
    };
    if v5 {
        -1i32
    } else {
        let v6: u8 = v0.borrow()[v1 as usize];
        let v7: i32 = (v6 as i32);
        v7
    }
}
fn word_code_12(mut v0: i32) -> bool {
    let mut v1: bool = v0 >= 97i32;
    let mut v3: bool = if v1 {
        let mut v2: bool = v0 <= 122i32;
        v2
    } else {
        false
    };
    let v7: bool = if v3 {
        true
    } else {
        let mut v4: bool = v0 >= 65i32;
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
fn edge_at_10(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32) -> bool {
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: i32 = code_at_11(v0, v2);
    let mut v4: bool = word_code_12(v3);
    let v5: i32 = code_at_11(v0, v1);
    let v6: bool = word_code_12(v5);
    let v7: bool = v4 != v6;
    v7
}
fn bytes_from_14(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: Rc<RefCell<Vec<u8>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v3 >= v4;
        if v5 {
            return true;
        } else {
            let v6: i32 = v2.wrapping_add(v3);
            let v7: i32 = code_at_11(v0, v6);
            let v8: i32 = code_at_11(&v1, v3);
            let v9: bool = v7 == v8;
            if v9 {
                let v10: i32 = v3.wrapping_add(1i32);
                v3 = v10;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn text_at_13(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v2.as_bytes().to_vec()));
    let mut v4: i32 = 0i32;
    bytes_from_14(v0, v3, v1, v4)
}
fn code_in_17(mut v0: Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v2 < v3;
        if v4 {
            let v5: i32 = code_at_11(&v0, v2);
            let v6: bool = v5 == v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        } else {
            return false;
        }
    }
}
fn fits_16(mut v0: US3, mut v1: i32) -> bool {
    match v0 {
        US3::US3_AnyLetter => {
            true
        }
        US3::US3_AnyOf(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v11.as_bytes().to_vec()));
            let v13: i32 = 0i32;
            code_in_17(v12, v1, v13)
        }
        US3::US3_Blanks => {
            let mut v3: bool = v1 == 32i32;
            if v3 {
                true
            } else {
                let mut v4: bool = v1 == 9i32;
                v4
            }
        }
        US3::US3_NoneOf(v6) => {
            let v6: Rc<str> = v6;
            let v7: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v6.as_bytes().to_vec()));
            let v8: i32 = 0i32;
            let v9: bool = code_in_17(v7, v1, v8);
            let v10: bool = v9 == false;
            v10
        }
        US3::US3_WordLetters => {
            word_code_12(v1)
        }
    }
}
fn run_length_15(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: US3, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = v2.wrapping_add(v3);
        let v5: i32 = (v0.clone().borrow().len() as i32);
        let v6: bool = v4 < v5;
        let v9: bool = if v6 {
            let v7: i32 = code_at_11(v0, v4);
            fits_16(v1.clone(), v7)
        } else {
            false
        };
        if v9 {
            let v10: i32 = v3.wrapping_add(1i32);
            v3 = v10;
            continue;
        } else {
            return v3;
        }
    }
}
fn method18(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH3::UH3_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH3> = v6;
                let v7: US2 = US2::US2_Text(v5.clone());
                let v8: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v2.clone()));
                let v9: bool = matches_at_8(v0, v8, v1);
                if v9 {
                    return US5::US5_Some(v5);
                } else {
                    v3 = v6;
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US5::US5_None;
            }
        }
    }
}
fn method19(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH3::UH3_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let v6: Rc<UH3> = v6;
                let v7: US2 = US2::US2_Edge;
                let v8: US2 = US2::US2_Text(v5.clone());
                let v9: US2 = US2::US2_Edge;
                let v10: Rc<UH2> = Rc::new(UH2::UH2_Cons(v9, v2.clone()));
                let v11: Rc<UH2> = Rc::new(UH2::UH2_Cons(v8, v10));
                let v12: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v11));
                let v13: bool = matches_at_8(v0, v12, v1);
                if v13 {
                    return US5::US5_Some(v5);
                } else {
                    v3 = v6;
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return US5::US5_None;
            }
        }
    }
}
fn matches_at_8(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH2>, mut v2: i32) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let mut v3: US2 = v3;
                let mut v4: Rc<UH2> = v4;
                match v3 {
                    US2::US2_Edge => {
                        let v27: bool = edge_at_10(v0, v2);
                        if v27 {
                            v1 = v4;
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_End => {
                        let v33: i32 = (v0.clone().borrow().len() as i32);
                        let v34: bool = v2 == v33;
                        if v34 {
                            v1 = v4;
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Maybe(v19) => {
                        let v19: Rc<str> = v19;
                        let v20: bool = text_at_13(v0, v2, v19.clone());
                        let v24: bool = if v20 {
                            let v21: i32 = (v19.len() as i32);
                            let v22: i32 = v2.wrapping_add(v21);
                            matches_at_8(v0, v4.clone(), v22)
                        } else {
                            false
                        };
                        if v24 {
                            return true;
                        } else {
                            v1 = v4;
                            continue;
                        }
                    }
                    US2::US2_Run(v37, v38) => {
                        let v37: US3 = v37;
                        let v38: i32 = v38;
                        let v39: i32 = 0i32;
                        let v40: i32 = run_length_15(v0, v37, v2, v39);
                        return attempt_9(v0, v2, v4, v38, v40);
                    }
                    US2::US2_Start => {
                        let v30: bool = v2 == 0i32;
                        if v30 {
                            v1 = v4;
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Text(v5) => {
                        let v5: Rc<str> = v5;
                        let v6: bool = text_at_13(v0, v2, v5.clone());
                        if v6 {
                            let v7: i32 = (v5.len() as i32);
                            let v8: i32 = v2.wrapping_add(v7);
                            (v1, v2) = (v4, v8);
                            continue;
                        } else {
                            return false;
                        }
                    }
                    US2::US2_Texts(v11) => {
                        let v11: Rc<UH3> = v11;
                        let v12: US5 = method18(v0, v2, v4, v11);
                        let v13: bool = match v12 {
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
                        let v15: Rc<UH3> = v15;
                        let v16: US5 = method19(v0, v2, v4, v15);
                        let v17: bool = match v16 {
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
fn found_from_7(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH2>, mut v2: i32) -> bool {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v2 > v3;
        if v4 {
            return false;
        } else {
            let v5: bool = matches_at_8(v0, v1.clone(), v2);
            if v5 {
                return true;
            } else {
                let v6: i32 = v2.wrapping_add(1i32);
                v2 = v6;
                continue;
            }
        }
    }
}
fn method6(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: Rc<UH1>) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<UH2> = v3;
                let mut v4: Rc<UH1> = v4;
                let v5: i32 = 0i32;
                let v6: bool = found_from_7(v0, v3.clone(), v5);
                if v6 {
                    return US4::US4_Some(v3);
                } else {
                    v1 = v4;
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
    let mut v2: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v0.as_bytes().to_vec()));
    let mut v3: US4 = method6(&v2, v1);
    let mut v4: bool = match v3 {
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
fn method1(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: US0 = v3;
                let mut v4: Rc<UH0> = v4;
                let v278: Rc<UH1> = match &v3 {
                    US0::US0_CPointerArrow => {
                        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")->"); } LIT.with(|lit| lit.clone()) };
                        let v179: US2 = US2::US2_Text(v178);
                        let v180: US3 = US3::US3_WordLetters;
                        let v181: US2 = US2::US2_Run(v180, 1i32);
                        let v182: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v183: Rc<UH2> = Rc::new(UH2::UH2_Cons(v181, v182));
                        let v184: Rc<UH2> = Rc::new(UH2::UH2_Cons(v179, v183));
                        let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->ptr"); } LIT.with(|lit| lit.clone()) };
                        let v186: US2 = US2::US2_Text(v185);
                        let v187: US2 = US2::US2_Edge;
                        let v188: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v189: Rc<UH2> = Rc::new(UH2::UH2_Cons(v187, v188));
                        let v190: Rc<UH2> = Rc::new(UH2::UH2_Cons(v186, v189));
                        let v191: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v192: Rc<UH1> = Rc::new(UH1::UH1_Cons(v190, v191));
                        Rc::new(UH1::UH1_Cons(v184, v192))
                    }
                    US0::US0_Comment => {
                        let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v243: US2 = US2::US2_Text(v242);
                        let v244: US3 = US3::US3_Blanks;
                        let v245: US2 = US2::US2_Run(v244, 0i32);
                        let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                        let v247: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                        let v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("##"); } LIT.with(|lit| lit.clone()) };
                        let v249: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/*"); } LIT.with(|lit| lit.clone()) };
                        let v250: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v251: Rc<UH3> = Rc::new(UH3::UH3_Cons(v249, v250));
                        let v252: Rc<UH3> = Rc::new(UH3::UH3_Cons(v248, v251));
                        let v253: Rc<UH3> = Rc::new(UH3::UH3_Cons(v247, v252));
                        let v254: Rc<UH3> = Rc::new(UH3::UH3_Cons(v246, v253));
                        let v255: US2 = US2::US2_Texts(v254);
                        let v256: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v257: Rc<UH2> = Rc::new(UH2::UH2_Cons(v255, v256));
                        let v258: Rc<UH2> = Rc::new(UH2::UH2_Cons(v245, v257));
                        let v259: Rc<UH2> = Rc::new(UH2::UH2_Cons(v243, v258));
                        let v260: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v259, v260))
                    }
                    US0::US0_ExceptionBoundary => {
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("try"); } LIT.with(|lit| lit.clone()) };
                        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("with ex"); } LIT.with(|lit| lit.clone()) };
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catch"); } LIT.with(|lit| lit.clone()) };
                        let v9: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v10: Rc<UH3> = Rc::new(UH3::UH3_Cons(v8, v9));
                        let v11: Rc<UH3> = Rc::new(UH3::UH3_Cons(v7, v10));
                        let v12: Rc<UH3> = Rc::new(UH3::UH3_Cons(v6, v11));
                        let v13: Rc<UH3> = Rc::new(UH3::UH3_Cons(v5, v12));
                        let v14: US2 = US2::US2_Words(v13);
                        let v15: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH2> = Rc::new(UH2::UH2_Cons(v14, v15));
                        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v16, v17))
                    }
                    US0::US0_HostTypeText => {
                        let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Fn"); } LIT.with(|lit| lit.clone()) };
                        let v195: US2 = US2::US2_Text(v194);
                        let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Once"); } LIT.with(|lit| lit.clone()) };
                        let v197: US2 = US2::US2_Maybe(v196);
                        let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() ->"); } LIT.with(|lit| lit.clone()) };
                        let v199: US2 = US2::US2_Text(v198);
                        let v200: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v201: Rc<UH2> = Rc::new(UH2::UH2_Cons(v199, v200));
                        let v202: Rc<UH2> = Rc::new(UH2::UH2_Cons(v197, v201));
                        let v203: Rc<UH2> = Rc::new(UH2::UH2_Cons(v195, v202));
                        let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() => void"); } LIT.with(|lit| lit.clone()) };
                        let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("regex::Match"); } LIT.with(|lit| lit.clone()) };
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leptos::prelude::"); } LIT.with(|lit| lit.clone()) };
                        let v207: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v208: Rc<UH3> = Rc::new(UH3::UH3_Cons(v206, v207));
                        let v209: Rc<UH3> = Rc::new(UH3::UH3_Cons(v205, v208));
                        let v210: Rc<UH3> = Rc::new(UH3::UH3_Cons(v204, v209));
                        let v211: US2 = US2::US2_Texts(v210);
                        let v212: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v213: Rc<UH2> = Rc::new(UH2::UH2_Cons(v211, v212));
                        let v214: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v215: Rc<UH1> = Rc::new(UH1::UH1_Cons(v213, v214));
                        Rc::new(UH1::UH1_Cons(v203, v215))
                    }
                    US0::US0_LibraryBinding => {
                        let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("FParsec."); } LIT.with(|lit| lit.clone()) };
                        let v263: US2 = US2::US2_Text(v262);
                        let v264: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v265: Rc<UH2> = Rc::new(UH2::UH2_Cons(v263, v264));
                        let v266: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v265, v266))
                    }
                    US0::US0_PlatformBoundary => {
                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cfg"); } LIT.with(|lit| lit.clone()) };
                        let v86: US2 = US2::US2_Text(v85);
                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let v88: US2 = US2::US2_Maybe(v87);
                        let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
                        let v90: US2 = US2::US2_Text(v89);
                        let v91: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v92: Rc<UH2> = Rc::new(UH2::UH2_Cons(v90, v91));
                        let v93: Rc<UH2> = Rc::new(UH2::UH2_Cons(v88, v92));
                        let v94: Rc<UH2> = Rc::new(UH2::UH2_Cons(v86, v93));
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#[cfg"); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("builtin.os.tag"); } LIT.with(|lit| lit.clone()) };
                        let v97: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v98: Rc<UH3> = Rc::new(UH3::UH3_Cons(v96, v97));
                        let v99: Rc<UH3> = Rc::new(UH3::UH3_Cons(v95, v98));
                        let v100: US2 = US2::US2_Texts(v99);
                        let v101: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v102: Rc<UH2> = Rc::new(UH2::UH2_Cons(v100, v101));
                        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#if"); } LIT.with(|lit| lit.clone()) };
                        let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#endif"); } LIT.with(|lit| lit.clone()) };
                        let v105: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v106: Rc<UH3> = Rc::new(UH3::UH3_Cons(v104, v105));
                        let v107: Rc<UH3> = Rc::new(UH3::UH3_Cons(v103, v106));
                        let v108: US2 = US2::US2_Texts(v107);
                        let v109: US2 = US2::US2_Edge;
                        let v110: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v111: Rc<UH2> = Rc::new(UH2::UH2_Cons(v109, v110));
                        let v112: Rc<UH2> = Rc::new(UH2::UH2_Cons(v108, v111));
                        let v113: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v114: Rc<UH1> = Rc::new(UH1::UH1_Cons(v112, v113));
                        let v115: Rc<UH1> = Rc::new(UH1::UH1_Cons(v102, v114));
                        Rc::new(UH1::UH1_Cons(v94, v115))
                    }
                    US0::US0_SequenceComprehension => {
                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yield"); } LIT.with(|lit| lit.clone()) };
                        let v143: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v144: Rc<UH3> = Rc::new(UH3::UH3_Cons(v142, v143));
                        let v145: US2 = US2::US2_Words(v144);
                        let v146: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v147: Rc<UH2> = Rc::new(UH2::UH2_Cons(v145, v146));
                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                        let v149: US2 = US2::US2_Text(v148);
                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
                        let v151: US3 = US3::US3_NoneOf(v150.clone());
                        let v152: US2 = US2::US2_Run(v151, 0i32);
                        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("for"); } LIT.with(|lit| lit.clone()) };
                        let v154: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v155: Rc<UH3> = Rc::new(UH3::UH3_Cons(v153, v154));
                        let v156: US2 = US2::US2_Words(v155);
                        let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v158: US2 = US2::US2_Text(v157);
                        let v159: US3 = US3::US3_WordLetters;
                        let v160: US2 = US2::US2_Run(v159, 1i32);
                        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
                        let v162: US2 = US2::US2_Text(v161);
                        let v163: US3 = US3::US3_NoneOf(v150.clone());
                        let v164: US2 = US2::US2_Run(v163, 0i32);
                        let v165: US2 = US2::US2_Text(v150);
                        let v166: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v167: Rc<UH2> = Rc::new(UH2::UH2_Cons(v165, v166));
                        let v168: Rc<UH2> = Rc::new(UH2::UH2_Cons(v164, v167));
                        let v169: Rc<UH2> = Rc::new(UH2::UH2_Cons(v162, v168));
                        let v170: Rc<UH2> = Rc::new(UH2::UH2_Cons(v160, v169));
                        let v171: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158, v170));
                        let v172: Rc<UH2> = Rc::new(UH2::UH2_Cons(v156, v171));
                        let v173: Rc<UH2> = Rc::new(UH2::UH2_Cons(v152, v172));
                        let v174: Rc<UH2> = Rc::new(UH2::UH2_Cons(v149, v173));
                        let v175: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v176: Rc<UH1> = Rc::new(UH1::UH1_Cons(v174, v175));
                        Rc::new(UH1::UH1_Cons(v147, v176))
                    }
                    US0::US0_TypeDefinition => {
                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v118: US2 = US2::US2_Text(v117.clone());
                        let v119: US3 = US3::US3_Blanks;
                        let v120: US2 = US2::US2_Run(v119, 0i32);
                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("class "); } LIT.with(|lit| lit.clone()) };
                        let v122: US2 = US2::US2_Text(v121);
                        let v123: US3 = US3::US3_WordLetters;
                        let v124: US2 = US2::US2_Run(v123, 1i32);
                        let v125: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v126: Rc<UH2> = Rc::new(UH2::UH2_Cons(v124, v125));
                        let v127: Rc<UH2> = Rc::new(UH2::UH2_Cons(v122, v126));
                        let v128: Rc<UH2> = Rc::new(UH2::UH2_Cons(v120, v127));
                        let v129: Rc<UH2> = Rc::new(UH2::UH2_Cons(v118, v128));
                        let v130: US2 = US2::US2_Text(v117);
                        let v131: US3 = US3::US3_Blanks;
                        let v132: US2 = US2::US2_Run(v131, 1i32);
                        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("def __"); } LIT.with(|lit| lit.clone()) };
                        let v134: US2 = US2::US2_Text(v133);
                        let v135: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v136: Rc<UH2> = Rc::new(UH2::UH2_Cons(v134, v135));
                        let v137: Rc<UH2> = Rc::new(UH2::UH2_Cons(v132, v136));
                        let v138: Rc<UH2> = Rc::new(UH2::UH2_Cons(v130, v137));
                        let v139: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v140: Rc<UH1> = Rc::new(UH1::UH1_Cons(v138, v139));
                        Rc::new(UH1::UH1_Cons(v129, v140))
                    }
                    US0::US0_UnionBridge => {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("match"); } LIT.with(|lit| lit.clone()) };
                        let v20: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v21: Rc<UH3> = Rc::new(UH3::UH3_Cons(v19, v20));
                        let v22: US2 = US2::US2_Words(v21);
                        let v23: US3 = US3::US3_AnyLetter;
                        let v24: US2 = US2::US2_Run(v23, 0i32);
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ok"); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Error"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Err"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Some"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Choice1Of2"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Choice2Of2"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Poll"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v34: Rc<UH3> = Rc::new(UH3::UH3_Cons(v32, v33));
                        let v35: Rc<UH3> = Rc::new(UH3::UH3_Cons(v31, v34));
                        let v36: Rc<UH3> = Rc::new(UH3::UH3_Cons(v30, v35));
                        let v37: Rc<UH3> = Rc::new(UH3::UH3_Cons(v29, v36));
                        let v38: Rc<UH3> = Rc::new(UH3::UH3_Cons(v28, v37));
                        let v39: Rc<UH3> = Rc::new(UH3::UH3_Cons(v27, v38));
                        let v40: Rc<UH3> = Rc::new(UH3::UH3_Cons(v26, v39));
                        let v41: Rc<UH3> = Rc::new(UH3::UH3_Cons(v25, v40));
                        let v42: US2 = US2::US2_Words(v41);
                        let v43: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v44: Rc<UH2> = Rc::new(UH2::UH2_Cons(v42, v43));
                        let v45: Rc<UH2> = Rc::new(UH2::UH2_Cons(v24, v44));
                        let v46: Rc<UH2> = Rc::new(UH2::UH2_Cons(v22, v45));
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("case"); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v49: Rc<UH3> = Rc::new(UH3::UH3_Cons(v47, v48));
                        let v50: US2 = US2::US2_Words(v49);
                        let v51: US3 = US3::US3_AnyLetter;
                        let v52: US2 = US2::US2_Run(v51, 0i32);
                        let v53: US2 = US2::US2_Edge;
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ok("); } LIT.with(|lit| lit.clone()) };
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Error("); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v57: Rc<UH3> = Rc::new(UH3::UH3_Cons(v55, v56));
                        let v58: Rc<UH3> = Rc::new(UH3::UH3_Cons(v54, v57));
                        let v59: US2 = US2::US2_Texts(v58);
                        let v60: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v61: Rc<UH2> = Rc::new(UH2::UH2_Cons(v59, v60));
                        let v62: Rc<UH2> = Rc::new(UH2::UH2_Cons(v53, v61));
                        let v63: Rc<UH2> = Rc::new(UH2::UH2_Cons(v52, v62));
                        let v64: Rc<UH2> = Rc::new(UH2::UH2_Cons(v50, v63));
                        let v65: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v66: Rc<UH1> = Rc::new(UH1::UH1_Cons(v64, v65));
                        Rc::new(UH1::UH1_Cons(v46, v66))
                    }
                    US0::US0_UnionBridgeHeader => {
                        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'match !"); } LIT.with(|lit| lit.clone()) };
                        let v69: US2 = US2::US2_Text(v68);
                        let v70: US3 = US3::US3_WordLetters;
                        let v71: US2 = US2::US2_Run(v70, 1i32);
                        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" with'"); } LIT.with(|lit| lit.clone()) };
                        let v73: US2 = US2::US2_Text(v72);
                        let v74: US3 = US3::US3_Blanks;
                        let v75: US2 = US2::US2_Run(v74, 0i32);
                        let v76: US2 = US2::US2_End;
                        let v77: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v78: Rc<UH2> = Rc::new(UH2::UH2_Cons(v76, v77));
                        let v79: Rc<UH2> = Rc::new(UH2::UH2_Cons(v75, v78));
                        let v80: Rc<UH2> = Rc::new(UH2::UH2_Cons(v73, v79));
                        let v81: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71, v80));
                        let v82: Rc<UH2> = Rc::new(UH2::UH2_Cons(v69, v81));
                        let v83: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v82, v83))
                    }
                    US0::US0_ZigCapture => {
                        let v217: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("if"); } LIT.with(|lit| lit.clone()) };
                        let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("while"); } LIT.with(|lit| lit.clone()) };
                        let v219: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("catch"); } LIT.with(|lit| lit.clone()) };
                        let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orelse"); } LIT.with(|lit| lit.clone()) };
                        let v221: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v222: Rc<UH3> = Rc::new(UH3::UH3_Cons(v220, v221));
                        let v223: Rc<UH3> = Rc::new(UH3::UH3_Cons(v219, v222));
                        let v224: Rc<UH3> = Rc::new(UH3::UH3_Cons(v218, v223));
                        let v225: Rc<UH3> = Rc::new(UH3::UH3_Cons(v217, v224));
                        let v226: US2 = US2::US2_Words(v225);
                        let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                        let v228: US3 = US3::US3_NoneOf(v227.clone());
                        let v229: US2 = US2::US2_Run(v228, 0i32);
                        let v230: US2 = US2::US2_Text(v227.clone());
                        let v231: US3 = US3::US3_WordLetters;
                        let v232: US2 = US2::US2_Run(v231, 1i32);
                        let v233: US2 = US2::US2_Text(v227);
                        let v234: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v235: Rc<UH2> = Rc::new(UH2::UH2_Cons(v233, v234));
                        let v236: Rc<UH2> = Rc::new(UH2::UH2_Cons(v232, v235));
                        let v237: Rc<UH2> = Rc::new(UH2::UH2_Cons(v230, v236));
                        let v238: Rc<UH2> = Rc::new(UH2::UH2_Cons(v229, v237));
                        let v239: Rc<UH2> = Rc::new(UH2::UH2_Cons(v226, v238));
                        let v240: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v239, v240))
                    }
                };
                let v279: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v280: Rc<UH1> = method2(v278, v279);
                let v281: bool = found_5(v0.clone(), v280);
                if v281 {
                    return US1::US1_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn construct_of_0(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: US0 = US0::US0_ExceptionBoundary;
    let mut v3: US0 = US0::US0_UnionBridge;
    let mut v4: US0 = US0::US0_UnionBridgeHeader;
    let v5: US0 = US0::US0_PlatformBoundary;
    let v6: US0 = US0::US0_TypeDefinition;
    let v7: US0 = US0::US0_SequenceComprehension;
    let v8: US0 = US0::US0_CPointerArrow;
    let v9: US0 = US0::US0_HostTypeText;
    let v10: US0 = US0::US0_ZigCapture;
    let v11: US0 = US0::US0_Comment;
    let v12: US0 = US0::US0_LibraryBinding;
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
        let mut v1: US1 = construct_of_0(v0);
        let mut v2: bool = match v1 {
            US1::US1_None => {
                true
            }
            _ => {
                false
            }
        };
        let mut v3: bool = v2 != true;
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_flow_construct(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
