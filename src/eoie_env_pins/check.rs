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
    UH0_Cons(Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Matches,
    US1_Shadowed(Rc<str>),
    US1_Elsewhere(Rc<str>),
    US1_Missing,
}
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method5(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn pins_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        let mut v4: bool = v0 >= v1;
        let mut v6: bool = if v4 {
            true
        } else {
            let mut v5: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5
        };
        if v6 {
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v8: Rc<UH0> = method4(v2.clone(), v7.clone());
            return (v8.clone(), v3.clone());
        } else {
            let mut v9: bool = v0 >= 0i32;
            let mut v12: bool = if v9 {
                let mut v10: i32 = command_line_argument_count_1();
                let mut v11: bool = v0 < v10;
                v11
            } else {
                false
            };
            let mut v16: US0 = if v12 {
                let mut v13: Rc<str> = command_line_argument_2(v0);
                US0::US0_Some(v13.clone())
            } else {
                US0::US0_None
            };
            let mut v20: Rc<str> = match &v16 {
                US0::US0_None => {
                    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18.clone()
                }
                US0::US0_Some(v17) => {
                    let mut v17: Rc<str> = v17.clone();
                    v17.clone()
                }
            };
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let mut v22: Option<usize> = v20.find(&*v21);
            let mut v23: Option<usize> = method5(v22.clone());
            let mut v24: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v25: Option<i32> = v23.map(|x| v24(x));
            let mut v27: i32 = v25.unwrap_or(-1i32);
            let mut v28: bool = v27 <= 0i32;
            let (mut v41, mut v42): (Rc<str>, Rc<str>) = if v28 {
                let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v20.clone(), v29.clone())
            } else {
                let mut v30: usize = ((0i32) as usize);
                let mut v31: usize = ((v27) as usize);
                let mut v32: Rc<str> = Rc::<str>::from(&(v20)[v30..v31]);
                let mut v33: i32 = v27.wrapping_add(1i32);
                let mut v34: i32 = (v20.clone().len() as i32);
                let mut v35: i32 = v34.wrapping_sub(v27);
                let mut v36: i32 = v35.wrapping_sub(1i32);
                let mut v37: usize = ((v33) as usize);
                let mut v38: i32 = v33.wrapping_add(v36);
                let mut v39: usize = ((v38) as usize);
                let mut v40: Rc<str> = Rc::<str>::from(&(v20)[v37..v39]);
                (v32.clone(), v40.clone())
            };
            let mut v43: bool = v42.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v43 {
                let mut v46: bool = if v9 {
                    let mut v44: i32 = command_line_argument_count_1();
                    let mut v45: bool = v0 < v44;
                    v45
                } else {
                    false
                };
                let mut v50: US0 = if v46 {
                    let mut v47: Rc<str> = command_line_argument_2(v0);
                    US0::US0_Some(v47.clone())
                } else {
                    US0::US0_None
                };
                let mut v54: Rc<str> = match &v50 {
                    US0::US0_None => {
                        let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v52.clone()
                    }
                    US0::US0_Some(v51) => {
                        let mut v51: Rc<str> = v51.clone();
                        v51.clone()
                    }
                };
                let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expects tool=pinned-path, received "); } LIT.with(|lit| lit.clone()) };
                let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
                return (v2.clone(), v56.clone());
            } else {
                let mut v57: i32 = v0.wrapping_add(1i32);
                let mut v58: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41.clone(), v42.clone(), v2.clone()));
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v0, v1, v2, v3) = (v57, v1, v58.clone(), v59.clone());
                continue;
            }
        }
    }
}
fn get_environment_variable_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1.clone()
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn loop_10(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn file_exists_11(mut v0: Rc<str>) -> bool {
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
fn found_on_path_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v4: bool = v1 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = loop_9(v5.clone());
            let mut v7: Rc<str> = loop_10(v6.clone());
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
            let mut v11: i32 = v1.wrapping_sub(1i32);
            let mut v12: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = if v12 {
                file_exists_11(v10.clone())
            } else {
                false
            };
            let mut v16: Rc<UH1> = if v14 {
                Rc::new(UH1::UH1_Cons(v10.clone(), v3.clone()))
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v11, v2.clone(), v16.clone());
            continue;
        }
    }
}
fn listed_13(mut v0: Rc<UH1>, mut v1: Rc<str>) -> bool {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(v2.replace(&*v4, &*v5));
                let mut v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
                let mut v8: Rc<str> = Rc::<str>::from(v1.replace(&*v4, &*v5));
                let mut v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
                let mut v10: bool = v7.clone() == v9.clone();
                if v10 {
                    return true;
                } else {
                    (v0, v1) = (v3.clone(), v1.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return false;
            }
        }
    }
}
fn drift_of_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH1>) -> US1 {
    match &*v2 {
        UH1::UH1_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(v4.replace(&*v6, &*v7));
            let mut v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
            let mut v10: Rc<str> = Rc::<str>::from(v1.replace(&*v6, &*v7));
            let mut v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
            let mut v12: bool = v9.clone() == v11.clone();
            if v12 {
                US1::US1_Matches
            } else {
                let mut v14: bool = listed_13(v2.clone(), v1.clone());
                if v14 {
                    US1::US1_Shadowed(v4.clone())
                } else {
                    US1::US1_Elsewhere(v4.clone())
                }
            }
        }
        UH1::UH1_Nil => {
            US1::US1_Missing
        }
    }
}
fn checked_7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v4, v5, v6) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: i32 = (v0.clone().borrow().len() as i32);
                let mut v8: i32 = v7.wrapping_sub(1i32);
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v10: bool = v4.contains(&*v9);
                let mut v16: Rc<str> = if v10 {
                    v4.clone()
                } else {
                    let mut v11: bool = cfg!(windows);
                    let mut v14: Rc<str> = if v11 {
                        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
                        v12.clone()
                    } else {
                        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v13.clone()
                    };
                    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v14));
                    v15.clone()
                };
                let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v18: Rc<UH1> = found_on_path_8(v0.clone(), v8, v16.clone(), v17.clone());
                let mut v19: US1 = drift_of_12(v4.clone(), v5.clone(), v18.clone());
                let mut v48: Rc<str> = match &v19 {
                    US1::US1_Elsewhere(v30) => {
                        let mut v30: Rc<str> = v30.clone();
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unpinned: PATH finds "); } LIT.with(|lit| lit.clone()) };
                        let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v31));
                        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v30));
                        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", and the pinned "); } LIT.with(|lit| lit.clone()) };
                        let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v5));
                        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not on PATH"); } LIT.with(|lit| lit.clone()) };
                        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                        v38.clone()
                    }
                    US1::US1_Matches => {
                        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok "); } LIT.with(|lit| lit.clone()) };
                        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v20));
                        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v5));
                        v22.clone()
                    }
                    US1::US1_Missing => {
                        let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing: no "); } LIT.with(|lit| lit.clone()) };
                        let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v39));
                        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v4));
                        let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" on PATH, pinned "); } LIT.with(|lit| lit.clone()) };
                        let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                        let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v5));
                        v44.clone()
                    }
                    US1::US1_Shadowed(v23) => {
                        let mut v23: Rc<str> = v23.clone();
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shadowed: "); } LIT.with(|lit| lit.clone()) };
                        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v24));
                        let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v23));
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" comes first on PATH, pinned "); } LIT.with(|lit| lit.clone()) };
                        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v5));
                        v29.clone()
                    }
                };
                let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v48));
                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                let mut v52: bool = match &v19 {
                    US1::US1_Matches => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v54: u64 = if v52 {
                    v3
                } else {
                    let mut v53: u64 = v3.wrapping_add(1u64);
                    v53
                };
                (v0, v1, v2, v3) = (v0.clone(), v6.clone(), v51.clone(), v54);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2.clone(), v3);
            }
        }
    }
}
fn loop_14(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
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
fn method15(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method17(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_16(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn env_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = command_line_argument_count_1();
    let mut v2: bool = 1i32 < v1;
    let mut v7: US0 = if v2 {
        let mut v3: i32 = 1i32;
        let mut v4: Rc<str> = command_line_argument_2(v3);
        US0::US0_Some(v4.clone())
    } else {
        US0::US0_None
    };
    let mut v11: Rc<str> = match &v7 {
        US0::US0_None => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8.clone();
            v8.clone()
        }
    };
    let mut v12: bool = v11.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let mut v14: bool = if v12 {
        true
    } else {
        let mut v13: bool = v0 < 3i32;
        v13
    };
    if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: env "); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: env usage: env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v17);
        2i32
    } else {
        let mut v18: i32 = 2i32;
        let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v21, mut v22): (Rc<UH0>, Rc<str>) = pins_from_3(v18, v0, v19.clone(), v20.clone());
        let mut v23: bool = v22.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v23 {
            let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: env check "); } LIT.with(|lit| lit.clone()) };
            let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v22));
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            eprintln!("{}", v29);
            2i32
        } else {
            let mut v30: bool = cfg!(windows);
            let mut v33: Rc<str> = if v30 {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
                v31.clone()
            } else {
                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                v32.clone()
            };
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PATH"); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = get_environment_variable_6(v34.clone());
            let mut v36: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v35.split(&*v33).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v37: u64 = 0u64;
            let (mut v38, mut v39): (Rc<str>, u64) = checked_7(v36.clone(), v21.clone(), v20.clone(), v37);
            let mut v40: Rc<str> = loop_14(v38.clone());
            println!("{}", v40);
            let mut v41: bool = v39 == 0u64;
            let mut v44: Rc<str> = if v41 {
                let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v42.clone()
            } else {
                let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drift"); } LIT.with(|lit| lit.clone()) };
                v43.clone()
            };
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie env check "); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
            let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pins="); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
            let mut v49: i32 = 0i32;
            let mut v50: i32 = method15(v21.clone(), v49);
            let mut v51: Rc<str> = format_real_16(v50);
            let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v51));
            let mut v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" drifted="); } LIT.with(|lit| lit.clone()) };
            let mut v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
            let mut v55: Rc<str> = format_real_18(v39);
            let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
            eprintln!("{}", v56);
            if v41 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        env_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_env_run() -> i32 {
    closure0()()
}
