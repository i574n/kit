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
pub struct Mut0 { pub l0: i32 }
pub struct Mut1 { pub l0: i32 }
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Missing,
    US1_Present(Rc<str>, u64),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US1, US1, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Same,
    US2_LeftOnly,
    US2_RightOnly,
    US2_LeftNewer,
    US2_RightNewer,
}
#[derive(Clone)]
enum US3 {
    US3_InStep,
    US3_LeftAhead(Rc<str>),
    US3_RightAhead(Rc<str>),
    US3_Diverged(Rc<str>),
}
pub struct Mut2 { pub l0: Rc<str> }
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
fn directory_exists_2(mut v0: Rc<str>) -> bool {
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
fn method6(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method9(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut1>>) -> bool {
    let v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
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
fn sorted_names_5(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Vec<std::string::String> = method6(v4);
    let mut v6: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v5));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
    let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method7(v7, &v9) {
        let v11: i32 = v9.borrow().l0;
        let v12: std::string::String = v6.borrow()[v11 as usize].clone();
        let v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        v8.borrow_mut()[v11 as usize] = v14;
        let v15: i32 = v11.wrapping_add(1i32);
        v9.borrow_mut().l0 = v15;
        ()
    };
    let v16: Rc<RefCell<Vec<Rc<str>>>> = v8 ;
    let mut v17: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method8();
    let mut v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v16.borrow().clone()));
    let mut v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v16.borrow().iter().map(|x| v17(x.clone())).collect::<Vec<_>>()));
    let v20: i32 = (v16.borrow().len() as i32);
    let mut v21: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
    while method7(v20, &v21) {
        let v23: i32 = v21.borrow().l0;
        let v24: Rc<str> = v18.borrow()[v23 as usize].clone();
        let v25: Rc<str> = v19.borrow()[v23 as usize].clone();
        let mut v26: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v23 }));
        while method9(&v19, v25.clone(), &v26) {
            let v28: i32 = v26.borrow().l0;
            let v29: i32 = v28.wrapping_sub(1i32);
            let v30: Rc<str> = v18.borrow()[v29 as usize].clone();
            v18.borrow_mut()[v28 as usize] = v30;
            let v31: Rc<str> = v19.borrow()[v29 as usize].clone();
            v19.borrow_mut()[v28 as usize] = v31;
            v26.borrow_mut().l0 = v29;
            ()
        };
        let v32: i32 = v26.borrow().l0;
        v18.borrow_mut()[v32 as usize] = v24;
        let v33: i32 = v26.borrow().l0;
        v19.borrow_mut()[v33 as usize] = v25;
        let v34: i32 = v23.wrapping_add(1i32);
        v21.borrow_mut().l0 = v34;
        ()
    };
    let v35: Rc<RefCell<Vec<Rc<str>>>> = v18 ;
    v35
}
fn children_pushed_10(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v7: bool = &*v0 == "";
            let v10: Rc<str> = if v7 {
                v6
            } else {
                let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v6));
                v9
            };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v3));
            (v2, v3) = (v5, v11);
            continue;
        }
    }
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
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
fn files_under_4(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == "";
                let mut v8: Rc<str> = if v5 {
                    v0.clone()
                } else {
                    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3.clone()));
                    v7
                };
                let mut v9: bool = directory_exists_2(v8.clone());
                if v9 {
                    let v10: Rc<RefCell<Vec<Rc<str>>>> = sorted_names_5(v8);
                    let v11: i32 = (v10.clone().borrow().len() as i32);
                    let v12: i32 = v11.wrapping_sub(1i32);
                    let v13: Rc<UH0> = children_pushed_10(v3, &v10, v12, v4);
                    v1 = v13;
                    continue;
                } else {
                    let v15: bool = file_exists_11(v8);
                    if v15 {
                        let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v2));
                        (v1, v2) = (v4, v16);
                        continue;
                    } else {
                        v1 = v4;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    Rc::new(move |mut v2: Rc<UH0>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US0> = method14(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method14(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US0> {
    closure2(v0, v1)
}
fn merged_13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = 0i32;
                let mut v5: Rc<dyn Fn(Rc<UH0>) -> US0> = method14(v2.clone(), v4);
                let mut v6: US0 = v5(v1.clone());
                let mut v7: bool = match v6 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v8: bool = v7 != true;
                let v10: Rc<UH0> = if v8 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn paths_from_3(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v6));
                let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v9: Rc<UH0> = files_under_4(v1.clone(), v7, v8);
                let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v11: Rc<UH0> = method12(v9, v10);
                let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v12));
                let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v15: Rc<UH0> = files_under_4(v2.clone(), v13, v14);
                let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v17: Rc<UH0> = method12(v15, v16);
                let mut v18: Rc<UH0> = merged_13(v11, v3);
                let mut v19: Rc<UH0> = merged_13(v17, v18);
                (v0, v3) = (v5, v19);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: US1 = v3;
                let mut v4: US1 = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
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
fn mirrored_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6;
                let mut v7: Rc<UH0> = v7;
                let mut v8: bool = &*v6 == "";
                let v11: Rc<str> = if v8 {
                    v0.clone()
                } else {
                    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v6.clone()));
                    v10
                };
                let v12: bool = file_exists_11(v11.clone());
                let v27: US1 = if v12 {
                    let v14: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v11);
                    let v16: Vec<u8> = v14.unwrap();
                    let mut v17: Vec<u8> = method17(v16);
                    let mut v19: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v17);
                    let mut v21: std::string::String = v19.unwrap();
                    let v23: Rc<str> = Rc::<str>::from(String::as_str(&v21));
                    let v24: u64 = last_write_ms_18(v11);
                    US1::US1_Present(v23, v24)
                } else {
                    US1::US1_Missing
                };
                let v30: Rc<str> = if v8 {
                    v1.clone()
                } else {
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v6.clone()));
                    v29
                };
                let v31: bool = file_exists_11(v30.clone());
                let v46: US1 = if v31 {
                    let v33: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v30);
                    let v35: Vec<u8> = v33.unwrap();
                    let v36: Vec<u8> = method17(v35);
                    let v38: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v36);
                    let v40: std::string::String = v38.unwrap();
                    let v42: Rc<str> = Rc::<str>::from(String::as_str(&v40));
                    let v43: u64 = last_write_ms_18(v30);
                    US1::US1_Present(v42, v43)
                } else {
                    US1::US1_Missing
                };
                let v47: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v27, v46, v3));
                (v2, v3) = (v7, v47);
                continue;
            }
            UH0::UH0_Nil => {
                let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method16(v3, v4);
            }
        }
    }
}
fn drift_of_20(mut v0: &Rc<str>, mut v1: US1, mut v2: US1) -> US2 {
    match v1 {
        US1::US1_Missing => {
            match v2 {
                US1::US1_Missing => {
                    US2::US2_Same
                }
                US1::US1_Present(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: u64 = v5;
                    US2::US2_RightOnly
                }
            }
        }
        US1::US1_Present(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: u64 = v10;
            match v2 {
                US1::US1_Missing => {
                    US2::US2_LeftOnly
                }
                US1::US1_Present(v12, v13) => {
                    let v12: Rc<str> = v12;
                    let v13: u64 = v13;
                    let v14: bool = &*v9 == &*v12;
                    if v14 {
                        US2::US2_Same
                    } else {
                        let v16: bool = v10 > v13;
                        if v16 {
                            US2::US2_LeftNewer
                        } else {
                            US2::US2_RightNewer
                        }
                    }
                }
            }
        }
    }
}
fn drift_word_21(mut v0: US2) -> Rc<str> {
    match v0 {
        US2::US2_LeftNewer => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-newer"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_LeftOnly => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-only"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_RightNewer => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-newer"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_RightOnly => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-only"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_Same => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("same"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn drift_lines_19(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: u64) -> (Rc<str>, u64) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let mut v4: US1 = v4;
                let mut v5: US1 = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: US2 = drift_of_20(&v3, v4, v5);
                match &v7 {
                    US2::US2_Same => {
                        v0 = v6;
                        continue;
                    }
                    _ => {
                        let v10: Rc<str> = drift_word_21(v7);
                        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v10));
                        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v3));
                        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v15: u64 = v2.wrapping_add(1u64);
                        (v0, v1, v2) = (v6, v14, v15);
                        continue;
                    }
                }
            }
            UH1::UH1_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn mirror_state_of_22(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: Rc<str>) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v16, v17, v18, v19) => {
                let v16: Rc<str> = v16;
                let mut v17: US1 = v17;
                let mut v18: US1 = v18;
                let mut v19: Rc<UH1> = v19;
                let v20: US2 = drift_of_20(&v16, v17, v18);
                match &v20 {
                    US2::US2_Same => {
                        v0 = v19;
                        continue;
                    }
                    _ => {
                        let v23: bool = match v20 {
                            US2::US2_LeftNewer => {
                                true
                            }
                            US2::US2_LeftOnly => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v23 {
                            let v24: bool = &*v1 == "";
                            let v27: Rc<str> = if v24 {
                                v16
                            } else {
                                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
                                let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v16));
                                v26
                            };
                            (v0, v1) = (v19, v27);
                            continue;
                        } else {
                            let v29: bool = &*v2 == "";
                            let v32: Rc<str> = if v29 {
                                v16
                            } else {
                                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
                                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v16));
                                v31
                            };
                            (v0, v2) = (v19, v32);
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
                    return US3::US3_InStep;
                } else {
                    let mut v7: bool = &*v2 == "";
                    if v7 {
                        return US3::US3_LeftAhead(v1);
                    } else {
                        if v3 {
                            return US3::US3_RightAhead(v2);
                        } else {
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) }));
                            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
                            return US3::US3_Diverged(v11);
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
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn method24(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: US1 = v3;
                let mut v4: US1 = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method26(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_25(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn mirror_checked_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> i32 {
    let v3: bool = match &*v2 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror usage: mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v4);
        2i32
    } else {
        let mut v5: bool = directory_exists_2(v0.clone());
        let mut v6: bool = v5 == false;
        let mut v9: bool = if v6 {
            true
        } else {
            let mut v7: bool = directory_exists_2(v1.clone());
            let mut v8: bool = v7 == false;
            v8
        };
        if v9 {
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror check needs two existing roots, received "); } LIT.with(|lit| lit.clone()) }, v0));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" and "); } LIT.with(|lit| lit.clone()) }));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v1));
            eprintln!("{}", v12);
            2i32
        } else {
            let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH0> = paths_from_3(v2, v0.clone(), v1.clone(), v13);
            let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v16: Rc<UH0> = method12(v14, v15);
            let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let mut v18: Rc<UH1> = mirrored_15(v0, v1, v16, v17);
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v20: u64 = 0u64;
            let (mut v21, mut v22): (Rc<str>, u64) = drift_lines_19(v18.clone(), v19.clone(), v20);
            let v23: US3 = mirror_state_of_22(v18.clone(), v19.clone(), v19);
            let v24: bool = &*v21 != "";
            if v24 {
                let v25: Rc<str> = loop_23(v21);
                println!("{}", v25);
                ()
            };
            let v36: Rc<str> = match &v23 {
                US3::US3_Diverged(v31) => {
                    let v31: Rc<str> = v31.clone();
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diverged"); } LIT.with(|lit| lit.clone()) };
                    v32
                }
                US3::US3_InStep => {
                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in-step"); } LIT.with(|lit| lit.clone()) };
                    v26
                }
                US3::US3_LeftAhead(v27) => {
                    let v27: Rc<str> = v27.clone();
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-ahead"); } LIT.with(|lit| lit.clone()) };
                    v28
                }
                US3::US3_RightAhead(v29) => {
                    let v29: Rc<str> = v29.clone();
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("right-ahead"); } LIT.with(|lit| lit.clone()) };
                    v30
                }
            };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror check "); } LIT.with(|lit| lit.clone()) }, v36));
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) }));
            let v39: i32 = 0i32;
            let v40: i32 = method24(v18, v39);
            let v41: Rc<str> = format_real_25(v40);
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v41));
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differing="); } LIT.with(|lit| lit.clone()) }));
            let v44: Rc<str> = format_real_27(v22);
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
            eprintln!("{}", v45);
            let v46: bool = match v23 {
                US3::US3_InStep => {
                    false
                }
                _ => {
                    true
                }
            };
            if v46 {
                1i32
            } else {
                0i32
            }
        }
    }
}
fn command_line_argument_count_29() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_30(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_28(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_29();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US4 = if v4 {
        let mut v5: Rc<str> = command_line_argument_30(v0);
        US4::US4_Some(v5)
    } else {
        US4::US4_None
    };
    match v8 {
        US4::US4_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US4::US4_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_28(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn is_check_31(mut v0: Rc<str>) -> bool {
    let mut v1: bool = &*v0 == "check";
    v1
}
fn mirror_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_28(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let mut v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let mut v8: Rc<str> = v8;
                                    let mut v9: Rc<UH0> = v9;
                                    let v10: bool = is_check_31(v4.clone());
                                    if v10 {
                                        mirror_checked_1(v6, v8, v9)
                                    } else {
                                        let v12: bool = is_check_31(v4);
                                        let v13: bool = v12 == false;
                                        if v13 {
                                            let v14: i32 = kit_mirror_remote::kit_mirror_remote_run();
                                            v14
                                        } else {
                                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            mirror_checked_1(v15.clone(), v15, v16)
                                        }
                                    }
                                }
                                _ => {
                                    let v20: bool = is_check_31(v4);
                                    let mut v21: bool = v20 == false;
                                    if v21 {
                                        let v22: i32 = kit_mirror_remote::kit_mirror_remote_run();
                                        v22
                                    } else {
                                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                        mirror_checked_1(v23.clone(), v23, v24)
                                    }
                                }
                            }
                        }
                        _ => {
                            let v28: bool = is_check_31(v4);
                            let v29: bool = v28 == false;
                            if v29 {
                                let v30: i32 = kit_mirror_remote::kit_mirror_remote_run();
                                v30
                            } else {
                                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                mirror_checked_1(v31.clone(), v31, v32)
                            }
                        }
                    }
                }
                _ => {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    mirror_checked_1(v36.clone(), v36, v37)
                }
            }
        }
        _ => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            mirror_checked_1(v40.clone(), v40, v41)
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
pub type SpiralShape_2617b4e49837 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut2;
