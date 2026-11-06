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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_ScopeChanged,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_Package(Rc<str>, Rc<UH0>, Rc<UH0>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US3, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<UH0>, Rc<UH4>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>, Rc<str>),
    US5_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn after_words_4(mut v0: i32, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = v0 > 0i32;
                if v4 {
                    let v5: i32 = v0.wrapping_sub(1i32);
                    (v0, v1) = (v5, v3);
                    continue;
                } else {
                    return v1;
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
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
        while method9(v7, &v9) {
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
        method8(v16, v17)
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method12(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method10(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let v7: i32 = method11(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2.clone(), v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method12(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            let v13: bool = &*v12 != "";
            if v13 {
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
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method13(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn scope_read_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let (mut v53, mut v54, mut v55): (Rc<str>, Rc<UH0>, Rc<str>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3)
        }
        _ => {
            let (mut v50, mut v51, mut v52): (Rc<str>, Rc<UH0>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: bool = "--changed" == &*v4;
                    let v10: US1 = if v6 {
                        let v7: US2 = US2::US2_ScopeChanged;
                        US1::US1_Some(v7)
                    } else {
                        US1::US1_None
                    };
                    match &v10 {
                        US1::US1_Some(v11) => {
                            let v11: US2 = v11.clone();
                            match &*v5 {
                                UH0::UH0_Nil => {
                                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("option "); } LIT.with(|lit| lit.clone()) }, v4));
                                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no value"); } LIT.with(|lit| lit.clone()) }));
                                    (v1, v2, v13)
                                }
                                _ => {
                                    let (mut v24, mut v25, mut v26): (Rc<str>, Rc<UH0>, Rc<str>) = match Rc::unwrap_or_clone(v5) {
                                        UH0::UH0_Cons(v14, v15) => {
                                            let v14: Rc<str> = v14;
                                            let v15: Rc<UH0> = v15;
                                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                            let v17: Rc<UH0> = split_list_7(v16, v14);
                                            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let v19: Rc<UH0> = method10(v17, v18);
                                            let v20: Rc<UH0> = method13(v2, v19);
                                            scope_read_6(v15, v1, v20, v3)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v24, v25, v26)
                                }
                            }
                        }
                        _ => {
                            let (mut v44, mut v45, mut v46): (Rc<str>, Rc<UH0>, Rc<str>) = match v10 {
                                US1::US1_None => {
                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                    let v31: bool = v4.starts_with(&*v30);
                                    if v31 {
                                        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v4));
                                        (v1, v2, v32)
                                    } else {
                                        let v33: bool = &*v1 == "";
                                        let (mut v38, mut v39, mut v40): (Rc<str>, Rc<UH0>, Rc<str>) = if v33 {
                                            let (mut v34, mut v35, mut v36): (Rc<str>, Rc<UH0>, Rc<str>) = scope_read_6(v5, v4, v2, v3);
                                            (v34, v35, v36)
                                        } else {
                                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word "); } LIT.with(|lit| lit.clone()) }, v4));
                                            (v1, v2, v37)
                                        };
                                        (v38, v39, v40)
                                    }
                                }
                                _ => unreachable!(),
                            };
                            (v44, v45, v46)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v50, v51, v52)
        }
    };
    (v53, v54, v55)
}
fn scope_args_of_5(mut v0: Rc<UH0>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v3, mut v4, mut v5): (Rc<str>, Rc<UH0>, Rc<str>) = scope_read_6(v0, v1.clone(), v2, v1);
    let v6: bool = &*v5 == "";
    let v8: bool = if v6 {
        let v7: bool = &*v3 == "";
        v7
    } else {
        false
    };
    let (mut v16, mut v17, mut v18): (Rc<str>, Rc<UH0>, Rc<str>) = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected <repo root> --changed <path,path...>"); } LIT.with(|lit| lit.clone()) };
        (v3, v4, v9)
    } else {
        let v11: bool = if v6 {
            match &*v4 {
                UH0::UH0_Nil => {
                    true
                }
                _ => {
                    false
                }
            }
        } else {
            false
        };
        let (mut v13, mut v14, mut v15): (Rc<str>, Rc<UH0>, Rc<str>) = if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--changed <path,path...> is required"); } LIT.with(|lit| lit.clone()) };
            (v3, v4, v12)
        } else {
            (v3, v4, v5)
        };
        (v13, v14, v15)
    };
    (v16, v17, v18)
}
fn method14(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
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
fn method16(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn listed_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let mut v1: Rc<Vec<Rc<str>>> = Rc::new((v0).borrow().clone());
    let mut v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method16(v1, v4, v5)
}
fn method17(mut v0: Rc<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = (v0)[v4].clone();
            let v8: i32 = v1.wrapping_sub(1i32);
            let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7, v2));
            (v1, v2) = (v8, v9);
            continue;
        }
    }
}
fn package_from_19(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<RefCell<Vec<Rc<str>>>>) -> US3 {
    let v3: Rc<UH0> = listed_15(v1);
    let v4: Rc<UH0> = listed_15(v2);
    US3::US3_Package(v0, v3, v4)
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<RefCell<Vec<Rc<str>>>> = v3;
            let v4: Rc<RefCell<Vec<Rc<str>>>> = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH2> = method18(v5, v1);
            let v7: US3 = package_from_19(v2, v3, v4);
            Rc::new(UH2::UH2_Cons(v7, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn among_23(mut v0: Rc<UH0>, mut v1: Rc<str>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = &*v2 == &*v1;
                if v4 {
                    return true;
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn any_among_22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: bool = among_23(v1.clone(), v2);
                if v4 {
                    return true;
                } else {
                    v0 = v3;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn dropped_25(mut v0: Rc<UH0>, mut v1: i32) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = v1 <= 0i32;
                if v4 {
                    return v0;
                } else {
                    let v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3, v5);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn method26(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn index_in_29(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: i32) -> US4 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == &*v1;
                if v5 {
                    return US4::US4_Some(v2);
                } else {
                    let v7: i32 = v2.wrapping_add(1i32);
                    (v0, v2) = (v4, v7);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn position_28(mut v0: Rc<UH0>, mut v1: Rc<str>) -> i32 {
    let mut v2: i32 = 0i32;
    let v3: US4 = index_in_29(v0, v1, v2);
    match v3 {
        US4::US4_None => {
            0i32
        }
        US4::US4_Some(v4) => {
            let v4: i32 = v4;
            v4
        }
    }
}
fn earliest_27(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v4, v5, v6) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH3> = v6;
                let v7: bool = &*v4 == &*v1;
                let mut v9: i32 = if v7 {
                    position_28(v2.clone(), v5)
                } else {
                    v3
                };
                let v10: bool = v9 < v3;
                let v11: i32 = if v10 {
                    v9
                } else {
                    v3
                };
                (v0, v3) = (v6, v11);
                continue;
            }
            UH3::UH3_Nil => {
                return v3;
            }
        }
    }
}
fn reproved_from_24(mut v0: Rc<UH3>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<UH0> {
    let v3: i32 = 0i32;
    let v4: i32 = method26(v2.clone(), v3);
    let v5: i32 = earliest_27(v0, v1, v2.clone(), v4);
    dropped_25(v2, v5)
}
fn scope_in_21(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH3>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v4, v5) => {
            let v4: US3 = v4;
            let v5: Rc<UH2> = v5;
            match v4 {
                US3::US3_Package(v6, v7, v8) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<UH0> = v7;
                    let v8: Rc<UH0> = v8;
                    let mut v9: Rc<UH4> = scope_in_21(v5, v1.clone(), v2.clone(), v3.clone());
                    let v10: bool = any_among_22(v7, v1);
                    if v10 {
                        Rc::new(UH4::UH4_Cons(v6, v8, v9))
                    } else {
                        let v12: bool = among_23(v2, v6.clone());
                        if v12 {
                            let v13: Rc<UH0> = reproved_from_24(v3, v6.clone(), v8);
                            Rc::new(UH4::UH4_Cons(v6, v13, v9))
                        } else {
                            v9
                        }
                    }
                }
            }
        }
        UH2::UH2_Nil => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn without_spi_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    if v2 {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: i32 = v3.wrapping_sub(5i32);
        let v5: Rc<str> = string_slice(&v0, 0i32 as i64, v4 as i64);
        v5
    } else {
        v0
    }
}
fn module_in_32(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v3: bool = v1.starts_with(&*v2);
    if v3 {
        let v4: i32 = (v2.len() as i32);
        let v5: i32 = (v1.clone().len() as i32);
        let v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = string_slice(&v1, v4 as i64, v6 as i64);
        let v8: Rc<str> = without_spi_33(v7);
        US0::US0_Some(v8)
    } else {
        US0::US0_None
    }
}
fn longer_34(mut v0: US5, mut v1: Rc<str>, mut v2: Rc<str>) -> US5 {
    match &v0 {
        US5::US5_None => {
            US5::US5_Some(v2, v1)
        }
        US5::US5_Some(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: i32 = (v3.len() as i32);
            let v6: i32 = (v2.clone().len() as i32);
            let v7: bool = v5 >= v6;
            if v7 {
                v0
            } else {
                US5::US5_Some(v2, v1)
            }
        }
    }
}
fn located_in_31(mut v0: Rc<UH2>, mut v1: Rc<str>, mut v2: US5) -> US5 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: US3 = v3;
                let v4: Rc<UH2> = v4;
                match v3 {
                    US3::US3_Package(v5, v6, v7) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<UH0> = v6;
                        let v7: Rc<UH0> = v7;
                        let v8: US0 = module_in_32(v5.clone(), v1.clone());
                        match v8 {
                            US0::US0_None => {
                                v0 = v4;
                                continue;
                            }
                            US0::US0_Some(v9) => {
                                let mut v9: Rc<str> = v9;
                                let v10: US5 = longer_34(v2, v9, v5);
                                (v0, v2) = (v4, v10);
                                continue;
                            }
                        }
                    }
                }
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn hits_of_30(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: US5 = US5::US5_None;
                let v5: US5 = located_in_31(v0.clone(), v2, v4);
                match v5 {
                    US5::US5_None => {
                        v1 = v3;
                        continue;
                    }
                    US5::US5_Some(v6, v7) => {
                        let v6: Rc<str> = v6;
                        let v7: Rc<str> = v7;
                        let v8: Rc<UH3> = hits_of_30(v0, v3);
                        return Rc::new(UH3::UH3_Cons(v6, v7, v8));
                    }
                }
            }
            UH0::UH0_Nil => {
                return { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn dirs_of_35(mut v0: Rc<UH3>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH3> = v3;
            let v4: Rc<UH0> = dirs_of_35(v3);
            Rc::new(UH0::UH0_Cons(v1, v4))
        }
        UH3::UH3_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method36(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: US3 = v2;
                let v3: Rc<UH2> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn grown_38(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US3 = v2;
            let v3: Rc<UH2> = v3;
            match v2 {
                US3::US3_Package(v4, v5, v6) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: Rc<UH0> = v6;
                    let v7: Rc<UH0> = grown_38(v3, v1.clone());
                    let v8: bool = among_23(v7.clone(), v4.clone());
                    let mut v9: bool = v8 == false;
                    let v11: bool = if v9 {
                        any_among_22(v5, v1)
                    } else {
                        false
                    };
                    if v11 {
                        Rc::new(UH0::UH0_Cons(v4, v7))
                    } else {
                        v7
                    }
                }
            }
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn closed_over_37(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: i32) -> Rc<UH0> {
    loop {
        let v3: Rc<UH0> = grown_38(v0.clone(), v1.clone());
        let v4: bool = v2 <= 0i32;
        let v10: bool = if v4 {
            true
        } else {
            let v5: i32 = 0i32;
            let v6: i32 = method26(v3.clone(), v5);
            let v7: i32 = 0i32;
            let v8: i32 = method26(v1, v7);
            let mut v9: bool = v6 == v8;
            v9
        };
        if v10 {
            return v3;
        } else {
            let v11: i32 = v2.wrapping_sub(1i32);
            (v1, v2) = (v3, v11);
            continue;
        }
    }
}
fn scope_of_20(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH4> {
    let mut v2: Rc<UH3> = hits_of_30(v0.clone(), v1);
    let v3: Rc<UH0> = dirs_of_35(v2.clone());
    let v4: i32 = 0i32;
    let v5: i32 = method36(v0.clone(), v4);
    let v6: Rc<UH0> = closed_over_37(v0.clone(), v3.clone(), v5);
    scope_in_21(v0, v6, v3, v2)
}
fn scope_lines_39(mut v0: Rc<UH4>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<UH0> = v2;
            let v3: Rc<UH4> = v3;
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v6: i32 = 0i32;
            let v7: i32 = method14(v5.clone(), v2, v6);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v11));
            let v13: Rc<UH0> = scope_lines_39(v3);
            Rc::new(UH0::UH0_Cons(v12, v13))
        }
        UH4::UH4_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method40(mut v0: Rc<UH0>) -> () {
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
fn method42(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_41(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method42(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn scope_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_1(v1);
    let v3: Rc<UH0> = after_words_4(v0, v2);
    let (mut v4, mut v5, mut v6): (Rc<str>, Rc<UH0>, Rc<str>) = scope_args_of_5(v3);
    let v7: bool = &*v6 != "";
    if v7 {
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: repo scope: "); } LIT.with(|lit| lit.clone()) }, v6));
        eprintln!("{}", v8);
        2i32
    } else {
        let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = kit_proof_scope_manifests::kit_proof_scope_manifests_under(&v4);
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v11: i32 = 0i32;
        let v12: i32 = method14(v10.clone(), v5, v11);
        let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
        let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
        let v15: Rc<RefCell<Vec<Rc<str>>>> = kit_proof_scope_manifests::kit_proof_scope_anchored_paths(&v4, v14);
        let v16: Rc<UH0> = listed_15(v15);
        let v17: Rc<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>> = Rc::new((v9).borrow().clone());
        let v18: usize = (v17).len();
        let v19: i32 = ((v18) as i32);
        let v20: i32 = v19.wrapping_sub(1i32);
        let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let v22: Rc<UH1> = method17(v17, v20, v21);
        let v23: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let v24: Rc<UH2> = method18(v22, v23);
        let v25: Rc<UH4> = scope_of_20(v24, v16);
        let v26: Rc<UH0> = scope_lines_39(v25);
        method40(v26.clone());
        let v27: i32 = (v9.borrow().len() as i32);
        let v28: Rc<str> = format_real_41(v27);
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages="); } LIT.with(|lit| lit.clone()) }, v28));
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" proof_sets="); } LIT.with(|lit| lit.clone()) }));
        let v31: i32 = 0i32;
        let v32: i32 = method26(v26, v31);
        let v33: Rc<str> = format_real_41(v32);
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v33));
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit repo scope ok "); } LIT.with(|lit| lit.clone()) }, v34));
        println!("{}", v35);
        0i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        scope_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_proof_scope_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
