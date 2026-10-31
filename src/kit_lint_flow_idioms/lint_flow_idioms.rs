#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_ScopingIdiom,
    US1_HostCombinator,
    US1_ThunkOpener,
    US1_OneConstructTernary,
    US1_OneCallParse,
    US1_OneOpHostOperator,
    US1_GleamRebindingBlock,
    US1_HealedAlready,
    US1_LambdaArrow,
    US1_OneOpTernary,
    US1_FsharpPipeMutate,
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
                let v295: Rc<UH1> = match &v3 {
                    US1::US1_FsharpPipeMutate => {
                        let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'|> fun x -> _"); } LIT.with(|lit| lit.clone()) };
                        let v268: US2 = US2::US2_Text(v267.clone());
                        let v269: US3 = US3::US3_WordLetters;
                        let v270: US2 = US2::US2_Run(v269.clone(), 1i32);
                        let v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_!"); } LIT.with(|lit| lit.clone()) };
                        let v272: US2 = US2::US2_Text(v271.clone());
                        let v273: US3 = US3::US3_WordLetters;
                        let v274: US2 = US2::US2_Run(v273.clone(), 1i32);
                        let v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <- Some x'"); } LIT.with(|lit| lit.clone()) };
                        let v276: US2 = US2::US2_Text(v275.clone());
                        let v277: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v278: Rc<UH2> = Rc::new(UH2::UH2_Cons(v276.clone(), v277.clone()));
                        let v279: Rc<UH2> = Rc::new(UH2::UH2_Cons(v274.clone(), v278.clone()));
                        let v280: Rc<UH2> = Rc::new(UH2::UH2_Cons(v272.clone(), v279.clone()));
                        let v281: Rc<UH2> = Rc::new(UH2::UH2_Cons(v270.clone(), v280.clone()));
                        let v282: Rc<UH2> = Rc::new(UH2::UH2_Cons(v268.clone(), v281.clone()));
                        let v283: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v282.clone(), v283.clone()))
                    }
                    US1::US1_GleamRebindingBlock => {
                        let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v139: US2 = US2::US2_Text(v138.clone());
                        let v140: US3 = US3::US3_Blanks;
                        let v141: US2 = US2::US2_Run(v140.clone(), 1i32);
                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("case trace_state"); } LIT.with(|lit| lit.clone()) };
                        let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option.None ->"); } LIT.with(|lit| lit.clone()) };
                        let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x -> x"); } LIT.with(|lit| lit.clone()) };
                        let v145: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v146: Rc<UH3> = Rc::new(UH3::UH3_Cons(v144.clone(), v145.clone()));
                        let v147: Rc<UH3> = Rc::new(UH3::UH3_Cons(v143.clone(), v146.clone()));
                        let v148: Rc<UH3> = Rc::new(UH3::UH3_Cons(v142.clone(), v147.clone()));
                        let v149: US2 = US2::US2_Texts(v148.clone());
                        let v150: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v151: Rc<UH2> = Rc::new(UH2::UH2_Cons(v149.clone(), v150.clone()));
                        let v152: Rc<UH2> = Rc::new(UH2::UH2_Cons(v141.clone(), v151.clone()));
                        let v153: Rc<UH2> = Rc::new(UH2::UH2_Cons(v139.clone(), v152.clone()));
                        let v154: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v153.clone(), v154.clone()))
                    }
                    US1::US1_HealedAlready => {
                        let v156: US2 = US2::US2_Start;
                        let v157: US3 = US3::US3_Blanks;
                        let v158: US2 = US2::US2_Run(v157.clone(), 0i32);
                        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? $'"); } LIT.with(|lit| lit.clone()) };
                        let v160: US2 = US2::US2_Text(v159.clone());
                        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v162: US3 = US3::US3_NoneOf(v161.clone());
                        let v163: US2 = US2::US2_Run(v162.clone(), 1i32);
                        let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("' : bool ?>"); } LIT.with(|lit| lit.clone()) };
                        let v165: US2 = US2::US2_Text(v164.clone());
                        let v166: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v167: Rc<UH2> = Rc::new(UH2::UH2_Cons(v165.clone(), v166.clone()));
                        let v168: Rc<UH2> = Rc::new(UH2::UH2_Cons(v163.clone(), v167.clone()));
                        let v169: Rc<UH2> = Rc::new(UH2::UH2_Cons(v160.clone(), v168.clone()));
                        let v170: Rc<UH2> = Rc::new(UH2::UH2_Cons(v158.clone(), v169.clone()));
                        let v171: Rc<UH2> = Rc::new(UH2::UH2_Cons(v156.clone(), v170.clone()));
                        let v172: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v171.clone(), v172.clone()))
                    }
                    US1::US1_HostCombinator => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".all("); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("String.forall"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".any("); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Array.map (fun "); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v30: Rc<UH3> = Rc::new(UH3::UH3_Cons(v28.clone(), v29.clone()));
                        let v31: Rc<UH3> = Rc::new(UH3::UH3_Cons(v27.clone(), v30.clone()));
                        let v32: Rc<UH3> = Rc::new(UH3::UH3_Cons(v26.clone(), v31.clone()));
                        let v33: Rc<UH3> = Rc::new(UH3::UH3_Cons(v25.clone(), v32.clone()));
                        let v34: US2 = US2::US2_Texts(v33.clone());
                        let v35: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v36: Rc<UH2> = Rc::new(UH2::UH2_Cons(v34.clone(), v35.clone()));
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".map(|"); } LIT.with(|lit| lit.clone()) };
                        let v38: US2 = US2::US2_Text(v37.clone());
                        let v39: US3 = US3::US3_WordLetters;
                        let v40: US2 = US2::US2_Run(v39.clone(), 1i32);
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                        let v42: US2 = US2::US2_Text(v41.clone());
                        let v43: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v44: Rc<UH2> = Rc::new(UH2::UH2_Cons(v42.clone(), v43.clone()));
                        let v45: Rc<UH2> = Rc::new(UH2::UH2_Cons(v40.clone(), v44.clone()));
                        let v46: Rc<UH2> = Rc::new(UH2::UH2_Cons(v38.clone(), v45.clone()));
                        let v47: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v48: Rc<UH1> = Rc::new(UH1::UH1_Cons(v46.clone(), v47.clone()));
                        Rc::new(UH1::UH1_Cons(v36.clone(), v48.clone()))
                    }
                    US1::US1_LambdaArrow => {
                        let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fun"); } LIT.with(|lit| lit.clone()) };
                        let v175: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v176: Rc<UH3> = Rc::new(UH3::UH3_Cons(v174.clone(), v175.clone()));
                        let v177: US2 = US2::US2_Words(v176.clone());
                        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                        let v179: US2 = US2::US2_Text(v178.clone());
                        let v180: US3 = US3::US3_WordLetters;
                        let v181: US2 = US2::US2_Run(v180.clone(), 1i32);
                        let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ->"); } LIT.with(|lit| lit.clone()) };
                        let v183: US2 = US2::US2_Text(v182.clone());
                        let v184: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v185: Rc<UH2> = Rc::new(UH2::UH2_Cons(v183.clone(), v184.clone()));
                        let v186: Rc<UH2> = Rc::new(UH2::UH2_Cons(v181.clone(), v185.clone()));
                        let v187: Rc<UH2> = Rc::new(UH2::UH2_Cons(v179.clone(), v186.clone()));
                        let v188: Rc<UH2> = Rc::new(UH2::UH2_Cons(v177.clone(), v187.clone()));
                        let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'!x ' |> \\ x =>"); } LIT.with(|lit| lit.clone()) };
                        let v190: US2 = US2::US2_Text(v189.clone());
                        let v191: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v192: Rc<UH2> = Rc::new(UH2::UH2_Cons(v190.clone(), v191.clone()));
                        let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fn"); } LIT.with(|lit| lit.clone()) };
                        let v194: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v195: Rc<UH3> = Rc::new(UH3::UH3_Cons(v193.clone(), v194.clone()));
                        let v196: US2 = US2::US2_Words(v195.clone());
                        let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" $'!x '"); } LIT.with(|lit| lit.clone()) };
                        let v198: US2 = US2::US2_Text(v197.clone());
                        let v199: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v200: Rc<UH2> = Rc::new(UH2::UH2_Cons(v198.clone(), v199.clone()));
                        let v201: Rc<UH2> = Rc::new(UH2::UH2_Cons(v196.clone(), v200.clone()));
                        let v202: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v203: Rc<UH1> = Rc::new(UH1::UH1_Cons(v201.clone(), v202.clone()));
                        let v204: Rc<UH1> = Rc::new(UH1::UH1_Cons(v192.clone(), v203.clone()));
                        Rc::new(UH1::UH1_Cons(v188.clone(), v204.clone()))
                    }
                    US1::US1_OneCallParse => {
                        let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("strto"); } LIT.with(|lit| lit.clone()) };
                        let v109: US2 = US2::US2_Text(v108.clone());
                        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lfd"); } LIT.with(|lit| lit.clone()) };
                        let v111: US3 = US3::US3_AnyOf(v110.clone());
                        let v112: US2 = US2::US2_Run(v111.clone(), 0i32);
                        let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                        let v114: US2 = US2::US2_Text(v113.clone());
                        let v115: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v116: Rc<UH2> = Rc::new(UH2::UH2_Cons(v114.clone(), v115.clone()));
                        let v117: Rc<UH2> = Rc::new(UH2::UH2_Cons(v112.clone(), v116.clone()));
                        let v118: Rc<UH2> = Rc::new(UH2::UH2_Cons(v109.clone(), v117.clone()));
                        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("string.match("); } LIT.with(|lit| lit.clone()) };
                        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiralParseInt"); } LIT.with(|lit| lit.clone()) };
                        let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Int32(String("); } LIT.with(|lit| lit.clone()) };
                        let v122: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v123: Rc<UH3> = Rc::new(UH3::UH3_Cons(v121.clone(), v122.clone()));
                        let v124: Rc<UH3> = Rc::new(UH3::UH3_Cons(v120.clone(), v123.clone()));
                        let v125: Rc<UH3> = Rc::new(UH3::UH3_Cons(v119.clone(), v124.clone()));
                        let v126: US2 = US2::US2_Texts(v125.clone());
                        let v127: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v128: Rc<UH2> = Rc::new(UH2::UH2_Cons(v126.clone(), v127.clone()));
                        let v129: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v130: Rc<UH1> = Rc::new(UH1::UH1_Cons(v128.clone(), v129.clone()));
                        Rc::new(UH1::UH1_Cons(v118.clone(), v130.clone()))
                    }
                    US1::US1_OneConstructTernary => {
                        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(if !"); } LIT.with(|lit| lit.clone()) };
                        let v62: US2 = US2::US2_Text(v61.clone());
                        let v63: US3 = US3::US3_WordLetters;
                        let v64: US2 = US2::US2_Run(v63.clone(), 1i32);
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" then \""); } LIT.with(|lit| lit.clone()) };
                        let v66: US2 = US2::US2_Text(v65.clone());
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                        let v68: US3 = US3::US3_NoneOf(v67.clone());
                        let v69: US2 = US2::US2_Run(v68.clone(), 0i32);
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\" else \""); } LIT.with(|lit| lit.clone()) };
                        let v71: US2 = US2::US2_Text(v70.clone());
                        let v72: US3 = US3::US3_NoneOf(v67.clone());
                        let v73: US2 = US2::US2_Run(v72.clone(), 0i32);
                        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                        let v75: US2 = US2::US2_Text(v74.clone());
                        let v76: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v77: Rc<UH2> = Rc::new(UH2::UH2_Cons(v75.clone(), v76.clone()));
                        let v78: Rc<UH2> = Rc::new(UH2::UH2_Cons(v73.clone(), v77.clone()));
                        let v79: Rc<UH2> = Rc::new(UH2::UH2_Cons(v71.clone(), v78.clone()));
                        let v80: Rc<UH2> = Rc::new(UH2::UH2_Cons(v69.clone(), v79.clone()));
                        let v81: Rc<UH2> = Rc::new(UH2::UH2_Cons(v66.clone(), v80.clone()));
                        let v82: Rc<UH2> = Rc::new(UH2::UH2_Cons(v64.clone(), v81.clone()));
                        let v83: Rc<UH2> = Rc::new(UH2::UH2_Cons(v62.clone(), v82.clone()));
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(\"true\" if !"); } LIT.with(|lit| lit.clone()) };
                        let v85: US2 = US2::US2_Text(v84.clone());
                        let v86: US3 = US3::US3_WordLetters;
                        let v87: US2 = US2::US2_Run(v86.clone(), 1i32);
                        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" else \"false\")"); } LIT.with(|lit| lit.clone()) };
                        let v89: US2 = US2::US2_Text(v88.clone());
                        let v90: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v91: Rc<UH2> = Rc::new(UH2::UH2_Cons(v89.clone(), v90.clone()));
                        let v92: Rc<UH2> = Rc::new(UH2::UH2_Cons(v87.clone(), v91.clone()));
                        let v93: Rc<UH2> = Rc::new(UH2::UH2_Cons(v85.clone(), v92.clone()));
                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(if (!"); } LIT.with(|lit| lit.clone()) };
                        let v95: US2 = US2::US2_Text(v94.clone());
                        let v96: US3 = US3::US3_WordLetters;
                        let v97: US2 = US2::US2_Run(v96.clone(), 1i32);
                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") \"true\" else \"false\")"); } LIT.with(|lit| lit.clone()) };
                        let v99: US2 = US2::US2_Text(v98.clone());
                        let v100: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v101: Rc<UH2> = Rc::new(UH2::UH2_Cons(v99.clone(), v100.clone()));
                        let v102: Rc<UH2> = Rc::new(UH2::UH2_Cons(v97.clone(), v101.clone()));
                        let v103: Rc<UH2> = Rc::new(UH2::UH2_Cons(v95.clone(), v102.clone()));
                        let v104: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v105: Rc<UH1> = Rc::new(UH1::UH1_Cons(v103.clone(), v104.clone()));
                        let v106: Rc<UH1> = Rc::new(UH1::UH1_Cons(v93.clone(), v105.clone()));
                        Rc::new(UH1::UH1_Cons(v83.clone(), v106.clone()))
                    }
                    US1::US1_OneOpHostOperator => {
                        let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$0?\""); } LIT.with(|lit| lit.clone()) };
                        let v133: US2 = US2::US2_Text(v132.clone());
                        let v134: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v135: Rc<UH2> = Rc::new(UH2::UH2_Cons(v133.clone(), v134.clone()));
                        let v136: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v135.clone(), v136.clone()))
                    }
                    US1::US1_OneOpTernary => {
                        let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                        let v207: US2 = US2::US2_Text(v206.clone());
                        let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
                        let v209: US2 = US2::US2_Maybe(v208.clone());
                        let v210: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
                        let v211: US2 = US2::US2_Text(v210.clone());
                        let v212: US3 = US3::US3_WordLetters;
                        let v213: US2 = US2::US2_Run(v212.clone(), 1i32);
                        let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ?? undefined"); } LIT.with(|lit| lit.clone()) };
                        let v215: US2 = US2::US2_Text(v214.clone());
                        let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                        let v217: US2 = US2::US2_Maybe(v216.clone());
                        let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v219: US2 = US2::US2_Text(v218.clone());
                        let v220: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v221: Rc<UH2> = Rc::new(UH2::UH2_Cons(v219.clone(), v220.clone()));
                        let v222: Rc<UH2> = Rc::new(UH2::UH2_Cons(v217.clone(), v221.clone()));
                        let v223: Rc<UH2> = Rc::new(UH2::UH2_Cons(v215.clone(), v222.clone()));
                        let v224: Rc<UH2> = Rc::new(UH2::UH2_Cons(v213.clone(), v223.clone()));
                        let v225: Rc<UH2> = Rc::new(UH2::UH2_Cons(v211.clone(), v224.clone()));
                        let v226: Rc<UH2> = Rc::new(UH2::UH2_Cons(v209.clone(), v225.clone()));
                        let v227: Rc<UH2> = Rc::new(UH2::UH2_Cons(v207.clone(), v226.clone()));
                        let v228: US2 = US2::US2_Text(v206.clone());
                        let v229: US3 = US3::US3_Blanks;
                        let v230: US2 = US2::US2_Run(v229.clone(), 0i32);
                        let v231: US2 = US2::US2_Text(v208.clone());
                        let v232: US3 = US3::US3_Blanks;
                        let v233: US2 = US2::US2_Run(v232.clone(), 0i32);
                        let v234: US2 = US2::US2_Text(v210.clone());
                        let v235: US3 = US3::US3_WordLetters;
                        let v236: US2 = US2::US2_Run(v235.clone(), 1i32);
                        let v237: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ? "); } LIT.with(|lit| lit.clone()) };
                        let v238: US2 = US2::US2_Text(v237.clone());
                        let v239: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v240: US3 = US3::US3_NoneOf(v239.clone());
                        let v241: US2 = US2::US2_Run(v240.clone(), 1i32);
                        let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" : "); } LIT.with(|lit| lit.clone()) };
                        let v243: US2 = US2::US2_Text(v242.clone());
                        let v244: US3 = US3::US3_NoneOf(v216.clone());
                        let v245: US2 = US2::US2_Run(v244.clone(), 1i32);
                        let v246: US2 = US2::US2_Text(v216.clone());
                        let v247: US3 = US3::US3_Blanks;
                        let v248: US2 = US2::US2_Run(v247.clone(), 0i32);
                        let v249: US2 = US2::US2_Text(v218.clone());
                        let v250: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v251: Rc<UH2> = Rc::new(UH2::UH2_Cons(v249.clone(), v250.clone()));
                        let v252: Rc<UH2> = Rc::new(UH2::UH2_Cons(v248.clone(), v251.clone()));
                        let v253: Rc<UH2> = Rc::new(UH2::UH2_Cons(v246.clone(), v252.clone()));
                        let v254: Rc<UH2> = Rc::new(UH2::UH2_Cons(v245.clone(), v253.clone()));
                        let v255: Rc<UH2> = Rc::new(UH2::UH2_Cons(v243.clone(), v254.clone()));
                        let v256: Rc<UH2> = Rc::new(UH2::UH2_Cons(v241.clone(), v255.clone()));
                        let v257: Rc<UH2> = Rc::new(UH2::UH2_Cons(v238.clone(), v256.clone()));
                        let v258: Rc<UH2> = Rc::new(UH2::UH2_Cons(v236.clone(), v257.clone()));
                        let v259: Rc<UH2> = Rc::new(UH2::UH2_Cons(v234.clone(), v258.clone()));
                        let v260: Rc<UH2> = Rc::new(UH2::UH2_Cons(v233.clone(), v259.clone()));
                        let v261: Rc<UH2> = Rc::new(UH2::UH2_Cons(v231.clone(), v260.clone()));
                        let v262: Rc<UH2> = Rc::new(UH2::UH2_Cons(v230.clone(), v261.clone()));
                        let v263: Rc<UH2> = Rc::new(UH2::UH2_Cons(v228.clone(), v262.clone()));
                        let v264: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v265: Rc<UH1> = Rc::new(UH1::UH1_Cons(v263.clone(), v264.clone()));
                        Rc::new(UH1::UH1_Cons(v227.clone(), v265.clone()))
                    }
                    US1::US1_ScopingIdiom => {
                        let v5: US2 = US2::US2_Start;
                        let v6: US3 = US3::US3_Blanks;
                        let v7: US2 = US2::US2_Run(v6.clone(), 0i32);
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Gleam = \\ () => "); } LIT.with(|lit| lit.clone()) };
                        let v9: US2 = US2::US2_Maybe(v8.clone());
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'let "); } LIT.with(|lit| lit.clone()) };
                        let v11: US2 = US2::US2_Text(v10.clone());
                        let v12: US3 = US3::US3_WordLetters;
                        let v13: US2 = US2::US2_Run(v12.clone(), 1i32);
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = fn ()"); } LIT.with(|lit| lit.clone()) };
                        let v15: US2 = US2::US2_Text(v14.clone());
                        let v16: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v17: Rc<UH2> = Rc::new(UH2::UH2_Cons(v15.clone(), v16.clone()));
                        let v18: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13.clone(), v17.clone()));
                        let v19: Rc<UH2> = Rc::new(UH2::UH2_Cons(v11.clone(), v18.clone()));
                        let v20: Rc<UH2> = Rc::new(UH2::UH2_Cons(v9.clone(), v19.clone()));
                        let v21: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7.clone(), v20.clone()));
                        let v22: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5.clone(), v21.clone()));
                        let v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v22.clone(), v23.clone()))
                    }
                    US1::US1_ThunkOpener => {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'(fun () ->'"); } LIT.with(|lit| lit.clone()) };
                        let v51: US2 = US2::US2_Text(v50.clone());
                        let v52: US3 = US3::US3_Blanks;
                        let v53: US2 = US2::US2_Run(v52.clone(), 0i32);
                        let v54: US2 = US2::US2_End;
                        let v55: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        let v56: Rc<UH2> = Rc::new(UH2::UH2_Cons(v54.clone(), v55.clone()));
                        let v57: Rc<UH2> = Rc::new(UH2::UH2_Cons(v53.clone(), v56.clone()));
                        let v58: Rc<UH2> = Rc::new(UH2::UH2_Cons(v51.clone(), v57.clone()));
                        let v59: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        Rc::new(UH1::UH1_Cons(v58.clone(), v59.clone()))
                    }
                };
                let v296: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v297: Rc<UH1> = method2(v295.clone(), v296.clone());
                let v298: bool = found_5(v0.clone(), v297.clone());
                if v298 {
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
fn idiom_of_0(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v2: US1 = US1::US1_ScopingIdiom;
    let v3: US1 = US1::US1_HostCombinator;
    let v4: US1 = US1::US1_ThunkOpener;
    let v5: US1 = US1::US1_OneConstructTernary;
    let v6: US1 = US1::US1_OneCallParse;
    let v7: US1 = US1::US1_OneOpHostOperator;
    let v8: US1 = US1::US1_GleamRebindingBlock;
    let v9: US1 = US1::US1_HealedAlready;
    let v10: US1 = US1::US1_FsharpPipeMutate;
    let v11: US1 = US1::US1_LambdaArrow;
    let v12: US1 = US1::US1_OneOpTernary;
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
        let v1: US0 = idiom_of_0(v0.clone());
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
pub fn kit_lint_flow_idiom(v0: &str) -> bool {
    closure0()(Rc::<str>::from(v0))
}
