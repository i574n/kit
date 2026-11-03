#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Idle,
    US3_LiveJob(Rc<str>),
    US3_RecentEdit(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Claimed(Rc<str>, i32),
    US4_NothingReady,
    US4_AllLeased,
    US4_AllLive,
}
#[derive(Clone)]
enum US5 {
    US5_Unclaimed,
    US5_LeasedTo(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US5_Expired(Rc<str>, Rc<str>, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US7 {
    US7_LeasedBy(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US7_JobRunning(Rc<str>),
    US7_RecentEditOn(Rc<str>),
    US7_PathOverlap(Rc<str>, Rc<str>),
    US7_DeclinedByYou(Rc<str>, Rc<str>, u64, Rc<UH0>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(US7),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_OutcomeGranted,
    US8_OutcomeRefused,
    US8_OutcomeNone,
    US8_OutcomeDropped,
    US8_OutcomeReleaseRefused,
    US8_OutcomeUnknown,
}
#[derive(Clone)]
enum US9 {
    US9_ChangeUpsert,
    US9_ChangeRemove,
    US9_ChangeNone,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method2(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method5(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method5(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16.clone(), v17.clone())
    }
}
fn words_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_3(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v2, v3)
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: u64 = *v4;
                let v5: Rc<UH0> = v5.clone();
                let v6: Rc<UH1> = v6.clone();
                let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4, v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let mut v7: Rc<UH0> = globs_from_12(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_9(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 5i32;
    let v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v0.starts_with(&*v5);
        let mut v7: bool = v6 == false;
        v7
    };
    if v8 {
        US0::US0_None
    } else {
        let v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method10(v12.clone());
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let v25: Rc<str> = if v18 {
            let v19: i32 = v17.wrapping_sub(2i32);
            let v20: usize = ((2i32) as usize);
            let v21: i32 = 2i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v10)[v20..v22]);
            v23.clone()
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24.clone()
        };
        let v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method11(v26.clone());
        let mut v28: Rc<dyn Fn((u64)) -> US1> = closure2();
        let mut v29: Option<US1> = v27.map(|x| v28(x));
        let v31: US1 = US1::US1_None;
        let v32: US1 = v31.clone();
        let v33: US1 = v29.unwrap_or(v32);
        match &v33 {
            US1::US1_None => {
                US0::US0_None
            }
            US1::US1_Some(v34) => {
                let v34: u64 = *v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH0> = globs_from_12(v2.clone(), v37);
                US0::US0_Some(v35.clone(), v36.clone(), v34, v38.clone())
            }
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: US0 = claim_of_row_9(v2.clone());
                let v11: Rc<UH1> = match &v4 {
                    US0::US0_None => {
                        v1.clone()
                    }
                    US0::US0_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5.clone();
                        let v6: Rc<str> = v6.clone();
                        let mut v7: u64 = *v7;
                        let v8: Rc<UH0> = v8.clone();
                        Rc::new(UH1::UH1_Cons(v5.clone(), v6.clone(), v7, v8.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v3.clone(), v11.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn claims_of_text_6(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_3(v1, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH1> = method8(v5, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method7(v7, v8)
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method14(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method14(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure3(v0, v1)
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method13(v0.clone(), v4, v2.clone());
            let v6: i32 = 0i32;
            let mut v7: Rc<dyn Fn(Rc<UH0>) -> US2> = method14(v3.clone(), v6);
            let v8: US2 = v7(v0);
            let mut v9: bool = match &v8 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v10: bool = v9 != true;
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH0::UH0_Cons(v3, v5.clone()))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method15(v3, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn closure4(mut v0: Rc<UH0>) -> Rc<dyn Fn(Rc<str>) -> US3> {
    Rc::new(move |mut v1: Rc<str>| -> US3 {
        let v2: i32 = 0i32;
        let mut v3: Rc<dyn Fn(Rc<UH0>) -> US2> = method14(v1.clone(), v2);
        let v4: US2 = v3(v0.clone());
        let v5: bool = match &v4 {
            US2::US2_None => {
                true
            }
            _ => {
                false
            }
        };
        let v6: bool = v5 != true;
        if v6 {
            US3::US3_LiveJob(v1.clone())
        } else {
            US3::US3_Idle
        }
    })
}
fn claim_of_20(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US0 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: bool = &*v3 == &*v1;
                if v8 {
                    return US0::US0_Some(v3.clone(), v4.clone(), v5, v6.clone());
                } else {
                    v0 = v7.clone();
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn state_of_19(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64) -> US5 {
    let mut v3: US0 = claim_of_20(v0, v1);
    match &v3 {
        US0::US0_None => {
            US5::US5_Unclaimed
        }
        US0::US0_Some(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let mut v7: u64 = *v7;
            let v8: Rc<UH0> = v8.clone();
            let mut v9: bool = v2 < v7;
            if v9 {
                US5::US5_LeasedTo(v5.clone(), v6.clone(), v7, v8.clone())
            } else {
                US5::US5_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn decliner_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let v6: i32 = (v1.clone().len() as i32);
        let mut v7: i32 = v6.wrapping_sub(1i32);
        let v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let v10: usize = ((v9) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn finishing_18(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>) -> bool {
    let v4: US5 = state_of_19(v0, v3, v2);
    match &v4 {
        US5::US5_Expired(v10, v11, v12, v13) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = v11.clone();
            let v12: u64 = *v12;
            let v13: Rc<UH0> = v13.clone();
            let mut v14: Rc<str> = decliner_21(v10, v11, v12, v13);
            let mut v15: bool = &*v14 == "";
            v15
        }
        US5::US5_LeasedTo(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let mut v7: u64 = *v7;
            let v8: Rc<UH0> = v8.clone();
            let mut v9: bool = &*v6 == &*v1;
            v9
        }
        US5::US5_Unclaimed => {
            false
        }
    }
}
fn lease_refusal_idle_23(mut v0: US3) -> US6 {
    match &v0 {
        US3::US3_Idle => {
            US6::US6_None
        }
        US3::US3_LiveJob(v1) => {
            let v1: Rc<str> = v1.clone();
            let v2: US7 = US7::US7_JobRunning(v1);
            US6::US6_Some(v2)
        }
        US3::US3_RecentEdit(v4) => {
            let v4: Rc<str> = v4.clone();
            let v5: US7 = US7::US7_RecentEditOn(v4);
            US6::US6_Some(v5)
        }
    }
}
fn lease_refusal_22(mut v0: Rc<UH1>, mut v1: US3, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US6 {
    let v5: US5 = state_of_19(v0, v2, v4);
    match &v5 {
        US5::US5_LeasedTo(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH0> = v9.clone();
            let v10: bool = &*v7 == &*v3;
            if v10 {
                US6::US6_None
            } else {
                let v12: Rc<str> = decliner_21(v6.clone(), v7.clone(), v8, v9.clone());
                let v13: bool = &*v12 == &*v3;
                if v13 {
                    let mut v14: US7 = US7::US7_DeclinedByYou(v6.clone(), v7.clone(), v8, v9.clone());
                    US6::US6_Some(v14)
                } else {
                    let v16: Rc<str> = decliner_21(v6.clone(), v7.clone(), v8, v9.clone());
                    let v17: bool = &*v16 != "";
                    if v17 {
                        lease_refusal_idle_23(v1.clone())
                    } else {
                        let v19: US7 = US7::US7_LeasedBy(v6, v7, v8, v9);
                        US6::US6_Some(v19)
                    }
                }
            }
        }
        _ => {
            lease_refusal_idle_23(v1)
        }
    }
}
fn first_free_17(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<dyn Fn(Rc<str>) -> US3>, mut v4: Rc<str>, mut v5: u64, mut v6: bool, mut v7: i32, mut v8: i32) -> (US4, i32, i32) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v10, v11) => {
                let v10: Rc<str> = v10.clone();
                let v11: Rc<UH0> = v11.clone();
                let v12: i32 = v1.wrapping_add(1i32);
                let mut v15: bool = if v6 {
                    let v13: bool = finishing_18(v2.clone(), v4.clone(), v5, v10.clone());
                    let mut v14: bool = v13 == false;
                    v14
                } else {
                    false
                };
                if v15 {
                    (v0, v1) = (v11.clone(), v12);
                    continue;
                } else {
                    let v19: US3 = v3(v10.clone());
                    let v20: US6 = lease_refusal_22(v2.clone(), v19.clone(), v10.clone(), v4.clone(), v5);
                    match &v20 {
                        US6::US6_None => {
                            let v21: US4 = US4::US4_Claimed(v10.clone(), v1);
                            return (v21.clone(), v7, v8);
                        }
                        US6::US6_Some(v22) => {
                            let v22: US7 = v22.clone();
                            match &v22 {
                                US7::US7_LeasedBy(v23, v24, v25, v26) => {
                                    let v23: Rc<str> = v23.clone();
                                    let v24: Rc<str> = v24.clone();
                                    let v25: u64 = *v25;
                                    let v26: Rc<UH0> = v26.clone();
                                    let v27: i32 = v7.wrapping_add(1i32);
                                    (v0, v1, v7) = (v11.clone(), v12, v27);
                                    continue;
                                }
                                _ => {
                                    let v31: i32 = v8.wrapping_add(1i32);
                                    (v0, v1, v8) = (v11.clone(), v12, v31);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v9: US4 = US4::US4_NothingReady;
                return (v9.clone(), v7, v8);
            }
        }
    }
}
fn choose_16(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<dyn Fn(Rc<str>) -> US3>, mut v3: Rc<str>, mut v4: u64) -> US4 {
    let v5: i32 = 1i32;
    let v6: bool = true;
    let mut v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11): (US4, i32, i32) = first_free_17(v0.clone(), v5, v1.clone(), v2.clone(), v3.clone(), v4, v6, v7, v8);
    match &v9 {
        US4::US4_Claimed(v12, v13) => {
            let v12: Rc<str> = v12.clone();
            let v13: i32 = *v13;
            US4::US4_Claimed(v12, v13)
        }
        _ => {
            let mut v15: i32 = 1i32;
            let v16: bool = false;
            let v17: i32 = 0i32;
            let v18: i32 = 0i32;
            let (mut v19, mut v20, mut v21): (US4, i32, i32) = first_free_17(v0, v15, v1, v2, v3, v4, v16, v17, v18);
            match &v19 {
                US4::US4_Claimed(v22, v23) => {
                    let v22: Rc<str> = v22.clone();
                    let v23: i32 = *v23;
                    US4::US4_Claimed(v22, v23)
                }
                _ => {
                    let v25: bool = v20 > 0i32;
                    let v27: bool = if v25 {
                        let v26: bool = v20 >= v21;
                        v26
                    } else {
                        false
                    };
                    if v27 {
                        US4::US4_AllLeased
                    } else {
                        let mut v29: bool = v21 > 0i32;
                        if v29 {
                            US4::US4_AllLive
                        } else {
                            US4::US4_NothingReady
                        }
                    }
                }
            }
        }
    }
}
fn method25(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: bool = &*v3 == &*v0;
                let mut v9: Rc<UH0> = if v8 {
                    v6.clone()
                } else {
                    v2.clone()
                };
                (v1, v2) = (v7.clone(), v9.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v2.clone();
            }
        }
    }
}
fn held_paths_24(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method25(v1, v0, v2)
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let v4: u64 = *v4;
            let v5: Rc<UH0> = v5.clone();
            let v6: Rc<UH1> = v6.clone();
            let mut v7: Rc<UH1> = method27(v6, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v7))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>, mut v4: Rc<UH1>, mut v5: Rc<UH1>) -> Rc<UH1> {
    match &*v4 {
        UH1::UH1_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH0> = v9.clone();
            let v10: Rc<UH1> = v10.clone();
            let v11: Rc<UH1> = method28(v0.clone(), v1, v2, v3, v10, v5.clone());
            let v12: bool = &*v6 != &*v0;
            if v12 {
                Rc::new(UH1::UH1_Cons(v6, v7, v8, v9, v11.clone()))
            } else {
                v11
            }
        }
        UH1::UH1_Nil => {
            v5
        }
    }
}
fn upserted_26(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH1> {
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method28(v1.clone(), v2.clone(), v3, v4.clone(), v0, v5);
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v1, v2, v3, v4, v7));
    method27(v6, v8)
}
fn method30(mut v0: u64, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: bool = v0 < v5;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_21(v3.clone(), v4.clone(), v5, v6.clone());
                    let v10: bool = &*v9 == "";
                    v10
                } else {
                    false
                };
                let v13: i32 = if v11 {
                    let v12: i32 = v2.wrapping_add(1i32);
                    v12
                } else {
                    v2
                };
                (v1, v2) = (v7.clone(), v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn live_count_29(mut v0: Rc<UH1>, mut v1: u64) -> i32 {
    let v2: i32 = 0i32;
    method30(v1, v0, v2)
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn outcome_word_32(mut v0: US8) -> Rc<str> {
    let mut v7: Rc<str> = match &v0 {
        US8::US8_OutcomeDropped => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US8::US8_OutcomeGranted => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US8::US8_OutcomeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US8::US8_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US8::US8_OutcomeReleaseRefused => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US8::US8_OutcomeUnknown => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn position_of_33(mut v0: Rc<str>, mut v1: Rc<UH0>) -> i32 {
    let v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH0>) -> US2> = method14(v0, v2);
    let v4: US2 = v3(v1);
    let mut v7: i32 = match &v4 {
        US2::US2_None => {
            -1i32
        }
        US2::US2_Some(v5) => {
            let v5: i32 = *v5;
            v5
        }
    };
    let v8: i32 = v7.wrapping_add(1i32);
    v8
}
fn change_word_34(mut v0: US9) -> Rc<str> {
    let v4: Rc<str> = match &v0 {
        US9::US9_ChangeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US9::US9_ChangeRemove => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US9::US9_ChangeUpsert => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn none_reason_35(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_AllLeased => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all-leased"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US4::US4_AllLive => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all-live"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        _ => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none-ready"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn claim_next_0(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let mut v7: Rc<UH0> = words_1(v2);
    let v8: Rc<UH0> = words_1(v3);
    let mut v9: Rc<UH1> = claims_of_text_6(v5);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method13(v8.clone(), v7.clone(), v10);
    let v12: Rc<UH0> = method15(v8, v11);
    let v13: Rc<UH0> = words_1(v4);
    let mut v14: Rc<dyn Fn(Rc<str>) -> US3> = closure4(v13);
    let mut v15: US4 = choose_16(v12, v9.clone(), v14, v0.clone(), v6);
    match &v15 {
        US4::US4_Claimed(v16, v17) => {
            let v16: Rc<str> = v16.clone();
            let v17: i32 = *v17;
            let v18: u64 = (v1 as u64);
            let v19: u64 = v18.wrapping_mul(60000u64);
            let v20: u64 = v6.wrapping_add(v19);
            let v21: Rc<UH0> = held_paths_24(v9.clone(), v16.clone());
            let v22: Rc<UH1> = upserted_26(v9.clone(), v16.clone(), v0.clone(), v20, v21.clone());
            let v23: i32 = live_count_29(v22, v6);
            let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v25: i32 = 0i32;
            let v26: i32 = method31(v24.clone(), v21, v25);
            let v27: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
            let mut v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v27).as_ref().clone()));
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from((*v28.borrow()).join(&*v29));
            let v31: US8 = US8::US8_OutcomeGranted;
            let v32: Rc<str> = outcome_word_32(v31);
            let v33: i32 = position_of_33(v16.clone(), v7);
            let v34: US9 = US9::US9_ChangeUpsert;
            let v35: Rc<str> = change_word_34(v34);
            (v32, v16, v0.clone(), v20, v33, v23, v30, v35)
        }
        _ => {
            let v36: US8 = US8::US8_OutcomeNone;
            let v37: Rc<str> = outcome_word_32(v36);
            let v38: i32 = live_count_29(v9, v6);
            let v39: Rc<str> = none_reason_35(v15);
            let v40: US9 = US9::US9_ChangeNone;
            let v41: Rc<str> = change_word_34(v40);
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v37, v42, v0, 0u64, 0i32, v38, v39, v41)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        claim_next_0(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_next(v0: &str, v1: i32, v2: &str, v3: &str, v4: &str, v5: &str, v6: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), v6)
}
