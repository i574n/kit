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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, i32, i32),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
pub struct Mut1 { pub l0: i32 }
pub struct Mut2 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method3(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16, v17)
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
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
                        let v8: bool = v4 == b'\n';
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
fn method8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
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
fn method9(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_in_6(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = 0i32;
    let v8: i32 = method7(v5.clone(), v6, v7);
    let mut v9: i32 = v6.wrapping_sub(1i32);
    let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
    let v11: i32 = (v10.clone().len() as i32);
    let v12: i32 = method8(v10.clone(), v11);
    let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
    let mut v14: Option<i32> = (v13).trim().parse::<i32>().ok();
    let v15: Option<i32> = method9(v14);
    let mut v16: Rc<dyn Fn((i32)) -> US1> = closure1();
    let mut v17: Option<US1> = v15.map(|x| v16(x));
    let v19: US1 = US1::US1_None;
    let v20: US1 = v19.clone();
    let v21: US1 = v17.unwrap_or(v20);
    match v21 {
        US1::US1_None => {
            -1i32
        }
        US1::US1_Some(v22) => {
            let v22: i32 = v22;
            v22
        }
    }
}
fn entry_in_5(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v0.starts_with(&*v3);
    let v5: bool = v4 == false;
    let v8: bool = if v5 {
        true
    } else {
        let v6: i32 = (v2.clone().borrow().len() as i32);
        let v7: bool = v6 < 5i32;
        v7
    };
    if v8 {
        US0::US0_None
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v12: Rc<UH0> = split_list_1(v10, v11);
        match Rc::unwrap_or_clone(v12) {
            UH0::UH0_Cons(v13, v14) => {
                let mut v13: Rc<str> = v13;
                let mut v14: Rc<UH0> = v14;
                match Rc::unwrap_or_clone(v14) {
                    UH0::UH0_Cons(v15, v16) => {
                        let v15: Rc<str> = v15;
                        let mut v16: Rc<UH0> = v16;
                        match Rc::unwrap_or_clone(v16) {
                            UH0::UH0_Cons(v17, v18) => {
                                let mut v17: Rc<str> = v17;
                                let v18: Rc<UH0> = v18;
                                let v19: Rc<str> = v2.borrow()[1i32 as usize].clone();
                                let v20: Rc<str> = v2.borrow()[3i32 as usize].clone();
                                let v21: i32 = number_in_6(v15);
                                let v22: i32 = number_in_6(v17);
                                US0::US0_Some(v19, v20, v21, v22)
                            }
                            _ => {
                                US0::US0_None
                            }
                        }
                    }
                    _ => {
                        US0::US0_None
                    }
                }
            }
            _ => {
                US0::US0_None
            }
        }
    }
}
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: US0 = entry_in_5(v2);
                let v33: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = match v4 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: i32 = v7;
                        let v8: i32 = v8;
                        let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![(v5, v6, v7, v8)]));
                        let v10: i32 = (v1.clone().borrow().len() as i32);
                        let v11: i32 = (v9.clone().borrow().len() as i32);
                        let v12: i32 = v10.wrapping_add(v11);
                        let mut v13: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, i32)>::default(); v12 as usize]));
                        let mut v14: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                        while method10(v12, &v14) {
                            let mut v16: i32 = v14.borrow().l0;
                            let mut v17: bool = v16 < v10;
                            let (mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, i32, i32) = if v17 {
                                let (mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, i32, i32) = v1.borrow()[v16 as usize].clone();
                                (v18, v19, v20, v21)
                            } else {
                                let v22: i32 = v16.wrapping_sub(v10);
                                let (mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, i32) = v9.borrow()[v22 as usize].clone();
                                (v23, v24, v25, v26)
                            };
                            v13.borrow_mut()[v16 as usize] = (v27, v28, v29, v30);
                            let v31: i32 = v16.wrapping_add(1i32);
                            v14.borrow_mut().l0 = v31;
                            ()
                        };
                        v13
                    }
                };
                (v0, v1) = (v3, v33);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn baseline_entries_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_1(v1, v4);
    let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![]));
    let v7: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = method4(v5, v6);
    v7
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
        baseline_entries_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn binding_of_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    v4
}
fn method16(mut v0: &Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_15(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method16(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn row_of_13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = binding_of_14(v0.clone());
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) }, v4));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = binding_of_14(v2.clone());
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CensusCount (\""); } LIT.with(|lit| lit.clone()) }, v0));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v2));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v13));
    let v15: Rc<str> = format_real_15(v3);
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32, "); } LIT.with(|lit| lit.clone()) }));
    let v18: Rc<str> = format_real_15(v1);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) }));
    v20
}
fn appended_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
        v4
    }
}
fn rendered_from_12(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: i32 = (v2.clone().borrow().len() as i32);
        let v6: bool = v3 >= v5;
        if v6 {
            return v4;
        } else {
            let v7: i32 = v3.wrapping_add(1i32);
            let (mut v8, mut v9): (Rc<str>, i32) = v2.borrow()[v3 as usize].clone();
            let v10: Rc<str> = row_of_13(v0.clone(), v1, v8, v9);
            let v11: Rc<str> = appended_17(v4, v10);
            (v3, v4) = (v7, v11);
            continue;
        }
    }
}
fn baseline_rendered_11(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let v4: i32 = 0i32;
    rendered_from_12(v3, v2, v1, v4, v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i32, Rc<RefCell<Vec<(Rc<str>, i32)>>>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<RefCell<Vec<(Rc<str>, i32)>>>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v3: Rc<str>| -> Rc<str> {
        baseline_rendered_11(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn row_package_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = v2 ;
    let v4: i32 = (v3.clone().borrow().len() as i32);
    let v5: bool = 1i32 < v4;
    let mut v9: US2 = if v5 {
        let v6: Rc<str> = v3.borrow()[1i32 as usize].clone();
        US2::US2_Some(v6)
    } else {
        US2::US2_None
    };
    match v9 {
        US2::US2_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn walked_has_21(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 < v3;
        if v4 {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: bool = &*v5 == &*v1;
            if v6 {
                return true;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        } else {
            return false;
        }
    }
}
fn method19(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl census_"); } LIT.with(|lit| lit.clone()) };
                let v6: bool = v3.starts_with(&*v5);
                let v10: bool = if v6 {
                    let v7: Rc<str> = row_package_20(v3.clone());
                    let v8: i32 = 0i32;
                    walked_has_21(v0, v7, v8)
                } else {
                    false
                };
                let v11: bool = v10 == false;
                let mut v13: Rc<str> = if v11 {
                    appended_17(v2, v3)
                } else {
                    v2
                };
                (v1, v2) = (v4, v13);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn baseline_merged_18(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH0> = split_list_1(v3, v2);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = method19(&v1, v4, v5);
    appended_17(v6, v0)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<str>| -> Rc<str> {
        baseline_merged_18(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_style_baseline_entries(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_style_baseline_rendered(v0: &str, v1: i32, v2: Rc<RefCell<Vec<(Rc<str>, i32)>>>, v3: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3))
}
pub fn kit_style_baseline_merged(v0: &str, v1: Rc<RefCell<Vec<Rc<str>>>>, v2: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
pub type SpiralShape_97df63ebabc3 = Mut2;
