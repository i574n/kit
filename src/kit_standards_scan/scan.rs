#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_NoCommentLines,
    US0_LineWidth(i32),
    US0_LfOnly,
    US0_IntegerOnly(Rc<str>),
    US0_TestTargets,
    US0_HostFree,
    US0_LibReferenced,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoStandards,
    UH0_StandardThen(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_DeclaredRule(US0),
    US1_UnknownRule(Rc<str>),
    US1_NotARule,
}
#[derive(Clone)]
enum US2 {
    US2_BlankLine,
    US2_CodeLine,
    US2_DirectiveLine,
    US2_TodoLine,
    US2_CommentLine,
}
pub struct Mut0 { pub l0: Rc<str> }
fn declared_row_of_3(mut v0: Rc<str>) -> US1 {
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
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= "); } LIT.with(|lit| lit.clone()) };
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
        US1::US1_NotARule
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= NoCommentLines"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v0.contains(&*v14);
        if v15 {
            let v16: US0 = US0::US0_NoCommentLines;
            US1::US1_DeclaredRule(v16)
        } else {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LfOnly"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v0.contains(&*v18);
            if v19 {
                let v20: US0 = US0::US0_LfOnly;
                US1::US1_DeclaredRule(v20)
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= TestTargets"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v0.contains(&*v22);
                if v23 {
                    let v24: US0 = US0::US0_TestTargets;
                    US1::US1_DeclaredRule(v24)
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= HostFree"); } LIT.with(|lit| lit.clone()) };
                    let v27: bool = v0.contains(&*v26);
                    if v27 {
                        let v28: US0 = US0::US0_HostFree;
                        US1::US1_DeclaredRule(v28)
                    } else {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LibReferenced"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = v0.contains(&*v30);
                        if v31 {
                            let v32: US0 = US0::US0_LibReferenced;
                            US1::US1_DeclaredRule(v32)
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LineWidth "); } LIT.with(|lit| lit.clone()) };
                            let v35: bool = v0.contains(&*v34);
                            if v35 {
                                let v36: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v34).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                let v37: i32 = (v36).parse::<i32>().unwrap_or(120);
                                let v38: US0 = US0::US0_LineWidth(v37);
                                US1::US1_DeclaredRule(v38)
                            } else {
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= IntegerOnly "); } LIT.with(|lit| lit.clone()) };
                                let v41: bool = v0.contains(&*v40);
                                if v41 {
                                    let v42: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v40).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                    let v43: US0 = US0::US0_IntegerOnly(v42);
                                    US1::US1_DeclaredRule(v43)
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
                                        US1::US1_UnknownRule(v51)
                                    } else {
                                        US1::US1_NotARule
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
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v2: bool = &*v0 == "";
        if v2 {
            return v1;
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v4: US1 = declared_row_of_3(v3);
            match v4 {
                US1::US1_DeclaredRule(v5) => {
                    let v5: US0 = v5;
                    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    let v7: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v5, v1));
                    (v0, v1) = (v6, v7);
                    continue;
                }
                _ => {
                    let v9: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
                    v0 = v9;
                    continue;
                }
            }
        }
    }
}
fn declared_or_default_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH0> = declared_from_2(v0, v1);
    match &*v2 {
        UH0::UH0_NoStandards => {
            let v3: US0 = US0::US0_NoCommentLines;
            let v4: US0 = US0::US0_LineWidth(120i32);
            let v5: US0 = US0::US0_LfOnly;
            let v6: US0 = US0::US0_TestTargets;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoStandards); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v6, v7));
            let v9: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v5, v8));
            let v10: Rc<UH0> = Rc::new(UH0::UH0_StandardThen(v4, v9));
            Rc::new(UH0::UH0_StandardThen(v3, v10))
        }
        _ => {
            v2
        }
    }
}
fn file_scan_start_4() -> (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0i32, false, v0, false, false, false, false, false)
}
fn outside_quotes_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            (v1, v2) = (v5, v8);
            continue;
        }
    }
}
fn unquoted_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    outside_quotes_9(v2, v3, v4)
}
fn method11(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn line_findings_7(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: &Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: bool) -> Rc<str> {
    match v0 {
        US0::US0_HostFree => {
            match v5 {
                US2::US2_CodeLine => {
                    let v143: Rc<str> = unquoted_8(v4);
                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                    let v145: bool = v143.contains(&*v144);
                    if v145 {
                        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a host macro in a host-free package; reach for lib/spiral (sm', am', listm', file_system) or the OS boundary package"); } LIT.with(|lit| lit.clone()) };
                        let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                        let v148: Rc<str> = format_real_10(v3);
                        let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v148));
                        let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v151: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free  "); } LIT.with(|lit| lit.clone()) }, v146));
                        let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v152));
                        v153
                    } else {
                        let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v154
                    }
                }
                _ => {
                    let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v156
                }
            }
        }
        US0::US0_IntegerOnly(v55) => {
            let v55: Rc<str> = v55;
            match v5 {
                US2::US2_CodeLine => {
                    let v56: bool = &*v55 == &*v2;
                    let v58: bool = if v56 {
                        let v57: bool = { let line = &*v4; line.contains("f64") || line.contains("f32") };
                        v57
                    } else {
                        false
                    };
                    if v58 {
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("floating point in an integer-only module"); } LIT.with(|lit| lit.clone()) };
                        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                        let v61: Rc<str> = format_real_10(v3);
                        let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only  "); } LIT.with(|lit| lit.clone()) }, v59));
                        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v65));
                        v66
                    } else {
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v67
                    }
                }
                _ => {
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v69
                }
            }
        }
        US0::US0_LfOnly => {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v44: bool = v4.ends_with(&*v43);
            if v44 {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CRLF line ending"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                let v47: Rc<str> = format_real_10(v3);
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lf-only  "); } LIT.with(|lit| lit.clone()) }, v45));
                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v51));
                v52
            } else {
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v53
            }
        }
        US0::US0_LibReferenced => {
            let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v158
        }
        US0::US0_LineWidth(v27) => {
            let v27: i32 = v27;
            let v28: i32 = (v4).trim_end_matches(char::from(13u8)).chars().count() as i32;
            let v29: bool = v27 < v28;
            if v29 {
                let v30: Rc<str> = format_real_10(v28);
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns, limit "); } LIT.with(|lit| lit.clone()) }));
                let v32: Rc<str> = format_real_10(v27);
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                let v35: Rc<str> = format_real_10(v3);
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line-width  "); } LIT.with(|lit| lit.clone()) }, v33));
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v39));
                v40
            } else {
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v41
            }
        }
        US0::US0_NoCommentLines => {
            match v5 {
                US2::US2_CommentLine => {
                    if v8 {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v15
                    } else {
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a comment line; encode it in names or drop it"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                        let v18: Rc<str> = format_real_10(v3);
                        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines  "); } LIT.with(|lit| lit.clone()) }, v16));
                        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
                        v23
                    }
                }
                _ => {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v25
                }
            }
        }
        US0::US0_TestTargets => {
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v72: bool = v4.starts_with(&*v71);
            let v75: bool = if v72 {
                true
            } else {
                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
                let v74: bool = v4.starts_with(&*v73);
                v74
            };
            let v79: bool = if v75 {
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v77: bool = v4.contains(&*v76);
                let v78: bool = v77 == false;
                v78
            } else {
                false
            };
            let v81: bool = if v79 {
                let v80: bool = v12 == false;
                v80
            } else {
                false
            };
            let v83: bool = if v81 {
                let v82: bool = v14 == false;
                v82
            } else {
                false
            };
            if v83 {
                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a test block in a module without a notebook header never runs; start the file with ///- --package . and a :# <module> static = prelude"); } LIT.with(|lit| lit.clone()) };
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                let v86: Rc<str> = format_real_10(v3);
                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
                let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v87, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets  "); } LIT.with(|lit| lit.clone()) }, v84));
                let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v90));
                v91
            } else {
                let v93: bool = if v79 {
                    let v92: bool = v12 == false;
                    v92
                } else {
                    false
                };
                let v94: bool = v93 && v13;
                if v94 {
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a ///> _ main above the first test skips the whole module cell in notebooks, so every test sees nothing; move ///> _ and main below the last test"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                    let v97: Rc<str> = format_real_10(v3);
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets  "); } LIT.with(|lit| lit.clone()) }, v95));
                    let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v101));
                    v102
                } else {
                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
                    let v104: bool = v4.starts_with(&*v103);
                    let v108: bool = if v104 {
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                        let v106: bool = v4.contains(&*v105);
                        let v107: bool = v106 == false;
                        v107
                    } else {
                        false
                    };
                    let v112: bool = if v108 {
                        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                        let v110: bool = v9.starts_with(&*v109);
                        let v111: bool = v110 == false;
                        v111
                    } else {
                        false
                    };
                    let v114: bool = if v112 {
                        let v113: bool = v10 == false;
                        v113
                    } else {
                        false
                    };
                    if v114 {
                        let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test block without a ///> target line above it or in the file header"); } LIT.with(|lit| lit.clone()) };
                        let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                        let v117: Rc<str> = format_real_10(v3);
                        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v117));
                        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets  "); } LIT.with(|lit| lit.clone()) }, v115));
                        let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v121));
                        v122
                    } else {
                        let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= main () : i32 ="); } LIT.with(|lit| lit.clone()) };
                        let v124: bool = v4.starts_with(&*v123);
                        let v125: bool = v124 && v12;
                        let v127: bool = if v125 {
                            let v126: bool = &*v9 != "///> _";
                            v126
                        } else {
                            false
                        };
                        let v129: bool = if v127 {
                            let v128: bool = v11 == false;
                            v128
                        } else {
                            false
                        };
                        if v129 {
                            let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a main after a test block builds on the default Zig target; put ///> _ above it"); } LIT.with(|lit| lit.clone()) };
                            let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
                            let v132: Rc<str> = format_real_10(v3);
                            let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                            let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets  "); } LIT.with(|lit| lit.clone()) }, v130));
                            let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                            let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v136));
                            v137
                        } else {
                            let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v138
                        }
                    }
                }
            }
        }
    }
}
fn rules_applied_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: &Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: bool, mut v15: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoStandards => {
                return v15;
            }
            UH0::UH0_StandardThen(v16, v17) => {
                let v16: US0 = v16;
                let v17: Rc<UH0> = v17;
                let v18: Rc<str> = line_findings_7(v16, v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6, v7, v8, v9.clone(), v10, v11, v12, v13, v14);
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v18));
                (v0, v15) = (v17, v19);
                continue;
            }
        }
    }
}
fn lines_scanned_5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: bool, mut v8: Rc<str>, mut v9: bool, mut v10: bool, mut v11: bool, mut v12: bool, mut v13: bool) -> (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) {
    let v14: bool = &*v3 == "";
    let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) = if v14 {
        (v5, v6, v7, v8, v9, v10, v11, v12, v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
        let v16: Rc<str> = std::rc::Rc::<str>::from((v15).trim_start());
        let v17: bool = &*v16 == "";
        let v19: bool = if v17 {
            true
        } else {
            let v18: bool = &*v16 == "\r";
            v18
        };
        let v34: US2 = if v19 {
            US2::US2_BlankLine
        } else {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
            let v22: bool = v16.starts_with(&*v21);
            if v22 {
                US2::US2_DirectiveLine
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let v25: bool = v16.starts_with(&*v24);
                if v25 {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                    let v27: bool = v16.contains(&*v26);
                    if v27 {
                        US2::US2_TodoLine
                    } else {
                        US2::US2_CommentLine
                    }
                } else {
                    US2::US2_CodeLine
                }
            }
        };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = rules_applied_6(v0.clone(), v1.clone(), v2.clone(), v4, v15.clone(), v34.clone(), &v5, v6, v7, v8.clone(), v9, v10, v11, v12, v13, v35);
        let v38: bool = match &v34 {
            US2::US2_CommentLine => {
                v7
            }
            US2::US2_TodoLine => {
                true
            }
            _ => {
                false
            }
        };
        let v40: Rc<str> = match v34 {
            US2::US2_BlankLine => {
                v8.clone()
            }
            _ => {
                let v39: Rc<str> = std::rc::Rc::<str>::from((v15).trim_start());
                v39
            }
        };
        let v49: bool = if v9 {
            true
        } else {
            let v41: bool = v4 == 2i32;
            let v44: bool = if v41 {
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v43: bool = v8.starts_with(&*v42);
                v43
            } else {
                false
            };
            if v44 {
                let v45: Rc<str> = std::rc::Rc::<str>::from((v15).trim_start());
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                let v47: bool = v45.starts_with(&*v46);
                v47
            } else {
                false
            }
        };
        let v57: bool = if v10 {
            true
        } else {
            let v50: bool = v4 == 2i32;
            let v53: bool = if v50 {
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v52: bool = v8.starts_with(&*v51);
                v52
            } else {
                false
            };
            if v53 {
                let v54: Rc<str> = std::rc::Rc::<str>::from((v15).trim_start());
                let v55: bool = &*v54 == "///> _";
                v55
            } else {
                false
            }
        };
        let v64: bool = if v11 {
            true
        } else {
            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v59: bool = v15.starts_with(&*v58);
            if v59 {
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v61: bool = v15.contains(&*v60);
                let v62: bool = v61 == false;
                v62
            } else {
                false
            }
        };
        let v70: bool = if v13 {
            true
        } else {
            let v65: bool = v4 == 1i32;
            if v65 {
                let v66: Rc<str> = std::rc::Rc::<str>::from((v15).trim_start());
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v68: bool = v66.starts_with(&*v67);
                v68
            } else {
                false
            }
        };
        let v77: bool = if v12 {
            true
        } else {
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= main () : i32 ="); } LIT.with(|lit| lit.clone()) };
            let v72: bool = v15.starts_with(&*v71);
            let v74: bool = if v72 {
                let v73: bool = &*v8 == "///> _";
                v73
            } else {
                false
            };
            if v74 {
                let v75: bool = v11 == false;
                v75
            } else {
                false
            }
        };
        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v36.clone()));
        let v79: i32 = (v36).lines().count() as i32;
        let v80: i32 = v6.wrapping_add(v79);
        let v81: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v82: i32 = v4.wrapping_add(1i32);
        let (mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) = lines_scanned_5(v0, v1, v2, v81, v82, v78, v80, v38, v40, v49, v57, v64, v77, v70);
        (v83, v84, v85, v86, v87, v88, v89, v90, v91)
    };
    (v92, v93, v94, v95, v96, v97, v98, v99, v100)
}
fn file_findings_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<UH0> = declared_or_default_1(v0);
    let v5: i32 = 1i32;
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) = file_scan_start_4();
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool, bool) = lines_scanned_5(v4, v1.clone(), v2, v3.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let v24: Rc<str> = kit_rec_cycle::kit_rec_cycle_findings(&*v1, &*v3);
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v24));
    v25
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        file_findings_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_standards_file_findings(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
