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
struct Mut0 { l0: i32, l1: Rc<UH0> }
struct Mut1 { l0: Rc<str> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>),
    US0_None,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method3(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method3(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16.clone(), v17.clone())
    }
}
fn baseline_row_of_spi_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v0.starts_with(&*v3);
    let mut v5: bool = v4 == false;
    let v8: bool = if v5 {
        true
    } else {
        let v6: i32 = (v2.clone().borrow().len() as i32);
        let v7: bool = v6 < 7i32;
        v7
    };
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: Rc<str> = v2.borrow()[1i32 as usize].clone();
        let v11: Rc<str> = v2.borrow()[3i32 as usize].clone();
        let mut v12: Rc<str> = v2.borrow()[5i32 as usize].clone();
        let v13: bool = &*v11 == "";
        if v13 {
            let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v14.borrow_mut().push(v10);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  "); } LIT.with(|lit| lit.clone()) };
            v14.borrow_mut().push(v15);
            v14.borrow_mut().push(v12);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  -"); } LIT.with(|lit| lit.clone()) };
            v14.borrow_mut().push(v16);
            let mut v17: Rc<Vec<Rc<str>>> = Rc::new(v14.borrow().clone());
            let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
            v20.clone()
        } else {
            let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v21.borrow_mut().push(v10);
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  "); } LIT.with(|lit| lit.clone()) };
            v21.borrow_mut().push(v22);
            v21.borrow_mut().push(v11);
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            v21.borrow_mut().push(v23);
            v21.borrow_mut().push(v12);
            let mut v24: Rc<Vec<Rc<str>>> = Rc::new(v21.borrow().clone());
            let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from((*v25.borrow()).join(&*v26));
            v27.clone()
        }
    }
}
fn method4(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method4(v3, v1.clone());
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
            let v7: Rc<str> = baseline_row_of_spi_5(v6);
            Rc::new(UH0::UH0_Cons(v7, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method6(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
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
fn baseline_rows_of_spi_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
    let v8: Rc<UH0> = split_list_1(v1, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method4(v8, v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v12: Rc<UH0> = method6(v10, v11);
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v14: i32 = 0i32;
    let v15: i32 = method7(v13.clone(), v12, v14);
    let v16: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    let v18: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v5));
    v18
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        baseline_rows_of_spi_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_9(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method10(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
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
                let mut v12: i32 = v2.wrapping_add(1i32);
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
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
fn words_in_17(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method2(v2.clone(), v5.clone());
        } else {
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let mut v9: bool = &*v7 == "";
            let v11: Rc<UH0> = if v9 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()))
            };
            (v1, v2) = (v8, v11.clone());
            continue;
        }
    }
}
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_14(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method15(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method16(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v11: i32 = 0i32;
    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = words_in_17(v10.clone(), v11, v12.clone());
    match &*v13 {
        UH0::UH0_Cons(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<UH0> = v15.clone();
            match &*v15 {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<str> = v16.clone();
                    let mut v17: Rc<UH0> = v17.clone();
                    match &*v17 {
                        UH0::UH0_Cons(v18, v19) => {
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH0> = v19.clone();
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: Option<usize> = v14.rfind(&*v20);
                            let v22: Option<usize> = method18(v21.clone());
                            let mut v23: Rc<dyn Fn((usize)) -> i32> = closure2();
                            let mut v24: Option<i32> = v22.map(|x| v23(x));
                            let v26: i32 = v24.unwrap_or(-1i32);
                            let v27: bool = &*v16 == "long";
                            let v29: bool = if v27 {
                                true
                            } else {
                                let v28: bool = &*v16 == "block";
                                v28
                            };
                            let v31: bool = if v29 {
                                true
                            } else {
                                let v30: bool = &*v16 == "flow";
                                v30
                            };
                            let v33: bool = if v31 {
                                true
                            } else {
                                let v32: bool = &*v16 == "cast";
                                v32
                            };
                            let v35: bool = if v33 {
                                true
                            } else {
                                let v34: bool = &*v16 == "arith";
                                v34
                            };
                            let v46: bool = if v35 {
                                let v36: bool = &*v18 == "long";
                                let v38: bool = if v36 {
                                    true
                                } else {
                                    let v37: bool = &*v18 == "block";
                                    v37
                                };
                                let v40: bool = if v38 {
                                    true
                                } else {
                                    let v39: bool = &*v18 == "flow";
                                    v39
                                };
                                let v42: bool = if v40 {
                                    true
                                } else {
                                    let v41: bool = &*v18 == "cast";
                                    v41
                                };
                                let v44: bool = if v42 {
                                    true
                                } else {
                                    let v43: bool = &*v18 == "arith";
                                    v43
                                };
                                let v45: bool = v44 == false;
                                v45
                            } else {
                                false
                            };
                            let v47: bool = v26 <= 0i32;
                            if v47 {
                                US0::US0_None
                            } else {
                                let v49: usize = ((0i32) as usize);
                                let v50: usize = ((v26) as usize);
                                let v51: Rc<str> = Rc::<str>::from(&(v14)[v49..v50]);
                                if v46 {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    US0::US0_Some(v51.clone(), v52.clone(), v16.clone())
                                } else {
                                    US0::US0_Some(v51.clone(), v16.clone(), v18.clone())
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
        _ => {
            US0::US0_None
        }
    }
}
fn baseline_from_13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: US0 = known_of_14(v5.clone());
            match &v6 {
                US0::US0_None => {
                    let v13: i32 = v1.wrapping_add(1i32);
                    v1 = v13;
                    continue;
                }
                US0::US0_Some(v7, v8, v9) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<str> = v8.clone();
                    let mut v9: Rc<str> = v9.clone();
                    let v10: i32 = v1.wrapping_add(1i32);
                    let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7.clone(), v8.clone(), v9.clone(), v2.clone()));
                    (v1, v2) = (v10, v11.clone());
                    continue;
                }
            }
        }
    }
}
fn baseline_of_12(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    baseline_from_13(v5.clone(), v6, v7.clone())
}
fn method19(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH1> = v5.clone();
                let v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn baseline_row_spi_21(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = format_real_9(v0);
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    v5.borrow_mut().push(v6);
    v5.borrow_mut().push(v4);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = LintKnown (\""); } LIT.with(|lit| lit.clone()) };
    v5.borrow_mut().push(v7);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let mut v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v12.borrow_mut().push(v11);
    v12.borrow_mut().push(v1);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
    v12.borrow_mut().push(v13.clone());
    v12.borrow_mut().push(v2);
    v12.borrow_mut().push(v13);
    v12.borrow_mut().push(v3);
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
    v12.borrow_mut().push(v14);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let mut v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
    v17
}
fn method20(mut v0: Rc<UH1>, mut v1: i32, mut v2: Rc<UH0>) -> (i32, Rc<UH0>) {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let v6: Rc<UH1> = v6.clone();
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: Rc<str> = baseline_row_spi_21(v1, v3.clone(), v4.clone(), v5.clone());
                let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v2.clone()));
                (v0, v1, v2) = (v6.clone(), v7, v9.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return (v1, v2.clone());
            }
        }
    }
}
fn baseline_spi_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH1> = baseline_of_12(v0);
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH1> = method19(v1, v2);
    let v4: i32 = 1i32;
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7): (i32, Rc<UH0>) = method20(v3, v4, v5);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method2(v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let mut v12: i32 = method7(v10.clone(), v9, v11);
    let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    let mut v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v17.borrow_mut().push(v16);
    v17.borrow_mut().push(v15);
    let v18: Rc<Vec<Rc<str>>> = Rc::new(v17.borrow().clone());
    let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v18).as_ref().clone()));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v21: Rc<str> = Rc::<str>::from((*v19.borrow()).join(&*v20));
    v21
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method22(v3, v1.clone());
            let mut v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method23(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn baseline_written_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = baseline_spi_11(v1.clone());
    std::fs::write(&*v0, &*v2).unwrap();
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<UH0> = split_list_1(v4, v1);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method22(v5, v6);
    let v8: i32 = 0i32;
    let mut v9: i32 = method23(v7, v8);
    format_real_9(v9)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        baseline_written_8(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_baseline_rows(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_lint_baseline_write(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
