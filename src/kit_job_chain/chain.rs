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
pub struct Mut0 { pub l0: Rc<str> }
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
        let mut v1: usize = (v0);
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
            let mut v12: Option<usize> = method4(v11);
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
                (v1, v2) = (v7, v26);
                continue;
            }
        }
    }
}
fn links_timeout_2(mut v0: Rc<UH0>, mut v1: u64) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<UH1> = v3;
                let mut v4: Rc<UH0> = v4;
                match Rc::unwrap_or_clone(v3) {
                    UH1::UH1_Cons(v5, v6) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<UH1> = v6;
                        match Rc::unwrap_or_clone(v6) {
                            UH1::UH1_Cons(v7, v8) => {
                                let mut v7: Rc<str> = v7;
                                let v8: Rc<UH1> = v8;
                                let v9: bool = &*v7 == "";
                                let mut v14: US2 = if v9 {
                                    US2::US2_None
                                } else {
                                    let mut v11: i32 = 0i32;
                                    let mut v12: i32 = 0i32;
                                    decimal_from_3(v7, v11, v12)
                                };
                                match v14 {
                                    US2::US2_None => {
                                        return US1::US1_None;
                                    }
                                    US2::US2_Some(v15) => {
                                        let v15: i32 = v15;
                                        let v16: bool = v15 > 0i32;
                                        if v16 {
                                            let v17: u64 = (v15 as u64);
                                            let v18: u64 = v1.wrapping_add(v17);
                                            (v0, v1) = (v4, v18);
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
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn list_joined_8(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let mut v4: Rc<UH1> = list_joined_8(v3, v1);
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn links_flattened_7(mut v0: Rc<UH0>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<UH1> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH1> = links_flattened_7(v3);
            let v5: Rc<UH1> = list_joined_8(v2, v4);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            Rc::new(UH1::UH1_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
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
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
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
fn method12(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH1>) -> () {
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
fn captured_9(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US3 {
    let mut v2: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method10(v1, v3);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH1> = method11(v4, v5);
    method12(v2.clone(), v6);
    let mut v7: Rc<std::io::Result<std::process::Output>> = Rc::new(v2.borrow_mut().output());
    let v8: bool = v7.is_ok();
    let (mut v14, mut v15, mut v16): (bool, i32, Rc<str>) = if v8 {
        let v9: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stderr).as_ref());
        let v10: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v7).as_ref().unwrap().stdout).as_ref());
        let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
        let mut v12: i32 = (&*v7).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v12, v11)
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v13)
    };
    let v18: bool = if v14 {
        let v17: bool = v15 == 0i32;
        v17
    } else {
        false
    };
    if v18 {
        US3::US3_Some(v16)
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
                v2 = v12;
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn printed_trimmed_13(mut v0: Rc<str>) -> () {
    let mut v1: i32 = (v0.clone().len() as i32);
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
    match v7 {
        US1::US1_None => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain refused: every --serial link needs a positive <timeout-ms>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v8);
            2i32
        }
        US1::US1_Some(v9) => {
            let v9: u64 = v9;
            let v15: Rc<UH1> = match &v1 {
                US0::US0_AfterAny => {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                }
                US0::US0_AfterOk => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v10, v11))
                }
            };
            let v24: Rc<str> = match &*v5 {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<UH1> = v16.clone();
                    let v17: Rc<UH0> = v17.clone();
                    match Rc::unwrap_or_clone(v16) {
                        UH1::UH1_Cons(v18, v19) => {
                            let v18: Rc<str> = v18;
                            let v19: Rc<UH1> = v19;
                            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-serial"); } LIT.with(|lit| lit.clone()) }));
                            v20
                        }
                        _ => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-serial"); } LIT.with(|lit| lit.clone()) };
                            v21
                        }
                    }
                }
                _ => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-serial"); } LIT.with(|lit| lit.clone()) };
                    v23
                }
            };
            let v25: Rc<str> = format_real_5(v9);
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--run-serial"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<UH1> = Rc::new(UH1::UH1_Cons(v28, v15));
            let v30: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27, v29));
            let v31: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26.clone(), v30));
            let v32: Rc<UH1> = links_flattened_7(v5);
            let v33: Rc<UH1> = list_joined_8(v31, v32);
            let v34: bool = &*v2 == "";
            let v47: Rc<UH1> = if v34 {
                let v35: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v33));
                let v36: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25, v35));
                Rc::new(UH1::UH1_Cons(v24, v36))
            } else {
                let v41: Rc<str> = match v1 {
                    US0::US0_AfterAny => {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
                        v38
                    }
                    US0::US0_AfterOk => {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
                        v39
                    }
                };
                let mut v42: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v33));
                let mut v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25, v42));
                let v44: Rc<UH1> = Rc::new(UH1::UH1_Cons(v24, v43));
                let v45: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v44));
                Rc::new(UH1::UH1_Cons(v41, v45))
            };
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<UH1> = Rc::new(UH1::UH1_Cons(v48, v47));
            let v50: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26, v49));
            let v51: US3 = captured_9(v0, v50);
            match v51 {
                US3::US3_None => {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain could not start its serial job"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v53);
                    2i32
                }
                US3::US3_Some(v52) => {
                    let v52: Rc<str> = v52;
                    printed_trimmed_13(v52);
                    0i32
                }
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
fn format_real_18(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
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
        match Rc::unwrap_or_clone(v3) {
            UH0::UH0_Cons(v20, v21) => {
                let v20: Rc<UH1> = v20;
                let v21: Rc<UH0> = v21;
                let v22: bool = &*v2 == "";
                let v29: Rc<UH1> = if v22 {
                    v20
                } else {
                    let v26: Rc<str> = match &v1 {
                        US0::US0_AfterAny => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
                            v23
                        }
                        US0::US0_AfterOk => {
                            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
                            v24
                        }
                    };
                    let v27: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v20));
                    Rc::new(UH1::UH1_Cons(v26, v27))
                };
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v31, v29));
                let v33: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30, v32));
                let v34: US3 = captured_9(v0.clone(), v33);
                let v38: Rc<str> = match v34 {
                    US3::US3_None => {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v36
                    }
                    US3::US3_Some(v35) => {
                        let v35: Rc<str> = v35;
                        v35
                    }
                };
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id=job-"); } LIT.with(|lit| lit.clone()) };
                let v40: Option<usize> = v38.find(&*v39);
                let v41: Option<usize> = method4(v40);
                let mut v42: Rc<dyn Fn((usize)) -> i32> = closure1();
                let mut v43: Option<i32> = v41.map(|x| v42(x));
                let v45: i32 = v43.unwrap_or(-1i32);
                let v46: bool = v45 < 0i32;
                let v67: Rc<str> = if v46 {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v47
                } else {
                    let v48: i32 = v45.wrapping_add(3i32);
                    let v49: i32 = (v38.clone().len() as i32);
                    let v50: i32 = v49.wrapping_sub(v45);
                    let v51: i32 = v50.wrapping_sub(3i32);
                    let v52: usize = ((v48) as usize);
                    let v53: i32 = v48.wrapping_add(v51);
                    let v54: usize = ((v53) as usize);
                    let v55: Rc<str> = Rc::<str>::from(&(v38)[v52..v54]);
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v55.split(&*v56).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v58: Rc<str> = v57.borrow()[0i32 as usize].clone();
                    let v59: i32 = (v58.clone().len() as i32);
                    let v60: i32 = 0i32;
                    let v61: i32 = method14(v58.clone(), v59, v60);
                    let v62: i32 = v59.wrapping_sub(1i32);
                    let v63: Rc<str> = string_slice(&v58, v61 as i64, v62 as i64);
                    let v64: i32 = (v63.clone().len() as i32);
                    let v65: i32 = method15(v63.clone(), v64);
                    let v66: Rc<str> = string_slice(&v63, 0i32 as i64, v65 as i64);
                    v66
                };
                let v68: bool = &*v67 == "";
                if v68 {
                    let v69: i32 = 0i32;
                    let v70: i32 = method17(v4.clone(), v69);
                    let v71: i32 = v70.wrapping_add(1i32);
                    let v72: Rc<str> = format_real_18(v71);
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain stopped: link "); } LIT.with(|lit| lit.clone()) }, v72));
                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" did not start ("); } LIT.with(|lit| lit.clone()) }));
                    let v75: i32 = (v38.clone().len() as i32);
                    let v76: i32 = 0i32;
                    let v77: i32 = method14(v38.clone(), v75, v76);
                    let v78: i32 = v75.wrapping_sub(1i32);
                    let v79: Rc<str> = string_slice(&v38, v77 as i64, v78 as i64);
                    let v80: i32 = (v79.clone().len() as i32);
                    let v81: i32 = method15(v79.clone(), v80);
                    let v82: Rc<str> = string_slice(&v79, 0i32 as i64, v81 as i64);
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v82));
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v83, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("); started before it: "); } LIT.with(|lit| lit.clone()) }));
                    let v85: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v86: Rc<UH1> = method11(v4, v85);
                    let v87: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v88: i32 = 0i32;
                    let v89: i32 = method19(v87.clone(), v86, v88);
                    let v90: Rc<Vec<Rc<str>>> = Rc::new(v87.borrow().clone());
                    let v91: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v90).as_ref().clone()));
                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = Rc::<str>::from((*v91.borrow()).join(&*v92));
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v93));
                    eprintln!("{}", v94);
                    return 2i32;
                } else {
                    printed_trimmed_13(v38);
                    let v95: Rc<UH1> = Rc::new(UH1::UH1_Cons(v67.clone(), v4));
                    (v2, v3, v4) = (v67, v21, v95);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v5: i32 = 0i32;
                let v6: i32 = method17(v4.clone(), v5);
                let mut v7: Rc<str> = format_real_18(v6);
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job chain ok jobs="); } LIT.with(|lit| lit.clone()) }, v7));
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ids="); } LIT.with(|lit| lit.clone()) }));
                let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let mut v11: Rc<UH1> = method11(v4, v10);
                let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let mut v13: i32 = 0i32;
                let mut v14: i32 = method19(v12.clone(), v11, v13);
                let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
                let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v17));
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v18));
                println!("{}", v19);
                return 0i32;
            }
        }
    }
}
fn command_line_argument_count_20() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_21(mut v0: i32, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 <= v0;
        if v3 {
            let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method11(v2, v4);
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
                US3::US3_Some(v11)
            } else {
                US3::US3_None
            };
            let v18: Rc<str> = match v14 {
                US3::US3_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US3::US3_Some(v15) => {
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
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn segments_23(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v8, v9) => {
                let v8: Rc<str> = v8;
                let v9: Rc<UH1> = v9;
                let v10: bool = &*v8 == "--";
                if v10 {
                    let mut v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let mut v13: Rc<UH1> = method11(v1, v12);
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
                let mut v4: Rc<UH1> = method11(v1, v3);
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                return method24(v2, v6);
            }
        }
    }
}
fn head_from_25(mut v0: Rc<UH1>, mut v1: US0, mut v2: Rc<str>, mut v3: bool, mut v4: bool) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v6, v7) => {
                let v6: Rc<str> = v6;
                let mut v7: Rc<UH1> = v7;
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
                                return US5::US5_None;
                            }
                        }
                    }
                }
            }
            UH1::UH1_Nil => {
                return US5::US5_Some(v1, v2, v3, v4);
            }
        }
    }
}
fn short_link_26(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
                    let mut v7: i32 = v1.wrapping_add(1i32);
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
fn method27(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method29(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US3> = Rc::new(move |mut v0: (Rc<str>)| -> US3 {
        let mut v1: Rc<str> = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_28() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let mut v7: bool = true; v5 });
    let v9: Option<Rc<str>> = _optionm_map_;
    let v10: Option<Rc<str>> = method29(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US3> = closure2();
    let mut v12: Option<US3> = v10.map(|x| v11(x));
    let mut v14: US3 = US3::US3_None;
    let v15: US3 = v14.clone();
    let v16: US3 = v12.unwrap_or(v15);
    match v16 {
        US3::US3_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18
        }
        US3::US3_Some(v17) => {
            let v17: Rc<str> = v17;
            v17
        }
    }
}
fn chain_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = command_line_argument_count_20();
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = arguments_from_21(v0, v1, v2);
    let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = segments_23(v3, v4, v5);
    let v44: US4 = match Rc::unwrap_or_clone(v6) {
        UH0::UH0_Cons(v9, v10) => {
            let v9: Rc<UH1> = v9;
            let v10: Rc<UH0> = v10;
            let mut v11: US0 = US0::US0_AfterAny;
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v13: bool = false;
            let mut v14: bool = false;
            let v15: US5 = head_from_25(v9, v11, v12, v13, v14);
            match v15 {
                US5::US5_None => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                    US4::US4_ChainRefused(v16)
                }
                US5::US5_Some(v18, v19, v20, v21) => {
                    let v18: US0 = v18;
                    let v19: Rc<str> = v19;
                    let v20: bool = v20;
                    let v21: bool = v21;
                    let v22: i32 = 1i32;
                    let v23: i32 = short_link_26(v10.clone(), v22);
                    let v24: i32 = 0i32;
                    let v25: i32 = method27(v10.clone(), v24);
                    let v26: bool = v25 == 0i32;
                    if v26 {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no job to start; job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
                        US4::US4_ChainRefused(v27)
                    } else {
                        let v29: bool = v23 > 0i32;
                        if v29 {
                            let v30: Rc<str> = format_real_18(v23);
                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("link "); } LIT.with(|lit| lit.clone()) }, v30));
                            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) }));
                            US4::US4_ChainRefused(v32)
                        } else {
                            let v34: bool = v20 && v21;
                            if v34 {
                                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--serial and --run-serial exclude each other"); } LIT.with(|lit| lit.clone()) };
                                US4::US4_ChainRefused(v35)
                            } else {
                                US4::US4_ChainPlanned(v18, v19, v20, v21, v10)
                            }
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one, --after-ok with --ok, and prints every id; --serial runs the links in order inside one job, so a waiting link holds no process)"); } LIT.with(|lit| lit.clone()) };
            US4::US4_ChainRefused(v7)
        }
    };
    match v44 {
        US4::US4_ChainPlanned(v47, v48, v49, v50, v51) => {
            let v47: US0 = v47;
            let v48: Rc<str> = v48;
            let v49: bool = v49;
            let v50: bool = v50;
            let v51: Rc<UH0> = v51;
            let v52: Rc<str> = current_executable_28();
            if v49 {
                serial_started_1(v52, v47, v48, v49, v50, v51)
            } else {
                let v54: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                chain_started_16(v52, v47, v48, v51, v54)
            }
        }
        US4::US4_ChainRefused(v45) => {
            let v45: Rc<str> = v45;
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job chain refused: "); } LIT.with(|lit| lit.clone()) }, v45));
            eprintln!("{}", v46);
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
pub type SpiralShape_97df63ebabc3 = Mut0;
