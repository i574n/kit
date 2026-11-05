#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
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
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
struct Mut1 { l0: i32 }
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method4(v3, v1.clone());
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
fn method5(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
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
fn printed_3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method4(v0.clone(), v2.clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method5(v4.clone(), v3.clone(), v5);
    let v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    let v11: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v12: bool = v11 != 0i32;
    if v12 {
        v11
    } else {
        v1
    }
}
fn outcome_word_7(mut v0: US0) -> Rc<str> {
    let v7: Rc<str> = match &v0 {
        US0::US0_OutcomeDropped => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped"); } LIT.with(|lit| lit.clone()) };
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
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn outcome_of_6(mut v0: Rc<str>) -> US0 {
    let v1: US0 = US0::US0_OutcomeGranted;
    let v2: Rc<str> = outcome_word_7(v1);
    let v3: bool = &*v2 == &*v0;
    if v3 {
        US0::US0_OutcomeGranted
    } else {
        let v5: US0 = US0::US0_OutcomeRefused;
        let v6: Rc<str> = outcome_word_7(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US0::US0_OutcomeRefused
        } else {
            let mut v9: US0 = US0::US0_OutcomeNone;
            let mut v10: Rc<str> = outcome_word_7(v9);
            let v11: bool = &*v10 == &*v0;
            if v11 {
                US0::US0_OutcomeNone
            } else {
                let v13: US0 = US0::US0_OutcomeDropped;
                let v14: Rc<str> = outcome_word_7(v13);
                let v15: bool = &*v14 == &*v0;
                if v15 {
                    US0::US0_OutcomeDropped
                } else {
                    let v17: US0 = US0::US0_OutcomeReleaseRefused;
                    let v18: Rc<str> = outcome_word_7(v17);
                    let v19: bool = &*v18 == &*v0;
                    if v19 {
                        US0::US0_OutcomeReleaseRefused
                    } else {
                        let v21: US0 = US0::US0_OutcomeUnknown;
                        let v22: Rc<str> = outcome_word_7(v21);
                        US0::US0_OutcomeUnknown
                    }
                }
            }
        }
    }
}
fn method9(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_8(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method9(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn concat5_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn civil_of_days_13(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
    let v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let mut v10: u64 = v8.wrapping_sub(v9);
    let v11: u64 = v10.wrapping_div(365u64);
    let v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let v25: u64 = v24.wrapping_add(1u64);
    let v26: bool = v20 < 10u64;
    let v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_11(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_13(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_12(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_12(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_12(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_12(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_12(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_12(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_12(v24, v45, v44, v43.clone(), v43)
}
fn receipt_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32) -> Rc<str> {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok claimed="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v1));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v2));
    let v15: Rc<str> = utc_stamp_11(v3);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expires="); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
    let v18: bool = v6 > 0i32;
    let v23: Rc<str> = if v18 {
        let v19: Rc<str> = format_real_8(v6);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rank="); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
        v21
    } else {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v17));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ttl_min="); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: Rc<str> = format_real_8(v4);
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = format_real_8(v5);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v30, v31)), v23));
    v33
}
fn claim_answered_2(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<UH0>, mut v7: Rc<str>, mut v8: bool, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>) -> i32 {
    let v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = if v0 {
        let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_queue::kit_claim_next(&*v4, v5, &*v10, &*v11, &*v13, &*v9, v14);
        (v15.clone(), v16.clone(), v17.clone(), v18, v19, v20, v21.clone(), v22.clone())
    } else {
        if v1 {
            let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_lease::kit_claim_release(&*v3, &*v4, v5, &*v9, v14);
            (v23.clone(), v24.clone(), v25.clone(), v26, v27, v28, v29.clone(), v30.clone())
        } else {
            let v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v32: i32 = 0i32;
            let v33: i32 = method5(v31.clone(), v6.clone(), v32);
            let v34: Rc<Vec<Rc<str>>> = Rc::new(v31.borrow().clone());
            let v35: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v34).as_ref().clone()));
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = Rc::<str>::from((*v35.borrow()).join(&*v36));
            let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_lease::kit_claim_lease(&*v3, &*v4, v5, &*v37, &*v10, &*v12, &*v13, &*v9, v14);
            (v38.clone(), v39.clone(), v40.clone(), v41, v42, v43, v44.clone(), v45.clone())
        }
    };
    let v62: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v7.clone(), v8, v9.clone(), v61.clone(), v55.clone(), v56.clone(), v57, v60.clone(), v12.clone())]));
    let v63: Rc<str> = agile_claim_store::kit_claims_applied(v62);
    let v64: bool = &*v63 != "";
    if v64 {
        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v2));
        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v66, v67)), v63));
        let v70: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v69 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v70
    } else {
        let v71: US0 = outcome_of_6(v54.clone());
        match &v71 {
            US0::US0_OutcomeDropped => {
                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile release ok id="); } LIT.with(|lit| lit.clone()) };
                let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v3));
                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
                let v108: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v105, v106)), v4));
                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" released="); } LIT.with(|lit| lit.clone()) };
                let v111: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v108, v109)), v60));
                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
                let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v112));
                let v114: Rc<str> = format_real_8(v59);
                let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v113, v114));
                let v116: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v117: Rc<UH0> = Rc::new(UH0::UH0_Cons(v115.clone(), v116.clone()));
                let v118: i32 = 0i32;
                printed_3(v117.clone(), v118)
            }
            US0::US0_OutcomeGranted => {
                let v72: bool = &*v60 == "";
                let v76: Rc<str> = if v72 {
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths none"); } LIT.with(|lit| lit.clone()) };
                    v73.clone()
                } else {
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths "); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v60));
                    v75.clone()
                };
                let v77: Rc<str> = receipt_10(v2.clone(), v55.clone(), v56.clone(), v57, v5, v59, v58);
                let v78: Rc<str> = agile_claim_item::kit_claim_item_lines(&*v7, &*v55);
                let v79: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v80: Rc<UH0> = Rc::new(UH0::UH0_Cons(v76.clone(), v79.clone()));
                let v81: Rc<UH0> = Rc::new(UH0::UH0_Cons(v78.clone(), v80.clone()));
                let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v77.clone(), v81.clone()));
                let v83: i32 = 0i32;
                printed_3(v82.clone(), v83)
            }
            US0::US0_OutcomeNone => {
                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile next ok claimed=none reason="); } LIT.with(|lit| lit.clone()) };
                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v60));
                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
                let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
                let v98: Rc<str> = format_real_8(v59);
                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v98));
                let v100: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v101: Rc<UH0> = Rc::new(UH0::UH0_Cons(v99.clone(), v100.clone()));
                let v102: i32 = 3i32;
                printed_3(v101.clone(), v102)
            }
            US0::US0_OutcomeRefused => {
                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile claim refused id="); } LIT.with(|lit| lit.clone()) };
                let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v3));
                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v86, v87)), v60));
                let v90: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v91: Rc<UH0> = Rc::new(UH0::UH0_Cons(v89.clone(), v90.clone()));
                let v92: i32 = 5i32;
                printed_3(v91.clone(), v92)
            }
            US0::US0_OutcomeReleaseRefused => {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile release refused id="); } LIT.with(|lit| lit.clone()) };
                let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v3));
                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
                let v124: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v121, v122)), v60));
                let v125: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v126: Rc<UH0> = Rc::new(UH0::UH0_Cons(v124.clone(), v125.clone()));
                let v127: i32 = 5i32;
                printed_3(v126.clone(), v127)
            }
            US0::US0_OutcomeUnknown => {
                let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no open task with id "); } LIT.with(|lit| lit.clone()) };
                let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v3));
                let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v2));
                let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v135: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v132, v133)), v130));
                let v136: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v135 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v136
            }
        }
    }
}
fn method15(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn get_current_directory_14() -> Rc<str> {
    let v1: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
    let v3: std::path::PathBuf = v1.unwrap();
    let mut v4: std::path::PathBuf = method15(v3);
    let v6: std::path::Display = v4.display();
    let mut v8: std::string::String = format!("{}", v6);
    let mut v10: Rc<str> = Rc::<str>::from(String::as_str(&v8));
    v10
}
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn anchored_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: bool = v4.starts_with(&*v3);
    let v14: bool = if v5 {
        true
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
        let v7: Option<usize> = v4.find(&*v6);
        let mut v8: Option<usize> = method18(v7.clone());
        let mut v9: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v10: Option<i32> = v8.map(|x| v9(x));
        let v12: i32 = v10.unwrap_or(-1i32);
        let v13: bool = v12 == 1i32;
        v13
    };
    let v15: i32 = (v4.clone().len() as i32);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("./"); } LIT.with(|lit| lit.clone()) };
    let v17: bool = v4.starts_with(&*v16);
    let v23: Rc<str> = if v17 {
        let v18: i32 = v15.wrapping_sub(2i32);
        let v19: usize = ((2i32) as usize);
        let v20: i32 = 2i32.wrapping_add(v18);
        let v21: usize = ((v20) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v4)[v19..v21]);
        v22.clone()
    } else {
        v4.clone()
    };
    if v14 {
        v4.clone()
    } else {
        let v24: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v3)), v23));
        v26.clone()
    }
}
fn method16(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method16(v0.clone(), v4, v2.clone());
            let v6: Rc<str> = anchored_17(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn claim_requested_1(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<UH0>) -> i32 {
    let mut v8: Rc<str> = get_current_directory_14();
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH0> = method16(v8.clone(), v7.clone(), v9.clone());
    let (mut v11, mut v12, mut v13, mut v14, mut v15): (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = agile_claim_rank::kit_claim_ranking(&*v0);
    let v16: bool = v11 == false;
    if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v3));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
        let v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v21
    } else {
        let (mut v22, mut v23): (bool, Rc<str>) = agile_claim_store::kit_claims_loaded(&*v0);
        claim_answered_2(v1, v2, v3.clone(), v4.clone(), v5.clone(), v6, v10.clone(), v0.clone(), v22, v23.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone())
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_21(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_19(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_20();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_21(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match &v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_19(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method23(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method23(v7, v9.clone()) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn listed_paths_25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v1, v2) = (v4, v6.clone());
            continue;
        }
    }
}
fn order_of_24(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<RefCell<Vec<Rc<str>>>>) -> (bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) {
    let mut v8: i32 = (v7.clone().borrow().len() as i32);
    let mut v9: i32 = v8.wrapping_sub(1i32);
    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = listed_paths_25(v7, v9, v10);
    (v1, v2, v3, v4, v5, v6, v11)
}
fn claim_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_19(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    match &*v5 {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<UH0> = v7.clone();
                            let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
                            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = method22(v7.clone(), v8.clone());
                            let mut v10: Rc<RefCell<Vec<(Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>)>>> = agile_claim_request::agile_claim_request_parsed(&v4, v9);
                            let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<RefCell<Vec<Rc<str>>>>) = v10.borrow()[0i32 as usize].clone();
                            let v19: bool = &*v11 != "";
                            if v19 {
                                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v4));
                                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v22)), v11));
                                let v25: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v24 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v25
                            } else {
                                let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (bool, bool, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>) = order_of_24(v11.clone(), v12, v13, v14.clone(), v15.clone(), v16.clone(), v17, v18.clone());
                                claim_requested_1(v6.clone(), v26, v27, v28.clone(), v29.clone(), v30.clone(), v31, v32.clone())
                            }
                        }
                        _ => {
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
                            let v36: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v35 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v36
                        }
                    }
                }
                _ => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
                    let v39: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v38 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v39
                }
            }
        }
        _ => {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile: usage: next|claim|release <root> ..."); } LIT.with(|lit| lit.clone()) };
            let v42: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v41 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v42
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        claim_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_claim_run() -> i32 {
    closure0()()
}
