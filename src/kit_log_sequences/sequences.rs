#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_AsRows,
    US1_Modulo(u64),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, u64, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, u64, u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Before,
    US5_After,
    US5_Missing,
}
#[derive(Clone)]
enum US4 {
    US4_GapMs(u64),
    US4_CountIn(u64),
    US4_Verdict(US5),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(u64, US4, Rc<UH1>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US6 {
    US6_Some(regex::Regex),
    US6_None,
}
fn method2(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn events_of_1(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v7: i32 = (v6.clone().borrow().len() as i32);
            let v8: bool = v7 < 3i32;
            let v65: US2 = if v8 {
                US2::US2_None
            } else {
                let v10: Rc<RefCell<Vec<Rc<str>>>> = v6.clone() ;
                let v11: i32 = (v10.clone().borrow().len() as i32);
                let v12: bool = 0i32 < v11;
                let v16: US0 = if v12 {
                    let mut v13: Rc<str> = v10.clone().borrow()[0i32 as usize].clone();
                    US0::US0_Some(v13.clone())
                } else {
                    US0::US0_None
                };
                let v20: Rc<str> = match &v16 {
                    US0::US0_None => {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    }
                    US0::US0_Some(v17) => {
                        let v17: Rc<str> = v17.clone();
                        v17.clone()
                    }
                };
                let v21: Rc<RefCell<Vec<Rc<str>>>> = v6.clone() ;
                let v22: i32 = (v21.clone().borrow().len() as i32);
                let mut v23: bool = 1i32 < v22;
                let v27: US0 = if v23 {
                    let mut v24: Rc<str> = v21.clone().borrow()[1i32 as usize].clone();
                    US0::US0_Some(v24.clone())
                } else {
                    US0::US0_None
                };
                let v31: Rc<str> = match &v27 {
                    US0::US0_None => {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v29.clone()
                    }
                    US0::US0_Some(v28) => {
                        let v28: Rc<str> = v28.clone();
                        v28.clone()
                    }
                };
                let v32: Option<u64> = (v31).trim().parse::<u64>().ok();
                let v33: Option<u64> = method2(v32.clone());
                let mut v34: Rc<dyn Fn((u64)) -> US3> = closure1();
                let mut v35: Option<US3> = v33.map(|x| v34(x));
                let v37: US3 = US3::US3_None;
                let v38: US3 = v37.clone();
                let v39: US3 = v35.unwrap_or(v38);
                let v42: u64 = match &v39 {
                    US3::US3_None => {
                        0u64
                    }
                    US3::US3_Some(v40) => {
                        let v40: u64 = *v40;
                        v40
                    }
                };
                let v43: Rc<RefCell<Vec<Rc<str>>>> = v6 ;
                let v44: i32 = (v43.clone().borrow().len() as i32);
                let v45: bool = 2i32 < v44;
                let v49: US0 = if v45 {
                    let v46: Rc<str> = v43.clone().borrow()[2i32 as usize].clone();
                    US0::US0_Some(v46.clone())
                } else {
                    US0::US0_None
                };
                let v53: Rc<str> = match &v49 {
                    US0::US0_None => {
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v51.clone()
                    }
                    US0::US0_Some(v50) => {
                        let v50: Rc<str> = v50.clone();
                        v50.clone()
                    }
                };
                let v54: Option<u64> = (v53).trim().parse::<u64>().ok();
                let v55: Option<u64> = method2(v54.clone());
                let mut v56: Option<US3> = v55.map(|x| v34(x));
                let v58: US3 = US3::US3_None;
                let v59: US3 = v58.clone();
                let v60: US3 = v56.unwrap_or(v59);
                let v63: u64 = match &v60 {
                    US3::US3_None => {
                        0u64
                    }
                    US3::US3_Some(v61) => {
                        let v61: u64 = *v61;
                        v61
                    }
                };
                US2::US2_Some(v20.clone(), v42, v63)
            };
            match &v65 {
                US2::US2_None => {
                    let v72: i32 = v1.wrapping_sub(1i32);
                    (v0, v1, v2) = (v0.clone(), v72, v2.clone());
                    continue;
                }
                US2::US2_Some(v66, v67, v68) => {
                    let v66: Rc<str> = v66.clone();
                    let v67: u64 = *v67;
                    let v68: u64 = *v68;
                    let v69: i32 = v1.wrapping_sub(1i32);
                    let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v66.clone(), v67, v68, v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v69, v70.clone());
                    continue;
                }
            }
        }
    }
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: u64 = *v2;
                let v3: US4 = v3.clone();
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
fn interval_rows_3(mut v0: Rc<str>, mut v1: US0, mut v2: US1, mut v3: Rc<UH0>, mut v4: US2, mut v5: bool, mut v6: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v9, v10, v11, v12) => {
                let v9: Rc<str> = v9.clone();
                let v10: u64 = *v10;
                let v11: u64 = *v11;
                let v12: Rc<UH0> = v12.clone();
                let mut v13: bool = &*v9 == &*v0;
                if v13 {
                    let v16: bool = match &v1 {
                        US0::US0_None => {
                            true
                        }
                        US0::US0_Some(v14) => {
                            let v14: Rc<str> = v14.clone();
                            v5
                        }
                    };
                    let v33: Rc<UH1> = match &v4 {
                        US2::US2_None => {
                            v6.clone()
                        }
                        US2::US2_Some(v17, v18, v19) => {
                            let v17: Rc<str> = v17.clone();
                            let v18: u64 = *v18;
                            let v19: u64 = *v19;
                            if v16 {
                                let v20: bool = v11 > v19;
                                let v22: u64 = if v20 {
                                    let v21: u64 = v11.wrapping_sub(v19);
                                    v21
                                } else {
                                    0u64
                                };
                                let v28: u64 = match &v2 {
                                    US1::US1_AsRows => {
                                        v22
                                    }
                                    US1::US1_Modulo(v23) => {
                                        let mut v23: u64 = *v23;
                                        let mut v24: bool = v23 == 0u64;
                                        if v24 {
                                            v22
                                        } else {
                                            let mut v25: u64 = v22.wrapping_rem(v23);
                                            v25
                                        }
                                    }
                                };
                                let v29: US4 = US4::US4_GapMs(v28);
                                Rc::new(UH1::UH1_Cons(v10, v29.clone(), v6.clone()))
                            } else {
                                v6.clone()
                            }
                        }
                    };
                    let mut v34: US2 = US2::US2_Some(v9.clone(), v10, v11);
                    let mut v35: bool = false;
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v12.clone(), v34.clone(), v35, v33.clone());
                    continue;
                } else {
                    let v41: bool = if v5 {
                        true
                    } else {
                        match &v1 {
                            US0::US0_None => {
                                false
                            }
                            US0::US0_Some(v37) => {
                                let v37: Rc<str> = v37.clone();
                                let v38: bool = &*v37 == &*v9;
                                v38
                            }
                        }
                    };
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v12.clone(), v4.clone(), v41, v6.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method4(v6.clone(), v7.clone());
            }
        }
    }
}
fn method7(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn rows_text_5(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: u64 = *v3;
                let v4: US4 = v4.clone();
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                let v9: Rc<str> = format_real_6(v3);
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v7));
                let mut v25: Rc<str> = match &v4 {
                    US4::US4_CountIn(v14) => {
                        let v14: u64 = *v14;
                        format_real_6(v14)
                    }
                    US4::US4_GapMs(v12) => {
                        let v12: u64 = *v12;
                        format_real_6(v12)
                    }
                    US4::US4_Verdict(v16) => {
                        let v16: US5 = v16.clone();
                        match &v16 {
                            US5::US5_After => {
                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                                v18.clone()
                            }
                            US5::US5_Before => {
                                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("before"); } LIT.with(|lit| lit.clone()) };
                                v17.clone()
                            }
                            US5::US5_Missing => {
                                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing"); } LIT.with(|lit| lit.clone()) };
                                v19.clone()
                            }
                        }
                    }
                };
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v25));
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                (v0, v1, v2) = (v0.clone(), v5.clone(), v28.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v2.clone();
            }
        }
    }
}
fn interval_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    let v5: bool = &*v3 == "";
    let v8: US0 = if v5 {
        US0::US0_None
    } else {
        US0::US0_Some(v3.clone())
    };
    let v9: bool = v4 == 0u64;
    let v12: US1 = if v9 {
        US1::US1_AsRows
    } else {
        US1::US1_Modulo(v4)
    };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v13).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v15: i32 = (v14.clone().borrow().len() as i32);
    let v16: i32 = v15.wrapping_sub(1i32);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = events_of_1(v14.clone(), v16, v17.clone());
    let v19: US2 = US2::US2_None;
    let v20: bool = false;
    let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v22: Rc<UH1> = interval_rows_3(v2.clone(), v8.clone(), v12.clone(), v18.clone(), v19.clone(), v20, v21.clone());
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = rows_text_5(v0.clone(), v22.clone(), v23.clone());
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#unpaired\t"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: u64 = 0u64;
    let v28: Rc<str> = format_real_6(v27);
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v28));
    v29.clone()
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64| -> Rc<str> {
        interval_text_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn count_within_10(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<UH0>, mut v4: u64) -> u64 {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let v6: u64 = *v6;
                let v7: u64 = *v7;
                let v8: Rc<UH0> = v8.clone();
                let v9: bool = &*v5 == &*v0;
                let v11: bool = if v9 {
                    let v10: bool = v7 >= v1;
                    v10
                } else {
                    false
                };
                let mut v13: bool = if v11 {
                    let v12: bool = v7 <= v2;
                    v12
                } else {
                    false
                };
                let v15: u64 = if v13 {
                    let v14: u64 = v4.wrapping_add(1u64);
                    v14
                } else {
                    v4
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2, v8.clone(), v15);
                continue;
            }
            UH0::UH0_Nil => {
                return v4;
            }
        }
    }
}
fn window_rows_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v5 {
            UH0::UH0_Cons(v9, v10, v11, v12) => {
                let v9: Rc<str> = v9.clone();
                let v10: u64 = *v10;
                let v11: u64 = *v11;
                let v12: Rc<UH0> = v12.clone();
                let mut v13: bool = &*v9 == &*v1;
                if v13 {
                    let v14: u64 = v11.wrapping_add(v2);
                    let v15: u64 = v11.wrapping_add(v3);
                    let v16: u64 = 0u64;
                    let v17: u64 = count_within_10(v0.clone(), v14, v15, v4.clone(), v16);
                    let v18: US4 = US4::US4_CountIn(v17);
                    let v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v10, v18.clone(), v6.clone()));
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v12.clone(), v19.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2, v3, v4.clone(), v12.clone(), v6.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method4(v6.clone(), v7.clone());
            }
        }
    }
}
fn window_text_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v8: i32 = (v7.clone().borrow().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = events_of_1(v7.clone(), v9, v10.clone());
    let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v13: Rc<UH1> = window_rows_9(v2.clone(), v3.clone(), v4, v5, v11.clone(), v11.clone(), v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = rows_text_5(v0.clone(), v13.clone(), v14.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#unpaired\t"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: u64 = 0u64;
    let v19: Rc<str> = format_real_6(v18);
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v19));
    v20.clone()
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: u64| -> Rc<str> {
        window_text_8(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn first_line_12(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US3 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3.clone();
                let v4: u64 = *v4;
                let v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let v7: bool = &*v3 == &*v0;
                if v7 {
                    return US3::US3_Some(v4);
                } else {
                    (v0, v1) = (v0.clone(), v6.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US3::US3_None;
            }
        }
    }
}
fn order_text_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = (v5.clone().borrow().len() as i32);
    let v7: i32 = v6.wrapping_sub(1i32);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = events_of_1(v5.clone(), v7, v8.clone());
    let v10: US3 = first_line_12(v2.clone(), v9.clone());
    let v11: US3 = first_line_12(v3.clone(), v9.clone());
    let (mut v29, mut v30): (u64, US4) = match &v10 {
        US3::US3_Some(v12) => {
            let v12: u64 = *v12;
            match &v11 {
                US3::US3_Some(v13) => {
                    let mut v13: u64 = *v13;
                    let v14: bool = v12 < v13;
                    let v17: US5 = if v14 {
                        US5::US5_Before
                    } else {
                        US5::US5_After
                    };
                    let v18: US4 = US4::US4_Verdict(v17.clone());
                    (v12, v18.clone())
                }
                _ => {
                    let (mut v21, mut v22): (u64, US4) = match &v11 {
                        US3::US3_None => {
                            let v19: US5 = US5::US5_Missing;
                            let v20: US4 = US4::US4_Verdict(v19.clone());
                            (v12, v20.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v21, v22.clone())
                }
            }
        }
        _ => {
            let (mut v27, mut v28): (u64, US4) = match &v10 {
                US3::US3_None => {
                    let mut v25: US5 = US5::US5_Missing;
                    let v26: US4 = US4::US4_Verdict(v25.clone());
                    (0u64, v26.clone())
                }
                _ => unreachable!(),
            };
            (v27, v28.clone())
        }
    };
    let v31: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v29, v30.clone(), v31.clone()));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v34: Rc<str> = rows_text_5(v0.clone(), v32.clone(), v33.clone());
    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#unpaired\t"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let v37: u64 = 0u64;
    let v38: Rc<str> = format_real_6(v37);
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v38));
    v39.clone()
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        order_text_11(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn pending_count_15(mut v0: Rc<UH0>, mut v1: u64) -> u64 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: u64 = *v3;
                let v4: u64 = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v6: u64 = v1.wrapping_add(1u64);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: u64 = *v3;
                let v4: u64 = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3, v4, v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn paired_17(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let v6: u64 = *v6;
                let v7: u64 = *v7;
                let v8: Rc<UH0> = v8.clone();
                let v9: bool = v2 > v7;
                let v11: u64 = if v9 {
                    let v10: u64 = v2.wrapping_sub(v7);
                    v10
                } else {
                    0u64
                };
                let v12: US4 = US4::US4_GapMs(v11);
                let mut v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v12.clone(), v4.clone()));
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2, v8.clone(), v13.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v4.clone();
            }
        }
    }
}
fn latency_rows_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> (Rc<UH1>, u64) {
    let (mut v32, mut v33): (Rc<UH1>, u64) = match &*v2 {
        UH0::UH0_Nil => {
            let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH1> = method4(v4.clone(), v5.clone());
            let v7: u64 = 0u64;
            let v8: u64 = pending_count_15(v3.clone(), v7);
            (v6.clone(), v8)
        }
        _ => {
            let (mut v30, mut v31): (Rc<UH1>, u64) = match &*v2 {
                UH0::UH0_Cons(v9, v10, v11, v12) => {
                    let v9: Rc<str> = v9.clone();
                    let v10: u64 = *v10;
                    let v11: u64 = *v11;
                    let v12: Rc<UH0> = v12.clone();
                    let mut v13: bool = &*v9 == &*v0;
                    if v13 {
                        let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v10, v11, v3.clone()));
                        let (mut v15, mut v16): (Rc<UH1>, u64) = latency_rows_14(v0.clone(), v1.clone(), v12.clone(), v14.clone(), v4.clone());
                        (v15.clone(), v16)
                    } else {
                        let v17: bool = &*v9 == &*v1;
                        let (mut v26, mut v27): (Rc<UH1>, u64) = if v17 {
                            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v20: Rc<UH0> = method16(v3.clone(), v19.clone());
                            let v21: Rc<UH1> = paired_17(v9.clone(), v10, v11, v20.clone(), v4.clone());
                            let (mut v22, mut v23): (Rc<UH1>, u64) = latency_rows_14(v0.clone(), v1.clone(), v12.clone(), v18.clone(), v21.clone());
                            (v22.clone(), v23)
                        } else {
                            let (mut v24, mut v25): (Rc<UH1>, u64) = latency_rows_14(v0.clone(), v1.clone(), v12.clone(), v3.clone(), v4.clone());
                            (v24.clone(), v25)
                        };
                        (v26.clone(), v27)
                    }
                }
                _ => unreachable!(),
            };
            (v30.clone(), v31)
        }
    };
    (v32.clone(), v33)
}
fn latency_text_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = (v5.clone().borrow().len() as i32);
    let v7: i32 = v6.wrapping_sub(1i32);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = events_of_1(v5.clone(), v7, v8.clone());
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v12, mut v13): (Rc<UH1>, u64) = latency_rows_14(v2.clone(), v3.clone(), v9.clone(), v10.clone(), v11.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = rows_text_5(v0.clone(), v12.clone(), v14.clone());
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#unpaired\t"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = format_real_6(v13);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    v19.clone()
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        latency_text_13(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method22(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    (v0, v1)
}
fn regex_captures_21(mut v0: Rc<str>, mut v1: regex::Regex) -> Vec<std::collections::HashMap<Rc<str>, Rc<str>>> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let v7: bool = true; let _capture_move = (move || { //;
    let v9: Box<std::string::String> = Box::new(v5);
    let v11: &'static mut std::string::String = Box::leak(v9);
    let mut v13: regex::Regex = v1.clone();
    let v15: regex::CaptureMatches = v13.captures_iter(v11);
    let v17: bool = true; let _regex_captures : Vec<_> = v15.map(|x| { //;
    let v19: regex::Captures<'static> = x;
    let v21: Box<regex::Captures<'static>> = Box::new(v19);
    let mut v23: &'static mut regex::Captures<'static> = Box::leak(v21);
    let mut v25: regex::CaptureNames = v1.capture_names();
    let v27: bool = true; let _regex_captures : std::collections::HashMap<_, _> = v25.map(|x| { //;
    let v29: Rc<str> = x.unwrap_or("").to_string().into();
    let v31: Rc<str> = v29.clone();
    let v33: Rc<str> = v23.name(&v29).map(|x| x.as_str()).unwrap_or("").to_string().into();
    let (mut v34, mut v35): (Rc<str>, Rc<str>) = method22(v31.clone(), v33.clone());
    let v36: (Rc<str>, Rc<str>) = (v34, v35);
    let v38: bool = true; v36 }).collect();
    let v40: std::collections::HashMap<Rc<str>, Rc<str>> = _regex_captures;
    let v42: bool = true; v40 }).collect::<Vec<_>>();
    let v44: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _regex_captures;
    let v46: bool = true; v44 })();
    let v48: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _capture_move;
    v48.clone()
}
fn method23() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ms"); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn stamp_in_20(mut v0: Rc<str>, mut v1: regex::Regex) -> u64 {
    let mut v2: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_21(v0.clone(), v1.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: bool = true; let _vec_fold_ = v2.into_iter().fold(v3, |acc, x| { //;
    let v7: Rc<str> = acc;
    let v9: std::collections::HashMap<Rc<str>, Rc<str>> = x;
    let v10: bool = &*v7 != "";
    let v17: Rc<str> = if v10 {
        v7.clone()
    } else {
        let v11: Rc<str> = method23();
        let mut v13: Option<Rc<str>> = std::collections::HashMap::get(&v9, &v11).map(|x| x).cloned();
        let v15: Rc<str> = v7.clone();
        let v16: Rc<str> = v13.unwrap_or(v15);
        v16.clone()
    };
    let v19: bool = true; v17 });
    let v21: Rc<str> = _vec_fold_;
    let v22: Option<u64> = (v21).trim().parse::<u64>().ok();
    let mut v23: Option<u64> = method2(v22.clone());
    let mut v24: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v25: Option<US3> = v23.map(|x| v24(x));
    let v27: US3 = US3::US3_None;
    let v28: US3 = v27.clone();
    let v29: US3 = v25.unwrap_or(v28);
    match &v29 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v30) => {
            let v30: u64 = *v30;
            v30
        }
    }
}
fn matches_in_24(mut v0: Rc<str>, mut v1: regex::Regex) -> u64 {
    let mut v2: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_21(v0, v1);
    let v3: usize = ((v2).len() as usize);
    let v4: u64 = (v3 as u64);
    v4
}
fn event_row_25(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
    let v5: Rc<str> = format_real_6(v1);
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v5)), v3));
    let v8: Rc<str> = format_real_6(v2);
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    v11.clone()
}
fn events_in_19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: regex::Regex, mut v3: regex::Regex, mut v4: US6, mut v5: Rc<str>) -> Rc<str> {
    loop {
        let v6: i32 = (v0.clone().borrow().len() as i32);
        let v7: bool = v1 >= v6;
        if v7 {
            return v5.clone();
        } else {
            let v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v9: u64 = stamp_in_20(v8.clone(), v2.clone());
            let v10: i32 = v1.wrapping_add(1i32);
            let v11: u64 = (v10 as u64);
            let v12: u64 = matches_in_24(v8.clone(), v3.clone());
            let mut v13: bool = v12 > 0u64;
            let v17: Rc<str> = if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = event_row_25(v14.clone(), v11, v9);
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v15));
                v16.clone()
            } else {
                v5.clone()
            };
            let v26: Rc<str> = match &v4 {
                US6::US6_None => {
                    v17.clone()
                }
                US6::US6_Some(v18) => {
                    let v18: regex::Regex = v18.clone();
                    let v19: u64 = matches_in_24(v8.clone(), v18.clone());
                    let v20: bool = v19 > 0u64;
                    if v20 {
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("second"); } LIT.with(|lit| lit.clone()) };
                        let v22: Rc<str> = event_row_25(v21.clone(), v11, v9);
                        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v22));
                        v23.clone()
                    } else {
                        v17.clone()
                    }
                }
            };
            (v0, v1, v2, v3, v4, v5) = (v0.clone(), v10, v2.clone(), v3.clone(), v4.clone(), v26.clone());
            continue;
        }
    }
}
fn compiled_26(mut v0: Rc<str>) -> regex::Regex {
    let mut v2: Result<regex::Regex, regex::Error> = regex::Regex::new(&v0);
    let v4: regex::Regex = v2.unwrap();
    v4
}
fn events_text_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: regex::Regex = compiled_26(v1.clone());
    let v8: regex::Regex = compiled_26(v2.clone());
    let v9: bool = &*v3 == "";
    let mut v13: US6 = if v9 {
        US6::US6_None
    } else {
        let v11: regex::Regex = compiled_26(v3.clone());
        US6::US6_Some(v11.clone())
    };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    events_in_19(v5.clone(), v6, v7.clone(), v8.clone(), v13.clone(), v14.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        events_text_18(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_sequence_interval(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
pub fn kit_sequence_window(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64, v5: u64) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4, v5)
}
pub fn kit_sequence_order(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn kit_sequence_latency(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn kit_sequence_events(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
