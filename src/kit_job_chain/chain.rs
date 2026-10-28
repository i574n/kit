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
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_ChainPlanned(US0, Rc<str>, bool, bool, Rc<UH0>),
    US4_ChainRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(US0, Rc<str>, bool, bool),
    US5_None,
}
fn method4(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_from_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> US2 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return US2::US2_Some(v2);
        } else {
            let v6: usize = ((v1) as usize);
            let mut v7: i32 = v1.wrapping_add(1i32);
            let v8: usize = ((v7) as usize);
            let v9: Rc<str> = Rc::<str>::from(&(v0)[v6..v8]);
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
            let mut v11: Option<usize> = v10.find(&*v9);
            let mut v12: Option<usize> = method4(v11.clone());
            let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
            let mut v14: Option<i32> = v12.map(|x| v13(x));
            let v16: i32 = v14.unwrap_or(-1i32);
            let v17: bool = v2 > 214748364i32;
            let v21: bool = if v17 {
                true
            } else {
                let v18: bool = v2 == 214748364i32;
                if v18 {
                    let v19: bool = v16 > 7i32;
                    v19
                } else {
                    false
                }
            };
            let v22: bool = v16 < 0i32;
            let v23: bool = v22 || v21;
            if v23 {
                return US2::US2_None;
            } else {
                let v25: i32 = v2.wrapping_mul(10i32);
                let v26: i32 = v25.wrapping_add(v16);
                (v0, v1, v2) = (v0.clone(), v7, v26);
                continue;
            }
        }
    }
}
fn links_timeout_2(mut v0: Rc<UH0>, mut v1: u64) -> US1 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                match &*v3 {
                    UH1::UH1_Cons(v5, v6) => {
                        let v5: Rc<str> = v5.clone();
                        let v6: Rc<UH1> = v6.clone();
                        match &*v6 {
                            UH1::UH1_Cons(v7, v8) => {
                                let mut v7: Rc<str> = v7.clone();
                                let v8: Rc<UH1> = v8.clone();
                                let v9: bool = &*v7 == "";
                                let mut v14: US2 = if v9 {
                                    US2::US2_None
                                } else {
                                    let mut v11: i32 = 0i32;
                                    let mut v12: i32 = 0i32;
                                    decimal_from_3(v7.clone(), v11, v12)
                                };
                                match &v14 {
                                    US2::US2_None => {
                                        return US1::US1_None;
                                    }
                                    US2::US2_Some(v15) => {
                                        let v15: i32 = *v15;
                                        let v16: bool = v15 > 0i32;
                                        if v16 {
                                            let v17: u64 = (v15 as u64);
                                            let v18: u64 = v1.wrapping_add(v17);
                                            (v0, v1) = (v4.clone(), v18);
                                            continue;
                                        } else {
                                            return US1::US1_None;
                                        }
                                    }
                                }
                            }
                            _ => {
                                return US1::US1_None;
                            }
                        }
                    }
                    _ => {
                        return US1::US1_None;
                    }
                }
            }
            UH0::UH0_Nil => {
                return US1::US1_Some(v1);
            }
        }
    }
}
fn method6(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn list_joined_8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = list_joined_8(v3, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn links_flattened_7(mut v0: Rc<UH0>) -> Rc<UH1> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<UH1> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH1> = links_flattened_7(v3.clone());
            let v5: Rc<UH1> = list_joined_8(v2.clone(), v4.clone());
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            Rc::new(UH1::UH1_Cons(v6.clone(), v5.clone()))
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn method12(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH1>) -> () {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn captured_9(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US3 {
    let mut v2: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method10(v1.clone(), v3.clone());
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method11(v4.clone(), v5.clone());
    method12(v2.clone(), v6.clone());
    let mut v7: Rc<std::io::Result<std::process::Output>> = Rc::new(v2.borrow_mut().output());
    let v8: bool = v7.is_ok();
    let (mut v14, mut v15, mut v16): (bool, i32, Rc<str>) = if v8 {
        let v9: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stderr).as_ref());
        let v10: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stdout).as_ref());
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
        let mut v12: i32 = (&*v7).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v12, v11.clone())
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v13.clone())
    };
    let v18: bool = if v14 {
        let v17: bool = v15 == 0i32;
        v17
    } else {
        false
    };
    if v18 {
        US3::US3_Some(v16.clone())
    } else {
        US3::US3_None
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                    let mut v7: bool = v4 == b'\r';
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
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                    let mut v7: bool = v4 == b'\r';
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
fn printed_trimmed_13(mut v0: Rc<str>) -> () {
    let v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method14(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method15(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    println!("{}", v8);
    ()
}
fn serial_started_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: bool, mut v4: bool, mut v5: Rc<UH0>) -> i32 {
    let v6: u64 = 0u64;
    let mut v7: US1 = links_timeout_2(v5.clone(), v6);
    match &v7 {
        US1::US1_None => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain refused: every --serial link needs a positive <timeout-ms>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v8);
            2i32
        }
        US1::US1_Some(v9) => {
            let v9: u64 = *v9;
            let v15: Rc<UH1> = match &v1 {
                US0::US0_AfterAny => {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                }
                US0::US0_AfterOk => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v10.clone(), v11.clone()))
                }
            };
            let v25: Rc<str> = match &*v5 {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<UH1> = v16.clone();
                    let v17: Rc<UH0> = v17.clone();
                    match &*v16 {
                        UH1::UH1_Cons(v18, v19) => {
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH1> = v19.clone();
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-serial"); } LIT.with(|lit| lit.clone()) };
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
                            v21.clone()
                        }
                        _ => {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-serial"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        }
                    }
                }
                _ => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-serial"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
            };
            let v26: Rc<str> = format_real_5(v9);
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--run-serial"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<UH1> = Rc::new(UH1::UH1_Cons(v29.clone(), v15.clone()));
            let v31: Rc<UH1> = Rc::new(UH1::UH1_Cons(v28.clone(), v30.clone()));
            let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27.clone(), v31.clone()));
            let v33: Rc<UH1> = links_flattened_7(v5.clone());
            let v34: Rc<UH1> = list_joined_8(v32.clone(), v33.clone());
            let v35: bool = &*v2 == "";
            let v48: Rc<UH1> = if v35 {
                let v36: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v34.clone()));
                let v37: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26.clone(), v36.clone()));
                Rc::new(UH1::UH1_Cons(v25.clone(), v37.clone()))
            } else {
                let v42: Rc<str> = match &v1 {
                    US0::US0_AfterAny => {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
                        v39.clone()
                    }
                    US0::US0_AfterOk => {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
                        v40.clone()
                    }
                };
                let v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v34.clone()));
                let mut v44: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26.clone(), v43.clone()));
                let mut v45: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25.clone(), v44.clone()));
                let v46: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v45.clone()));
                Rc::new(UH1::UH1_Cons(v42.clone(), v46.clone()))
            };
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<UH1> = Rc::new(UH1::UH1_Cons(v49.clone(), v48.clone()));
            let v51: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27.clone(), v50.clone()));
            let v52: US3 = captured_9(v0.clone(), v51.clone());
            match &v52 {
                US3::US3_None => {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain could not start its serial job"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v54);
                    2i32
                }
                US3::US3_Some(v53) => {
                    let v53: Rc<str> = v53.clone();
                    printed_trimmed_13(v53.clone());
                    0i32
                }
            }
        }
    }
}
fn method17(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn format_real_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn chain_started_16(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> i32 {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v22, v23) => {
                let v22: Rc<UH1> = v22.clone();
                let v23: Rc<UH0> = v23.clone();
                let v24: bool = &*v2 == "";
                let v31: Rc<UH1> = if v24 {
                    v22.clone()
                } else {
                    let v28: Rc<str> = match &v1 {
                        US0::US0_AfterAny => {
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                        US0::US0_AfterOk => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                    };
                    let v29: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v22.clone()));
                    Rc::new(UH1::UH1_Cons(v28.clone(), v29.clone()))
                };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33.clone(), v31.clone()));
                let v35: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32.clone(), v34.clone()));
                let v36: US3 = captured_9(v0.clone(), v35.clone());
                let v40: Rc<str> = match &v36 {
                    US3::US3_None => {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v38.clone()
                    }
                    US3::US3_Some(v37) => {
                        let v37: Rc<str> = v37.clone();
                        v37.clone()
                    }
                };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id=job-"); } LIT.with(|lit| lit.clone()) };
                let v42: Option<usize> = v40.find(&*v41);
                let v43: Option<usize> = method4(v42.clone());
                let mut v44: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v45: Option<i32> = v43.map(|x| v44(x));
                let v47: i32 = v45.unwrap_or(-1i32);
                let v48: bool = v47 < 0i32;
                let v69: Rc<str> = if v48 {
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v49.clone()
                } else {
                    let v50: i32 = v47.wrapping_add(3i32);
                    let v51: i32 = (v40.clone().len() as i32);
                    let v52: i32 = v51.wrapping_sub(v47);
                    let v53: i32 = v52.wrapping_sub(3i32);
                    let v54: usize = ((v50) as usize);
                    let v55: i32 = v50.wrapping_add(v53);
                    let v56: usize = ((v55) as usize);
                    let v57: Rc<str> = Rc::<str>::from(&(v40)[v54..v56]);
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v57.split(&*v58).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v60: Rc<str> = v59.clone().borrow()[0i32 as usize].clone();
                    let v61: i32 = (v60.clone().len() as i32);
                    let v62: i32 = 0i32;
                    let v63: i32 = method14(v60.clone(), v61, v62);
                    let v64: i32 = v61.wrapping_sub(1i32);
                    let v65: Rc<str> = string_slice(&v60.clone(), v63 as i64, v64 as i64);
                    let v66: i32 = (v65.clone().len() as i32);
                    let v67: i32 = method15(v65.clone(), v66);
                    let v68: Rc<str> = string_slice(&v65.clone(), 0i32 as i64, v67 as i64);
                    v68.clone()
                };
                let v70: bool = &*v69 == "";
                if v70 {
                    let v71: i32 = 0i32;
                    let v72: i32 = method17(v4.clone(), v71);
                    let v73: i32 = v72.wrapping_add(1i32);
                    let v74: Rc<str> = format_real_18(v73);
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain stopped: link "); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v74));
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" did not start ("); } LIT.with(|lit| lit.clone()) };
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                    let v79: i32 = (v40.clone().len() as i32);
                    let v80: i32 = 0i32;
                    let v81: i32 = method14(v40.clone(), v79, v80);
                    let v82: i32 = v79.wrapping_sub(1i32);
                    let v83: Rc<str> = string_slice(&v40.clone(), v81 as i64, v82 as i64);
                    let v84: i32 = (v83.clone().len() as i32);
                    let v85: i32 = method15(v83.clone(), v84);
                    let v86: Rc<str> = string_slice(&v83.clone(), 0i32 as i64, v85 as i64);
                    let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v86));
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("); started before it: "); } LIT.with(|lit| lit.clone()) };
                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                    let v90: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v91: Rc<UH1> = method11(v4.clone(), v90.clone());
                    let v92: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v93: i32 = 0i32;
                    let v94: i32 = method19(v92.clone(), v91.clone(), v93);
                    let v95: Rc<Vec<Rc<str>>> = Rc::new(v92.borrow().clone());
                    let v96: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v95).as_ref().clone()));
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v98: Rc<str> = Rc::<str>::from((*v96.borrow()).join(&*v97));
                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v98));
                    eprintln!("{}", v99);
                    return 2i32;
                } else {
                    printed_trimmed_13(v40.clone());
                    let v100: Rc<UH1> = Rc::new(UH1::UH1_Cons(v69.clone(), v4.clone()));
                    (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v69.clone(), v23.clone(), v100.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v5: i32 = 0i32;
                let v6: i32 = method17(v4.clone(), v5);
                let mut v7: Rc<str> = format_real_18(v6);
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain ok jobs="); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ids="); } LIT.with(|lit| lit.clone()) };
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v13: Rc<UH1> = method11(v4.clone(), v12.clone());
                let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v15: i32 = 0i32;
                let v16: i32 = method19(v14.clone(), v13.clone(), v15);
                let v17: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
                let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v20));
                println!("{}", v21);
                return 0i32;
            }
        }
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_21(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 <= v0;
        if v3 {
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method11(v2.clone(), v4.clone());
        } else {
            let v6: i32 = v0.wrapping_add(1i32);
            let mut v7: bool = v0 >= 0i32;
            let v10: bool = if v7 {
                let v8: i32 = command_line_argument_count_20();
                let v9: bool = v0 < v8;
                v9
            } else {
                false
            };
            let mut v14: US3 = if v10 {
                let mut v11: Rc<str> = command_line_argument_22(v0);
                US3::US3_Some(v11.clone())
            } else {
                US3::US3_None
            };
            let v18: Rc<str> = match &v14 {
                US3::US3_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                }
                US3::US3_Some(v15) => {
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
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
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
fn segments_23(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8.clone();
                let v9: Rc<UH1> = v9.clone();
                let v10: bool = &*v8 == "--";
                if v10 {
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH1> = method11(v1.clone(), v12.clone());
                    let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v2.clone()));
                    (v0, v1, v2) = (v9.clone(), v11.clone(), v14.clone());
                    continue;
                } else {
                    let v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v8.clone(), v1.clone()));
                    (v0, v1, v2) = (v9.clone(), v16.clone(), v2.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH1> = method11(v1.clone(), v3.clone());
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                return method24(v2.clone(), v6.clone());
            }
        }
    }
}
fn head_from_25(mut v0: Rc<UH1>, mut v1: US0, mut v2: Rc<str>, mut v3: bool, mut v4: bool) -> US5 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v6, v7) => {
                let v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
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
                        let mut v14: bool = &*v6 == "--run-serial";
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
                                return US5::US5_None;
                            }
                        }
                    }
                }
            }
            UH1::UH1_Nil => {
                return US5::US5_Some(v1.clone(), v2.clone(), v3, v4);
            }
        }
    }
}
fn short_link_26(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = 0i32;
                let v5: i32 = method17(v2.clone(), v4);
                let v6: bool = v5 < 3i32;
                if v6 {
                    return v1;
                } else {
                    let mut v7: i32 = v1.wrapping_add(1i32);
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
fn method27(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<UH1> = v2.clone();
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
fn method29(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US3> = Rc::new(move |mut v0: (Rc<str>)| -> US3 {
        let v1: Rc<str> = (v0);
        US3::US3_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_28() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let mut v7: bool = true; v5 });
    let v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method29(v9.clone());
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US3> = closure2();
    let mut v12: Option<US3> = v10.map(|x| v11(x));
    let mut v14: US3 = US3::US3_None;
    let v15: US3 = v14.clone();
    let v16: US3 = v12.unwrap_or(v15);
    match &v16 {
        US3::US3_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
        US3::US3_Some(v17) => {
            let v17: Rc<str> = v17.clone();
            v17.clone()
        }
    }
}
fn chain_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: i32 = command_line_argument_count_20();
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = arguments_from_21(v0, v1, v2.clone());
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = segments_23(v3.clone(), v4.clone(), v5.clone());
    let v48: US4 = match &*v6 {
        UH0::UH0_Cons(v9, v10) => {
            let v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let mut v11: US0 = US0::US0_AfterAny;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = false;
            let mut v14: bool = false;
            let v15: US5 = head_from_25(v9.clone(), v11.clone(), v12.clone(), v13, v14);
            match &v15 {
                US5::US5_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_ChainRefused(v16.clone())
                }
                US5::US5_Some(v18, v19, v20, v21) => {
                    let v18: US0 = v18.clone();
                    let v19: Rc<str> = v19.clone();
                    let v20: bool = *v20;
                    let v21: bool = *v21;
                    let v22: i32 = 1i32;
                    let v23: i32 = short_link_26(v10.clone(), v22);
                    let v24: i32 = 0i32;
                    let v25: i32 = method27(v10.clone(), v24);
                    let v26: bool = v25 == 0i32;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; "); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        US4::US4_ChainRefused(v29.clone())
                    } else {
                        let v31: bool = v23 > 0i32;
                        if v31 {
                            let v32: Rc<str> = format_real_18(v23);
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link "); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) };
                            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                            US4::US4_ChainRefused(v36.clone())
                        } else {
                            let v38: bool = v20 && v21;
                            if v38 {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--serial and --run-serial exclude each other"); } LIT.with(|lit| lit.clone()) };
                                US4::US4_ChainRefused(v39.clone())
                            } else {
                                US4::US4_ChainPlanned(v18.clone(), v19.clone(), v20, v21, v10.clone())
                            }
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
            US4::US4_ChainRefused(v7.clone())
        }
    };
    match &v48 {
        US4::US4_ChainPlanned(v52, v53, v54, v55, v56) => {
            let v52: US0 = v52.clone();
            let v53: Rc<str> = v53.clone();
            let v54: bool = *v54;
            let v55: bool = *v55;
            let v56: Rc<UH0> = v56.clone();
            let v57: Rc<str> = current_executable_28();
            if v54 {
                serial_started_1(v57.clone(), v52.clone(), v53.clone(), v54, v55, v56.clone())
            } else {
                let v59: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                chain_started_16(v57.clone(), v52.clone(), v53.clone(), v56.clone(), v59.clone())
            }
        }
        US4::US4_ChainRefused(v49) => {
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
        chain_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_chain_run() -> i32 {
    closure0()()
}
