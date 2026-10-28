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
    US0_AfterAny,
    US0_AfterOk,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<UH1>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_ChainPlanned(US0, Rc<str>, bool, bool, Rc<UH0>),
    US2_ChainRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(US0, Rc<str>, bool, bool),
    US3_None,
}
fn method2(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let mut v4: bool = &*v2 == "";
                let v6: Rc<UH1> = if v4 {
                    v1.clone()
                } else {
                    Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()))
                };
                (v0, v1) = (v3.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method4(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH1>) -> () {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                drop(v0.borrow_mut().arg(&*v2));
                (v0, v1) = (v0.clone(), v3.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method6(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method7(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<UH1> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
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
                        let v8: bool = v4 == b'\n';
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
            let v3: i32 = v1.wrapping_sub(1i32);
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
                        let v8: bool = v4 == b'\n';
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
fn serial_ran_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<UH0>, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v14, v15) => {
                let v14: Rc<UH1> = v14.clone();
                let v15: Rc<UH0> = v15.clone();
                let mut v16: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v20: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18.clone(), v19.clone()));
                let v21: Rc<UH1> = Rc::new(UH1::UH1_Cons(v17.clone(), v20.clone()));
                let v22: Rc<UH1> = method2(v14.clone(), v21.clone());
                let v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v24: Rc<UH1> = method3(v22.clone(), v23.clone());
                method4(v16.clone(), v24.clone());
                let mut v25: Rc<std::io::Result<std::process::Output>> = Rc::new(v16.borrow_mut().output());
                let v26: bool = v25.is_ok();
                let (mut v32, mut v33, mut v34): (bool, i32, Rc<str>) = if v26 {
                    let v27: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v25).as_ref().unwrap().stderr).as_ref());
                    let v28: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v25).as_ref().unwrap().stdout).as_ref());
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
                    let v30: i32 = (&*v25).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v30, v29.clone())
                } else {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v31.clone())
                };
                let v36: bool = if v32 {
                    let v35: bool = v33 == 0i32;
                    v35
                } else {
                    false
                };
                let v39: US1 = if v36 {
                    US1::US1_Some(v34.clone())
                } else {
                    US1::US1_None
                };
                match &v39 {
                    US1::US1_None => {
                        let v51: i32 = v3.wrapping_add(1i32);
                        let v52: Rc<str> = format_real_5(v51);
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain link "); } LIT.with(|lit| lit.clone()) };
                        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v52));
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
                        eprintln!("{}", v56);
                        let v58: bool = match &v1 {
                            US0::US0_AfterAny => {
                                true
                            }
                            US0::US0_AfterOk => {
                                false
                            }
                        };
                        if v58 {
                            let v59: i32 = v4.wrapping_add(1i32);
                            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v15.clone(), v51, v59);
                            continue;
                        } else {
                            let v61: i32 = 0i32;
                            let v62: i32 = method7(v15.clone(), v61);
                            let v63: Rc<str> = format_real_5(v62);
                            let v64: Rc<str> = format_real_5(v51);
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ran="); } LIT.with(|lit| lit.clone()) };
                            let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v64));
                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed=1 skipped="); } LIT.with(|lit| lit.clone()) };
                            let v69: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v66, v67)), v63));
                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain stopped serial "); } LIT.with(|lit| lit.clone()) };
                            let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v69));
                            println!("{}", v71);
                            return 1i32;
                        }
                    }
                    US1::US1_Some(v40) => {
                        let v40: Rc<str> = v40.clone();
                        let v41: i32 = (v40.clone().len() as i32);
                        let v42: i32 = 0i32;
                        let v43: i32 = method8(v40.clone(), v41, v42);
                        let v44: i32 = v41.wrapping_sub(1i32);
                        let v45: Rc<str> = string_slice(&v40.clone(), v43 as i64, v44 as i64);
                        let v46: i32 = (v45.clone().len() as i32);
                        let v47: i32 = method9(v45.clone(), v46);
                        let v48: Rc<str> = string_slice(&v45.clone(), 0i32 as i64, v47 as i64);
                        println!("{}", v48);
                        let v49: i32 = v3.wrapping_add(1i32);
                        (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v15.clone(), v49, v4);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v5: Rc<str> = format_real_5(v3);
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain ok serial ran="); } LIT.with(|lit| lit.clone()) };
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed="); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                let v10: Rc<str> = format_real_5(v4);
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                println!("{}", v11);
                let mut v12: bool = v4 == 0i32;
                if v12 {
                    return 0i32;
                } else {
                    return 1i32;
                }
            }
        }
    }
}
fn command_line_argument_count_10() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_12(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_11(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 <= v0;
        if v3 {
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method3(v2.clone(), v4.clone());
        } else {
            let v6: i32 = v0.wrapping_add(1i32);
            let v7: bool = v0 >= 0i32;
            let v10: bool = if v7 {
                let v8: i32 = command_line_argument_count_10();
                let v9: bool = v0 < v8;
                v9
            } else {
                false
            };
            let v14: US1 = if v10 {
                let mut v11: Rc<str> = command_line_argument_12(v0);
                US1::US1_Some(v11.clone())
            } else {
                US1::US1_None
            };
            let v18: Rc<str> = match &v14 {
                US1::US1_None => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US1::US1_Some(v15) => {
                    let v15: Rc<str> = v15.clone();
                    v15.clone()
                }
            };
            let v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18.clone(), v2.clone()));
            (v0, v1, v2) = (v6, v1, v19.clone());
            continue;
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<UH1> = v2.clone();
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
fn segments_13(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let v9: Rc<UH1> = v9.clone();
                let v10: bool = &*v8 == "--";
                if v10 {
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH1> = method3(v1.clone(), v12.clone());
                    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v2.clone()));
                    (v0, v1, v2) = (v9.clone(), v11.clone(), v14.clone());
                    continue;
                } else {
                    let mut v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8.clone(), v1.clone()));
                    (v0, v1, v2) = (v9.clone(), v16.clone(), v2.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH1> = method3(v1.clone(), v3.clone());
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                return method14(v2.clone(), v6.clone());
            }
        }
    }
}
fn head_from_15(mut v0: Rc<UH1>, mut v1: US0, mut v2: Rc<str>, mut v3: bool, mut v4: bool) -> US3 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v6, v7) => {
                let v6: Rc<str> = v6.clone();
                let v7: Rc<UH1> = v7.clone();
                let v8: bool = &*v6 == "--ok";
                if v8 {
                    let v9: US0 = US0::US0_AfterOk;
                    (v0, v1, v2, v3, v4) = (v7.clone(), v9.clone(), v2.clone(), v3, v4);
                    continue;
                } else {
                    let mut v11: bool = &*v6 == "--serial";
                    if v11 {
                        let mut v12: bool = true;
                        (v0, v1, v2, v3, v4) = (v7.clone(), v1.clone(), v2.clone(), v12, v4);
                        continue;
                    } else {
                        let v14: bool = &*v6 == "--run-serial";
                        if v14 {
                            let v15: bool = true;
                            (v0, v1, v2, v3, v4) = (v7.clone(), v1.clone(), v2.clone(), v3, v15);
                            continue;
                        } else {
                            let v17: bool = &*v2 == "";
                            let v20: bool = if v17 {
                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
                                let v19: bool = v6.starts_with(&*v18);
                                v19
                            } else {
                                false
                            };
                            if v20 {
                                (v0, v1, v2, v3, v4) = (v7.clone(), v1.clone(), v6.clone(), v3, v4);
                                continue;
                            } else {
                                return US3::US3_None;
                            }
                        }
                    }
                }
            }
            UH1::UH1_Nil => {
                return US3::US3_Some(v1.clone(), v2.clone(), v3, v4);
            }
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn short_link_16(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<UH1> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = 0i32;
                let v5: i32 = method17(v2.clone(), v4);
                let v6: bool = v5 < 3i32;
                if v6 {
                    return v1;
                } else {
                    let v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v3.clone(), v7);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return 0i32;
            }
        }
    }
}
fn method19(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US1> = Rc::new(move |mut v0: (Rc<str>)| -> US1 {
        let v1: Rc<str> = (v0);
        US1::US1_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_18() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method19(v9.clone());
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let v14: US1 = US1::US1_None;
    let v15: US1 = v14.clone();
    let mut v16: US1 = v12.unwrap_or(v15);
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
fn serial_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: i32 = command_line_argument_count_10();
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = arguments_from_11(v0, v1, v2.clone());
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = segments_13(v3.clone(), v4.clone(), v5.clone());
    let v48: US2 = match &*v6 {
        UH0::UH0_Cons(v9, v10) => {
            let v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let mut v11: US0 = US0::US0_AfterAny;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: bool = false;
            let v14: bool = false;
            let v15: US3 = head_from_15(v9.clone(), v11.clone(), v12.clone(), v13, v14);
            match &v15 {
                US3::US3_None => {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ChainRefused(v16.clone())
                }
                US3::US3_Some(v18, v19, v20, v21) => {
                    let v18: US0 = v18.clone();
                    let v19: Rc<str> = v19.clone();
                    let v20: bool = *v20;
                    let v21: bool = *v21;
                    let v22: i32 = 1i32;
                    let v23: i32 = short_link_16(v10.clone(), v22);
                    let v24: i32 = 0i32;
                    let mut v25: i32 = method7(v10.clone(), v24);
                    let v26: bool = v25 == 0i32;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; "); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        US2::US2_ChainRefused(v29.clone())
                    } else {
                        let v31: bool = v23 > 0i32;
                        if v31 {
                            let v32: Rc<str> = format_real_5(v23);
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link "); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) };
                            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                            US2::US2_ChainRefused(v36.clone())
                        } else {
                            let v38: bool = v20 && v21;
                            if v38 {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--serial and --run-serial exclude each other"); } LIT.with(|lit| lit.clone()) };
                                US2::US2_ChainRefused(v39.clone())
                            } else {
                                US2::US2_ChainPlanned(v18.clone(), v19.clone(), v20, v21, v10.clone())
                            }
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
            US2::US2_ChainRefused(v7.clone())
        }
    };
    match &v48 {
        US2::US2_ChainPlanned(v52, v53, v54, v55, v56) => {
            let v52: US0 = v52.clone();
            let v53: Rc<str> = v53.clone();
            let v54: bool = *v54;
            let v55: bool = *v55;
            let v56: Rc<UH0> = v56.clone();
            let v57: Rc<str> = current_executable_18();
            let v58: i32 = 0i32;
            let v59: i32 = 0i32;
            serial_ran_1(v57.clone(), v52.clone(), v56.clone(), v58, v59)
        }
        US2::US2_ChainRefused(v49) => {
            let v49: Rc<str> = v49.clone();
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain refused: "); } LIT.with(|lit| lit.clone()) };
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
            eprintln!("{}", v51);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        serial_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_serial_run() -> i32 {
    closure0()()
}
