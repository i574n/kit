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
    UH0_Cons(Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_ParentSet(Rc<str>, Rc<str>),
    US3_ParentCleared(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_ParentsWritten(i32),
    US4_ParentsRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Replaced,
    US5_TemporaryWriteFailed,
    US5_RenameFailed,
    US5_RenameFailedTemporaryKept,
    US5_ReadbackMismatch,
    US5_ChangedSinceRead,
}
#[derive(Clone)]
enum US6 {
    US6_ModuleListed,
    US6_ModuleAdded(Rc<str>),
    US6_NoModulesSection,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
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
fn method5(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method5(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
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
                let mut v6: bool = v4 == b'\t';
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
                let mut v6: bool = v4 == b'\t';
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
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn loop_12(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
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
fn quoted_pair_11(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_3(v1, v0);
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let mut v6: Rc<UH1> = v6;
                    match Rc::unwrap_or_clone(v6) {
                        UH1::UH1_Nil => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
                            let v8: bool = v5.ends_with(&*v7);
                            if v8 {
                                let mut v9: Rc<str> = loop_12(v5);
                                let v10: bool = &*v3 == "";
                                let mut v12: bool = if v10 {
                                    true
                                } else {
                                    let v11: bool = &*v9 == "";
                                    v11
                                };
                                if v12 {
                                    US1::US1_None
                                } else {
                                    US1::US1_Some(v3, v9)
                                }
                            } else {
                                US1::US1_None
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
fn link_of_line_7(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method8(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method9(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" = Parent (\""); } LIT.with(|lit| lit.clone()) };
    let v10: Option<usize> = v8.find(&*v9);
    let v11: Option<usize> = method10(v10);
    let mut v12: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v13: Option<i32> = v11.map(|x| v12(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 < 0i32;
    if v16 {
        US0::US0_None
    } else {
        let v18: i32 = v15.wrapping_add(12i32);
        let v19: i32 = (v8.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(1i32);
        let v21: Rc<str> = string_slice(&v8, v18 as i64, v20 as i64);
        let v22: US1 = quoted_pair_11(v21);
        match v22 {
            US1::US1_None => {
                US0::US0_None
            }
            US1::US1_Some(v23, v24) => {
                let v23: Rc<str> = v23;
                let v24: Rc<str> = v24;
                US0::US0_Some(v23, v24)
            }
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US0 = link_of_line_7(v2);
                let mut v9: Rc<UH0> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        Rc::new(UH0::UH0_Cons(v5, v6, v1))
                    }
                };
                (v0, v1) = (v3, v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn links_of_text_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_3(v1, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method6(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v7, v8)
}
fn method13(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                v0.borrow_mut().push((v3, v4));
                let mut v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn links_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<UH0> = links_of_text_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let v4: i32 = method13(v2.clone(), v1, v3);
    let v5: Rc<Vec<(Rc<str>, Rc<str>)>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
        links_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_16() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_17(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_15(mut v0: i32) -> Rc<UH1> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_16();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_17(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let mut v12: Rc<UH1> = arguments_from_15(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method21(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_of_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_20(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method21(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn task_known_22(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) }, v1));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    let v4: bool = v0.contains(&*v3);
    v4
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method24(v4, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method25(mut v0: US3, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method25(v0.clone(), v5, v2);
            let v11: Rc<str> = match v0 {
                US3::US3_ParentCleared(v9) => {
                    let mut v9: Rc<str> = v9;
                    v9
                }
                US3::US3_ParentSet(v7, v8) => {
                    let v7: Rc<str> = v7;
                    let v8: Rc<str> = v8;
                    v7
                }
            };
            let mut v12: bool = &*v3 != &*v11;
            if v12 {
                Rc::new(UH0::UH0_Cons(v3, v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn links_changed_23(mut v0: Rc<UH0>, mut v1: US3) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = method25(v1.clone(), v0, v2);
    match v1 {
        US3::US3_ParentCleared(v9) => {
            let mut v9: Rc<str> = v9;
            v3
        }
        US3::US3_ParentSet(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5, v6));
            method24(v3, v7)
        }
    }
}
fn slug_of_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v3));
    v6
}
fn row_of_link_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = slug_of_29(v0.clone());
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl parent_of_"); } LIT.with(|lit| lit.clone()) }, v2));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Parent (\""); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v0));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) }));
    v8
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<UH0> = v4;
            let v5: Rc<UH1> = method27(v4, v1);
            let mut v6: Rc<str> = row_of_link_28(v2, v3);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH1::UH1_Cons(v7, v5))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method30(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
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
fn parents_text_26(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method27(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: i32 = 0i32;
    let v5: i32 = method30(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion task_parent =\n    | Parent :: string * string -> task_parent\n"); } LIT.with(|lit| lit.clone()) }, v9));
    v10
}
fn renamed_within_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replaced_31(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
    let v3: bool = std::fs::write(&*v2, &*v1).is_ok();
    let v16: US5 = if v3 {
        let v4: i32 = 40i32;
        let v5: bool = renamed_within_32(v0.clone(), v2.clone(), v4);
        if v5 {
            let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
            if v6 {
                US5::US5_Replaced
            } else {
                US5::US5_ReadbackMismatch
            }
        } else {
            let v10: bool = std::fs::remove_file(&*v2).is_ok() || !std::path::Path::new(&*v2).exists();
            if v10 {
                US5::US5_RenameFailed
            } else {
                US5::US5_RenameFailedTemporaryKept
            }
        }
    } else {
        US5::US5_TemporaryWriteFailed
    };
    match v16 {
        US5::US5_Replaced => {
            true
        }
        _ => {
            false
        }
    }
}
fn method33(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn module_listed_34(mut v0: Rc<str>) -> US6 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    parents\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    parents"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.ends_with(&*v3);
        v4
    };
    if v5 {
        US6::US6_ModuleListed
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        if v8 {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    parents\n"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v9));
            US6::US6_ModuleAdded(v10)
        } else {
            US6::US6_NoModulesSection
        }
    }
}
fn parents_changed_18(mut v0: Rc<str>, mut v1: US3) -> US4 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = text_of_19(v6);
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/parents.spi"); } LIT.with(|lit| lit.clone()) }));
    let mut v9: Rc<str> = text_of_19(v8.clone());
    let v25: Rc<str> = match &v1 {
        US3::US3_ParentCleared(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: bool = task_known_22(v7.clone(), v20.clone());
            if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            } else {
                v20
            }
        }
        US3::US3_ParentSet(v10, v11) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = v11.clone();
            let mut v12: bool = task_known_22(v7.clone(), v10.clone());
            let v14: Rc<str> = if v12 {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                v10.clone()
            };
            let v15: bool = &*v14 != "";
            if v15 {
                v10
            } else {
                let v16: bool = task_known_22(v7.clone(), v11.clone());
                if v16 {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v17
                } else {
                    v11
                }
            }
        }
    };
    let v26: Rc<UH0> = links_of_text_1(v9.clone());
    let v27: Rc<UH0> = links_changed_23(v26, v1);
    let v28: bool = &*v7 == "";
    if v28 {
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no backlog state at "); } LIT.with(|lit| lit.clone()) }, v5));
        US4::US4_ParentsRefused(v29)
    } else {
        let v31: bool = &*v25 != "";
        if v31 {
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task "); } LIT.with(|lit| lit.clone()) }, v25));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in state/agile.spi"); } LIT.with(|lit| lit.clone()) }));
            US4::US4_ParentsRefused(v33)
        } else {
            let v35: Rc<str> = text_of_19(v8.clone());
            let v36: bool = &*v35 != &*v9;
            if v36 {
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/parents.spi changed since it was read; run again"); } LIT.with(|lit| lit.clone()) };
                US4::US4_ParentsRefused(v37)
            } else {
                let v39: Rc<str> = parents_text_26(v27.clone());
                let v40: bool = replaced_31(v8, v39);
                let v41: bool = v40 == false;
                if v41 {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/parents.spi could not be written"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_ParentsRefused(v42)
                } else {
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
                    let v45: i32 = 0i32;
                    let v46: i32 = method33(v27, v45);
                    let v47: Rc<str> = text_of_19(v44.clone());
                    let v48: US6 = module_listed_34(v47);
                    match v48 {
                        US6::US6_ModuleAdded(v52) => {
                            let v52: Rc<str> = v52;
                            let v53: bool = replaced_31(v44, v52);
                            if v53 {
                                US4::US4_ParentsWritten(v46)
                            } else {
                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj not updated"); } LIT.with(|lit| lit.clone()) };
                                US4::US4_ParentsRefused(v55)
                            }
                        }
                        US6::US6_ModuleListed => {
                            US4::US4_ParentsWritten(v46)
                        }
                        US6::US6_NoModulesSection => {
                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list parents in"); } LIT.with(|lit| lit.clone()) };
                            US4::US4_ParentsRefused(v50)
                        }
                    }
                }
            }
        }
    }
}
fn method36(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_35(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method36(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn parent_edit_run_14() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH1> = arguments_from_15(v0);
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH1::UH1_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH1> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH1::UH1_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH1> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH1::UH1_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let mut v9: Rc<UH1> = v9;
                                    match Rc::unwrap_or_clone(v9) {
                                        UH1::UH1_Cons(v10, v11) => {
                                            let v10: Rc<str> = v10;
                                            let v11: Rc<UH1> = v11;
                                            match Rc::unwrap_or_clone(v11) {
                                                UH1::UH1_Cons(v12, v13) => {
                                                    let mut v12: Rc<str> = v12;
                                                    let mut v13: Rc<UH1> = v13;
                                                    match Rc::unwrap_or_clone(v13) {
                                                        UH1::UH1_Nil => {
                                                            let v14: bool = &*v10 == "--parent";
                                                            if v14 {
                                                                let v15: bool = &*v12 == "--clear";
                                                                let v18: US3 = if v15 {
                                                                    US3::US3_ParentCleared(v8.clone())
                                                                } else {
                                                                    US3::US3_ParentSet(v8.clone(), v12.clone())
                                                                };
                                                                let v19: US4 = parents_changed_18(v6, v18);
                                                                match v19 {
                                                                    US4::US4_ParentsRefused(v29) => {
                                                                        let v29: Rc<str> = v29;
                                                                        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile edit --parent: "); } LIT.with(|lit| lit.clone()) }, v29));
                                                                        eprintln!("{}", v30);
                                                                        2i32
                                                                    }
                                                                    US4::US4_ParentsWritten(v20) => {
                                                                        let v20: i32 = v20;
                                                                        let v22: Rc<str> = if v15 {
                                                                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
                                                                            v21
                                                                        } else {
                                                                            v12
                                                                        };
                                                                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" parent="); } LIT.with(|lit| lit.clone()) }, v22));
                                                                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" links="); } LIT.with(|lit| lit.clone()) }));
                                                                        let v25: Rc<str> = format_real_35(v20);
                                                                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
                                                                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile edit ok child="); } LIT.with(|lit| lit.clone()) }, v8));
                                                                        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
                                                                        println!("{}", v28);
                                                                        0i32
                                                                    }
                                                                }
                                                            } else {
                                                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                                eprintln!("{}", v33);
                                                                2i32
                                                            }
                                                        }
                                                        _ => {
                                                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                            eprintln!("{}", v35);
                                                            2i32
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                                    eprintln!("{}", v37);
                                                    2i32
                                                }
                                            }
                                        }
                                        _ => {
                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                            eprintln!("{}", v39);
                                            2i32
                                        }
                                    }
                                }
                                _ => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v41);
                                    2i32
                                }
                            }
                        }
                        _ => {
                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v43);
                            2i32
                        }
                    }
                }
                _ => {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v45);
                    2i32
                }
            }
        }
        _ => {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: agile edit <root> <child-id> --parent <epic-id|--clear>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v47);
            2i32
        }
    }
}
fn closure2() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        parent_edit_run_14()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_parent_links(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_parent_edit_run() -> i32 {
    closure2()()
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
