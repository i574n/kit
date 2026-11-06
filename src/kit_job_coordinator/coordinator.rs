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
    US0_ReceiptRead(Rc<str>, Rc<UH0>, Rc<str>),
    US0_ReceiptRefused(Rc<str>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
pub struct Mut1 { pub l0: i32 }
pub struct Mut2 { pub l0: Rc<str> }
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
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
fn value_after_7(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-build-only "); } LIT.with(|lit| lit.clone()) }, v0));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) }));
    let mut v4: i32 = (v3.clone().len() as i32);
    let v5: bool = v1.starts_with(&*v3);
    if v5 {
        let mut v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(v4);
        let mut v8: usize = ((v4) as usize);
        let mut v9: i32 = v4.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        let mut v12: i32 = (v11.clone().len() as i32);
        let v13: i32 = 0i32;
        let v14: i32 = method8(v11.clone(), v12, v13);
        let mut v15: i32 = v12.wrapping_sub(1i32);
        let v16: Rc<str> = string_slice(&v11, v14 as i64, v15 as i64);
        let v17: i32 = (v16.clone().len() as i32);
        let v18: i32 = method9(v16.clone(), v17);
        let v19: Rc<str> = string_slice(&v16, 0i32 as i64, v18 as i64);
        US1::US1_Some(v19)
    } else {
        US1::US1_None
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method10(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn receipt_folded_6(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package"); } LIT.with(|lit| lit.clone()) };
    let v5: US1 = value_after_7(v4, v3.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ebin"); } LIT.with(|lit| lit.clone()) };
    let v7: US1 = value_after_7(v6, v3.clone());
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("entry"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US1 = value_after_7(v8, v3);
    let (mut v22, mut v23, mut v24): (Rc<str>, Rc<UH0>, Rc<str>) = match v5 {
        US1::US1_Some(v10) => {
            let mut v10: Rc<str> = v10;
            (v10, v1, v2)
        }
        _ => {
            let (mut v19, mut v20, mut v21): (Rc<str>, Rc<UH0>, Rc<str>) = match v7 {
                US1::US1_Some(v11) => {
                    let mut v11: Rc<str> = v11;
                    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v12));
                    let v14: Rc<UH0> = method10(v1, v13);
                    (v0, v14, v2)
                }
                _ => {
                    let (mut v16, mut v17, mut v18): (Rc<str>, Rc<UH0>, Rc<str>) = match v9 {
                        US1::US1_Some(v15) => {
                            let mut v15: Rc<str> = v15;
                            (v0, v1, v15)
                        }
                        _ => {
                            (v0, v1, v2)
                        }
                    };
                    (v16, v17, v18)
                }
            };
            (v19, v20, v21)
        }
    };
    (v22, v23, v24)
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let (mut v15, mut v16, mut v17): (Rc<str>, Rc<UH0>, Rc<str>) = match &*v0 {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<UH0> = v5.clone();
            let (mut v6, mut v7, mut v8): (Rc<str>, Rc<UH0>, Rc<str>) = receipt_folded_6(v1, v2, v3, v4);
            method5(v5, v6, v7, v8)
        }
        _ => {
            let (mut v12, mut v13, mut v14): (Rc<str>, Rc<UH0>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Nil => {
                    (v1, v2, v3)
                }
                _ => unreachable!(),
            };
            (v12, v13, v14)
        }
    };
    (v15, v16, v17)
}
fn receipt_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = split_list_2(v1, v0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<UH0>, Rc<str>) = method5(v2, v3.clone(), v4, v3);
    let mut v8: bool = match &*v6 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the receipt names no ebin; run the build again"); } LIT.with(|lit| lit.clone()) };
        US0::US0_ReceiptRefused(v9)
    } else {
        let mut v11: bool = &*v7 == "";
        if v11 {
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the receipt names no entry module"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReceiptRefused(v12)
        } else {
            US0::US0_ReceiptRead(v5, v6, v7)
        }
    }
}
fn erl_args_11(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":main()"); } LIT.with(|lit| lit.clone()) }));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-eval"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v5));
    let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v6));
    let mut v8: Rc<UH0> = method10(v1, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-noshell"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-pa"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v8));
    Rc::new(UH0::UH0_Cons(v9, v11))
}
fn method13(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method13(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let mut v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let mut v15: Rc<str> = v4.borrow()[v14 as usize].clone();
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
fn erl_args_of_text_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: US0 = receipt_of_1(v0);
    match v1 {
        US0::US0_ReceiptRead(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<str> = v4;
            let v5: Rc<UH0> = erl_args_11(&v2, v3, v4);
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
            let v7: Rc<RefCell<Vec<Rc<str>>>> = method12(v5, v6);
            v7
        }
        US0::US0_ReceiptRefused(v8) => {
            let mut v8: Rc<str> = v8;
            Rc::new(RefCell::new(vec![]))
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<Rc<str>>>> {
        erl_args_of_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn build_args_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/apps/cells/spi/run_notebook.ps1"); } LIT.with(|lit| lit.clone()) }));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/src/kit_coordinator_actor/actor.spi"); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-File"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--path"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--build-only"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--test"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the_coordinator_serves_the_spool_its_environment_names"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--no-html"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--output-path"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v13));
    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v14));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v16));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v22));
    Rc::new(UH0::UH0_Cons(v5, v23))
}
fn method17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method18(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method21(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn built_15(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/coordinator-build.ipynb"); } LIT.with(|lit| lit.clone()) }));
    let mut v8: Rc<UH0> = build_args_16(v3, v4, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v9)));
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = method17(v8, v11);
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = method3(v12, v13);
    method18(v10.clone(), v14);
    let mut v15: Rc<std::io::Result<std::process::Output>> = Rc::new(v10.borrow_mut().output());
    let v16: bool = v15.is_ok();
    let (mut v22, mut v23, mut v24): (bool, i32, Rc<str>) = if v16 {
        let v17: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v15).as_ref().unwrap().stderr).as_ref());
        let v18: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v15).as_ref().unwrap().stdout).as_ref());
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
        let v20: i32 = (&*v15).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v20, v19)
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v21)
    };
    let v25: US0 = receipt_of_1(v24.clone());
    match v25 {
        US0::US0_ReceiptRead(v32, v33, v34) => {
            let v32: Rc<str> = v32;
            let v33: Rc<UH0> = v33;
            let v34: Rc<str> = v34;
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/coordinator.receipt"); } LIT.with(|lit| lit.clone()) }));
            std::fs::write(&*v35, &*v24).unwrap();
            let mut v37: i32 = 0i32;
            let v38: i32 = method19(v33, v37);
            let v39: Rc<str> = format_real_20(v38);
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job coordinator build ok ebins="); } LIT.with(|lit| lit.clone()) }, v39));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" package="); } LIT.with(|lit| lit.clone()) }));
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v32));
            println!("{}", v42);
            0i32
        }
        US0::US0_ReceiptRefused(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = format_real_20(v23);
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (exit "); } LIT.with(|lit| lit.clone()) }, v27));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job coordinator build: "); } LIT.with(|lit| lit.clone()) }, v26));
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
            eprintln!("{}", v31);
            1i32
        }
    }
}
fn file_exists_23(mut v0: Rc<str>) -> bool {
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
fn method24(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn directory_exists_27(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method26(v3, v1);
            let v5: bool = directory_exists_27(v2.clone());
            let mut v6: bool = v5 == false;
            if v6 {
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
fn missing_ebins_25(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: &Rc<str>) -> Rc<UH0> {
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method26(v1, v3)
}
fn method29(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US1> = Rc::new(move |mut v0: (Rc<str>)| -> US1 {
        let mut v1: Rc<str> = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn current_executable_28() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let mut v10: Option<Rc<str>> = method29(v9);
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US1> = closure2();
    let mut v12: Option<US1> = v10.map(|x| v11(x));
    let v14: US1 = US1::US1_None;
    let mut v15: US1 = v14.clone();
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
fn quoted_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("''"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }, v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
    v5
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method32(v3, v1);
            let v5: Rc<str> = quoted_31(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method33(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn start_script_30(mut v0: &Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = quoted_31(v3);
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$env:KIT_COORDINATOR_SPOOL = "); } LIT.with(|lit| lit.clone()) }, v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; $env:KIT_COORDINATOR_KIT = "); } LIT.with(|lit| lit.clone()) }));
    let mut v8: Rc<str> = quoted_31(v4);
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; & erl "); } LIT.with(|lit| lit.clone()) }));
    let mut v11: Rc<UH0> = erl_args_11(v0, v1, v2);
    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method32(v11, v12);
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v15: i32 = 0i32;
    let v16: i32 = method33(v14.clone(), v13, v15);
    let v17: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v20));
    v21
}
fn started_22(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/coordinator.receipt"); } LIT.with(|lit| lit.clone()) }));
    let mut v8: bool = file_exists_23(v7.clone());
    let v21: Rc<str> = if v8 {
        let mut v10: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v7);
        let mut v12: Vec<u8> = v10.unwrap();
        let v13: Vec<u8> = method24(v12);
        let mut v15: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v13);
        let v17: std::string::String = v15.unwrap();
        let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
        v19
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v20
    };
    let v22: US0 = receipt_of_1(v21);
    match v22 {
        US0::US0_ReceiptRead(v26, v27, v28) => {
            let v26: Rc<str> = v26;
            let v27: Rc<UH0> = v27;
            let v28: Rc<str> = v28;
            let v29: Rc<UH0> = missing_ebins_25(&v26, v27.clone(), &v28);
            let v30: bool = match Rc::unwrap_or_clone(v29) {
                UH0::UH0_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            let v31: bool = v30 != true;
            if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job coordinator start: a built ebin is gone; run kit job coordinator build"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", v32);
                2i32
            } else {
                let v33: bool = &*v5 == "";
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job coordinator start: --spool <dir> is required"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v34);
                    2i32
                } else {
                    let v35: Rc<str> = current_executable_28();
                    let v36: Rc<str> = start_script_30(&v26, v27, v28, v5, v35.clone());
                    let mut v37: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v35)));
                    let v38: bool = &*v36 == "";
                    let v70: Rc<UH0> = if v38 {
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-Command"); } LIT.with(|lit| lit.clone()) };
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("604800000"); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coordinator"); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v46));
                        let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v44, v47));
                        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v48));
                        let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v49));
                        let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41, v50));
                        let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v51));
                        Rc::new(UH0::UH0_Cons(v39, v52))
                    } else {
                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-Command"); } LIT.with(|lit| lit.clone()) };
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-NoProfile"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh"); } LIT.with(|lit| lit.clone()) };
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("604800000"); } LIT.with(|lit| lit.clone()) };
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit-coordinator"); } LIT.with(|lit| lit.clone()) };
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("start"); } LIT.with(|lit| lit.clone()) };
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job"); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v62: Rc<UH0> = Rc::new(UH0::UH0_Cons(v60, v61));
                        let v63: Rc<UH0> = Rc::new(UH0::UH0_Cons(v59, v62));
                        let v64: Rc<UH0> = Rc::new(UH0::UH0_Cons(v58, v63));
                        let v65: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57, v64));
                        let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v56, v65));
                        let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v55, v66));
                        let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v54, v67));
                        Rc::new(UH0::UH0_Cons(v36, v68))
                    };
                    let v71: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v72: Rc<UH0> = method3(v70, v71);
                    method18(v37.clone(), v72);
                    let mut v73: Rc<std::io::Result<std::process::Output>> = Rc::new(v37.borrow_mut().output());
                    let v74: bool = v73.is_ok();
                    let (mut v80, mut v81, mut v82): (bool, i32, Rc<str>) = if v74 {
                        let v75: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v73).as_ref().unwrap().stderr).as_ref());
                        let v76: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v73).as_ref().unwrap().stdout).as_ref());
                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v75));
                        let v78: i32 = (&*v73).as_ref().unwrap().status.code().unwrap_or(-2);
                        (true, v78, v77)
                    } else {
                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (false, -1i32, v79)
                    };
                    print!("{}", v82);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let v84: bool = if v80 {
                        let v83: bool = v81 == 0i32;
                        v83
                    } else {
                        false
                    };
                    if v84 {
                        0i32
                    } else {
                        1i32
                    }
                }
            }
        }
        US0::US0_ReceiptRefused(v23) => {
            let v23: Rc<str> = v23;
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job coordinator start: "); } LIT.with(|lit| lit.clone()) }, v23));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; run kit job coordinator build"); } LIT.with(|lit| lit.clone()) }));
            eprintln!("{}", v25);
            2i32
        }
    }
}
fn coordinator_run_14(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<RefCell<Vec<(bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>)>>> = kit_job_coordinator_args::kit_job_coordinator_args_read(2i32);
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (bool, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = v1.borrow()[0i32 as usize].clone();
    let mut v8: bool = &*v7 != "";
    if v8 {
        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: job coordinator: "); } LIT.with(|lit| lit.clone()) }, v7));
        eprintln!("{}", v9);
        2i32
    } else {
        if v2 {
            built_15(v0, v2, v3, v4, v5, v6, v7)
        } else {
            started_22(v0, v2, v3, v4, v5, v6, v7)
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        coordinator_run_14(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_coordinator_erl_args(v0: &str) -> Rc<RefCell<Vec<Rc<str>>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_job_coordinator_run(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
pub type SpiralShape_97df63ebabc3 = Mut2;
