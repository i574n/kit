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
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(f64),
    US3_None,
}
fn operator_from_2(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                let v10: bool = v4 == b'!';
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
fn filters_from_1(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoFilters); } CASE.with(|case| case.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v3.clone(), v4.clone())
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = std::rc::Rc::<str>::from((v1).split_once(&*v5).map(|(_, rest)| rest).unwrap_or(""));
        let (mut v7, mut v8): (Rc<UH0>, Rc<str>) = filters_from_1(v0.clone(), v6.clone());
        let v9: Rc<str> = std::rc::Rc::<str>::from((v1).split(&*v5).next().unwrap_or(""));
        let v10: i32 = 0i32;
        let v11: i32 = operator_from_2(v9.clone(), v10);
        let v12: bool = v11 < 0i32;
        let v54: US2 = if v12 {
            US2::US2_None
        } else {
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: i32 = (v9.clone().len() as i32);
            let v16: bool = v14 < v15;
            let mut v19: bool = if v16 {
                let v17: u8 = v9.clone().as_bytes()[v14 as usize];
                let v18: bool = v17 == b'=';
                v18
            } else {
                false
            };
            let v22: bool = if v19 {
                let v20: u8 = v9.clone().as_bytes()[v11 as usize];
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
            let v28: i32 = v11.wrapping_add(v23);
            let v29: usize = ((v28) as usize);
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
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
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
fn row_passes_4(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> bool {
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
                        let v10: bool = (v7).trim().parse::<f64>().is_ok();
                        let v14: US3 = if v10 {
                            let v11: f64 = (v7).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v11)
                        } else {
                            US3::US3_None
                        };
                        let v15: bool = (v5).trim().parse::<f64>().is_ok();
                        let mut v19: US3 = if v15 {
                            let v16: f64 = (v5).trim().parse::<f64>().unwrap_or(0.0);
                            US3::US3_Some(v16)
                        } else {
                            US3::US3_None
                        };
                        match &v14 {
                            US3::US3_Some(v20) => {
                                let v20: f64 = *v20;
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
fn kept_marks_3(mut v0: Rc<UH0>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v5: bool = v2 > v3;
        if v5 {
            return v4.clone();
        } else {
            let v6: i32 = v2.wrapping_add(1i32);
            let v7: bool = row_passes_4(v0.clone(), v1.clone(), v2);
            let v10: Rc<str> = if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            };
            let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v10.clone(), v4.clone()));
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v6, v3, v11.clone());
            continue;
        }
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn rows_kept_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let v2: Rc<str> = kit_table_text::kit_table_problem(&*v0);
    let v3: bool = &*v2 != "";
    let v7: US0 = if v3 {
        US0::US0_TableUnreadable(v2.clone())
    } else {
        let v5: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(kit_table_text::kit_table_tsv(&*v0).lines().map(|line| line.split(char::from(9u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
        US0::US0_TableLoaded(v5.clone())
    };
    match &v7 {
        US0::US0_TableLoaded(v9) => {
            let v9: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = v9.clone();
            let (mut v10, mut v11): (Rc<UH0>, Rc<str>) = filters_from_1(v9.clone(), v1.clone());
            let v12: bool = &*v11 != "";
            if v12 {
                (false, v11.clone())
            } else {
                let v13: i32 = 1i32;
                let v14: i32 = ((v9).len().max(1) - 1) as i32;
                let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v16: Rc<UH1> = kept_marks_3(v10.clone(), v9.clone(), v13, v14, v15.clone());
                let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<UH1> = method5(v16.clone(), v17.clone());
                let mut v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                v19.borrow_mut().push(v20);
                let v21: i32 = 1i32;
                let v22: i32 = method6(v19.clone(), v18.clone(), v21);
                let v23: Rc<Vec<Rc<str>>> = Rc::new(v19.borrow().clone());
                let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v23).as_ref().clone()));
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = Rc::<str>::from((*v24.borrow()).join(&*v25));
                (true, v26.clone())
            }
        }
        US0::US0_TableUnreadable(v8) => {
            let v8: Rc<str> = v8.clone();
            (false, v8.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        rows_kept_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_filter_kept(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
