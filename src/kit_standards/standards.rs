#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_NoCommentLines,
    US1_LineWidth(i32),
    US1_LfOnly,
    US1_IntegerOnly(Rc<str>),
    US1_TestTargets,
    US1_HostFree,
    US1_LibReferenced,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoStandards,
    UH0_StandardThen(US1, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_DeclaredRule(US1),
    US2_UnknownRule(Rc<str>),
    US2_NotARule,
}
struct Mut0 { l0: i32 }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
struct Mut2 { l0: Rc<str> }
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn tally_start_4() -> (i32, i32, Rc<str>, i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, v0.clone(), 0i32, 0i32, v0)
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_6(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_7(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn directory_exists_8(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn declared_row_of_11(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("let "); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0.starts_with(&*v3);
        v4
    };
    let v8: bool = if v5 {
        true
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v0.starts_with(&*v6);
        v7
    };
    let v11: bool = if v8 {
        true
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":~ "); } LIT.with(|lit| lit.clone()) };
        let mut v10: bool = v0.starts_with(&*v9);
        v10
    };
    let v12: bool = v11 == false;
    if v12 {
        US2::US2_NotARule
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= NoCommentLines"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v0.contains(&*v14);
        if v15 {
            let v16: US1 = US1::US1_NoCommentLines;
            US2::US2_DeclaredRule(v16.clone())
        } else {
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LfOnly"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v0.contains(&*v18);
            if v19 {
                let v20: US1 = US1::US1_LfOnly;
                US2::US2_DeclaredRule(v20.clone())
            } else {
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= TestTargets"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v0.contains(&*v22);
                if v23 {
                    let v24: US1 = US1::US1_TestTargets;
                    US2::US2_DeclaredRule(v24.clone())
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= HostFree"); } LIT.with(|lit| lit.clone()) };
                    let mut v27: bool = v0.contains(&*v26);
                    if v27 {
                        let v28: US1 = US1::US1_HostFree;
                        US2::US2_DeclaredRule(v28.clone())
                    } else {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LibReferenced"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = v0.contains(&*v30);
                        if v31 {
                            let v32: US1 = US1::US1_LibReferenced;
                            US2::US2_DeclaredRule(v32.clone())
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LineWidth "); } LIT.with(|lit| lit.clone()) };
                            let v35: bool = v0.contains(&*v34);
                            if v35 {
                                let v36: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v34).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                let v37: i32 = (v36).parse::<i32>().unwrap_or(120);
                                let v38: US1 = US1::US1_LineWidth(v37);
                                US2::US2_DeclaredRule(v38.clone())
                            } else {
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= IntegerOnly "); } LIT.with(|lit| lit.clone()) };
                                let v41: bool = v0.contains(&*v40);
                                if v41 {
                                    let v42: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v40).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                    let v43: US1 = US1::US1_IntegerOnly(v42.clone());
                                    US2::US2_DeclaredRule(v43.clone())
                                } else {
                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = "); } LIT.with(|lit| lit.clone()) };
                                    let v46: bool = v0.contains(&*v45);
                                    let v50: bool = if v46 {
                                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") : "); } LIT.with(|lit| lit.clone()) };
                                        let v48: bool = v0.contains(&*v47);
                                        let v49: bool = v48 == false;
                                        v49
                                    } else {
                                        false
                                    };
                                    if v50 {
                                        let v51: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v45).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                        US2::US2_UnknownRule(v51.clone())
                                    } else {
                                        US2::US2_NotARule
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn declared_from_10(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let mut v4: US2 = declared_row_of_11(v3.clone());
            match &v4 {
                US2::US2_DeclaredRule(v5) => {
                    let mut v5: US1 = v5.clone();
                    let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    let v7: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v5.clone(), v1.clone()));
                    (v0, v1) = (v6.clone(), v7.clone());
                    continue;
                }
                _ => {
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    (v0, v1) = (v9.clone(), v1.clone());
                    continue;
                }
            }
        }
    }
}
fn package_standards_9(mut v0: Rc<str>) -> (Rc<UH0>, bool) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = declared_from_10(v3, v4);
    match &*v5 {
        UH0::UH0_NoStandards => {
            let mut v6: US1 = US1::US1_NoCommentLines;
            let v7: US1 = US1::US1_LineWidth(120i32);
            let v8: US1 = US1::US1_LfOnly;
            let v9: US1 = US1::US1_TestTargets;
            let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v9, v10));
            let v12: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v8, v11));
            let v13: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v7, v12));
            let v14: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v6, v13));
            (v14, false)
        }
        _ => {
            (v5, true)
        }
    }
}
fn rule_words_12(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoStandards => {
                return v1.clone();
            }
            UH0::UH0_StandardThen(v2, v3) => {
                let mut v2: US1 = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v1 == "";
                let v40: Rc<str> = if v4 {
                    match &v2 {
                        US1::US1_HostFree => {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        }
                        US1::US1_IntegerOnly(v9) => {
                            let v9: Rc<str> = v9.clone();
                            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only"); } LIT.with(|lit| lit.clone()) };
                            v10.clone()
                        }
                        US1::US1_LfOnly => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lf-only"); } LIT.with(|lit| lit.clone()) };
                            v8.clone()
                        }
                        US1::US1_LibReferenced => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lib-referenced"); } LIT.with(|lit| lit.clone()) };
                            v13.clone()
                        }
                        US1::US1_LineWidth(v6) => {
                            let mut v6: i32 = *v6;
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line-width"); } LIT.with(|lit| lit.clone()) };
                            v7.clone()
                        }
                        US1::US1_NoCommentLines => {
                            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines"); } LIT.with(|lit| lit.clone()) };
                            v5.clone()
                        }
                        US1::US1_TestTargets => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                            v11.clone()
                        }
                    }
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v21));
                    let v38: Rc<str> = match &v2 {
                        US1::US1_HostFree => {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                            v30.clone()
                        }
                        US1::US1_IntegerOnly(v27) => {
                            let mut v27: Rc<str> = v27.clone();
                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only"); } LIT.with(|lit| lit.clone()) };
                            v28.clone()
                        }
                        US1::US1_LfOnly => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lf-only"); } LIT.with(|lit| lit.clone()) };
                            v26.clone()
                        }
                        US1::US1_LibReferenced => {
                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lib-referenced"); } LIT.with(|lit| lit.clone()) };
                            v31.clone()
                        }
                        US1::US1_LineWidth(v24) => {
                            let v24: i32 = *v24;
                            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line-width"); } LIT.with(|lit| lit.clone()) };
                            v25.clone()
                        }
                        US1::US1_NoCommentLines => {
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines"); } LIT.with(|lit| lit.clone()) };
                            v23.clone()
                        }
                        US1::US1_TestTargets => {
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                            v29.clone()
                        }
                    };
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v38));
                    v39.clone()
                };
                (v0, v1) = (v3.clone(), v40.clone());
                continue;
            }
        }
    }
}
fn method15(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method16(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure1()
}
fn method18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut1>>) -> bool {
    let v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let v8: bool = &*v7 < &*v1;
        let v11: i32 = if v8 {
            -1i32
        } else {
            let v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn sorted_names_14(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: bool = directory_exists_8(v0.clone());
    if v1 {
        let v3: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v5: Vec<std::string::String> = v3.unwrap();
        let mut v6: Vec<std::string::String> = method15(v5.clone());
        let v7: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v6));
        let v8: i32 = (v7.clone().borrow().len() as i32);
        let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v8 as usize]));
        let mut v10: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
        while method16(v8, v10.clone()) {
            let v12: i32 = v10.borrow().l0;
            let v13: std::string::String = v7.clone().borrow()[v12 as usize].clone();
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            v9.clone().borrow_mut()[v12 as usize] = v15.clone();
            let v16: i32 = v12.wrapping_add(1i32);
            v10.borrow_mut().l0 = v16;
            ()
        };
        let v17: Rc<RefCell<Vec<Rc<str>>>> = v9 ;
        let mut v18: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method17();
        let v19: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v17.borrow().clone()));
        let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v17.borrow().iter().map(|x| v18(x.clone())).collect::<Vec<_>>()));
        let v21: i32 = (v17.clone().borrow().len() as i32);
        let mut v22: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 1i32 }));
        while method16(v21, v22.clone()) {
            let v24: i32 = v22.borrow().l0;
            let v25: Rc<str> = v19.clone().borrow()[v24 as usize].clone();
            let v26: Rc<str> = v20.clone().borrow()[v24 as usize].clone();
            let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v24 }));
            while method18(v20.clone(), v26.clone(), v27.clone()) {
                let v29: i32 = v27.borrow().l0;
                let v30: i32 = v29.wrapping_sub(1i32);
                let v31: Rc<str> = v19.clone().borrow()[v30 as usize].clone();
                v19.clone().borrow_mut()[v29 as usize] = v31.clone();
                let v32: Rc<str> = v20.clone().borrow()[v30 as usize].clone();
                v20.clone().borrow_mut()[v29 as usize] = v32.clone();
                v27.borrow_mut().l0 = v30;
                ()
            };
            let v33: i32 = v27.borrow().l0;
            v19.clone().borrow_mut()[v33 as usize] = v25.clone();
            let v34: i32 = v27.borrow().l0;
            v20.clone().borrow_mut()[v34 as usize] = v26.clone();
            let v35: i32 = v24.wrapping_add(1i32);
            v22.borrow_mut().l0 = v35;
            ()
        };
        let v36: Rc<RefCell<Vec<Rc<str>>>> = v19 ;
        v36.clone()
    } else {
        let v37: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        v37.clone()
    }
}
fn file_exists_19(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v1.ends_with(&*v2);
        let mut v6: bool = if v3 {
            true
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spir"); } LIT.with(|lit| lit.clone()) };
            let mut v5: bool = v1.ends_with(&*v4);
            v5
        };
        let v8: bool = if v6 {
            let v7: bool = &*v1 != "standards.spi";
            v7
        } else {
            false
        };
        if v8 {
            let v9: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
            file_exists_19(v9.clone())
        } else {
            false
        }
    })
}
fn names_where_20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<dyn Fn(Rc<str>) -> bool>, mut v3: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v4: bool = v1 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: bool = v2(v5.clone());
            let v9: Rc<UH1> = if v7 {
                Rc::new(UH1::UH1_Cons(v5.clone(), v3.clone()))
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v6, v2.clone(), v9.clone());
            continue;
        }
    }
}
fn folder_named_c3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = &*v1 == "target";
        let mut v4: bool = if v2 {
            true
        } else {
            let v3: bool = &*v1 == "node_modules";
            v3
        };
        let v7: bool = if v4 {
            true
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v1.starts_with(&*v5);
            v6
        };
        let v8: bool = v7 == false;
        if v8 {
            let v9: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
            directory_exists_8(v9.clone())
        } else {
            false
        }
    })
}
fn method23(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let mut v4: Rc<UH1> = v4.clone();
            let mut v5: Rc<UH1> = method23(v0.clone(), v4, v2.clone());
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v6)), v3));
            Rc::new(UH1::UH1_Cons(v8, v5))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn package_entry_24(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default().as_str());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --package ..\n"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.starts_with(&*v2);
    if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --package ..\r\n"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v1.starts_with(&*v4);
        v5
    }
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v3: bool = v1.ends_with(&*v2);
        let mut v6: bool = if v3 {
            true
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spir"); } LIT.with(|lit| lit.clone()) };
            let mut v5: bool = v1.ends_with(&*v4);
            v5
        };
        let v8: bool = if v6 {
            let v7: bool = &*v1 != "standards.spi";
            v7
        } else {
            false
        };
        if v8 {
            let v9: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
            package_entry_24(v9.clone())
        } else {
            false
        }
    })
}
fn entries_in_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
    let v3: Rc<RefCell<Vec<Rc<str>>>> = sorted_names_14(v2.clone());
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = closure4(v2);
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = names_where_20(v3, v5, v6, v7);
    let v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method23(v1, v8, v9)
}
fn method25(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = method25(v3, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: Rc<UH1> = entries_in_22(v0.clone(), v3.clone());
                let mut v6: Rc<UH1> = method25(v2.clone(), v5.clone());
                (v0, v1, v2) = (v0.clone(), v4.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method26(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn spiral_files_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = sorted_names_14(v0.clone());
    let mut v2: i32 = (v1.clone().borrow().len() as i32);
    let v3: i32 = v2.wrapping_sub(1i32);
    let mut v4: Rc<dyn Fn(Rc<str>) -> bool> = closure2(v0.clone());
    let mut v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH1> = names_where_20(v1.clone(), v3, v4, v5);
    let v7: Rc<dyn Fn(Rc<str>) -> bool> = folder_named_c3(v0.clone());
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH1> = names_where_20(v1, v3, v7, v8);
    let mut v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH1> = method21(v0, v9, v10);
    let v12: Rc<UH1> = method25(v6, v11);
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v14: i32 = 0i32;
    let v15: i32 = method26(v13.clone(), v12, v14);
    let v16: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
    let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v16).as_ref().clone()));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from((*v17.borrow()).join(&*v18));
    v19
}
fn method29(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure5() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_stem_28(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.rfind(&*v1);
    let v3: Option<usize> = method29(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure5();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        v0.clone()
    } else {
        let v9: i32 = v7.wrapping_add(1i32);
        let mut v10: i32 = (v0.clone().len() as i32);
        let v11: i32 = v10.wrapping_sub(v7);
        let v12: i32 = v11.wrapping_sub(1i32);
        let v13: usize = ((v9) as usize);
        let v14: i32 = v9.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        v16.clone()
    }
}
fn files_checked_27(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> (i32, i32, Rc<str>, i32, i32, Rc<str>) {
    let v9: bool = &*v2 == "";
    let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v9 {
        (v3, v4, v5.clone(), v6, v7, v8.clone())
    } else {
        let mut v10: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let v11: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v10).to_string_lossy().replace(char::from(92u8), "/").as_str());
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards.spi"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v12).to_string_lossy().replace(char::from(92u8), "/").as_str());
        let v14: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
        let v15: Rc<str> = file_stem_28(v10.clone());
        let v16: Rc<str> = std::rc::Rc::<str>::from((v15).rsplit_once(char::from(46u8)).map(|(stem, _)| stem).unwrap_or(&*v15));
        let v17: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
        let mut v18: Rc<str> = kit_standards_scan::kit_standards_file_findings(&*v14, &*v11, &*v16, &*v17);
        let v19: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v20: i32 = v4.wrapping_add(1i32);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v18));
        let mut v22: i32 = (v18).lines().count() as i32;
        let v23: i32 = v6.wrapping_add(v22);
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (i32, i32, Rc<str>, i32, i32, Rc<str>) = files_checked_27(v0.clone(), v1.clone(), v19.clone(), v3, v20, v21.clone(), v23, v7, v8.clone());
        (v24, v25, v26.clone(), v27, v28, v29.clone())
    };
    (v30, v31, v32.clone(), v33, v34, v35.clone())
}
fn method32(mut v0: Rc<RefCell<Mut2>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_31(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn unknown_rules_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = &*v0 == "";
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v7: US2 = declared_row_of_11(v6.clone());
            match &v7 {
                US2::US2_UnknownRule(v8) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: i32 = v2.wrapping_add(1i32);
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                    let v13: Rc<str> = format_real_31(v2);
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unknown-rule  "); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v8));
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a rule this kit knows, so it never runs"); } LIT.with(|lit| lit.clone()) };
                    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v14, Rc::<str>::from(format!("{}{}", v18, v19))));
                    (v0, v1, v2, v3) = (v5.clone(), v1.clone(), v9, v21.clone());
                    continue;
                }
                _ => {
                    let v23: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v5.clone(), v1.clone(), v23, v3.clone());
                    continue;
                }
            }
        }
    }
}
fn package_findings_33(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoStandards => {
                return v2.clone();
            }
            UH0::UH0_StandardThen(v3, v4) => {
                let v3: US1 = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                match &v3 {
                    US1::US1_LibReferenced => {
                        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v5).to_string_lossy().replace(char::from(92u8), "/").as_str());
                        let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v6).unwrap_or_default().as_str());
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-"); } LIT.with(|lit| lit.clone()) };
                        let v9: bool = v7.contains(&*v8);
                        let v13: Rc<str> = if v9 {
                            v2.clone()
                        } else {
                            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":1  lib-referenced  package.spiproj does not list spiral-; every package reuses lib/spiral\n"); } LIT.with(|lit| lit.clone()) };
                            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                            v12.clone()
                        };
                        (v0, v1, v2) = (v4.clone(), v1.clone(), v13.clone());
                        continue;
                    }
                    _ => {
                        (v0, v1, v2) = (v4.clone(), v1.clone(), v2.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn packages_checked_5(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: i32, mut v7: Rc<str>) -> (i32, i32, Rc<str>, i32, i32, Rc<str>) {
    let v8: bool = v1 <= v0;
    let (mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v8 {
        (v2, v3, v4.clone(), v5, v6, v7.clone())
    } else {
        let v9: Rc<str> = argument_6(v0);
        let mut v10: bool = &*v9 == "--json";
        let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v10 {
            let v11: i32 = v0.wrapping_add(1i32);
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v11, v1, v2, v3, v4.clone(), v5, v6, v7.clone());
            (v12, v13, v14, v15, v16, v17)
        } else {
            let mut v18: bool = directory_exists_8(v9.clone());
            let v19: bool = v18 == false;
            let (mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v19 {
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v9));
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  package  not a directory\n"); } LIT.with(|lit| lit.clone()) };
                let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let v23: i32 = v5.wrapping_add(1i32);
                let v24: i32 = v0.wrapping_add(1i32);
                let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v24, v1, v2, v3, v22, v23, v6, v7);
                (v25, v26, v27, v28, v29, v30)
            } else {
                let (mut v31, mut v32): (Rc<UH0>, bool) = package_standards_9(v9.clone());
                let v33: i32 = v2.wrapping_add(1i32);
                let v34: i32 = if v32 {
                    1i32
                } else {
                    0i32
                };
                let v35: i32 = v6.wrapping_add(v34);
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = rule_words_12(v31.clone(), v36.clone());
                let v38: Rc<str> = spiral_files_13(v9.clone());
                let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, Rc<str>, i32, i32, Rc<str>) = files_checked_27(v31.clone(), v9.clone(), v38, v33, v3, v4, v5, v35, v37);
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards.spi"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v9).join(&*v45).to_string_lossy().replace(char::from(92u8), "/").as_str());
                let v47: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v46).unwrap_or_default().as_str());
                let v48: i32 = 1i32;
                let v49: Rc<str> = unknown_rules_30(v47, v46, v48, v36);
                let v50: Rc<str> = package_findings_33(v31, v9, v49);
                let v51: i32 = v0.wrapping_add(1i32);
                let v52: bool = &*v50 == "";
                let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v52 {
                    (v39, v40, v41.clone(), v42, v43, v44.clone())
                } else {
                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v50));
                    let v54: i32 = (v50).lines().count() as i32;
                    let v55: i32 = v42.wrapping_add(v54);
                    (v39, v40, v53, v55, v43, v44)
                };
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v51, v1, v56, v57, v58, v59, v60, v61);
                (v62, v63, v64, v65, v66, v67)
            };
            (v68, v69, v70, v71, v72, v73)
        };
        (v74, v75, v76, v77, v78, v79)
    };
    (v80, v81, v82, v83, v84, v85)
}
fn rules_run_1() -> i32 {
    let mut v0: i32 = argument_count_2();
    let v1: bool = v0 < 3i32;
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards <package-dir>... [--json]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v2);
        2i32
    } else {
        let v3: i32 = 2i32;
        let mut v4: i32 = argument_count_2();
        let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (i32, i32, Rc<str>, i32, i32, Rc<str>) = tally_start_4();
        let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v3, v4, v5, v6, v7.clone(), v8, v9, v10.clone());
        let v17: Rc<str> = format_real_31(v11);
        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages="); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" declared="); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let mut v22: Rc<str> = format_real_31(v15);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let v24: Rc<str> = format_real_31(v12);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        let v29: Rc<str> = format_real_31(v14);
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v23, Rc::<str>::from(format!("{}{}", v28, v29))));
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let v33: bool = std::env::args().skip(2i32 as usize).any(|argument| argument == &*v32);
        if v33 {
            let v34: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let escape = |value: &str| value.replace(b, &format!("{b}{b}")).replace(q, &format!("{b}{q}")); let items = (v13).lines().filter_map(|line| { let mut parts = line.splitn(3, "  "); let place = parts.next()?; let rule = parts.next()?; let detail = parts.next().unwrap_or(""); let (path, number) = place.rsplit_once(char::from(58u8))?; Some(format!("{{{q}path{q}:{q}{}{q},{q}line{q}:{},{q}rule{q}:{q}{}{q},{q}detail{q}:{q}{}{q}}}", escape(path), number, rule, escape(detail))) }).collect::<Vec<_>>(); items.join(",") }.as_str());
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"findings\":["); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"count\":"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let v39: Rc<str> = format_real_31(v14);
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
            println!("{}", v42);
            ()
        } else {
            let v43: bool = &*v13 != "";
            if v43 {
                let v44: Rc<str> = std::rc::Rc::<str>::from((v13).trim_end());
                println!("{}", v44);
                ()
            };
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards ok "); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v31));
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rules="); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v47)), v16));
            println!("{}", v49);
            ()
        };
        let v50: bool = v14 == 0i32;
        if v50 {
            0i32
        } else {
            1i32
        }
    }
}
fn census_requested_34() -> bool {
    let mut v0: bool = std::env::args().nth(2).is_some_and(|flag| matches!(flag.as_str(), "--census" | "--baseline" | "--baseline-merge" | "--ratchet"));
    v0
}
fn standards_run_0() -> i32 {
    let mut v0: bool = census_requested_34();
    if v0 {
        let v1: i32 = kit_style_census::kit_census_run();
        v1
    } else {
        rules_run_1()
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        standards_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_standards_run() -> i32 {
    closure0()()
}
