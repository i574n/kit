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
    US0_SetupReady(Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US0_SetupMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US3 {
    US3_AllTests,
    US3_NamedTests(Rc<UH0>),
    US3_ListTests,
}
#[derive(Clone)]
enum US2 {
    US2_RunProgram(Rc<str>, Rc<UH0>),
    US2_RunTests(Rc<str>, US3),
    US2_RunRefused(Rc<str>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method3(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn program_ran_1(mut v0: Rc<str>, mut v1: Rc<UH0>) -> i32 {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = method2(v1, v2);
    let mut v4: Option<std::process::ExitStatus> = std::process::Command::new(&*v0).args((v3).borrow().iter().map(|a| &**a)).status().ok();
    let v5: i32 = v4.and_then(|s| s.code()).unwrap_or(127);
    v5
}
fn get_environment_variable_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn setup_of_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US0 {
    let mut v4: bool = &*v0 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_LIB (the lib/spiral package folder)"); } LIT.with(|lit| lit.clone()) };
        US0::US0_SetupMissing(v5)
    } else {
        let v7: bool = &*v1 == "";
        if v7 {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_CORE (the core package folder)"); } LIT.with(|lit| lit.clone()) };
            US0::US0_SetupMissing(v8)
        } else {
            let v10: bool = &*v2 == "";
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_COMPILER_DLL (SpiralCompiler.dll)"); } LIT.with(|lit| lit.clone()) };
                US0::US0_SetupMissing(v11)
            } else {
                US0::US0_SetupReady(v0, v1, v2, v3)
            }
        }
    }
}
fn command_line_argument_count_7() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_8(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_6(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_7();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US1 = if v4 {
        let v5: Rc<str> = command_line_argument_8(v0);
        US1::US1_Some(v5)
    } else {
        US1::US1_None
    };
    match v8 {
        US1::US1_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_6(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method10(v16, v17)
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method12(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v5: i32 = (v2.clone().len() as i32);
                let v6: i32 = 0i32;
                let v7: i32 = method15(v2.clone(), v5, v6);
                let mut v8: i32 = v5.wrapping_sub(1i32);
                let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
                let v10: i32 = (v9.clone().len() as i32);
                let v11: i32 = method16(v9.clone(), v10);
                let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
                let v13: Rc<UH0> = split_list_9(v4, v12);
                let v33: US1 = match Rc::unwrap_or_clone(v13) {
                    UH0::UH0_Cons(v14, v15) => {
                        let v14: Rc<str> = v14;
                        let v15: Rc<UH0> = v15;
                        match Rc::unwrap_or_clone(v15) {
                            UH0::UH0_Cons(v16, v17) => {
                                let v16: Rc<str> = v16;
                                let v17: Rc<UH0> = v17;
                                match Rc::unwrap_or_clone(v17) {
                                    UH0::UH0_Cons(v18, v19) => {
                                        let v18: Rc<str> = v18;
                                        let v19: Rc<UH0> = v19;
                                        let v20: bool = &*v14 == ":#";
                                        let v22: bool = if v20 {
                                            let v21: bool = &*v16 != "";
                                            v21
                                        } else {
                                            false
                                        };
                                        let v24: bool = if v22 {
                                            let v23: bool = &*v18 != "static";
                                            v23
                                        } else {
                                            false
                                        };
                                        if v24 {
                                            US1::US1_Some(v16)
                                        } else {
                                            US1::US1_None
                                        }
                                    }
                                    _ => {
                                        US1::US1_None
                                    }
                                }
                            }
                            _ => {
                                US1::US1_None
                            }
                        }
                    }
                    _ => {
                        US1::US1_None
                    }
                };
                let v37: Rc<UH0> = match v33 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v34) => {
                        let v34: Rc<str> = v34;
                        Rc::new(UH0::UH0_Cons(v34, v1))
                    }
                };
                (v0, v1) = (v3, v37);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn test_names_in_13(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_9(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method14(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method10(v7, v8)
}
fn method17(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<UH0> = v2;
                println!("{}", v1);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn method20(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_19(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method20(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn stem_of_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v1));
    let mut v4: Rc<UH0> = split_list_9(v1, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = method22(v4, v5.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v5));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
    v12
}
fn method25(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method25(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v5));
                let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v6));
                let mut v8: Rc<UH0> = method25(v1, v7);
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn test_args_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>) -> Rc<UH0> {
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = method24(v2, v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-File"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--path"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--output-path"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--no-html"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v5));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v11));
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v12));
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v14));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v16));
    Rc::new(UH0::UH0_Cons(v6, v17))
}
fn run_run_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_LIB"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_4(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_CORE"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = get_environment_variable_4(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_COMPILER_DLL"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = get_environment_variable_4(v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = get_environment_variable_4(v6.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/run-cache"); } LIT.with(|lit| lit.clone()) }));
    let v12: US0 = setup_of_5(v1, v3, v5, v11);
    let v13: i32 = 0i32;
    let v14: Rc<UH0> = arguments_from_6(v13);
    let v19: Rc<UH0> = match Rc::unwrap_or_clone(v14) {
        UH0::UH0_Cons(v15, v16) => {
            let v15: Rc<str> = v15;
            let v16: Rc<UH0> = v16;
            v16
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    };
    let v86: US2 = match Rc::unwrap_or_clone(v19) {
        UH0::UH0_Cons(v22, v23) => {
            let v22: Rc<str> = v22;
            let v23: Rc<UH0> = v23;
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v25: bool = v22.ends_with(&*v24);
            let v26: bool = v25 == false;
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit run takes a .spi file first"); } LIT.with(|lit| lit.clone()) };
                US2::US2_RunRefused(v27)
            } else {
                match Rc::unwrap_or_clone(v23) {
                    UH0::UH0_Cons(v31, v32) => {
                        let v31: Rc<str> = v31;
                        let v32: Rc<UH0> = v32;
                        let v33: bool = "--" == &*v31;
                        if v33 {
                            US2::US2_RunProgram(v22, v32)
                        } else {
                            let v35: bool = "--list" == &*v31;
                            if v35 {
                                match &*v32 {
                                    UH0::UH0_Nil => {
                                        let v36: US3 = US3::US3_ListTests;
                                        US2::US2_RunTests(v22, v36)
                                    }
                                    _ => {
                                        let v38: bool = "--test" == &*v31;
                                        if v38 {
                                            match Rc::unwrap_or_clone(v32) {
                                                UH0::UH0_Cons(v39, v40) => {
                                                    let v39: Rc<str> = v39;
                                                    let v40: Rc<UH0> = v40;
                                                    match Rc::unwrap_or_clone(v40) {
                                                        UH0::UH0_Nil => {
                                                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                            let v42: Rc<UH0> = split_list_9(v41, v39);
                                                            let v43: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                            let v44: Rc<UH0> = method12(v42, v43);
                                                            match &*v44 {
                                                                UH0::UH0_Nil => {
                                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test needs at least one name"); } LIT.with(|lit| lit.clone()) };
                                                                    US2::US2_RunRefused(v45)
                                                                }
                                                                _ => {
                                                                    let v47: US3 = US3::US3_NamedTests(v44);
                                                                    US2::US2_RunTests(v22, v47)
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown kit run arguments; use --test a,b, --list or -- args"); } LIT.with(|lit| lit.clone()) };
                                                            US2::US2_RunRefused(v50)
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        } else {
                                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown kit run arguments; use --test a,b, --list or -- args"); } LIT.with(|lit| lit.clone()) };
                                            US2::US2_RunRefused(v54)
                                        }
                                    }
                                }
                            } else {
                                let v58: bool = "--test" == &*v31;
                                if v58 {
                                    match Rc::unwrap_or_clone(v32) {
                                        UH0::UH0_Cons(v59, v60) => {
                                            let v59: Rc<str> = v59;
                                            let v60: Rc<UH0> = v60;
                                            match Rc::unwrap_or_clone(v60) {
                                                UH0::UH0_Nil => {
                                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                    let v62: Rc<UH0> = split_list_9(v61, v59);
                                                    let v63: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v64: Rc<UH0> = method12(v62, v63);
                                                    match &*v64 {
                                                        UH0::UH0_Nil => {
                                                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test needs at least one name"); } LIT.with(|lit| lit.clone()) };
                                                            US2::US2_RunRefused(v65)
                                                        }
                                                        _ => {
                                                            let v67: US3 = US3::US3_NamedTests(v64);
                                                            US2::US2_RunTests(v22, v67)
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown kit run arguments; use --test a,b, --list or -- args"); } LIT.with(|lit| lit.clone()) };
                                                    US2::US2_RunRefused(v70)
                                                }
                                            }
                                        }
                                        UH0::UH0_Nil => {
                                            let v73: US3 = US3::US3_AllTests;
                                            US2::US2_RunTests(v22, v73)
                                        }
                                    }
                                } else {
                                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown kit run arguments; use --test a,b, --list or -- args"); } LIT.with(|lit| lit.clone()) };
                                    US2::US2_RunRefused(v77)
                                }
                            }
                        }
                    }
                    UH0::UH0_Nil => {
                        let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        US2::US2_RunProgram(v22, v29)
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit run <file.spi> [--test a,b | --list] [-- args]"); } LIT.with(|lit| lit.clone()) };
            US2::US2_RunRefused(v20)
        }
    };
    match &v86 {
        US2::US2_RunRefused(v87) => {
            let v87: Rc<str> = v87.clone();
            let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) }, v87));
            eprintln!("{}", v88);
            2i32
        }
        _ => {
            match v12 {
                US0::US0_SetupMissing(v89) => {
                    let v89: Rc<str> = v89;
                    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: kit run needs "); } LIT.with(|lit| lit.clone()) }, v89));
                    eprintln!("{}", v90);
                    2i32
                }
                US0::US0_SetupReady(v91, v92, v93, v94) => {
                    let v91: Rc<str> = v91;
                    let v92: Rc<str> = v92;
                    let v93: Rc<str> = v93;
                    let v94: Rc<str> = v94;
                    match v86 {
                        US2::US2_RunProgram(v95, v96) => {
                            let v95: Rc<str> = v95;
                            let v96: Rc<UH0> = v96;
                            let (mut v97, mut v98): (Rc<str>, Rc<str>) = kit_run_build::kit_run_built_exe(&v91, &v92, &v93, &v94, &v95);
                            let v99: bool = "" == &*v98;
                            if v99 {
                                program_ran_1(v97, v96)
                            } else {
                                let v101: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: kit run "); } LIT.with(|lit| lit.clone()) }, v98));
                                eprintln!("{}", v101);
                                1i32
                            }
                        }
                        US2::US2_RunTests(v103, v104) => {
                            let v103: Rc<str> = v103;
                            let v104: US3 = v104;
                            match &v104 {
                                US3::US3_ListTests => {
                                    let v105: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v103).unwrap_or_default().as_str());
                                    let v106: Rc<UH0> = test_names_in_13(v105);
                                    method17(v106.clone());
                                    let v107: i32 = 0i32;
                                    let v108: i32 = method18(v106, v107);
                                    let v109: Rc<str> = format_real_19(v108);
                                    let v110: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit run list ok tests="); } LIT.with(|lit| lit.clone()) }, v109));
                                    println!("{}", v110);
                                    0i32
                                }
                                _ => {
                                    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_CELLS_RUNNER"); } LIT.with(|lit| lit.clone()) };
                                    let v112: Rc<str> = get_environment_variable_4(v111);
                                    let v113: bool = &*v112 == "";
                                    if v113 {
                                        let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: kit run --test needs SPIRAL_CELLS_RUNNER (the Cells run_notebook.ps1)"); } LIT.with(|lit| lit.clone()) };
                                        eprintln!("{}", v114);
                                        2i32
                                    } else {
                                        let v117: Rc<UH0> = match v104 {
                                            US3::US3_NamedTests(v115) => {
                                                let v115: Rc<UH0> = v115;
                                                v115
                                            }
                                            _ => {
                                                { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                                            }
                                        };
                                        let v118: Rc<str> = get_environment_variable_4(v6);
                                        let v119: Rc<str> = Rc::<str>::from(v118.replace(&*v8, &*v9));
                                        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/run-cache"); } LIT.with(|lit| lit.clone()) }));
                                        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                                        let v122: Rc<str> = stem_of_21(v103.clone());
                                        let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                                        let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v123, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-tests.ipynb"); } LIT.with(|lit| lit.clone()) }));
                                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
                                        let v126: Rc<UH0> = test_args_23(v112, v103, v117, v124);
                                        program_ran_1(v125, v126)
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        run_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_run_run() -> i32 {
    closure0()()
}
use kit_run_build::SpiralShape_2617b4e49837 as Mut0;
use kit_run_build::SpiralShape_97df63ebabc3 as Mut2;
pub type SpiralShape_652accfc6a8d = Mut1;
