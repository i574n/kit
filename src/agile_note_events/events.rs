#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i64, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
#[derive(Clone)]
enum US0 {
    US0_Some(i64),
    US0_None,
}
fn method2(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let v12: i32 = v1.wrapping_sub(1i32);
            let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6, v7, v8, v9, v10, v11, v2));
            (v1, v2) = (v12, v13);
            continue;
        }
    }
}
fn facts_list_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_facts::kit_agile_fact_rows(&v0);
    let v2: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v1).borrow().clone());
    let v3: usize = (v2).len();
    let v4: i32 = ((v3) as i32);
    let v5: i32 = v4.wrapping_sub(1i32);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v2, v5, v6)
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i64 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method8(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method7(v16, v17)
    }
}
fn method11(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US0> = Rc::new(move |mut v0: (i64)| -> US0 {
        let v1: i64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stamp_hour_10(mut v0: Rc<str>) -> i64 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 < 13i32;
    if v2 {
        0i64
    } else {
        let v3: usize = ((0i32) as usize);
        let v4: usize = ((4i32) as usize);
        let v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let v7: Option<i64> = method11(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US0> = closure1();
        let mut v9: Option<US0> = v7.map(|x| v8(x));
        let v11: US0 = US0::US0_None;
        let v12: US0 = v11.clone();
        let v13: US0 = v9.unwrap_or(v12);
        let v16: i64 = match v13 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let v18: usize = ((7i32) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method11(v20);
        let mut v22: Option<US0> = v21.map(|x| v8(x));
        let v24: US0 = US0::US0_None;
        let v25: US0 = v24.clone();
        let v26: US0 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v27) => {
                let v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method11(v33);
        let mut v35: Option<US0> = v34.map(|x| v8(x));
        let v37: US0 = US0::US0_None;
        let v38: US0 = v37.clone();
        let v39: US0 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        let v73: i64 = v72.wrapping_mul(24i64);
        let v74: usize = ((11i32) as usize);
        let v75: usize = ((13i32) as usize);
        let v76: Rc<str> = Rc::<str>::from(&(v0)[v74..v75]);
        let v77: Option<i64> = (v76).trim().parse::<i64>().ok();
        let v78: Option<i64> = method11(v77);
        let mut v79: Option<US0> = v78.map(|x| v8(x));
        let v81: US0 = US0::US0_None;
        let v82: US0 = v81.clone();
        let v83: US0 = v79.unwrap_or(v82);
        let v86: i64 = match v83 {
            US0::US0_None => {
                0i64
            }
            US0::US0_Some(v84) => {
                let v84: i64 = v84;
                v84
            }
        };
        let v87: i64 = v73.wrapping_add(v86);
        v87
    }
}
fn method9(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<UH2> = split_list_6(v4, v2);
                let v25: Rc<UH1> = match Rc::unwrap_or_clone(v5) {
                    UH2::UH2_Cons(v6, v7) => {
                        let v6: Rc<str> = v6;
                        let v7: Rc<UH2> = v7;
                        match Rc::unwrap_or_clone(v7) {
                            UH2::UH2_Cons(v8, v9) => {
                                let mut v8: Rc<str> = v8;
                                let mut v9: Rc<UH2> = v9;
                                match Rc::unwrap_or_clone(v9) {
                                    UH2::UH2_Cons(v10, v11) => {
                                        let v10: Rc<str> = v10;
                                        let v11: Rc<UH2> = v11;
                                        match Rc::unwrap_or_clone(v11) {
                                            UH2::UH2_Cons(v12, v13) => {
                                                let v12: Rc<str> = v12;
                                                let v13: Rc<UH2> = v13;
                                                match Rc::unwrap_or_clone(v13) {
                                                    UH2::UH2_Cons(v14, v15) => {
                                                        let v14: Rc<str> = v14;
                                                        let v15: Rc<UH2> = v15;
                                                        match Rc::unwrap_or_clone(v15) {
                                                            UH2::UH2_Cons(v16, v17) => {
                                                                let v16: Rc<str> = v16;
                                                                let v17: Rc<UH2> = v17;
                                                                let v18: i64 = stamp_hour_10(v12);
                                                                Rc::new(UH1::UH1_Cons(v8, v18, v16, v1))
                                                            }
                                                            _ => {
                                                                v1
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        v1
                                                    }
                                                }
                                            }
                                            _ => {
                                                v1
                                            }
                                        }
                                    }
                                    _ => {
                                        v1
                                    }
                                }
                            }
                            _ => {
                                v1
                            }
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v25);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn note_events_in_4(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH2> = split_list_6(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = method9(v2, v3);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method5(v4, v5)
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: i64 = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH1> = method12(v5, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8, v9) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: Rc<UH1> = note_events_in_4(v8);
                let v11: Rc<UH1> = method12(v1, v10);
                (v0, v1) = (v9, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: i64 = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH1> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn note_events_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> {
    let v1: Rc<UH0> = facts_list_1(v0);
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method3(v1, v2);
    let v4: Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method13(v4.clone(), v3, v5);
    let v7: Rc<Vec<(Rc<str>, i64, Rc<str>)>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> {
        note_events_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_note_events(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_0408665020a9 = Mut0;
