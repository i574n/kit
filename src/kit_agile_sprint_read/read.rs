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
enum US1 {
    US1_SprintOpen,
    US1_SprintClosed,
}
#[derive(Clone)]
enum US2 {
    US2_Committed,
    US2_Dropped,
}
#[derive(Clone)]
enum US0 {
    US0_SprintLine(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1),
    US0_ItemLine(Rc<str>, Rc<str>, i32, US2),
    US0_NoSprintLine,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US0, Rc<UH0>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, US2, Rc<UH2>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>, Rc<UH1>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH3> }
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool, Rc<UH4>),
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(i32, i32, i32, bool, Rc<UH5>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32, i32, i32, bool),
    US4_None,
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(i32, i32, i32, Rc<UH6>),
}
fn closure1(mut v0: Rc<str>, mut v1: i32, mut v2: US2) -> Rc<dyn Fn(Rc<str>, i32, US2) -> bool> {
    Rc::new(move |mut v3: Rc<str>, mut v4: i32, mut v5: US2| -> bool {
        let v6: bool = &*v3 == &*v0;
        v6
    })
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: US2) -> Rc<dyn Fn(Rc<str>, i32, US2) -> bool> {
    closure1(v0, v1, v2)
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: US2, mut v3: Rc<dyn Fn(Rc<str>, i32, US2) -> bool>, mut v4: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v4) {
            UH2::UH2_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let v7: US2 = v7;
                let v8: Rc<UH2> = v8;
                let mut v9: bool = v3(v5, v6, v7);
                if v9 {
                    return true;
                } else {
                    let v10: Rc<dyn Fn(Rc<str>, i32, US2) -> bool> = method4(v0.clone(), v1, v2.clone());
                    (v3, v4) = (v10, v8);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: US2, mut v3: Rc<UH2>, mut v4: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v3) {
        UH2::UH2_Cons(v5, v6, v7, v8) => {
            let v5: Rc<str> = v5;
            let v6: i32 = v6;
            let v7: US2 = v7;
            let v8: Rc<UH2> = v8;
            let mut v9: Rc<UH2> = method6(v0.clone(), v1, v2.clone(), v8, v4);
            let v10: bool = &*v5 == &*v0;
            let (mut v11, mut v12, mut v13): (Rc<str>, i32, US2) = if v10 {
                (v0, v1, v2)
            } else {
                (v5, v6, v7)
            };
            Rc::new(UH2::UH2_Cons(v11, v12, v13, v9))
        }
        UH2::UH2_Nil => {
            v4
        }
    }
}
fn method3(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: US0 = v3;
                let mut v4: Rc<UH0> = v4;
                let v17: Rc<UH2> = match v3 {
                    US0::US0_ItemLine(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: i32 = v7;
                        let v8: US2 = v8;
                        let mut v9: bool = &*v5 == &*v0;
                        if v9 {
                            let v10: Rc<dyn Fn(Rc<str>, i32, US2) -> bool> = method4(v6.clone(), v7, v8.clone());
                            let v11: bool = method5(v6.clone(), v7, v8.clone(), v10, v2.clone());
                            if v11 {
                                let v12: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                                method6(v6, v7, v8, v2, v12)
                            } else {
                                Rc::new(UH2::UH2_Cons(v6, v7, v8, v2))
                            }
                        } else {
                            v2
                        }
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v4, v17);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method7(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let mut v3: i32 = v3;
                let mut v4: US2 = v4;
                let v5: Rc<UH2> = v5;
                let v6: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: US1 = v7;
            let v8: Rc<UH2> = v8;
            let mut v9: Rc<UH1> = v9;
            let v10: Rc<UH1> = method2(v0.clone(), v9, v2);
            let v11: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v12: Rc<UH2> = method3(v3.clone(), v0, v11);
            let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH2> = method7(v12, v13);
            Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v7, v14, v10))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method9(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH3> = v3;
                let mut v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH3> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method10(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH3> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH3> = v9.borrow().l1.clone();
        let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method9(v16, v17)
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
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
fn method16(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US3> = Rc::new(move |mut v0: (i32)| -> US3 {
        let mut v1: i32 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn minutes_of_13(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH3> = split_list_8(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH3::UH3_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH3> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH3::UH3_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH3> = v6;
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = Rc::<str>::from(v5.replace(&*v7, &*v8));
                    let v10: i32 = (v9.clone().len() as i32);
                    let v11: i32 = 0i32;
                    let v12: i32 = method14(v9.clone(), v10, v11);
                    let v13: i32 = v10.wrapping_sub(1i32);
                    let v14: Rc<str> = string_slice(&v9, v12 as i64, v13 as i64);
                    let v15: i32 = (v14.clone().len() as i32);
                    let v16: i32 = method15(v14.clone(), v15);
                    let v17: Rc<str> = string_slice(&v14, 0i32 as i64, v16 as i64);
                    let v18: Option<i32> = (v17).trim().parse::<i32>().ok();
                    let v19: Option<i32> = method16(v18);
                    let mut v20: Rc<dyn Fn((i32)) -> US3> = closure2();
                    let mut v21: Option<US3> = v19.map(|x| v20(x));
                    let v23: US3 = US3::US3_None;
                    let v24: US3 = v23.clone();
                    let v25: US3 = v21.unwrap_or(v24);
                    match v25 {
                        US3::US3_None => {
                            0i32
                        }
                        US3::US3_Some(v26) => {
                            let v26: i32 = v26;
                            v26
                        }
                    }
                }
                _ => {
                    0i32
                }
            }
        }
        _ => {
            0i32
        }
    }
}
fn item_line_12(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Dropped)"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.ends_with(&*v2);
    let v6: US2 = if v3 {
        US2::US2_Dropped
    } else {
        US2::US2_Committed
    };
    let v7: Rc<str> = v1.borrow()[1i32 as usize].clone();
    let v8: Rc<str> = v1.borrow()[3i32 as usize].clone();
    let mut v9: Rc<str> = v1.borrow()[4i32 as usize].clone();
    let v10: i32 = minutes_of_13(v9);
    US0::US0_ItemLine(v7, v8, v10, v6)
}
fn head_line_17(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let mut v1: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let mut v2: Rc<str> = v0.borrow()[3i32 as usize].clone();
    let mut v3: Rc<str> = v0.borrow()[5i32 as usize].clone();
    let mut v4: Rc<str> = v0.borrow()[7i32 as usize].clone();
    let v5: Rc<str> = v0.borrow()[8i32 as usize].clone();
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SprintClosed"); } LIT.with(|lit| lit.clone()) };
    let v7: bool = v5.contains(&*v6);
    let v10: US1 = if v7 {
        US1::US1_SprintClosed
    } else {
        US1::US1_SprintOpen
    };
    US0::US0_SprintLine(v1, v2, v3, v4, v10)
}
fn method11(mut v0: Rc<UH3>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH3> = v3;
            let mut v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v7: i32 = (v6.clone().borrow().len() as i32);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl sprint_item_"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v2.starts_with(&*v8);
            let v11: bool = if v9 {
                let v10: bool = v7 >= 5i32;
                v10
            } else {
                false
            };
            let mut v20: US0 = if v11 {
                item_line_12(v2, v6)
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl sprint_"); } LIT.with(|lit| lit.clone()) };
                let v14: bool = v2.starts_with(&*v13);
                let v16: bool = if v14 {
                    let v15: bool = v7 >= 9i32;
                    v15
                } else {
                    false
                };
                if v16 {
                    head_line_17(v6)
                } else {
                    US0::US0_NoSprintLine
                }
            };
            Rc::new(UH0::UH0_Cons(v20, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn closure3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) -> bool> {
    Rc::new(move |mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: US1, mut v9: Rc<UH2>| -> bool {
        let v10: bool = &*v4 == &*v0;
        v10
    })
}
fn method19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) -> bool> {
    closure3(v0, v1, v2, v3)
}
fn method20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) -> bool>, mut v5: Rc<UH1>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v5) {
            UH1::UH1_Cons(v6, v7, v8, v9, v10, v11, v12) => {
                let v6: Rc<str> = v6;
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: US1 = v10;
                let v11: Rc<UH2> = v11;
                let v12: Rc<UH1> = v12;
                let v13: bool = v4(v6, v7, v8, v9, v10, v11);
                if v13 {
                    return true;
                } else {
                    let v14: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) -> bool> = method19(v0.clone(), v1.clone(), v2.clone(), v3.clone());
                    (v4, v5) = (v14, v12);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: US1, mut v9: Rc<UH1>, mut v10: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v9) {
        UH1::UH1_Cons(v11, v12, v13, v14, v15, v16, v17) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = v12;
            let v13: Rc<str> = v13;
            let v14: Rc<str> = v14;
            let v15: US1 = v15;
            let v16: Rc<UH2> = v16;
            let v17: Rc<UH1> = v17;
            let v18: Rc<UH1> = method21(v0.clone(), v1, v2, v3, v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v17, v10);
            let v19: bool = &*v11 == &*v0;
            let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) = if v19 {
                let mut v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                (v4, v5, v6, v7, v8, v20)
            } else {
                (v11, v12, v13, v14, v15, v16)
            };
            Rc::new(UH1::UH1_Cons(v21, v22, v23, v24, v25, v26, v18))
        }
        UH1::UH1_Nil => {
            v10
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: US0 = v2;
                let mut v3: Rc<UH0> = v3;
                let v16: Rc<UH1> = match v2 {
                    US0::US0_SprintLine(v4, v5, v6, v7, v8) => {
                        let mut v4: Rc<str> = v4;
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: Rc<str> = v7;
                        let v8: US1 = v8;
                        let mut v9: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US1, Rc<UH2>) -> bool> = method19(v4.clone(), v5.clone(), v6.clone(), v7.clone());
                        let v10: bool = method20(v4.clone(), v5.clone(), v6.clone(), v7.clone(), v9, v1.clone());
                        if v10 {
                            let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                            method21(v4.clone(), v5.clone(), v6.clone(), v7.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8, v1, v11)
                        } else {
                            let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                            Rc::new(UH1::UH1_Cons(v4, v5, v6, v7, v8, v13, v1))
                        }
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v3, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US1 = v6;
                let v7: Rc<UH2> = v7;
                let v8: Rc<UH1> = v8;
                let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v1));
                (v0, v1) = (v8, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn sprints_of_1(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH3> = split_list_8(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method11(v5, v6);
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = method18(v7.clone(), v8);
    let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = method22(v9, v10);
    let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v11, v12)
}
fn method25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<UH2>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<UH2>, mut v11: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v10) {
        UH2::UH2_Cons(v12, v13, v14, v15) => {
            let v12: Rc<str> = v12;
            let v13: i32 = v13;
            let v14: US2 = v14;
            let v15: Rc<UH2> = v15;
            let v16: Rc<UH4> = method25(v0, v1, v2, v3, v4.clone(), v5, v6.clone(), v7.clone(), v8.clone(), v9.clone(), v15, v11);
            let v17: bool = match v14 {
                US2::US2_Committed => {
                    true
                }
                _ => {
                    false
                }
            };
            let v18: bool = match v4 {
                US1::US1_SprintOpen => {
                    true
                }
                _ => {
                    false
                }
            };
            Rc::new(UH4::UH4_Cons(v6, v7, v8, v9, v18, v12, v13, v17, v16))
        }
        UH2::UH2_Nil => {
            v11
        }
    }
}
fn flat_of_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<UH2>) -> Rc<UH4> {
    let v6: bool = match &*v5 {
        UH2::UH2_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v6 {
        let v7: bool = match v4 {
            US1::US1_SprintOpen => {
                true
            }
            _ => {
                false
            }
        };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH4::UH4_Cons(v0, v1, v2, v3, v7, v8, 0i32, true, v9))
    } else {
        let v11: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
        method25(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5.clone(), v0, v1, v2, v3, v5, v11)
    }
}
fn method26(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: bool = v6;
            let v7: Rc<str> = v7;
            let v8: i32 = v8;
            let mut v9: bool = v9;
            let v10: Rc<UH4> = v10;
            let v11: Rc<UH4> = method26(v10, v1);
            Rc::new(UH4::UH4_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v11))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn method23(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US1 = v6;
                let v7: Rc<UH2> = v7;
                let v8: Rc<UH1> = v8;
                let mut v9: Rc<UH4> = flat_of_24(v2, v3, v4, v5, v6, v7);
                let v10: Rc<UH4> = method26(v1, v9);
                (v0, v1) = (v8, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>>, mut v1: Rc<UH4>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: bool = v7;
                let v8: Rc<str> = v8;
                let mut v9: i32 = v9;
                let v10: bool = v10;
                let v11: Rc<UH4> = v11;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9, v10));
                let v12: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v11, v12);
                continue;
            }
            UH4::UH4_Nil => {
                return v2;
            }
        }
    }
}
fn flat_rows_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> {
    let mut v1: Rc<UH1> = sprints_of_1(v0);
    let mut v2: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH4> = method23(v1, v2);
    let mut v4: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method27(v4.clone(), v3, v5);
    let v7: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> {
        flat_rows_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method30(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH5::UH5_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: i32 = v2;
                let mut v3: i32 = v3;
                let mut v4: i32 = v4;
                let v5: bool = v5;
                let v6: Rc<UH5> = v6;
                let v7: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH5::UH5_Nil => {
                return v1;
            }
        }
    }
}
fn review_of_line_32(mut v0: Rc<str>) -> US4 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl sprint_review_"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v3: bool = v2 == false;
    if v3 {
        US4::US4_None
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: bool = v7 < 3i32;
        if v8 {
            US4::US4_None
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = v6.borrow()[2i32 as usize].clone();
            let v12: Rc<UH3> = split_list_8(v10, v11);
            match Rc::unwrap_or_clone(v12) {
                UH3::UH3_Cons(v13, v14) => {
                    let v13: Rc<str> = v13;
                    let v14: Rc<UH3> = v14;
                    match Rc::unwrap_or_clone(v14) {
                        UH3::UH3_Cons(v15, v16) => {
                            let v15: Rc<str> = v15;
                            let v16: Rc<UH3> = v16;
                            match Rc::unwrap_or_clone(v16) {
                                UH3::UH3_Cons(v17, v18) => {
                                    let v17: Rc<str> = v17;
                                    let v18: Rc<UH3> = v18;
                                    match Rc::unwrap_or_clone(v18) {
                                        UH3::UH3_Cons(v19, v20) => {
                                            let v19: Rc<str> = v19;
                                            let mut v20: Rc<UH3> = v20;
                                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) };
                                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v23: Rc<str> = Rc::<str>::from(v17.replace(&*v21, &*v22));
                                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                            let v25: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v22));
                                            let v26: i32 = (v25.clone().len() as i32);
                                            let v27: i32 = 0i32;
                                            let v28: i32 = method14(v25.clone(), v26, v27);
                                            let v29: i32 = v26.wrapping_sub(1i32);
                                            let v30: Rc<str> = string_slice(&v25, v28 as i64, v29 as i64);
                                            let v31: i32 = (v30.clone().len() as i32);
                                            let v32: i32 = method15(v30.clone(), v31);
                                            let v33: Rc<str> = string_slice(&v30, 0i32 as i64, v32 as i64);
                                            let v34: Option<i32> = (v33).trim().parse::<i32>().ok();
                                            let v35: Option<i32> = method16(v34);
                                            let mut v36: Rc<dyn Fn((i32)) -> US3> = closure2();
                                            let mut v37: Option<US3> = v35.map(|x| v36(x));
                                            let v39: US3 = US3::US3_None;
                                            let v40: US3 = v39.clone();
                                            let v41: US3 = v37.unwrap_or(v40);
                                            let v44: i32 = match v41 {
                                                US3::US3_None => {
                                                    0i32
                                                }
                                                US3::US3_Some(v42) => {
                                                    let v42: i32 = v42;
                                                    v42
                                                }
                                            };
                                            let v45: Rc<str> = Rc::<str>::from(v15.replace(&*v21, &*v22));
                                            let v46: Rc<str> = Rc::<str>::from(v45.replace(&*v24, &*v22));
                                            let v47: i32 = (v46.clone().len() as i32);
                                            let v48: i32 = 0i32;
                                            let v49: i32 = method14(v46.clone(), v47, v48);
                                            let v50: i32 = v47.wrapping_sub(1i32);
                                            let v51: Rc<str> = string_slice(&v46, v49 as i64, v50 as i64);
                                            let v52: i32 = (v51.clone().len() as i32);
                                            let v53: i32 = method15(v51.clone(), v52);
                                            let v54: Rc<str> = string_slice(&v51, 0i32 as i64, v53 as i64);
                                            let v55: Option<i32> = (v54).trim().parse::<i32>().ok();
                                            let v56: Option<i32> = method16(v55);
                                            let mut v57: Option<US3> = v56.map(|x| v36(x));
                                            let v59: US3 = US3::US3_None;
                                            let v60: US3 = v59.clone();
                                            let v61: US3 = v57.unwrap_or(v60);
                                            let v64: i32 = match v61 {
                                                US3::US3_None => {
                                                    0i32
                                                }
                                                US3::US3_Some(v62) => {
                                                    let v62: i32 = v62;
                                                    v62
                                                }
                                            };
                                            let v65: Rc<str> = Rc::<str>::from(v19.replace(&*v21, &*v22));
                                            let v66: Rc<str> = Rc::<str>::from(v65.replace(&*v24, &*v22));
                                            let v67: i32 = (v66.clone().len() as i32);
                                            let v68: i32 = 0i32;
                                            let v69: i32 = method14(v66.clone(), v67, v68);
                                            let v70: i32 = v67.wrapping_sub(1i32);
                                            let v71: Rc<str> = string_slice(&v66, v69 as i64, v70 as i64);
                                            let v72: i32 = (v71.clone().len() as i32);
                                            let v73: i32 = method15(v71.clone(), v72);
                                            let v74: Rc<str> = string_slice(&v71, 0i32 as i64, v73 as i64);
                                            let v75: Option<i32> = (v74).trim().parse::<i32>().ok();
                                            let v76: Option<i32> = method16(v75);
                                            let mut v77: Option<US3> = v76.map(|x| v36(x));
                                            let v79: US3 = US3::US3_None;
                                            let v80: US3 = v79.clone();
                                            let v81: US3 = v77.unwrap_or(v80);
                                            let v84: i32 = match v81 {
                                                US3::US3_None => {
                                                    0i32
                                                }
                                                US3::US3_Some(v82) => {
                                                    let v82: i32 = v82;
                                                    v82
                                                }
                                            };
                                            let v85: bool = v44 == 0i32;
                                            US4::US4_Some(v64, v44, v84, v85)
                                        }
                                        _ => {
                                            US4::US4_None
                                        }
                                    }
                                }
                                _ => {
                                    US4::US4_None
                                }
                            }
                        }
                        _ => {
                            US4::US4_None
                        }
                    }
                }
                _ => {
                    US4::US4_None
                }
            }
        }
    }
}
fn method31(mut v0: Rc<UH3>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH3> = v3;
                let mut v4: US4 = review_of_line_32(v2);
                let v11: Rc<UH5> = match v4 {
                    US4::US4_None => {
                        v1
                    }
                    US4::US4_Some(v5, v6, v7, v8) => {
                        let v5: i32 = v5;
                        let v6: i32 = v6;
                        let v7: i32 = v7;
                        let v8: bool = v8;
                        Rc::new(UH5::UH5_Cons(v5, v6, v7, v8, v1))
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn reviews_of_29(mut v0: Rc<str>) -> Rc<UH5> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH3> = split_list_8(v1, v4);
    let v6: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH5> = method31(v5, v6);
    let v8: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    method30(v7, v8)
}
fn method33(mut v0: Rc<UH5>, mut v1: Rc<UH6>) -> Rc<UH6> {
    match Rc::unwrap_or_clone(v0) {
        UH5::UH5_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: i32 = v2;
            let mut v3: i32 = v3;
            let mut v4: i32 = v4;
            let v5: bool = v5;
            let v6: Rc<UH5> = v6;
            let v7: Rc<UH6> = method33(v6, v1);
            Rc::new(UH6::UH6_Cons(v2, v3, v4, v7))
        }
        UH5::UH5_Nil => {
            v1
        }
    }
}
fn method34(mut v0: Rc<RefCell<Vec<(i32, i32, i32)>>>, mut v1: Rc<UH6>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH6::UH6_Cons(v3, v4, v5, v6) => {
                let mut v3: i32 = v3;
                let mut v4: i32 = v4;
                let v5: i32 = v5;
                let v6: Rc<UH6> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH6::UH6_Nil => {
                return v2;
            }
        }
    }
}
fn review_rows_of_28(mut v0: Rc<str>) -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
    let mut v1: Rc<UH5> = reviews_of_29(v0);
    let mut v2: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH6> = method33(v1, v2);
    let mut v4: Rc<RefCell<Vec<(i32, i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method34(v4.clone(), v3, v5);
    let v7: Rc<Vec<(i32, i32, i32)>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<(i32, i32, i32)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(i32, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(i32, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
        review_rows_of_28(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_flat_rows(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_sprint_review_rows(v0: &str) -> Rc<RefCell<Vec<(i32, i32, i32)>>> {
    closure4()(Rc::<str>::from(v0))
}
pub type SpiralShape_4fa472513c6d = Mut0;
