#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_TableLoaded(std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>),
    US0_TableUnreadable(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_WhereEqual,
    US1_WhereNotEqual,
    US1_WhereGreater,
    US1_WhereGreaterEqual,
    US1_WhereLess,
    US1_WhereLessEqual,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoFilters,
    UH0_FilterThen(i32, US1, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, US1, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(i32, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(f64),
    US3_None,
}
fn operator_from_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return -1i32;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'<';
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'>';
                v6
            };
            let v9: bool = if v7 {
                true
            } else {
                let v8: bool = v4 == b'=';
                v8
            };
            let v11: bool = if v9 {
                true
            } else {
                let mut v10: bool = v4 == b'!';
                v10
            };
            if v11 {
                return v1;
            } else {
                let v12: i32 = v1.wrapping_add(1i32);
                v1 = v12;
                continue;
            }
        }
    }
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn filters_from_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let mut v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3, v4)
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v7, mut v8): (Rc<UH0>, Rc<str>) = filters_from_2(v0.clone(), v6);
        let v9: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let mut v10: i32 = 0i32;
        let v11: i32 = operator_from_3(v9.clone(), v10);
        let v12: bool = v11 < 0i32;
        let v54: US2 = if v12 {
            US2::US2_None
        } else {
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: i32 = (v9.clone().len() as i32);
            let v16: bool = v14 < v15;
            let v19: bool = if v16 {
                let v17: u8 = v9.clone().as_bytes()[v14 as usize];
                let v18: bool = v17 == b'=';
                v18
            } else {
                false
            };
            let v22: bool = if v19 {
                let mut v20: u8 = v9.clone().as_bytes()[v11 as usize];
                let v21: bool = v20 != b'=';
                v21
            } else {
                false
            };
            let v23: i32 = if v22 {
                2i32
            } else {
                1i32
            };
            let v24: usize = ((0i32) as usize);
            let v25: usize = ((v11) as usize);
            let v26: Rc<str> = Rc::<str>::from(&(v9)[v24..v25]);
            let v27: usize = ((v11) as usize);
            let mut v28: i32 = v11.wrapping_add(v23);
            let mut v29: usize = ((v28) as usize);
            let v30: Rc<str> = Rc::<str>::from(&(v9)[v27..v29]);
            let v31: i32 = v15.wrapping_sub(v11);
            let v32: i32 = v31.wrapping_sub(v23);
            let v33: usize = ((v28) as usize);
            let v34: i32 = v28.wrapping_add(v32);
            let v35: usize = ((v34) as usize);
            let v36: Rc<str> = Rc::<str>::from(&(v9)[v33..v35]);
            let v37: bool = &*v30 == "!=";
            let v52: US1 = if v37 {
                US1::US1_WhereNotEqual
            } else {
                let v39: bool = &*v30 == ">";
                if v39 {
                    US1::US1_WhereGreater
                } else {
                    let v41: bool = &*v30 == ">=";
                    if v41 {
                        US1::US1_WhereGreaterEqual
                    } else {
                        let v43: bool = &*v30 == "<";
                        if v43 {
                            US1::US1_WhereLess
                        } else {
                            let v45: bool = &*v30 == "<=";
                            if v45 {
                                US1::US1_WhereLessEqual
                            } else {
                                US1::US1_WhereEqual
                            }
                        }
                    }
                }
            };
            US2::US2_Some(v26, v52, v36)
        };
        match v54 {
            US2::US2_None => {
                (v7, v8)
            }
            US2::US2_Some(v55, v56, v57) => {
                let v55: Rc<str> = v55;
                let mut v56: US1 = v56;
                let v57: Rc<str> = v57;
                let v58: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v55)).map(|index| index as i32).unwrap_or(-1);
                let v59: bool = v58 < 0i32;
                if v59 {
                    let v60: bool = &*v55 == "";
                    let v66: Rc<str> = if v60 {
                        v8
                    } else {
                        let v61: bool = &*v8 == "";
                        if v61 {
                            v55
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            concat5_4(v55, v62, v8, v63.clone(), v63.clone())
                        }
                    };
                    (v7, v66)
                } else {
                    let v67: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v58, v56, v57, v7));
                    (v67, v8)
                }
            }
        }
    }
}
fn all_columns_5(mut v0: i32, mut v1: i32) -> Rc<UH1> {
    let mut v2: bool = v1 >= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: i32 = v1.wrapping_add(1i32);
        let v5: Rc<UH1> = all_columns_5(v0, v4);
        Rc::new(UH1::UH1_Cons(v1, v5))
    }
}
fn named_columns_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<UH1>, Rc<str>) {
    let v3: bool = &*v1 == "";
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        (v4, v2)
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v7: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v6)).map(|index| index as i32).unwrap_or(-1);
        let v8: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let v9: bool = v7 < 0i32;
        let v17: Rc<str> = if v9 {
            let mut v10: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
            let v11: bool = &*v2 == "";
            if v11 {
                v10
            } else {
                let v12: bool = &*v10 == "";
                if v12 {
                    v2
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_4(v2, v5, v10, v13.clone(), v13.clone())
                }
            }
        } else {
            v2
        };
        let (mut v18, mut v19): (Rc<UH1>, Rc<str>) = named_columns_6(v0, v8, v17);
        if v9 {
            (v18, v19)
        } else {
            let mut v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7, v18));
            (v20, v19)
        }
    }
}
fn row_passes_8(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let v3: i32 = v3;
                let mut v4: US1 = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<str> = std::rc::Rc::<str>::from((v1).get(v2 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v33: bool = match &v4 {
                    US1::US1_WhereEqual => {
                        let v8: bool = &*v7 == &*v5;
                        v8
                    }
                    US1::US1_WhereNotEqual => {
                        let v9: bool = &*v7 != &*v5;
                        v9
                    }
                    _ => {
                        let mut v10: bool = (v7).trim().parse::<f64>().is_ok();
                        let mut v14: US3 = if v10 {
                            let v11: f64 = (v7).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v11)
                        } else {
                            US3::US3_None
                        };
                        let v15: bool = (v5).trim().parse::<f64>().is_ok();
                        let v19: US3 = if v15 {
                            let v16: f64 = (v5).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v16)
                        } else {
                            US3::US3_None
                        };
                        match v14 {
                            US3::US3_Some(v20) => {
                                let mut v20: f64 = v20;
                                match v19 {
                                    US3::US3_Some(v21) => {
                                        let v21: f64 = v21;
                                        match v4 {
                                            US1::US1_WhereGreater => {
                                                let v22: bool = v20 > v21;
                                                v22
                                            }
                                            US1::US1_WhereGreaterEqual => {
                                                let v23: bool = v20 >= v21;
                                                v23
                                            }
                                            US1::US1_WhereLess => {
                                                let v24: bool = v20 < v21;
                                                v24
                                            }
                                            US1::US1_WhereLessEqual => {
                                                let v25: bool = v20 <= v21;
                                                v25
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    _ => {
                                        false
                                    }
                                }
                            }
                            _ => {
                                false
                            }
                        }
                    }
                };
                if v33 {
                    v0 = v6;
                    continue;
                } else {
                    return false;
                }
            }
            UH0::UH0_NoFilters => {
                return true;
            }
        }
    }
}
fn grouped_rows_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32, mut v3: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v4: i32, mut v5: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v6: i32 = ((v0).len().max(1) - 1) as i32;
        let v7: bool = v4 > v6;
        if v7 {
            return v5;
        } else {
            let v8: bool = row_passes_8(v1.clone(), v0.clone(), v4);
            if v8 {
                let v9: i32 = v4.wrapping_add(1i32);
                let mut v10: Rc<str> = std::rc::Rc::<str>::from((v0).get(v4 as usize).and_then(|cells| cells.get(v2 as usize)).map(|cell| &**cell).unwrap_or(""));
                let v11: i32 = (v3).borrow().get(&*v10).copied().unwrap_or(-1);
                let v12: bool = v11 < 0i32;
                let mut v14: i32 = if v12 {
                    1i32
                } else {
                    let mut v13: i32 = v11.wrapping_add(1i32);
                    v13
                };
                { (v3).borrow_mut().insert(v10.clone(), v14); };
                let v16: Rc<UH2> = if v12 {
                    Rc::new(UH2::UH2_Cons(v10, v5))
                } else {
                    v5
                };
                (v4, v5) = (v9, v16);
                continue;
            } else {
                let v18: i32 = v4.wrapping_add(1i32);
                v4 = v18;
                continue;
            }
        }
    }
}
fn method12(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let mut v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn((char)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> i32> = Rc::new(move |mut v0: (char)| -> i32 {
        let mut v1: char = (v0);
        let mut v2: usize = v1.len_utf8();
        let v3: i32 = ((v2) as i32);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn text_order_from_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v2 >= v3;
        let v7: bool = if v4 {
            let v5: i32 = (v1.clone().len() as i32);
            let v6: bool = v2 >= v5;
            v6
        } else {
            false
        };
        if v7 {
            return 0i32;
        } else {
            if v4 {
                return -1i32;
            } else {
                let v8: i32 = (v1.clone().len() as i32);
                let v9: bool = v2 >= v8;
                if v9 {
                    return 1i32;
                } else {
                    let mut v10: usize = ((v2) as usize);
                    let v11: Option<char> = (&*v0)[v10..].chars().next();
                    let v12: Option<char> = method12(v11);
                    let mut v13: Rc<dyn Fn((char)) -> u32> = closure1();
                    let mut v14: Option<u32> = v12.map(|x| v13(x));
                    let v16: u32 = v14.unwrap_or(0u32);
                    let v17: usize = ((v2) as usize);
                    let v18: Option<char> = (&*v1)[v17..].chars().next();
                    let v19: Option<char> = method12(v18);
                    let mut v20: Option<u32> = v19.map(|x| v13(x));
                    let v22: u32 = v20.unwrap_or(0u32);
                    let v23: bool = v16 < v22;
                    if v23 {
                        return -1i32;
                    } else {
                        let v24: bool = v16 > v22;
                        if v24 {
                            return 1i32;
                        } else {
                            let v25: usize = ((v2) as usize);
                            let v26: Option<char> = (&*v0)[v25..].chars().next();
                            let v27: Option<char> = method12(v26);
                            let mut v28: Rc<dyn Fn((char)) -> i32> = closure2();
                            let mut v29: Option<i32> = v27.map(|x| v28(x));
                            let v31: i32 = v29.unwrap_or(1i32);
                            let v32: i32 = v2.wrapping_add(v31);
                            v2 = v32;
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn ranked_10(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<str>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH2::UH2_Cons(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<UH2> = v6.clone();
            let v7: i32 = (v0).borrow().get(&*v1).copied().unwrap_or(-1);
            let v8: i32 = (v0).borrow().get(&*v5).copied().unwrap_or(-1);
            let v9: bool = v7 > v8;
            let v15: bool = if v9 {
                true
            } else {
                let mut v10: bool = v7 == v8;
                if v10 {
                    let v11: i32 = 0i32;
                    let v12: i32 = text_order_from_11(v1.clone(), v5.clone(), v11);
                    let mut v13: bool = v12 < 0i32;
                    v13
                } else {
                    false
                }
            };
            if v15 {
                Rc::new(UH2::UH2_Cons(v1, v2))
            } else {
                let v17: Rc<UH2> = ranked_10(v0, v1, v6);
                Rc::new(UH2::UH2_Cons(v5, v17))
            }
        }
        UH2::UH2_Nil => {
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v1, v3))
        }
    }
}
fn method9(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: Rc<UH2> = ranked_10(v0.clone(), v3, v2);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn printed_groups_13(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH2> = v5;
                let v6: bool = (v3 as u64) < v2;
                let v7: bool = v6 == false;
                if v7 {
                    return v3;
                } else {
                    let v8: i32 = (v0).borrow().get(&*v4).copied().unwrap_or(-1);
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string().as_str());
                    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    v10.borrow_mut().push(v9);
                    v10.borrow_mut().push(v4);
                    let v11: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
                    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
                    println!("{}", v14);
                    let v15: i32 = v3.wrapping_add(1i32);
                    (v1, v3) = (v5, v15);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method14(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH2> = v4;
                let v5: i32 = (v0).borrow().get(&*v3).copied().unwrap_or(-1);
                let v6: i32 = v2.wrapping_add(v5);
                (v1, v2) = (v4, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: i32 = v3;
            let mut v4: Rc<UH1> = v4;
            let v5: Rc<UH2> = method16(v0.clone(), v4, v2);
            let v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(0i32 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            Rc::new(UH2::UH2_Cons(v6, v5))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH2> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method19(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5) => {
            let mut v4: i32 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH2> = method19(v0.clone(), v1, v5, v3);
            let v7: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v4 as usize)).map(|cell| &**cell).unwrap_or(""));
            Rc::new(UH2::UH2_Cons(v7, v6))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn listed_rows_18(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32) -> (i32, i32, i32) {
    let v8: i32 = ((v0).len().max(1) - 1) as i32;
    let v9: bool = v4 > v8;
    let (mut v38, mut v39, mut v40): (i32, i32, i32) = if v9 {
        (v5, v6, v7)
    } else {
        let mut v10: bool = row_passes_8(v1.clone(), v0.clone(), v4);
        let v11: bool = v10 == false;
        let (mut v35, mut v36, mut v37): (i32, i32, i32) = if v11 {
            let v12: i32 = v4.wrapping_add(1i32);
            let mut v13: i32 = v5.wrapping_add(1i32);
            let (mut v14, mut v15, mut v16): (i32, i32, i32) = listed_rows_18(v0, v1, v2, v3, v12, v13, v6, v7);
            (v14, v15, v16)
        } else {
            let v17: bool = (v7 as u64) < v3;
            if v17 {
                let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH2> = method19(v0.clone(), v4, v2.clone(), v18);
                let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v21: i32 = 0i32;
                let v22: i32 = method17(v20.clone(), v19, v21);
                let v23: Rc<Vec<Rc<str>>> = Rc::new(v20.borrow().clone());
                let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v23).as_ref().clone()));
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = Rc::<str>::from((*v24.borrow()).join(&*v25));
                println!("{}", v26);
                ()
            };
            let v27: i32 = v5.wrapping_add(1i32);
            let mut v28: i32 = v6.wrapping_add(1i32);
            let mut v29: i32 = if v17 {
                1i32
            } else {
                0i32
            };
            let v30: i32 = v7.wrapping_add(v29);
            let v31: i32 = v4.wrapping_add(1i32);
            let (mut v32, mut v33, mut v34): (i32, i32, i32) = listed_rows_18(v0, v1, v2, v3, v31, v27, v28, v30);
            (v32, v33, v34)
        };
        (v35, v36, v37)
    };
    (v38, v39, v40)
}
fn plain_query_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> i32 {
    let v9: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let mut v10: bool = &*v9 != "";
    let mut v14: US0 = if v10 {
        US0::US0_TableUnreadable(v9)
    } else {
        let v12: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US0::US0_TableLoaded(v12)
    };
    match v14 {
        US0::US0_TableLoaded(v17) => {
            let v17: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v17;
            let (mut v18, mut v19): (Rc<UH0>, Rc<str>) = filters_from_2(v17.clone(), v1);
            let mut v20: bool = &*v2 == "";
            let (mut v26, mut v27): (Rc<UH1>, Rc<str>) = if v20 {
                let v21: i32 = (v17).get(0i32 as usize).map_or(0, |r| r.len()) as i32;
                let v22: i32 = 0i32;
                let v23: Rc<UH1> = all_columns_5(v21, v22);
                (v23, v19)
            } else {
                named_columns_6(v17.clone(), v2, v19)
            };
            let mut v28: bool = &*v3 == "";
            let v30: i32 = if v28 {
                -1i32
            } else {
                let mut v29: i32 = (v17).first().and_then(|header| header.iter().position(|item| &**item == &*v3)).map(|index| index as i32).unwrap_or(-1);
                v29
            };
            let v31: bool = &*v3 != "";
            let v33: bool = if v31 {
                let v32: bool = v30 < 0i32;
                v32
            } else {
                false
            };
            let v40: Rc<str> = if v33 {
                let v34: bool = &*v27 == "";
                if v34 {
                    v3.clone()
                } else {
                    if v28 {
                        v27
                    } else {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        concat5_4(v27, v35, v3.clone(), v36.clone(), v36.clone())
                    }
                }
            } else {
                v27
            };
            let v41: bool = &*v40 != "";
            if v41 {
                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) }, v40));
                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) }));
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = std::rc::Rc::<str>::from((v17).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v44)).unwrap_or_default().as_str());
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v45));
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v47));
                eprintln!("{}", v48);
                2i32
            } else {
                let v49: bool = v30 >= 0i32;
                if v49 {
                    let v50: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
                    let v51: i32 = 1i32;
                    let v52: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v53: Rc<UH2> = grouped_rows_7(v17.clone(), v18, v30, v50.clone(), v51, v52);
                    let v54: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v55: Rc<UH2> = method9(v50.clone(), v53.clone(), v54);
                    let mut v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v56.borrow_mut().push(v57);
                    v56.borrow_mut().push(v3);
                    let v58: Rc<Vec<Rc<str>>> = Rc::new(v56.borrow().clone());
                    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v58).as_ref().clone()));
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v61: Rc<str> = Rc::<str>::from((*v59.borrow()).join(&*v60));
                    println!("{}", v61);
                    let v62: i32 = 0i32;
                    let v63: i32 = printed_groups_13(v50.clone(), v55, v5, v62);
                    let v64: bool = v8 == false;
                    if v64 {
                        let v65: i32 = 0i32;
                        let v66: i32 = method14(v53.clone(), v65);
                        let v67: Rc<str> = std::rc::Rc::<str>::from((v66).to_string().as_str());
                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) }, v67));
                        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) }));
                        let v70: Rc<str> = std::rc::Rc::<str>::from((v63).to_string().as_str());
                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                        let v72: i32 = ((v17).len().max(1) - 1) as i32;
                        let v73: i32 = 0i32;
                        let v74: i32 = method15(v50, v53, v73);
                        let v75: Rc<str> = std::rc::Rc::<str>::from((v72).to_string().as_str());
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) }, v75));
                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) }));
                        let v78: Rc<str> = std::rc::Rc::<str>::from((v74).to_string().as_str());
                        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
                        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v71));
                        println!("{}", v80);
                        ()
                    };
                    0i32
                } else {
                    let v81: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v82: Rc<UH2> = method16(v17.clone(), v26.clone(), v81);
                    let v83: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v84: i32 = 0i32;
                    let v85: i32 = method17(v83.clone(), v82, v84);
                    let v86: Rc<Vec<Rc<str>>> = Rc::new(v83.borrow().clone());
                    let v87: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v86).as_ref().clone()));
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = Rc::<str>::from((*v87.borrow()).join(&*v88));
                    println!("{}", v89);
                    let v90: i32 = 1i32;
                    let v91: i32 = 0i32;
                    let v92: i32 = 0i32;
                    let v93: i32 = 0i32;
                    let (mut v94, mut v95, mut v96): (i32, i32, i32) = listed_rows_18(v17, v18, v26, v5, v90, v91, v92, v93);
                    let v97: bool = v8 == false;
                    if v97 {
                        let v98: Rc<str> = std::rc::Rc::<str>::from((v94).to_string().as_str());
                        let v99: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) }, v98));
                        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) }));
                        let v101: Rc<str> = std::rc::Rc::<str>::from((v95).to_string().as_str());
                        let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
                        let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) }));
                        let v104: Rc<str> = std::rc::Rc::<str>::from((v96).to_string().as_str());
                        let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v104));
                        println!("{}", v105);
                        ()
                    };
                    0i32
                }
            }
        }
        US0::US0_TableUnreadable(v15) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) }, v15));
            eprintln!("{}", v16);
            2i32
        }
    }
}
fn no_request_20() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0, false)
}
fn plain_entry_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_20();
    plain_query_1(v0, v1, v2, v3, v9, v4, v11, v12, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool| -> i32 {
        plain_entry_0(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_plain(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64, v5: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
