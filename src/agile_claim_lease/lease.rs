#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, u64, Rc<UH1>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_OutcomeGranted,
    US0_OutcomeRefused,
    US0_OutcomeNone,
    US0_OutcomeDropped,
    US0_OutcomeReleaseRefused,
    US0_OutcomeUnknown,
}
#[derive(Clone)]
enum US1 {
    US1_ChangeUpsert,
    US1_ChangeRemove,
    US1_ChangeNone,
}
struct Mut0 { l0: i32, l1: Rc<UH1> }
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Unclaimed,
    US4_LeasedTo(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US4_Expired(Rc<str>, Rc<str>, u64, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_LeasedBy(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US5_JobRunning(Rc<str>),
    US5_RecentEditOn(Rc<str>),
    US5_PathOverlap(Rc<str>, Rc<str>),
    US5_DeclinedByYou(Rc<str>, Rc<str>, u64, Rc<UH1>),
}
struct Mut1 { l0: Rc<str> }
#[derive(Clone)]
enum US6 {
    US6_Some(i32),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Idle,
    US7_LiveJob(Rc<str>),
    US7_RecentEdit(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Some(US5),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>, Rc<str>),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<str>),
    US10_None,
}
fn outcome_word_2(mut v0: US0) -> Rc<str> {
    let v7: Rc<str> = match &v0 {
        US0::US0_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_OutcomeGranted => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_OutcomeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_OutcomeReleaseRefused => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_OutcomeUnknown => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn method4(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: u64 = *v5;
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = method4(v0.clone(), v7, v2.clone());
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8.clone()))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn released_3(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method4(v1, v0, v2)
}
fn decliner_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>) -> Rc<str> {
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
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn method6(mut v0: u64, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = v0 < v5;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_7(v3.clone(), v4.clone(), v5, v6.clone());
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
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn live_count_5(mut v0: Rc<UH0>, mut v1: u64) -> i32 {
    let v2: i32 = 0i32;
    method6(v1, v0, v2)
}
fn change_word_8(mut v0: US1) -> Rc<str> {
    let mut v4: Rc<str> = match &v0 {
        US1::US1_ChangeNone => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_ChangeRemove => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_ChangeUpsert => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let mut v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn declined_row_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>, mut v4: u64, mut v5: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v6: US0 = US0::US0_OutcomeDropped;
    let v7: Rc<str> = outcome_word_2(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
    let v10: Rc<UH0> = released_3(v3, v0.clone());
    let v11: i32 = live_count_5(v10, v4);
    let v12: US1 = US1::US1_ChangeUpsert;
    let v13: Rc<str> = change_word_8(v12);
    (v7, v0, v9, v2, 0i32, v11, v5, v13)
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: u64 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v4, v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method13(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method12(v16.clone(), v17.clone())
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH1> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let mut v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH1> = globs_from_18(v0, v6);
        Rc::new(UH1::UH1_Cons(v5, v7))
    }
}
fn claim_of_row_15(mut v0: Rc<str>) -> US2 {
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
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method16(v12.clone());
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
        let v27: Option<u64> = method17(v26.clone());
        let mut v28: Rc<dyn Fn((u64)) -> US3> = closure2();
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
                let v38: Rc<UH1> = globs_from_18(v2.clone(), v37);
                US2::US2_Some(v35.clone(), v36.clone(), v34, v38.clone())
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: US2 = claim_of_row_15(v2.clone());
                let v11: Rc<UH0> = match &v4 {
                    US2::US2_None => {
                        v1.clone()
                    }
                    US2::US2_Some(v5, v6, v7, v8) => {
                        let mut v5: Rc<str> = v5.clone();
                        let v6: Rc<str> = v6.clone();
                        let v7: u64 = *v7;
                        let v8: Rc<UH1> = v8.clone();
                        Rc::new(UH0::UH0_Cons(v5.clone(), v6.clone(), v7, v8.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v3.clone(), v11.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn claims_of_text_9(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_11(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method14(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method10(v7, v8)
}
fn claim_of_20(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US2 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = &*v3 == &*v1;
                if v8 {
                    return US2::US2_Some(v3.clone(), v4.clone(), v5, v6.clone());
                } else {
                    v0 = v7.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn state_of_19(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> US4 {
    let mut v3: US2 = claim_of_20(v0, v1);
    match &v3 {
        US2::US2_None => {
            US4::US4_Unclaimed
        }
        US2::US2_Some(v5, v6, v7, v8) => {
            let mut v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: u64 = *v7;
            let v8: Rc<UH1> = v8.clone();
            let mut v9: bool = v2 < v7;
            if v9 {
                US4::US4_LeasedTo(v5.clone(), v6.clone(), v7, v8.clone())
            } else {
                US4::US4_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn method23(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn refusal_reason_21(mut v0: US5) -> Rc<str> {
    match &v0 {
        US5::US5_DeclinedByYou(v24, v25, v26, v27) => {
            let v24: Rc<str> = v24.clone();
            let v25: Rc<str> = v25.clone();
            let v26: u64 = *v26;
            let v27: Rc<UH1> = v27.clone();
            let mut v28: Rc<str> = format_real_22(v26);
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("declined-by-you-until:"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
            v30
        }
        US5::US5_JobRunning(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-running:"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
            v13
        }
        US5::US5_LeasedBy(v1, v2, v3, v4) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            let mut v3: u64 = *v3;
            let mut v4: Rc<UH1> = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leased-by:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = format_real_22(v3);
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10
        }
        US5::US5_PathOverlap(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
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
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = method25(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn words_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = split_list_11(v1, v0);
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method25(v2, v3)
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    Rc::new(move |mut v2: Rc<UH1>| -> US6 {
        match &*v2 {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US6::US6_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US6> = method26(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH1::UH1_Nil => {
                US6::US6_None
            }
        }
    })
}
fn method26(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    closure3(v0, v1)
}
fn lease_refusal_idle_29(mut v0: US7) -> US8 {
    match &v0 {
        US7::US7_Idle => {
            US8::US8_None
        }
        US7::US7_LiveJob(v1) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: US5 = US5::US5_JobRunning(v1);
            US8::US8_Some(v2)
        }
        US7::US7_RecentEdit(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: US5 = US5::US5_RecentEditOn(v4);
            US8::US8_Some(v5)
        }
    }
}
fn lease_refusal_28(mut v0: Rc<UH0>, mut v1: US7, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US8 {
    let mut v5: US4 = state_of_19(v0, v2, v4);
    match &v5 {
        US4::US4_LeasedTo(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH1> = v9.clone();
            let v10: bool = &*v7 == &*v3;
            if v10 {
                US8::US8_None
            } else {
                let v12: Rc<str> = decliner_7(v6.clone(), v7.clone(), v8, v9.clone());
                let v13: bool = &*v12 == &*v3;
                if v13 {
                    let mut v14: US5 = US5::US5_DeclinedByYou(v6.clone(), v7.clone(), v8, v9.clone());
                    US8::US8_Some(v14)
                } else {
                    let v16: Rc<str> = decliner_7(v6.clone(), v7.clone(), v8, v9.clone());
                    let v17: bool = &*v16 != "";
                    if v17 {
                        lease_refusal_idle_29(v1.clone())
                    } else {
                        let v19: US5 = US5::US5_LeasedBy(v6, v7, v8, v9);
                        US8::US8_Some(v19)
                    }
                }
            }
        }
        _ => {
            lease_refusal_idle_29(v1)
        }
    }
}
fn glob_stem_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method16(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
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
fn globs_overlap_33(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = glob_stem_34(v0);
    let mut v3: Rc<str> = glob_stem_34(v1);
    let mut v4: bool = v3.starts_with(&*v2);
    if v4 {
        true
    } else {
        let mut v5: bool = v2.starts_with(&*v3);
        v5
    }
}
fn overlap_in_32(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US10 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: bool = globs_overlap_33(v0.clone(), v3.clone());
                if v5 {
                    return US10::US10_Some(v0.clone());
                } else {
                    v1 = v4.clone();
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US10::US10_None;
            }
        }
    }
}
fn first_overlap_31(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> US10 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: US10 = overlap_in_32(v3.clone(), v1.clone());
                match &v5 {
                    US10::US10_None => {
                        v0 = v4.clone();
                        continue;
                    }
                    US10::US10_Some(v6) => {
                        let v6: Rc<str> = v6.clone();
                        return US10::US10_Some(v6.clone());
                    }
                }
            }
            UH1::UH1_Nil => {
                return US10::US10_None;
            }
        }
    }
}
fn overlapping_claim_30(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64) -> US9 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v5, v6, v7, v8, v9) => {
                let mut v5: Rc<str> = v5.clone();
                let v6: Rc<str> = v6.clone();
                let v7: u64 = *v7;
                let v8: Rc<UH1> = v8.clone();
                let mut v9: Rc<UH0> = v9.clone();
                let v10: bool = v3 < v7;
                let v12: bool = if v10 {
                    let v11: bool = &*v5 != &*v2;
                    v11
                } else {
                    false
                };
                let mut v15: US10 = if v12 {
                    first_overlap_31(v0.clone(), v8.clone())
                } else {
                    US10::US10_None
                };
                match &v15 {
                    US10::US10_None => {
                        v1 = v9.clone();
                        continue;
                    }
                    US10::US10_Some(v16) => {
                        let v16: Rc<str> = v16.clone();
                        return US9::US9_Some(v5.clone(), v16.clone());
                    }
                }
            }
            UH0::UH0_Nil => {
                return US9::US9_None;
            }
        }
    }
}
fn refusal_of_27(mut v0: Rc<UH0>, mut v1: US7, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH1>, mut v5: u64) -> US8 {
    let v6: US8 = lease_refusal_28(v0.clone(), v1, v2.clone(), v3, v5);
    match &v6 {
        US8::US8_None => {
            let mut v9: US9 = overlapping_claim_30(v4, v0, v2, v5);
            match &v9 {
                US9::US9_None => {
                    US8::US8_None
                }
                US9::US9_Some(v10, v11) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = v11.clone();
                    let v12: US5 = US5::US5_PathOverlap(v10, v11);
                    US8::US8_Some(v12)
                }
            }
        }
        US8::US8_Some(v7) => {
            let v7: US5 = v7.clone();
            US8::US8_Some(v7)
        }
    }
}
fn method36(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = &*v3 == &*v0;
                let mut v9: Rc<UH1> = if v8 {
                    v6.clone()
                } else {
                    v2.clone()
                };
                (v1, v2) = (v7.clone(), v9.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn held_paths_35(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method36(v1, v0, v2)
}
fn method38(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: u64 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH0> = v6.clone();
            let v7: Rc<UH0> = method38(v6, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    match &*v4 {
        UH0::UH0_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let v11: Rc<UH0> = method39(v0.clone(), v1, v2, v3, v10, v5.clone());
            let v12: bool = &*v6 != &*v0;
            if v12 {
                Rc::new(UH0::UH0_Cons(v6, v7, v8, v9, v11.clone()))
            } else {
                v11
            }
        }
        UH0::UH0_Nil => {
            v5
        }
    }
}
fn upserted_37(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH1>) -> Rc<UH0> {
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method39(v1.clone(), v2.clone(), v3, v4.clone(), v0, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v2, v3, v4, v7));
    method38(v6, v8)
}
fn position_of_40(mut v0: Rc<str>, mut v1: Rc<UH1>) -> i32 {
    let v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH1>) -> US6> = method26(v0, v2);
    let mut v4: US6 = v3(v1);
    let v7: i32 = match &v4 {
        US6::US6_None => {
            -1i32
        }
        US6::US6_Some(v5) => {
            let mut v5: i32 = *v5;
            v5
        }
    };
    let v8: i32 = v7.wrapping_add(1i32);
    v8
}
fn method41(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn claim_lease_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v10: Rc<UH0> = claims_of_text_9(v8);
    let v11: bool = &*v0 == "release";
    if v11 {
        let v12: u64 = (v3 as u64);
        let v13: u64 = v12.wrapping_mul(60000u64);
        let mut v14: u64 = v9.wrapping_add(v13);
        let mut v15: US4 = state_of_19(v10.clone(), v1.clone(), v9);
        match &v15 {
            US4::US4_LeasedTo(v16, v17, v18, v19) => {
                let v16: Rc<str> = v16.clone();
                let v17: Rc<str> = v17.clone();
                let v18: u64 = *v18;
                let v19: Rc<UH1> = v19.clone();
                let v20: bool = &*v17 == &*v2;
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                    declined_row_1(v1.clone(), v2.clone(), v14, v10.clone(), v9, v21)
                } else {
                    let v30: Rc<str> = decliner_7(v16.clone(), v17.clone(), v18, v19.clone());
                    let v31: bool = &*v30 != "";
                    if v31 {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                        declined_row_1(v1.clone(), v2.clone(), v14, v10.clone(), v9, v32)
                    } else {
                        let v41: US0 = US0::US0_OutcomeReleaseRefused;
                        let v42: Rc<str> = outcome_word_2(v41);
                        let v43: i32 = live_count_5(v10.clone(), v9);
                        let v44: US5 = US5::US5_LeasedBy(v16, v17.clone(), v18, v19);
                        let v45: Rc<str> = refusal_reason_21(v44);
                        let v46: US1 = US1::US1_ChangeNone;
                        let v47: Rc<str> = change_word_8(v46);
                        (v42, v1.clone(), v17, v18, 0i32, v43, v45, v47)
                    }
                }
            }
            US4::US4_Unclaimed => {
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                declined_row_1(v1.clone(), v2.clone(), v14, v10.clone(), v9, v64)
            }
            _ => {
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                declined_row_1(v1.clone(), v2.clone(), v14, v10.clone(), v9, v73)
            }
        }
    } else {
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v99: Rc<UH1> = words_24(v6, v98.clone());
        let v100: i32 = 0i32;
        let mut v101: Rc<dyn Fn(Rc<UH1>) -> US6> = method26(v1.clone(), v100);
        let v102: US6 = v101(v99);
        let v103: bool = match &v102 {
            US6::US6_None => {
                true
            }
            _ => {
                false
            }
        };
        let v104: bool = v103 != true;
        let v105: bool = v104 == false;
        if v105 {
            let v106: US0 = US0::US0_OutcomeUnknown;
            let v107: Rc<str> = outcome_word_2(v106);
            let v108: US1 = US1::US1_ChangeNone;
            let v109: Rc<str> = change_word_8(v108);
            let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v107, v1.clone(), v2.clone(), 0u64, 0i32, 0i32, v110, v109)
        } else {
            let v111: Rc<UH1> = words_24(v7, v98.clone());
            let v112: i32 = 0i32;
            let mut v113: Rc<dyn Fn(Rc<UH1>) -> US6> = method26(v1.clone(), v112);
            let v114: US6 = v113(v111);
            let v115: bool = match &v114 {
                US6::US6_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v116: bool = v115 != true;
            let v119: US7 = if v116 {
                US7::US7_LiveJob(v1.clone())
            } else {
                US7::US7_Idle
            };
            let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v121: Rc<UH1> = words_24(v4, v120);
            let v122: US8 = refusal_of_27(v10.clone(), v119, v1.clone(), v2.clone(), v121.clone(), v9);
            match &v122 {
                US8::US8_None => {
                    let v130: bool = match &*v121 {
                        UH1::UH1_Nil => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v132: Rc<UH1> = if v130 {
                        held_paths_35(v10.clone(), v1.clone())
                    } else {
                        v121
                    };
                    let v133: u64 = (v3 as u64);
                    let v134: u64 = v133.wrapping_mul(60000u64);
                    let v135: u64 = v9.wrapping_add(v134);
                    let v136: Rc<UH0> = upserted_37(v10.clone(), v1.clone(), v2.clone(), v135, v132.clone());
                    let v137: i32 = live_count_5(v136, v9);
                    let v138: Rc<UH1> = words_24(v5, v98);
                    let v139: i32 = position_of_40(v1.clone(), v138);
                    let v140: US0 = US0::US0_OutcomeGranted;
                    let v141: Rc<str> = outcome_word_2(v140);
                    let v142: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v143: i32 = 0i32;
                    let v144: i32 = method41(v142.clone(), v132, v143);
                    let v145: Rc<Vec<Rc<str>>> = Rc::new(v142.borrow().clone());
                    let v146: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v145).as_ref().clone()));
                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                    let v148: Rc<str> = Rc::<str>::from((*v146.borrow()).join(&*v147));
                    let v149: US1 = US1::US1_ChangeUpsert;
                    let v150: Rc<str> = change_word_8(v149);
                    (v141, v1.clone(), v2.clone(), v135, v139, v137, v148, v150)
                }
                US8::US8_Some(v123) => {
                    let v123: US5 = v123.clone();
                    let v124: US0 = US0::US0_OutcomeRefused;
                    let v125: Rc<str> = outcome_word_2(v124);
                    let v126: i32 = live_count_5(v10, v9);
                    let v127: Rc<str> = refusal_reason_21(v123);
                    let v128: US1 = US1::US1_ChangeNone;
                    let v129: Rc<str> = change_word_8(v128);
                    (v125, v1, v2, 0u64, 0i32, v126, v127, v129)
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        claim_lease_0(v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_lease(v0: &str, v1: &str, v2: &str, v3: i32, v4: &str, v5: &str, v6: &str, v7: &str, v8: &str, v9: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6), Rc::<str>::from(v7), Rc::<str>::from(v8), v9)
}
