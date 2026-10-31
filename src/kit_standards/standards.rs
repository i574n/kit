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
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
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
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_6(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_3();
        let v4: bool = v1 < v3;
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
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn directory_exists_8(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn declared_row_of_11(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("let "); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
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
        let v10: bool = v0.starts_with(&*v9);
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
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LfOnly"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v0.contains(&*v18);
            if v19 {
                let v20: US1 = US1::US1_LfOnly;
                US2::US2_DeclaredRule(v20.clone())
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= TestTargets"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v0.contains(&*v22);
                if v23 {
                    let v24: US1 = US1::US1_TestTargets;
                    US2::US2_DeclaredRule(v24.clone())
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= HostFree"); } LIT.with(|lit| lit.clone()) };
                    let v27: bool = v0.contains(&*v26);
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
        let v2: bool = &*v0 == "";
        if v2 {
            return v1.clone();
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v4: US2 = declared_row_of_11(v3.clone());
            match &v4 {
                US2::US2_DeclaredRule(v5) => {
                    let v5: US1 = v5.clone();
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
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().replace(char::from(92u8), "/").as_str());
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = declared_from_10(v3, v4);
    match &*v5 {
        UH0::UH0_NoStandards => {
            let mut v6: US1 = US1::US1_NoCommentLines;
            let v7: US1 = US1::US1_LineWidth(120i32);
            let v8: US1 = US1::US1_LfOnly;
            let v9: US1 = US1::US1_TestTargets;
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
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
                let v2: US1 = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v1 == "";
                let v40: Rc<str> = if v4 {
                    match &v2 {
                        US1::US1_HostFree => {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                            v12.clone()
                        }
                        US1::US1_IntegerOnly(v9) => {
                            let v9: Rc<str> = v9.clone();
                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only"); } LIT.with(|lit| lit.clone()) };
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
                            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines"); } LIT.with(|lit| lit.clone()) };
                            v5.clone()
                        }
                        US1::US1_TestTargets => {
                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                            v11.clone()
                        }
                    }
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v21));
                    let v38: Rc<str> = match &v2 {
                        US1::US1_HostFree => {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                            v30.clone()
                        }
                        US1::US1_IntegerOnly(v27) => {
                            let v27: Rc<str> = v27.clone();
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
fn files_checked_13(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> (i32, i32, Rc<str>, i32, i32, Rc<str>) {
    let v9: bool = &*v2 == "";
    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v9 {
        (v3, v4, v5.clone(), v6, v7, v8.clone())
    } else {
        let v10: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let v11: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v10).to_string_lossy().replace(char::from(92u8), "/").as_str());
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards.spi"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v12).to_string_lossy().replace(char::from(92u8), "/").as_str());
        let v14: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
        let v15: Rc<str> = std::rc::Rc::<str>::from((v10).rsplit_once(char::from(46u8)).map(|(stem, _)| stem).unwrap_or(&*v10));
        let v16: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
        let v17: Rc<str> = kit_standards_scan::kit_standards_file_findings(&*v14, &*v11, &*v15, &*v16);
        let v18: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v19: i32 = v4.wrapping_add(1i32);
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v17));
        let v21: i32 = (v17).lines().count() as i32;
        let v22: i32 = v6.wrapping_add(v21);
        let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, Rc<str>, i32, i32, Rc<str>) = files_checked_13(v0.clone(), v1.clone(), v18.clone(), v3, v19, v20.clone(), v22, v7, v8.clone());
        (v23, v24, v25.clone(), v26, v27, v28.clone())
    };
    (v29, v30, v31.clone(), v32, v33, v34.clone())
}
fn method16(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_15(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method16(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn unknown_rules_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: bool = &*v0 == "";
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
            let mut v6: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v7: US2 = declared_row_of_11(v6.clone());
            match &v7 {
                US2::US2_UnknownRule(v8) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: i32 = v2.wrapping_add(1i32);
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                    let v13: Rc<str> = format_real_15(v2);
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unknown-rule  "); } LIT.with(|lit| lit.clone()) };
                    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v8));
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a rule this kit knows, so it never runs"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
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
fn package_findings_17(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoStandards => {
                return v2.clone();
            }
            UH0::UH0_StandardThen(v3, v4) => {
                let v3: US1 = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                match &v3 {
                    US1::US1_LibReferenced => {
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
                        let mut v6: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v5).to_string_lossy().replace(char::from(92u8), "/").as_str());
                        let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v6).unwrap_or_default().as_str());
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-"); } LIT.with(|lit| lit.clone()) };
                        let v9: bool = v7.contains(&*v8);
                        let v13: Rc<str> = if v9 {
                            v2.clone()
                        } else {
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
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
        let v10: bool = &*v9 == "--json";
        let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v10 {
            let v11: i32 = v0.wrapping_add(1i32);
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v11, v1, v2, v3, v4.clone(), v5, v6, v7.clone());
            (v12, v13, v14.clone(), v15, v16, v17.clone())
        } else {
            let v18: bool = directory_exists_8(v9.clone());
            let v19: bool = v18 == false;
            let (mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v19 {
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v9));
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":0  package  not a directory\n"); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let v23: i32 = v5.wrapping_add(1i32);
                let v24: i32 = v0.wrapping_add(1i32);
                let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v24, v1, v2, v3, v22.clone(), v23, v6, v7.clone());
                (v25, v26, v27.clone(), v28, v29, v30.clone())
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
                let v38: Rc<str> = std::rc::Rc::<str>::from({ let mut names = std::fs::read_dir(&*v9).into_iter().flatten().flatten().filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| (name.ends_with(".spi") || name.ends_with(".spir")) && name != "standards.spi").collect::<Vec<_>>(); names.sort(); names.join("
") }.as_str());
                let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (i32, i32, Rc<str>, i32, i32, Rc<str>) = files_checked_13(v31.clone(), v9.clone(), v38.clone(), v33, v3, v4.clone(), v5, v35, v37.clone());
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards.spi"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v9).join(&*v45).to_string_lossy().replace(char::from(92u8), "/").as_str());
                let v47: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v46).unwrap_or_default().as_str());
                let v48: i32 = 1i32;
                let v49: Rc<str> = unknown_rules_14(v47.clone(), v46.clone(), v48, v36.clone());
                let v50: Rc<str> = package_findings_17(v31.clone(), v9.clone(), v49.clone());
                let v51: i32 = v0.wrapping_add(1i32);
                let v52: bool = &*v50 == "";
                let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v52 {
                    (v39, v40, v41.clone(), v42, v43, v44.clone())
                } else {
                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v50));
                    let v54: i32 = (v50).lines().count() as i32;
                    let v55: i32 = v42.wrapping_add(v54);
                    (v39, v40, v53.clone(), v55, v43, v44.clone())
                };
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v51, v1, v56, v57, v58.clone(), v59, v60, v61.clone());
                (v62, v63, v64.clone(), v65, v66, v67.clone())
            };
            (v68, v69, v70.clone(), v71, v72, v73.clone())
        };
        (v74, v75, v76.clone(), v77, v78, v79.clone())
    };
    (v80, v81, v82.clone(), v83, v84, v85.clone())
}
fn rules_run_1() -> i32 {
    let mut v0: i32 = argument_count_2();
    let v1: bool = v0 < 3i32;
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards <package-dir>... [--json]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v2);
        2i32
    } else {
        let v3: i32 = 2i32;
        let v4: i32 = argument_count_2();
        let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (i32, i32, Rc<str>, i32, i32, Rc<str>) = tally_start_4();
        let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (i32, i32, Rc<str>, i32, i32, Rc<str>) = packages_checked_5(v3, v4, v5, v6, v7.clone(), v8, v9, v10.clone());
        let v17: Rc<str> = format_real_15(v11);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages="); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" declared="); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let v22: Rc<str> = format_real_15(v15);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let v24: Rc<str> = format_real_15(v12);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        let v29: Rc<str> = format_real_15(v14);
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v23, Rc::<str>::from(format!("{}{}", v28, v29))));
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let v33: bool = std::env::args().skip(2i32 as usize).any(|argument| argument == &*v32);
        if v33 {
            let v34: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let escape = |value: &str| value.replace(b, &format!("{b}{b}")).replace(q, &format!("{b}{q}")); let items = (v13).lines().filter_map(|line| { let mut parts = line.splitn(3, "  "); let place = parts.next()?; let rule = parts.next()?; let detail = parts.next().unwrap_or(""); let (path, number) = place.rsplit_once(char::from(58u8))?; Some(format!("{{{q}path{q}:{q}{}{q},{q}line{q}:{},{q}rule{q}:{q}{}{q},{q}detail{q}:{q}{}{q}}}", escape(path), number, rule, escape(detail))) }).collect::<Vec<_>>(); items.join(",") }.as_str());
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"findings\":["); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"count\":"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let v39: Rc<str> = format_real_15(v14);
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
fn census_requested_18() -> bool {
    let mut v0: bool = std::env::args().nth(2).is_some_and(|flag| matches!(flag.as_str(), "--census" | "--baseline" | "--baseline-merge" | "--ratchet"));
    v0
}
fn standards_run_0() -> i32 {
    let mut v0: bool = census_requested_18();
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
