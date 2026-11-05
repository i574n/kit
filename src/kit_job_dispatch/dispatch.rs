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
    US0_Dispatched(Rc<str>, Rc<str>),
    US0_RemoteUnreachable(Rc<str>),
    US0_RemoteRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_RequestRead(Rc<str>, Rc<str>, Rc<UH0>),
    US1_RequestRefused(Rc<str>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
struct Mut1 { l0: Rc<str> }
fn outcome_code_1(mut v0: US0) -> i32 {
    match &v0 {
        US0::US0_Dispatched(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            0i32
        }
        US0::US0_RemoteRefused(v4) => {
            let mut v4: Rc<str> = v4.clone();
            2i32
        }
        US0::US0_RemoteUnreachable(v3) => {
            let v3: Rc<str> = v3.clone();
            4i32
        }
    }
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn words_from_3(mut v0: i32, mut v1: i32) -> Rc<UH0> {
    let mut v2: bool = v1 <= v0;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v5: i32 = v0.wrapping_add(1i32);
        let mut v6: Rc<UH0> = words_from_3(v5, v1);
        Rc::new(UH0::UH0_Cons(v4, v6))
    }
}
fn plain_argument_6(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    let v3: bool = v2 == false;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.contains(&*v4);
        let mut v6: bool = v5 == false;
        v6
    } else {
        false
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v5: bool = if v1 {
                    plain_argument_6(v2.clone())
                } else {
                    false
                };
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn request_of_4(mut v0: Rc<UH0>) -> US1 {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            let (mut v15, mut v16): (Rc<str>, Rc<UH0>) = match &*v2 {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let mut v4: Rc<UH0> = v4.clone();
                    match &*v4 {
                        UH0::UH0_Cons(v5, v6) => {
                            let mut v5: Rc<str> = v5.clone();
                            let mut v6: Rc<UH0> = v6.clone();
                            let v7: bool = &*v3 == "--bin";
                            if v7 {
                                (v5, v6)
                            } else {
                                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                                (v8, v2.clone())
                            }
                        }
                        _ => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                            (v11, v2.clone())
                        }
                    }
                }
                _ => {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                    (v14, v2)
                }
            };
            let v17: bool = &*v1 == "";
            let v20: bool = if v17 {
                true
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let mut v19: bool = v1.starts_with(&*v18);
                v19
            };
            if v20 {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
                US1::US1_RequestRefused(v21)
            } else {
                let v23: bool = match &*v16 {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on <machine> needs the job: [options] <label> <timeout-ms> <program> [args...]"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_RequestRefused(v24)
                } else {
                    let v26: bool = true;
                    let v27: bool = method5(v16.clone(), v26);
                    let v28: bool = v27 == false;
                    if v28 {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on forwards arguments without quote characters (they would be re-split by the remote shell)"); } LIT.with(|lit| lit.clone()) };
                        US1::US1_RequestRefused(v29)
                    } else {
                        US1::US1_RequestRead(v1, v15, v16)
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RequestRefused(v35)
        }
    }
}
fn ssh_to_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=10"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ServerAliveInterval=15"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v6));
    let mut v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v7));
    let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v9));
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v10));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v11));
    let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v12));
    Rc::new(UH0::UH0_Cons(v2, v13))
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method10(v3, v1.clone());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v2)), v5));
            Rc::new(UH0::UH0_Cons(v7, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn remote_line_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job start "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v3));
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method10(v2, v5);
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method11(v7.clone(), v6, v8);
    let v10: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v12));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v13));
    v14
}
fn ssh_arguments_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    let v3: Rc<str> = remote_line_9(v0.clone(), v1, v2);
    ssh_to_8(v0, v3)
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
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
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method14(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3.clone();
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method19(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method21(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method21(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method13(v16.clone(), v17.clone())
    }
}
fn receipt_id_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start ok id="); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method19(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: i32 = v7.wrapping_add(20i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v7);
        let mut v13: i32 = v12.wrapping_sub(20i32);
        let mut v14: usize = ((v10) as usize);
        let v15: i32 = v10.wrapping_add(v13);
        let v16: usize = ((v15) as usize);
        let v17: Rc<str> = Rc::<str>::from(&(v0)[v14..v16]);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(v17.replace(&*v19, &*v18));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
        let v24: Rc<UH0> = split_list_20(v18.clone(), v23.clone());
        match &*v24 {
            UH0::UH0_Cons(v25, v26) => {
                let v25: Rc<str> = v25.clone();
                let v26: Rc<UH0> = v26.clone();
                v25.clone()
            }
            UH0::UH0_Nil => {
                v22.clone()
            }
        }
    }
}
fn method23(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_22(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn outcome_of_15(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: Rc<str>) -> US0 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: i32 = (v6.clone().len() as i32);
    let mut v8: i32 = 0i32;
    let mut v9: i32 = method16(v6.clone(), v7, v8);
    let v10: i32 = v7.wrapping_sub(1i32);
    let v11: Rc<str> = string_slice(&v6, v9 as i64, v10 as i64);
    let v12: i32 = (v11.clone().len() as i32);
    let mut v13: i32 = method17(v11.clone(), v12);
    let mut v14: Rc<str> = string_slice(&v11, 0i32 as i64, v13 as i64);
    let v15: Rc<str> = receipt_id_18(v3);
    let v16: bool = v1 == false;
    if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh did not run"); } LIT.with(|lit| lit.clone()) };
        US0::US0_RemoteUnreachable(v17)
    } else {
        let mut v19: bool = &*v15 != "";
        if v19 {
            US0::US0_Dispatched(v0, v15)
        } else {
            let v21: bool = v2 == 255i32;
            let v23: bool = if v21 {
                true
            } else {
                let v22: bool = &*v14 == "";
                v22
            };
            if v23 {
                let v24: bool = &*v14 == "";
                let v28: Rc<str> = if v24 {
                    let v25: Rc<str> = format_real_22(v2);
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh exit "); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v25));
                    v27
                } else {
                    v14.clone()
                };
                US0::US0_RemoteUnreachable(v28)
            } else {
                US0::US0_RemoteRefused(v14)
            }
        }
    }
}
fn joined_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v3)), v1));
    v6
}
fn label_of_id_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_20(v1.clone(), v0.clone());
    match &*v2 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            match &*v4 {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5.clone();
                    let mut v6: Rc<UH0> = v6.clone();
                    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let mut v8: i32 = 0i32;
                    let mut v9: i32 = method11(v7.clone(), v6, v8);
                    let v10: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
                    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
                    let v12: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v1));
                    v12
                }
                _ => {
                    v0.clone()
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn format_real_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method23(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn recorded_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> () {
    let mut v5: Rc<str> = joined_25(v0, v2.clone());
    let v7: Result<(), std::io::Error> = std::fs::create_dir_all(&*v5);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("placement"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = joined_25(v5.clone(), v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("on "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v1));
    std::fs::write(&*v9, &*v11).unwrap();
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = joined_25(v5.clone(), v13);
    let v15: Rc<str> = label_of_id_26(v2);
    std::fs::write(&*v14, &*v15).unwrap();
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = joined_25(v5.clone(), v17);
    let mut v19: Rc<str> = format_real_27(v4);
    std::fs::write(&*v18, &*v19).unwrap();
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remote-bin"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = joined_25(v5, v21);
    std::fs::write(&*v22, &*v3).unwrap();
    ()
}
fn outcome_line_28(mut v0: US0, mut v1: Rc<str>) -> Rc<str> {
    match &v0 {
        US0::US0_Dispatched(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start ok id="); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" machine="); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v6)), v2));
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" placement=remote"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10
        }
        US0::US0_RemoteRefused(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start refused machine="); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v1));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(v20.replace(&*v25, &*v26));
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v27));
            v28
        }
        US0::US0_RemoteUnreachable(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job start unreachable machine="); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v1));
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(v11.replace(&*v16, &*v17));
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v18));
            v19
        }
    }
}
fn dispatch_run_0(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 4i32;
    let mut v2: i32 = command_line_argument_count_2();
    let v3: i32 = v2.wrapping_add(1i32);
    let mut v4: Rc<UH0> = words_from_3(v1, v3);
    let mut v5: US1 = request_of_4(v4.clone());
    match &v5 {
        US1::US1_RequestRead(v9, v10, v11) => {
            let mut v9: Rc<str> = v9.clone();
            let v10: Rc<str> = v10.clone();
            let v11: Rc<UH0> = v11.clone();
            let v12: Rc<UH0> = ssh_arguments_7(v9.clone(), v10.clone(), v11.clone());
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v13)));
            let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v16: Rc<UH0> = method12(v12.clone(), v15.clone());
            let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v18: Rc<UH0> = method13(v16.clone(), v17.clone());
            method14(v14.clone(), v18.clone());
            let mut v19: Rc<std::io::Result<std::process::Output>> = Rc::new(v14.borrow_mut().output());
            let v20: bool = v19.is_ok();
            let (mut v26, mut v27, mut v28): (bool, i32, Rc<str>) = if v20 {
                let v21: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v19).as_ref().unwrap().stderr).as_ref());
                let v22: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v19).as_ref().unwrap().stdout).as_ref());
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
                let v24: i32 = (&*v19).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v24, v23.clone())
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v25.clone())
            };
            let v29: US0 = outcome_of_15(v9.clone(), v26, v27, v28.clone());
            match &v29 {
                US0::US0_Dispatched(v30, v31) => {
                    let v30: Rc<str> = v30.clone();
                    let v31: Rc<str> = v31.clone();
                    let v32: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    recorded_24(v0.clone(), v30.clone(), v31.clone(), v10.clone(), v32)
                }
                _ => {
                    ()
                }
            };
            let v33: Rc<str> = outcome_line_28(v29.clone(), v9.clone());
            println!("{}", v33);
            outcome_code_1(v29.clone())
        }
        US1::US1_RequestRefused(v6) => {
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job start "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
            eprintln!("{}", v8);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        dispatch_run_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_30(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method31(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method32(mut v0: Rc<UH0>, mut v1: bool) -> bool {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v5: bool = if v1 {
                    plain_argument_6(v2.clone())
                } else {
                    false
                };
                (v0, v1) = (v3.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method34(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method34(v3, v1.clone());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v2)), v5));
            Rc::new(UH0::UH0_Cons(v7, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn forward_line_33(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = method34(v1, v4);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v7: i32 = 0i32;
    let mut v8: i32 = method11(v6.clone(), v5, v7);
    let mut v9: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from((*v10.borrow()).join(&*v11));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v12));
    v13
}
fn forward_code_35(mut v0: bool, mut v1: i32) -> i32 {
    let mut v2: bool = v0 == false;
    let mut v4: bool = if v2 {
        true
    } else {
        let v3: bool = v1 == 255i32;
        v3
    };
    if v4 {
        4i32
    } else {
        v1
    }
}
fn forward_run_29(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = 3i32;
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
    let mut v4: Rc<str> = joined_25(v1, v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("remote-bin"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = joined_25(v4, v5);
    let v7: bool = file_exists_30(v6.clone());
    let v28: Rc<str> = if v7 {
        let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let v11: Vec<u8> = v9.unwrap();
        let v12: Vec<u8> = method31(v11);
        let mut v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
        let v16: std::string::String = v14.unwrap();
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        let mut v19: i32 = (v18.clone().len() as i32);
        let v20: i32 = 0i32;
        let v21: i32 = method16(v18.clone(), v19, v20);
        let v22: i32 = v19.wrapping_sub(1i32);
        let v23: Rc<str> = string_slice(&v18, v21 as i64, v22 as i64);
        let v24: i32 = (v23.clone().len() as i32);
        let v25: i32 = method17(v23.clone(), v24);
        let v26: Rc<str> = string_slice(&v23, 0i32 as i64, v25 as i64);
        v26
    } else {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
        v27
    };
    let v29: i32 = 2i32;
    let v30: i32 = command_line_argument_count_2();
    let v31: i32 = v30.wrapping_add(1i32);
    let v32: Rc<UH0> = words_from_3(v29, v31);
    let v33: bool = true;
    let v34: bool = method32(v32.clone(), v33);
    let v35: bool = v34 == false;
    if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job forwarding refuses quote characters in arguments"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v36);
        2i32
    } else {
        let v37: Rc<str> = forward_line_33(v28, v32);
        let v38: Rc<UH0> = ssh_to_8(v0.clone(), v37);
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
        let mut v40: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v39)));
        let v41: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v42: Rc<UH0> = method12(v38, v41);
        let v43: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v44: Rc<UH0> = method13(v42, v43);
        method14(v40.clone(), v44);
        let mut v45: Rc<std::io::Result<std::process::Output>> = Rc::new(v40.borrow_mut().output());
        let v46: bool = v45.is_ok();
        let (mut v52, mut v53, mut v54): (bool, i32, Rc<str>) = if v46 {
            let v47: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v45).as_ref().unwrap().stderr).as_ref());
            let v48: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v45).as_ref().unwrap().stdout).as_ref());
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v47));
            let v50: i32 = (&*v45).as_ref().unwrap().status.code().unwrap_or(-2);
            (true, v50, v49)
        } else {
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, -1i32, v51)
        };
        let v55: i32 = forward_code_35(v52, v53);
        let v56: bool = v55 == 4i32;
        let v66: bool = if v56 {
            let v57: i32 = (v54.clone().len() as i32);
            let v58: i32 = 0i32;
            let v59: i32 = method16(v54.clone(), v57, v58);
            let v60: i32 = v57.wrapping_sub(1i32);
            let v61: Rc<str> = string_slice(&v54.clone(), v59 as i64, v60 as i64);
            let v62: i32 = (v61.clone().len() as i32);
            let v63: i32 = method17(v61.clone(), v62);
            let v64: Rc<str> = string_slice(&v61, 0i32 as i64, v63 as i64);
            let v65: bool = &*v64 == "";
            v65
        } else {
            false
        };
        let v77: Rc<str> = if v66 {
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job unreachable machine="); } LIT.with(|lit| lit.clone()) };
            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v0));
            v68
        } else {
            let v69: i32 = (v54.clone().len() as i32);
            let v70: i32 = 0i32;
            let v71: i32 = method16(v54.clone(), v69, v70);
            let v72: i32 = v69.wrapping_sub(1i32);
            let v73: Rc<str> = string_slice(&v54, v71 as i64, v72 as i64);
            let v74: i32 = (v73.clone().len() as i32);
            let v75: i32 = method17(v73.clone(), v74);
            let v76: Rc<str> = string_slice(&v73, 0i32 as i64, v75 as i64);
            v76
        };
        println!("{}", v77);
        v55
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        forward_run_29(v1.clone(), v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn listed_on_37(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v2));
    let mut v5: Rc<str> = forward_line_33(v1, v4);
    let mut v6: Rc<UH0> = ssh_to_8(v0.clone(), v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v7)));
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method12(v6, v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method13(v10, v11);
    method14(v8.clone(), v12);
    let mut v13: Rc<std::io::Result<std::process::Output>> = Rc::new(v8.borrow_mut().output());
    let mut v14: bool = v13.is_ok();
    let (mut v20, mut v21, mut v22): (bool, i32, Rc<str>) = if v14 {
        let v15: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v13).as_ref().unwrap().stderr).as_ref());
        let v16: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v13).as_ref().unwrap().stdout).as_ref());
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
        let v18: i32 = (&*v13).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v18, v17)
    } else {
        let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v19)
    };
    let v23: i32 = forward_code_35(v20, v21);
    let v24: bool = v23 == 4i32;
    let v34: bool = if v24 {
        let v25: i32 = (v22.clone().len() as i32);
        let v26: i32 = 0i32;
        let v27: i32 = method16(v22.clone(), v25, v26);
        let v28: i32 = v25.wrapping_sub(1i32);
        let v29: Rc<str> = string_slice(&v22.clone(), v27 as i64, v28 as i64);
        let v30: i32 = (v29.clone().len() as i32);
        let v31: i32 = method17(v29.clone(), v30);
        let v32: Rc<str> = string_slice(&v29, 0i32 as i64, v31 as i64);
        let v33: bool = &*v32 == "";
        v33
    } else {
        false
    };
    let mut v45: Rc<str> = if v34 {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job unreachable machine="); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v0));
        v36
    } else {
        let v37: i32 = (v22.clone().len() as i32);
        let v38: i32 = 0i32;
        let v39: i32 = method16(v22.clone(), v37, v38);
        let mut v40: i32 = v37.wrapping_sub(1i32);
        let v41: Rc<str> = string_slice(&v22, v39 as i64, v40 as i64);
        let v42: i32 = (v41.clone().len() as i32);
        let v43: i32 = method17(v41.clone(), v42);
        let v44: Rc<str> = string_slice(&v41, 0i32 as i64, v43 as i64);
        v44
    };
    println!("{}", v45);
    v23
}
fn list_on_run_36() -> i32 {
    let mut v0: i32 = 4i32;
    let mut v1: i32 = command_line_argument_count_2();
    let mut v2: i32 = v1.wrapping_add(1i32);
    let v3: Rc<UH0> = words_from_3(v0, v2);
    match &*v3 {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<UH0> = v5.clone();
            match &*v5 {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6.clone();
                    let v7: Rc<UH0> = v7.clone();
                    match &*v7 {
                        UH0::UH0_Cons(v8, v9) => {
                            let mut v8: Rc<str> = v8.clone();
                            let mut v9: Rc<UH0> = v9.clone();
                            let v10: bool = &*v6 == "--bin";
                            if v10 {
                                listed_on_37(v4.clone(), v8, v9)
                            } else {
                                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                                listed_on_37(v4.clone(), v12, v5.clone())
                            }
                        }
                        _ => {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                            listed_on_37(v4.clone(), v15, v5.clone())
                        }
                    }
                }
                _ => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
                    listed_on_37(v4, v18, v5)
                }
            }
        }
        UH0::UH0_Nil => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job list --on needs a machine (an ssh alias)"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v21);
            2i32
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        list_on_run_36()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_dispatch_run(v0: &str) -> i32 {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_job_forward_run(v0: &str, v1: &str) -> i32 {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_job_list_on() -> i32 {
    closure3()()
}
