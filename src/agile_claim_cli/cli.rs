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
    US1_RequestRead(Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>),
    US1_RequestRefused(Rc<str>),
}
struct Mut1 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method3(v3, v1.clone());
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
fn method4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
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
fn printed_2(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method3(v0.clone(), v2.clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method4(v4.clone(), v3.clone(), v5);
    let v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    let v11: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v12: bool = v11 != 0i32;
    if v12 {
        v11
    } else {
        v1
    }
}
fn outcome_word_6(mut v0: US0) -> Rc<str> {
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
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
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
fn outcome_of_5(mut v0: Rc<str>) -> US0 {
    let v1: US0 = US0::US0_OutcomeGranted;
    let v2: Rc<str> = outcome_word_6(v1);
    let mut v3: bool = &*v2 == &*v0;
    if v3 {
        US0::US0_OutcomeGranted
    } else {
        let v5: US0 = US0::US0_OutcomeRefused;
        let v6: Rc<str> = outcome_word_6(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US0::US0_OutcomeRefused
        } else {
            let mut v9: US0 = US0::US0_OutcomeNone;
            let v10: Rc<str> = outcome_word_6(v9);
            let v11: bool = &*v10 == &*v0;
            if v11 {
                US0::US0_OutcomeNone
            } else {
                let v13: US0 = US0::US0_OutcomeDropped;
                let v14: Rc<str> = outcome_word_6(v13);
                let v15: bool = &*v14 == &*v0;
                if v15 {
                    US0::US0_OutcomeDropped
                } else {
                    let v17: US0 = US0::US0_OutcomeReleaseRefused;
                    let v18: Rc<str> = outcome_word_6(v17);
                    let v19: bool = &*v18 == &*v0;
                    if v19 {
                        US0::US0_OutcomeReleaseRefused
                    } else {
                        let v21: US0 = US0::US0_OutcomeUnknown;
                        let v22: Rc<str> = outcome_word_6(v21);
                        US0::US0_OutcomeUnknown
                    }
                }
            }
        }
    }
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn concat5_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn civil_of_days_12(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
    let v2: u64 = v1.wrapping_div(146097u64);
    let mut v3: u64 = v2.wrapping_mul(146097u64);
    let mut v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let mut v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
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
fn utc_stamp_10(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_12(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let mut v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_11(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_11(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_11(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_11(v27, v28, v29.clone(), v29.clone(), v29)
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
        concat5_11(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_11(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_11(v24, v45, v44, v43.clone(), v43)
}
fn receipt_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: i32, mut v5: i32, mut v6: i32) -> Rc<str> {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile "); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok claimed="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v1));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v2));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expires="); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let v17: Rc<str> = utc_stamp_10(v3);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ttl_min="); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = format_real_7(v4);
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
    let v25: Rc<str> = format_real_7(v5);
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: bool = v6 > 0i32;
    let v32: Rc<str> = if v27 {
        let v28: Rc<str> = format_real_7(v6);
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rank="); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
        v30
    } else {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v31
    };
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v32));
    v33
}
fn claim_answered_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH0>, mut v5: Rc<str>, mut v6: bool, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>) -> i32 {
    let v11: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v12: bool = &*v0 == "next";
    let (mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = if v12 {
        let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_queue::kit_claim_next(&*v2, v3, &*v8, &*v9, &*v7, v11);
        (v13.clone(), v14.clone(), v15.clone(), v16, v17, v18, v19.clone(), v20.clone())
    } else {
        let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v22: i32 = 0i32;
        let v23: i32 = method4(v21.clone(), v4.clone(), v22);
        let v24: Rc<Vec<Rc<str>>> = Rc::new(v21.borrow().clone());
        let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from((*v25.borrow()).join(&*v26));
        let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, Rc<str>, u64, i32, i32, Rc<str>, Rc<str>) = agile_claim_lease::kit_claim_lease(&*v0, &*v1, &*v2, v3, &*v27, &*v8, &*v10, &*v7, v11);
        (v28.clone(), v29.clone(), v30.clone(), v31, v32, v33, v34.clone(), v35.clone())
    };
    let v44: Rc<str> = agile_claim_store::kit_claims_applied(&*v5, v6, &*v7, &*v43, &*v37, &*v38, v39, &*v42, &*v10);
    let v45: bool = &*v44 != "";
    if v45 {
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v0));
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v47, v48)), v44));
        let v51: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v50 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v51
    } else {
        let v52: US0 = outcome_of_5(v36.clone());
        match &v52 {
            US0::US0_OutcomeDropped => {
                let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile release ok id="); } LIT.with(|lit| lit.clone()) };
                let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v1));
                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" owner="); } LIT.with(|lit| lit.clone()) };
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v86, v87)), v2));
                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" released="); } LIT.with(|lit| lit.clone()) };
                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v89, v90)), v42));
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
                let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                let v95: Rc<str> = format_real_7(v41);
                let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v95));
                let v97: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v98: Rc<UH0> = Rc::new(UH0::UH0_Cons(v96.clone(), v97.clone()));
                let v99: i32 = 0i32;
                printed_2(v98.clone(), v99)
            }
            US0::US0_OutcomeGranted => {
                let v53: bool = &*v42 == "";
                let v57: Rc<str> = if v53 {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths none"); } LIT.with(|lit| lit.clone()) };
                    v54.clone()
                } else {
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths "); } LIT.with(|lit| lit.clone()) };
                    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v42));
                    v56.clone()
                };
                let v58: Rc<str> = receipt_9(v0.clone(), v37.clone(), v38.clone(), v39, v3, v41, v40);
                let v59: Rc<str> = agile_claim_item::kit_claim_item_lines(&*v5, &*v37);
                let v60: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57.clone(), v60.clone()));
                let v62: Rc<UH0> = Rc::new(UH0::UH0_Cons(v59.clone(), v61.clone()));
                let v63: Rc<UH0> = Rc::new(UH0::UH0_Cons(v58.clone(), v62.clone()));
                let v64: i32 = 0i32;
                printed_2(v63.clone(), v64)
            }
            US0::US0_OutcomeNone => {
                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile next ok claimed=none reason="); } LIT.with(|lit| lit.clone()) };
                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v42));
                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wip="); } LIT.with(|lit| lit.clone()) };
                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                let v79: Rc<str> = format_real_7(v41);
                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                let v81: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v80.clone(), v81.clone()));
                let v83: i32 = 3i32;
                printed_2(v82.clone(), v83)
            }
            US0::US0_OutcomeRefused => {
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile claim refused id="); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v1));
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v67, v68)), v42));
                let v71: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v72: Rc<UH0> = Rc::new(UH0::UH0_Cons(v70.clone(), v71.clone()));
                let v73: i32 = 5i32;
                printed_2(v72.clone(), v73)
            }
            US0::US0_OutcomeReleaseRefused => {
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile release refused id="); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v1));
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
                let v105: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v102, v103)), v42));
                let v106: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v107: Rc<UH0> = Rc::new(UH0::UH0_Cons(v105.clone(), v106.clone()));
                let v108: i32 = 5i32;
                printed_2(v107.clone(), v108)
            }
            US0::US0_OutcomeUnknown => {
                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no open task with id "); } LIT.with(|lit| lit.clone()) };
                let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v1));
                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                let v113: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v0));
                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v116: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v113, v114)), v111));
                let v117: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v116 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v117
            }
        }
    }
}
fn words_from_13(mut v0: i32, mut v1: i32) -> Rc<UH0> {
    let v2: bool = v1 <= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v5: i32 = v0.wrapping_add(1i32);
        let v6: Rc<UH0> = words_from_13(v5, v1);
        Rc::new(UH0::UH0_Cons(v4.clone(), v6.clone()))
    }
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method17(v3, v1.clone());
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
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method20(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8.clone() }));
        while method20(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method19(v16.clone(), v17.clone())
    }
}
fn globs_of_flag_16(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH0> = split_list_18(v1, v0);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method17(v2, v3)
}
fn plain_text_21(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    let v5: bool = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v3: bool = v0.contains(&*v2);
        let mut v4: bool = v3 == false;
        v4
    } else {
        false
    };
    let mut v9: bool = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v0.contains(&*v6);
        let mut v8: bool = v7 == false;
        v8
    } else {
        false
    };
    let v13: bool = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v0.contains(&*v10);
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    let v17: bool = if v13 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v0.contains(&*v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    };
    let v21: bool = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v0.contains(&*v18);
        let v20: bool = v19 == false;
        v20
    } else {
        false
    };
    let v25: bool = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v0.contains(&*v22);
        let v24: bool = v23 == false;
        v24
    } else {
        false
    };
    let v29: bool = if v25 {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v27: bool = v0.contains(&*v26);
        let v28: bool = v27 == false;
        v28
    } else {
        false
    };
    if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v31: bool = v0.contains(&*v30);
        let v32: bool = v31 == false;
        v32
    } else {
        false
    }
}
fn method23(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ttl_read_22(mut v0: Rc<str>) -> US2 {
    let v1: Option<i32> = (v0).trim().parse::<i32>().ok();
    let v2: Option<i32> = method23(v1.clone());
    let mut v3: Rc<dyn Fn((i32)) -> US2> = closure1();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let v6: US2 = US2::US2_None;
    let v7: US2 = v6.clone();
    let mut v8: US2 = v4.unwrap_or(v7);
    match &v8 {
        US2::US2_None => {
            US2::US2_None
        }
        US2::US2_Some(v9) => {
            let mut v9: i32 = *v9;
            let v10: bool = v9 > 0i32;
            if v10 {
                US2::US2_Some(v9)
            } else {
                US2::US2_None
            }
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v5: bool = if v1 {
                    plain_text_21(v2.clone())
                } else {
                    false
                };
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn flags_read_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<UH0>) -> US1 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v11, v12) => {
                let v11: Rc<str> = v11.clone();
                let v12: Rc<UH0> = v12.clone();
                let (mut v19, mut v20): (Rc<str>, Rc<UH0>) = match &*v12 {
                    UH0::UH0_Cons(v13, v14) => {
                        let v13: Rc<str> = v13.clone();
                        let v14: Rc<UH0> = v14.clone();
                        (v13.clone(), v14.clone())
                    }
                    UH0::UH0_Nil => {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        (v15.clone(), v16.clone())
                    }
                };
                let v21: Rc<UH0> = globs_of_flag_16(v19.clone());
                let v22: bool = &*v11 == "--any";
                if v22 {
                    v0 = v12.clone();
                    continue;
                } else {
                    let v24: bool = &*v11 == "--as";
                    let v26: bool = if v24 {
                        plain_text_21(v19.clone())
                    } else {
                        false
                    };
                    if v26 {
                        (v0, v3) = (v20.clone(), v19.clone());
                        continue;
                    } else {
                        let v28: bool = &*v11 == "--ttl";
                        if v28 {
                            let v29: US2 = ttl_read_22(v19.clone());
                            match &v29 {
                                US2::US2_None => {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ttl expects positive minutes, got '"); } LIT.with(|lit| lit.clone()) };
                                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v19));
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                                    return US1::US1_RequestRefused(v35.clone());
                                }
                                US2::US2_Some(v30) => {
                                    let v30: i32 = *v30;
                                    (v0, v4) = (v20.clone(), v30);
                                    continue;
                                }
                            }
                        } else {
                            let v39: bool = &*v11 == "--paths";
                            let v41: bool = if v39 {
                                let v40: bool = &*v1 == "claim";
                                v40
                            } else {
                                false
                            };
                            let v44: bool = if v41 {
                                let v42: bool = match &*v21 {
                                    UH0::UH0_Nil => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let v43: bool = v42 != true;
                                v43
                            } else {
                                false
                            };
                            let v47: bool = if v44 {
                                let v45: bool = true;
                                method24(v21.clone(), v45)
                            } else {
                                false
                            };
                            if v47 {
                                (v0, v5) = (v20.clone(), v21.clone());
                                continue;
                            } else {
                                if v39 {
                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--paths expects comma-separated globs without spaces, quotes or parentheses, got '"); } LIT.with(|lit| lit.clone()) };
                                    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v19));
                                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
                                    return US1::US1_RequestRefused(v52.clone());
                                } else {
                                    if v24 {
                                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as expects a lane name, got '"); } LIT.with(|lit| lit.clone()) };
                                        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v19));
                                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                                        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                                        return US1::US1_RequestRefused(v57.clone());
                                    } else {
                                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected argument "); } LIT.with(|lit| lit.clone()) };
                                        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v11));
                                        return US1::US1_RequestRefused(v60.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v6: bool = &*v3 == "";
                if v6 {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as <lane> is required"); } LIT.with(|lit| lit.clone()) };
                    return US1::US1_RequestRefused(v7.clone());
                } else {
                    return US1::US1_RequestRead(v1.clone(), v2.clone(), v3.clone(), v4, v5.clone());
                }
            }
        }
    }
}
fn closure2(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v1: Rc<UH0>| -> US2 {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let v5: bool = &*v3 == "--any";
                if v5 {
                    US2::US2_Some(v0)
                } else {
                    let v7: i32 = v0.wrapping_add(1i32);
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US2> = method25(v7);
                    v8(v4.clone())
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method25(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure2(v0)
}
fn request_of_14(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    let v2: bool = &*v0 == "next";
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: Rc<dyn Fn(Rc<UH0>) -> US2> = method25(v3);
        let v5: US2 = v4(v1.clone());
        let v6: bool = match &v5 {
            US2::US2_None => {
                true
            }
            _ => {
                false
            }
        };
        let v7: bool = v6 != true;
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = 90i32;
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            flags_read_15(v1.clone(), v0.clone(), v8.clone(), v8, v9, v10)
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next claims only with --any"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RequestRefused(v12)
        }
    } else {
        match &*v1 {
            UH0::UH0_Cons(v15, v16) => {
                let v15: Rc<str> = v15.clone();
                let v16: Rc<UH0> = v16.clone();
                let v17: bool = plain_text_21(v15.clone());
                let v21: bool = if v17 {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v19: bool = v15.starts_with(&*v18);
                    let v20: bool = v19 == false;
                    v20
                } else {
                    false
                };
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v23: i32 = 90i32;
                    let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    flags_read_15(v16, v0, v15.clone(), v22, v23, v24)
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID, got '"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v15));
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                    US1::US1_RequestRefused(v29)
                }
            }
            UH0::UH0_Nil => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected an item ID"); } LIT.with(|lit| lit.clone()) };
                US1::US1_RequestRefused(v32)
            }
        }
    }
}
fn claim_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: i32 = 0i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = 2i32;
    let v5: usize = std::env::args().count();
    let v6: i32 = ((v5) as i32);
    let v7: i32 = v6.wrapping_sub(2i32);
    let mut v8: Rc<UH0> = words_from_13(v4, v7);
    let mut v9: US1 = request_of_14(v3.clone(), v8.clone());
    match &v9 {
        US1::US1_RequestRead(v17, v18, v19, v20, v21) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = v19.clone();
            let v20: i32 = *v20;
            let v21: Rc<UH0> = v21.clone();
            let (mut v22, mut v23, mut v24, mut v25): (bool, Rc<str>, Rc<str>, Rc<str>) = agile_claim_rank::kit_claim_ranking(&*v1);
            let v26: bool = v22 == false;
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v3));
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                let v31: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v30 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v31
            } else {
                let (mut v32, mut v33): (bool, Rc<str>) = agile_claim_store::kit_claims_loaded(&*v1);
                claim_answered_1(v17.clone(), v18.clone(), v19.clone(), v20, v21.clone(), v1.clone(), v32, v33.clone(), v23.clone(), v24.clone(), v25.clone())
            }
        }
        US1::US1_RequestRefused(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v3));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v10));
            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
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
