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
    US0_Some(Rc<UH0>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
fn listing_command_2() -> Rc<UH0> {
    let mut v0: bool = cfg!(windows);
    if v0 {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/c"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v4.clone()));
        let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v5.clone()));
        Rc::new(UH0::UH0_Cons(v1.clone(), v6.clone()))
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v8.clone(), v9.clone()))
    }
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v2 == "";
                let v6: Rc<UH0> = if v4 {
                    v1.clone()
                } else {
                    Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()))
                };
                (v0, v1) = (v3.clone(), v6.clone());
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
                let v2: Rc<str> = v2.clone();
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
fn method5(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                drop(v0.borrow_mut().arg(&*v2));
                (v0, v1) = (v0.clone(), v3.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn names_from_7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v8: Rc<str> = v7.clone().borrow()[0i32 as usize].clone();
            let v9: i32 = (v8.clone().len() as i32);
            let v10: i32 = 0i32;
            let mut v11: i32 = method8(v8.clone(), v9, v10);
            let mut v12: i32 = v9.wrapping_sub(1i32);
            let v13: Rc<str> = string_slice(&v8.clone(), v11 as i64, v12 as i64);
            let v14: i32 = (v13.clone().len() as i32);
            let v15: i32 = method9(v13.clone(), v14);
            let v16: Rc<str> = string_slice(&v13.clone(), 0i32 as i64, v15 as i64);
            let v17: i32 = v1.wrapping_add(1i32);
            let v18: bool = &*v16 == "";
            let v20: Rc<UH0> = if v18 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v16.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v17, v20.clone());
            continue;
        }
    }
}
fn method11(mut v0: Rc<str>) -> Rc<str> {
    v0
}
fn method12(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0
}
fn go_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Vec<Rc<str>>) -> Vec<Rc<str>> {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v5.ends_with(&*v7);
            let v9: bool = v8 == false;
            let v15: Rc<str> = if v9 {
                v5.clone()
            } else {
                let v10: bool = v6 == 1i32;
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                } else {
                    let mut v12: i32 = v6.wrapping_sub(2i32);
                    let v13: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v12 as i64);
                    v13.clone()
                }
            };
            let v16: i32 = v2.wrapping_add(1i32);
            let v17: bool = &*v15 == "";
            let v26: Vec<Rc<str>> = if v17 {
                v3.clone()
            } else {
                let v18: Rc<str> = method11(v15.clone());
                let mut v19: Vec<Rc<str>> = method12(v3.clone());
                let v21: bool = true; let mut v19 = v19;
                let v23: bool = true; v19.push(v18);
                let v25: Vec<Rc<str>> = v19;
                v25.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v1, v16, v26.clone());
            continue;
        }
    }
}
fn method13(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0
}
fn names_of_listing_6(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Vec<Rc<str>> = Vec::new();
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let v5: bool = v4 > 0i32;
    let v9: bool = if v5 {
        let v6: i32 = v4.wrapping_sub(1i32);
        let v7: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
        let mut v8: bool = &*v7 == "";
        v8
    } else {
        false
    };
    let mut v11: i32 = if v9 {
        let v10: i32 = v4.wrapping_sub(1i32);
        v10
    } else {
        v4
    };
    let mut v12: i32 = 0i32;
    let v13: Vec<Rc<str>> = go_10(v3.clone(), v11, v12, v1.clone());
    let v14: Vec<Rc<str>> = method13(v13.clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v14));
    let v16: i32 = 0i32;
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    names_from_7(v15.clone(), v16, v17.clone())
}
fn attached_names_1() -> US0 {
    let mut v0: Rc<UH0> = listing_command_2();
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            let mut v3: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v1)));
            let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v5: Rc<UH0> = method3(v2.clone(), v4.clone());
            let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v7: Rc<UH0> = method4(v5.clone(), v6.clone());
            method5(v3.clone(), v7.clone());
            let mut v8: Rc<std::io::Result<std::process::Output>> = Rc::new(v3.borrow_mut().output());
            let v9: bool = v8.is_ok();
            let (mut v15, mut v16, mut v17): (bool, i32, Rc<str>) = if v9 {
                let v10: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v8).as_ref().unwrap().stderr).as_ref());
                let mut v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v8).as_ref().unwrap().stdout).as_ref());
                let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
                let v13: i32 = (&*v8).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v13, v12.clone())
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v14.clone())
            };
            let mut v19: bool = if v15 {
                let v18: bool = v16 == 0i32;
                v18
            } else {
                false
            };
            let v22: US1 = if v19 {
                US1::US1_Some(v17.clone())
            } else {
                US1::US1_None
            };
            match &v22 {
                US1::US1_None => {
                    US0::US0_None
                }
                US1::US1_Some(v23) => {
                    let v23: Rc<str> = v23.clone();
                    let v24: Rc<UH0> = names_of_listing_6(v23.clone());
                    US0::US0_Some(v24.clone())
                }
            }
        }
        UH0::UH0_Nil => {
            US0::US0_None
        }
    }
}
fn method16(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US1> = Rc::new(move |mut v0: (Rc<str>)| -> US1 {
        let v1: Rc<str> = (v0);
        US1::US1_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_15() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method16(v9.clone());
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let v14: US1 = US1::US1_None;
    let v15: US1 = v14.clone();
    let v16: US1 = v12.unwrap_or(v15);
    match &v16 {
        US1::US1_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
        US1::US1_Some(v17) => {
            let v17: Rc<str> = v17.clone();
            v17.clone()
        }
    }
}
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn joined_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_21() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_21();
    joined_19(v1, v0)
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_20(v0);
    joined_19(v2, v1)
}
fn method22(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn detached_names_14() -> US0 {
    let mut v0: Rc<str> = current_executable_15();
    let v1: bool = cfg!(windows);
    let v28: Rc<UH0> = if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("doctor-env"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/c"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v9.clone()));
        let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v10.clone()));
        let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v11.clone()));
        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v12.clone()));
        let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v13.clone()));
        let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v14.clone()));
        Rc::new(UH0::UH0_Cons(v2.clone(), v15.clone()))
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("doctor-env"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21.clone(), v22.clone()));
        let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20.clone(), v23.clone()));
        let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19.clone(), v24.clone()));
        let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18.clone(), v25.clone()));
        Rc::new(UH0::UH0_Cons(v17.clone(), v26.clone()))
    };
    let mut v29: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
    let v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH0> = method3(v28.clone(), v30.clone());
    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v33: Rc<UH0> = method4(v31.clone(), v32.clone());
    method5(v29.clone(), v33.clone());
    let mut v34: Rc<std::io::Result<std::process::Output>> = Rc::new(v29.borrow_mut().output());
    let v35: bool = v34.is_ok();
    let (mut v41, mut v42, mut v43): (bool, i32, Rc<str>) = if v35 {
        let v36: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v34).as_ref().unwrap().stderr).as_ref());
        let v37: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v34).as_ref().unwrap().stdout).as_ref());
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v36));
        let v39: i32 = (&*v34).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v39, v38.clone())
    } else {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v40.clone())
    };
    let v45: bool = if v41 {
        let v44: bool = v42 == 0i32;
        v44
    } else {
        false
    };
    let v48: US1 = if v45 {
        US1::US1_Some(v43.clone())
    } else {
        US1::US1_None
    };
    match &v48 {
        US1::US1_None => {
            US0::US0_None
        }
        US1::US1_Some(v50) => {
            let v50: Rc<str> = v50.clone();
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id=job-"); } LIT.with(|lit| lit.clone()) };
            let v52: Option<usize> = v50.find(&*v51);
            let v53: Option<usize> = method17(v52.clone());
            let mut v54: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v55: Option<i32> = v53.map(|x| v54(x));
            let v57: i32 = v55.unwrap_or(-1i32);
            let v58: bool = v57 < 0i32;
            if v58 {
                US0::US0_None
            } else {
                let v60: i32 = v57.wrapping_add(3i32);
                let v61: i32 = (v50.clone().len() as i32);
                let v62: i32 = v61.wrapping_sub(v57);
                let v63: i32 = v62.wrapping_sub(3i32);
                let v64: usize = ((v60) as usize);
                let v65: i32 = v60.wrapping_add(v63);
                let v66: usize = ((v65) as usize);
                let v67: Rc<str> = Rc::<str>::from(&(v50)[v64..v66]);
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v67.split(&*v68).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v70: Rc<str> = v69.clone().borrow()[0i32 as usize].clone();
                let v71: i32 = (v70.clone().len() as i32);
                let v72: i32 = 0i32;
                let v73: i32 = method8(v70.clone(), v71, v72);
                let v74: i32 = v71.wrapping_sub(1i32);
                let v75: Rc<str> = string_slice(&v70.clone(), v73 as i64, v74 as i64);
                let v76: i32 = (v75.clone().len() as i32);
                let v77: i32 = method9(v75.clone(), v76);
                let v78: Rc<str> = string_slice(&v75.clone(), 0i32 as i64, v77 as i64);
                let mut v79: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
                let v80: bool = &*v78 == "";
                let v92: Rc<UH0> = if v80 {
                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                    let v83: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v84: Rc<UH0> = Rc::new(UH0::UH0_Cons(v82.clone(), v83.clone()));
                    Rc::new(UH0::UH0_Cons(v81.clone(), v84.clone()))
                } else {
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v89: Rc<UH0> = Rc::new(UH0::UH0_Cons(v87.clone(), v88.clone()));
                    let v90: Rc<UH0> = Rc::new(UH0::UH0_Cons(v86.clone(), v89.clone()));
                    Rc::new(UH0::UH0_Cons(v78.clone(), v90.clone()))
                };
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
                let v94: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v95: Rc<UH0> = Rc::new(UH0::UH0_Cons(v93.clone(), v94.clone()));
                let v96: Rc<UH0> = method4(v92.clone(), v95.clone());
                method5(v79.clone(), v96.clone());
                let mut v97: Rc<std::io::Result<std::process::Output>> = Rc::new(v79.borrow_mut().output());
                let v98: bool = v97.is_ok();
                let (mut v104, mut v105, mut v106): (bool, i32, Rc<str>) = if v98 {
                    let v99: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v97).as_ref().unwrap().stderr).as_ref());
                    let v100: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v97).as_ref().unwrap().stdout).as_ref());
                    let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v99));
                    let v102: i32 = (&*v97).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v102, v101.clone())
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v103.clone())
                };
                let v108: bool = if v104 {
                    let v107: bool = v105 == 0i32;
                    v107
                } else {
                    false
                };
                let v111: US1 = if v108 {
                    US1::US1_Some(v106.clone())
                } else {
                    US1::US1_None
                };
                match &v111 {
                    US1::US1_None => {
                        US0::US0_None
                    }
                    US1::US1_Some(v113) => {
                        let v113: Rc<str> = v113.clone();
                        let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                        let v115: Rc<str> = job_file_18(v78.clone(), v114.clone());
                        let v117: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v115);
                        let v119: Vec<u8> = v117.unwrap();
                        let v120: Vec<u8> = method22(v119.clone());
                        let v122: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v120);
                        let v124: std::string::String = v122.unwrap();
                        let v126: Rc<str> = Rc::<str>::from(String::as_str(&v124));
                        let v127: Rc<UH0> = names_of_listing_6(v126.clone());
                        US0::US0_Some(v127.clone())
                    }
                }
            }
        }
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let v5: bool = &*v3 == &*v0;
                if v5 {
                    return US1::US1_Some(v3.clone());
                } else {
                    (v0, v1) = (v0.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method23(v0.clone(), v4.clone(), v2.clone());
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v3.starts_with(&*v6);
            let v10: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_"); } LIT.with(|lit| lit.clone()) };
                let v9: bool = v3.starts_with(&*v8);
                v9
            };
            let v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CARGO_"); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v3.starts_with(&*v11);
                v12
            };
            let v16: bool = if v13 {
                true
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTUP_"); } LIT.with(|lit| lit.clone()) };
                let v15: bool = v3.starts_with(&*v14);
                v15
            };
            let mut v19: bool = if v16 {
                let v17: US1 = method24(v3.clone(), v0.clone());
                match &v17 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            } else {
                false
            };
            let v23: bool = if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_JOB_"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v3.starts_with(&*v20);
                let v22: bool = v21 == false;
                v22
            } else {
                false
            };
            if v23 {
                Rc::new(UH0::UH0_Cons(v3.clone(), v5.clone()))
            } else {
                v5.clone()
            }
        }
        UH0::UH0_Nil => {
            v2.clone()
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method25(v0.clone(), v4.clone(), v2.clone());
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v3.starts_with(&*v6);
            let v10: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_"); } LIT.with(|lit| lit.clone()) };
                let v9: bool = v3.starts_with(&*v8);
                v9
            };
            let v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CARGO_"); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v3.starts_with(&*v11);
                v12
            };
            let v16: bool = if v13 {
                true
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTUP_"); } LIT.with(|lit| lit.clone()) };
                let v15: bool = v3.starts_with(&*v14);
                v15
            };
            let mut v19: bool = if v16 {
                let v17: US1 = method24(v3.clone(), v0.clone());
                match &v17 {
                    US1::US1_None => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            } else {
                false
            };
            if v19 {
                Rc::new(UH0::UH0_Cons(v3.clone(), v5.clone()))
            } else {
                v5.clone()
            }
        }
        UH0::UH0_Nil => {
            v2.clone()
        }
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method26(v3.clone(), v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            Rc::new(UH0::UH0_Cons(v6.clone(), v4.clone()))
        }
        UH0::UH0_Nil => {
            v1.clone()
        }
    }
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method27(v3.clone(), v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            Rc::new(UH0::UH0_Cons(v6.clone(), v4.clone()))
        }
        UH0::UH0_Nil => {
            v1.clone()
        }
    }
}
fn method28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
fn method29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
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
fn env_drift_0() -> (bool, Rc<str>) {
    let mut v0: US0 = attached_names_1();
    let v1: US0 = detached_names_14();
    match &v0 {
        US0::US0_Some(v2) => {
            let v2: Rc<UH0> = v2.clone();
            match &v1 {
                US0::US0_Some(v3) => {
                    let mut v3: Rc<UH0> = v3.clone();
                    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v5: Rc<UH0> = method23(v2.clone(), v3.clone(), v4.clone());
                    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v7: Rc<UH0> = method25(v3.clone(), v2.clone(), v6.clone());
                    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v9: Rc<UH0> = method26(v5.clone(), v8.clone());
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v11: Rc<UH0> = method27(v7.clone(), v10.clone());
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH0> = method4(v9.clone(), v12.clone());
                    let v14: Rc<UH0> = method28(v11.clone(), v13.clone());
                    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v16: Rc<UH0> = method4(v14.clone(), v15.clone());
                    let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v18: i32 = 0i32;
                    let mut v19: i32 = method29(v17.clone(), v16.clone(), v18);
                    let v20: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
                    let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v22));
                    (true, v23.clone())
                }
                _ => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, v24.clone())
                }
            }
        }
        _ => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v27.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>) {
        env_drift_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_doctor_env_drift() -> (bool, Rc<str>) {
    closure0()()
}
