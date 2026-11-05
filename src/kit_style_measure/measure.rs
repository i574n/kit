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
struct Mut0 { l0: i32 }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum US1 {
    US1_Ok(regex::Regex),
    US1_Error(regex::Error),
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
                let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v4));
                let mut v6: bool = &*v4 == "";
                let mut v9: bool = if v6 {
                    true
                } else {
                    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v8: bool = v4.ends_with(&*v7);
                    v8
                };
                let v12: Rc<str> = if v9 {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                } else {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                };
                let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v12));
                (v0, v1) = (v3.clone(), v13.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
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
fn paths_from_6(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v6));
            let mut v8: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let mut v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v3.clone()));
            (v2, v3) = (v5, v10.clone());
            continue;
        }
    }
}
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US0> = Rc::new(move |mut v0: Vec<std::string::String>| -> US0 {
        US0::US0_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7() -> Rc<dyn Fn(Vec<std::string::String>) -> US0> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US0> = Rc::new(move |mut v0: std::io::Error| -> US0 {
        US0::US0_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8() -> Rc<dyn Fn(std::io::Error) -> US0> {
    error_c2()
}
fn method10(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method10(v7, v9.clone()) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let mut v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut1>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
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
fn child_paths_5(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US0> = method7();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US0> = method8();
    let mut v5: US0 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    let v26: Rc<UH0> = match v5 {
        US0::US0_Error(v23) => {
            let v23: std::io::Error = v23;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6.clone();
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let v11: Rc<UH0> = acc;
            let mut v13: std::string::String = x;
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15.clone(), v11.clone()));
            let v18: bool = true; v17 });
            let v20: Rc<UH0> = _vec_fold_;
            let v21: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method4(v20.clone(), v21.clone())
        }
    };
    let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v28: Rc<RefCell<Vec<Rc<str>>>> = method9(v26.clone(), v27.clone());
    let v29: Rc<RefCell<Vec<Rc<str>>>> = v28 ;
    let mut v30: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method11();
    let mut v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v29.borrow().clone()));
    let mut v32: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v29.borrow().iter().map(|x| v30(x.clone())).collect::<Vec<_>>()));
    let v33: i32 = (v29.clone().borrow().len() as i32);
    let mut v34: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
    while method10(v33, v34.clone()) {
        let v36: i32 = v34.borrow().l0;
        let v37: Rc<str> = v31.borrow()[v36 as usize].clone();
        let v38: Rc<str> = v32.borrow()[v36 as usize].clone();
        let mut v39: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v36 }));
        while method12(v32.clone(), v38.clone(), v39.clone()) {
            let v41: i32 = v39.borrow().l0;
            let v42: i32 = v41.wrapping_sub(1i32);
            let v43: Rc<str> = v31.borrow()[v42 as usize].clone();
            v31.borrow_mut()[v41 as usize] = v43.clone();
            let v44: Rc<str> = v32.borrow()[v42 as usize].clone();
            v32.borrow_mut()[v41 as usize] = v44.clone();
            v39.borrow_mut().l0 = v42;
            ()
        };
        let v45: i32 = v39.borrow().l0;
        v31.borrow_mut()[v45 as usize] = v37.clone();
        let v46: i32 = v39.borrow().l0;
        v32.borrow_mut()[v46 as usize] = v38.clone();
        let v47: i32 = v36.wrapping_add(1i32);
        v34.borrow_mut().l0 = v47;
        ()
    };
    let v48: Rc<RefCell<Vec<Rc<str>>>> = v31 ;
    let v49: i32 = (v48.clone().borrow().len() as i32);
    let v50: i32 = v49.wrapping_sub(1i32);
    let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    paths_from_6(v0.clone(), v48.clone(), v50, v51.clone())
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                let mut v5: bool = v2.ends_with(&*v4);
                let v7: Rc<UH0> = if v5 {
                    Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()))
                } else {
                    v1.clone()
                };
                (v0, v1) = (v3.clone(), v7.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn spi_paths_3(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = child_paths_5(v0);
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method13(v1, v2);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method4(v3, v4)
}
fn package_text_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH0> = spi_paths_3(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method2(v1, v2)
}
fn lines_of_14(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: bool = v3.ends_with(&*v2);
    let mut v10: Rc<str> = if v5 {
        let mut v6: i32 = v4.wrapping_sub(1i32);
        let v7: usize = ((0i32) as usize);
        let mut v8: usize = ((v6) as usize);
        let mut v9: Rc<str> = Rc::<str>::from(&(v3)[v7..v8]);
        v9.clone()
    } else {
        v3.clone()
    };
    let v11: bool = &*v3 == "";
    if v11 {
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v12.clone()
    } else {
        let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v10.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        v13.clone()
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure4() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn package_name_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: bool = v3.ends_with(&*v2);
    let mut v10: Rc<str> = if v4 {
        let mut v5: i32 = (v3.clone().len() as i32);
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: usize = ((0i32) as usize);
        let mut v8: usize = ((v6) as usize);
        let mut v9: Rc<str> = Rc::<str>::from(&(v3)[v7..v8]);
        v9.clone()
    } else {
        v3.clone()
    };
    let v11: Option<usize> = v10.rfind(&*v2);
    let v12: Option<usize> = method16(v11.clone());
    let mut v13: Rc<dyn Fn((usize)) -> i32> = closure4();
    let mut v14: Option<i32> = v12.map(|x| v13(x));
    let mut v16: i32 = v14.unwrap_or(-1i32);
    let mut v17: bool = v16 < 0i32;
    if v17 {
        v10.clone()
    } else {
        let v18: i32 = v16.wrapping_add(1i32);
        let v19: i32 = (v10.clone().len() as i32);
        let v20: i32 = v19.wrapping_sub(v16);
        let v21: i32 = v20.wrapping_sub(1i32);
        let v22: usize = ((v18) as usize);
        let v23: i32 = v18.wrapping_add(v21);
        let v24: usize = ((v23) as usize);
        let v25: Rc<str> = Rc::<str>::from(&(v10)[v22..v24]);
        v25.clone()
    }
}
fn method18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
fn tests_from_17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: bool, mut v3: i32) -> i32 {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v6.starts_with(&*v7);
            let v24: Rc<str> = if v8 {
                let mut v9: i32 = (v6.clone().len() as i32);
                let mut v10: i32 = v9.wrapping_sub(5i32);
                let v11: usize = ((5i32) as usize);
                let v12: i32 = 5i32.wrapping_add(v10);
                let mut v13: usize = ((v12) as usize);
                let mut v14: Rc<str> = Rc::<str>::from(&(v6)[v11..v13]);
                let v15: i32 = (v14.clone().len() as i32);
                let mut v16: i32 = 0i32;
                let mut v17: i32 = method18(v14.clone(), v15, v16);
                let v18: i32 = v15.wrapping_sub(1i32);
                let v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
                let v20: i32 = (v19.clone().len() as i32);
                let v21: i32 = method19(v19.clone(), v20);
                let v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
                v22.clone()
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            };
            let v26: bool = if v8 {
                let v25: bool = &*v24 != "_";
                v25
            } else {
                v2
            };
            let v27: i32 = v1.wrapping_add(1i32);
            let v29: i32 = if v26 {
                let v28: i32 = v3.wrapping_add(1i32);
                v28
            } else {
                v3
            };
            (v1, v2, v3) = (v27, v26, v29);
            continue;
        }
    }
}
fn matches_from_22(mut v0: regex::Regex, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v2.wrapping_sub(1i32);
            let mut v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let v7: bool = v0.is_match(&*v6);
            let mut v9: i32 = if v7 {
                let mut v8: i32 = v3.wrapping_add(1i32);
                v8
            } else {
                v3
            };
            (v2, v3) = (v5, v9);
            continue;
        }
    }
}
fn ok_c5() -> Rc<dyn Fn(regex::Regex) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Regex) -> US1> = Rc::new(move |mut v0: regex::Regex| -> US1 {
        US1::US1_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method23() -> Rc<dyn Fn(regex::Regex) -> US1> {
    ok_c5()
}
fn error_c6() -> Rc<dyn Fn(regex::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(regex::Error) -> US1> = Rc::new(move |mut v0: regex::Error| -> US1 {
        US1::US1_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24() -> Rc<dyn Fn(regex::Error) -> US1> {
    error_c6()
}
fn matching_lines_21(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>) -> i32 {
    let mut v3: Result<regex::Regex, regex::Error> = regex::Regex::new(&v1);
    let mut v4: Rc<dyn Fn(regex::Regex) -> US1> = method23();
    let mut v5: Rc<dyn Fn(regex::Error) -> US1> = method24();
    let mut v6: US1 = match v3 { Ok(x) => v4(x), Err(e) => v5(e) };
    match &v6 {
        US1::US1_Error(v12) => {
            let v12: regex::Error = v12.clone();
            0i32
        }
        US1::US1_Ok(v7) => {
            let v7: regex::Regex = v7.clone();
            let mut v8: i32 = (v0.clone().borrow().len() as i32);
            let mut v9: i32 = v8.wrapping_sub(1i32);
            let mut v10: i32 = 0i32;
            matches_from_22(v7, v0, v9, v10)
        }
    }
}
fn rows_from_20(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    loop {
        let mut v6: i32 = (v0.clone().borrow().len() as i32);
        let v7: bool = v4 >= v6;
        if v7 {
            return v5.clone();
        } else {
            let (mut v8, mut v9): (Rc<str>, Rc<str>) = v0.borrow()[v4 as usize].clone();
            let mut v10: i32 = matching_lines_21(v2.clone(), v9.clone());
            let v11: i32 = v4.wrapping_add(1i32);
            let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![(v1.clone(), v8.clone(), v10, v3)]));
            let mut v13: i32 = (v5.clone().borrow().len() as i32);
            let mut v14: i32 = (v12.clone().borrow().len() as i32);
            let v15: i32 = v13.wrapping_add(v14);
            let mut v16: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, i32)>::default(); v15 as usize]));
            let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
            while method10(v15, v17.clone()) {
                let v19: i32 = v17.borrow().l0;
                let v20: bool = v19 < v13;
                let (mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, i32, i32) = if v20 {
                    let (mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, i32, i32) = v5.borrow()[v19 as usize].clone();
                    (v21.clone(), v22.clone(), v23, v24)
                } else {
                    let v25: i32 = v19.wrapping_sub(v13);
                    let (mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<str>, i32, i32) = v12.borrow()[v25 as usize].clone();
                    (v26.clone(), v27.clone(), v28, v29)
                };
                v16.borrow_mut()[v19 as usize] = (v30.clone(), v31.clone(), v32, v33);
                let mut v34: i32 = v19.wrapping_add(1i32);
                v17.borrow_mut().l0 = v34;
                ()
            };
            (v4, v5) = (v11, v16.clone());
            continue;
        }
    }
}
fn measured_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    let mut v1: Rc<str> = package_text_1(v0.clone());
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = lines_of_14(v1);
    let mut v3: Rc<str> = package_name_15(v0);
    let mut v4: i32 = (v2.clone().borrow().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: bool = false;
    let v7: i32 = 0i32;
    let mut v8: i32 = tests_from_17(v2.clone(), v5, v6, v7);
    let mut v9: i32 = v4.wrapping_sub(v8);
    let mut v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = kit_style_probes::kit_style_probe_specs();
    let v11: i32 = 0i32;
    let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![]));
    let mut v13: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = rows_from_20(v10, v3, v2, v9, v11, v12);
    v13
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
        measured_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn list_of_base_26(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v1, v2) = (v4, v6.clone());
            continue;
        }
    }
}
fn directory_exists_30(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = directory_exists_30(v2.clone());
                let mut v8: bool = if v4 {
                    let mut v5: Rc<UH0> = spi_paths_3(v2.clone());
                    let mut v6: bool = match &*v5 {
                        UH0::UH0_Nil => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v7: bool = v6 != true;
                    v7
                } else {
                    false
                };
                let mut v10: Rc<UH0> = if v8 {
                    Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()))
                } else {
                    v1.clone()
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
fn package_children_28(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<UH0> = child_paths_5(v0);
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method29(v1, v2);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method4(v3, v4)
}
fn method31(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
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
fn packages_from_27(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: Rc<UH0> = spi_paths_3(v4.clone());
                let v7: bool = match &*v6 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v8: bool = v7 != true;
                let v12: Rc<UH0> = if v8 {
                    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH0::UH0_Cons(v4.clone(), v9.clone()))
                } else {
                    package_children_28(v4.clone())
                };
                let mut v13: Rc<UH0> = method31(v12.clone(), v1.clone());
                (v0, v1) = (v5.clone(), v13.clone());
                continue;
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method4(v1.clone(), v2.clone());
            }
        }
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let mut v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method10(v7, v9.clone()) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let mut v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let mut v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let mut v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn packages_under_25(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = list_of_base_26(v0, v2, v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = packages_from_27(v4, v5);
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v8: Rc<RefCell<Vec<Rc<str>>>> = method32(v6, v7);
    v8
}
fn closure7() -> Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<RefCell<Vec<Rc<str>>>>| -> Rc<RefCell<Vec<Rc<str>>>> {
        packages_under_25(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_style_measured(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_style_packages_under(v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure7()(v0)
}
