#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ListParsed(Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>),
    US1_ListRejected(Rc<str>),
}
fn command_line_argument_count_2() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn list_root_0() -> Rc<str> {
    let v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            v6
                        }
                        _ => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v8
                        }
                    }
                }
                _ => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10
                }
            }
        }
        _ => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        list_root_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn status_word_known_6(mut v0: Rc<str>) -> bool {
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
fn list_args_from_5(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<str>) -> US1 {
    loop {
        let v8: bool = v1 <= v0;
        if v8 {
            let v9: bool = agile_planning_filters::kit_agile_planning_filters_valid(&*v3, &*v4);
            if v9 {
                return US1::US1_ListParsed(v2, v3, v4, v5, v6, v7);
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid planning filter (expected --load light|medium|heavy|none and --assignee agent|pair|user|none)"); } LIT.with(|lit| lit.clone()) };
                return US1::US1_ListRejected(v11);
            }
        } else {
            let v14: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v15: bool = &*v14 == "--json";
            if v15 {
                let v16: i32 = v0.wrapping_add(1i32);
                let v17: bool = true;
                (v0, v5) = (v16, v17);
                continue;
            } else {
                let v19: bool = &*v14 == "--load";
                if v19 {
                    let v20: i32 = v0.wrapping_add(2i32);
                    let v21: i32 = v0.wrapping_add(1i32);
                    let v22: Rc<str> = usize::try_from(v21).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    (v0, v3) = (v20, v22);
                    continue;
                } else {
                    let v24: bool = &*v14 == "--assignee";
                    if v24 {
                        let v25: i32 = v0.wrapping_add(2i32);
                        let v26: i32 = v0.wrapping_add(1i32);
                        let v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        (v0, v4) = (v25, v27);
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
                                (v0, v7) = (v37, v31);
                                continue;
                            } else {
                                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sort expects priority|difficulty|progress, got "); } LIT.with(|lit| lit.clone()) }, v31));
                                return US1::US1_ListRejected(v39);
                            }
                        } else {
                            let v42: i32 = v0.wrapping_add(1i32);
                            let v43: Rc<str> = usize::try_from(v42).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                            let v44: i32 = agile_task_query::kit_query_flag_width(&*v14, &*v43);
                            let v45: bool = v44 > 0i32;
                            if v45 {
                                let v46: i32 = v0.wrapping_add(v44);
                                let v47: bool = v44 == 2i32;
                                let v49: Rc<str> = if v47 {
                                    v43
                                } else {
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v48
                                };
                                let v50: bool = &*v6 == "";
                                let v53: Rc<str> = if v50 {
                                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v51
                                } else {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v52
                                };
                                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v53));
                                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v14));
                                let v56: bool = &*v49 == "";
                                let v59: Rc<str> = if v56 {
                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    v57
                                } else {
                                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                    v58
                                };
                                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v59));
                                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v49));
                                (v0, v6) = (v46, v61);
                                continue;
                            } else {
                                let v63: bool = v44 < 0i32;
                                if v63 {
                                    let v64: Rc<str> = agile_task_query::kit_query_flag_refusal(&*v14, &*v43);
                                    return US1::US1_ListRejected(v64);
                                } else {
                                    let v66: bool = status_word_known_6(v14.clone());
                                    if v66 {
                                        (v0, v2) = (v42, v14);
                                        continue;
                                    } else {
                                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid agile status filter: "); } LIT.with(|lit| lit.clone()) }, v14));
                                        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected all, Planned, Active, Blocked, Paused or Done; or "); } LIT.with(|lit| lit.clone()) }));
                                        let v70: Rc<str> = agile_task_query::kit_query_usage();
                                        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                                        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", --sort priority|difficulty|progress)"); } LIT.with(|lit| lit.clone()) }));
                                        return US1::US1_ListRejected(v72);
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
fn list_request_4() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
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
    let v10: US1 = list_args_from_5(v0, v5, v6.clone(), v6.clone(), v6, v7, v8.clone(), v9);
    match v10 {
        US1::US1_ListParsed(v11, v12, v13, v14, v15, v16) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = v12;
            let v13: Rc<str> = v13;
            let v14: bool = v14;
            let v15: Rc<str> = v15;
            let v16: Rc<str> = v16;
            (true, v8, v11, v12, v13, v14, v15, v16)
        }
        US1::US1_ListRejected(v17) => {
            let v17: Rc<str> = v17;
            (false, v17, v8.clone(), v8.clone(), v8.clone(), false, v8.clone(), v8)
        }
    }
}
fn closure1() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
        list_request_4()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_list_root() -> Rc<str> {
    closure0()()
}
pub fn kit_agile_list_request() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) {
    closure1()()
}
