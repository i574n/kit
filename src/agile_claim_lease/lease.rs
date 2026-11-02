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
    US0_Unclaimed,
    US0_LeasedTo(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US0_Expired(Rc<str>, Rc<str>, u64, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_OutcomeGranted,
    US2_OutcomeRefused,
    US2_OutcomeNone,
    US2_OutcomeDropped,
    US2_OutcomeReleaseRefused,
    US2_OutcomeUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_ChangeUpsert,
    US3_ChangeRemove,
    US3_ChangeNone,
}
#[derive(Clone)]
enum US4 {
    US4_LeasedBy(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US4_JobRunning(Rc<str>),
    US4_RecentEditOn(Rc<str>),
    US4_PathOverlap(Rc<str>, Rc<str>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US5 {
    US5_Idle,
    US5_LiveJob(Rc<str>),
    US5_RecentEdit(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(US4),
    US6_None,
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
struct Mut1 { l0: i32, l1: Rc<UH1> }
#[derive(Clone)]
enum US9 {
    US9_Some(u64),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(i32),
    US10_None,
}
fn method3(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = v4.clone();
            let mut v5: u64 = *v5;
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = method3(v0.clone(), v7, v2.clone());
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
fn released_2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v1, v0, v2)
}
fn method5(mut v0: u64, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = v0 < v5;
                let v10: i32 = if v8 {
                    let mut v9: i32 = v2.wrapping_add(1i32);
                    v9
                } else {
                    v2
                };
                (v1, v2) = (v7.clone(), v10);
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
fn claim_of_7(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US1 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: u64 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                let v8: bool = &*v3 == &*v1;
                if v8 {
                    return US1::US1_Some(v3.clone(), v4.clone(), v5, v6.clone());
                } else {
                    v0 = v7.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn state_of_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> US0 {
    let v3: US1 = claim_of_7(v0, v1);
    match &v3 {
        US1::US1_None => {
            US0::US0_Unclaimed
        }
        US1::US1_Some(v5, v6, v7, v8) => {
            let mut v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: u64 = *v7;
            let v8: Rc<UH1> = v8.clone();
            let mut v9: bool = v2 < v7;
            if v9 {
                US0::US0_LeasedTo(v5.clone(), v6.clone(), v7, v8.clone())
            } else {
                US0::US0_Expired(v5, v6, v7, v8)
            }
        }
    }
}
fn outcome_word_8(mut v0: US2) -> Rc<str> {
    let v7: Rc<str> = match &v0 {
        US2::US2_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_OutcomeGranted => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Granted"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_OutcomeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_OutcomeRefused => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Refused"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_OutcomeReleaseRefused => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ReleaseRefused"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_OutcomeUnknown => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Unknown"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    let v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn change_word_9(mut v0: US3) -> Rc<str> {
    let mut v4: Rc<str> = match &v0 {
        US3::US3_ChangeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US3::US3_ChangeRemove => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US3::US3_ChangeUpsert => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let mut v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn method12(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_11(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method12(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn refusal_reason_10(mut v0: US4) -> Rc<str> {
    match &v0 {
        US4::US4_JobRunning(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-running:"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
            v13
        }
        US4::US4_LeasedBy(v1, v2, v3, v4) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<str> = v2.clone();
            let v3: u64 = *v3;
            let mut v4: Rc<UH1> = v4.clone();
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leased-by:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = format_real_11(v3);
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10
        }
        US4::US4_PathOverlap(v17, v18) => {
            let mut v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("path-overlap:"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v17));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v21)), v18));
            v23
        }
        US4::US4_RecentEditOn(v14) => {
            let v14: Rc<str> = v14.clone();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("recent-edit:"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            v16
        }
    }
}
fn claim_dropped_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let mut v4: Rc<UH0> = released_2(v2.clone(), v0.clone());
    let mut v5: i32 = live_count_4(v4, v3);
    let v6: US0 = state_of_6(v2.clone(), v0.clone(), v3);
    match &v6 {
        US0::US0_Expired(v32, v33, v34, v35) => {
            let v32: Rc<str> = v32.clone();
            let v33: Rc<str> = v33.clone();
            let v34: u64 = *v34;
            let v35: Rc<UH1> = v35.clone();
            let v36: US2 = US2::US2_OutcomeDropped;
            let v37: Rc<str> = outcome_word_8(v36);
            let v38: US3 = US3::US3_ChangeRemove;
            let v39: Rc<str> = change_word_9(v38);
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
            (v37, v0.clone(), v1.clone(), 0u64, 0i32, v5, v40, v39)
        }
        US0::US0_LeasedTo(v7, v8, v9, v10) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = v8.clone();
            let mut v9: u64 = *v9;
            let v10: Rc<UH1> = v10.clone();
            let v11: bool = &*v8 == &*v1;
            if v11 {
                let v12: US2 = US2::US2_OutcomeDropped;
                let v13: Rc<str> = outcome_word_8(v12);
                let v14: US3 = US3::US3_ChangeRemove;
                let v15: Rc<str> = change_word_9(v14);
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
                (v13, v0.clone(), v1.clone(), 0u64, 0i32, v5, v16, v15)
            } else {
                let mut v17: US2 = US2::US2_OutcomeReleaseRefused;
                let v18: Rc<str> = outcome_word_8(v17);
                let v19: i32 = live_count_4(v2, v3);
                let v20: US4 = US4::US4_LeasedBy(v7, v8.clone(), v9, v10);
                let v21: Rc<str> = refusal_reason_10(v20);
                let mut v22: US3 = US3::US3_ChangeNone;
                let v23: Rc<str> = change_word_9(v22);
                (v18, v0.clone(), v8, v9, 0i32, v19, v21, v23)
            }
        }
        US0::US0_Unclaimed => {
            let v41: US2 = US2::US2_OutcomeDropped;
            let v42: Rc<str> = outcome_word_8(v41);
            let v43: US3 = US3::US3_ChangeNone;
            let v44: Rc<str> = change_word_9(v43);
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            (v42, v0, v1, 0u64, 0i32, v5, v45, v44)
        }
    }
}
fn lease_refusal_15(mut v0: Rc<UH0>, mut v1: US5, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US6 {
    let mut v5: US0 = state_of_6(v0, v2, v4);
    match &v5 {
        US0::US0_LeasedTo(v6, v7, v8, v9) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH1> = v9.clone();
            let v10: bool = &*v7 == &*v3;
            if v10 {
                US6::US6_None
            } else {
                let v12: US4 = US4::US4_LeasedBy(v6, v7, v8, v9);
                US6::US6_Some(v12)
            }
        }
        _ => {
            match &v1 {
                US5::US5_Idle => {
                    US6::US6_None
                }
                US5::US5_LiveJob(v15) => {
                    let v15: Rc<str> = v15.clone();
                    let mut v16: US4 = US4::US4_JobRunning(v15);
                    US6::US6_Some(v16)
                }
                US5::US5_RecentEdit(v18) => {
                    let v18: Rc<str> = v18.clone();
                    let v19: US4 = US4::US4_RecentEditOn(v18);
                    US6::US6_Some(v19)
                }
            }
        }
    }
}
fn method21(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn glob_stem_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method21(v2.clone());
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
fn globs_overlap_19(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = glob_stem_20(v0);
    let v3: Rc<str> = glob_stem_20(v1);
    let mut v4: bool = v3.starts_with(&*v2);
    if v4 {
        true
    } else {
        let mut v5: bool = v2.starts_with(&*v3);
        v5
    }
}
fn overlap_in_18(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US8 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: bool = globs_overlap_19(v0.clone(), v3.clone());
                if v5 {
                    return US8::US8_Some(v0.clone());
                } else {
                    v1 = v4.clone();
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn first_overlap_17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> US8 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: US8 = overlap_in_18(v3.clone(), v1.clone());
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
            UH1::UH1_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn overlapping_claim_16(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64) -> US7 {
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
                let v15: US8 = if v12 {
                    first_overlap_17(v0.clone(), v8.clone())
                } else {
                    US8::US8_None
                };
                match &v15 {
                    US8::US8_None => {
                        v1 = v9.clone();
                        continue;
                    }
                    US8::US8_Some(v16) => {
                        let mut v16: Rc<str> = v16.clone();
                        return US7::US7_Some(v5.clone(), v16.clone());
                    }
                }
            }
            UH0::UH0_Nil => {
                return US7::US7_None;
            }
        }
    }
}
fn refusal_of_14(mut v0: Rc<UH0>, mut v1: US5, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH1>, mut v5: u64) -> US6 {
    let v6: US6 = lease_refusal_15(v0.clone(), v1, v2.clone(), v3, v5);
    match &v6 {
        US6::US6_None => {
            let mut v9: US7 = overlapping_claim_16(v4, v0, v2, v5);
            match &v9 {
                US7::US7_None => {
                    US6::US6_None
                }
                US7::US7_Some(v10, v11) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = v11.clone();
                    let v12: US4 = US4::US4_PathOverlap(v10, v11);
                    US6::US6_Some(v12)
                }
            }
        }
        US6::US6_Some(v7) => {
            let v7: US4 = v7.clone();
            US6::US6_Some(v7)
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3.clone();
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
fn held_paths_22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method23(v1, v0, v2)
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let mut v4: u64 = *v4;
            let mut v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH0> = v6.clone();
            let v7: Rc<UH0> = method25(v6, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    match &*v4 {
        UH0::UH0_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let v11: Rc<UH0> = method26(v0.clone(), v1, v2, v3, v10, v5.clone());
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
fn upserted_24(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH1>) -> Rc<UH0> {
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method26(v1.clone(), v2.clone(), v3, v4.clone(), v0, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v2, v3, v4, v7));
    method25(v6, v8)
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = method28(v3, v1.clone());
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
fn method30(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
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
fn method31(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8.clone() }));
        while method31(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let mut v16: Rc<UH1> = v9.borrow().l1.clone();
        let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method30(v16.clone(), v17.clone())
    }
}
fn words_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: Rc<UH1> = split_list_29(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method28(v2, v3)
}
fn position_of_32(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: bool = &*v3 == &*v0;
                if v5 {
                    return v2;
                } else {
                    let v6: i32 = v2.wrapping_add(1i32);
                    (v1, v2) = (v4.clone(), v6);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return 0i32;
            }
        }
    }
}
fn method33(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
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
fn claim_taken_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<str>, mut v5: Rc<UH0>, mut v6: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let v7: US5 = US5::US5_Idle;
    let v8: US6 = refusal_of_14(v5.clone(), v7, v0.clone(), v1.clone(), v3.clone(), v6);
    match &v8 {
        US6::US6_None => {
            let mut v16: bool = match &*v3 {
                UH1::UH1_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            let v18: Rc<UH1> = if v16 {
                held_paths_22(v5.clone(), v0.clone())
            } else {
                v3
            };
            let v19: u64 = (v2 as u64);
            let v20: u64 = v19.wrapping_mul(60000u64);
            let v21: u64 = v6.wrapping_add(v20);
            let mut v22: Rc<UH0> = upserted_24(v5.clone(), v0.clone(), v1.clone(), v21, v18.clone());
            let v23: i32 = live_count_4(v22, v6);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<UH1> = words_27(v4, v24);
            let v26: i32 = 1i32;
            let v27: i32 = position_of_32(v0.clone(), v25, v26);
            let v28: US2 = US2::US2_OutcomeGranted;
            let v29: Rc<str> = outcome_word_8(v28);
            let mut v30: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let mut v31: i32 = 0i32;
            let v32: i32 = method33(v30.clone(), v18, v31);
            let v33: Rc<Vec<Rc<str>>> = Rc::new(v30.borrow().clone());
            let v34: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v33).as_ref().clone()));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from((*v34.borrow()).join(&*v35));
            let v37: US3 = US3::US3_ChangeUpsert;
            let v38: Rc<str> = change_word_9(v37);
            (v29, v0.clone(), v1.clone(), v21, v27, v23, v36, v38)
        }
        US6::US6_Some(v9) => {
            let mut v9: US4 = v9.clone();
            let v10: US2 = US2::US2_OutcomeRefused;
            let v11: Rc<str> = outcome_word_8(v10);
            let v12: i32 = live_count_4(v5, v6);
            let v13: Rc<str> = refusal_reason_10(v9);
            let v14: US3 = US3::US3_ChangeNone;
            let v15: Rc<str> = change_word_9(v14);
            (v11, v0, v1, 0u64, 0i32, v12, v13, v15)
        }
    }
}
fn method35(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
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
fn method38(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US9> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US9> = Rc::new(move |mut v0: (u64)| -> US9 {
        let mut v1: u64 = (v0);
        US9::US9_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn every_other_39(mut v0: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<UH1> = v2.clone();
            match &*v2 {
                UH1::UH1_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let mut v4: Rc<UH1> = v4.clone();
                    let mut v5: Rc<UH1> = every_other_39(v4);
                    Rc::new(UH1::UH1_Cons(v1, v5))
                }
                _ => {
                    v0.clone()
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn claim_of_row_37(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH1> = split_list_29(v1.clone(), v0.clone());
    match &*v2 {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH1> = v4.clone();
            match &*v4 {
                UH1::UH1_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5.clone();
                    let v6: Rc<UH1> = v6.clone();
                    match &*v6 {
                        UH1::UH1_Cons(v7, v8) => {
                            let v7: Rc<str> = v7.clone();
                            let v8: Rc<UH1> = v8.clone();
                            match &*v8 {
                                UH1::UH1_Cons(v9, v10) => {
                                    let mut v9: Rc<str> = v9.clone();
                                    let v10: Rc<UH1> = v10.clone();
                                    match &*v10 {
                                        UH1::UH1_Cons(v11, v12) => {
                                            let v11: Rc<str> = v11.clone();
                                            let v12: Rc<UH1> = v12.clone();
                                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
                                            let v14: Option<usize> = v11.find(&*v13);
                                            let v15: Option<usize> = method21(v14.clone());
                                            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
                                            let mut v17: Option<i32> = v15.map(|x| v16(x));
                                            let v19: i32 = v17.unwrap_or(-1i32);
                                            let v20: bool = v19 > 2i32;
                                            let v27: Rc<str> = if v20 {
                                                let v21: i32 = v19.wrapping_sub(2i32);
                                                let mut v22: usize = ((2i32) as usize);
                                                let v23: i32 = 2i32.wrapping_add(v21);
                                                let v24: usize = ((v23) as usize);
                                                let v25: Rc<str> = Rc::<str>::from(&(v11)[v22..v24]);
                                                v25.clone()
                                            } else {
                                                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                v26.clone()
                                            };
                                            let v28: Option<u64> = (v27).trim().parse::<u64>().ok();
                                            let v29: Option<u64> = method38(v28.clone());
                                            let mut v30: Rc<dyn Fn((u64)) -> US9> = closure2();
                                            let mut v31: Option<US9> = v29.map(|x| v30(x));
                                            let v33: US9 = US9::US9_None;
                                            let v34: US9 = v33.clone();
                                            let v35: US9 = v31.unwrap_or(v34);
                                            match &v35 {
                                                US9::US9_None => {
                                                    US1::US1_None
                                                }
                                                US9::US9_Some(v36) => {
                                                    let v36: u64 = *v36;
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
                                                    let v38: bool = v3.starts_with(&*v37);
                                                    if v38 {
                                                        let v39: Rc<UH1> = every_other_39(v12.clone());
                                                        US1::US1_Some(v5.clone(), v9.clone(), v36, v39.clone())
                                                    } else {
                                                        US1::US1_None
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            US1::US1_None
                                        }
                                    }
                                }
                                _ => {
                                    US1::US1_None
                                }
                            }
                        }
                        _ => {
                            US1::US1_None
                        }
                    }
                }
                _ => {
                    US1::US1_None
                }
            }
        }
        _ => {
            US1::US1_None
        }
    }
}
fn method36(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let mut v4: US1 = claim_of_row_37(v2.clone());
                let v11: Rc<UH0> = match &v4 {
                    US1::US1_None => {
                        v1.clone()
                    }
                    US1::US1_Some(v5, v6, v7, v8) => {
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
fn claims_of_text_34(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH1> = split_list_29(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method36(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method35(v7, v8)
}
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US10> {
    Rc::new(move |mut v2: Rc<UH1>| -> US10 {
        match &*v2 {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US10::US10_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US10> = method40(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH1::UH1_Nil => {
                US10::US10_None
            }
        }
    })
}
fn method40(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US10> {
    closure3(v0, v1)
}
fn claim_lease_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    let mut v9: Rc<UH0> = claims_of_text_34(v7);
    let v10: bool = &*v0 == "release";
    if v10 {
        claim_dropped_1(v1.clone(), v2.clone(), v9.clone(), v8)
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<UH1> = words_27(v6, v19);
        let v21: i32 = 0i32;
        let mut v22: Rc<dyn Fn(Rc<UH1>) -> US10> = method40(v1.clone(), v21);
        let v23: US10 = v22(v20);
        let v24: bool = match &v23 {
            US10::US10_None => {
                true
            }
            _ => {
                false
            }
        };
        let v25: bool = v24 != true;
        let v26: bool = v25 == false;
        if v26 {
            let v27: US2 = US2::US2_OutcomeUnknown;
            let v28: Rc<str> = outcome_word_8(v27);
            let v29: US3 = US3::US3_ChangeNone;
            let mut v30: Rc<str> = change_word_9(v29);
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v28, v1.clone(), v2.clone(), 0u64, 0i32, 0i32, v31, v30)
        } else {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<UH1> = words_27(v4, v32);
            claim_taken_13(v1, v2, v3, v33, v5, v9, v8)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: u64| -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
        claim_lease_0(v0.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claim_lease(v0: &str, v1: &str, v2: &str, v3: i32, v4: &str, v5: &str, v6: &str, v7: &str, v8: u64) -> (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6), Rc::<str>::from(v7), v8)
}
