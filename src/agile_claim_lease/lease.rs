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
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Idle,
    US1_LiveJob(Rc<str>),
    US1_RecentEdit(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, u64, Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, u64, Rc<UH0>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
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
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_OutcomeGranted,
    US9_OutcomeRefused,
    US9_OutcomeNone,
    US9_OutcomeDropped,
    US9_OutcomeReleaseRefused,
    US9_OutcomeUnknown,
}
#[derive(Clone)]
enum US10 {
    US10_ChangeUpsert,
    US10_ChangeRemove,
    US10_ChangeNone,
}
struct Mut1 { l0: Rc<str> }
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method2(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
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
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
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
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method5(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
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
fn words_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = split_list_3(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v2, v3)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v0.clone(), v8);
                    v9(v5.clone())
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
fn method8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: u64 = *v4;
                let mut v5: Rc<UH0> = v5.clone();
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4, v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method11(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH0> = globs_from_13(v0, v6);
        Rc::new(UH0::UH0_Cons(v5, v7))
    }
}
fn claim_of_row_10(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 5i32;
    let v8: bool = if v4 {
        true
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v0.starts_with(&*v5);
        let v7: bool = v6 == false;
        v7
    };
    if v8 {
        US2::US2_None
    } else {
        let v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let mut v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method11(v12.clone());
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let mut v18: bool = v17 > 2i32;
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
        let v27: Option<u64> = method12(v26.clone());
        let mut v28: Rc<dyn Fn((u64)) -> US3> = closure3();
        let mut v29: Option<US3> = v27.map(|x| v28(x));
        let v31: US3 = US3::US3_None;
        let v32: US3 = v31.clone();
        let v33: US3 = v29.unwrap_or(v32);
        match &v33 {
            US3::US3_None => {
                US2::US2_None
            }
            US3::US3_Some(v34) => {
                let v34: u64 = *v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH0> = globs_from_13(v2.clone(), v37);
                US2::US2_Some(v35.clone(), v36.clone(), v34, v38.clone())
            }
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: US2 = claim_of_row_10(v2.clone());
                let v11: Rc<UH1> = match &v4 {
                    US2::US2_None => {
                        v1.clone()
                    }
                    US2::US2_Some(v5, v6, v7, v8) => {
                        let mut v5: Rc<str> = v5.clone();
                        let v6: Rc<str> = v6.clone();
                        let v7: u64 = *v7;
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
fn claims_of_text_7(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_3(v1, v4);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method9(v5, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method8(v7, v8)
}
fn lease_refusal_idle_16(mut v0: US1) -> US4 {
    match &v0 {
        US1::US1_Idle => {
            US4::US4_None
        }
        US1::US1_LiveJob(v1) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: US5 = US5::US5_JobRunning(v1);
            US4::US4_Some(v2)
        }
        US1::US1_RecentEdit(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: US5 = US5::US5_RecentEditOn(v4);
            US4::US4_Some(v5)
        }
    }
}
fn claim_of_18(mut v0: Rc<UH1>, mut v1: Rc<str>) -> US2 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<UH1> = v7.clone();
                let v8: bool = &*v3 == &*v1;
                if v8 {
                    return US2::US2_Some(v3.clone(), v4.clone(), v5, v6.clone());
                } else {
                    v0 = v7.clone();
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn state_of_17(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64) -> US6 {
    let mut v3: US2 = claim_of_18(v0, v1);
    match &v3 {
        US2::US2_None => {
            US6::US6_Unclaimed
        }
        US2::US2_Some(v5, v6, v7, v8) => {
            let mut v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: u64 = *v7;
            let v8: Rc<UH0> = v8.clone();
            let mut v9: bool = v2 < v7;
            if v9 {
                US6::US6_LeasedTo(v5.clone(), v6.clone(), v7, v8.clone())
            } else {
                US6::US6_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn decliner_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v1.starts_with(&*v4);
    if v5 {
        let v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(1i32);
        let v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let v10: usize = ((v9) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn lease_refusal_15(mut v0: Rc<UH1>, mut v1: US1, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US4 {
    let mut v5: US6 = state_of_17(v0, v2, v4);
    match &v5 {
        US6::US6_LeasedTo(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH0> = v9.clone();
            let v10: bool = &*v7 == &*v3;
            if v10 {
                US4::US4_None
            } else {
                let mut v12: Rc<str> = decliner_19(v6.clone(), v7.clone(), v8, v9.clone());
                let v13: bool = &*v12 == &*v3;
                if v13 {
                    let mut v14: US5 = US5::US5_DeclinedByYou(v6.clone(), v7.clone(), v8, v9.clone());
                    US4::US4_Some(v14)
                } else {
                    let v16: Rc<str> = decliner_19(v6.clone(), v7.clone(), v8, v9.clone());
                    let v17: bool = &*v16 != "";
                    if v17 {
                        lease_refusal_idle_16(v1.clone())
                    } else {
                        let v19: US5 = US5::US5_LeasedBy(v6, v7, v8, v9);
                        US4::US4_Some(v19)
                    }
                }
            }
        }
        _ => {
            lease_refusal_idle_16(v1)
        }
    }
}
fn glob_stem_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method11(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        v0.clone()
    } else {
        let mut v9: usize = ((0i32) as usize);
        let v10: usize = ((v7) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v0)[v9..v10]);
        v11.clone()
    }
}
fn globs_overlap_23(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = glob_stem_24(v0);
    let mut v3: Rc<str> = glob_stem_24(v1);
    let mut v4: bool = v3.starts_with(&*v2);
    if v4 {
        true
    } else {
        let mut v5: bool = v2.starts_with(&*v3);
        v5
    }
}
fn overlap_in_22(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US8 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = globs_overlap_23(v0.clone(), v3.clone());
                if v5 {
                    return US8::US8_Some(v0.clone());
                } else {
                    v1 = v4.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn first_overlap_21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> US8 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: US8 = overlap_in_22(v3.clone(), v1.clone());
                match &v5 {
                    US8::US8_None => {
                        v0 = v4.clone();
                        continue;
                    }
                    US8::US8_Some(v6) => {
                        let v6: Rc<str> = v6.clone();
                        return US8::US8_Some(v6.clone());
                    }
                }
            }
            UH0::UH0_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn overlapping_claim_20(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: u64) -> US7 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v5, v6, v7, v8, v9) => {
                let mut v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: u64 = *v7;
                let v8: Rc<UH0> = v8.clone();
                let mut v9: Rc<UH1> = v9.clone();
                let v10: bool = v3 < v7;
                let mut v12: bool = if v10 {
                    let v11: bool = &*v5 != &*v2;
                    v11
                } else {
                    false
                };
                let mut v15: US8 = if v12 {
                    first_overlap_21(v0.clone(), v8.clone())
                } else {
                    US8::US8_None
                };
                match &v15 {
                    US8::US8_None => {
                        v1 = v9.clone();
                        continue;
                    }
                    US8::US8_Some(v16) => {
                        let v16: Rc<str> = v16.clone();
                        return US7::US7_Some(v5.clone(), v16.clone());
                    }
                }
            }
            UH1::UH1_Nil => {
                return US7::US7_None;
            }
        }
    }
}
fn refusal_of_14(mut v0: Rc<UH1>, mut v1: US1, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: u64) -> US4 {
    let v6: US4 = lease_refusal_15(v0.clone(), v1, v2.clone(), v3, v5);
    match &v6 {
        US4::US4_None => {
            let mut v9: US7 = overlapping_claim_20(v4, v0, v2, v5);
            match &v9 {
                US7::US7_None => {
                    US4::US4_None
                }
                US7::US7_Some(v10, v11) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = v11.clone();
                    let mut v12: US5 = US5::US5_PathOverlap(v10, v11);
                    US4::US4_Some(v12)
                }
            }
        }
        US4::US4_Some(v7) => {
            let v7: US5 = v7.clone();
            US4::US4_Some(v7)
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<UH1> = v7.clone();
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
fn held_paths_25(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method26(v1, v0, v2)
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: u64 = *v4;
            let mut v5: Rc<UH0> = v5.clone();
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH1> = method28(v6, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v7))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>, mut v4: Rc<UH1>, mut v5: Rc<UH1>) -> Rc<UH1> {
    match &*v4 {
        UH1::UH1_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH0> = v9.clone();
            let v10: Rc<UH1> = v10.clone();
            let v11: Rc<UH1> = method29(v0.clone(), v1, v2, v3, v10, v5.clone());
            let mut v12: bool = &*v6 != &*v0;
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
fn upserted_27(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH0>) -> Rc<UH1> {
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method29(v1.clone(), v2.clone(), v3, v4.clone(), v0, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v1, v2, v3, v4, v7));
    method28(v6, v8)
}
fn method31(mut v0: u64, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<UH1> = v7.clone();
                let v8: bool = v0 < v5;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_19(v3.clone(), v4.clone(), v5, v6.clone());
                    let v10: bool = &*v9 == "";
                    v10
                } else {
                    false
                };
                let v13: i32 = if v11 {
                    let mut v12: i32 = v2.wrapping_add(1i32);
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
fn live_count_30(mut v0: Rc<UH1>, mut v1: u64) -> i32 {
    let v2: i32 = 0i32;
    method31(v1, v0, v2)
}
fn position_of_32(mut v0: Rc<str>, mut v1: Rc<UH0>) -> i32 {
    let v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v0, v2);
    let mut v4: US0 = v3(v1);
    let v7: i32 = match &v4 {
        US0::US0_None => {
            -1i32
        }
        US0::US0_Some(v5) => {
            let mut v5: i32 = *v5;
            v5
        }
    };
    let v8: i32 = v7.wrapping_add(1i32);
    v8
}
fn outcome_word_33(mut v0: US9) -> Rc<str> {
    let v7: Rc<str> = match &v0 {
        US9::US9_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US9::US9_OutcomeGranted => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US9::US9_OutcomeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US9::US9_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US9::US9_OutcomeReleaseRefused => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US9::US9_OutcomeUnknown => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn method34(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn change_word_35(mut v0: US10) -> Rc<str> {
    let mut v4: Rc<str> = match &v0 {
        US10::US10_ChangeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US10::US10_ChangeRemove => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US10::US10_ChangeUpsert => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let mut v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn method38(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_37(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn refusal_reason_36(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_DeclinedByYou(v24, v25, v26, v27) => {
            let v24: Rc<str> = v24.clone();
            let v25: Rc<str> = v25.clone();
            let v26: u64 = *v26;
            let v27: Rc<UH0> = v27.clone();
            let mut v28: Rc<str> = format_real_37(v26);
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("declined-by-you-until:"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
            v30
        }
        US5::US5_JobRunning(v11) => {
            let v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-running:"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
            v13
        }
        US5::US5_LeasedBy(v1, v2, v3, v4) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            let mut v3: u64 = *v3;
            let mut v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leased-by:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = format_real_37(v3);
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10
        }
        US5::US5_PathOverlap(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let mut v18: Rc<str> = v18.clone();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("path-overlap:"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v17));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v21)), v18));
            v23
        }
        US5::US5_RecentEditOn(v14) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recent-edit:"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            v16
        }
    }
}
fn lease_claimed_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<UH0> = words_1(v5, v9.clone());
    let v11: i32 = 0i32;
    let mut v12: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v0.clone(), v11);
    let v13: US0 = v12(v10);
    let mut v14: bool = match &v13 {
        US0::US0_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v15: bool = v14 != true;
    if v15 {
        let v16: Rc<UH0> = words_1(v6, v9.clone());
        let v17: i32 = 0i32;
        let mut v18: Rc<dyn Fn(Rc<UH0>) -> US0> = method6(v0.clone(), v17);
        let v19: US0 = v18(v16);
        let v20: bool = match &v19 {
            US0::US0_None => {
                true
            }
            _ => {
                false
            }
        };
        let v21: bool = v20 != true;
        let v24: US1 = if v21 {
            US1::US1_LiveJob(v0.clone())
        } else {
            US1::US1_Idle
        };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<UH0> = words_1(v3, v25);
        let v27: Rc<UH1> = claims_of_text_7(v7);
        let mut v28: US4 = refusal_of_14(v27.clone(), v24, v0.clone(), v1.clone(), v26.clone(), v8);
        match &v28 {
            US4::US4_None => {
                let v36: bool = match &*v26 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v38: Rc<UH0> = if v36 {
                    held_paths_25(v27.clone(), v0.clone())
                } else {
                    v26
                };
                let v39: u64 = (v2 as u64);
                let v40: u64 = v39.wrapping_mul(60000u64);
                let v41: u64 = v8.wrapping_add(v40);
                let v42: Rc<UH1> = upserted_27(v27.clone(), v0.clone(), v1.clone(), v41, v38.clone());
                let v43: i32 = live_count_30(v42, v8);
                let v44: Rc<UH0> = words_1(v4, v9);
                let v45: i32 = position_of_32(v0.clone(), v44);
                let v46: US9 = US9::US9_OutcomeGranted;
                let v47: Rc<str> = outcome_word_33(v46);
                let v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v49: i32 = 0i32;
                let v50: i32 = method34(v48.clone(), v38, v49);
                let v51: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v51).as_ref().clone()));
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = Rc::<str>::from((*v52.borrow()).join(&*v53));
                let v55: US10 = US10::US10_ChangeUpsert;
                let v56: Rc<str> = change_word_35(v55);
                (v47, v0.clone(), v1.clone(), v41, v45, v43, v54, v56)
            }
            US4::US4_Some(v29) => {
                let mut v29: US5 = v29.clone();
                let v30: US9 = US9::US9_OutcomeRefused;
                let v31: Rc<str> = outcome_word_33(v30);
                let v32: i32 = live_count_30(v27, v8);
                let v33: Rc<str> = refusal_reason_36(v29);
                let v34: US10 = US10::US10_ChangeNone;
                let v35: Rc<str> = change_word_35(v34);
                (v31, v0.clone(), v1.clone(), 0u64, 0i32, v32, v33, v35)
            }
        }
    } else {
        let v73: US9 = US9::US9_OutcomeUnknown;
        let v74: Rc<str> = outcome_word_33(v73);
        let v75: US10 = US10::US10_ChangeNone;
        let v76: Rc<str> = change_word_35(v75);
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v74, v0, v1, 0u64, 0i32, 0i32, v77, v76)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        lease_claimed_0(v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method41(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: u64 = *v5;
            let v6: Rc<UH0> = v6.clone();
            let v7: Rc<UH1> = v7.clone();
            let v8: Rc<UH1> = method41(v0.clone(), v7, v2.clone());
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v8.clone()))
            } else {
                v8
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn released_40(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method41(v1, v0, v2)
}
fn declined_row_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: u64, mut v5: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v6: US9 = US9::US9_OutcomeDropped;
    let v7: Rc<str> = outcome_word_33(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
    let v10: Rc<UH1> = released_40(v3, v0.clone());
    let v11: i32 = live_count_30(v10, v4);
    let mut v12: US10 = US10::US10_ChangeUpsert;
    let v13: Rc<str> = change_word_35(v12);
    (v7, v0, v9, v2, 0i32, v11, v5, v13)
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        let mut v5: Rc<UH1> = claims_of_text_7(v3.clone());
        let v6: u64 = (v2 as u64);
        let v7: u64 = v6.wrapping_mul(60000u64);
        let v8: u64 = v4.wrapping_add(v7);
        let mut v9: US6 = state_of_17(v5.clone(), v0.clone(), v4);
        match &v9 {
            US6::US6_LeasedTo(v10, v11, v12, v13) => {
                let v10: Rc<str> = v10.clone();
                let v11: Rc<str> = v11.clone();
                let mut v12: u64 = *v12;
                let v13: Rc<UH0> = v13.clone();
                let mut v14: bool = &*v11 == &*v1;
                if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                    declined_row_39(v0.clone(), v1.clone(), v8, v5.clone(), v4, v15.clone())
                } else {
                    let v24: Rc<str> = decliner_19(v10.clone(), v11.clone(), v12, v13.clone());
                    let v25: bool = &*v24 != "";
                    if v25 {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                        declined_row_39(v0.clone(), v1.clone(), v8, v5.clone(), v4, v26.clone())
                    } else {
                        let v35: US9 = US9::US9_OutcomeReleaseRefused;
                        let v36: Rc<str> = outcome_word_33(v35.clone());
                        let v37: i32 = live_count_30(v5.clone(), v4);
                        let v38: US5 = US5::US5_LeasedBy(v10.clone(), v11.clone(), v12, v13.clone());
                        let v39: Rc<str> = refusal_reason_36(v38.clone());
                        let v40: US10 = US10::US10_ChangeNone;
                        let v41: Rc<str> = change_word_35(v40.clone());
                        (v36.clone(), v0.clone(), v11.clone(), v12, 0i32, v37, v39.clone(), v41.clone())
                    }
                }
            }
            US6::US6_Unclaimed => {
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                declined_row_39(v0.clone(), v1.clone(), v8, v5.clone(), v4, v58.clone())
            }
            _ => {
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                declined_row_39(v0.clone(), v1.clone(), v8, v5.clone(), v4, v67.clone())
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_lease(v0: &str, v1: &str, v2: i32, v3: &str, v4: &str, v5: &str, v6: &str, v7: &str, v8: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6), Rc::<str>::from(v7), v8)
}
pub fn kit_claim_release(v0: &str, v1: &str, v2: i32, v3: &str, v4: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3), v4)
}
