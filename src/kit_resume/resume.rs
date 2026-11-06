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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NextWork,
    US1_RecentJobs,
    US1_MachineWindows,
    US1_AuthorNotes,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_2();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn file_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn state_file_text_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = file_exists_9(v6.clone());
    if v7 {
        let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let v11: Vec<u8> = v9.unwrap();
        let v12: Vec<u8> = method10(v11);
        let v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
        let v16: std::string::String = v14.unwrap();
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        v18
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method14(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method13(v16, v17)
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
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
fn segment_modules_15(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: i32 = (v5.clone().len() as i32);
                let v8: i32 = 0i32;
                let mut v9: i32 = method16(v5.clone(), v7, v8);
                let v10: i32 = v7.wrapping_sub(1i32);
                let v11: Rc<str> = string_slice(&v5, v9 as i64, v10 as i64);
                let v12: i32 = (v11.clone().len() as i32);
                let mut v13: i32 = method17(v11.clone(), v12);
                let v14: Rc<str> = string_slice(&v11, 0i32 as i64, v13 as i64);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v14.ends_with(&*v15);
                if v16 {
                    let v17: bool = &*v14 == "modules:";
                    (v0, v1) = (v6, v17);
                    continue;
                } else {
                    let v21: bool = if v1 {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
                        let v20: bool = v14.starts_with(&*v19);
                        v20
                    } else {
                        false
                    };
                    if v21 {
                        let v22: bool = true;
                        let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v2));
                        (v0, v1, v2) = (v6, v22, v23);
                        continue;
                    } else {
                        v0 = v6;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method13(v2, v3);
            }
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let v5: Rc<UH0> = method18(v0.clone(), v4, v2);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            v6.borrow_mut().push(v7);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = state_file_text_8(v0, v11);
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            v13.borrow_mut().push(v14);
            v13.borrow_mut().push(v12);
            let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
            let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v17, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn state_segments_text_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = state_file_text_8(v0.clone(), v2);
    let mut v4: Rc<UH0> = split_list_12(v1, v3);
    let v5: bool = false;
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = segment_modules_15(v4, v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method18(v0, v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let v12: i32 = method19(v10.clone(), v9, v11);
    let mut v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    v16
}
fn state_text_with_notes_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = state_file_text_8(v0.clone(), v1);
    let v3: Rc<str> = state_segments_text_11(v0);
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v4.borrow_mut().push(v2);
    v4.borrow_mut().push(v3);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn state_text_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v1.borrow_mut().push(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    state_text_with_notes_7(v6)
}
fn declared_from_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn field_end_24(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US2::US2_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US2 = match v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_25(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_23(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US2 = US2::US2_ScanPlain;
            let mut v6: i32 = field_end_24(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_25(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn concat5_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn unescaped_from_27(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_28(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_28(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_27(v3, v4, v5, v6, v7)
}
fn note_at_22(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let v5: i32 = field_start_23(v0.clone(), v2, v3, v4);
    let mut v6: US2 = US2::US2_ScanPlain;
    let v7: i32 = field_end_24(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_26(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_23(v0.clone(), v2, v3, v10);
    let v12: US2 = US2::US2_ScanPlain;
    let mut v13: i32 = field_end_24(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_26(v14);
    let v16: i32 = 4i32;
    let v17: i32 = field_start_23(v0.clone(), v2, v3, v16);
    let v18: US2 = US2::US2_ScanPlain;
    let v19: i32 = field_end_24(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_26(v20);
    (v9, v15, v21)
}
fn authored_row_29(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v5: i32 = v1.wrapping_add(6i32);
    let mut v6: i32 = (v0.clone().len() as i32);
    let v7: i32 = 0i32;
    let v8: i32 = field_start_23(v0.clone(), v5, v6, v7);
    let mut v9: US2 = US2::US2_ScanPlain;
    let v10: i32 = field_end_24(v0.clone(), v8, v6, v9);
    let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v8 as usize..v10 as usize).unwrap_or(""));
    let v12: Rc<str> = unquoted_26(v11);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v12));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = std::rc::Rc::<str>::from((v2).replace(char::from(10u8), " ").chars().take(160i32 as usize).collect::<String>().as_str());
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    v16
}
fn authored_rows_from_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = declared_from_21(v0.clone(), v1.clone(), v3);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v4;
        } else {
            let (mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>) = note_at_22(v0.clone(), v5);
            let v10: bool = &*v8 == &*v2;
            let v14: Rc<str> = if v10 {
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v12: Rc<str> = authored_row_29(v0.clone(), v5, v9, v7);
                let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                v13
            } else {
                v4
            };
            let v15: i32 = v5.wrapping_add(1i32);
            (v3, v4) = (v15, v14);
            continue;
        }
    }
}
fn author_notes_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = state_text_6(v0);
    let mut v4: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = authored_rows_from_20(v4, v5, v2.clone(), v6, v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from({ let mut rows = (v8).split(char::from(10u8)).filter(|row| !row.is_empty()).collect::<Vec<_>>(); rows.sort_by(|a, b| b.cmp(a)); rows.truncate(8i32 as usize); rows.join(&char::from(10u8).to_string()) }.as_str());
    let v10: bool = &*v9 == "";
    if v10 {
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no notes by "); } LIT.with(|lit| lit.clone()) }, v2));
        v11
    } else {
        v9
    }
}
fn section_shown_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: i32) -> i32 {
    let v7: bool = match &v3 {
        US1::US1_AuthorNotes => {
            let v5: bool = &*v2 == "";
            let mut v6: bool = v5 == false;
            v6
        }
        _ => {
            true
        }
    };
    if v7 {
        let v15: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent notes by author (agile notes --as)"); } LIT.with(|lit| lit.clone()) };
                v11
            }
            US1::US1_MachineWindows => {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine windows (window list)"); } LIT.with(|lit| lit.clone()) };
                v10
            }
            US1::US1_NextWork => {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== next work (agile next, WSJF)"); } LIT.with(|lit| lit.clone()) };
                v8
            }
            US1::US1_RecentJobs => {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== recent jobs (job list)"); } LIT.with(|lit| lit.clone()) };
                v9
            }
        };
        println!("{}", v15);
        let v30: Rc<str> = match &v3 {
            US1::US1_AuthorNotes => {
                author_notes_5(v0, v1, v2)
            }
            _ => {
                let v28: Rc<str> = match v3 {
                    US1::US1_MachineWindows => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window\nlist"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US1::US1_NextWork => {
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile\nnext\n"); } LIT.with(|lit| lit.clone()) }, v0));
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n8"); } LIT.with(|lit| lit.clone()) }));
                        let v19: bool = &*v1 == "";
                        let v22: Rc<str> = if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--team\n"); } LIT.with(|lit| lit.clone()) }, v1));
                            v21
                        };
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v22));
                        v23
                    }
                    US1::US1_RecentJobs => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job\nlist\n12"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    _ => unreachable!(),
                };
                let v29: Rc<str> = std::rc::Rc::<str>::from(std::env::current_exe().ok().and_then(|exe| std::process::Command::new(exe).args((v28).split(char::from(10u8)).filter(|item| !item.is_empty())).output().ok()).map(|out| String::from_utf8_lossy(&out.stdout).trim_end().to_owned()).unwrap_or_default().as_str());
                v29
            }
        };
        println!("{}", v30);
        let v31: i32 = v4.wrapping_add(1i32);
        v31
    } else {
        v4
    }
}
fn method31(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_30(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn resume_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v2).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v4).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let mut v6: bool = &*v1 == "";
    let mut v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.starts_with(&*v7);
        v8
    };
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: usage: kit resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v10);
        2i32
    } else {
        let v11: US1 = US1::US1_NextWork;
        let v12: i32 = 0i32;
        let mut v13: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v11, v12);
        let v14: US1 = US1::US1_RecentJobs;
        let v15: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v14, v13);
        let v16: US1 = US1::US1_MachineWindows;
        let v17: i32 = section_shown_4(v1.clone(), v3.clone(), v5.clone(), v16, v15);
        let v18: US1 = US1::US1_AuthorNotes;
        let v19: i32 = section_shown_4(v1, v3.clone(), v5.clone(), v18, v17);
        let v20: Rc<str> = format_real_30(v19);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit resume ok sections="); } LIT.with(|lit| lit.clone()) }, v20));
        let v22: bool = &*v3 == "";
        let v25: Rc<str> = if v22 {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v23
        } else {
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" team="); } LIT.with(|lit| lit.clone()) }, v3));
            v24
        };
        let v26: bool = &*v5 == "";
        let v29: Rc<str> = if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v27
        } else {
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) }, v5));
            v28
        };
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v29));
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v30));
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unix="); } LIT.with(|lit| lit.clone()) }));
        let v33: Rc<str> = std::rc::Rc::<str>::from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0).to_string().as_str());
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
        println!("{}", v34);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        resume_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_resume_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
