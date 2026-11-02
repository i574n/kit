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
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH0>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method2(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
            let mut v7: bool = if v5 {
                let mut v6: bool = &*v2 != ".";
                v6
            } else {
                false
            };
            if v7 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn method5(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method5(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method4(v16.clone(), v17.clone())
    }
}
fn segments_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v1));
    let mut v4: Rc<UH0> = split_list_3(v1, v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method2(v4, v5)
}
fn method6(mut v0: Rc<UH0>) -> US0 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                let mut v5: bool = v2.contains(&*v4);
                if v5 {
                    return US0::US0_Some(v2.clone());
                } else {
                    v0 = v3.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn method7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method9(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method9(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method11(v3, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method14(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method12(v3, v1.clone());
            let mut v5: i32 = (v2.clone().len() as i32);
            let mut v6: i32 = 0i32;
            let mut v7: i32 = method13(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method14(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method15(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn base_of_16(mut v0: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
            let mut v4: bool = v1.contains(&*v3);
            if v4 {
                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
            } else {
                let mut v6: Rc<UH0> = base_of_16(v2);
                Rc::new(UH0::UH0_Cons(v1, v6))
            }
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
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
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US1> = method19(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure1(v0, v1)
}
fn method24(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn rest_matches_23(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Nil => {
                        let mut v4: bool = v0.ends_with(&*v2);
                        return v4;
                    }
                    _ => {
                        let mut v5: Option<usize> = v0.find(&*v2);
                        let mut v6: Option<usize> = method24(v5.clone());
                        let mut v7: Rc<dyn Fn((usize)) -> i32> = closure2();
                        let mut v8: Option<i32> = v6.map(|x| v7(x));
                        let v10: i32 = v8.unwrap_or(-1i32);
                        let v11: i32 = (v2.clone().len() as i32);
                        let v12: i32 = v10.wrapping_add(v11);
                        let v13: bool = v10 >= 0i32;
                        if v13 {
                            let mut v14: i32 = (v0.clone().len() as i32);
                            let mut v15: i32 = v14.wrapping_sub(v12);
                            let v16: usize = ((v12) as usize);
                            let v17: i32 = v12.wrapping_add(v15);
                            let v18: usize = ((v17) as usize);
                            let v19: Rc<str> = Rc::<str>::from(&(v0)[v16..v18]);
                            (v0, v1) = (v19.clone(), v3.clone());
                            continue;
                        } else {
                            return false;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                return true;
            }
        }
    }
}
fn wildcard_22(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = split_list_3(v2, v0.clone());
    match &*v3 {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            match &*v6 {
                UH0::UH0_Nil => {
                    let mut v7: bool = &*v0 == &*v1;
                    v7
                }
                _ => {
                    let mut v8: i32 = (v5.clone().len() as i32);
                    let mut v9: bool = v1.starts_with(&*v5);
                    if v9 {
                        let v10: i32 = (v1.clone().len() as i32);
                        let v11: i32 = v10.wrapping_sub(v8);
                        let v12: usize = ((v8) as usize);
                        let v13: i32 = v8.wrapping_add(v11);
                        let mut v14: usize = ((v13) as usize);
                        let mut v15: Rc<str> = Rc::<str>::from(&(v1)[v12..v14]);
                        rest_matches_23(v15, v6)
                    } else {
                        false
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v4: bool = &*v1 == "";
            v4
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = wildcard_22(v3.clone(), v0.clone());
                if v5 {
                    return US0::US0_Some(v3.clone());
                } else {
                    v1 = v4.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn any_suffix_26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    loop {
        let mut v2: bool = glob_match_25(v0.clone(), v1.clone());
        if v2 {
            return true;
        } else {
            match &*v1 {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let mut v4: Rc<UH0> = v4.clone();
                    v1 = v4.clone();
                    continue;
                }
                UH0::UH0_Nil => {
                    return false;
                }
            }
        }
    }
}
fn glob_match_25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = &*v3 == "**";
                if v5 {
                    return any_suffix_26(v4.clone(), v1.clone());
                } else {
                    match &*v1 {
                        UH0::UH0_Cons(v7, v8) => {
                            let mut v7: Rc<str> = v7.clone();
                            let mut v8: Rc<UH0> = v8.clone();
                            let mut v9: bool = wildcard_22(v3.clone(), v7.clone());
                            if v9 {
                                (v0, v1) = (v4.clone(), v8.clone());
                                continue;
                            } else {
                                return false;
                            }
                        }
                        UH0::UH0_Nil => {
                            return false;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                match &*v1 {
                    UH0::UH0_Nil => {
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
        }
    }
}
fn kept_20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: Rc<str>, mut v5: Rc<UH0>) -> bool {
    let mut v9: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            let mut v6: US0 = method21(v4, v3);
            let mut v7: bool = match &v6 {
                US0::US0_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v8: bool = v7 != true;
            v8
        }
    };
    let v11: bool = match &*v2 {
        UH0::UH0_Nil => {
            false
        }
        _ => {
            glob_match_25(v2, v5.clone())
        }
    };
    let v12: bool = glob_match_25(v1, v5);
    let v13: bool = v12 && v9;
    if v13 {
        let mut v14: bool = v11 == false;
        v14
    } else {
        false
    }
}
fn scan_18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v5: Rc<UH0>, mut v6: std::rc::Rc<Vec<(std::rc::Rc<str>, bool, bool)>>, mut v7: i32, mut v8: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v9: usize = v6.len();
        let v10: i32 = ((v9) as i32);
        let v11: bool = v7 >= v10;
        if v11 {
            return v8.clone();
        } else {
            let v12: usize = ((v7) as usize);
            let v13: Rc<str> = v6[v12].0.clone();
            let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v14.clone()));
            let v16: Rc<UH0> = method11(v5.clone(), v15.clone());
            let v17: usize = ((v7) as usize);
            let v18: bool = v6[v17].2;
            if v18 {
                let v19: i32 = v7.wrapping_add(1i32);
                v7 = v19;
                continue;
            } else {
                let v21: usize = ((v7) as usize);
                let v22: bool = v6[v21].1;
                if v22 {
                    let v23: i32 = v7.wrapping_add(1i32);
                    let v24: i32 = 0i32;
                    let mut v25: Rc<dyn Fn(Rc<UH0>) -> US1> = method19(v13.clone(), v24);
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".git"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache"); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31.clone(), v32.clone()));
                    let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30.clone(), v33.clone()));
                    let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29.clone(), v34.clone()));
                    let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28.clone(), v35.clone()));
                    let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27.clone(), v36.clone()));
                    let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26.clone(), v37.clone()));
                    let v39: US1 = v25(v38.clone());
                    let v40: bool = match &v39 {
                        US1::US1_None => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v41: bool = v40 != true;
                    let v43: Rc<UH1> = if v41 {
                        v8.clone()
                    } else {
                        Rc::new(UH1::UH1_Cons(v16.clone(), v8.clone()))
                    };
                    (v7, v8) = (v23, v43.clone());
                    continue;
                } else {
                    let v45: bool = kept_20(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v13.clone(), v16.clone());
                    let v54: bool = if v45 {
                        let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v47: i32 = 0i32;
                        let v48: i32 = method7(v46.clone(), v16.clone(), v47);
                        let v49: Rc<Vec<Rc<str>>> = Rc::new(v46.borrow().clone());
                        let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                        let v53: bool = v4.borrow_mut().insert(v52);
                        v53
                    } else {
                        false
                    };
                    let v55: i32 = v7.wrapping_add(1i32);
                    v7 = v55;
                    continue;
                }
            }
        }
    }
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<UH0> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = method27(v3, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn walked_17(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v5: Rc<UH1>) -> () {
    loop {
        match &*v5 {
            UH1::UH1_Cons(v6, v7) => {
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let mut v9: i32 = 0i32;
                let v10: i32 = method7(v8.clone(), v6.clone(), v9);
                let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
                let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
                let mut v15: Rc<str> = method8(v0.clone(), v14.clone());
                let v16: std::rc::Rc<Vec<(std::rc::Rc<str>, bool, bool)>> = std::rc::Rc::new(std::fs::read_dir(&*v15).map(|entries| entries.filter_map(Result::ok).filter_map(|entry| entry.file_type().ok().map(|kind| (std::rc::Rc::<str>::from(entry.file_name().to_string_lossy().as_ref()), kind.is_dir(), kind.is_symlink()))).collect::<Vec<_>>()).unwrap_or_default());
                let v17: i32 = 0i32;
                let v18: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v19: Rc<UH1> = scan_18(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v6.clone(), v16.clone(), v17, v18.clone());
                let v20: Rc<UH1> = method27(v19.clone(), v7.clone());
                v5 = v20.clone();
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn matching_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let mut v4: Rc<UH0> = segments_1(v1);
    let mut v5: US0 = method6(v4.clone());
    let mut v6: bool = match &v5 {
        US0::US0_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v7: bool = v6 != true;
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v9: i32 = 0i32;
    let v10: i32 = method7(v8.clone(), v4.clone(), v9);
    let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
    let mut v15: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeSet::new()));
    let v16: bool = v7 == false;
    let v20: bool = if v16 {
        let v17: Rc<str> = method8(v0.clone(), v14.clone());
        let v18: bool = directory_exists_10(v17);
        let v19: bool = v18 == false;
        v19
    } else {
        false
    };
    if v20 {
        let v21: bool = v15.borrow_mut().insert(v14);
        let v22: std::rc::Rc<Vec<std::rc::Rc<str>>> = std::rc::Rc::new(v15.borrow().iter().cloned().collect::<Vec<_>>());
        v22
    } else {
        let v27: Rc<UH0> = if v7 {
            v4.clone()
        } else {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v24));
            method11(v4, v25)
        };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<UH0> = split_list_3(v28, v2);
        let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v31: Rc<UH0> = method12(v29, v30);
        let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v33: Rc<UH0> = method15(v31, v32);
        let v34: Rc<UH0> = segments_1(v3);
        let v35: Rc<UH0> = base_of_16(v27.clone());
        let v36: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v37: Rc<UH1> = Rc::new(UH1::UH1_Cons(v35, v36));
        walked_17(v0, v27, v34, v33, v15.clone(), v37);
        let v38: std::rc::Rc<Vec<std::rc::Rc<str>>> = std::rc::Rc::new(v15.borrow().iter().cloned().collect::<Vec<_>>());
        v38
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
        matching_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_walk_matching(v0: &str, v1: &str, v2: &str, v3: &str) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
