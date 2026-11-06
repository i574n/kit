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
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH1>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i64, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, i64, i32, Rc<UH1>, Rc<UH3>),
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<UH1>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method3(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
            Rc::new(UH1::UH1_Cons(v7, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method6(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method5(v16, v17)
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                        let v8: bool = v4 == b'\n';
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
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method7(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let mut v7: i32 = method8(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method9(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH1::UH1_Cons(v12, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method10(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
            let v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn team_words_2(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_4(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = method7(v2, v3);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method10(v4, v5);
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method3(v6, v7)
}
fn teams_from_1(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, i64) = v0.borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_sub(1i32);
            let mut v9: Rc<UH1> = team_words_2(v6);
            let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v9, v2));
            (v1, v2) = (v8, v10);
            continue;
        }
    }
}
fn events_from_12(mut v0: Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5, mut v6): (Rc<str>, i64, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let mut v7: i32 = v1.wrapping_sub(1i32);
            let v8: Rc<UH2> = Rc::new(UH2::UH2_Cons(v4, v5, v6, v2));
            (v1, v2) = (v7, v8);
            continue;
        }
    }
}
fn events_of_11(mut v0: Rc<str>) -> Rc<UH2> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, i64, Rc<str>)>>> = agile_note_events::kit_agile_note_events(&v0);
    let mut v2: i32 = (v1.clone().borrow().len() as i32);
    let v3: i32 = v2.wrapping_sub(1i32);
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    events_from_12(v1, v3, v4)
}
fn method16(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method18(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_17(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_19(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn activity_line_15(mut v0: i64, mut v1: Rc<str>, mut v2: i64, mut v3: i32, mut v4: Rc<UH1>) -> Rc<str> {
    let v5: i32 = 0i32;
    let v6: i32 = method16(v4, v5);
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile cadence activity team="); } LIT.with(|lit| lit.clone()) }, v1));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" days_ago="); } LIT.with(|lit| lit.clone()) }));
    let mut v9: i64 = v0.wrapping_sub(v2);
    let v10: Rc<str> = format_real_17(v9);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v10));
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes="); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = format_real_19(v3);
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" touched="); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = format_real_19(v6);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    v17
}
fn method14(mut v0: i64, mut v1: Rc<UH3>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH3::UH3_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: i64 = v4;
            let v5: i32 = v5;
            let v6: Rc<UH1> = v6;
            let mut v7: Rc<UH3> = v7;
            let v8: Rc<UH1> = method14(v0, v7, v2);
            let mut v9: Rc<str> = activity_line_15(v0, v3, v4, v5, v6);
            Rc::new(UH1::UH1_Cons(v9, v8))
        }
        UH3::UH3_Nil => {
            v2
        }
    }
}
fn method21(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i64 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH3> = v6;
                let mut v7: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v3 == &*v0;
                if v6 {
                    return US0::US0_Some(v3, v4);
                } else {
                    v1 = v5;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US1> {
    Rc::new(move |mut v2: Rc<UH1>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US1> = method27(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method27(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US1> {
    closure1(v0, v1)
}
fn method26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<UH3>, mut v4: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v3) {
        UH3::UH3_Cons(v5, v6, v7, v8, v9) => {
            let v5: Rc<str> = v5;
            let v6: i64 = v6;
            let mut v7: i32 = v7;
            let v8: Rc<UH1> = v8;
            let mut v9: Rc<UH3> = v9;
            let v10: Rc<UH3> = method26(v0.clone(), v1.clone(), v2, v9, v4);
            let v11: i32 = 0i32;
            let mut v12: Rc<dyn Fn(Rc<UH1>) -> US1> = method27(v0.clone(), v11);
            let v13: US1 = v12(v8.clone());
            let v14: bool = match v13 {
                US1::US1_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v15: bool = v14 != true;
            let v17: Rc<UH1> = if v15 {
                v8.clone()
            } else {
                Rc::new(UH1::UH1_Cons(v0, v8.clone()))
            };
            let v18: bool = &*v5 == &*v1;
            let v20: bool = if v18 {
                let v19: bool = v6 == v2;
                v19
            } else {
                false
            };
            let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, i64, i32, Rc<UH1>) = if v20 {
                let v21: i32 = v7.wrapping_add(1i32);
                (v5, v6, v21, v17)
            } else {
                (v5, v6, v7, v8)
            };
            Rc::new(UH3::UH3_Cons(v22, v23, v24, v25, v10))
        }
        UH3::UH3_Nil => {
            v4
        }
    }
}
fn method28(mut v0: Rc<str>, mut v1: i64, mut v2: Rc<UH3>, mut v3: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v2) {
        UH3::UH3_Cons(v4, v5, v6, v7, v8) => {
            let v4: Rc<str> = v4;
            let v5: i64 = v5;
            let v6: i32 = v6;
            let mut v7: Rc<UH1> = v7;
            let v8: Rc<UH3> = v8;
            let mut v9: Rc<UH3> = method28(v0.clone(), v1, v8, v3);
            let v10: bool = &*v4 == &*v0;
            let mut v12: bool = if v10 {
                let v11: bool = v5 == v1;
                v11
            } else {
                false
            };
            if v12 {
                Rc::new(UH3::UH3_Cons(v4, v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH3::UH3_Nil => {
            v3
        }
    }
}
fn method29(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: i64 = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH3> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn activity_added_25(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<str>) -> Rc<UH3> {
    let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH3> = method28(v1.clone(), v2, v0.clone(), v4);
    let v6: i32 = 0i32;
    let mut v7: i32 = method29(v5, v6);
    let v8: bool = v7 == 0i32;
    if v8 {
        let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v3, v9));
        Rc::new(UH3::UH3_Cons(v1, v2, 1i32, v10, v0))
    } else {
        let mut v12: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method26(v3, v1, v2, v0, v12)
    }
}
fn method24(mut v0: Rc<str>, mut v1: i64, mut v2: Rc<str>, mut v3: i64, mut v4: Rc<UH1>, mut v5: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v4) {
            UH1::UH1_Cons(v6, v7) => {
                let v6: Rc<str> = v6;
                let mut v7: Rc<UH1> = v7;
                let v8: Rc<UH3> = activity_added_25(v5, v6, v3, v0.clone());
                (v4, v5) = (v7, v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v5;
            }
        }
    }
}
fn method22(mut v0: i64, mut v1: Rc<UH0>, mut v2: Rc<UH2>, mut v3: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4;
                let v5: i64 = v5;
                let v6: Rc<str> = v6;
                let mut v7: Rc<UH2> = v7;
                let v8: i64 = v5.wrapping_div(24i64);
                let mut v9: bool = v8 < v0;
                let v17: Rc<UH3> = if v9 {
                    v3
                } else {
                    let v10: US0 = method23(v4.clone(), v1.clone());
                    let v15: Rc<UH1> = match v10 {
                        US0::US0_None => {
                            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                        }
                        US0::US0_Some(v11, v12) => {
                            let v11: Rc<str> = v11;
                            let mut v12: Rc<UH1> = v12;
                            v12
                        }
                    };
                    method24(v4, v5, v6, v8, v15, v3)
                };
                (v2, v3) = (v7, v17);
                continue;
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn activity_of_20(mut v0: Rc<UH0>, mut v1: Rc<UH2>, mut v2: i64) -> Rc<UH3> {
    let v3: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH3> = method22(v2, v0, v1, v3);
    let v5: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method21(v4, v5)
}
fn method30(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v1) {
        UH3::UH3_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: i64 = v4;
            let v5: i32 = v5;
            let v6: Rc<UH1> = v6;
            let mut v7: Rc<UH3> = v7;
            let v8: Rc<UH3> = method30(v0.clone(), v7, v2);
            let mut v9: bool = &*v0 == "";
            let v11: bool = if v9 {
                true
            } else {
                let v10: bool = &*v3 == &*v0;
                v10
            };
            if v11 {
                Rc::new(UH3::UH3_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH3::UH3_Nil => {
            v2
        }
    }
}
fn activity_lines_13(mut v0: Rc<UH0>, mut v1: Rc<UH2>, mut v2: i64, mut v3: i64, mut v4: Rc<str>) -> Rc<UH1> {
    let v5: i64 = v2.wrapping_sub(v3);
    let v6: Rc<UH3> = activity_of_20(v0, v1, v5);
    let mut v7: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH3> = method30(v4, v6, v7);
    let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method14(v2, v8, v9)
}
fn method31(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn activity_report_0(mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v4: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> = agile_quality_state::kit_agile_cadence_rows(&v3);
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: i32 = v5.wrapping_sub(1i32);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = teams_from_1(&v4, v6, v7);
    let mut v9: Rc<UH2> = events_of_11(v3);
    let v10: Rc<UH1> = activity_lines_13(v8, v9, v2, v1, v0);
    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v12: i32 = 0i32;
    let v13: i32 = method31(v11.clone(), v10, v12);
    let v14: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        activity_report_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method34(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method34(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method35(mut v0: i64, mut v1: i64, mut v2: Rc<UH2>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4;
                let v5: i64 = v5;
                let v6: Rc<str> = v6;
                let mut v7: Rc<UH2> = v7;
                let v8: bool = v5 < v1;
                let v10: bool = if v8 {
                    true
                } else {
                    let mut v9: bool = v5 >= v0;
                    v9
                };
                let v16: bool = if v10 {
                    true
                } else {
                    let v11: i32 = 0i32;
                    let mut v12: Rc<dyn Fn(Rc<UH1>) -> US1> = method27(v6.clone(), v11);
                    let v13: US1 = v12(v3.clone());
                    let v14: bool = match v13 {
                        US1::US1_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v15: bool = v14 != true;
                    v15
                };
                let v18: Rc<UH1> = if v16 {
                    v3
                } else {
                    Rc::new(UH1::UH1_Cons(v6, v3))
                };
                (v2, v3) = (v7, v18);
                continue;
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method37(mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: Rc<UH2>, mut v4: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v3) {
        UH2::UH2_Cons(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5;
            let v6: i64 = v6;
            let mut v7: Rc<str> = v7;
            let v8: Rc<UH2> = v8;
            let mut v9: Rc<UH2> = method37(v0.clone(), v1, v2, v8, v4);
            let v10: bool = &*v7 == &*v0;
            let mut v12: bool = if v10 {
                let v11: bool = v6 >= v1;
                v11
            } else {
                false
            };
            let v14: bool = if v12 {
                let v13: bool = v6 < v2;
                v13
            } else {
                false
            };
            if v14 {
                Rc::new(UH2::UH2_Cons(v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH2::UH2_Nil => {
            v4
        }
    }
}
fn method38(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i64 = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH2> = v5;
                let v6: i32 = 0i32;
                let mut v7: Rc<dyn Fn(Rc<UH1>) -> US1> = method27(v2.clone(), v6);
                let v8: US1 = v7(v1.clone());
                let mut v9: bool = match v8 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v10: bool = v9 != true;
                let mut v12: Rc<UH1> = if v10 {
                    v1
                } else {
                    Rc::new(UH1::UH1_Cons(v2, v1))
                };
                (v0, v1) = (v5, v12);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method39(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i64 = v3;
                let v4: Rc<str> = v4;
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
fn method36(mut v0: Rc<UH2>, mut v1: i64, mut v2: i64, mut v3: Rc<UH1>, mut v4: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v3) {
        UH1::UH1_Cons(v5, v6) => {
            let v5: Rc<str> = v5;
            let v6: Rc<UH1> = v6;
            let mut v7: Rc<UH1> = method36(v0.clone(), v1, v2, v6, v4);
            let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH2> = method37(v5.clone(), v2, v1, v0, v8);
            let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH1> = method38(v9.clone(), v10);
            let mut v12: i32 = 0i32;
            let v13: i32 = method39(v9, v12);
            let v14: i32 = 0i32;
            let v15: i32 = method16(v11, v14);
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile cadence round author="); } LIT.with(|lit| lit.clone()) }, v5));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes="); } LIT.with(|lit| lit.clone()) }));
            let v18: Rc<str> = format_real_19(v13);
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) }));
            let v21: Rc<str> = format_real_19(v15);
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
            Rc::new(UH1::UH1_Cons(v22, v7))
        }
        UH1::UH1_Nil => {
            v4
        }
    }
}
fn round_lines_33(mut v0: Rc<UH2>, mut v1: i64, mut v2: i64) -> Rc<UH1> {
    let v3: i64 = v1.wrapping_sub(v2);
    let v4: i64 = v1.wrapping_add(1i64);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method35(v4, v3, v0.clone(), v5);
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = method5(v6, v7);
    let mut v9: i32 = 0i32;
    let v10: i32 = method16(v8.clone(), v9);
    let v11: Rc<str> = format_real_19(v10);
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile cadence round ok authors="); } LIT.with(|lit| lit.clone()) }, v11));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window_hours="); } LIT.with(|lit| lit.clone()) }));
    let v14: Rc<str> = format_real_17(v2);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH1> = method36(v0, v4, v3, v8, v16);
    let v18: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v15, v18));
    method34(v17, v19)
}
fn round_report_32(mut v0: i64, mut v1: i64, mut v2: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v3: Rc<UH2> = events_of_11(v2);
    let v4: Rc<UH1> = round_lines_33(v3, v1, v0);
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v6: i32 = 0i32;
    let mut v7: i32 = method31(v5.clone(), v4, v6);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    v9
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i64, i64) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i64, i64) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: i64, mut v2: i64| -> Rc<RefCell<Vec<Rc<str>>>> {
        round_report_32(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_cadence_activity_report(v0: &str, v1: i64, v2: i64, v3: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3))
}
pub fn kit_cadence_round_report(v0: &str, v1: i64, v2: i64) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure2()(Rc::<str>::from(v0), v1, v2)
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
