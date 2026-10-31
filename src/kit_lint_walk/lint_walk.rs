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
    US0_MacroLong,
    US0_MacroBlock,
    US0_MacroFlow,
    US0_MacroCast,
    US0_MacroArith,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US0),
    US2_None,
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn directory_exists_3(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method5(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
fn entry_names_4(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v9.clone()));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method5(v18.clone(), v19.clone())
}
fn method8(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method8(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn method6(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v8: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = &*v3 == "target";
                    v7
                };
                let mut v10: bool = if v8 {
                    true
                } else {
                    let v9: bool = &*v3 == "node_modules";
                    v9
                };
                let v12: bool = if v10 {
                    true
                } else {
                    let mut v11: bool = &*v3 == "_build";
                    v11
                };
                let mut v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = &*v3 == "deps";
                    v13
                };
                let v16: bool = if v14 {
                    true
                } else {
                    let v15: bool = &*v3 == "bin";
                    v15
                };
                let v18: bool = if v16 {
                    true
                } else {
                    let v17: bool = &*v3 == "obj";
                    v17
                };
                let v21: Rc<UH0> = if v18 {
                    v2.clone()
                } else {
                    let v19: Rc<str> = method7(v0.clone(), v3.clone());
                    Rc::new(UH0::UH0_Cons(v19.clone(), v2.clone()))
                };
                (v0, v1, v2) = (v0.clone(), v4.clone(), v21.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn spi_files_1(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: bool = file_exists_2(v4.clone());
                if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v8: bool = v4.ends_with(&*v7);
                    let mut v10: Rc<UH0> = if v8 {
                        Rc::new(UH0::UH0_Cons(v4.clone(), v1.clone()))
                    } else {
                        v1.clone()
                    };
                    (v0, v1) = (v5.clone(), v10.clone());
                    continue;
                } else {
                    let v12: bool = directory_exists_3(v4.clone());
                    if v12 {
                        let v13: Rc<UH0> = entry_names_4(v4.clone());
                        let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v15: Rc<UH0> = method5(v13.clone(), v14.clone());
                        let v16: Rc<UH0> = method6(v4.clone(), v15.clone(), v5.clone());
                        (v0, v1) = (v16.clone(), v1.clone());
                        continue;
                    } else {
                        (v0, v1) = (v5.clone(), v1.clone());
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method5(v1.clone(), v2.clone());
            }
        }
    }
}
fn method9(mut v0: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<str>>>>, mut v1: Rc<UH0>) -> () {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                v0.borrow_mut().push(v2);
                (v0, v1) = (v0.clone(), v3.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn files_under_0(mut v0: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v1: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v2));
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = spi_files_1(v3, v4);
    method9(v1.clone(), v5);
    let mut v6: std::rc::Rc<Vec<std::rc::Rc<str>>> = std::rc::Rc::new(v1.borrow().clone());
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>>> = Rc::new(move |mut v0: Rc<str>| -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
        files_under_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn class_word_14(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_MacroArith => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US0::US0_MacroBlock => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US0::US0_MacroCast => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_MacroFlow => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_MacroLong => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn method13(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<UH1>) -> US1 {
    loop {
        match &*v5 {
            UH1::UH1_Cons(v7, v8, v9, v10) => {
                let mut v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let mut v10: Rc<UH1> = v10.clone();
                let mut v11: bool = &*v7 == &*v0;
                let v13: bool = if v11 {
                    let v12: bool = &*v8 == &*v2;
                    v12
                } else {
                    false
                };
                let v16: bool = if v13 {
                    let mut v14: Rc<str> = class_word_14(v3.clone());
                    let v15: bool = &*v9 == &*v14;
                    v15
                } else {
                    false
                };
                if v16 {
                    return US1::US1_Some(v7.clone(), v8.clone(), v9.clone());
                } else {
                    (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v10.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn known_12(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> bool {
    let mut v6: US1 = method13(v1, v2, v3, v4, v5, v0);
    let mut v7: bool = match &v6 {
        US1::US1_None => {
            true
        }
        _ => {
            false
        }
    };
    let v8: bool = v7 != true;
    v8
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                let mut v6: bool = v4 == b'\t';
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
fn words_in_20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method5(v2.clone(), v5.clone());
        } else {
            let mut v7: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let v9: bool = &*v7 == "";
            let mut v11: Rc<UH0> = if v9 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v8, v11.clone());
            continue;
        }
    }
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_17(mut v0: Rc<str>) -> US1 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: i32 = method18(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method19(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v11: i32 = 0i32;
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = words_in_20(v10.clone(), v11, v12.clone());
    match &*v13 {
        UH0::UH0_Cons(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<UH0> = v15.clone();
            match &*v15 {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<UH0> = v17.clone();
                    match &*v17 {
                        UH0::UH0_Cons(v18, v19) => {
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH0> = v19.clone();
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v21: Option<usize> = v14.rfind(&*v20);
                            let v22: Option<usize> = method16(v21.clone());
                            let mut v23: Rc<dyn Fn((usize)) -> i32> = closure3();
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
                                US1::US1_None
                            } else {
                                let v49: usize = ((0i32) as usize);
                                let v50: usize = ((v26) as usize);
                                let v51: Rc<str> = Rc::<str>::from(&(v14)[v49..v50]);
                                if v46 {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    US1::US1_Some(v51.clone(), v52.clone(), v16.clone())
                                } else {
                                    US1::US1_Some(v51.clone(), v16.clone(), v18.clone())
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
            }
        }
        _ => {
            US1::US1_None
        }
    }
}
fn rows_of_file_15(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: Option<usize> = v6.find(&*v7);
            let v9: Option<usize> = method16(v8.clone());
            let mut v10: Rc<dyn Fn((usize)) -> i32> = closure2();
            let mut v11: Option<i32> = v9.map(|x| v10(x));
            let v13: i32 = v11.unwrap_or(-1i32);
            let mut v14: bool = v13 < 0i32;
            let v18: Rc<str> = if v14 {
                v6.clone()
            } else {
                let v15: usize = ((0i32) as usize);
                let v16: usize = ((v13) as usize);
                let v17: Rc<str> = Rc::<str>::from(&(v6)[v15..v16]);
                v17.clone()
            };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v19)), v18));
            let v22: US1 = known_of_17(v21.clone());
            match &v22 {
                US1::US1_None => {
                    let v29: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v29, v3.clone());
                    continue;
                }
                US1::US1_Some(v23, v24, v25) => {
                    let mut v23: Rc<str> = v23.clone();
                    let mut v24: Rc<str> = v24.clone();
                    let v25: Rc<str> = v25.clone();
                    let v26: i32 = v2.wrapping_add(1i32);
                    let v27: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23.clone(), v24.clone(), v25.clone(), v3.clone()));
                    (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v26, v27.clone());
                    continue;
                }
            }
        }
    }
}
fn known_text_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> bool {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v0.replace(&*v10, &*v11));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v12));
    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v13.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v15: i32 = 1i32;
    let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH1> = rows_of_file_15(v1.clone(), v14.clone(), v15, v16.clone());
    known_12(v17.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone())
}
fn known_in_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
    let mut v4: bool = v3 == 0i32;
    let mut v24: US2 = if v4 {
        let v5: US0 = US0::US0_MacroLong;
        US2::US2_Some(v5)
    } else {
        let mut v7: bool = v3 == 1i32;
        if v7 {
            let v8: US0 = US0::US0_MacroBlock;
            US2::US2_Some(v8)
        } else {
            let mut v10: bool = v3 == 2i32;
            if v10 {
                let mut v11: US0 = US0::US0_MacroFlow;
                US2::US2_Some(v11)
            } else {
                let v13: bool = v3 == 3i32;
                if v13 {
                    let mut v14: US0 = US0::US0_MacroCast;
                    US2::US2_Some(v14)
                } else {
                    let v16: bool = v3 == 4i32;
                    if v16 {
                        let v17: US0 = US0::US0_MacroArith;
                        US2::US2_Some(v17)
                    } else {
                        US2::US2_None
                    }
                }
            }
        }
    };
    match &v24 {
        US2::US2_None => {
            false
        }
        US2::US2_Some(v25) => {
            let v25: US0 = v25.clone();
            let v26: i32 = 0i32;
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            known_text_11(v0, v1, v26, v2, v25, v27)
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32| -> bool {
        known_in_10(v0.clone(), v1.clone(), v2.clone(), v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_walk_files(v0: &str) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_lint_known(v0: &str, v1: &str, v2: &str, v3: i32) -> bool {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
