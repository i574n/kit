#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ListParsed(Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>),
    US0_ListRejected(Rc<str>),
}
fn status_word_known_2(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 == "all";
    if v1 {
        true
    } else {
        let v2: bool = &*v0 == "Planned";
        if v2 {
            true
        } else {
            let v3: bool = &*v0 == "Active";
            if v3 {
                true
            } else {
                let v4: bool = &*v0 == "Blocked";
                if v4 {
                    true
                } else {
                    let v5: bool = &*v0 == "Paused";
                    if v5 {
                        true
                    } else {
                        let v6: bool = &*v0 == "Done";
                        v6
                    }
                }
            }
        }
    }
}
fn list_args_from_1(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<str>) -> US0 {
    loop {
        let v8: bool = v1 <= v0;
        if v8 {
            let v9: bool = agile_planning_filters::kit_agile_planning_filters_valid(&*v3, &*v4);
            if v9 {
                return US0::US0_ListParsed(v2.clone(), v3.clone(), v4.clone(), v5, v6.clone(), v7.clone());
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid planning filter (expected --load light|medium|heavy|none and --assignee agent|pair|user|none)"); } LIT.with(|lit| lit.clone()) };
                return US0::US0_ListRejected(v11.clone());
            }
        } else {
            let v14: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v15: bool = &*v14 == "--json";
            if v15 {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = true;
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v16, v1, v2.clone(), v3.clone(), v4.clone(), v17, v6.clone(), v7.clone());
                continue;
            } else {
                let v19: bool = &*v14 == "--load";
                if v19 {
                    let v20: i32 = v0.wrapping_add(2i32);
                    let v21: i32 = v0.wrapping_add(1i32);
                    let v22: Rc<str> = usize::try_from(v21).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v20, v1, v2.clone(), v22.clone(), v4.clone(), v5, v6.clone(), v7.clone());
                    continue;
                } else {
                    let v24: bool = &*v14 == "--assignee";
                    if v24 {
                        let v25: i32 = v0.wrapping_add(2i32);
                        let v26: i32 = v0.wrapping_add(1i32);
                        let v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v25, v1, v2.clone(), v3.clone(), v27.clone(), v5, v6.clone(), v7.clone());
                        continue;
                    } else {
                        let v29: bool = &*v14 == "--sort";
                        if v29 {
                            let v30: i32 = v0.wrapping_add(1i32);
                            let v31: Rc<str> = usize::try_from(v30).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v32: bool = &*v31 == "priority";
                            let v34: bool = if v32 {
                                true
                            } else {
                                let v33: bool = &*v31 == "difficulty";
                                v33
                            };
                            let v36: bool = if v34 {
                                true
                            } else {
                                let v35: bool = &*v31 == "progress";
                                v35
                            };
                            if v36 {
                                let v37: i32 = v0.wrapping_add(2i32);
                                (v0, v1, v2, v3, v4, v5, v6, v7) = (v37, v1, v2.clone(), v3.clone(), v4.clone(), v5, v6.clone(), v31.clone());
                                continue;
                            } else {
                                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sort expects priority|difficulty|progress, got "); } LIT.with(|lit| lit.clone()) };
                                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v31));
                                return US0::US0_ListRejected(v40.clone());
                            }
                        } else {
                            let v43: i32 = v0.wrapping_add(1i32);
                            let v44: Rc<str> = usize::try_from(v43).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v45: i32 = agile_task_query::kit_query_flag_width(&*v14, &*v44);
                            let v46: bool = v45 > 0i32;
                            if v46 {
                                let v47: i32 = v0.wrapping_add(v45);
                                let v48: bool = v45 == 2i32;
                                let v50: Rc<str> = if v48 {
                                    v44.clone()
                                } else {
                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v49.clone()
                                };
                                let v51: bool = &*v6 == "";
                                let v54: Rc<str> = if v51 {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v52.clone()
                                } else {
                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v53.clone()
                                };
                                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v54)), v14));
                                let v57: bool = &*v50 == "";
                                let v60: Rc<str> = if v57 {
                                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v58.clone()
                                } else {
                                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v59.clone()
                                };
                                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v56, v60)), v50));
                                (v0, v1, v2, v3, v4, v5, v6, v7) = (v47, v1, v2.clone(), v3.clone(), v4.clone(), v5, v62.clone(), v7.clone());
                                continue;
                            } else {
                                let v64: bool = v45 < 0i32;
                                if v64 {
                                    let v65: Rc<str> = agile_task_query::kit_query_flag_refusal(&*v14, &*v44);
                                    return US0::US0_ListRejected(v65.clone());
                                } else {
                                    let v67: bool = status_word_known_2(v14.clone());
                                    if v67 {
                                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v43, v1, v14.clone(), v3.clone(), v4.clone(), v5, v6.clone(), v7.clone());
                                        continue;
                                    } else {
                                        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) };
                                        let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v14));
                                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done; or "); } LIT.with(|lit| lit.clone()) };
                                        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                                        let v73: Rc<str> = agile_task_query::kit_query_usage();
                                        let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
                                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", --sort priority|difficulty|progress)"); } LIT.with(|lit| lit.clone()) };
                                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
                                        return US0::US0_ListRejected(v76.clone());
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
fn list_request_0() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
    let v0: i32 = 2i32;
    let v1: usize = std::env::args().count();
    let v2: i32 = ((v1) as i32);
    let v3: bool = v2 > 2i32;
    let v5: i32 = if v3 {
        let v4: i32 = v2.wrapping_sub(2i32);
        v4
    } else {
        0i32
    };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let v7: bool = false;
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let v10: US0 = list_args_from_1(v0, v5, v6.clone(), v6.clone(), v6.clone(), v7, v8.clone(), v9.clone());
    match &v10 {
        US0::US0_ListParsed(v11, v12, v13, v14, v15, v16) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let v14: bool = *v14;
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = v16.clone();
            (true, v8.clone(), v11.clone(), v12.clone(), v13.clone(), v14, v15.clone(), v16.clone())
        }
        US0::US0_ListRejected(v17) => {
            let v17: Rc<str> = v17.clone();
            (false, v17.clone(), v8.clone(), v8.clone(), v8.clone(), false, v8.clone(), v8.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
        list_request_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_list_request() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
    closure0()()
}
