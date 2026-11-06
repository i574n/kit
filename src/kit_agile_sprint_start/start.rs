#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(i64),
    US0_None,
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_ItemOpen,
    US1_ItemDone,
    US1_ItemUnknown,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US1, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_SprintOpen,
    US3_SprintClosed,
}
#[derive(Clone)]
enum US4 {
    US4_Committed,
    US4_Dropped,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, US4, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Started(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US3, Rc<UH2>),
    US2_AnotherSprintOpen(Rc<str>),
    US2_ItemNotOpen(Rc<str>),
    US2_ItemUnestimated(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(US2),
    US5_None,
}
fn method2(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US0> = Rc::new(move |mut v0: (i64)| -> US0 {
        let mut v1: i64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn two_digits_6(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        let mut v2: Rc<str> = format_real_4(v0);
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v2));
        v3
    } else {
        format_real_4(v0)
    }
}
fn civil_date_3(mut v0: i64) -> Rc<str> {
    let mut v1: i64 = v0.wrapping_add(719468i64);
    let mut v2: bool = v1 >= 0i64;
    let v4: i64 = if v2 {
        v1
    } else {
        let v3: i64 = v1.wrapping_sub(146096i64);
        v3
    };
    let v5: i64 = v4.wrapping_div(146097i64);
    let v6: i64 = v5.wrapping_mul(146097i64);
    let mut v7: i64 = v1.wrapping_sub(v6);
    let mut v8: i64 = v7.wrapping_div(1460i64);
    let mut v9: i64 = v7.wrapping_sub(v8);
    let mut v10: i64 = v7.wrapping_div(36524i64);
    let mut v11: i64 = v9.wrapping_add(v10);
    let v12: i64 = v7.wrapping_div(146096i64);
    let v13: i64 = v11.wrapping_sub(v12);
    let v14: i64 = v13.wrapping_div(365i64);
    let v15: i64 = 365i64.wrapping_mul(v14);
    let v16: i64 = v14.wrapping_div(4i64);
    let v17: i64 = v15.wrapping_add(v16);
    let v18: i64 = v14.wrapping_div(100i64);
    let v19: i64 = v17.wrapping_sub(v18);
    let v20: i64 = v7.wrapping_sub(v19);
    let v21: i64 = 5i64.wrapping_mul(v20);
    let v22: i64 = v21.wrapping_add(2i64);
    let v23: i64 = v22.wrapping_div(153i64);
    let v24: i64 = 153i64.wrapping_mul(v23);
    let v25: i64 = v24.wrapping_add(2i64);
    let v26: i64 = v25.wrapping_div(5i64);
    let v27: i64 = v20.wrapping_sub(v26);
    let v28: i64 = v27.wrapping_add(1i64);
    let v29: bool = v23 < 10i64;
    let v32: i64 = if v29 {
        let v30: i64 = v23.wrapping_add(3i64);
        v30
    } else {
        let v31: i64 = v23.wrapping_sub(9i64);
        v31
    };
    let v33: i64 = v5.wrapping_mul(400i64);
    let v34: i64 = v14.wrapping_add(v33);
    let v35: bool = v32 <= 2i64;
    let v36: i64 = if v35 {
        1i64
    } else {
        0i64
    };
    let v37: i64 = v34.wrapping_add(v36);
    let v38: Rc<str> = format_real_4(v37);
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let v40: Rc<str> = two_digits_6(v32);
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let v43: Rc<str> = two_digits_6(v28);
    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
    v44
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method8(v3, v1);
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
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
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
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method10(v16, v17)
    }
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method12(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let mut v7: i32 = method13(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let mut v11: i32 = method14(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn items_given_7(mut v0: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                match &*v2 {
                    UH0::UH0_Cons(v3, v4) => {
                        let v3: Rc<str> = v3.clone();
                        let v4: Rc<UH0> = v4.clone();
                        let v5: bool = &*v1 == "--items";
                        if v5 {
                            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let mut v7: Rc<UH0> = split_list_9(v6, v3);
                            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v9: Rc<UH0> = method12(v7, v8);
                            let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            return method8(v9, v10);
                        } else {
                            v0 = v2;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v2;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn standing_of_17(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        US1::US1_ItemUnknown
    } else {
        let v3: bool = &*v0 == "Done";
        if v3 {
            US1::US1_ItemDone
        } else {
            US1::US1_ItemOpen
        }
    }
}
fn candidate_of_16(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i32, US1) {
    let mut v2: Rc<str> = agile_task_census::kit_agile_task_status_of(&v0, &v1);
    let v3: i32 = agile_planning_fields::kit_agile_planning_minutes_of(&v0, &v1);
    let v4: US1 = standing_of_17(v2);
    (v1, v3, v4)
}
fn method15(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method15(v0.clone(), v4, v2);
            let (mut v6, mut v7, mut v8): (Rc<str>, i32, US1) = candidate_of_16(v0, v3);
            Rc::new(UH1::UH1_Cons(v6, v7, v8, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method19(mut v0: Rc<UH1>, mut v1: US5) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US1 = v4;
                let v5: Rc<UH1> = v5;
                let v18: US5 = match &v1 {
                    US5::US5_None => {
                        let mut v7: bool = match v4 {
                            US1::US1_ItemOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v8: bool = v7 != true;
                        if v8 {
                            let mut v9: US2 = US2::US2_ItemNotOpen(v2);
                            US5::US5_Some(v9)
                        } else {
                            let mut v11: bool = v3 <= 0i32;
                            if v11 {
                                let v12: US2 = US2::US2_ItemUnestimated(v2);
                                US5::US5_Some(v12)
                            } else {
                                US5::US5_None
                            }
                        }
                    }
                    US5::US5_Some(v6) => {
                        let v6: US2 = v6.clone();
                        v1
                    }
                };
                (v0, v1) = (v5, v18);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn first_refusal_18(mut v0: Rc<UH1>) -> US5 {
    let mut v1: US5 = US5::US5_None;
    method19(v0, v1)
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: US1 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH2> = method20(v5, v1);
            let mut v7: US4 = US4::US4_Committed;
            Rc::new(UH2::UH2_Cons(v2, v3, v7, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method22(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method21(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US1 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![(v2, v3)]));
                let mut v7: i32 = (v1.clone().borrow().len() as i32);
                let mut v8: i32 = (v6.clone().borrow().len() as i32);
                let mut v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
                while method22(v9, &v11) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, i32) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, i32) = v1.borrow()[v13 as usize].clone();
                        (v15, v16)
                    } else {
                        let v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, i32) = v6.borrow()[v17 as usize].clone();
                        (v18, v19)
                    };
                    v10.borrow_mut()[v13 as usize] = (v20, v21);
                    let v22: i32 = v13.wrapping_add(1i32);
                    v11.borrow_mut().l0 = v22;
                    ()
                };
                (v0, v1) = (v5, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn modules_with_sprints_23(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v0.clone()));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    sprints\n"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    if v3 {
        v0
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0.ends_with(&*v4);
        if v5 {
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    sprints\n"); } LIT.with(|lit| lit.clone()) }));
            v6
        } else {
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    sprints\n"); } LIT.with(|lit| lit.clone()) }));
            v7
        }
    }
}
fn method25(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US4 = v4;
                let v5: Rc<UH2> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_26(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn outcome_text_24(mut v0: US2) -> (i32, Rc<str>) {
    match v0 {
        US2::US2_AnotherSprintOpen(v15) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint start refused: sprint "); } LIT.with(|lit| lit.clone()) }, v15));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is still open"); } LIT.with(|lit| lit.clone()) }));
            (1i32, v17)
        }
        US2::US2_ItemNotOpen(v18) => {
            let v18: Rc<str> = v18;
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint start refused: item "); } LIT.with(|lit| lit.clone()) }, v18));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is unknown or Done"); } LIT.with(|lit| lit.clone()) }));
            (1i32, v20)
        }
        US2::US2_ItemUnestimated(v21) => {
            let v21: Rc<str> = v21;
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint start refused: item "); } LIT.with(|lit| lit.clone()) }, v21));
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no estimate"); } LIT.with(|lit| lit.clone()) }));
            (1i32, v23)
        }
        US2::US2_Started(v1, v2, v3, v4, v5, v6) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: US3 = v5;
            let v6: Rc<UH2> = v6;
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method25(v6, v7);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile sprint start ok id="); } LIT.with(|lit| lit.clone()) }, v1));
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) }));
            let mut v11: Rc<str> = format_real_26(v8);
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" finish="); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v4));
            (0i32, v14)
        }
    }
}
fn sprint_started_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let v5: Option<i64> = (v2).trim().parse::<i64>().ok();
    let v6: Option<i64> = method2(v5);
    let mut v7: Rc<dyn Fn((i64)) -> US0> = closure1();
    let mut v8: Option<US0> = v6.map(|x| v7(x));
    let mut v10: US0 = US0::US0_None;
    let mut v11: US0 = v10.clone();
    let v12: US0 = v8.unwrap_or(v11);
    let v15: i64 = match v12 {
        US0::US0_None => {
            0i64
        }
        US0::US0_Some(v13) => {
            let v13: i64 = v13;
            v13
        }
    };
    let v16: bool = v15 <= 0i64;
    let v18: bool = if v16 {
        true
    } else {
        let v17: bool = &*v1 == "";
        v17
    };
    if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile usage: sprint start <root> <id> <days> <goal> [--items A,B]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v19);
        2i32
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(v0.replace(&*v20, &*v21));
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
        let v25: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v24).unwrap_or_default().as_str());
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/sprints.spi"); } LIT.with(|lit| lit.clone()) }));
        let v27: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v26).unwrap_or_default().as_str());
        let v28: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v29: i64 = (v28 as i64);
        let v30: i64 = v29.wrapping_div(86400000i64);
        let v31: Rc<str> = civil_date_3(v30);
        let v32: i64 = v30.wrapping_add(v15);
        let v33: Rc<str> = civil_date_3(v32);
        let v34: Rc<UH0> = items_given_7(v4);
        let v35: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v36: Rc<UH1> = method15(v25, v34, v35);
        let v37: Rc<str> = kit_agile_sprint_state::kit_agile_open_sprint(&v27);
        let v38: bool = &*v37 != "";
        let v48: US2 = if v38 {
            US2::US2_AnotherSprintOpen(v37)
        } else {
            let v40: US5 = first_refusal_18(v36.clone());
            match v40 {
                US5::US5_None => {
                    let v42: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    let v43: Rc<UH2> = method20(v36.clone(), v42);
                    let v44: US3 = US3::US3_SprintOpen;
                    US2::US2_Started(v1.clone(), v3.clone(), v31.clone(), v33.clone(), v44, v43)
                }
                US5::US5_Some(v41) => {
                    let v41: US2 = v41;
                    v41
                }
            }
        };
        match &v48 {
            US2::US2_Started(v49, v50, v51, v52, v53, v54) => {
                let v49: Rc<str> = v49.clone();
                let v50: Rc<str> = v50.clone();
                let v51: Rc<str> = v51.clone();
                let v52: Rc<str> = v52.clone();
                let v53: US3 = v53.clone();
                let v54: Rc<UH2> = v54.clone();
                let v55: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![]));
                let v56: Rc<RefCell<Vec<(Rc<str>, i32)>>> = method21(v36, v55);
                let v57: Rc<str> = kit_agile_sprint_append::kit_agile_sprint_appended(&v27, &v1, &v3, &v31, &v33, v56);
                std::fs::write(&*v26, &*v57).unwrap();
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
                let v60: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v59).unwrap_or_default().as_str());
                let v61: Rc<str> = modules_with_sprints_23(v60.clone());
                let v62: bool = &*v61 != &*v60;
                if v62 {
                    std::fs::write(&*v59, &*v61).unwrap();
                    ()
                }
            }
            _ => {
                ()
            }
        };
        let (mut v64, mut v65): (i32, Rc<str>) = outcome_text_24(v48);
        let v66: bool = v64 == 0i32;
        if v66 {
            println!("{}", v65);
            ()
        } else {
            eprintln!("{}", v65);
            ()
        };
        v64
    }
}
fn method27(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn started_0(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> i32 {
    let v5: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let v6: usize = (v5).len();
    let mut v7: i32 = ((v6) as i32);
    let mut v8: i32 = v7.wrapping_sub(1i32);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH0> = method27(v5, v8, v9);
    sprint_started_1(v4, v3, v2, v1, v10)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<RefCell<Vec<Rc<str>>>>| -> i32 {
        started_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_started(v0: &str, v1: &str, v2: &str, v3: &str, v4: Rc<RefCell<Vec<Rc<str>>>>) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), v4)
}
use agile_task_census::SpiralShape_97df63ebabc3 as Mut0;
use kit_agile_sprint_state::SpiralShape_2617b4e49837 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut1;
