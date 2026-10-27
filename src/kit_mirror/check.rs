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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32 }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Missing,
    US2_Present(Rc<str>, u64),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US2, US2, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_SameOnBoth,
    US3_LeftOnly,
    US3_RightOnly,
    US3_LeftNewer,
    US3_RightNewer,
}
#[derive(Clone)]
enum US4 {
    US4_InStep,
    US4_LeftAhead(Rc<str>),
    US4_RightAhead(Rc<str>),
    US4_Diverged(Rc<str>),
}
struct Mut2 { l0: Rc<str> }
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn directory_exists_3(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method7(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method8(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0.clone();
    let v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut1>>) -> bool {
    let v3: i32 = v2.borrow().l0.clone();
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0.clone();
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let mut v8: bool = &*v7 < &*v1;
        let v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn sorted_names_6(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Vec<std::string::String> = method7(v4.clone());
    let mut v6: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v5));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
    let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method8(v7, v9.clone()) {
        let v11: i32 = v9.borrow().l0.clone();
        let v12: std::string::String = v6.clone().borrow()[v11 as usize].clone();
        let v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        v8.clone().borrow_mut()[v11 as usize] = v14.clone();
        let v15: i32 = v11.wrapping_add(1i32);
        v9.borrow_mut().l0 = v15;
        ()
    };
    let v16: Rc<RefCell<Vec<Rc<str>>>> = v8 ;
    let mut v17: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method9();
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v16.borrow().clone()));
    let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v16.borrow().iter().map(|x| v17(x.clone())).collect::<Vec<_>>()));
    let v20: i32 = (v16.clone().borrow().len() as i32);
    let mut v21: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
    while method8(v20, v21.clone()) {
        let v23: i32 = v21.borrow().l0.clone();
        let v24: Rc<str> = v18.clone().borrow()[v23 as usize].clone();
        let v25: Rc<str> = v19.clone().borrow()[v23 as usize].clone();
        let mut v26: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v23 }));
        while method10(v19.clone(), v25.clone(), v26.clone()) {
            let v28: i32 = v26.borrow().l0.clone();
            let v29: i32 = v28.wrapping_sub(1i32);
            let v30: Rc<str> = v18.clone().borrow()[v29 as usize].clone();
            v18.clone().borrow_mut()[v28 as usize] = v30.clone();
            let v31: Rc<str> = v19.clone().borrow()[v29 as usize].clone();
            v19.clone().borrow_mut()[v28 as usize] = v31.clone();
            v26.borrow_mut().l0 = v29;
            ()
        };
        let v32: i32 = v26.borrow().l0.clone();
        v18.clone().borrow_mut()[v32 as usize] = v24.clone();
        let v33: i32 = v26.borrow().l0.clone();
        v19.clone().borrow_mut()[v33 as usize] = v25.clone();
        let v34: i32 = v23.wrapping_add(1i32);
        v21.borrow_mut().l0 = v34;
        ()
    };
    let v35: Rc<RefCell<Vec<Rc<str>>>> = v18 ;
    v35.clone()
}
fn children_pushed_11(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let mut v7: bool = &*v0 == "";
            let v11: Rc<str> = if v7 {
                v6.clone()
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v8)), v6));
                v10.clone()
            };
            let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v3.clone()));
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v5, v12.clone());
            continue;
        }
    }
}
fn file_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn files_under_5(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = &*v3 == "";
                let mut v9: Rc<str> = if v5 {
                    v0.clone()
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v6)), v3));
                    v8.clone()
                };
                let v10: bool = directory_exists_3(v9.clone());
                if v10 {
                    let v11: Rc<RefCell<Vec<Rc<str>>>> = sorted_names_6(v9.clone());
                    let v12: i32 = (v11.clone().borrow().len() as i32);
                    let v13: i32 = v12.wrapping_sub(1i32);
                    let v14: Rc<UH0> = children_pushed_11(v3.clone(), v11.clone(), v13, v4.clone());
                    (v0, v1, v2) = (v0.clone(), v14.clone(), v2.clone());
                    continue;
                } else {
                    let v16: bool = file_exists_12(v9.clone());
                    if v16 {
                        let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v2.clone()));
                        (v0, v1, v2) = (v0.clone(), v4.clone(), v17.clone());
                        continue;
                    } else {
                        (v0, v1, v2) = (v0.clone(), v4.clone(), v2.clone());
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
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
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v2: Rc<UH0>| -> US1 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method15(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure2(v0, v1)
}
fn merged_14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = 0i32;
                let mut v5: Rc<dyn Fn(Rc<UH0>) -> US1> = method15(v2.clone(), v4);
                let mut v6: US1 = v5(v1.clone());
                let mut v7: bool = match &v6 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v8: bool = v7 != true;
                let v10: Rc<UH0> = if v8 {
                    v1.clone()
                } else {
                    Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()))
                };
                (v0, v1) = (v3.clone(), v10.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn paths_from_4(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v5: bool = v0 >= v1;
        if v5 {
            return v4.clone();
        } else {
            let mut v6: bool = v0 >= 0i32;
            let mut v9: bool = if v6 {
                let mut v7: i32 = command_line_argument_count_1();
                let mut v8: bool = v0 < v7;
                v8
            } else {
                false
            };
            let v13: US0 = if v9 {
                let v10: Rc<str> = command_line_argument_2(v0);
                US0::US0_Some(v10.clone())
            } else {
                US0::US0_None
            };
            let mut v17: Rc<str> = match &v13 {
                US0::US0_None => {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v15.clone()
                }
                US0::US0_Some(v14) => {
                    let v14: Rc<str> = v14.clone();
                    v14.clone()
                }
            };
            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v18.clone()));
            let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v21: Rc<UH0> = files_under_5(v2.clone(), v19.clone(), v20.clone());
            let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v23: Rc<UH0> = method13(v21.clone(), v22.clone());
            let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v24.clone()));
            let mut v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v27: Rc<UH0> = files_under_5(v3.clone(), v25.clone(), v26.clone());
            let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v29: Rc<UH0> = method13(v27.clone(), v28.clone());
            let v30: i32 = v0.wrapping_add(1i32);
            let v31: Rc<UH0> = merged_14(v23.clone(), v4.clone());
            let v32: Rc<UH0> = merged_14(v29.clone(), v31.clone());
            (v0, v1, v2, v3, v4) = (v30, v1, v2.clone(), v3.clone(), v32.clone());
            continue;
        }
    }
}
fn method17(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn last_write_ms_18(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let mut v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let mut v9: u64 = v8.as_secs();
            let v10: u64 = v9.wrapping_mul(1000u64);
            let v11: u32 = v8.subsec_millis();
            let v12: u64 = (v11 as u64);
            let v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn method19(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: US2 = v3.clone();
                let mut v4: US2 = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn mirrored_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH0> = v7.clone();
                let mut v8: bool = &*v6 == "";
                let v12: Rc<str> = if v8 {
                    v0.clone()
                } else {
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v9)), v6));
                    v11.clone()
                };
                let v13: bool = file_exists_12(v12.clone());
                let v28: US2 = if v13 {
                    let v15: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v12);
                    let mut v17: Vec<u8> = v15.unwrap();
                    let v18: Vec<u8> = method17(v17.clone());
                    let v20: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v18);
                    let v22: std::string::String = v20.unwrap();
                    let v24: Rc<str> = Rc::<str>::from(String::as_str(&v22));
                    let v25: u64 = last_write_ms_18(v12.clone());
                    US2::US2_Present(v24.clone(), v25)
                } else {
                    US2::US2_Missing
                };
                let v32: Rc<str> = if v8 {
                    v1.clone()
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v29)), v6));
                    v31.clone()
                };
                let v33: bool = file_exists_12(v32.clone());
                let v48: US2 = if v33 {
                    let v35: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v32);
                    let v37: Vec<u8> = v35.unwrap();
                    let v38: Vec<u8> = method17(v37.clone());
                    let v40: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v38);
                    let v42: std::string::String = v40.unwrap();
                    let v44: Rc<str> = Rc::<str>::from(String::as_str(&v42));
                    let v45: u64 = last_write_ms_18(v32.clone());
                    US2::US2_Present(v44.clone(), v45)
                } else {
                    US2::US2_Missing
                };
                let v49: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6.clone(), v28.clone(), v48.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v7.clone(), v49.clone());
                continue;
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method19(v3.clone(), v4.clone());
            }
        }
    }
}
fn drift_of_21(mut v0: Rc<str>, mut v1: US2, mut v2: US2) -> US3 {
    match &v1 {
        US2::US2_Missing => {
            match &v2 {
                US2::US2_Missing => {
                    US3::US3_SameOnBoth
                }
                US2::US2_Present(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let mut v5: u64 = *v5;
                    US3::US3_RightOnly
                }
            }
        }
        US2::US2_Present(v9, v10) => {
            let mut v9: Rc<str> = v9.clone();
            let v10: u64 = *v10;
            match &v2 {
                US2::US2_Missing => {
                    US3::US3_LeftOnly
                }
                US2::US2_Present(v12, v13) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: u64 = *v13;
                    let v14: bool = &*v9 == &*v12;
                    if v14 {
                        US3::US3_SameOnBoth
                    } else {
                        let v16: bool = v10 > v13;
                        if v16 {
                            US3::US3_LeftNewer
                        } else {
                            US3::US3_RightNewer
                        }
                    }
                }
            }
        }
    }
}
fn drift_lines_20(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: US2 = v4.clone();
                let mut v5: US2 = v5.clone();
                let mut v6: Rc<UH1> = v6.clone();
                let mut v7: US3 = drift_of_21(v3.clone(), v4.clone(), v5.clone());
                match &v7 {
                    US3::US3_SameOnBoth => {
                        (v0, v1, v2) = (v6.clone(), v1.clone(), v2);
                        continue;
                    }
                    _ => {
                        let mut v17: Rc<str> = match &v7 {
                            US3::US3_LeftNewer => {
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-newer"); } LIT.with(|lit| lit.clone()) };
                                v12.clone()
                            }
                            US3::US3_LeftOnly => {
                                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-only"); } LIT.with(|lit| lit.clone()) };
                                v10.clone()
                            }
                            US3::US3_RightNewer => {
                                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-newer"); } LIT.with(|lit| lit.clone()) };
                                v13.clone()
                            }
                            US3::US3_RightOnly => {
                                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-only"); } LIT.with(|lit| lit.clone()) };
                                v11.clone()
                            }
                            _ => unreachable!(),
                        };
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v17));
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v3));
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                        let v24: u64 = v2.wrapping_add(1u64);
                        (v0, v1, v2) = (v6.clone(), v23.clone(), v24);
                        continue;
                    }
                }
            }
            UH1::UH1_Nil => {
                return (v1.clone(), v2);
            }
        }
    }
}
fn mirror_state_of_22(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>) -> US4 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v17, v18, v19, v20) => {
                let mut v17: Rc<str> = v17.clone();
                let v18: US2 = v18.clone();
                let v19: US2 = v19.clone();
                let v20: Rc<UH1> = v20.clone();
                let mut v21: US3 = drift_of_21(v17.clone(), v18.clone(), v19.clone());
                match &v21 {
                    US3::US3_SameOnBoth => {
                        (v0, v1, v2) = (v20.clone(), v1.clone(), v2.clone());
                        continue;
                    }
                    _ => {
                        let v24: bool = match &v21 {
                            US3::US3_LeftNewer => {
                                true
                            }
                            US3::US3_LeftOnly => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v24 {
                            let v25: bool = &*v1 == "";
                            let v29: Rc<str> = if v25 {
                                v17.clone()
                            } else {
                                let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v26)), v17));
                                v28.clone()
                            };
                            (v0, v1, v2) = (v20.clone(), v29.clone(), v2.clone());
                            continue;
                        } else {
                            let v31: bool = &*v2 == "";
                            let v35: Rc<str> = if v31 {
                                v17.clone()
                            } else {
                                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v32)), v17));
                                v34.clone()
                            };
                            (v0, v1, v2) = (v20.clone(), v1.clone(), v35.clone());
                            continue;
                        }
                    }
                }
            }
            UH1::UH1_Nil => {
                let v3: bool = &*v1 == "";
                let mut v5: bool = if v3 {
                    let mut v4: bool = &*v2 == "";
                    v4
                } else {
                    false
                };
                if v5 {
                    return US4::US4_InStep;
                } else {
                    let mut v7: bool = &*v2 == "";
                    if v7 {
                        return US4::US4_LeftAhead(v1.clone());
                    } else {
                        if v3 {
                            return US4::US4_RightAhead(v2.clone());
                        } else {
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) };
                            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v10)), v2));
                            return US4::US4_Diverged(v12.clone());
                        }
                    }
                }
            }
        }
    }
}
fn loop_23(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn method24(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: US2 = v3.clone();
                let mut v4: US2 = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method26(mut v0: Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_25(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn format_real_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn mirror_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = command_line_argument_count_1();
    let v2: bool = 2i32 < v1;
    let mut v7: US0 = if v2 {
        let v3: i32 = 2i32;
        let mut v4: Rc<str> = command_line_argument_2(v3);
        US0::US0_Some(v4.clone())
    } else {
        US0::US0_None
    };
    let v11: Rc<str> = match &v7 {
        US0::US0_None => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8.clone();
            v8.clone()
        }
    };
    let v12: i32 = command_line_argument_count_1();
    let v13: bool = 3i32 < v12;
    let v18: US0 = if v13 {
        let v14: i32 = 3i32;
        let v15: Rc<str> = command_line_argument_2(v14);
        US0::US0_Some(v15.clone())
    } else {
        US0::US0_None
    };
    let v22: Rc<str> = match &v18 {
        US0::US0_None => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v20.clone()
        }
        US0::US0_Some(v19) => {
            let v19: Rc<str> = v19.clone();
            v19.clone()
        }
    };
    let v23: i32 = command_line_argument_count_1();
    let v24: bool = 1i32 < v23;
    let v29: US0 = if v24 {
        let v25: i32 = 1i32;
        let mut v26: Rc<str> = command_line_argument_2(v25);
        US0::US0_Some(v26.clone())
    } else {
        US0::US0_None
    };
    let v33: Rc<str> = match &v29 {
        US0::US0_None => {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v31.clone()
        }
        US0::US0_Some(v30) => {
            let v30: Rc<str> = v30.clone();
            v30.clone()
        }
    };
    let v34: bool = &*v33 != "check";
    let v36: bool = if v34 {
        true
    } else {
        let v35: bool = v0 < 5i32;
        v35
    };
    if v36 {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror "); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror usage: mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v39);
        2i32
    } else {
        let v40: bool = directory_exists_3(v11.clone());
        let v41: bool = v40 == false;
        let v44: bool = if v41 {
            true
        } else {
            let v42: bool = directory_exists_3(v22.clone());
            let v43: bool = v42 == false;
            v43
        };
        if v44 {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror check needs two existing roots, received "); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v11));
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" and "); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v47)), v22));
            eprintln!("{}", v49);
            2i32
        } else {
            let v50: i32 = 4i32;
            let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v52: Rc<UH0> = paths_from_4(v50, v0, v11.clone(), v22.clone(), v51.clone());
            let v53: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v54: Rc<UH0> = method13(v52.clone(), v53.clone());
            let v55: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v56: Rc<UH1> = mirrored_16(v11.clone(), v22.clone(), v54.clone(), v55.clone());
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v58: u64 = 0u64;
            let (mut v59, mut v60): (Rc<str>, u64) = drift_lines_20(v56.clone(), v57.clone(), v58);
            let v61: US4 = mirror_state_of_22(v56.clone(), v57.clone(), v57.clone());
            let v62: bool = &*v59 != "";
            if v62 {
                let v63: Rc<str> = loop_23(v59.clone());
                println!("{}", v63);
                ()
            };
            let v74: Rc<str> = match &v61 {
                US4::US4_Diverged(v69) => {
                    let v69: Rc<str> = v69.clone();
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diverged"); } LIT.with(|lit| lit.clone()) };
                    v70.clone()
                }
                US4::US4_InStep => {
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in-step"); } LIT.with(|lit| lit.clone()) };
                    v64.clone()
                }
                US4::US4_LeftAhead(v65) => {
                    let v65: Rc<str> = v65.clone();
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-ahead"); } LIT.with(|lit| lit.clone()) };
                    v66.clone()
                }
                US4::US4_RightAhead(v67) => {
                    let v67: Rc<str> = v67.clone();
                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-ahead"); } LIT.with(|lit| lit.clone()) };
                    v68.clone()
                }
            };
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror check "); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v74));
            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
            let v79: i32 = 0i32;
            let v80: i32 = method24(v56.clone(), v79);
            let v81: Rc<str> = format_real_25(v80);
            let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v81));
            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differing="); } LIT.with(|lit| lit.clone()) };
            let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
            let v85: Rc<str> = format_real_27(v60);
            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v85));
            eprintln!("{}", v86);
            let v87: bool = match &v61 {
                US4::US4_InStep => {
                    false
                }
                _ => {
                    true
                }
            };
            if v87 {
                1i32
            } else {
                0i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        mirror_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_run() -> i32 {
    closure0()()
}
