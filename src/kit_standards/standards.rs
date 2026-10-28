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
#[derive(Clone)]
enum US3 {
    US3_BlankLine,
    US3_CodeLine,
    US3_DirectiveLine,
    US3_TodoLine,
    US3_CommentLine,
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
    (0i32, 0i32, v0.clone(), 0i32, 0i32, v0.clone())
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
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
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
    let v5: Rc<UH0> = declared_from_10(v3.clone(), v4.clone());
    match &*v5 {
        UH0::UH0_NoStandards => {
            let mut v6: US1 = US1::US1_NoCommentLines;
            let v7: US1 = US1::US1_LineWidth(120i32);
            let v8: US1 = US1::US1_LfOnly;
            let v9: US1 = US1::US1_TestTargets;
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v9.clone(), v10.clone()));
            let v12: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v8.clone(), v11.clone()));
            let v13: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v7.clone(), v12.clone()));
            let v14: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v6.clone(), v13.clone()));
            (v14.clone(), false)
        }
        _ => {
            (v5.clone(), true)
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
fn file_scan_start_14() -> (Rc<str>, i32, bool, Rc<str>, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0i32, false, v0.clone(), false)
}
fn outside_quotes_19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: i32 = v1.wrapping_add(2i32);
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            (v0, v1, v2) = (v0.clone(), v5, v9.clone());
            continue;
        }
    }
}
fn unquoted_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    outside_quotes_19(v2.clone(), v3, v4.clone())
}
fn method21(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn line_findings_17(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US3, mut v6: Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool) -> Rc<str> {
    match &v0 {
        US1::US1_HostFree => {
            match &v5 {
                US3::US3_CodeLine => {
                    let v115: Rc<str> = unquoted_18(v4.clone());
                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                    let v117: bool = v115.contains(&*v116);
                    if v117 {
                        let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a host macro in a host-free package; reach for lib/spiral (sm', am', listm', file_system) or the OS boundary package"); } LIT.with(|lit| lit.clone()) };
                        let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v119));
                        let v121: Rc<str> = format_real_20(v3);
                        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v121));
                        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v124: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v123));
                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v125, v123)), v118));
                        let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v124, Rc::<str>::from(format!("{}{}", v127, v128))));
                        v130.clone()
                    } else {
                        let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v131.clone()
                    }
                }
                _ => {
                    let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v133.clone()
                }
            }
        }
        US1::US1_IntegerOnly(v67) => {
            let v67: Rc<str> = v67.clone();
            match &v5 {
                US3::US3_CodeLine => {
                    let v68: bool = &*v67 == &*v2;
                    let v70: bool = if v68 {
                        let v69: bool = { let line = &*v4; line.contains("f64") || line.contains("f32") };
                        v69
                    } else {
                        false
                    };
                    if v70 {
                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("floating point in an integer-only module"); } LIT.with(|lit| lit.clone()) };
                        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v72));
                        let v74: Rc<str> = format_real_20(v3);
                        let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                        let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only"); } LIT.with(|lit| lit.clone()) };
                        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v78, v76)), v71));
                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v77, Rc::<str>::from(format!("{}{}", v80, v81))));
                        v83.clone()
                    } else {
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v84.clone()
                    }
                }
                _ => {
                    let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v86.clone()
                }
            }
        }
        US1::US1_LfOnly => {
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v51: bool = v4.ends_with(&*v50);
            if v51 {
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CRLF line ending"); } LIT.with(|lit| lit.clone()) };
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v53));
                let v55: Rc<str> = format_real_20(v3);
                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lf-only"); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v59, v57)), v52));
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v58, Rc::<str>::from(format!("{}{}", v61, v62))));
                v64.clone()
            } else {
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v65.clone()
            }
        }
        US1::US1_LibReferenced => {
            let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v135.clone()
        }
        US1::US1_LineWidth(v28) => {
            let v28: i32 = *v28;
            let v29: i32 = (v4).trim_end_matches(char::from(13u8)).chars().count() as i32;
            let v30: bool = v28 < v29;
            if v30 {
                let v31: Rc<str> = format_real_20(v29);
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns, limit "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let v34: Rc<str> = format_real_20(v28);
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v36));
                let v38: Rc<str> = format_real_20(v3);
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line-width"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v42, v40)), v35));
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v41, Rc::<str>::from(format!("{}{}", v44, v45))));
                v47.clone()
            } else {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v48.clone()
            }
        }
        US1::US1_NoCommentLines => {
            match &v5 {
                US3::US3_CommentLine => {
                    if v8 {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v11.clone()
                    } else {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a comment line; encode it in names or drop it"); } LIT.with(|lit| lit.clone()) };
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v13));
                        let v15: Rc<str> = format_real_20(v3);
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines"); } LIT.with(|lit| lit.clone()) };
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v17)), v12));
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v18, Rc::<str>::from(format!("{}{}", v21, v22))));
                        v24.clone()
                    }
                }
                _ => {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v26.clone()
                }
            }
        }
        US1::US1_TestTargets => {
            let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
            let v89: bool = v4.starts_with(&*v88);
            let v93: bool = if v89 {
                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v91: bool = v4.contains(&*v90);
                let v92: bool = v91 == false;
                v92
            } else {
                false
            };
            let v97: bool = if v93 {
                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                let v95: bool = v9.starts_with(&*v94);
                let v96: bool = v95 == false;
                v96
            } else {
                false
            };
            let v99: bool = if v97 {
                let v98: bool = v10 == false;
                v98
            } else {
                false
            };
            if v99 {
                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test block without a ///> target line above it or in the file header"); } LIT.with(|lit| lit.clone()) };
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v101));
                let v103: Rc<str> = format_real_20(v3);
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                let v109: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v107, v105)), v100));
                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v106, Rc::<str>::from(format!("{}{}", v109, v110))));
                v112.clone()
            } else {
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v113.clone()
            }
        }
    }
}
fn rules_applied_16(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US3, mut v6: Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoStandards => {
                return v11.clone();
            }
            UH0::UH0_StandardThen(v12, v13) => {
                let v12: US1 = v12.clone();
                let v13: Rc<UH0> = v13.clone();
                let v14: Rc<str> = line_findings_17(v12.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10);
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v14));
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v13.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10, v15.clone());
                continue;
            }
        }
    }
}
fn lines_scanned_15(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: bool, mut v8: Rc<str>, mut v9: bool) -> (Rc<str>, i32, bool, Rc<str>, bool) {
    let v10: bool = &*v3 == "";
    let (mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, i32, bool, Rc<str>, bool) = if v10 {
        (v5.clone(), v6, v7, v8.clone(), v9)
    } else {
        let v11: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
        let v12: Rc<str> = std::rc::Rc::<str>::from((v11).trim_start());
        let v13: bool = &*v12 == "";
        let v15: bool = if v13 {
            true
        } else {
            let v14: bool = &*v12 == "\r";
            v14
        };
        let v32: US3 = if v15 {
            US3::US3_BlankLine
        } else {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
            let v18: bool = v12.starts_with(&*v17);
            if v18 {
                US3::US3_DirectiveLine
            } else {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v12.starts_with(&*v20);
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                    let v25: bool = v12.contains(&*v24);
                    if v25 {
                        US3::US3_TodoLine
                    } else {
                        US3::US3_CommentLine
                    }
                } else {
                    US3::US3_CodeLine
                }
            }
        };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = rules_applied_16(v0.clone(), v1.clone(), v2.clone(), v4, v11.clone(), v32.clone(), v5.clone(), v6, v7, v8.clone(), v9, v33.clone());
        let v36: bool = match &v32 {
            US3::US3_CommentLine => {
                v7
            }
            US3::US3_TodoLine => {
                true
            }
            _ => {
                false
            }
        };
        let v38: Rc<str> = match &v32 {
            US3::US3_BlankLine => {
                v8.clone()
            }
            _ => {
                let v37: Rc<str> = std::rc::Rc::<str>::from((v11).trim_start());
                v37.clone()
            }
        };
        let v47: bool = if v9 {
            true
        } else {
            let v39: bool = v4 == 2i32;
            let v42: bool = if v39 {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v41: bool = v8.starts_with(&*v40);
                v41
            } else {
                false
            };
            if v42 {
                let v43: Rc<str> = std::rc::Rc::<str>::from((v11).trim_start());
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                let v45: bool = v43.starts_with(&*v44);
                v45
            } else {
                false
            }
        };
        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v34));
        let v49: i32 = (v34).lines().count() as i32;
        let v50: i32 = v6.wrapping_add(v49);
        let v51: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v52: i32 = v4.wrapping_add(1i32);
        let (mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, i32, bool, Rc<str>, bool) = lines_scanned_15(v0.clone(), v1.clone(), v2.clone(), v51.clone(), v52, v48.clone(), v50, v36, v38.clone(), v47);
        (v53.clone(), v54, v55, v56.clone(), v57)
    };
    (v58.clone(), v59, v60, v61.clone(), v62)
}
fn files_checked_13(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> (i32, i32, Rc<str>, i32, i32, Rc<str>) {
    let v9: bool = &*v2 == "";
    let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (i32, i32, Rc<str>, i32, i32, Rc<str>) = if v9 {
        (v3, v4, v5.clone(), v6, v7, v8.clone())
    } else {
        let v10: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let v11: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v1).join(&*v10).to_string_lossy().replace(char::from(92u8), "/").as_str());
        let v12: Rc<str> = std::rc::Rc::<str>::from((v10).rsplit_once(char::from(46u8)).map(|(stem, _)| stem).unwrap_or(&*v10));
        let v13: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
        let v14: i32 = 1i32;
        let (mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, i32, bool, Rc<str>, bool) = file_scan_start_14();
        let (mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, i32, bool, Rc<str>, bool) = lines_scanned_15(v0.clone(), v11.clone(), v12.clone(), v13.clone(), v14, v15.clone(), v16, v17, v18.clone(), v19);
        let v25: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v26: i32 = v4.wrapping_add(1i32);
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v20));
        let v28: i32 = v6.wrapping_add(v21);
        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (i32, i32, Rc<str>, i32, i32, Rc<str>) = files_checked_13(v0.clone(), v1.clone(), v25.clone(), v3, v26, v27.clone(), v28, v7, v8.clone());
        (v29, v30, v31.clone(), v32, v33, v34.clone())
    };
    (v35, v36, v37.clone(), v38, v39, v40.clone())
}
fn unknown_rules_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
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
                    let v13: Rc<str> = format_real_20(v2);
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
fn package_findings_23(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
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
                let v49: Rc<str> = unknown_rules_22(v47.clone(), v46.clone(), v48, v36.clone());
                let v50: Rc<str> = package_findings_23(v31.clone(), v9.clone(), v49.clone());
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
        let v17: Rc<str> = format_real_20(v11);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages="); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" declared="); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
        let v22: Rc<str> = format_real_20(v15);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        let v24: Rc<str> = format_real_20(v12);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        let v29: Rc<str> = format_real_20(v14);
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v23, Rc::<str>::from(format!("{}{}", v28, v29))));
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let v33: bool = std::env::args().skip(2i32 as usize).any(|argument| argument == &*v32);
        if v33 {
            let v34: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let escape = |value: &str| value.replace(b, &format!("{b}{b}")).replace(q, &format!("{b}{q}")); let items = (v13).lines().filter_map(|line| { let mut parts = line.splitn(3, "  "); let place = parts.next()?; let rule = parts.next()?; let detail = parts.next().unwrap_or(""); let (path, number) = place.rsplit_once(char::from(58u8))?; Some(format!("{{{q}path{q}:{q}{}{q},{q}line{q}:{},{q}rule{q}:{q}{}{q},{q}detail{q}:{q}{}{q}}}", escape(path), number, rule, escape(detail))) }).collect::<Vec<_>>(); items.join(",") }.as_str());
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"findings\":["); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"count\":"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let v39: Rc<str> = format_real_20(v14);
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
fn census_requested_24() -> bool {
    let mut v0: bool = std::env::args().nth(2).is_some_and(|flag| matches!(flag.as_str(), "--census" | "--baseline" | "--baseline-merge" | "--ratchet"));
    v0
}
fn standards_run_0() -> i32 {
    let mut v0: bool = census_requested_24();
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
