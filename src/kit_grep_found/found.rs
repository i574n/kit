#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
enum US0 {
    US0_Ok(Vec<std::string::String>),
    US0_Error(std::io::Error),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut0 { pub l0: i32 }
#[derive(Clone)]
enum US2 {
    US2_Ok(regex::Regex),
    US2_Error(regex::Error),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>),
    US3_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(i32, Rc<UH1>),
}
pub struct Mut1 { pub l0: i32 }
fn directory_exists_1(mut v0: Rc<str>) -> bool {
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
fn root_form_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: bool = v4 > 1i32;
    let mut v7: bool = if v5 {
        let mut v6: bool = v3.ends_with(&*v2);
        v6
    } else {
        false
    };
    if v7 {
        let mut v8: i32 = v4.wrapping_sub(1i32);
        let mut v9: usize = ((0i32) as usize);
        let v10: usize = ((v8) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v3)[v9..v10]);
        v11
    } else {
        v3
    }
}
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US0> = Rc::new(move |mut v0: Vec<std::string::String>| -> US0 {
        US0::US0_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method4() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method5() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn closure3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v2: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US1::US1_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method9(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure3(v0, v1)
}
fn file_exists_10(mut v0: Rc<str>) -> bool {
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
fn entry_step_8(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<UH0>, Rc<UH0>) {
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3.clone()));
    let mut v6: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v5);
    let mut v7: bool = v6.is_ok();
    let mut v8: bool = v7 == false;
    let mut v11: bool = if v8 {
        false
    } else {
        let mut v9: std::fs::Metadata = v6.unwrap();
        let v10: bool = v9.file_type().is_symlink();
        v10
    };
    if v11 {
        (v1, v2)
    } else {
        let mut v12: bool = directory_exists_1(v5.clone());
        if v12 {
            let mut v13: i32 = 0i32;
            let mut v14: Rc<dyn Fn(Rc<UH0>) -> US1> = method9(v3, v13);
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dist"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("obj"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v23));
            let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v24));
            let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v25));
            let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v26));
            let v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v27));
            let v29: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v28));
            let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v29));
            let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v30));
            let v32: US1 = v14(v31);
            let v33: bool = match v32 {
                US1::US1_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v34: bool = v33 != true;
            let v36: Rc<UH0> = if v34 {
                v1
            } else {
                Rc::new(UH0::UH0_Cons(v5, v1))
            };
            (v36, v2)
        } else {
            let v37: bool = file_exists_10(v5.clone());
            if v37 {
                let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
                (v1, v38)
            } else {
                (v1, v2)
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> (Rc<UH0>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let (mut v6, mut v7): (Rc<UH0>, Rc<UH0>) = entry_step_8(v0.clone(), v2, v3, v4);
                (v1, v2, v3) = (v5, v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn files_walked_3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v2).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                let mut v6: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method4();
                let mut v7: Rc<dyn Fn(std::io::Error) -> US0> = method5();
                let mut v8: US0 = match v5 { Ok(x) => v6(x), Err(e) => v7(e) };
                let v29: Rc<UH0> = match v8 {
                    US0::US0_Error(v26) => {
                        let v26: std::io::Error = v26;
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                    }
                    US0::US0_Ok(v9) => {
                        let mut v9: Vec<std::string::String> = v9;
                        let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let mut v12: bool = true; let _vec_fold_ = v9.into_iter().fold(v10, |acc, x| { //;
                        let mut v14: Rc<UH0> = acc;
                        let mut v16: std::string::String = x;
                        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
                        let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v14));
                        let v21: bool = true; v20 });
                        let v23: Rc<UH0> = _vec_fold_;
                        let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        method6(v23, v24)
                    }
                };
                let (mut v30, mut v31): (Rc<UH0>, Rc<UH0>) = method7(v2, v29, v3, v1);
                (v0, v1) = (v30, v31);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure4() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn base_name_12(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Option<usize> = v3.rfind(&*v2);
    let mut v5: Option<usize> = method13(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure4();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let mut v9: i32 = v7.unwrap_or(-1i32);
    let v10: bool = v9 < 0i32;
    if v10 {
        v3
    } else {
        let mut v11: i32 = v9.wrapping_add(1i32);
        let mut v12: i32 = (v3.clone().len() as i32);
        let mut v13: i32 = v12.wrapping_sub(v9);
        let mut v14: i32 = v13.wrapping_sub(1i32);
        let mut v15: usize = ((v11) as usize);
        let mut v16: i32 = v11.wrapping_add(v14);
        let mut v17: usize = ((v16) as usize);
        let v18: Rc<str> = Rc::<str>::from(&(v3)[v15..v17]);
        v18
    }
}
fn closure5() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn glob_wanted_14(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Option<usize> = v0.find(&*v2);
    let mut v4: Option<usize> = method13(v3);
    let mut v5: Rc<dyn Fn((usize)) -> i32> = closure5();
    let mut v6: Option<i32> = v4.map(|x| v5(x));
    let mut v8: i32 = v6.unwrap_or(-1i32);
    let mut v9: i32 = (v0.clone().len() as i32);
    let v10: bool = &*v0 == "";
    if v10 {
        true
    } else {
        let mut v11: bool = v8 < 0i32;
        if v11 {
            let mut v12: bool = &*v1 == &*v0;
            v12
        } else {
            let mut v13: usize = ((0i32) as usize);
            let mut v14: usize = ((v8) as usize);
            let mut v15: Rc<str> = Rc::<str>::from(&(v0)[v13..v14]);
            let mut v16: i32 = v8.wrapping_add(1i32);
            let mut v17: i32 = v9.wrapping_sub(v8);
            let v18: i32 = v17.wrapping_sub(1i32);
            let v19: usize = ((v16) as usize);
            let v20: i32 = v16.wrapping_add(v18);
            let v21: usize = ((v20) as usize);
            let mut v22: Rc<str> = Rc::<str>::from(&(v0)[v19..v21]);
            let v23: bool = v1.starts_with(&*v15);
            if v23 {
                let v24: bool = v1.ends_with(&*v22);
                v24
            } else {
                false
            }
        }
    }
}
fn method15(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = base_name_12(v3.clone());
                let mut v6: bool = glob_wanted_14(v0.clone(), v5);
                let v21: Rc<RefCell<Vec<Rc<str>>>> = if v6 {
                    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v3]));
                    let mut v8: i32 = (v2.clone().borrow().len() as i32);
                    let mut v9: i32 = (v7.clone().borrow().len() as i32);
                    let v10: i32 = v8.wrapping_add(v9);
                    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v10 as usize]));
                    let mut v12: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                    while method15(v10, &v12) {
                        let mut v14: i32 = v12.borrow().l0;
                        let mut v15: bool = v14 < v8;
                        let v19: Rc<str> = if v15 {
                            let mut v16: Rc<str> = v2.borrow()[v14 as usize].clone();
                            v16
                        } else {
                            let mut v17: i32 = v14.wrapping_sub(v8);
                            let v18: Rc<str> = v7.borrow()[v17 as usize].clone();
                            v18
                        };
                        v11.borrow_mut()[v14 as usize] = v19;
                        let v20: i32 = v14.wrapping_add(1i32);
                        v12.borrow_mut().l0 = v20;
                        ()
                    };
                    v11
                } else {
                    v2
                };
                (v1, v2) = (v4, v21);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn found_files_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: bool = directory_exists_1(v1.clone());
    let v10: Rc<UH0> = if v2 {
        let mut v3: Rc<str> = root_form_2(v1);
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4));
        let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        files_walked_3(v5, v6)
    } else {
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v8))
    };
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = method6(v10, v11);
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = method11(v0, v12, v13);
    v14
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        found_files_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn ok_c7() -> Rc<dyn Fn(regex::Regex) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Regex) -> US2> = Rc::new(move |mut v0: regex::Regex| -> US2 {
        US2::US2_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19() -> Rc<dyn Fn(regex::Regex) -> US2> {
    ok_c7()
}
fn error_c8() -> Rc<dyn Fn(regex::Error) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Error) -> US2> = Rc::new(move |mut v0: regex::Error| -> US2 {
        US2::US2_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20() -> Rc<dyn Fn(regex::Error) -> US2> {
    error_c8()
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        false
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10(mut v0: regex::Regex) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let v2: bool = v0.is_match(&*v1);
        v2
    })
}
fn pattern_matcher_18(mut v0: Rc<str>, mut v1: bool) -> Rc<dyn Fn(Rc<str>) -> bool> {
    let mut v3: Rc<str> = if v1 {
        let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(?i)"); } LIT.with(|lit| lit.clone()) }, v0));
        v2
    } else {
        v0
    };
    let mut v5: Result<regex::Regex, regex::Error> = regex::Regex::new(&v3);
    let mut v6: Rc<dyn Fn(regex::Regex) -> US2> = method19();
    let mut v7: Rc<dyn Fn(regex::Error) -> US2> = method20();
    let mut v8: US2 = match v5 { Ok(x) => v6(x), Err(e) => v7(e) };
    match v8 {
        US2::US2_Error(v11) => {
            let mut v11: regex::Error = v11;
            closure9()
        }
        US2::US2_Ok(v9) => {
            let mut v9: regex::Regex = v9;
            closure10(v9)
        }
    }
}
fn closure11(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
        let mut v3: bool = v2.contains(&*v0);
        v3
    })
}
fn caseless_matcher_21(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    closure11(v1)
}
fn closure12(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let v2: bool = v1.contains(&*v0);
        v2
    })
}
fn matcher_of_17(mut v0: Rc<str>, mut v1: bool, mut v2: bool) -> Rc<dyn Fn(Rc<str>) -> bool> {
    if v2 {
        pattern_matcher_18(v0, v1)
    } else {
        if v1 {
            caseless_matcher_21(v0)
        } else {
            closure12(v0)
        }
    }
}
fn lines_of_24(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: bool = v3.ends_with(&*v2);
    let v10: Rc<str> = if v5 {
        let mut v6: i32 = v4.wrapping_sub(1i32);
        let mut v7: usize = ((0i32) as usize);
        let mut v8: usize = ((v6) as usize);
        let mut v9: Rc<str> = Rc::<str>::from(&(v3)[v7..v8]);
        v9
    } else {
        v3.clone()
    };
    let mut v11: bool = &*v3 == "";
    if v11 {
        let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v12
    } else {
        let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        v13
    }
}
fn found_in_25(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = v0.borrow()[v2 as usize].clone();
            let mut v7: bool = v1(v6);
            let mut v9: Rc<UH1> = if v7 {
                Rc::new(UH1::UH1_Cons(v2, v3))
            } else {
                v3
            };
            (v2, v3) = (v5, v9);
            continue;
        }
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<i32>>>) -> Rc<RefCell<Vec<i32>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: i32 = v2;
                let mut v3: Rc<UH1> = v3;
                let mut v4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![v2]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let mut v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method15(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let mut v12: bool = v11 < v5;
                    let mut v16: i32 = if v12 {
                        let mut v13: i32 = v1.borrow()[v11 as usize];
                        v13
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let mut v15: i32 = v4.borrow()[v14 as usize];
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let mut v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn shown_path_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: bool = v4.starts_with(&*v5);
    if v7 {
        let mut v8: i32 = (v4.clone().len() as i32);
        let mut v9: i32 = v8.wrapping_sub(v6);
        let v10: usize = ((v6) as usize);
        let mut v11: i32 = v6.wrapping_add(v9);
        let mut v12: usize = ((v11) as usize);
        let mut v13: Rc<str> = Rc::<str>::from(&(v4)[v10..v12]);
        v13
    } else {
        v4
    }
}
fn file_hits_of_23(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<str>) -> US3 {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8_lossy(&std::fs::read(&*v2).unwrap_or_default()).as_ref());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = lines_of_24(v3);
    let mut v5: i32 = (v4.clone().borrow().len() as i32);
    let mut v6: i32 = v5.wrapping_sub(1i32);
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH1> = found_in_25(&v4, v1, v6, v7);
    let mut v9: bool = match &*v8 {
        UH1::UH1_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v9 {
        US3::US3_None
    } else {
        let mut v11: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
        let mut v12: Rc<RefCell<Vec<i32>>> = method26(v8, v11);
        let mut v13: Rc<str> = shown_path_27(v0, v2);
        US3::US3_Some(v13, v12, v4)
    }
}
fn hits_from_22(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: &Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    loop {
        let mut v5: i32 = (v2.clone().borrow().len() as i32);
        let mut v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let mut v7: Rc<str> = v2.borrow()[v3 as usize].clone();
            let mut v8: US3 = file_hits_of_23(v0.clone(), v1.clone(), v7);
            let v33: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = match v8 {
                US3::US3_None => {
                    v4
                }
                US3::US3_Some(v9, v10, v11) => {
                    let mut v9: Rc<str> = v9;
                    let v10: Rc<RefCell<Vec<i32>>> = v10;
                    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = v11;
                    let mut v12: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(vec![(v9, v10, v11)]));
                    let mut v13: i32 = (v4.clone().borrow().len() as i32);
                    let mut v14: i32 = (v12.clone().borrow().len() as i32);
                    let mut v15: i32 = v13.wrapping_add(v14);
                    let mut v16: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>::default(); v15 as usize]));
                    let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                    while method15(v15, &v17) {
                        let v19: i32 = v17.borrow().l0;
                        let v20: bool = v19 < v13;
                        let (mut v28, mut v29, mut v30): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = if v20 {
                            let (mut v21, mut v22, mut v23): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v4.borrow()[v19 as usize].clone();
                            (v21, v22, v23)
                        } else {
                            let v24: i32 = v19.wrapping_sub(v13);
                            let (mut v25, mut v26, mut v27): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v12.borrow()[v24 as usize].clone();
                            (v25, v26, v27)
                        };
                        v16.borrow_mut()[v19 as usize] = (v28, v29, v30);
                        let v31: i32 = v19.wrapping_add(1i32);
                        v17.borrow_mut().l0 = v31;
                        ()
                    };
                    v16
                }
            };
            let v34: i32 = v3.wrapping_add(1i32);
            (v3, v4) = (v34, v33);
            continue;
        }
    }
}
fn closure13() -> Rc<dyn Fn(((Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>))) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>))) -> Rc<str>> = Rc::new(move |mut v0: ((Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>))| -> Rc<str> {
        let (mut v1, mut v2, mut v3): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = (v0);
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
        v6
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method28() -> Rc<dyn Fn(((Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>))) -> Rc<str>> {
    closure13()
}
fn method29(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut1>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let mut v8: bool = &*v7 < &*v1;
        let mut v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let mut v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn found_hits_16(mut v0: bool, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<RefCell<Vec<Rc<str>>>>, mut v4: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    let mut v5: Rc<str> = root_form_2(v4);
    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = matcher_of_17(v2, v1, v0);
    let mut v7: i32 = 0i32;
    let mut v8: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(vec![]));
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = hits_from_22(v5, v6, &v3, v7, v8);
    let v10: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = v9 ;
    let mut v11: Rc<dyn Fn(((Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>))) -> Rc<str>> = method28();
    let mut v12: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = Rc::new(RefCell::new(v10.borrow().clone()));
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.borrow().iter().map(|x| v11(x.clone())).collect::<Vec<_>>()));
    let mut v14: i32 = (v10.borrow().len() as i32);
    let mut v15: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
    while method15(v14, &v15) {
        let mut v17: i32 = v15.borrow().l0;
        let (mut v18, mut v19, mut v20): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v12.borrow()[v17 as usize].clone();
        let v21: Rc<str> = v13.borrow()[v17 as usize].clone();
        let mut v22: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v17 }));
        while method29(&v13, v21.clone(), &v22) {
            let v24: i32 = v22.borrow().l0;
            let v25: i32 = v24.wrapping_sub(1i32);
            let (mut v26, mut v27, mut v28): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v12.borrow()[v25 as usize].clone();
            v12.borrow_mut()[v24 as usize] = (v26, v27, v28);
            let v29: Rc<str> = v13.borrow()[v25 as usize].clone();
            v13.borrow_mut()[v24 as usize] = v29;
            v22.borrow_mut().l0 = v25;
            ()
        };
        let v30: i32 = v22.borrow().l0;
        v12.borrow_mut()[v30 as usize] = (v18, v19, v20);
        let v31: i32 = v22.borrow().l0;
        v13.borrow_mut()[v31 as usize] = v21;
        let v32: i32 = v17.wrapping_add(1i32);
        v15.borrow_mut().l0 = v32;
        ()
    };
    let v33: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = v12 ;
    v33
}
fn closure6() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, bool, bool) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>, bool, bool) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<str>, mut v3: bool, mut v4: bool| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
        found_hits_16(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn found_problem_30(mut v0: Rc<str>) -> Rc<str> {
    let v2: Result<regex::Regex, regex::Error> = regex::Regex::new(&v0);
    let mut v3: Rc<dyn Fn(regex::Regex) -> US2> = method19();
    let mut v4: Rc<dyn Fn(regex::Error) -> US2> = method20();
    let mut v5: US2 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    match v5 {
        US2::US2_Error(v8) => {
            let mut v8: regex::Error = v8;
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v8.to_string().as_str());
            v9
        }
        US2::US2_Ok(v6) => {
            let mut v6: regex::Regex = v6;
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn closure14() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        found_problem_30(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_grep_found_files(v0: &str, v1: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_grep_found_hits(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>, v2: &str, v3: bool, v4: bool) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> {
    closure6()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), v3, v4)
}
pub fn kit_grep_found_problem(v0: &str) -> Rc<str> {
    closure14()(Rc::<str>::from(v0))
}
pub type SpiralShape_2617b4e49837 = Mut0;
