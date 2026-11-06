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
enum US0 {
    US0_Planned,
    US0_Active,
    US0_Blocked,
    US0_Paused,
    US0_Done,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, US0, Rc<UH1>, Rc<UH1>, i64, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i64, i32, Rc<UH2>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn status_named_2(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = &*v0 == "Active";
    if v1 {
        US0::US0_Active
    } else {
        let v3: bool = &*v0 == "Blocked";
        if v3 {
            US0::US0_Blocked
        } else {
            let v5: bool = &*v0 == "Paused";
            if v5 {
                US0::US0_Paused
            } else {
                let v7: bool = &*v0 == "Done";
                if v7 {
                    US0::US0_Done
                } else {
                    US0::US0_Planned
                }
            }
        }
    }
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method4(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(v2.replace(&*v5, &*v6));
            Rc::new(UH1::UH1_Cons(v7, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
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
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
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
        method6(v16, v17)
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                    let v7: bool = v4 == b'\r';
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
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                    let v7: bool = v4 == b'\r';
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
fn method8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method8(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let v7: i32 = method9(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let mut v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method10(v9.clone(), v10);
            let mut v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH1::UH1_Cons(v12, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method11(v3, v1);
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
fn teams_of_3(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_5(v1, v0);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH1> = method8(v2, v3);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method11(v4, v5);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method4(v6, v7)
}
fn tracked_from_1(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, i64) = v0.borrow()[v1 as usize].clone();
            let v8: US0 = status_named_2(v5);
            let mut v9: Rc<UH1> = teams_of_3(v6);
            let mut v10: i32 = v1.wrapping_sub(1i32);
            let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v8, v9, v11, v7, v2));
            (v1, v2) = (v10, v12);
            continue;
        }
    }
}
fn method13(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method13(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i64, mut v2: Rc<UH2>, mut v3: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4;
                let v5: i64 = v5;
                let v6: i32 = v6;
                let v7: Rc<UH2> = v7;
                let v11: bool = if v3 {
                    true
                } else {
                    let v8: bool = &*v4 == &*v0;
                    if v8 {
                        let mut v9: bool = v5 == v1;
                        v9
                    } else {
                        false
                    }
                };
                (v2, v3) = (v7, v11);
                continue;
            }
            UH2::UH2_Nil => {
                return v3;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i64, mut v2: Rc<UH2>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH2::UH2_Cons(v4, v5, v6, v7) => {
            let v4: Rc<str> = v4;
            let v5: i64 = v5;
            let v6: i32 = v6;
            let v7: Rc<UH2> = v7;
            let v8: Rc<UH2> = method17(v0.clone(), v1, v7, v3);
            let mut v9: bool = &*v4 == &*v0;
            let v11: bool = if v9 {
                let mut v10: bool = v5 == v1;
                v10
            } else {
                false
            };
            let (mut v13, mut v14, mut v15): (Rc<str>, i64, i32) = if v11 {
                let mut v12: i32 = v6.wrapping_add(1i32);
                (v4, v5, v12)
            } else {
                (v4, v5, v6)
            };
            Rc::new(UH2::UH2_Cons(v13, v14, v15, v8))
        }
        UH2::UH2_Nil => {
            v3
        }
    }
}
fn method15(mut v0: i64, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: bool = false;
                let v6: bool = method16(v3.clone(), v0, v2.clone(), v5);
                let mut v10: Rc<UH2> = if v6 {
                    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                    method17(v3, v0, v2, v7)
                } else {
                    Rc::new(UH2::UH2_Cons(v3, v0, 1i32, v2))
                };
                (v1, v2) = (v4, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn method14(mut v0: i64, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7, v8) => {
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH1> = v6;
                let v7: i64 = v7;
                let v8: Rc<UH0> = v8;
                let mut v9: i64 = v7.wrapping_div(24i64);
                let mut v10: bool = match v4 {
                    US0::US0_Done => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v11: bool = v10 != true;
                let v13: bool = if v11 {
                    true
                } else {
                    let mut v12: bool = v9 < v0;
                    v12
                };
                let v15: Rc<UH2> = if v13 {
                    v2
                } else {
                    method15(v9, v5, v2)
                };
                (v1, v2) = (v8, v15);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
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
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US1> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US1> {
    closure1(v0, v1)
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: i64 = v4;
            let v5: i32 = v5;
            let v6: Rc<UH2> = v6;
            let v7: Rc<UH2> = method18(v0.clone(), v6, v2);
            let v8: bool = &*v0 == "";
            let v16: bool = if v8 {
                true
            } else {
                let mut v9: i32 = 0i32;
                let mut v10: Rc<dyn Fn(Rc<UH1>) -> US1> = method19(v0, v9);
                let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v3.clone(), v11));
                let v13: US1 = v10(v12);
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
            if v16 {
                Rc::new(UH2::UH2_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn method23(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_24(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn done_line_21(mut v0: i64, mut v1: Rc<str>, mut v2: i64, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile cadence done team="); } LIT.with(|lit| lit.clone()) }, v1));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" days_ago="); } LIT.with(|lit| lit.clone()) }));
    let v6: i64 = v0.wrapping_sub(v2);
    let v7: Rc<str> = format_real_22(v6);
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done="); } LIT.with(|lit| lit.clone()) }));
    let mut v10: Rc<str> = format_real_24(v3);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    v11
}
fn method20(mut v0: i64, mut v1: Rc<UH2>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH2::UH2_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: i64 = v4;
            let v5: i32 = v5;
            let v6: Rc<UH2> = v6;
            let v7: Rc<UH1> = method20(v0, v6, v2);
            let v8: Rc<str> = done_line_21(v0, v3, v4, v5);
            Rc::new(UH1::UH1_Cons(v8, v7))
        }
        UH2::UH2_Nil => {
            v2
        }
    }
}
fn method25(mut v0: i64, mut v1: i64, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5, v6, v7, v8, v9) => {
            let v4: Rc<str> = v4;
            let v5: US0 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH1> = v7;
            let v8: i64 = v8;
            let mut v9: Rc<UH0> = v9;
            let mut v10: Rc<UH0> = method25(v0, v1, v9, v3);
            let v11: bool = match &v5 {
                US0::US0_Active => {
                    true
                }
                _ => {
                    false
                }
            };
            let v14: bool = if v11 {
                let mut v12: i64 = v0.wrapping_sub(v8);
                let v13: bool = v12 > v1;
                v13
            } else {
                false
            };
            if v14 {
                Rc::new(UH0::UH0_Cons(v4, v5, v6, v7, v8, v10))
            } else {
                v10
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3;
            let v4: US0 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH1> = v6;
            let v7: i64 = v7;
            let v8: Rc<UH0> = v8;
            let mut v9: Rc<UH0> = method26(v0.clone(), v8, v2);
            let mut v10: bool = &*v0 == "";
            let v16: bool = if v10 {
                true
            } else {
                let v11: i32 = 0i32;
                let mut v12: Rc<dyn Fn(Rc<UH1>) -> US1> = method19(v0, v11);
                let v13: US1 = v12(v5.clone());
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
            if v16 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
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
fn silent_line_28(mut v0: i64, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<UH1>, mut v4: &Rc<UH1>, mut v5: i64) -> Rc<str> {
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile cadence silent id="); } LIT.with(|lit| lit.clone()) }, v1));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" teams="); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v9: i32 = 0i32;
    let mut v10: i32 = method29(v8.clone(), v3, v9);
    let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v14));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hours="); } LIT.with(|lit| lit.clone()) }));
    let v17: i64 = v0.wrapping_sub(v5);
    let v18: Rc<str> = format_real_22(v17);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v18));
    v19
}
fn method27(mut v0: i64, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3;
            let v4: US0 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH1> = v6;
            let v7: i64 = v7;
            let v8: Rc<UH0> = v8;
            let mut v9: Rc<UH1> = method27(v0, v8, v2);
            let mut v10: Rc<str> = silent_line_28(v0, v3, v4, v5, &v6, v7);
            Rc::new(UH1::UH1_Cons(v10, v9))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn done_lines_12(mut v0: Rc<UH0>, mut v1: i64, mut v2: i64, mut v3: i64, mut v4: Rc<str>) -> Rc<UH1> {
    let v5: i64 = v1.wrapping_div(24i64);
    let v6: i64 = v5.wrapping_sub(v3);
    let v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = method14(v6, v0.clone(), v7);
    let mut v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH2> = method18(v4.clone(), v8, v9);
    let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH1> = method20(v5, v10, v11);
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = method25(v1, v2, v0, v13);
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method26(v4, v14, v15);
    let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH1> = method27(v1, v16, v17);
    method13(v12, v18)
}
fn done_report_0(mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: i64, mut v4: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> = agile_quality_state::kit_agile_cadence_rows(&v4);
    let v6: i32 = (v5.clone().borrow().len() as i32);
    let v7: i32 = v6.wrapping_sub(1i32);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = tracked_from_1(&v5, v7, v8);
    let mut v10: Rc<UH1> = done_lines_12(v9, v3, v2, v1, v0);
    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v12: i32 = 0i32;
    let v13: i32 = method29(v11.clone(), v10, v12);
    let v14: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i64, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i64, i64, i64, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: i64, mut v2: i64, mut v3: i64, mut v4: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        done_report_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_cadence_done_report(v0: &str, v1: i64, v2: i64, v3: i64, v4: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), v1, v2, v3, Rc::<str>::from(v4))
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
