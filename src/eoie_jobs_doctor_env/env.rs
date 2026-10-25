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
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/c"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v4.clone()));
        let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v5.clone()));
        Rc::new(UH0::UH0_Cons(v1.clone(), v6.clone()))
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v8.clone(), v9.clone()))
    }
}
fn method3(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v12, mut v13): (Rc<str>, Rc<str>) = match &*v1 {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<UH0> = v5.clone();
            let (mut v6, mut v7): (Rc<str>, Rc<str>) = method3(v0.clone(), v5.clone(), v2.clone(), v3.clone());
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v6));
            (v9.clone(), v0.clone())
        }
        _ => {
            let (mut v10, mut v11): (Rc<str>, Rc<str>) = match &*v1 {
                UH0::UH0_Nil => {
                    (v2.clone(), v3.clone())
                }
                _ => unreachable!(),
            };
            (v10.clone(), v11.clone())
        }
    };
    (v12.clone(), v13.clone())
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
fn names_from_5(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v5.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v8: Rc<str> = v7.clone().borrow()[0i32 as usize].clone();
            let mut v9: i32 = (v8.clone().len() as i32);
            let mut v10: i32 = 0i32;
            let mut v11: i32 = method6(v8.clone(), v9, v10);
            let mut v12: i32 = v9.wrapping_sub(1i32);
            let mut v13: Rc<str> = string_slice(&v8.clone(), v11 as i64, v12 as i64);
            let mut v14: i32 = (v13.clone().len() as i32);
            let mut v15: i32 = method7(v13.clone(), v14);
            let mut v16: Rc<str> = string_slice(&v13.clone(), 0i32 as i64, v15 as i64);
            let mut v17: i32 = v1.wrapping_add(1i32);
            let mut v18: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v20: Rc<UH0> = if v18 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v16.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v17, v20.clone());
            continue;
        }
    }
}
fn method9(mut v0: Rc<str>) -> Rc<str> {
    v0.clone()
}
fn method10(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0.clone()
}
fn go_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Vec<Rc<str>>) -> Vec<Rc<str>> {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v5.ends_with(&*v7);
            let mut v9: bool = v8 == false;
            let mut v15: Rc<str> = if v9 {
                v5.clone()
            } else {
                let mut v10: bool = v6 == 1i32;
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                } else {
                    let mut v12: i32 = v6.wrapping_sub(2i32);
                    let mut v13: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v12 as i64);
                    v13.clone()
                }
            };
            let mut v16: i32 = v2.wrapping_add(1i32);
            let mut v17: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v26: Vec<Rc<str>> = if v17 {
                v3.clone()
            } else {
                let mut v18: Rc<str> = method9(v15.clone());
                let mut v19: Vec<Rc<str>> = method10(v3.clone());
                let mut v21: bool = true; let mut v19 = v19;
                let mut v23: bool = true; v19.push(v18);
                let mut v25: Vec<Rc<str>> = v19;
                v25.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v1, v16, v26.clone());
            continue;
        }
    }
}
fn method11(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0.clone()
}
fn names_of_listing_4(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Vec<Rc<str>> = Vec::new();
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: bool = v4 > 0i32;
    let mut v9: bool = if v5 {
        let mut v6: i32 = v4.wrapping_sub(1i32);
        let mut v7: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
        let mut v8: bool = v7.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        false
    };
    let mut v11: i32 = if v9 {
        let mut v10: i32 = v4.wrapping_sub(1i32);
        v10
    } else {
        v4
    };
    let mut v12: i32 = 0i32;
    let mut v13: Vec<Rc<str>> = go_8(v3.clone(), v11, v12, v1.clone());
    let mut v14: Vec<Rc<str>> = method11(v13.clone());
    let mut v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v14));
    let mut v16: i32 = 0i32;
    let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    names_from_5(v15.clone(), v16, v17.clone())
}
fn attached_names_1() -> US0 {
    let mut v0: Rc<UH0> = listing_command_2();
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v5, mut v6): (Rc<str>, Rc<str>) = method3(v3.clone(), v2.clone(), v4.clone(), v4.clone());
            let (mut v7, mut v8): (bool, Rc<str>) = match std::process::Command::new(&*v1).args((&*v5).split(&*v3).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            let mut v11: US1 = if v7 {
                US1::US1_Some(v8.clone())
            } else {
                US1::US1_None
            };
            match &v11 {
                US1::US1_None => {
                    US0::US0_None
                }
                US1::US1_Some(v12) => {
                    let mut v12: Rc<str> = v12.clone();
                    let mut v13: Rc<UH0> = names_of_listing_4(v12.clone());
                    US0::US0_Some(v13.clone())
                }
            }
        }
        UH0::UH0_Nil => {
            US0::US0_None
        }
    }
}
fn current_executable_13() -> Rc<str> {
    let mut v0: Rc<str> = std::env::current_exe().map(|path| Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
    v0.clone()
}
fn method14(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn joined_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_18() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_18();
    joined_16(v1.clone(), v0.clone())
}
fn job_file_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_17(v0.clone());
    joined_16(v2.clone(), v1.clone())
}
fn method19(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn detached_names_12() -> US0 {
    let mut v0: Rc<str> = current_executable_13();
    let mut v1: bool = cfg!(windows);
    let mut v28: Rc<UH0> = if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("doctor-env"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/c"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v9.clone()));
        let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v10.clone()));
        let mut v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v11.clone()));
        let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v12.clone()));
        let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v13.clone()));
        let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v14.clone()));
        Rc::new(UH0::UH0_Cons(v2.clone(), v15.clone()))
    } else {
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("doctor-env"); } LIT.with(|lit| lit.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21.clone(), v22.clone()));
        let mut v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20.clone(), v23.clone()));
        let mut v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19.clone(), v24.clone()));
        let mut v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18.clone(), v25.clone()));
        Rc::new(UH0::UH0_Cons(v17.clone(), v26.clone()))
    };
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v31, mut v32): (Rc<str>, Rc<str>) = method3(v29.clone(), v28.clone(), v30.clone(), v30.clone());
    let (mut v33, mut v34): (bool, Rc<str>) = match std::process::Command::new(&*v0).args((&*v31).split(&*v29).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let mut v37: US1 = if v33 {
        US1::US1_Some(v34.clone())
    } else {
        US1::US1_None
    };
    match &v37 {
        US1::US1_None => {
            US0::US0_None
        }
        US1::US1_Some(v39) => {
            let mut v39: Rc<str> = v39.clone();
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id=job-"); } LIT.with(|lit| lit.clone()) };
            let mut v41: Option<usize> = v39.find(&*v40);
            let mut v42: Option<usize> = method14(v41.clone());
            let mut v43: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v44: Option<i32> = v42.map(|x| v43(x));
            let mut v46: i32 = v44.unwrap_or(-1i32);
            let mut v47: bool = v46 < 0i32;
            if v47 {
                US0::US0_None
            } else {
                let mut v49: i32 = v46.wrapping_add(3i32);
                let mut v50: i32 = (v39.clone().len() as i32);
                let mut v51: i32 = v50.wrapping_sub(v46);
                let mut v52: i32 = v51.wrapping_sub(3i32);
                let mut v53: usize = ((v49) as usize);
                let mut v54: i32 = v49.wrapping_add(v52);
                let mut v55: usize = ((v54) as usize);
                let mut v56: Rc<str> = Rc::<str>::from(&(v39)[v53..v55]);
                let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let mut v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v56.split(&*v57).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v59: Rc<str> = v58.clone().borrow()[0i32 as usize].clone();
                let mut v60: i32 = (v59.clone().len() as i32);
                let mut v61: i32 = 0i32;
                let mut v62: i32 = method6(v59.clone(), v60, v61);
                let mut v63: i32 = v60.wrapping_sub(1i32);
                let mut v64: Rc<str> = string_slice(&v59.clone(), v62 as i64, v63 as i64);
                let mut v65: i32 = (v64.clone().len() as i32);
                let mut v66: i32 = method7(v64.clone(), v65);
                let mut v67: Rc<str> = string_slice(&v64.clone(), 0i32 as i64, v66 as i64);
                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
                let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
                let mut v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("30000"); } LIT.with(|lit| lit.clone()) };
                let mut v72: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                let mut v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
                let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait"); } LIT.with(|lit| lit.clone()) };
                let mut v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v73));
                let mut v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v76));
                let (mut v80, mut v81): (bool, Rc<str>) = match std::process::Command::new(&*v0).args((&*v79).split(&*v68).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
                let mut v84: US1 = if v80 {
                    US1::US1_Some(v81.clone())
                } else {
                    US1::US1_None
                };
                match &v84 {
                    US1::US1_None => {
                        US0::US0_None
                    }
                    US1::US1_Some(v86) => {
                        let mut v86: Rc<str> = v86.clone();
                        let mut v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
                        let mut v88: Rc<str> = job_file_15(v67.clone(), v87.clone());
                        let mut v90: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v88);
                        let mut v92: Vec<u8> = v90.unwrap();
                        let mut v93: Vec<u8> = method19(v92.clone());
                        let mut v95: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v93);
                        let mut v97: std::string::String = v95.unwrap();
                        let mut v99: Rc<str> = Rc::<str>::from(String::as_str(&v97));
                        let mut v100: Rc<UH0> = names_of_listing_4(v99.clone());
                        US0::US0_Some(v100.clone())
                    }
                }
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v3.clone() == v0.clone();
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
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<UH0> = method20(v0.clone(), v4.clone(), v2.clone());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v3.starts_with(&*v6);
            let mut v10: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_"); } LIT.with(|lit| lit.clone()) };
                let mut v9: bool = v3.starts_with(&*v8);
                v9
            };
            let mut v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CARGO_"); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v3.starts_with(&*v11);
                v12
            };
            let mut v16: bool = if v13 {
                true
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTUP_"); } LIT.with(|lit| lit.clone()) };
                let mut v15: bool = v3.starts_with(&*v14);
                v15
            };
            let mut v19: bool = if v16 {
                let mut v17: US1 = method21(v3.clone(), v0.clone());
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
            let mut v23: bool = if v19 {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_JOB_"); } LIT.with(|lit| lit.clone()) };
                let mut v21: bool = v3.starts_with(&*v20);
                let mut v22: bool = v21 == false;
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
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<UH0> = method22(v0.clone(), v4.clone(), v2.clone());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v3.starts_with(&*v6);
            let mut v10: bool = if v7 {
                true
            } else {
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_"); } LIT.with(|lit| lit.clone()) };
                let mut v9: bool = v3.starts_with(&*v8);
                v9
            };
            let mut v13: bool = if v10 {
                true
            } else {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CARGO_"); } LIT.with(|lit| lit.clone()) };
                let mut v12: bool = v3.starts_with(&*v11);
                v12
            };
            let mut v16: bool = if v13 {
                true
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RUSTUP_"); } LIT.with(|lit| lit.clone()) };
                let mut v15: bool = v3.starts_with(&*v14);
                v15
            };
            let mut v19: bool = if v16 {
                let mut v17: US1 = method21(v3.clone(), v0.clone());
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
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method23(v3.clone(), v1.clone());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            Rc::new(UH0::UH0_Cons(v6.clone(), v4.clone()))
        }
        UH0::UH0_Nil => {
            v1.clone()
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method24(v3.clone(), v1.clone());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            Rc::new(UH0::UH0_Cons(v6.clone(), v4.clone()))
        }
        UH0::UH0_Nil => {
            v1.clone()
        }
    }
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method27(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v12, mut v13): (Rc<str>, Rc<str>) = match &*v0 {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let (mut v5, mut v6): (Rc<str>, Rc<str>) = method27(v4.clone(), v1.clone(), v2.clone());
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v6));
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            (v8.clone(), v9.clone())
        }
        _ => {
            let (mut v10, mut v11): (Rc<str>, Rc<str>) = match &*v0 {
                UH0::UH0_Nil => {
                    (v1.clone(), v2.clone())
                }
                _ => unreachable!(),
            };
            (v10.clone(), v11.clone())
        }
    };
    (v12.clone(), v13.clone())
}
fn env_drift_0() -> (bool, Rc<str>) {
    let mut v0: US0 = attached_names_1();
    let mut v1: US0 = detached_names_12();
    match &v0 {
        US0::US0_Some(v2) => {
            let mut v2: Rc<UH0> = v2.clone();
            match &v1 {
                US0::US0_Some(v3) => {
                    let mut v3: Rc<UH0> = v3.clone();
                    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v5: Rc<UH0> = method20(v2.clone(), v3.clone(), v4.clone());
                    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v7: Rc<UH0> = method22(v3.clone(), v2.clone(), v6.clone());
                    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v9: Rc<UH0> = method23(v5.clone(), v8.clone());
                    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v11: Rc<UH0> = method24(v7.clone(), v10.clone());
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH0> = method25(v9.clone(), v12.clone());
                    let mut v14: Rc<UH0> = method26(v11.clone(), v13.clone());
                    let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v16: Rc<UH0> = method25(v14.clone(), v15.clone());
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let (mut v18, mut v19): (Rc<str>, Rc<str>) = method27(v16.clone(), v17.clone(), v17.clone());
                    (true, v18.clone())
                }
                _ => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, v20.clone())
                }
            }
        }
        _ => {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v23.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>) {
        env_drift_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_doctor_env_drift() -> (bool, Rc<str>) {
    closure0()()
}
