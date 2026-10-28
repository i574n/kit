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
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return -1i32;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
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
                (v0, v1) = (v0.clone(), v12);
                continue;
            }
        }
    }
}
fn filters_from_2(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v7, mut v8): (Rc<UH0>, Rc<str>) = filters_from_2(v0.clone(), v6.clone());
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
            let v31: bool = &*v30 == "!=";
            let v46: US1 = if v31 {
                US1::US1_WhereNotEqual
            } else {
                let v33: bool = &*v30 == ">";
                if v33 {
                    US1::US1_WhereGreater
                } else {
                    let v35: bool = &*v30 == ">=";
                    if v35 {
                        US1::US1_WhereGreaterEqual
                    } else {
                        let v37: bool = &*v30 == "<";
                        if v37 {
                            US1::US1_WhereLess
                        } else {
                            let v39: bool = &*v30 == "<=";
                            if v39 {
                                US1::US1_WhereLessEqual
                            } else {
                                US1::US1_WhereEqual
                            }
                        }
                    }
                }
            };
            let v47: i32 = v15.wrapping_sub(v11);
            let v48: i32 = v47.wrapping_sub(v23);
            let v49: usize = ((v28) as usize);
            let v50: i32 = v28.wrapping_add(v48);
            let v51: usize = ((v50) as usize);
            let v52: Rc<str> = Rc::<str>::from(&(v9)[v49..v51]);
            US2::US2_Some(v26.clone(), v46.clone(), v52.clone())
        };
        match &v54 {
            US2::US2_None => {
                (v7.clone(), v8.clone())
            }
            US2::US2_Some(v55, v56, v57) => {
                let v55: Rc<str> = v55.clone();
                let v56: US1 = v56.clone();
                let v57: Rc<str> = v57.clone();
                let v58: i32 = (v0).first().and_then(|header| header.iter().position(|item| &**item == &*v55)).map(|index| index as i32).unwrap_or(-1);
                let v59: bool = v58 < 0i32;
                if v59 {
                    let v60: bool = &*v55 == "";
                    let v66: Rc<str> = if v60 {
                        v8.clone()
                    } else {
                        let v61: bool = &*v8 == "";
                        if v61 {
                            v55.clone()
                        } else {
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = std::rc::Rc::<str>::from([&*v55, &*v62, &*v8, &*v63, &*v63].concat());
                            v64.clone()
                        }
                    };
                    (v7.clone(), v66.clone())
                } else {
                    let v67: Rc<UH0> = Rc::new(UH0::UH0_FilterThen(v58, v56.clone(), v57.clone(), v7.clone()));
                    (v67.clone(), v8.clone())
                }
            }
        }
    }
}
fn all_columns_4(mut v0: i32, mut v1: i32) -> Rc<UH1> {
    let v2: bool = v1 >= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: Rc<UH1> = all_columns_4(v0, v4);
        Rc::new(UH1::UH1_Cons(v1, v5.clone()))
    }
}
fn named_columns_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<UH1>, Rc<str>) {
    let v3: bool = &*v1 == "";
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        (v4.clone(), v2.clone())
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
                v10.clone()
            } else {
                let v12: bool = &*v10 == "";
                if v12 {
                    v2.clone()
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v14: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v5, &*v10, &*v13, &*v13].concat());
                    v14.clone()
                }
            }
        } else {
            v2.clone()
        };
        let (mut v18, mut v19): (Rc<UH1>, Rc<str>) = named_columns_5(v0.clone(), v8.clone(), v17.clone());
        if v9 {
            (v18.clone(), v19.clone())
        } else {
            let mut v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7, v18.clone()));
            (v20.clone(), v19.clone())
        }
    }
}
fn row_passes_7(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_FilterThen(v3, v4, v5, v6) => {
                let v3: i32 = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
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
                        match &v14 {
                            US3::US3_Some(v20) => {
                                let mut v20: f64 = *v20;
                                match &v19 {
                                    US3::US3_Some(v21) => {
                                        let v21: f64 = *v21;
                                        match &v4 {
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
                    (v0, v1, v2) = (v6.clone(), v1.clone(), v2);
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
fn grouped_rows_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32, mut v3: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v4: i32, mut v5: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v6: i32 = ((v0).len().max(1) - 1) as i32;
        let v7: bool = v4 > v6;
        if v7 {
            return v5.clone();
        } else {
            let v8: bool = row_passes_7(v1.clone(), v0.clone(), v4);
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
                    Rc::new(UH2::UH2_Cons(v10.clone(), v5.clone()))
                } else {
                    v5.clone()
                };
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v3.clone(), v9, v16.clone());
                continue;
            } else {
                let v18: i32 = v4.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1.clone(), v2, v3.clone(), v18, v5.clone());
                continue;
            }
        }
    }
}
fn method11(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn((char)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> i32> = Rc::new(move |mut v0: (char)| -> i32 {
        let mut v1: char = (v0);
        let v2: usize = v1.len_utf8();
        let v3: i32 = ((v2) as i32);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn text_order_from_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 >= v3;
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
                    let v12: Option<char> = method11(v11.clone());
                    let mut v13: Rc<dyn Fn((char)) -> u32> = closure1();
                    let mut v14: Option<u32> = v12.map(|x| v13(x));
                    let v16: u32 = v14.unwrap_or(0u32);
                    let v17: usize = ((v2) as usize);
                    let v18: Option<char> = (&*v1)[v17..].chars().next();
                    let v19: Option<char> = method11(v18.clone());
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
                            let v27: Option<char> = method11(v26.clone());
                            let mut v28: Rc<dyn Fn((char)) -> i32> = closure2();
                            let mut v29: Option<i32> = v27.map(|x| v28(x));
                            let v31: i32 = v29.unwrap_or(1i32);
                            let v32: i32 = v2.wrapping_add(v31);
                            (v0, v1, v2) = (v0.clone(), v1.clone(), v32);
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn ranked_9(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<str>, mut v2: Rc<UH2>) -> Rc<UH2> {
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
                    let v12: i32 = text_order_from_10(v1.clone(), v5.clone(), v11);
                    let mut v13: bool = v12 < 0i32;
                    v13
                } else {
                    false
                }
            };
            if v15 {
                Rc::new(UH2::UH2_Cons(v1.clone(), v2.clone()))
            } else {
                let v17: Rc<UH2> = ranked_9(v0.clone(), v1.clone(), v6.clone());
                Rc::new(UH2::UH2_Cons(v5.clone(), v17.clone()))
            }
        }
        UH2::UH2_Nil => {
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v1.clone(), v3.clone()))
        }
    }
}
fn method8(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                let v5: Rc<UH2> = ranked_9(v0.clone(), v3.clone(), v2.clone());
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v2.clone();
            }
        }
    }
}
fn printed_groups_12(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH2> = v5.clone();
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
                    (v0, v1, v2, v3) = (v0.clone(), v5.clone(), v2, v15);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method13(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH2> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                let v5: i32 = (v0).borrow().get(&*v3).copied().unwrap_or(-1);
                let v6: i32 = v2.wrapping_add(v5);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4) => {
            let v3: i32 = *v3;
            let v4: Rc<UH1> = v4.clone();
            let v5: Rc<UH2> = method15(v0.clone(), v4.clone(), v2.clone());
            let v6: Rc<str> = std::rc::Rc::<str>::from((v0).get(0i32 as usize).and_then(|cells| cells.get(v3 as usize)).map(|cell| &**cell).unwrap_or(""));
            Rc::new(UH2::UH2_Cons(v6.clone(), v5.clone()))
        }
        UH1::UH1_Nil => {
            v2.clone()
        }
    }
}
fn method16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn method18(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH1::UH1_Cons(v4, v5) => {
            let v4: i32 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH2> = method18(v0.clone(), v1, v5.clone(), v3.clone());
            let v7: Rc<str> = std::rc::Rc::<str>::from((v0).get(v1 as usize).and_then(|cells| cells.get(v4 as usize)).map(|cell| &**cell).unwrap_or(""));
            Rc::new(UH2::UH2_Cons(v7.clone(), v6.clone()))
        }
        UH1::UH1_Nil => {
            v3.clone()
        }
    }
}
fn listed_rows_17(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32) -> (i32, i32, i32) {
    let v8: i32 = ((v0).len().max(1) - 1) as i32;
    let v9: bool = v4 > v8;
    let (mut v38, mut v39, mut v40): (i32, i32, i32) = if v9 {
        (v5, v6, v7)
    } else {
        let mut v10: bool = row_passes_7(v1.clone(), v0.clone(), v4);
        let v11: bool = v10 == false;
        let (mut v35, mut v36, mut v37): (i32, i32, i32) = if v11 {
            let v12: i32 = v4.wrapping_add(1i32);
            let mut v13: i32 = v5.wrapping_add(1i32);
            let (mut v14, mut v15, mut v16): (i32, i32, i32) = listed_rows_17(v0.clone(), v1.clone(), v2.clone(), v3, v12, v13, v6, v7);
            (v14, v15, v16)
        } else {
            let v17: bool = (v7 as u64) < v3;
            if v17 {
                let v18: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH2> = method18(v0.clone(), v4, v2.clone(), v18.clone());
                let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v21: i32 = 0i32;
                let v22: i32 = method16(v20.clone(), v19.clone(), v21);
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
            let (mut v32, mut v33, mut v34): (i32, i32, i32) = listed_rows_17(v0.clone(), v1.clone(), v2.clone(), v3, v31, v27, v28, v30);
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
        US0::US0_TableUnreadable(v9.clone())
    } else {
        let v12: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US0::US0_TableLoaded(v12.clone())
    };
    match &v14 {
        US0::US0_TableLoaded(v19) => {
            let v19: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v19.clone();
            let (mut v20, mut v21): (Rc<UH0>, Rc<str>) = filters_from_2(v19.clone(), v1.clone());
            let v22: bool = &*v2 == "";
            let (mut v28, mut v29): (Rc<UH1>, Rc<str>) = if v22 {
                let v23: i32 = (v19).get(0i32 as usize).map_or(0, |r| r.len()) as i32;
                let v24: i32 = 0i32;
                let v25: Rc<UH1> = all_columns_4(v23, v24);
                (v25.clone(), v21.clone())
            } else {
                named_columns_5(v19.clone(), v2.clone(), v21.clone())
            };
            let v30: bool = &*v3 == "";
            let v32: i32 = if v30 {
                -1i32
            } else {
                let v31: i32 = (v19).first().and_then(|header| header.iter().position(|item| &**item == &*v3)).map(|index| index as i32).unwrap_or(-1);
                v31
            };
            let v33: bool = &*v3 != "";
            let v35: bool = if v33 {
                let v34: bool = v32 < 0i32;
                v34
            } else {
                false
            };
            let v42: Rc<str> = if v35 {
                let v36: bool = &*v29 == "";
                if v36 {
                    v3.clone()
                } else {
                    if v30 {
                        v29.clone()
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v37, &*v3, &*v38, &*v38].concat());
                        v39.clone()
                    }
                }
            } else {
                v29.clone()
            };
            let v43: bool = &*v42 != "";
            if v43 {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown columns: "); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v42));
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (header: "); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = std::rc::Rc::<str>::from((v19).first().map(|header| header.iter().map(|item| &**item).collect::<Vec<_>>().join(&*v48)).unwrap_or_default().as_str());
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v49));
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v55: Rc<str> = std::rc::Rc::<str>::from([&*v53, &*v52, &*v54, &*v54, &*v54].concat());
                eprintln!("{}", v55);
                2i32
            } else {
                let v56: bool = v32 >= 0i32;
                if v56 {
                    let v57: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<std::rc::Rc<str>, i32>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
                    let v58: i32 = 1i32;
                    let v59: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v60: Rc<UH2> = grouped_rows_6(v19.clone(), v20.clone(), v32, v57.clone(), v58, v59.clone());
                    let v61: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v62: Rc<UH2> = method8(v57.clone(), v60.clone(), v61.clone());
                    let mut v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                    v63.borrow_mut().push(v64);
                    v63.borrow_mut().push(v3);
                    let v65: Rc<Vec<Rc<str>>> = Rc::new(v63.borrow().clone());
                    let v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v65).as_ref().clone()));
                    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v68: Rc<str> = Rc::<str>::from((*v66.borrow()).join(&*v67));
                    println!("{}", v68);
                    let v69: i32 = 0i32;
                    let v70: i32 = printed_groups_12(v57.clone(), v62.clone(), v5, v69);
                    let v71: bool = v8 == false;
                    if v71 {
                        let v72: i32 = 0i32;
                        let v73: i32 = method13(v60.clone(), v72);
                        let v74: Rc<str> = std::rc::Rc::<str>::from((v73).to_string().as_str());
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" groups="); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v74));
                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
                        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                        let v79: Rc<str> = std::rc::Rc::<str>::from((v70).to_string().as_str());
                        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                        let v81: i32 = ((v19).len().max(1) - 1) as i32;
                        let v82: i32 = 0i32;
                        let v83: i32 = method14(v57.clone(), v60.clone(), v82);
                        let v84: Rc<str> = std::rc::Rc::<str>::from((v81).to_string().as_str());
                        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) };
                        let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v84));
                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
                        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
                        let v89: Rc<str> = std::rc::Rc::<str>::from((v83).to_string().as_str());
                        let v91: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v88, v89)), v80));
                        println!("{}", v91);
                        ()
                    };
                    0i32
                } else {
                    let v92: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v93: Rc<UH2> = method15(v19.clone(), v28.clone(), v92.clone());
                    let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v95: i32 = 0i32;
                    let v96: i32 = method16(v94.clone(), v93.clone(), v95);
                    let v97: Rc<Vec<Rc<str>>> = Rc::new(v94.borrow().clone());
                    let v98: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v97).as_ref().clone()));
                    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = Rc::<str>::from((*v98.borrow()).join(&*v99));
                    println!("{}", v100);
                    let v101: i32 = 1i32;
                    let v102: i32 = 0i32;
                    let v103: i32 = 0i32;
                    let v104: i32 = 0i32;
                    let (mut v105, mut v106, mut v107): (i32, i32, i32) = listed_rows_17(v19.clone(), v20.clone(), v28.clone(), v5, v101, v102, v103, v104);
                    let v108: bool = v8 == false;
                    if v108 {
                        let v109: Rc<str> = std::rc::Rc::<str>::from((v105).to_string().as_str());
                        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table query ok total="); } LIT.with(|lit| lit.clone()) };
                        let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v109));
                        let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched="); } LIT.with(|lit| lit.clone()) };
                        let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
                        let v114: Rc<str> = std::rc::Rc::<str>::from((v106).to_string().as_str());
                        let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v114));
                        let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) };
                        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v116));
                        let v118: Rc<str> = std::rc::Rc::<str>::from((v107).to_string().as_str());
                        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
                        println!("{}", v119);
                        ()
                    };
                    0i32
                }
            }
        }
        US0::US0_TableUnreadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v15, &*v17, &*v17, &*v17].concat());
            eprintln!("{}", v18);
            2i32
        }
    }
}
fn no_request_19() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v0.clone(), v0.clone(), v0.clone(), 20u64, v0.clone(), v0.clone(), false)
}
fn plain_entry_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> i32 {
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>, bool) = no_request_19();
    plain_query_1(v0, v1, v2, v3, v9, v4, v11, v12, v5)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool| -> i32 {
        plain_entry_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_plain(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64, v5: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
