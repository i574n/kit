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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
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
pub struct Mut1 { pub l0: Rc<str> }
fn method3(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: u64 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH0> = v7;
            let v8: Rc<UH0> = method3(v0.clone(), v7, v2);
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn released_2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v1, v0, v2)
}
fn decliner_6(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH1>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
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
fn method5(mut v0: u64, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: u64 = v5;
                let v6: Rc<UH1> = v6;
                let v7: Rc<UH0> = v7;
                let v8: bool = v0 < v5;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_6(&v3, v4, v5, &v6);
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
                (v1, v2) = (v7, v13);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn live_count_4(mut v0: Rc<UH0>, mut v1: u64) -> i32 {
    let v2: i32 = 0i32;
    method5(v1, v0, v2)
}
fn outcome_word_7(mut v0: US0) -> Rc<str> {
    let v7: Rc<str> = match v0 {
        US0::US0_OutcomeDropped => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_OutcomeGranted => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_OutcomeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_OutcomeReleaseRefused => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
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
fn change_word_8(mut v0: US1) -> Rc<str> {
    let v4: Rc<str> = match v0 {
        US1::US1_ChangeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US1::US1_ChangeRemove => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US1::US1_ChangeUpsert => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn declined_row_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH0>, mut v4: u64, mut v5: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v6: Rc<UH0> = released_2(v3, v0.clone());
    let v7: i32 = live_count_4(v6, v4);
    let v8: US0 = US0::US0_OutcomeDropped;
    let mut v9: Rc<str> = outcome_word_7(v8);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) }, v1));
    let v11: US1 = US1::US1_ChangeUpsert;
    let v12: Rc<str> = change_word_8(v11);
    (v9, v0, v10, v2, 0i32, v7, v5, v12)
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method13(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method12(v16, v17)
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_18(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH1> {
    let v2: i32 = (v0.clone().borrow().len() as i32);
    let v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH1> = globs_from_18(v0, v6);
        Rc::new(UH1::UH1_Cons(v5, v7))
    }
}
fn claim_of_row_15(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 5i32;
    let v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
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
        let v13: Option<usize> = method16(v12);
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
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        let v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method17(v26);
        let mut v28: Rc<dyn Fn((u64)) -> US3> = closure2();
        let mut v29: Option<US3> = v27.map(|x| v28(x));
        let v31: US3 = US3::US3_None;
        let v32: US3 = v31.clone();
        let v33: US3 = v29.unwrap_or(v32);
        match v33 {
            US3::US3_None => {
                US2::US2_None
            }
            US3::US3_Some(v34) => {
                let v34: u64 = v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH1> = globs_from_18(&v2, v37);
                US2::US2_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method14(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US2 = claim_of_row_15(v2);
                let v11: Rc<UH0> = match v4 {
                    US2::US2_None => {
                        v1
                    }
                    US2::US2_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        let v8: Rc<UH1> = v8;
                        Rc::new(UH0::UH0_Cons(v5, v6, v7, v8, v1))
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn claims_of_text_9(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_11(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method14(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method10(v7, v8)
}
fn claim_of_20(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: u64 = v5;
                let v6: Rc<UH1> = v6;
                let v7: Rc<UH0> = v7;
                let v8: bool = &*v3 == &*v1;
                let v11: bool = if v8 {
                    let mut v9: Rc<str> = decliner_6(&v3, v4.clone(), v5, &v6);
                    let v10: bool = &*v9 == "";
                    v10
                } else {
                    false
                };
                if v11 {
                    return US2::US2_Some(v3, v4, v5, v6);
                } else {
                    v0 = v7;
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
    let v3: US2 = claim_of_20(v0, v1);
    match v3 {
        US2::US2_None => {
            US4::US4_Unclaimed
        }
        US2::US2_Some(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: u64 = v7;
            let v8: Rc<UH1> = v8;
            let mut v9: bool = v2 < v7;
            if v9 {
                US4::US4_LeasedTo(v5, v6, v7, v8)
            } else {
                US4::US4_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn method23(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn refusal_reason_21(mut v0: US5) -> Rc<str> {
    match v0 {
        US5::US5_DeclinedByYou(v18, v19, v20, v21) => {
            let v18: Rc<str> = v18;
            let v19: Rc<str> = v19;
            let v20: u64 = v20;
            let v21: Rc<UH1> = v21;
            let v22: Rc<str> = format_real_22(v20);
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("declined-by-you-until:"); } LIT.with(|lit| lit.clone()) }, v22));
            v23
        }
        US5::US5_JobRunning(v9) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-running:"); } LIT.with(|lit| lit.clone()) }, v9));
            v10
        }
        US5::US5_LeasedBy(v1, v2, v3, v4) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = v2;
            let v3: u64 = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leased-by:"); } LIT.with(|lit| lit.clone()) }, v2));
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v7: Rc<str> = format_real_22(v3);
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            v8
        }
        US5::US5_PathOverlap(v13, v14) => {
            let v13: Rc<str> = v13;
            let mut v14: Rc<str> = v14;
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("path-overlap:"); } LIT.with(|lit| lit.clone()) }, v13));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v14));
            v17
        }
        US5::US5_RecentEditOn(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recent-edit:"); } LIT.with(|lit| lit.clone()) }, v11));
            v12
        }
    }
}
fn lease_released_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v5: Rc<UH0> = claims_of_text_9(v3);
    let v6: u64 = (v2 as u64);
    let v7: u64 = v6.wrapping_mul(60000u64);
    let v8: u64 = v4.wrapping_add(v7);
    let mut v9: US4 = state_of_19(v5.clone(), v0.clone(), v4);
    match v9 {
        US4::US4_LeasedTo(v10, v11, v12, v13) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            let v12: u64 = v12;
            let v13: Rc<UH1> = v13;
            let mut v14: bool = &*v11 == &*v1;
            if v14 {
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                declined_row_1(v0, v1, v8, v5, v4, v15)
            } else {
                let v24: i32 = live_count_4(v5, v4);
                let v25: US5 = US5::US5_LeasedBy(v10, v11.clone(), v12, v13);
                let v26: Rc<str> = refusal_reason_21(v25);
                let v27: US0 = US0::US0_OutcomeReleaseRefused;
                let mut v28: Rc<str> = outcome_word_7(v27);
                let mut v29: US1 = US1::US1_ChangeNone;
                let v30: Rc<str> = change_word_8(v29);
                (v28, v0, v11, v12, 0i32, v24, v26, v30)
            }
        }
        US4::US4_Unclaimed => {
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            declined_row_1(v0, v1, v8, v5, v4, v39)
        }
        _ => {
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
            declined_row_1(v0, v1, v8, v5, v4, v48)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, i32, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        lease_released_0(v0, v1, v2, v3, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_release(v0: &str, v1: &str, v2: i32, v3: &str, v4: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2, Rc::<str>::from(v3), v4)
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
