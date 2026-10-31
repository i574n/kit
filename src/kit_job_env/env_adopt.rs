#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_SnapshotUsable,
    US0_SnapshotEmpty,
    US0_SnapshotDropsRoot(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Ok,
    US2_Error(Rc<str>),
}
fn joined_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_4() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_4();
    joined_2(v1, v0)
}
fn job_file_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_3(v0);
    joined_2(v2, v1)
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn method7(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v5: Rc<str> = get_environment_variable_8(v3.clone());
                let mut v6: bool = &*v5 != "";
                let v19: bool = if v6 {
                    let v7: Option<char> = char::from_u32(0u32);
                    let v9: char = v7.unwrap();
                    let v10: Rc<str> = Rc::<str>::from(format!("{}", v9));
                    let v11: Rc<str> = Rc::<str>::from(v3.to_lowercase());
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    let v15: Rc<str> = Rc::<str>::from(v0.to_lowercase());
                    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v15));
                    let mut v17: bool = v16.contains(&*v14);
                    let mut v18: bool = v17 == false;
                    v18
                } else {
                    false
                };
                if v19 {
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
fn method11(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn value_in_12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: i32) -> US1 {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return US1::US1_None;
        } else {
            let mut v6: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let v7: i32 = (v6.clone().len() as i32);
            let v8: bool = v7 < 2i32;
            let v21: i32 = if v8 {
                -1i32
            } else {
                let v9: i32 = v7.wrapping_sub(1i32);
                let v10: usize = ((1i32) as usize);
                let v11: i32 = 1i32.wrapping_add(v9);
                let v12: usize = ((v11) as usize);
                let v13: Rc<str> = Rc::<str>::from(&(v6)[v10..v12]);
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v15: Option<usize> = v13.find(&*v14);
                let mut v16: Option<usize> = method11(v15.clone());
                let mut v17: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v18: Option<i32> = v16.map(|x| v17(x));
                let v20: i32 = v18.unwrap_or(-1i32);
                v20
            };
            let v22: bool = v21 < 0i32;
            let (mut v35, mut v36): (Rc<str>, Rc<str>) = if v22 {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v23.clone(), v23.clone())
            } else {
                let v24: i32 = v21.wrapping_add(1i32);
                let v25: usize = ((0i32) as usize);
                let v26: usize = ((v24) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v6)[v25..v26]);
                let v28: i32 = v21.wrapping_add(2i32);
                let v29: i32 = v7.wrapping_sub(v21);
                let v30: i32 = v29.wrapping_sub(2i32);
                let v31: usize = ((v28) as usize);
                let v32: i32 = v28.wrapping_add(v30);
                let v33: usize = ((v32) as usize);
                let v34: Rc<str> = Rc::<str>::from(&(v6)[v31..v33]);
                (v27.clone(), v34.clone())
            };
            let v37: bool = &*v35 == &*v1;
            let v43: bool = if v37 {
                let v38: bool = &*v35 != "";
                if v38 {
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let mut v40: bool = v35.starts_with(&*v39);
                    let v41: bool = v40 == false;
                    v41
                } else {
                    false
                }
            } else {
                false
            };
            if v43 {
                return US1::US1_Some(v36.clone());
            } else {
                let v45: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v45);
                continue;
            }
        }
    }
}
fn set_changed_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32) -> () {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 < v3;
        if v4 {
            let v5: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let v7: bool = v6 < 2i32;
            let v20: i32 = if v7 {
                -1i32
            } else {
                let v8: i32 = v6.wrapping_sub(1i32);
                let v9: usize = ((1i32) as usize);
                let v10: i32 = 1i32.wrapping_add(v8);
                let v11: usize = ((v10) as usize);
                let v12: Rc<str> = Rc::<str>::from(&(v5)[v9..v11]);
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v14: Option<usize> = v12.find(&*v13);
                let v15: Option<usize> = method11(v14.clone());
                let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v17: Option<i32> = v15.map(|x| v16(x));
                let v19: i32 = v17.unwrap_or(-1i32);
                v19
            };
            let v21: bool = v20 < 0i32;
            let (mut v34, mut v35): (Rc<str>, Rc<str>) = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v22.clone(), v22.clone())
            } else {
                let v23: i32 = v20.wrapping_add(1i32);
                let v24: usize = ((0i32) as usize);
                let v25: usize = ((v23) as usize);
                let v26: Rc<str> = Rc::<str>::from(&(v5)[v24..v25]);
                let v27: i32 = v20.wrapping_add(2i32);
                let v28: i32 = v6.wrapping_sub(v20);
                let v29: i32 = v28.wrapping_sub(2i32);
                let v30: usize = ((v27) as usize);
                let v31: i32 = v27.wrapping_add(v29);
                let v32: usize = ((v31) as usize);
                let v33: Rc<str> = Rc::<str>::from(&(v5)[v30..v32]);
                (v26.clone(), v33.clone())
            };
            let mut v36: bool = &*v34 != "";
            let mut v40: bool = if v36 {
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v38: bool = v34.starts_with(&*v37);
                let mut v39: bool = v38 == false;
                v39
            } else {
                false
            };
            let v47: bool = if v40 {
                let v41: i32 = 0i32;
                let v42: US1 = value_in_12(v1.clone(), v34.clone(), v41);
                let v45: bool = match &v42 {
                    US1::US1_Some(v43) => {
                        let v43: Rc<str> = v43.clone();
                        let v44: bool = &*v43 == &*v35;
                        v44
                    }
                    _ => {
                        false
                    }
                };
                let v46: bool = v45 != true;
                v46
            } else {
                false
            };
            if v47 {
                unsafe { std::env::set_var(&*v34, &*v35) };
                ()
            };
            let v48: i32 = v2.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v1.clone(), v48);
            continue;
        }
        return ();
    }
}
fn method13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: bool) -> bool {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v14: bool = if v3 {
                    let mut v6: Rc<str> = get_environment_variable_8(v4.clone());
                    let v7: bool = &*v6 == "";
                    if v7 {
                        true
                    } else {
                        let v8: Rc<str> = Rc::<str>::from(v4.to_lowercase());
                        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v8));
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                        let v12: bool = v1.contains(&*v11);
                        v12
                    }
                } else {
                    false
                };
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v5.clone(), v14);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn environment_snapshot_14() -> Rc<str> {
    let v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0.clone()
}
fn removed_extras_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<str>, mut v3: i32) -> () {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v3 < v4;
        if v5 {
            let mut v6: Rc<str> = v0.clone().borrow()[v3 as usize].clone();
            let v7: i32 = (v6.clone().len() as i32);
            let v8: bool = v7 < 2i32;
            let v21: i32 = if v8 {
                -1i32
            } else {
                let v9: i32 = v7.wrapping_sub(1i32);
                let v10: usize = ((1i32) as usize);
                let v11: i32 = 1i32.wrapping_add(v9);
                let v12: usize = ((v11) as usize);
                let v13: Rc<str> = Rc::<str>::from(&(v6)[v10..v12]);
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v15: Option<usize> = v13.find(&*v14);
                let mut v16: Option<usize> = method11(v15.clone());
                let mut v17: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v18: Option<i32> = v16.map(|x| v17(x));
                let v20: i32 = v18.unwrap_or(-1i32);
                v20
            };
            let v22: bool = v21 < 0i32;
            let (mut v35, mut v36): (Rc<str>, Rc<str>) = if v22 {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v23.clone(), v23.clone())
            } else {
                let v24: i32 = v21.wrapping_add(1i32);
                let v25: usize = ((0i32) as usize);
                let v26: usize = ((v24) as usize);
                let v27: Rc<str> = Rc::<str>::from(&(v6)[v25..v26]);
                let v28: i32 = v21.wrapping_add(2i32);
                let v29: i32 = v7.wrapping_sub(v21);
                let v30: i32 = v29.wrapping_sub(2i32);
                let v31: usize = ((v28) as usize);
                let v32: i32 = v28.wrapping_add(v30);
                let v33: usize = ((v32) as usize);
                let v34: Rc<str> = Rc::<str>::from(&(v6)[v31..v33]);
                (v27.clone(), v34.clone())
            };
            let v37: bool = &*v35 != "";
            let v41: bool = if v37 {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let mut v39: bool = v35.starts_with(&*v38);
                let mut v40: bool = v39 == false;
                v40
            } else {
                false
            };
            let v44: bool = if v41 {
                let v42: bool = v35.starts_with(&*v2);
                let v43: bool = v42 == false;
                v43
            } else {
                false
            };
            let v48: bool = if v44 {
                let v45: i32 = 0i32;
                let v46: US1 = value_in_12(v1.clone(), v35.clone(), v45);
                match &v46 {
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
            if v48 {
                unsafe { std::env::remove_var(&*v35) };
                ()
            };
            let v49: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2.clone(), v49);
            continue;
        }
        return ();
    }
}
fn environment_restored_9(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let v4: Rc<str> = Rc::<str>::from(v3.to_lowercase());
    let v5: bool = cfg!(windows);
    let v38: Rc<UH0> = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATH"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TEMP"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TMP"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12.clone(), v13.clone()));
        let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v14.clone()));
        let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10.clone(), v15.clone()));
        let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v16.clone()));
        let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v17.clone()));
        let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v18.clone()));
        Rc::new(UH0::UH0_Cons(v6.clone(), v19.clone()))
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATH"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TEMP"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TMP"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v30: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28.clone(), v29.clone()));
        let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27.clone(), v30.clone()));
        let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26.clone(), v31.clone()));
        let v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25.clone(), v32.clone()));
        let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24.clone(), v33.clone()));
        let mut v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23.clone(), v34.clone()));
        let mut v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22.clone(), v35.clone()));
        Rc::new(UH0::UH0_Cons(v21.clone(), v36.clone()))
    };
    let mut v39: bool = &*v0 != "";
    let v42: bool = if v39 {
        let mut v40: bool = true;
        method13(v2.clone(), v4.clone(), v38.clone(), v40)
    } else {
        false
    };
    let v43: bool = v42 == false;
    if v43 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env.environment_restored: refusing an empty or truncated snapshot"); } LIT.with(|lit| lit.clone()) }))
    };
    let v44: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v45: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v44).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v46: Rc<str> = std::rc::Rc::<str>::from(char::from(0u8).to_string());
    let v47: Rc<str> = environment_snapshot_14();
    let v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v47.split(&*v46).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v49: i32 = 0i32;
    removed_extras_15(v48.clone(), v45.clone(), v1.clone(), v49);
    let v50: i32 = 0i32;
    set_changed_10(v45.clone(), v48.clone(), v50)
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US2> = Rc::new(move || -> US2 {
        US2::US2_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn() -> US2> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US2> = Rc::new(move |mut v0: Rc<str>| -> US2 {
        US2::US2_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method19() -> Rc<dyn Fn(Rc<str>) -> US2> {
    error_c4()
}
fn job_environment_adopted_0(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = job_file_1(v0.clone(), v1.clone());
    let v3: bool = file_exists_5(v2.clone());
    if v3 {
        let v4: Rc<str> = text_or_empty_6(v2.clone());
        let v5: bool = &*v4 == "";
        let v28: US0 = if v5 {
            US0::US0_SnapshotEmpty
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATH"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TEMP"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TMP"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v14.clone()));
            let mut v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12.clone(), v15.clone()));
            let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v16.clone()));
            let mut v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10.clone(), v17.clone()));
            let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v18.clone()));
            let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v19.clone()));
            let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v20.clone()));
            let v22: US1 = method7(v4.clone(), v21.clone());
            match &v22 {
                US1::US1_None => {
                    US0::US0_SnapshotUsable
                }
                US1::US1_Some(v23) => {
                    let v23: Rc<str> = v23.clone();
                    US0::US0_SnapshotDropsRoot(v23.clone())
                }
            }
        };
        let v29: bool = match &v28 {
            US0::US0_SnapshotUsable => {
                true
            }
            _ => {
                false
            }
        };
        if v29 {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_JOB_"); } LIT.with(|lit| lit.clone()) };
            environment_restored_9(v4.clone(), v30.clone());
            let v31: bool = file_exists_16(v2.clone());
            let mut v35: Result<(), std::io::Error> = if v31 {
                let v33: Result<(), std::io::Error> = std::fs::remove_file(&*v2);
                v33
            } else {
                let v34: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                v34
            };
            let mut v36: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method17();
            let v38: Result<(), Rc<str>> = v35.map_err(|x| v36(x));
            let mut v39: Rc<dyn Fn() -> US2> = method18();
            let mut v40: Rc<dyn Fn(Rc<str>) -> US2> = method19();
            let v41: US2 = match v38 { Ok(()) => v39(), Err(e) => v40(e) };
            match &v41 {
                US2::US2_Error(v42) => {
                    let v42: Rc<str> = v42.clone();
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
                    eprintln!("{}", v44);
                    ()
                }
                US2::US2_Ok => {
                    ()
                }
            };
            v28.clone()
        } else {
            v28.clone()
        }
    } else {
        US0::US0_SnapshotUsable
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: US0 = job_environment_adopted_0(v0.clone());
        match &v1 {
            US0::US0_SnapshotDropsRoot(v4) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job environment not adopted: its snapshot has no "); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entry (empty or cut short), and restoring it would remove "); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v4));
                v9.clone()
            }
            US0::US0_SnapshotEmpty => {
                let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job environment not adopted: its snapshot is empty, and restoring it would remove every variable"); } LIT.with(|lit| lit.clone()) };
                v3.clone()
            }
            US0::US0_SnapshotUsable => {
                let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v2.clone()
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_environment_adopted(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
