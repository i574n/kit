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
pub struct Mut0 { pub l0: Rc<str> }
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
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: bool = &*v2 == "";
                let v6: Rc<UH1> = if v4 {
                    v1
                } else {
                    Rc::new(UH1::UH1_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH1>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method7(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn serial_ran_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<UH0>, mut v3: i32, mut v4: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v12, v13) => {
                let mut v12: Rc<UH1> = v12;
                let v13: Rc<UH0> = v13;
                let mut v14: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<UH1> = Rc::new(UH1::UH1_Cons(v16, v17));
                let v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v15, v18));
                let v20: Rc<UH1> = method2(v12, v19);
                let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v22: Rc<UH1> = method3(v20, v21);
                method4(v14.clone(), v22);
                let mut v23: Rc<std::io::Result<std::process::Output>> = Rc::new(v14.borrow_mut().output());
                let v24: bool = v23.is_ok();
                let (mut v30, mut v31, mut v32): (bool, i32, Rc<str>) = if v24 {
                    let v25: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v23).as_ref().unwrap().stderr).as_ref());
                    let v26: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v23).as_ref().unwrap().stdout).as_ref());
                    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
                    let v28: i32 = (&*v23).as_ref().unwrap().status.code().unwrap_or(-2);
                    (true, v28, v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (false, -1i32, v29)
                };
                let v34: bool = if v30 {
                    let v33: bool = v31 == 0i32;
                    v33
                } else {
                    false
                };
                let v37: US1 = if v34 {
                    US1::US1_Some(v32)
                } else {
                    US1::US1_None
                };
                match v37 {
                    US1::US1_None => {
                        let v49: i32 = v3.wrapping_add(1i32);
                        let v50: Rc<str> = format_real_5(v49);
                        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain link "); } LIT.with(|lit| lit.clone()) }, v50));
                        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed"); } LIT.with(|lit| lit.clone()) }));
                        eprintln!("{}", v52);
                        let v54: bool = match &v1 {
                            US0::US0_AfterAny => {
                                true
                            }
                            US0::US0_AfterOk => {
                                false
                            }
                        };
                        if v54 {
                            let v55: i32 = v4.wrapping_add(1i32);
                            (v2, v3, v4) = (v13, v49, v55);
                            continue;
                        } else {
                            let v57: i32 = 0i32;
                            let v58: i32 = method7(v13, v57);
                            let v59: Rc<str> = format_real_5(v58);
                            let v60: Rc<str> = format_real_5(v49);
                            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ran="); } LIT.with(|lit| lit.clone()) }, v60));
                            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed=1 skipped="); } LIT.with(|lit| lit.clone()) }));
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v59));
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain stopped serial "); } LIT.with(|lit| lit.clone()) }, v63));
                            println!("{}", v64);
                            return 1i32;
                        }
                    }
                    US1::US1_Some(v38) => {
                        let v38: Rc<str> = v38;
                        let v39: i32 = (v38.clone().len() as i32);
                        let v40: i32 = 0i32;
                        let v41: i32 = method8(v38.clone(), v39, v40);
                        let v42: i32 = v39.wrapping_sub(1i32);
                        let v43: Rc<str> = string_slice(&v38, v41 as i64, v42 as i64);
                        let v44: i32 = (v43.clone().len() as i32);
                        let v45: i32 = method9(v43.clone(), v44);
                        let v46: Rc<str> = string_slice(&v43, 0i32 as i64, v45 as i64);
                        println!("{}", v46);
                        let v47: i32 = v3.wrapping_add(1i32);
                        (v2, v3) = (v13, v47);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v5: Rc<str> = format_real_5(v3);
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain ok serial ran="); } LIT.with(|lit| lit.clone()) }, v5));
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed="); } LIT.with(|lit| lit.clone()) }));
                let v8: Rc<str> = format_real_5(v4);
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                println!("{}", v9);
                let v10: bool = v4 == 0i32;
                if v10 {
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
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_12(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_11(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 <= v0;
        if v3 {
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method3(v2, v4);
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
            let mut v14: US1 = if v10 {
                let mut v11: Rc<str> = command_line_argument_12(v0);
                US1::US1_Some(v11)
            } else {
                US1::US1_None
            };
            let v18: Rc<str> = match v14 {
                US1::US1_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US1::US1_Some(v15) => {
                    let v15: Rc<str> = v15;
                    v15
                }
            };
            let v19: Rc<UH1> = Rc::new(UH1::UH1_Cons(v18, v2));
            (v0, v2) = (v6, v19);
            continue;
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2;
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
fn segments_13(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8;
                let v9: Rc<UH1> = v9;
                let v10: bool = &*v8 == "--";
                if v10 {
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH1> = method3(v1, v12);
                    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v2));
                    (v0, v1, v2) = (v9, v11, v14);
                    continue;
                } else {
                    let v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8, v1));
                    (v0, v1) = (v9, v16);
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH1> = method3(v1, v3);
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                return method14(v2, v6);
            }
        }
    }
}
fn head_from_15(mut v0: Rc<UH1>, mut v1: US0, mut v2: Rc<str>, mut v3: bool, mut v4: bool) -> US3 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v6, v7) => {
                let v6: Rc<str> = v6;
                let v7: Rc<UH1> = v7;
                let v8: bool = &*v6 == "--ok";
                if v8 {
                    let v9: US0 = US0::US0_AfterOk;
                    (v0, v1) = (v7, v9);
                    continue;
                } else {
                    let mut v11: bool = &*v6 == "--serial";
                    if v11 {
                        let mut v12: bool = true;
                        (v0, v3) = (v7, v12);
                        continue;
                    } else {
                        let mut v14: bool = &*v6 == "--run-serial";
                        if v14 {
                            let v15: bool = true;
                            (v0, v4) = (v7, v15);
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
                                (v0, v2) = (v7, v6);
                                continue;
                            } else {
                                return US3::US3_None;
                            }
                        }
                    }
                }
            }
            UH1::UH1_Nil => {
                return US3::US3_Some(v1, v2, v3, v4);
            }
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
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
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = 0i32;
                let v5: i32 = method17(v2, v4);
                let v6: bool = v5 < 3i32;
                if v6 {
                    return v1;
                } else {
                    let v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v3, v7);
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
        let mut v1: Rc<str> = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_18() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method19(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US1> = closure1();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let mut v14: US1 = US1::US1_None;
    let v15: US1 = v14.clone();
    let v16: US1 = v12.unwrap_or(v15);
    match v16 {
        US1::US1_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US1::US1_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn serial_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = command_line_argument_count_10();
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = arguments_from_11(v0, v1, v2);
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = segments_13(v3, v4, v5);
    let v44: US2 = match Rc::unwrap_or_clone(v6) {
        UH0::UH0_Cons(v9, v10) => {
            let v9: Rc<UH1> = v9;
            let v10: Rc<UH0> = v10;
            let mut v11: US0 = US0::US0_AfterAny;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: bool = false;
            let mut v14: bool = false;
            let v15: US3 = head_from_15(v9, v11, v12, v13, v14);
            match v15 {
                US3::US3_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                    US2::US2_ChainRefused(v16)
                }
                US3::US3_Some(v18, v19, v20, v21) => {
                    let v18: US0 = v18;
                    let v19: Rc<str> = v19;
                    let v20: bool = v20;
                    let v21: bool = v21;
                    let v22: i32 = 1i32;
                    let mut v23: i32 = short_link_16(v10.clone(), v22);
                    let v24: i32 = 0i32;
                    let v25: i32 = method7(v10.clone(), v24);
                    let v26: bool = v25 == 0i32;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        US2::US2_ChainRefused(v27)
                    } else {
                        let v29: bool = v23 > 0i32;
                        if v29 {
                            let v30: Rc<str> = format_real_5(v23);
                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link "); } LIT.with(|lit| lit.clone()) }, v30));
                            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) }));
                            US2::US2_ChainRefused(v32)
                        } else {
                            let v34: bool = v20 && v21;
                            if v34 {
                                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--serial and --run-serial exclude each other"); } LIT.with(|lit| lit.clone()) };
                                US2::US2_ChainRefused(v35)
                            } else {
                                US2::US2_ChainPlanned(v18, v19, v20, v21, v10)
                            }
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
            US2::US2_ChainRefused(v7)
        }
    };
    match v44 {
        US2::US2_ChainPlanned(v47, v48, v49, v50, v51) => {
            let v47: US0 = v47;
            let v48: Rc<str> = v48;
            let v49: bool = v49;
            let v50: bool = v50;
            let v51: Rc<UH0> = v51;
            let v52: Rc<str> = current_executable_18();
            let v53: i32 = 0i32;
            let v54: i32 = 0i32;
            serial_ran_1(v52, v47, v51, v53, v54)
        }
        US2::US2_ChainRefused(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain refused: "); } LIT.with(|lit| lit.clone()) }, v45));
            eprintln!("{}", v46);
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
pub type SpiralShape_97df63ebabc3 = Mut0;
