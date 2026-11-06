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
enum US0 {
    US0_Ok(Vec<std::string::String>),
    US0_Error(std::io::Error),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
fn method1(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let mut v10: bool = if v4 {
                    true
                } else {
                    let mut v5: bool = &*v2 < &*v1;
                    let mut v8: i32 = if v5 {
                        -1i32
                    } else {
                        let mut v6: bool = &*v2 > &*v1;
                        if v6 {
                            1i32
                        } else {
                            0i32
                        }
                    };
                    let mut v9: bool = v8 == -1i32;
                    v9
                };
                let v11: Rc<str> = if v10 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US0> = Rc::new(move |mut v0: Vec<std::string::String>| -> US0 {
        US0::US0_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method2() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method5(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method5(v0.clone(), v4, v2);
            let mut v6: bool = v3.ends_with(&*v0);
            if v6 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn earliest_ending_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v1).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method2();
    let mut v5: Rc<dyn Fn(std::io::Error) -> US0> = method3();
    let mut v6: US0 = match v3 { Ok(x) => v4(x), Err(e) => v5(e) };
    let v27: Rc<UH0> = match v6 {
        US0::US0_Error(v24) => {
            let v24: std::io::Error = v24;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Ok(v7) => {
            let mut v7: Vec<std::string::String> = v7;
            let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
            let v12: Rc<UH0> = acc;
            let mut v14: std::string::String = x;
            let mut v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
            let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
            let v19: bool = true; v18 });
            let v21: Rc<UH0> = _vec_fold_;
            let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method4(v21, v22)
        }
    };
    let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v29: Rc<UH0> = method5(v0, v27, v28);
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method1(v29, v30)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        earliest_ending_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16, v17)
    }
}
fn method10(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v1 == "";
                let v43: bool = if v4 {
                    true
                } else {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v6: Rc<UH0> = split_list_8(v5.clone(), v2.clone());
                    let v23: u64 = match Rc::unwrap_or_clone(v6) {
                        UH0::UH0_Cons(v7, v8) => {
                            let mut v7: Rc<str> = v7;
                            let mut v8: Rc<UH0> = v8;
                            match Rc::unwrap_or_clone(v8) {
                                UH0::UH0_Cons(v9, v10) => {
                                    let mut v9: Rc<str> = v9;
                                    let mut v10: Rc<UH0> = v10;
                                    let v11: Option<u64> = (v9).trim().parse::<u64>().ok();
                                    let v12: Option<u64> = method10(v11);
                                    let mut v13: Rc<dyn Fn((u64)) -> US1> = closure4();
                                    let mut v14: Option<US1> = v12.map(|x| v13(x));
                                    let mut v16: US1 = US1::US1_None;
                                    let mut v17: US1 = v16.clone();
                                    let v18: US1 = v14.unwrap_or(v17);
                                    match v18 {
                                        US1::US1_None => {
                                            0u64
                                        }
                                        US1::US1_Some(v19) => {
                                            let v19: u64 = v19;
                                            v19
                                        }
                                    }
                                }
                                _ => {
                                    0u64
                                }
                            }
                        }
                        _ => {
                            0u64
                        }
                    };
                    let v24: Rc<UH0> = split_list_8(v5, v1.clone());
                    let v41: u64 = match Rc::unwrap_or_clone(v24) {
                        UH0::UH0_Cons(v25, v26) => {
                            let v25: Rc<str> = v25;
                            let v26: Rc<UH0> = v26;
                            match Rc::unwrap_or_clone(v26) {
                                UH0::UH0_Cons(v27, v28) => {
                                    let v27: Rc<str> = v27;
                                    let v28: Rc<UH0> = v28;
                                    let v29: Option<u64> = (v27).trim().parse::<u64>().ok();
                                    let v30: Option<u64> = method10(v29);
                                    let mut v31: Rc<dyn Fn((u64)) -> US1> = closure4();
                                    let mut v32: Option<US1> = v30.map(|x| v31(x));
                                    let v34: US1 = US1::US1_None;
                                    let v35: US1 = v34.clone();
                                    let v36: US1 = v32.unwrap_or(v35);
                                    match v36 {
                                        US1::US1_None => {
                                            0u64
                                        }
                                        US1::US1_Some(v37) => {
                                            let v37: u64 = v37;
                                            v37
                                        }
                                    }
                                }
                                _ => {
                                    0u64
                                }
                            }
                        }
                        _ => {
                            0u64
                        }
                    };
                    let v42: bool = v23 > v41;
                    v42
                };
                let v44: Rc<str> = if v43 {
                    v2
                } else {
                    v1
                };
                (v0, v1) = (v3, v44);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn jobs_root_11() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn joined_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn text_or_empty_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
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
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method12(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = joined_13(v1, v4.clone());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = joined_13(v7, v8);
            let mut v10: Rc<str> = text_or_empty_14(v9);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = 0i32;
            let mut v13: i32 = method15(v10.clone(), v11, v12);
            let mut v14: i32 = v11.wrapping_sub(1i32);
            let v15: Rc<str> = string_slice(&v10, v13 as i64, v14 as i64);
            let mut v16: i32 = (v15.clone().len() as i32);
            let mut v17: i32 = method16(v15.clone(), v16);
            let v18: Rc<str> = string_slice(&v15, 0i32 as i64, v17 as i64);
            let v19: bool = &*v18 == &*v0;
            if v19 {
                Rc::new(UH0::UH0_Cons(v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn newest_labeled_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_11();
    let mut v2: bool = &*v0 == "";
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v1).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v6: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method2();
        let mut v7: Rc<dyn Fn(std::io::Error) -> US0> = method3();
        let mut v8: US0 = match v5 { Ok(x) => v6(x), Err(e) => v7(e) };
        let v29: Rc<UH0> = match v8 {
            US0::US0_Error(v26) => {
                let v26: std::io::Error = v26;
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            }
            US0::US0_Ok(v9) => {
                let mut v9: Vec<std::string::String> = v9;
                let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v12: bool = true; let _vec_fold_ = v9.into_iter().fold(v10, |acc, x| { //;
                let mut v14: Rc<UH0> = acc;
                let mut v16: std::string::String = x;
                let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
                let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v14));
                let v21: bool = true; v20 });
                let v23: Rc<UH0> = _vec_fold_;
                let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                method4(v23, v24)
            }
        };
        let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v31: Rc<UH0> = method12(v0, v1, v29, v30);
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        method7(v31, v32)
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        newest_labeled_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method18(v3, v1);
            let mut v5: bool = &*v2 != "";
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
fn file_exists_19(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn finished_c6(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/exit"); } LIT.with(|lit| lit.clone()) }));
        file_exists_19(v4)
    })
}
fn method22(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: bool) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v17: bool = if v3 {
                    true
                } else {
                    let mut v6: Rc<str> = Rc::<str>::from(v4.to_lowercase());
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v8));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
                    let mut v13: bool = v12.contains(&*v1);
                    if v13 {
                        let mut v14: bool = v0(v4);
                        let v15: bool = v14 == false;
                        v15
                    } else {
                        false
                    }
                };
                (v2, v3) = (v5, v17);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn method21(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method21(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = Rc::<str>::from(v4.to_lowercase());
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
            let mut v10: bool = false;
            let v11: bool = method22(v0, v9, v1, v10);
            if v11 {
                Rc::new(UH0::UH0_Cons(v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method23(mut v0: u64, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method23(v0, v4, v2);
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v3.starts_with(&*v6);
            let v29: bool = if v7 {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<UH0> = split_list_8(v8, v3.clone());
                let v26: u64 = match Rc::unwrap_or_clone(v9) {
                    UH0::UH0_Cons(v10, v11) => {
                        let mut v10: Rc<str> = v10;
                        let v11: Rc<UH0> = v11;
                        match Rc::unwrap_or_clone(v11) {
                            UH0::UH0_Cons(v12, v13) => {
                                let v12: Rc<str> = v12;
                                let mut v13: Rc<UH0> = v13;
                                let mut v14: Option<u64> = (v12).trim().parse::<u64>().ok();
                                let v15: Option<u64> = method10(v14);
                                let mut v16: Rc<dyn Fn((u64)) -> US1> = closure4();
                                let mut v17: Option<US1> = v15.map(|x| v16(x));
                                let v19: US1 = US1::US1_None;
                                let v20: US1 = v19.clone();
                                let v21: US1 = v17.unwrap_or(v20);
                                match v21 {
                                    US1::US1_None => {
                                        0u64
                                    }
                                    US1::US1_Some(v22) => {
                                        let v22: u64 = v22;
                                        v22
                                    }
                                }
                            }
                            _ => {
                                0u64
                            }
                        }
                    }
                    _ => {
                        0u64
                    }
                };
                let v27: u64 = v26.wrapping_add(43200000u64);
                let v28: bool = v0 < v27;
                v28
            } else {
                false
            };
            if v29 {
                Rc::new(UH0::UH0_Cons(v3, v5))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn live_of_names_20(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: u64, mut v3: Rc<dyn Fn(Rc<str>) -> bool>) -> Rc<UH0> {
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = method23(v2, v0, v4);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method21(v3, v5, v1, v6)
}
fn method24(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn live_items_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_11();
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v6: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v7: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method2();
    let mut v8: Rc<dyn Fn(std::io::Error) -> US0> = method3();
    let mut v9: US0 = match v6 { Ok(x) => v7(x), Err(e) => v8(e) };
    let v30: Rc<UH0> = match v9 {
        US0::US0_Error(v27) => {
            let v27: std::io::Error = v27;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Ok(v10) => {
            let mut v10: Vec<std::string::String> = v10;
            let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v13: bool = true; let _vec_fold_ = v10.into_iter().fold(v11, |acc, x| { //;
            let v15: Rc<UH0> = acc;
            let mut v17: std::string::String = x;
            let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
            let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v15));
            let v22: bool = true; v21 });
            let v24: Rc<UH0> = _vec_fold_;
            let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method4(v24, v25)
        }
    };
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<UH0> = split_list_8(v31.clone(), v0);
    let v33: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v34: Rc<UH0> = method18(v32, v33);
    let v35: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v36: Rc<dyn Fn(Rc<str>) -> bool> = finished_c6(v4);
    let v37: Rc<UH0> = live_of_names_20(v30, v34, v35, v36);
    let v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v39: i32 = 0i32;
    let v40: i32 = method24(v38.clone(), v37, v39);
    let v41: Rc<Vec<Rc<str>>> = Rc::new(v38.borrow().clone());
    let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v41).as_ref().clone()));
    let v43: Rc<str> = Rc::<str>::from((*v42.borrow()).join(&*v31));
    v43
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        live_items_17(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_earliest_ending(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_newest_labeled(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_job_live_items(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
