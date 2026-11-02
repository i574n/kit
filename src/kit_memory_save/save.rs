#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_KindUser,
    US1_KindFeedback,
    US1_KindProject,
    US1_KindReference,
}
#[derive(Clone)]
enum US0 {
    US0_KindKnown(US1),
    US0_KindUnknown,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_Ok,
    US2_Error(Rc<str>),
}
struct Mut1 { l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_EntryRead(Rc<str>, Rc<str>, Rc<str>, US1, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US3_EntryRefused(Rc<str>),
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn kind_of_word_2(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_ascii_lowercase().as_str());
    let v3: bool = &*v0 == &*v2;
    if v3 {
        let v4: US1 = US1::US1_KindUser;
        US0::US0_KindKnown(v4)
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = std::rc::Rc::<str>::from((v6).to_ascii_lowercase().as_str());
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US1 = US1::US1_KindFeedback;
            US0::US0_KindKnown(v9)
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = std::rc::Rc::<str>::from((v11).to_ascii_lowercase().as_str());
            let mut v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US1 = US1::US1_KindProject;
                US0::US0_KindKnown(v14)
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_ascii_lowercase().as_str());
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US1 = US1::US1_KindReference;
                    US0::US0_KindKnown(v19)
                } else {
                    US0::US0_KindUnknown
                }
            }
        }
    }
}
fn slug_valid_3(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 != "";
    let v4: bool = if v1 {
        let v2: Rc<str> = std::rc::Rc::<str>::from((v0).to_ascii_lowercase().as_str());
        let v3: bool = &*v2 == &*v0;
        v3
    } else {
        false
    };
    let v8: bool = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.contains(&*v5);
        let v7: bool = v6 == false;
        v7
    } else {
        false
    };
    let v12: bool = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v0.contains(&*v9);
        let v11: bool = v10 == false;
        v11
    } else {
        false
    };
    let v16: bool = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v14: bool = v0.contains(&*v13);
        let v15: bool = v14 == false;
        v15
    } else {
        false
    };
    if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let v18: bool = v0.contains(&*v17);
        let v19: bool = v18 == false;
        v19
    } else {
        false
    }
}
fn file_present_4(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn index_line_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("- ["); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") — "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_1(v5, v2, v6.clone(), v6.clone(), v6);
    concat5_1(v3, v0, v4, v1, v7)
}
fn rendered_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\nname: "); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ndescription: "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\nmetadata:\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = concat5_1(v8, v1, v9, v2, v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_type"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = &*v4 == "";
    let v19: Rc<str> = if v13 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        concat5_1(v15, v12, v16, v4, v17)
    };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = match &v3 {
        US1::US1_KindFeedback => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
            v22
        }
        US1::US1_KindProject => {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
            v23
        }
        US1::US1_KindReference => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
            v24
        }
        US1::US1_KindUser => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
            v21
        }
    };
    let v26: Rc<str> = std::rc::Rc::<str>::from((v25).to_ascii_lowercase().as_str());
    let v27: bool = &*v26 == "";
    let v33: Rc<str> = if v27 {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v28
    } else {
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        concat5_1(v29, v20, v30, v26, v31)
    };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("originSessionId"); } LIT.with(|lit| lit.clone()) };
    let v35: bool = &*v5 == "";
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v36
    } else {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        concat5_1(v37, v34, v38, v5, v39)
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modified"); } LIT.with(|lit| lit.clone()) };
    let v43: bool = &*v6 == "";
    let v49: Rc<str> = if v43 {
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v44
    } else {
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        concat5_1(v45, v42, v46, v6, v47)
    };
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = concat5_1(v19, v33, v41, v49, v50.clone());
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\n"); } LIT.with(|lit| lit.clone()) };
    concat5_1(v11, v51, v52, v7, v50)
}
fn memory_saved_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> i32 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".md"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_1(v2.clone(), v5.clone(), v6.clone(), v6.clone(), v6.clone());
    let v8: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v7).to_string_lossy().as_ref());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v9).to_string_lossy().as_ref());
    let v11: bool = &*v4 == "-";
    let v22: Rc<str> = if v11 {
        let v12: std::io::Stdin = std::io::stdin();
        let mut v13: Result<std::string::String, std::io::Error> = std::io::read_to_string(v12);
        let v15: &str = &*v6;
        let v17: std::string::String = String::from(v15);
        let v19: std::string::String = v13.unwrap_or(v17);
        let v21: Rc<str> = Rc::<str>::from(String::as_str(&v19));
        v21.clone()
    } else {
        v4.clone()
    };
    let v23: Rc<str> = std::rc::Rc::<str>::from(v22.trim());
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--title"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = std::rc::Rc::<str>::from({ let args = std::env::args().collect::<Vec<_>>(); args.iter().position(|argument| argument == &*v24).and_then(|at| args.get(at + 1)).cloned().unwrap_or_default() }.as_str());
    let v26: US0 = kind_of_word_2(v1.clone());
    match &v26 {
        US0::US0_KindKnown(v32) => {
            let v32: US1 = v32.clone();
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**Why:**"); } LIT.with(|lit| lit.clone()) };
            let v34: bool = v23.contains(&*v33);
            let v37: bool = if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**How to apply:**"); } LIT.with(|lit| lit.clone()) };
                let v36: bool = v23.contains(&*v35);
                v36
            } else {
                false
            };
            let v38: bool = slug_valid_3(v2.clone());
            let v39: bool = v38 == false;
            if v39 {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: a slug is lowercase words joined by dashes: "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = concat5_1(v40.clone(), v2.clone(), v6.clone(), v6.clone(), v6.clone());
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = concat5_1(v42.clone(), v41.clone(), v6.clone(), v6.clone(), v6.clone());
                eprintln!("{}", v43);
                2i32
            } else {
                let v44: bool = &*v3 == "";
                let v46: bool = if v44 {
                    true
                } else {
                    let v45: bool = &*v23 == "";
                    v45
                };
                if v46 {
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: needs a description and a body"); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = concat5_1(v48.clone(), v47.clone(), v6.clone(), v6.clone(), v6.clone());
                    eprintln!("{}", v49);
                    2i32
                } else {
                    let v50: bool = file_present_4(v8.clone());
                    if v50 {
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: "); } LIT.with(|lit| lit.clone()) };
                        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" already exists; edit it instead"); } LIT.with(|lit| lit.clone()) };
                        let v53: Rc<str> = concat5_1(v51.clone(), v7.clone(), v52.clone(), v6.clone(), v6.clone());
                        let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                        let mut v55: Rc<str> = concat5_1(v54.clone(), v53.clone(), v6.clone(), v6.clone(), v6.clone());
                        eprintln!("{}", v55);
                        2i32
                    } else {
                        let v57: bool = match &v32 {
                            US1::US1_KindFeedback => {
                                true
                            }
                            US1::US1_KindProject => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let mut v59: bool = if v57 {
                            let mut v58: bool = v37 == false;
                            v58
                        } else {
                            false
                        };
                        if v59 {
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: a "); } LIT.with(|lit| lit.clone()) };
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" memory needs **Why:** and **How to apply:** lines"); } LIT.with(|lit| lit.clone()) };
                            let v62: Rc<str> = concat5_1(v60.clone(), v1.clone(), v61.clone(), v6.clone(), v6.clone());
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = concat5_1(v63.clone(), v62.clone(), v6.clone(), v6.clone(), v6.clone());
                            eprintln!("{}", v64);
                            2i32
                        } else {
                            let v65: bool = file_present_4(v10.clone());
                            let v66: bool = v65 == false;
                            if v66 {
                                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: no MEMORY.md in "); } LIT.with(|lit| lit.clone()) };
                                let v68: Rc<str> = concat5_1(v67.clone(), v0.clone(), v6.clone(), v6.clone(), v6.clone());
                                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                                let v70: Rc<str> = concat5_1(v69.clone(), v68.clone(), v6.clone(), v6.clone(), v6.clone());
                                eprintln!("{}", v70);
                                2i32
                            } else {
                                let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                let v72: Rc<str> = concat5_1(v71.clone(), v23.clone(), v71.clone(), v6.clone(), v6.clone());
                                let v73: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
                                let v74: bool = &*v73 == "";
                                let v76: bool = if v74 {
                                    true
                                } else {
                                    let v75: bool = v73.ends_with(&*v71);
                                    v75
                                };
                                let v77: Rc<str> = if v76 {
                                    v6.clone()
                                } else {
                                    v71.clone()
                                };
                                let v78: bool = &*v25 == "";
                                let v79: Rc<str> = if v78 {
                                    v2.clone()
                                } else {
                                    v25.clone()
                                };
                                let v80: Rc<str> = index_line_5(v79.clone(), v7.clone(), v3.clone());
                                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory"); } LIT.with(|lit| lit.clone()) };
                                let v82: Rc<str> = rendered_6(v7.clone(), v2.clone(), v3.clone(), v32.clone(), v81.clone(), v6.clone(), v6.clone(), v72.clone());
                                let v83: bool = { let path = std::path::Path::new(&*v8); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v82).as_bytes()).is_ok() };
                                let v86: bool = if v83 {
                                    let v84: Rc<str> = concat5_1(v73.clone(), v77.clone(), v80.clone(), v71.clone(), v6.clone());
                                    let v85: bool = { let path = std::path::Path::new(&*v10); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v84).as_bytes()).is_ok() };
                                    v85
                                } else {
                                    false
                                };
                                if v86 {
                                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory save ok file="); } LIT.with(|lit| lit.clone()) };
                                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" kind="); } LIT.with(|lit| lit.clone()) };
                                    let v89: Rc<str> = concat5_1(v87.clone(), v7.clone(), v88.clone(), v1.clone(), v6.clone());
                                    println!("{}", v89);
                                    0i32
                                } else {
                                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: could not write "); } LIT.with(|lit| lit.clone()) };
                                    let v91: Rc<str> = concat5_1(v90.clone(), v8.clone(), v6.clone(), v6.clone(), v6.clone());
                                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                                    let v93: Rc<str> = concat5_1(v92.clone(), v91.clone(), v6.clone(), v6.clone(), v6.clone());
                                    eprintln!("{}", v93);
                                    2i32
                                }
                            }
                        }
                    }
                }
            }
        }
        US0::US0_KindUnknown => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("save: unknown kind '"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'; use user, feedback, project or reference"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = concat5_1(v27.clone(), v1.clone(), v28.clone(), v6.clone(), v6.clone());
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = concat5_1(v30.clone(), v29.clone(), v6.clone(), v6.clone(), v6.clone());
            eprintln!("{}", v31);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>| -> i32 {
        memory_saved_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
fn method10(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method10(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method9(v16.clone(), v17.clone())
    }
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method11(v0.clone(), v4, v2.clone());
            let mut v6: bool = v3.contains(&*v0);
            let v7: bool = v6 == false;
            if v7 {
                Rc::new(UH0::UH0_Cons(v3, v5.clone()))
            } else {
                v5
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn file_exists_14(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US2> = Rc::new(move || -> US2 {
        US2::US2_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16() -> Rc<dyn Fn() -> US2> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US2> = Rc::new(move |mut v0: Rc<str>| -> US2 {
        US2::US2_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn(Rc<str>) -> US2> {
    error_c4()
}
fn method19(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_18(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method19(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn memory_forgotten_7(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".md"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = concat5_1(v1.clone(), v2.clone(), v3.clone(), v3.clone(), v3.clone());
    let v5: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v4).to_string_lossy().as_ref());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("archive"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v6).to_string_lossy().as_ref());
    let v8: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v7).join(&*v4).to_string_lossy().as_ref());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v9).to_string_lossy().as_ref());
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = concat5_1(v11.clone(), v4.clone(), v12.clone(), v3.clone(), v3.clone());
    let v14: bool = file_present_4(v5.clone());
    let v15: bool = v14 == false;
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forget: no "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = concat5_1(v16.clone(), v4.clone(), v17.clone(), v0.clone(), v3.clone());
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = concat5_1(v19.clone(), v18.clone(), v3.clone(), v3.clone(), v3.clone());
        eprintln!("{}", v20);
        2i32
    } else {
        let v21: bool = file_present_4(v8.clone());
        if v21 {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forget: archive/"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" already exists"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = concat5_1(v22.clone(), v4.clone(), v23.clone(), v3.clone(), v3.clone());
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = concat5_1(v25.clone(), v24.clone(), v3.clone(), v3.clone(), v3.clone());
            eprintln!("{}", v26);
            2i32
        } else {
            let v27: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
            let v28: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<UH0> = split_list_8(v29.clone(), v28.clone());
            let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v32: Rc<UH0> = method11(v13.clone(), v30.clone(), v31.clone());
            let v33: i32 = 0i32;
            let v34: i32 = method12(v30.clone(), v33);
            let v35: i32 = 0i32;
            let v36: i32 = method12(v32.clone(), v35);
            let v37: i32 = v34.wrapping_sub(v36);
            let v38: bool = { let path = std::path::Path::new(&*v8); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v27).as_bytes()).is_ok() };
            let v41: bool = if v38 {
                let v39: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
                let v40: bool = &*v39 == &*v27;
                v40
            } else {
                false
            };
            let v49: bool = if v41 {
                let v42: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                let v43: i32 = 0i32;
                let v44: i32 = method13(v42.clone(), v32.clone(), v43);
                let v45: Rc<Vec<Rc<str>>> = Rc::new(v42.borrow().clone());
                let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v45).as_ref().clone()));
                let v47: Rc<str> = Rc::<str>::from((*v46.borrow()).join(&*v29));
                let v48: bool = { let path = std::path::Path::new(&*v10); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v47).as_bytes()).is_ok() };
                v48
            } else {
                false
            };
            if v49 {
                let v50: bool = file_exists_14(v5.clone());
                let mut v54: Result<(), std::io::Error> = if v50 {
                    let v52: Result<(), std::io::Error> = std::fs::remove_file(&*v5);
                    v52
                } else {
                    let v53: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                    v53
                };
                let mut v55: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method15();
                let v57: Result<(), Rc<str>> = v54.map_err(|x| v55(x));
                let mut v58: Rc<dyn Fn() -> US2> = method16();
                let mut v59: Rc<dyn Fn(Rc<str>) -> US2> = method17();
                let v60: US2 = match v57 { Ok(()) => v58(), Err(e) => v59(e) };
                match &v60 {
                    US2::US2_Error(v61) => {
                        let v61: Rc<str> = v61.clone();
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v61));
                        eprintln!("{}", v63);
                        ()
                    }
                    US2::US2_Ok => {
                        ()
                    }
                };
                let v64: Rc<str> = format_real_18(v37);
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" index_lines="); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = concat5_1(v65.clone(), v64.clone(), v3.clone(), v3.clone(), v3.clone());
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory forget ok file="); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" archived=archive/"); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = concat5_1(v67.clone(), v4.clone(), v68.clone(), v4.clone(), v66.clone());
                println!("{}", v69);
                0i32
            } else {
                let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forget: could not archive "); } LIT.with(|lit| lit.clone()) };
                let v71: Rc<str> = concat5_1(v70.clone(), v4.clone(), v3.clone(), v3.clone(), v3.clone());
                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                let v73: Rc<str> = concat5_1(v72.clone(), v71.clone(), v3.clone(), v3.clone(), v3.clone());
                eprintln!("{}", v73);
                2i32
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        memory_forgotten_7(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn entry_of_file_21(mut v0: Rc<str>, mut v1: Rc<str>) -> US3 {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v2).unwrap_or_default().as_str());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n---\n"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = (v3).get(3i32 as usize..).and_then(|rest| rest.find(&*v4)).map(|at| at as i32 + 3i32).unwrap_or(-1);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("---\n"); } LIT.with(|lit| lit.clone()) };
    let v7: bool = v3.starts_with(&*v6);
    let v8: bool = v7 == false;
    let v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v5 < 0i32;
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no frontmatter"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = concat5_1(v1.clone(), v11.clone(), v12.clone(), v12.clone(), v12.clone());
        US3::US3_EntryRefused(v13.clone())
    } else {
        let v15: i32 = v5.wrapping_add(1i32);
        let v16: Rc<str> = std::rc::Rc::<str>::from((v3).get(4i32 as usize..v15 as usize).unwrap_or(""));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  type: "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v17)).unwrap_or(""));
        let v19: US0 = kind_of_word_2(v18.clone());
        match &v19 {
            US0::US0_KindKnown(v25) => {
                let v25: US1 = v25.clone();
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("name: "); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v26)).unwrap_or(""));
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("description: "); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v28)).unwrap_or(""));
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  node_type: "); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v30)).unwrap_or(""));
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  originSessionId: "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v32)).unwrap_or(""));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  modified: "); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v34)).unwrap_or(""));
                let v36: i32 = v5.wrapping_add(5i32);
                let v37: i32 = (v3.clone().len() as i32);
                let v38: Rc<str> = std::rc::Rc::<str>::from((v3).get(v36 as usize..v37 as usize).unwrap_or(""));
                let v39: Rc<str> = rendered_6(v1.clone(), v27.clone(), v29.clone(), v25.clone(), v31.clone(), v33.clone(), v35.clone(), v38.clone());
                let v40: bool = &*v39 == &*v3;
                if v40 {
                    US3::US3_EntryRead(v1.clone(), v27.clone(), v29.clone(), v25.clone(), v31.clone(), v33.clone(), v35.clone(), v38.clone())
                } else {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": does not render back byte-identical"); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = concat5_1(v1.clone(), v42.clone(), v43.clone(), v43.clone(), v43.clone());
                    US3::US3_EntryRefused(v44.clone())
                }
            }
            US0::US0_KindUnknown => {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": unknown metadata type "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = std::rc::Rc::<str>::from((v16).lines().find_map(|line| line.strip_prefix(&*v17)).unwrap_or(""));
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = concat5_1(v1.clone(), v20.clone(), v21.clone(), v22.clone(), v22.clone());
                US3::US3_EntryRefused(v23.clone())
            }
        }
    }
}
fn hook_replaced_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]("); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") — "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_1(v4.clone(), v1.clone(), v5, v2.clone(), v6.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = concat5_1(v4, v1, v8, v6.clone(), v6.clone());
    let v10: bool = v0.contains(&*v9);
    let v12: bool = if v10 {
        let v11: bool = v0.ends_with(&*v7);
        v11
    } else {
        false
    };
    if v12 {
        let mut v13: i32 = (v0.clone().len() as i32);
        let v14: i32 = (v2.len() as i32);
        let v15: i32 = v13.wrapping_sub(v14);
        let v16: Rc<str> = std::rc::Rc::<str>::from((v0).get(0i32 as usize..v15 as usize).unwrap_or(""));
        concat5_1(v16, v3, v6.clone(), v6.clone(), v6)
    } else {
        v0
    }
}
fn method22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: US1, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<UH0>, mut v18: Rc<UH0>) -> Rc<UH0> {
    match &*v17 {
        UH0::UH0_Cons(v19, v20) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<UH0> = v20.clone();
            let v21: Rc<UH0> = method22(v0.clone(), v1, v2, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12, v13, v14, v15, v16, v20, v18.clone());
            let v22: Rc<str> = hook_replaced_23(v19, v0, v3, v11);
            Rc::new(UH0::UH0_Cons(v22, v21))
        }
        UH0::UH0_Nil => {
            v18
        }
    }
}
fn memory_edited_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".md"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_1(v1, v4, v5.clone(), v5.clone(), v5.clone());
    let v7: bool = &*v3 == "-";
    let v18: Rc<str> = if v7 {
        let v8: std::io::Stdin = std::io::stdin();
        let mut v9: Result<std::string::String, std::io::Error> = std::io::read_to_string(v8);
        let v11: &str = &*v5;
        let mut v13: std::string::String = String::from(v11);
        let v15: std::string::String = v9.unwrap_or(v13);
        let v17: Rc<str> = Rc::<str>::from(String::as_str(&v15));
        v17
    } else {
        v3
    };
    let v19: Rc<str> = std::rc::Rc::<str>::from(v18.trim());
    let v20: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v6).to_string_lossy().as_ref());
    let v21: bool = file_present_4(v20);
    let v22: bool = v21 == false;
    if v22 {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: no "); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = concat5_1(v23, v6.clone(), v24, v0.clone(), v5.clone());
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = concat5_1(v26, v25, v5.clone(), v5.clone(), v5.clone());
        eprintln!("{}", v27);
        2i32
    } else {
        let v28: bool = &*v19 == "";
        if v28 {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: the new text is empty"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = concat5_1(v30, v29, v5.clone(), v5.clone(), v5.clone());
            eprintln!("{}", v31);
            2i32
        } else {
            let v32: bool = &*v2 != "description";
            let v34: bool = if v32 {
                let v33: bool = &*v2 != "body";
                v33
            } else {
                false
            };
            if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: the field is description or body"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = concat5_1(v36, v35, v5.clone(), v5.clone(), v5.clone());
                eprintln!("{}", v37);
                2i32
            } else {
                let v38: US3 = entry_of_file_21(v0.clone(), v6.clone());
                match &v38 {
                    US3::US3_EntryRead(v44, v45, v46, v47, v48, v49, v50, v51) => {
                        let v44: Rc<str> = v44.clone();
                        let v45: Rc<str> = v45.clone();
                        let v46: Rc<str> = v46.clone();
                        let v47: US1 = v47.clone();
                        let v48: Rc<str> = v48.clone();
                        let v49: Rc<str> = v49.clone();
                        let v50: Rc<str> = v50.clone();
                        let v51: Rc<str> = v51.clone();
                        let v52: bool = &*v2 == "body";
                        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, Rc<str>, Rc<str>, US1, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v52 {
                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v54: Rc<str> = concat5_1(v53.clone(), v19.clone(), v53, v5.clone(), v5.clone());
                            (v44.clone(), v45.clone(), v46.clone(), v47.clone(), v48.clone(), v49.clone(), v50.clone(), v54)
                        } else {
                            (v44.clone(), v45.clone(), v19, v47.clone(), v48.clone(), v49.clone(), v50.clone(), v51.clone())
                        };
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**Why:**"); } LIT.with(|lit| lit.clone()) };
                        let v64: bool = v62.contains(&*v63);
                        let v67: bool = if v64 {
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**How to apply:**"); } LIT.with(|lit| lit.clone()) };
                            let v66: bool = v62.contains(&*v65);
                            v66
                        } else {
                            false
                        };
                        let v69: bool = match &v47 {
                            US1::US1_KindFeedback => {
                                true
                            }
                            US1::US1_KindProject => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v71: bool = if v69 {
                            let v70: bool = v67 == false;
                            v70
                        } else {
                            false
                        };
                        if v71 {
                            let v76: Rc<str> = match &v47 {
                                US1::US1_KindFeedback => {
                                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feedback"); } LIT.with(|lit| lit.clone()) };
                                    v73
                                }
                                US1::US1_KindProject => {
                                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Project"); } LIT.with(|lit| lit.clone()) };
                                    v74
                                }
                                US1::US1_KindReference => {
                                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Reference"); } LIT.with(|lit| lit.clone()) };
                                    v75
                                }
                                US1::US1_KindUser => {
                                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("User"); } LIT.with(|lit| lit.clone()) };
                                    v72
                                }
                            };
                            let v77: Rc<str> = std::rc::Rc::<str>::from((v76).to_ascii_lowercase().as_str());
                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: a "); } LIT.with(|lit| lit.clone()) };
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" memory keeps **Why:** and **How to apply:** lines"); } LIT.with(|lit| lit.clone()) };
                            let v80: Rc<str> = concat5_1(v78, v77, v79, v5.clone(), v5.clone());
                            let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                            let v82: Rc<str> = concat5_1(v81, v80, v5.clone(), v5.clone(), v5.clone());
                            eprintln!("{}", v82);
                            2i32
                        } else {
                            let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MEMORY.md"); } LIT.with(|lit| lit.clone()) };
                            let v84: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v83).to_string_lossy().as_ref());
                            let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            let v86: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v84).unwrap_or_default().as_str());
                            let v87: Rc<UH0> = split_list_8(v85.clone(), v86);
                            let v88: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v89: Rc<UH0> = method22(v6.clone(), v44, v45, v46, v47, v48, v49, v50, v51, v55.clone(), v56.clone(), v57.clone(), v58.clone(), v59.clone(), v60.clone(), v61.clone(), v62.clone(), v87, v88);
                            let v97: bool = if v52 {
                                true
                            } else {
                                let v90: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v91: i32 = 0i32;
                                let v92: i32 = method13(v90.clone(), v89, v91);
                                let v93: Rc<Vec<Rc<str>>> = Rc::new(v90.borrow().clone());
                                let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                                let v95: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v85));
                                let v96: bool = { let path = std::path::Path::new(&*v84); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v95).as_bytes()).is_ok() };
                                v96
                            };
                            let v98: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v6).to_string_lossy().as_ref());
                            let v99: Rc<str> = rendered_6(v55, v56, v57, v58, v59, v60, v61, v62);
                            let v100: bool = { let path = std::path::Path::new(&*v98); let _ = path.parent().map(std::fs::create_dir_all); std::fs::write(path, (v99).as_bytes()).is_ok() };
                            let v101: bool = v100 && v97;
                            if v101 {
                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit memory edit ok file="); } LIT.with(|lit| lit.clone()) };
                                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                let v104: Rc<str> = concat5_1(v102, v6.clone(), v103, v2, v5.clone());
                                println!("{}", v104);
                                0i32
                            } else {
                                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: could not write "); } LIT.with(|lit| lit.clone()) };
                                let v106: Rc<str> = concat5_1(v105, v6, v5.clone(), v5.clone(), v5.clone());
                                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                                let v108: Rc<str> = concat5_1(v107, v106, v5.clone(), v5.clone(), v5.clone());
                                eprintln!("{}", v108);
                                2i32
                            }
                        }
                    }
                    US3::US3_EntryRefused(v39) => {
                        let v39: Rc<str> = v39.clone();
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: "); } LIT.with(|lit| lit.clone()) };
                        let v41: Rc<str> = concat5_1(v40, v39, v5.clone(), v5.clone(), v5.clone());
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: memory "); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = concat5_1(v42, v41, v5.clone(), v5.clone(), v5);
                        eprintln!("{}", v43);
                        2i32
                    }
                }
            }
        }
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> i32 {
        memory_edited_20(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_memory_save(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4))
}
pub fn kit_memory_forget(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_memory_edit(v0: &str, v1: &str, v2: &str, v3: &str) -> i32 {
    closure5()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
