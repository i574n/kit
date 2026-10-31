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
struct Mut0 { l0: Rc<str> }
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
            US1::US1_DeclaredRule(v16.clone())
        } else {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LfOnly"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v0.contains(&*v18);
            if v19 {
                let v20: US0 = US0::US0_LfOnly;
                US1::US1_DeclaredRule(v20.clone())
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= TestTargets"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v0.contains(&*v22);
                if v23 {
                    let v24: US0 = US0::US0_TestTargets;
                    US1::US1_DeclaredRule(v24.clone())
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= HostFree"); } LIT.with(|lit| lit.clone()) };
                    let v27: bool = v0.contains(&*v26);
                    if v27 {
                        let v28: US0 = US0::US0_HostFree;
                        US1::US1_DeclaredRule(v28.clone())
                    } else {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LibReferenced"); } LIT.with(|lit| lit.clone()) };
                        let v31: bool = v0.contains(&*v30);
                        if v31 {
                            let v32: US0 = US0::US0_LibReferenced;
                            US1::US1_DeclaredRule(v32.clone())
                        } else {
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= LineWidth "); } LIT.with(|lit| lit.clone()) };
                            let v35: bool = v0.contains(&*v34);
                            if v35 {
                                let v36: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v34).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                let v37: i32 = (v36).parse::<i32>().unwrap_or(120);
                                let v38: US0 = US0::US0_LineWidth(v37);
                                US1::US1_DeclaredRule(v38.clone())
                            } else {
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= IntegerOnly "); } LIT.with(|lit| lit.clone()) };
                                let v41: bool = v0.contains(&*v40);
                                if v41 {
                                    let v42: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(&*v40).map(|(_, rest)| rest.trim().trim_end_matches("u32").trim_end_matches("i32").trim_matches(char::from(34u8))).unwrap_or(""));
                                    let v43: US0 = US0::US0_IntegerOnly(v42.clone());
                                    US1::US1_DeclaredRule(v43.clone())
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
                                        US1::US1_UnknownRule(v51.clone())
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
            return v1.clone();
        } else {
            let v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
            let v4: US1 = declared_row_of_3(v3.clone());
            match &v4 {
                US1::US1_DeclaredRule(v5) => {
                    let v5: US0 = v5.clone();
                    let v6: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
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
fn file_scan_start_4() -> (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0i32, false, v0, false, false, false, false)
}
fn outside_quotes_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            (v0, v1, v2) = (v0.clone(), v5, v9.clone());
            continue;
        }
    }
}
fn unquoted_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    outside_quotes_9(v2.clone(), v3, v4.clone())
}
fn method11(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn line_findings_7(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool, mut v12: bool, mut v13: bool) -> Rc<str> {
    match &v0 {
        US0::US0_HostFree => {
            match &v5 {
                US2::US2_CodeLine => {
                    let v166: Rc<str> = unquoted_8(v4.clone());
                    let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
                    let v168: bool = v166.contains(&*v167);
                    if v168 {
                        let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a host macro in a host-free package; reach for lib/spiral (sm', am', listm', file_system) or the OS boundary package"); } LIT.with(|lit| lit.clone()) };
                        let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v171: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v170));
                        let v172: Rc<str> = format_real_10(v3);
                        let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
                        let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v174));
                        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-free"); } LIT.with(|lit| lit.clone()) };
                        let v178: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v176, v174)), v169));
                        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v181: Rc<str> = Rc::<str>::from(format!("{}{}", v175, Rc::<str>::from(format!("{}{}", v178, v179))));
                        v181.clone()
                    } else {
                        let v182: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v182.clone()
                    }
                }
                _ => {
                    let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v184.clone()
                }
            }
        }
        US0::US0_IntegerOnly(v70) => {
            let v70: Rc<str> = v70.clone();
            match &v5 {
                US2::US2_CodeLine => {
                    let v71: bool = &*v70 == &*v2;
                    let v73: bool = if v71 {
                        let v72: bool = { let line = &*v4; line.contains("f64") || line.contains("f32") };
                        v72
                    } else {
                        false
                    };
                    if v73 {
                        let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("floating point in an integer-only module"); } LIT.with(|lit| lit.clone()) };
                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v75));
                        let v77: Rc<str> = format_real_10(v3);
                        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("integer-only"); } LIT.with(|lit| lit.clone()) };
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v81, v79)), v74));
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v80, Rc::<str>::from(format!("{}{}", v83, v84))));
                        v86.clone()
                    } else {
                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v87.clone()
                    }
                }
                _ => {
                    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v89.clone()
                }
            }
        }
        US0::US0_LfOnly => {
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v54: bool = v4.ends_with(&*v53);
            if v54 {
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CRLF line ending"); } LIT.with(|lit| lit.clone()) };
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v56));
                let v58: Rc<str> = format_real_10(v3);
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lf-only"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v62, v60)), v55));
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v61, Rc::<str>::from(format!("{}{}", v64, v65))));
                v67.clone()
            } else {
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v68.clone()
            }
        }
        US0::US0_LibReferenced => {
            let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v186.clone()
        }
        US0::US0_LineWidth(v31) => {
            let v31: i32 = *v31;
            let v32: i32 = (v4).trim_end_matches(char::from(13u8)).chars().count() as i32;
            let v33: bool = v31 < v32;
            if v33 {
                let v34: Rc<str> = format_real_10(v32);
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns, limit "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                let v37: Rc<str> = format_real_10(v31);
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v39));
                let v41: Rc<str> = format_real_10(v3);
                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line-width"); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v45, v43)), v38));
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v44, Rc::<str>::from(format!("{}{}", v47, v48))));
                v50.clone()
            } else {
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v51.clone()
            }
        }
        US0::US0_NoCommentLines => {
            match &v5 {
                US2::US2_CommentLine => {
                    if v8 {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v14.clone()
                    } else {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a comment line; encode it in names or drop it"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v16));
                        let v18: Rc<str> = format_real_10(v3);
                        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no-comment-lines"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v20)), v15));
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v21, Rc::<str>::from(format!("{}{}", v24, v25))));
                        v27.clone()
                    }
                }
                _ => {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29.clone()
                }
            }
        }
        US0::US0_TestTargets => {
            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v92: bool = v4.starts_with(&*v91);
            let v95: bool = if v92 {
                true
            } else {
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
                let v94: bool = v4.starts_with(&*v93);
                v94
            };
            let v99: bool = if v95 {
                let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v97: bool = v4.contains(&*v96);
                let v98: bool = v97 == false;
                v98
            } else {
                false
            };
            let v101: bool = if v99 {
                let v100: bool = v12 == false;
                v100
            } else {
                false
            };
            let v103: bool = if v101 {
                let v102: bool = v13 == false;
                v102
            } else {
                false
            };
            if v103 {
                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a test block in a module without a notebook header never runs; start the file with ///- --package . and a :# <module> static = prelude"); } LIT.with(|lit| lit.clone()) };
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v105));
                let v107: Rc<str> = format_real_10(v3);
                let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v108, v109));
                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                let v113: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v111, v109)), v104));
                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v110, Rc::<str>::from(format!("{}{}", v113, v114))));
                v116.clone()
            } else {
                let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test "); } LIT.with(|lit| lit.clone()) };
                let v118: bool = v4.starts_with(&*v117);
                let v122: bool = if v118 {
                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                    let v120: bool = v4.contains(&*v119);
                    let v121: bool = v120 == false;
                    v121
                } else {
                    false
                };
                let v126: bool = if v122 {
                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                    let v124: bool = v9.starts_with(&*v123);
                    let v125: bool = v124 == false;
                    v125
                } else {
                    false
                };
                let v128: bool = if v126 {
                    let v127: bool = v10 == false;
                    v127
                } else {
                    false
                };
                if v128 {
                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test block without a ///> target line above it or in the file header"); } LIT.with(|lit| lit.clone()) };
                    let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v130));
                    let v132: Rc<str> = format_real_10(v3);
                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                    let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v134));
                    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                    let v138: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v136, v134)), v129));
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v135, Rc::<str>::from(format!("{}{}", v138, v139))));
                    v141.clone()
                } else {
                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":= main () : i32 ="); } LIT.with(|lit| lit.clone()) };
                    let v143: bool = v4.starts_with(&*v142);
                    let v144: bool = v143 && v12;
                    let v146: bool = if v144 {
                        let v145: bool = &*v9 != "///> _";
                        v145
                    } else {
                        false
                    };
                    let v148: bool = if v146 {
                        let v147: bool = v11 == false;
                        v147
                    } else {
                        false
                    };
                    if v148 {
                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a main after a test block builds on the default Zig target; put ///> _ above it"); } LIT.with(|lit| lit.clone()) };
                        let v150: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                        let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v150));
                        let v152: Rc<str> = format_real_10(v3);
                        let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v152));
                        let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                        let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v154));
                        let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("test-targets"); } LIT.with(|lit| lit.clone()) };
                        let v158: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v156, v154)), v149));
                        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v155, Rc::<str>::from(format!("{}{}", v158, v159))));
                        v161.clone()
                    } else {
                        let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v162.clone()
                    }
                }
            }
        }
    }
}
fn rules_applied_6(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: US2, mut v6: Rc<str>, mut v7: i32, mut v8: bool, mut v9: Rc<str>, mut v10: bool, mut v11: bool, mut v12: bool, mut v13: bool, mut v14: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoStandards => {
                return v14.clone();
            }
            UH0::UH0_StandardThen(v15, v16) => {
                let v15: US0 = v15.clone();
                let v16: Rc<UH0> = v16.clone();
                let v17: Rc<str> = line_findings_7(v15.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10, v11, v12, v13);
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v17));
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v16.clone(), v1.clone(), v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v7, v8, v9.clone(), v10, v11, v12, v13, v18.clone());
                continue;
            }
        }
    }
}
fn lines_scanned_5(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: bool, mut v8: Rc<str>, mut v9: bool, mut v10: bool, mut v11: bool, mut v12: bool) -> (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) {
    let v13: bool = &*v3 == "";
    let (mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) = if v13 {
        (v5.clone(), v6, v7, v8.clone(), v9, v10, v11, v12)
    } else {
        let v14: Rc<str> = std::rc::Rc::<str>::from((v3).split(char::from(10u8)).next().unwrap_or(""));
        let v15: Rc<str> = std::rc::Rc::<str>::from((v14).trim_start());
        let v16: bool = &*v15 == "";
        let v18: bool = if v16 {
            true
        } else {
            let v17: bool = &*v15 == "\r";
            v17
        };
        let v35: US2 = if v18 {
            US2::US2_BlankLine
        } else {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///"); } LIT.with(|lit| lit.clone()) };
            let v21: bool = v15.starts_with(&*v20);
            if v21 {
                US2::US2_DirectiveLine
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("//"); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v15.starts_with(&*v23);
                if v24 {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TO"); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DO"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TODO"); } LIT.with(|lit| lit.clone()) };
                    let v28: bool = v15.contains(&*v27);
                    if v28 {
                        US2::US2_TodoLine
                    } else {
                        US2::US2_CommentLine
                    }
                } else {
                    US2::US2_CodeLine
                }
            }
        };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = rules_applied_6(v0.clone(), v1.clone(), v2.clone(), v4, v14.clone(), v35.clone(), v5.clone(), v6, v7, v8.clone(), v9, v10, v11, v12, v36.clone());
        let v39: bool = match &v35 {
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
        let v41: Rc<str> = match &v35 {
            US2::US2_BlankLine => {
                v8.clone()
            }
            _ => {
                let v40: Rc<str> = std::rc::Rc::<str>::from((v14).trim_start());
                v40.clone()
            }
        };
        let v50: bool = if v9 {
            true
        } else {
            let v42: bool = v4 == 2i32;
            let v45: bool = if v42 {
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v44: bool = v8.starts_with(&*v43);
                v44
            } else {
                false
            };
            if v45 {
                let v46: Rc<str> = std::rc::Rc::<str>::from((v14).trim_start());
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///> "); } LIT.with(|lit| lit.clone()) };
                let v48: bool = v46.starts_with(&*v47);
                v48
            } else {
                false
            }
        };
        let v58: bool = if v10 {
            true
        } else {
            let v51: bool = v4 == 2i32;
            let v54: bool = if v51 {
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v53: bool = v8.starts_with(&*v52);
                v53
            } else {
                false
            };
            if v54 {
                let v55: Rc<str> = std::rc::Rc::<str>::from((v14).trim_start());
                let v56: bool = &*v55 == "///> _";
                v56
            } else {
                false
            }
        };
        let v65: bool = if v11 {
            true
        } else {
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":# "); } LIT.with(|lit| lit.clone()) };
            let v60: bool = v14.starts_with(&*v59);
            if v60 {
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" static ="); } LIT.with(|lit| lit.clone()) };
                let v62: bool = v14.contains(&*v61);
                let v63: bool = v62 == false;
                v63
            } else {
                false
            }
        };
        let v71: bool = if v12 {
            true
        } else {
            let v66: bool = v4 == 1i32;
            if v66 {
                let v67: Rc<str> = std::rc::Rc::<str>::from((v14).trim_start());
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///-"); } LIT.with(|lit| lit.clone()) };
                let v69: bool = v67.starts_with(&*v68);
                v69
            } else {
                false
            }
        };
        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v37));
        let v73: i32 = (v37).lines().count() as i32;
        let v74: i32 = v6.wrapping_add(v73);
        let v75: Rc<str> = std::rc::Rc::<str>::from((v3).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v76: i32 = v4.wrapping_add(1i32);
        let (mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) = lines_scanned_5(v0.clone(), v1.clone(), v2.clone(), v75.clone(), v76, v72.clone(), v74, v39, v41.clone(), v50, v58, v65, v71);
        (v77.clone(), v78, v79, v80.clone(), v81, v82, v83, v84)
    };
    (v85.clone(), v86, v87, v88.clone(), v89, v90, v91, v92)
}
fn file_findings_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<UH0> = declared_or_default_1(v0);
    let v5: i32 = 1i32;
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) = file_scan_start_4();
    let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, i32, bool, Rc<str>, bool, bool, bool, bool) = lines_scanned_5(v4, v1, v2, v3, v5, v6, v7, v8, v9, v10, v11, v12, v13);
    v14
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        file_findings_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_standards_file_findings(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
