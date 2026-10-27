#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ArgsParsed(Rc<str>, i32, Rc<str>, Rc<str>, bool),
    US0_ArgsRejected(Rc<str>),
}
fn load_words_2(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2.clone()
}
fn loads_valid_3(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let v4: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v5: bool = agile_planning_filters::kit_agile_planning_filters_valid(&*v4, "all");
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn next_request_from_1(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> US0 {
    loop {
        let v7: bool = v1 <= v0;
        if v7 {
            return US0::US0_ArgsParsed(v2.clone(), v3, v4.clone(), v5.clone(), v6);
        } else {
            let v9: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v10: i32 = v0.wrapping_add(1i32);
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v12: bool = &*v9 == "--as";
            if v12 {
                let v13: bool = &*v11 == "main";
                let v15: bool = if v13 {
                    true
                } else {
                    let v14: bool = &*v11 == "subagent";
                    v14
                };
                if v15 {
                    let v16: i32 = v0.wrapping_add(2i32);
                    (v0, v1, v2, v3, v4, v5, v6) = (v16, v1, v11.clone(), v3, v4.clone(), v5.clone(), v6);
                    continue;
                } else {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as expects main or subagent, got "); } LIT.with(|lit| lit.clone()) };
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
                    return US0::US0_ArgsRejected(v19.clone());
                }
            } else {
                let v22: bool = &*v9 == "--json";
                if v22 {
                    let v23: bool = true;
                    (v0, v1, v2, v3, v4, v5, v6) = (v10, v1, v2.clone(), v3, v4.clone(), v5.clone(), v23);
                    continue;
                } else {
                    let v25: bool = &*v9 == "--fits";
                    if v25 {
                        let v26: Rc<str> = kit_window_query::kit_fitting_loads();
                        (v0, v1, v2, v3, v4, v5, v6) = (v10, v1, v2.clone(), v3, v4.clone(), v26.clone(), v6);
                        continue;
                    } else {
                        let v28: bool = &*v9 == "--load";
                        if v28 {
                            let v29: bool = &*v11 != "";
                            let v33: bool = if v29 {
                                let v30: Rc<RefCell<Vec<Rc<str>>>> = load_words_2(v11.clone());
                                let v31: i32 = 0i32;
                                loads_valid_3(v30.clone(), v31)
                            } else {
                                false
                            };
                            if v33 {
                                let v34: i32 = v0.wrapping_add(2i32);
                                (v0, v1, v2, v3, v4, v5, v6) = (v34, v1, v2.clone(), v3, v4.clone(), v11.clone(), v6);
                                continue;
                            } else {
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--load expects light, medium, heavy or none, or a comma list of them, got "); } LIT.with(|lit| lit.clone()) };
                                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v11));
                                return US0::US0_ArgsRejected(v37.clone());
                            }
                        } else {
                            let v40: i32 = agile_task_query::kit_query_flag_width(&*v9, &*v11);
                            let v41: bool = v40 > 0i32;
                            if v41 {
                                let v42: i32 = v0.wrapping_add(v40);
                                let v43: bool = v40 == 2i32;
                                let v45: Rc<str> = if v43 {
                                    v11.clone()
                                } else {
                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v44.clone()
                                };
                                let v46: bool = &*v4 == "";
                                let v49: Rc<str> = if v46 {
                                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v47.clone()
                                } else {
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v48.clone()
                                };
                                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v49)), v9));
                                let v52: bool = &*v45 == "";
                                let v55: Rc<str> = if v52 {
                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v53.clone()
                                } else {
                                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v54.clone()
                                };
                                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v51, v55)), v45));
                                (v0, v1, v2, v3, v4, v5, v6) = (v42, v1, v2.clone(), v3, v57.clone(), v5.clone(), v6);
                                continue;
                            } else {
                                let v59: bool = v40 < 0i32;
                                if v59 {
                                    let v60: Rc<str> = agile_task_query::kit_query_flag_refusal(&*v9, &*v11);
                                    return US0::US0_ArgsRejected(v60.clone());
                                } else {
                                    let v62: i32 = (v9).parse::<i32>().ok().filter(|value| *value > 0).unwrap_or(0);
                                    let v63: bool = 0i32 < v62;
                                    if v63 {
                                        (v0, v1, v2, v3, v4, v5, v6) = (v10, v1, v2.clone(), v62, v4.clone(), v5.clone(), v6);
                                        continue;
                                    } else {
                                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected argument "); } LIT.with(|lit| lit.clone()) };
                                        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v9));
                                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
                                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
                                        let v69: Rc<str> = agile_task_query::kit_query_usage();
                                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected --as main|subagent, "); } LIT.with(|lit| lit.clone()) };
                                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v69));
                                        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", --load <light|medium|heavy|none,...>, --fits, --json or a positive count"); } LIT.with(|lit| lit.clone()) };
                                        let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v68, Rc::<str>::from(format!("{}{}", v71, v72))));
                                        let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
                                        return US0::US0_ArgsRejected(v76.clone());
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
fn next_request_0() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
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
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("main"); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = 10i32;
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = false;
    let v11: US0 = next_request_from_1(v0, v5, v6.clone(), v7, v8.clone(), v9.clone(), v10);
    match &v11 {
        US0::US0_ArgsParsed(v12, v13, v14, v15, v16) => {
            let v12: Rc<str> = v12.clone();
            let v13: i32 = *v13;
            let v14: Rc<str> = v14.clone();
            let v15: Rc<str> = v15.clone();
            let v16: bool = *v16;
            (true, v8.clone(), v12.clone(), v13, v14.clone(), v15.clone(), v16)
        }
        US0::US0_ArgsRejected(v17) => {
            let v17: Rc<str> = v17.clone();
            (false, v17.clone(), v8.clone(), 0i32, v8.clone(), v8.clone(), false)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
        next_request_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_next_request() -> (bool, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, bool) {
    closure0()()
}
