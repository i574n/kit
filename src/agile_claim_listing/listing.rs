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
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Idle,
    US3_LiveJob(Rc<str>),
    US3_RecentEdit(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_LeasedBy(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US5_JobRunning(Rc<str>),
    US5_RecentEditOn(Rc<str>),
    US5_PathOverlap(Rc<str>, Rc<str>),
    US5_DeclinedByYou(Rc<str>, Rc<str>, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US6 {
    US6_Unclaimed,
    US6_LeasedTo(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US6_Expired(Rc<str>, Rc<str>, u64, Rc<UH0>),
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method3(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16, v17)
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method4(v3, v1);
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
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method6(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure1(v0, v1)
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method5(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v3.clone(), v6);
            let mut v8: US0 = v7(v0);
            let mut v9: bool = match v8 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method7(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH1> = v6;
                let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method12(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_14(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let mut v7: Rc<UH0> = globs_from_14(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_11(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 5i32;
    let mut v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v0.starts_with(&*v5);
        let mut v7: bool = v6 == false;
        v7
    };
    if v8 {
        US1::US1_None
    } else {
        let v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method12(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let v25: Rc<str> = if v18 {
            let v19: i32 = v17.wrapping_sub(2i32);
            let v20: usize = ((2i32) as usize);
            let v21: i32 = 2i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v10)[v20..v22]);
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        let v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method13(v26);
        let mut v28: Rc<dyn Fn((u64)) -> US2> = closure3();
        let mut v29: Option<US2> = v27.map(|x| v28(x));
        let v31: US2 = US2::US2_None;
        let v32: US2 = v31.clone();
        let v33: US2 = v29.unwrap_or(v32);
        match v33 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v34) => {
                let v34: u64 = v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH0> = globs_from_14(&v2, v37);
                US1::US1_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US1 = claim_of_row_11(v2);
                let v11: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let mut v7: u64 = v7;
                        let mut v8: Rc<UH0> = v8;
                        Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v1))
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn claims_of_text_8(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_1(v1, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method10(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method9(v7, v8)
}
fn taken_16(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: bool = v0 > 0i32;
            if v4 {
                let v5: i32 = v0.wrapping_sub(1i32);
                let v6: Rc<UH0> = taken_16(v5, v3);
                Rc::new(UH0::UH0_Cons(v2, v6))
            } else {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn lease_refusal_idle_19(mut v0: US3) -> US4 {
    match v0 {
        US3::US3_Idle => {
            US4::US4_None
        }
        US3::US3_LiveJob(v1) => {
            let v1: Rc<str> = v1;
            let v2: US5 = US5::US5_JobRunning(v1);
            US4::US4_Some(v2)
        }
        US3::US3_RecentEdit(v4) => {
            let v4: Rc<str> = v4;
            let v5: US5 = US5::US5_RecentEditOn(v4);
            US4::US4_Some(v5)
        }
    }
}
fn decliner_22(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH0>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let v6: i32 = (v1.clone().len() as i32);
        let mut v7: i32 = v6.wrapping_sub(1i32);
        let mut v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let v10: usize = ((v9) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn claim_of_21(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: u64 = v5;
                let v6: Rc<UH0> = v6;
                let mut v7: Rc<UH1> = v7;
                let mut v8: bool = &*v3 == &*v1;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_22(&v3, v4.clone(), v5, &v6);
                    let v10: bool = &*v9 == "";
                    v10
                } else {
                    false
                };
                if v11 {
                    return US1::US1_Some(v3, v4, v5, v6);
                } else {
                    v0 = v7;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn state_of_20(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64) -> US6 {
    let mut v3: US1 = claim_of_21(v0, v1);
    match v3 {
        US1::US1_None => {
            US6::US6_Unclaimed
        }
        US1::US1_Some(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let mut v7: u64 = v7;
            let mut v8: Rc<UH0> = v8;
            let mut v9: bool = v2 < v7;
            if v9 {
                US6::US6_LeasedTo(v5, v6, v7, v8)
            } else {
                US6::US6_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v5, v6, v7, v8, v9) => {
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let mut v7: u64 = v7;
                let mut v8: Rc<UH0> = v8;
                let mut v9: Rc<UH1> = v9;
                let v10: bool = &*v5 == &*v0;
                let v13: bool = if v10 {
                    let v11: Rc<str> = decliner_22(&v5, v6.clone(), v7, &v8);
                    let v12: bool = &*v11 == &*v1;
                    v12
                } else {
                    false
                };
                let mut v15: bool = if v13 {
                    let mut v14: bool = v2 < v7;
                    v14
                } else {
                    false
                };
                if v15 {
                    return US1::US1_Some(v5, v6, v7, v8);
                } else {
                    v3 = v9;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn declined_by_23(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> US1 {
    method24(v1, v2, v3, v0)
}
fn lease_refusal_18(mut v0: Rc<UH1>, mut v1: US3, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US4 {
    let v5: US6 = state_of_20(v0.clone(), v2.clone(), v4);
    match v5 {
        US6::US6_LeasedTo(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: u64 = v8;
            let mut v9: Rc<UH0> = v9;
            let v10: bool = &*v7 == &*v3;
            if v10 {
                US4::US4_None
            } else {
                let v12: US5 = US5::US5_LeasedBy(v6, v7, v8, v9);
                US4::US4_Some(v12)
            }
        }
        _ => {
            let mut v15: US1 = declined_by_23(v0, v2, v3, v4);
            match v15 {
                US1::US1_None => {
                    lease_refusal_idle_19(v1)
                }
                US1::US1_Some(v16, v17, v18, v19) => {
                    let v16: Rc<str> = v16;
                    let v17: Rc<str> = v17;
                    let v18: u64 = v18;
                    let v19: Rc<UH0> = v19;
                    let v20: US5 = US5::US5_DeclinedByYou(v16, v17, v18, v19);
                    US4::US4_Some(v20)
                }
            }
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v3) {
        UH0::UH0_Cons(v5, v6) => {
            let v5: Rc<str> = v5;
            let v6: Rc<UH0> = v6;
            let mut v7: Rc<UH0> = method17(v0.clone(), v1.clone(), v2, v6, v4);
            let mut v8: US3 = US3::US3_Idle;
            let mut v9: US4 = lease_refusal_18(v0, v8, v5.clone(), v1, v2);
            let v12: bool = match v9 {
                US4::US4_None => {
                    true
                }
                US4::US4_Some(v10) => {
                    let v10: US5 = v10;
                    false
                }
            };
            if v12 {
                Rc::new(UH0::UH0_Cons(v5, v7))
            } else {
                v7
            }
        }
        UH0::UH0_Nil => {
            v4
        }
    }
}
fn candidates_15(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: u64, mut v4: i32) -> Rc<UH0> {
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method17(v1, v2, v3, v0, v5);
    taken_16(v4, v6)
}
fn method25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
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
fn listed_ids_0(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<UH0> = split_list_1(v5.clone(), v1);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method4(v6, v7);
    let mut v9: Rc<UH0> = split_list_1(v5, v2);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method4(v9, v10);
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method5(v8.clone(), v11, v12);
    let mut v14: Rc<UH0> = method7(v8, v13);
    let mut v15: Rc<UH1> = claims_of_text_8(v4);
    let v16: i32 = 5i32;
    let v17: Rc<UH0> = candidates_15(v14, v15, v3, v0, v16);
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v19: i32 = 0i32;
    let v20: i32 = method25(v18.clone(), v17, v19);
    let v21: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
    let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v21).as_ref().clone()));
    v22
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64| -> Rc<RefCell<Vec<Rc<str>>>> {
        listed_ids_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method27(v3, v1);
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
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method28(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v3.clone(), v6);
            let mut v8: US0 = v7(v0);
            let mut v9: bool = match v8 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method29(v0.clone(), v4, v2);
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v3.clone(), v6);
            let mut v8: US0 = v7(v0);
            let mut v9: bool = match v8 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            if v10 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method30(v0.clone(), v1.clone(), v5, v3);
            let mut v7: i32 = 0i32;
            let mut v8: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v4.clone(), v7);
            let mut v9: US0 = v8(v0);
            let v10: bool = match v9 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v11: bool = v10 != true;
            let v12: bool = v11 == false;
            let v19: bool = if v12 {
                let v13: i32 = 0i32;
                let mut v14: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v4.clone(), v13);
                let mut v15: US0 = v14(v1);
                let v16: bool = match v15 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v17: bool = v16 != true;
                let v18: bool = v17 == false;
                v18
            } else {
                false
            };
            if v19 {
                Rc::new(UH0::UH0_Cons(v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn lane_order_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<str>) {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH0> = split_list_1(v3.clone(), v0);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method27(v4, v5);
    let mut v7: Rc<UH0> = split_list_1(v3.clone(), v1);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method27(v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method28(v6.clone(), v9.clone(), v10);
    let v12: Rc<UH0> = split_list_1(v3.clone(), v2);
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v14: Rc<UH0> = method27(v12, v13);
    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method29(v6.clone(), v14.clone(), v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = method30(v6.clone(), v11, v14, v17);
    let v19: Rc<UH0> = method7(v16, v18);
    let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v21: i32 = 0i32;
    let v22: i32 = method25(v20.clone(), v19, v21);
    let v23: Rc<Vec<Rc<str>>> = Rc::new(v20.borrow().clone());
    let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v23).as_ref().clone()));
    let v25: Rc<str> = Rc::<str>::from((*v24.borrow()).join(&*v3));
    let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH0> = method29(v6, v9, v26);
    let mut v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v29: i32 = 0i32;
    let v30: i32 = method25(v28.clone(), v27, v29);
    let v31: Rc<Vec<Rc<str>>> = Rc::new(v28.borrow().clone());
    let v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v31).as_ref().clone()));
    let v33: Rc<str> = Rc::<str>::from((*v32.borrow()).join(&*v3));
    (v25, v33)
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, Rc<str>) {
        lane_order_26(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_listed_ids(v0: &str, v1: &str, v2: &str, v3: &str, v4: u64) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
pub fn kit_claim_lane_order(v0: &str, v1: &str, v2: &str) -> (Rc<str>, Rc<str>) {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_652accfc6a8d = Mut0;
