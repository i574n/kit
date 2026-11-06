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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<UH0>, Rc<UH3>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>),
    US4_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn file_exists_6(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn file_text_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_6(v0.clone());
    if v1 {
        let mut v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method7(v5);
        let mut v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
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
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_19(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(2i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn quoted_pair_18(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_10(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Nil => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                            let mut v8: bool = v5.ends_with(&*v7);
                            if v8 {
                                let mut v9: Rc<str> = loop_19(v5);
                                let v10: bool = &*v3 == "";
                                let mut v12: bool = if v10 {
                                    true
                                } else {
                                    let v11: bool = &*v9 == "";
                                    v11
                                };
                                if v12 {
                                    US2::US2_None
                                } else {
                                    US2::US2_Some(v3, v9)
                                }
                            } else {
                                US2::US2_None
                            }
                        }
                        _ => {
                            US2::US2_None
                        }
                    }
                }
                _ => {
                    US2::US2_None
                }
            }
        }
        _ => {
            US2::US2_None
        }
    }
}
fn link_of_line_14(mut v0: Rc<str>) -> US1 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method15(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method16(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Parent (\""); } LIT.with(|lit| lit.clone()) };
    let v10: Option<usize> = v8.find(&*v9);
    let v11: Option<usize> = method17(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    if v16 {
        US1::US1_None
    } else {
        let v18: i32 = v15.wrapping_add(12i32);
        let v19: i32 = (v8.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(1i32);
        let v21: Rc<str> = string_slice(&v8, v18 as i64, v20 as i64);
        let v22: US2 = quoted_pair_18(v21);
        match v22 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v23, v24) => {
                let v23: Rc<str> = v23;
                let v24: Rc<str> = v24;
                US1::US1_Some(v23, v24)
            }
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: US1 = link_of_line_14(v2);
                let mut v9: Rc<UH1> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        Rc::new(UH1::UH1_Cons(v5, v6, v1))
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn links_of_text_8(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_10(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH1> = method13(v5, v6);
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method9(v7, v8)
}
fn method20(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let mut v12: i32 = v1.wrapping_sub(1i32);
            let mut v13: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v2));
            (v1, v2) = (v12, v13);
            continue;
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method23(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<UH3>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<UH0> = v2;
                let mut v3: Rc<UH3> = v3;
                let v4: Rc<UH0> = method23(v1, v2);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v2: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US3> = method26(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method26(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure2(v0, v1)
}
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: i32 = 0i32;
                let mut v6: Rc<dyn Fn(Rc<UH0>) -> US3> = method26(v3.clone(), v5);
                let v7: US3 = v6(v1.clone());
                let mut v8: bool = match v7 {
                    US3::US3_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = v8 != true;
                let mut v13: Rc<UH0> = if v9 {
                    v1
                } else {
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v10));
                    method23(v1, v11)
                };
                (v0, v1) = (v4, v13);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn epics_of_24(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method25(v0, v1)
}
fn method30(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            let v5: Rc<UH0> = method30(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v5))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method31(v0.clone(), v5, v2);
            let v7: bool = &*v4 == &*v0;
            if v7 {
                Rc::new(UH1::UH1_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn children_of_29(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH1> = method31(v1, v0, v2);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method30(v3, v4)
}
fn method34(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: US4) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5, v6, v7, v8, v9, v10) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let mut v6: i32 = v6;
                let v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<UH2> = v10;
                let v11: bool = &*v3 == &*v0;
                let mut v13: US4 = if v11 {
                    US4::US4_Some(v3, v4, v5, v6, v7, v8, v9)
                } else {
                    v2
                };
                (v1, v2) = (v10, v13);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn fact_for_33(mut v0: Rc<UH2>, mut v1: Rc<str>) -> US4 {
    let mut v2: US4 = US4::US4_None;
    method34(v1, v0, v2)
}
fn method32(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: US4 = fact_for_33(v0.clone(), v4);
                let (mut v19, mut v20): (i32, i32) = match v6 {
                    US4::US4_Some(v7, v8, v9, v10, v11, v12, v13) => {
                        let v7: Rc<str> = v7;
                        let mut v8: Rc<str> = v8;
                        let mut v9: Rc<str> = v9;
                        let v10: i32 = v10;
                        let v11: Rc<str> = v11;
                        let mut v12: Rc<str> = v12;
                        let mut v13: Rc<str> = v13;
                        let v14: bool = &*v8 == "Done";
                        if v14 {
                            let v15: i32 = v2.wrapping_add(1i32);
                            let v16: i32 = v3.wrapping_add(v10);
                            (v15, v16)
                        } else {
                            (v2, v3)
                        }
                    }
                    _ => {
                        (v2, v3)
                    }
                };
                (v1, v2, v3) = (v5, v19, v20);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn method35(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: US4 = fact_for_33(v0.clone(), v4);
                let (mut v16, mut v17): (i32, i32) = match v6 {
                    US4::US4_Some(v7, v8, v9, v10, v11, v12, v13) => {
                        let v7: Rc<str> = v7;
                        let mut v8: Rc<str> = v8;
                        let mut v9: Rc<str> = v9;
                        let v10: i32 = v10;
                        let v11: Rc<str> = v11;
                        let mut v12: Rc<str> = v12;
                        let mut v13: Rc<str> = v13;
                        let v14: i32 = v2.wrapping_add(1i32);
                        let v15: i32 = v3.wrapping_add(v10);
                        (v14, v15)
                    }
                    _ => {
                        (v2, v3)
                    }
                };
                (v1, v2, v3) = (v5, v16, v17);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn method37(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_36(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method37(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn epic_line_28(mut v0: Rc<UH2>, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<UH0> = children_of_29(v1.clone(), v2.clone());
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let (mut v6, mut v7): (i32, i32) = method32(v0.clone(), v3, v4, v5);
    let mut v8: Rc<UH0> = children_of_29(v1, v2.clone());
    let mut v9: i32 = 0i32;
    let v10: i32 = 0i32;
    let (mut v11, mut v12): (i32, i32) = method35(v0, v8, v9, v10);
    let mut v13: Rc<str> = format_real_36(v6);
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  done="); } LIT.with(|lit| lit.clone()) }, v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = format_real_36(v11);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) }));
    let v20: Rc<str> = format_real_36(v7);
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v23: Rc<str> = format_real_36(v12);
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
    v24
}
fn child_line_39(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US4 = fact_for_33(v0, v1.clone());
    match v2 {
        US4::US4_None => {
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v1));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unknown"); } LIT.with(|lit| lit.clone()) }));
            v18
        }
        US4::US4_Some(v3, v4, v5, v6, v7, v8, v9) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: i32 = v6;
            let v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v1));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v4));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = format_real_36(v6);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v16
        }
    }
}
fn method38(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method38(v0.clone(), v4, v2);
            let mut v6: Rc<str> = child_line_39(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method27(mut v0: Rc<UH2>, mut v1: Rc<UH1>, mut v2: Rc<UH2>, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v4) {
        UH0::UH0_Cons(v6, v7) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<UH0> = v7;
            let mut v8: Rc<UH3> = method27(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v7, v5);
            let mut v9: Rc<str> = epic_line_28(v0, v1, v6.clone());
            let v10: Rc<UH0> = children_of_29(v3, v6);
            let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v12: Rc<UH0> = method38(v2, v10, v11);
            let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v12));
            Rc::new(UH3::UH3_Cons(v13, v8))
        }
        UH0::UH0_Nil => {
            v5
        }
    }
}
fn tree_text_of_21(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH0> {
    let mut v2: Rc<UH0> = epics_of_24(v1.clone());
    let mut v3: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH3> = method27(v0.clone(), v1.clone(), v0, v1, v2, v3);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method22(v4, v5)
}
fn method40(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                println!("{}", v1);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method41(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method42(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn tree_run_1() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<str> = Rc::<str>::from(v6.replace(&*v8, &*v9));
                            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
                            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/parents.spi"); } LIT.with(|lit| lit.clone()) }));
                            let mut v13: Rc<str> = file_text_5(v12);
                            let v14: Rc<UH1> = links_of_text_8(v13);
                            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
                            let v16: Rc<str> = file_text_5(v15);
                            let v17: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_facts::kit_agile_fact_rows(&v16);
                            let v18: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v17).borrow().clone());
                            let v19: usize = (v18).len();
                            let v20: i32 = ((v19) as i32);
                            let v21: i32 = v20.wrapping_sub(1i32);
                            let v22: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                            let v23: Rc<UH2> = method20(v18, v21, v22);
                            let v24: Rc<UH0> = tree_text_of_21(v23, v14.clone());
                            method40(v24);
                            let v25: Rc<UH0> = epics_of_24(v14.clone());
                            let v26: i32 = 0i32;
                            let v27: i32 = method41(v25, v26);
                            let v28: Rc<str> = format_real_36(v27);
                            let v29: i32 = 0i32;
                            let v30: i32 = method42(v14, v29);
                            let v31: Rc<str> = format_real_36(v30);
                            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile list ok tree epics="); } LIT.with(|lit| lit.clone()) }, v28));
                            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" links="); } LIT.with(|lit| lit.clone()) }));
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v31));
                            println!("{}", v34);
                            0i32
                        }
                        _ => {
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v35);
                            2i32
                        }
                    }
                }
                _ => {
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v37);
                    2i32
                }
            }
        }
        _ => {
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile list <root> --tree"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v39);
            2i32
        }
    }
}
fn closure3(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    Rc::new(move |mut v1: Rc<UH0>| -> US3 {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == "--tree";
                if v5 {
                    US3::US3_Some(v0)
                } else {
                    let v7: i32 = v0.wrapping_add(1i32);
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US3> = method44(v7);
                    v8(v4)
                }
            }
            UH0::UH0_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method44(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US3> {
    closure3(v0)
}
fn tree_wanted_43() -> bool {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: i32 = 0i32;
    let mut v3: Rc<dyn Fn(Rc<UH0>) -> US3> = method44(v2);
    let v4: US3 = v3(v1);
    let v5: bool = match v4 {
        US3::US3_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v6: bool = v5 != true;
    v6
}
fn list_routed_0() -> i32 {
    let mut v0: bool = tree_wanted_43();
    if v0 {
        tree_run_1()
    } else {
        let mut v2: i32 = agile_task_listing::kit_agile_list_run();
        v2
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        list_routed_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_list_routed() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
