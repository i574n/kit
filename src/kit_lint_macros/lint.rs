#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_MacroLong,
    US1_MacroBlock,
    US1_MacroFlow,
    US1_MacroCast,
    US1_MacroArith,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<str>, US1, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US1),
    US2_None,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5.clone())
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_Some(v9) => {
            let v9: Rc<str> = v9.clone();
            v9.clone()
        }
    }
}
fn value_after_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v2 <= v3;
        if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_3(v1);
            let mut v7: bool = &*v6 == &*v0;
            if v7 {
                return argument_3(v3);
            } else {
                (v0, v1, v2) = (v0.clone(), v3, v2);
                continue;
            }
        }
    }
}
fn file_exists_6(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method8(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method8(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn directory_exists_9(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn paths_from_11(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 <= v0;
        let mut v7: bool = if v3 {
            true
        } else {
            let mut v4: Rc<str> = argument_3(v0);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v4.starts_with(&*v5);
            v6
        };
        if v7 {
            let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method12(v2.clone(), v8.clone());
        } else {
            let v10: i32 = v0.wrapping_add(1i32);
            let v11: Rc<str> = argument_3(v0);
            let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v2.clone()));
            (v0, v1, v2) = (v10, v1, v12.clone());
            continue;
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method14(v0.clone(), v4, v2.clone());
            let mut v6: Rc<str> = method7(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method15(v3, v1.clone());
            let v5: bool = file_exists_6(v2.clone());
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
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let mut v4: Rc<UH0> = method16(v3, v1.clone());
            let v5: bool = file_exists_6(v2.clone());
            let mut v6: bool = v5 == false;
            if v6 {
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
fn entry_names_18(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let v9: Rc<UH0> = acc;
    let v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v9.clone()));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method12(v18.clone(), v19.clone())
}
fn method19(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
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
                let v10: bool = if v8 {
                    true
                } else {
                    let v9: bool = &*v3 == "node_modules";
                    v9
                };
                let v12: bool = if v10 {
                    true
                } else {
                    let v11: bool = &*v3 == "_build";
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
fn spi_files_17(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: bool = file_exists_6(v4.clone());
                if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v8: bool = v4.ends_with(&*v7);
                    let v10: Rc<UH0> = if v8 {
                        Rc::new(UH0::UH0_Cons(v4.clone(), v1.clone()))
                    } else {
                        v1.clone()
                    };
                    (v0, v1) = (v5.clone(), v10.clone());
                    continue;
                } else {
                    let v12: bool = directory_exists_9(v4.clone());
                    if v12 {
                        let v13: Rc<UH0> = entry_names_18(v4.clone());
                        let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v15: Rc<UH0> = method12(v13.clone(), v14.clone());
                        let v16: Rc<UH0> = method19(v4.clone(), v15.clone(), v5.clone());
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
                return method12(v1.clone(), v2.clone());
            }
        }
    }
}
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn added_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_lint_scan::kit_lint_added_text(&*v0);
    v1
}
fn relative_to_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v7: i32 = (v5.clone().len() as i32);
    let v8: bool = v6.starts_with(&*v5);
    if v8 {
        let v9: i32 = (v6.clone().len() as i32);
        let v10: i32 = v9.wrapping_sub(v7);
        let v11: usize = ((v7) as usize);
        let v12: i32 = v7.wrapping_add(v10);
        let v13: usize = ((v12) as usize);
        let mut v14: Rc<str> = Rc::<str>::from(&(v6)[v11..v13]);
        v14.clone()
    } else {
        v6.clone()
    }
}
fn scan_of_25(mut v0: Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = kit_lint_scan::kit_lint_scan(&*v0);
    (v1, v2, v3, v4, v5, v6)
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let mut v4: Rc<str> = v4.clone();
                let v5: US1 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v1.clone()));
                (v0, v1) = (v7.clone(), v8.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn class_at_28(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> i32 {
    let mut v4: i32 = kit_lint_classify::kit_lint_class(&*v0, v1, v2, v3);
    v4
}
fn walk_26(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: Rc<RefCell<Vec<i32>>>, mut v3: Rc<RefCell<Vec<bool>>>, mut v4: Rc<RefCell<Vec<Rc<str>>>>, mut v5: Rc<RefCell<Vec<Rc<str>>>>, mut v6: Rc<RefCell<Vec<i32>>>, mut v7: i32, mut v8: i32, mut v9: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v10: bool = v8 >= v7;
        if v10 {
            let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method27(v9.clone(), v11.clone());
        } else {
            let v13: Rc<str> = v4.clone().borrow()[v8 as usize].clone();
            let mut v14: bool = v3.clone().borrow()[v8 as usize].clone();
            let v15: i32 = v2.clone().borrow()[v8 as usize].clone();
            let v16: i32 = v1.clone().borrow()[v8 as usize].clone();
            let v17: i32 = class_at_28(v13.clone(), v14, v15, v16);
            let v18: bool = v17 == 0i32;
            let v38: US2 = if v18 {
                let v19: US1 = US1::US1_MacroLong;
                US2::US2_Some(v19.clone())
            } else {
                let v21: bool = v17 == 1i32;
                if v21 {
                    let v22: US1 = US1::US1_MacroBlock;
                    US2::US2_Some(v22.clone())
                } else {
                    let v24: bool = v17 == 2i32;
                    if v24 {
                        let v25: US1 = US1::US1_MacroFlow;
                        US2::US2_Some(v25.clone())
                    } else {
                        let v27: bool = v17 == 3i32;
                        if v27 {
                            let v28: US1 = US1::US1_MacroCast;
                            US2::US2_Some(v28.clone())
                        } else {
                            let v30: bool = v17 == 4i32;
                            if v30 {
                                let v31: US1 = US1::US1_MacroArith;
                                US2::US2_Some(v31.clone())
                            } else {
                                US2::US2_None
                            }
                        }
                    }
                }
            };
            match &v38 {
                US2::US2_None => {
                    let v45: i32 = v8.wrapping_add(1i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7, v45, v9.clone());
                    continue;
                }
                US2::US2_Some(v39) => {
                    let v39: US1 = v39.clone();
                    let v40: i32 = v6.clone().borrow()[v8 as usize].clone();
                    let v41: Rc<str> = v5.clone().borrow()[v8 as usize].clone();
                    let v42: i32 = v8.wrapping_add(1i32);
                    let v43: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v40, v41.clone(), v39.clone(), v13.clone(), v9.clone()));
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7, v42, v43.clone());
                    continue;
                }
            }
        }
    }
}
fn file_findings_24(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, Rc<UH1>) {
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = scan_of_25(v1.clone());
    let v8: i32 = (v2.clone().borrow().len() as i32);
    let v9: i32 = 0i32;
    let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = walk_26(v0.clone(), v7.clone(), v6.clone(), v5.clone(), v4.clone(), v3.clone(), v2.clone(), v8, v9, v10.clone());
    (v8, v11.clone())
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let mut v4: Rc<str> = v4.clone();
                let v5: US1 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v1.clone()));
                (v0, v1) = (v7.clone(), v8.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<UH1>) -> (i32, Rc<UH1>) {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
                let v9: Vec<u8> = v7.unwrap();
                let v10: Vec<u8> = method10(v9.clone());
                let v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
                let mut v14: std::string::String = v12.unwrap();
                let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".diff"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v4.ends_with(&*v17);
                let v21: bool = if v18 {
                    true
                } else {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".patch"); } LIT.with(|lit| lit.clone()) };
                    let v20: bool = v4.ends_with(&*v19);
                    v20
                };
                let v23: Rc<str> = if v21 {
                    added_22(v16.clone())
                } else {
                    v16.clone()
                };
                let v24: Rc<str> = relative_to_23(v0.clone(), v4.clone());
                let (mut v25, mut v26): (i32, Rc<UH1>) = file_findings_24(v24.clone(), v23.clone());
                let v27: i32 = v2.wrapping_add(v25);
                let v28: Rc<UH1> = method29(v26.clone(), v3.clone());
                (v0, v1, v2, v3) = (v0.clone(), v5.clone(), v27, v28.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3.clone());
            }
        }
    }
}
fn known_in_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>) -> bool {
    let v10: i32 = match &v4 {
        US1::US1_MacroArith => {
            4i32
        }
        US1::US1_MacroBlock => {
            1i32
        }
        US1::US1_MacroCast => {
            3i32
        }
        US1::US1_MacroFlow => {
            2i32
        }
        US1::US1_MacroLong => {
            0i32
        }
    };
    let v11: bool = kit_lint_classify::kit_lint_known(&*v0, &*v1, &*v3, v10);
    v11
}
fn method30(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: i32 = *v4;
            let v5: Rc<str> = v5.clone();
            let mut v6: US1 = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let v8: Rc<UH1> = v8.clone();
            let v9: Rc<UH1> = method30(v0.clone(), v8, v2.clone());
            let v10: bool = known_in_31(v0, v3.clone(), v4, v5.clone(), v6.clone(), v7.clone());
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v7, v9.clone()))
            } else {
                v9
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method33(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let mut v4: Rc<str> = v4.clone();
                let v5: US1 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: i32 = (v4.clone().len() as i32);
                let v9: bool = v8 > v1;
                let v10: i32 = if v9 {
                    v8
                } else {
                    v1
                };
                (v0, v1) = (v7.clone(), v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn name_width_32(mut v0: Rc<UH1>) -> i32 {
    let v1: i32 = 0i32;
    method33(v0, v1)
}
fn method34(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let mut v4: Rc<str> = v4.clone();
                let v5: US1 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v7.clone(), v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method35(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
fn row_of_37(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>) -> Rc<str> {
    let v10: i32 = match &v4 {
        US1::US1_MacroArith => {
            4i32
        }
        US1::US1_MacroBlock => {
            1i32
        }
        US1::US1_MacroCast => {
            3i32
        }
        US1::US1_MacroFlow => {
            2i32
        }
        US1::US1_MacroLong => {
            0i32
        }
    };
    let v11: Rc<str> = kit_lint_scan::kit_lint_row(v0, &*v1, v2, &*v3, v10, &*v5);
    v11
}
fn method36(mut v0: i32, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: i32 = *v4;
            let v5: Rc<str> = v5.clone();
            let mut v6: US1 = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let v8: Rc<UH1> = v8.clone();
            let v9: Rc<UH0> = method36(v0, v8, v2.clone());
            let v10: Rc<str> = row_of_37(v0, v3, v4, v5, v6, v7);
            Rc::new(UH0::UH0_Cons(v10, v9))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn lint_report_13(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> (Rc<UH0>, i32, i32, i32, i32) {
    let mut v7: Rc<UH0> = match &*v1 {
        UH0::UH0_Nil => {
            let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v0.clone(), v3.clone()))
        }
        _ => {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method14(v0.clone(), v1.clone(), v5.clone())
        }
    };
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = method15(v7.clone(), v8.clone());
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method16(v7.clone(), v10.clone());
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = spi_files_17(v11.clone(), v12.clone());
    let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = method12(v9.clone(), v14.clone());
    let v16: Rc<UH0> = method20(v13.clone(), v15.clone());
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = method12(v16.clone(), v17.clone());
    let v19: i32 = 0i32;
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v21, mut v22): (i32, Rc<UH1>) = method21(v0.clone(), v18.clone(), v19, v20.clone());
    let v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v24: Rc<UH1> = method27(v22.clone(), v23.clone());
    let v25: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v26: Rc<UH1> = method30(v2.clone(), v24.clone(), v25.clone());
    let v27: i32 = name_width_32(v26.clone());
    let v28: i32 = 0i32;
    let v29: i32 = method34(v24.clone(), v28);
    let v30: i32 = 0i32;
    let v31: i32 = method35(v18.clone(), v30);
    let v32: i32 = 0i32;
    let v33: i32 = method34(v26.clone(), v32);
    let v34: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH0> = method36(v27, v26.clone(), v34.clone());
    (v35.clone(), v31, v21, v29, v33)
}
fn method38(mut v0: Rc<UH0>) -> () {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1.clone();
                let v2: Rc<UH0> = v2.clone();
                println!("{}", v1);
                v0 = v2.clone();
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn elapsed_text_39(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_41() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_40() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_41();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn tally_text_42(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = kit_lint_scan::kit_lint_tally(v0, v1, v2, v3);
    v4
}
fn lint_run_0() -> i32 {
    let mut v0: u64 = now_ms_1();
    let v1: i32 = command_line_argument_count_2();
    let v2: i32 = 2i32;
    let v3: Rc<str> = argument_3(v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--baseline"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = 3i32;
    let mut v6: Rc<str> = value_after_5(v4.clone(), v5, v1);
    let mut v7: bool = &*v6 == "";
    let v9: bool = if v7 {
        true
    } else {
        file_exists_6(v6.clone())
    };
    let v11: Rc<str> = if v9 {
        v6.clone()
    } else {
        method7(v3.clone(), v6.clone())
    };
    let v12: bool = &*v3 == "";
    let v15: bool = if v12 {
        true
    } else {
        let v13: bool = directory_exists_9(v3.clone());
        let mut v14: bool = v13 == false;
        v14
    };
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint usage: lint macros <root> [<path>...] [--baseline <rows>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v16);
        2i32
    } else {
        let v17: bool = &*v11 != "";
        let v20: bool = if v17 {
            let v18: bool = file_exists_6(v11.clone());
            let v19: bool = v18 == false;
            v19
        } else {
            false
        };
        if v20 {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint macros: no baseline file at "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v6));
            eprintln!("{}", v22);
            2i32
        } else {
            let v23: bool = &*v11 == "";
            let v36: Rc<str> = if v23 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            } else {
                let v26: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v11);
                let v28: Vec<u8> = v26.unwrap();
                let v29: Vec<u8> = method10(v28.clone());
                let v31: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v29);
                let v33: std::string::String = v31.unwrap();
                let v35: Rc<str> = Rc::<str>::from(String::as_str(&v33));
                v35.clone()
            };
            let v37: i32 = 3i32;
            let v38: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v39: Rc<UH0> = paths_from_11(v37, v1, v38.clone());
            let (mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<UH0>, i32, i32, i32, i32) = lint_report_13(v3.clone(), v39.clone(), v36.clone());
            method38(v40.clone());
            let v45: Rc<str> = elapsed_text_39(v0);
            let v46: Rc<str> = clock_text_40();
            let v47: Rc<str> = tally_text_42(v41, v42, v43, v44);
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v47, v48)), v45)), v48)), v46));
            println!("{}", v52);
            let v53: bool = v44 == 0i32;
            if v53 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lint_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_run() -> i32 {
    closure0()()
}
