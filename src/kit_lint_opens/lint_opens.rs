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
enum UH1 {
    UH1_Nil,
    UH1_Cons(i32, Rc<str>, Rc<UH1>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_TypeSpace,
    US1_TermSpace,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, i32, US1, Rc<UH3>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH3>, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US1, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, i32, US1),
    US3_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, i32, US1, Rc<str>, Rc<UH4>),
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: i32 = *v2;
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<UH1>) -> (i32, Rc<UH1>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v5: i32 = v1.wrapping_add(1i32);
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v3.clone(), v2.clone()));
                (v0, v1, v2) = (v4.clone(), v5, v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2.clone());
            }
        }
    }
}
fn numbered_3(mut v0: Rc<UH0>) -> Rc<UH1> {
    let v1: i32 = 0i32;
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v3, mut v4): (i32, Rc<UH1>) = method5(v0, v1, v2);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method4(v4, v5)
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method8(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method8(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0.clone();
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.clone().borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method7(v16.clone(), v17.clone())
    }
}
fn rows_of_2(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_6(v1, v4);
    numbered_3(v5)
}
fn method10(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn directory_get_parent_9(mut v0: Rc<str>) -> Option<Rc<str>> {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method10(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let v21: bool = true; v19 });
    let v23: Option<Rc<str>> = _optionm_map_;
    v23.clone()
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: i32 = *v2;
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":< "); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v23: US0 = if v6 {
                    let mut v7: i32 = (v3.clone().len() as i32);
                    let mut v8: i32 = v7.wrapping_sub(3i32);
                    let mut v9: usize = ((3i32) as usize);
                    let v10: i32 = 3i32.wrapping_add(v8);
                    let v11: usize = ((v10) as usize);
                    let v12: Rc<str> = Rc::<str>::from(&(v3)[v9..v11]);
                    let mut v13: i32 = (v12.clone().len() as i32);
                    let mut v14: i32 = 0i32;
                    let v15: i32 = method13(v12.clone(), v13, v14);
                    let v16: i32 = v13.wrapping_sub(1i32);
                    let v17: Rc<str> = string_slice(&v12.clone(), v15 as i64, v16 as i64);
                    let v18: i32 = (v17.clone().len() as i32);
                    let v19: i32 = method14(v17.clone(), v18);
                    let v20: Rc<str> = string_slice(&v17.clone(), 0i32 as i64, v19 as i64);
                    US0::US0_Some(v20.clone())
                } else {
                    US0::US0_None
                };
                let v27: Rc<UH0> = match &v23 {
                    US0::US0_None => {
                        v1.clone()
                    }
                    US0::US0_Some(v24) => {
                        let v24: Rc<str> = v24.clone();
                        Rc::new(UH0::UH0_Cons(v24.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v4.clone(), v27.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn opened_modules_11(mut v0: Rc<UH1>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method12(v0, v1);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method7(v2, v3)
}
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method10(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
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
fn module_path_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v5: Option<usize> = v3.find(&*v4);
    let mut v6: Option<usize> = method18(v5.clone());
    let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v8: Option<i32> = v6.map(|x| v7(x));
    let v10: i32 = v8.unwrap_or(-1i32);
    let v11: bool = v10 >= 0i32;
    if v11 {
        let v12: i32 = v10.wrapping_add(1i32);
        let mut v13: i32 = (v3.clone().len() as i32);
        let mut v14: i32 = v13.wrapping_sub(v10);
        let v15: i32 = v14.wrapping_sub(1i32);
        let v16: usize = ((v12) as usize);
        let v17: i32 = v12.wrapping_add(v15);
        let v18: usize = ((v17) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v3)[v16..v18]);
        let v20: usize = ((0i32) as usize);
        let v21: usize = ((v10) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v3)[v20..v21]);
        let v23: Rc<str> = method19(v0.clone(), v22.clone());
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(v19.replace(&*v4, &*v24));
        let v26: Rc<str> = method19(v23.clone(), v25.clone());
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        v28.clone()
    } else {
        let v29: Rc<str> = method19(v1.clone(), v3.clone());
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
        let v32: bool = file_exists_20(v31.clone());
        if v32 {
            v31.clone()
        } else {
            let v33: bool = &*v2 != "";
            if v33 {
                let v34: Rc<str> = method19(v2.clone(), v3.clone());
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v30));
                v35.clone()
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36.clone()
            }
        }
    }
}
fn method22(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn module_text_21(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = &*v0 != "";
    let mut v3: bool = if v1 {
        file_exists_20(v0.clone())
    } else {
        false
    };
    if v3 {
        let v5: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v7: Vec<u8> = v5.unwrap();
        let mut v8: Vec<u8> = method22(v7);
        let v10: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v8);
        let v12: std::string::String = v10.unwrap();
        let mut v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        v14
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v15
    }
}
fn method24(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<UH3> = v5.clone();
                let mut v6: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2.clone(), v3, v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v1.clone();
            }
        }
    }
}
fn ident_length_26(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let mut v3: bool = v1 < v2;
        let v19: bool = if v3 {
            let v4: usize = ((v1) as usize);
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: usize = ((v5) as usize);
            let mut v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
            let mut v8: i32 = (v7.clone().len() as i32);
            let mut v9: bool = v8 == 1i32;
            if v9 {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_'"); } LIT.with(|lit| lit.clone()) };
                let v11: Option<usize> = v10.find(&*v7);
                let v12: Option<usize> = method18(v11.clone());
                let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v14: Option<i32> = v12.map(|x| v13(x));
                let v16: i32 = v14.unwrap_or(-1i32);
                let v17: bool = v16 >= 0i32;
                v17
            } else {
                false
            }
        } else {
            false
        };
        if v19 {
            let v20: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v20);
            continue;
        } else {
            return v1;
        }
    }
}
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4) => {
                let mut v2: i32 = *v2;
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":== "); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v60: US2 = if v6 {
                    let mut v7: i32 = (v3.clone().len() as i32);
                    let mut v8: i32 = v7.wrapping_sub(4i32);
                    let mut v9: usize = ((4i32) as usize);
                    let v10: i32 = 4i32.wrapping_add(v8);
                    let v11: usize = ((v10) as usize);
                    let v12: Rc<str> = Rc::<str>::from(&(v3)[v9..v11]);
                    let mut v13: US1 = US1::US1_TypeSpace;
                    US2::US2_Some(v13.clone(), v12.clone())
                } else {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":! "); } LIT.with(|lit| lit.clone()) };
                    let v16: bool = v3.starts_with(&*v15);
                    if v16 {
                        let v17: i32 = (v3.clone().len() as i32);
                        let v18: i32 = v17.wrapping_sub(3i32);
                        let v19: usize = ((3i32) as usize);
                        let v20: i32 = 3i32.wrapping_add(v18);
                        let v21: usize = ((v20) as usize);
                        let v22: Rc<str> = Rc::<str>::from(&(v3)[v19..v21]);
                        let v23: US1 = US1::US1_TypeSpace;
                        US2::US2_Some(v23.clone(), v22.clone())
                    } else {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":| "); } LIT.with(|lit| lit.clone()) };
                        let v26: bool = v3.starts_with(&*v25);
                        if v26 {
                            let v27: i32 = (v3.clone().len() as i32);
                            let v28: i32 = v27.wrapping_sub(3i32);
                            let v29: usize = ((3i32) as usize);
                            let v30: i32 = 3i32.wrapping_add(v28);
                            let v31: usize = ((v30) as usize);
                            let v32: Rc<str> = Rc::<str>::from(&(v3)[v29..v31]);
                            let v33: US1 = US1::US1_TypeSpace;
                            US2::US2_Some(v33.clone(), v32.clone())
                        } else {
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
                            let v36: bool = v3.starts_with(&*v35);
                            if v36 {
                                let v37: i32 = (v3.clone().len() as i32);
                                let v38: i32 = v37.wrapping_sub(3i32);
                                let v39: usize = ((3i32) as usize);
                                let v40: i32 = 3i32.wrapping_add(v38);
                                let v41: usize = ((v40) as usize);
                                let v42: Rc<str> = Rc::<str>::from(&(v3)[v39..v41]);
                                let v43: US1 = US1::US1_TermSpace;
                                US2::US2_Some(v43.clone(), v42.clone())
                            } else {
                                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
                                let v46: bool = v3.starts_with(&*v45);
                                if v46 {
                                    let v47: i32 = (v3.clone().len() as i32);
                                    let v48: i32 = v47.wrapping_sub(3i32);
                                    let v49: usize = ((3i32) as usize);
                                    let v50: i32 = 3i32.wrapping_add(v48);
                                    let v51: usize = ((v50) as usize);
                                    let v52: Rc<str> = Rc::<str>::from(&(v3)[v49..v51]);
                                    let v53: US1 = US1::US1_TermSpace;
                                    US2::US2_Some(v53.clone(), v52.clone())
                                } else {
                                    US2::US2_None
                                }
                            }
                        }
                    }
                };
                let v83: US3 = match &v60 {
                    US2::US2_None => {
                        US3::US3_None
                    }
                    US2::US2_Some(v61, v62) => {
                        let v61: US1 = v61.clone();
                        let v62: Rc<str> = v62.clone();
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^ "); } LIT.with(|lit| lit.clone()) };
                        let v64: bool = v62.starts_with(&*v63);
                        let v71: Rc<str> = if v64 {
                            let v65: i32 = (v62.clone().len() as i32);
                            let v66: i32 = v65.wrapping_sub(2i32);
                            let v67: usize = ((2i32) as usize);
                            let v68: i32 = 2i32.wrapping_add(v66);
                            let v69: usize = ((v68) as usize);
                            let v70: Rc<str> = Rc::<str>::from(&(v62)[v67..v69]);
                            v70.clone()
                        } else {
                            v62.clone()
                        };
                        let v72: i32 = 0i32;
                        let v73: i32 = ident_length_26(v71.clone(), v72);
                        let v74: usize = ((0i32) as usize);
                        let v75: usize = ((v73) as usize);
                        let v76: Rc<str> = Rc::<str>::from(&(v71)[v74..v75]);
                        let v77: bool = &*v76 == "";
                        if v77 {
                            US3::US3_None
                        } else {
                            US3::US3_Some(v76.clone(), v2, v61.clone())
                        }
                    }
                };
                let v89: Rc<UH3> = match &v83 {
                    US3::US3_None => {
                        v1.clone()
                    }
                    US3::US3_Some(v84, v85, v86) => {
                        let v84: Rc<str> = v84.clone();
                        let v85: i32 = *v85;
                        let v86: US1 = v86.clone();
                        Rc::new(UH3::UH3_Cons(v84.clone(), v85, v86.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v4.clone(), v89.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn definitions_23(mut v0: Rc<UH1>) -> Rc<UH3> {
    let v1: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH3> = method25(v0, v1);
    let mut v3: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method24(v2, v3)
}
fn method16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH2>) -> Rc<UH2> {
    match &*v3 {
        UH0::UH0_Cons(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: Rc<UH2> = method16(v0.clone(), v1.clone(), v2.clone(), v6, v4.clone());
            let mut v8: Rc<str> = module_path_17(v0, v1, v2, v5.clone());
            let mut v9: Rc<str> = module_text_21(v8);
            let v10: Rc<UH1> = rows_of_2(v9);
            let v11: Rc<UH3> = definitions_23(v10);
            Rc::new(UH2::UH2_Cons(v5, v11, v7))
        }
        UH0::UH0_Nil => {
            v4
        }
    }
}
fn opened_definitions_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH2> {
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method16(v0, v1, v2, v3, v4)
}
fn method28(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v0 {
            UH4::UH4_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH4> = v6.clone();
                let mut v7: Rc<UH4> = Rc::new(UH4::UH4_Cons(v2.clone(), v3, v4.clone(), v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH4::UH4_Nil => {
                return v1.clone();
            }
        }
    }
}
fn same_binding_34(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<str>, mut v4: i32, mut v5: US1) -> bool {
    let mut v6: bool = &*v0 == &*v3;
    if v6 {
        let mut v8: i32 = match &v2 {
            US1::US1_TermSpace => {
                1i32
            }
            US1::US1_TypeSpace => {
                0i32
            }
        };
        let v10: i32 = match &v5 {
            US1::US1_TermSpace => {
                1i32
            }
            US1::US1_TypeSpace => {
                0i32
            }
        };
        let v11: bool = v8 == v10;
        v11
    } else {
        false
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32, mut v2: US1) -> Rc<dyn Fn(Rc<str>, i32, US1) -> bool> {
    Rc::new(move |mut v3: Rc<str>, mut v4: i32, mut v5: US1| -> bool {
        same_binding_34(v0.clone(), v1, v2.clone(), v3.clone(), v4, v5.clone())
    })
}
fn method33(mut v0: Rc<str>, mut v1: i32, mut v2: US1) -> Rc<dyn Fn(Rc<str>, i32, US1) -> bool> {
    closure2(v0, v1, v2)
}
fn method32(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<dyn Fn(Rc<str>, i32, US1) -> bool>, mut v4: Rc<UH3>) -> bool {
    loop {
        match &*v4 {
            UH3::UH3_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: i32 = *v6;
                let mut v7: US1 = v7.clone();
                let mut v8: Rc<UH3> = v8.clone();
                let mut v9: bool = v3(v5.clone(), v6, v7.clone());
                if v9 {
                    return true;
                } else {
                    let v10: Rc<dyn Fn(Rc<str>, i32, US1) -> bool> = method33(v0.clone(), v1, v2.clone());
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2.clone(), v10.clone(), v8.clone());
                    continue;
                }
            }
            UH3::UH3_Nil => {
                return false;
            }
        }
    }
}
fn defined_in_31(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: i32, mut v3: US1) -> bool {
    let v4: Rc<dyn Fn(Rc<str>, i32, US1) -> bool> = method33(v1.clone(), v2, v3.clone());
    method32(v1, v2, v3, v4, v0)
}
fn method30(mut v0: Rc<str>, mut v1: i32, mut v2: US1, mut v3: Rc<UH2>, mut v4: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v3 {
            UH2::UH2_Cons(v5, v6, v7) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH3> = v6.clone();
                let mut v7: Rc<UH2> = v7.clone();
                let mut v8: bool = defined_in_31(v6.clone(), v0.clone(), v1, v2.clone());
                let v10: Rc<UH4> = if v8 {
                    Rc::new(UH4::UH4_Cons(v0.clone(), v1, v2.clone(), v5.clone(), v4.clone()))
                } else {
                    v4.clone()
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1, v2.clone(), v7.clone(), v10.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v4.clone();
            }
        }
    }
}
fn method29(mut v0: Rc<UH2>, mut v1: Rc<UH3>, mut v2: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v1 {
            UH3::UH3_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: i32 = *v4;
                let v5: US1 = v5.clone();
                let mut v6: Rc<UH3> = v6.clone();
                let mut v7: bool = &*v3 == "main";
                let mut v9: Rc<UH4> = if v7 {
                    v2.clone()
                } else {
                    method30(v3.clone(), v4, v5.clone(), v0.clone(), v2.clone())
                };
                (v0, v1, v2) = (v0.clone(), v6.clone(), v9.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v2.clone();
            }
        }
    }
}
fn open_clashes_27(mut v0: Rc<UH3>, mut v1: Rc<UH2>) -> Rc<UH4> {
    let mut v2: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH4> = method29(v1, v0, v2);
    let v4: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    method28(v3, v4)
}
fn closure3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v2: Rc<str>| -> bool {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
        let v5: bool = v2.starts_with(&*v4);
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  open-shadow"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let v10: bool = v2.contains(&*v9);
            v10
        } else {
            false
        }
    })
}
fn method38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure3(v0, v1)
}
fn method37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH0>) -> bool {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: bool = v2(v4.clone());
                if v6 {
                    return true;
                } else {
                    let mut v7: Rc<dyn Fn(Rc<str>) -> bool> = method38(v0.clone(), v1.clone());
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v7.clone(), v5.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn known_open_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> bool {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH0> = split_list_6(v3, v0);
    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method38(v1.clone(), v2.clone());
    method37(v1, v2, v5, v4)
}
fn method35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH4>, mut v3: Rc<UH4>) -> Rc<UH4> {
    match &*v2 {
        UH4::UH4_Cons(v4, v5, v6, v7, v8) => {
            let v4: Rc<str> = v4.clone();
            let v5: i32 = *v5;
            let mut v6: US1 = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<UH4> = v8.clone();
            let mut v9: Rc<UH4> = method35(v0.clone(), v1.clone(), v8, v3.clone());
            let v10: bool = known_open_36(v1, v0, v4.clone());
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH4::UH4_Cons(v4, v5, v6, v7, v9.clone()))
            } else {
                v9
            }
        }
        UH4::UH4_Nil => {
            v3
        }
    }
}
fn open_row_40(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: US1, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, Rc::<str>::from(format!("{:?}", v2))));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v1));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  open-shadow  from "); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v4));
    v14
}
fn method39(mut v0: Rc<str>, mut v1: Rc<UH4>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH4::UH4_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: US1 = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let mut v7: Rc<UH4> = v7.clone();
            let mut v8: Rc<UH0> = method39(v0.clone(), v7, v2.clone());
            let mut v9: Rc<str> = open_row_40(v0, v3, v4, v5, v6);
            Rc::new(UH0::UH0_Cons(v9, v8))
        }
        UH4::UH4_Nil => {
            v2
        }
    }
}
fn method41(mut v0: Rc<UH4>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH4::UH4_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: i32 = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH4> = v6.clone();
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6.clone(), v7);
                continue;
            }
            UH4::UH4_Nil => {
                return v1;
            }
        }
    }
}
fn open_rows_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, i32) {
    let mut v6: Rc<UH1> = rows_of_2(v4);
    let mut v7: Option<Rc<str>> = directory_get_parent_9(v2);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = v9;
    let v11: Rc<str> = v7.unwrap_or(v10);
    let v12: Rc<UH0> = opened_modules_11(v6.clone());
    let mut v13: Rc<UH2> = opened_definitions_15(v0, v11, v1, v12);
    let mut v14: Rc<UH3> = definitions_23(v6);
    let v15: Rc<UH4> = open_clashes_27(v14, v13);
    let v16: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH4> = method35(v3.clone(), v5, v15.clone(), v16);
    let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH0> = method39(v3, v17.clone(), v18);
    let v20: i32 = 0i32;
    let v21: i32 = method41(v15, v20);
    let v22: i32 = 0i32;
    let v23: i32 = method41(v17, v22);
    (v19, v21, v23)
}
fn method43(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method44(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.clone().borrow_mut()[v2 as usize] = v3.clone();
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn rows_array_42(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: i32 = 0i32;
    let mut v2: i32 = method43(v0.clone(), v1);
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v2 as usize]));
    let v4: i32 = 0i32;
    let v5: i32 = method44(v3.clone(), v0, v4);
    v3
}
fn open_rows_exported_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, i32, i32) {
    let (mut v6, mut v7, mut v8): (Rc<UH0>, i32, i32) = open_rows_1(v0, v1, v2, v3, v4, v5);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = rows_array_42(v6);
    (v9, v7, v8)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>| -> (Rc<RefCell<Vec<Rc<str>>>>, i32, i32) {
        open_rows_exported_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_open_rows(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str) -> (Rc<RefCell<Vec<Rc<str>>>>, i32, i32) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5))
}
