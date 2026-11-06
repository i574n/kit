#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_SprintStart,
    US0_SprintShow,
    US0_SprintRetro,
    US0_SprintAdd,
    US0_SprintDrop,
    US0_SprintClose,
    US0_SprintVelocity,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US0),
    US3_None,
}
fn sprint_usage_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile usage: sprint start <root> <id> <days> <goal> [--items A,B] | sprint show|retro <root> [<id>] | sprint add|drop <root> <id> <item> | sprint close <root> <id> | sprint velocity <root> [--last n]"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v0);
    2i32
}
fn method3(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method4(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_ran_2(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    match v0 {
        US0::US0_SprintAdd => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v36, v37) => {
                    let v36: Rc<str> = v36;
                    let v37: Rc<UH0> = v37;
                    match Rc::unwrap_or_clone(v37) {
                        UH0::UH0_Cons(v38, v39) => {
                            let v38: Rc<str> = v38;
                            let v39: Rc<UH0> = v39;
                            match Rc::unwrap_or_clone(v39) {
                                UH0::UH0_Cons(v40, v41) => {
                                    let v40: Rc<str> = v40;
                                    let v41: Rc<UH0> = v41;
                                    let v42: i32 = kit_agile_sprint_change::kit_agile_sprint_changed(&v36, &v38, &v40, false);
                                    v42
                                }
                                _ => {
                                    sprint_usage_1()
                                }
                            }
                        }
                        _ => {
                            sprint_usage_1()
                        }
                    }
                }
                _ => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintClose => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v62, v63) => {
                    let v62: Rc<str> = v62;
                    let v63: Rc<UH0> = v63;
                    match Rc::unwrap_or_clone(v63) {
                        UH0::UH0_Cons(v64, v65) => {
                            let v64: Rc<str> = v64;
                            let v65: Rc<UH0> = v65;
                            let v66: i32 = kit_agile_sprint_change::kit_agile_sprint_closed_run(&v62, &v64);
                            v66
                        }
                        _ => {
                            sprint_usage_1()
                        }
                    }
                }
                _ => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintDrop => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v49, v50) => {
                    let v49: Rc<str> = v49;
                    let v50: Rc<UH0> = v50;
                    match Rc::unwrap_or_clone(v50) {
                        UH0::UH0_Cons(v51, v52) => {
                            let v51: Rc<str> = v51;
                            let v52: Rc<UH0> = v52;
                            match Rc::unwrap_or_clone(v52) {
                                UH0::UH0_Cons(v53, v54) => {
                                    let v53: Rc<str> = v53;
                                    let v54: Rc<UH0> = v54;
                                    let v55: i32 = kit_agile_sprint_change::kit_agile_sprint_changed(&v49, &v51, &v53, true);
                                    v55
                                }
                                _ => {
                                    sprint_usage_1()
                                }
                            }
                        }
                        _ => {
                            sprint_usage_1()
                        }
                    }
                }
                _ => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintRetro => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v71, v72) => {
                    let v71: Rc<str> = v71;
                    let v72: Rc<UH0> = v72;
                    match Rc::unwrap_or_clone(v72) {
                        UH0::UH0_Cons(v73, v74) => {
                            let v73: Rc<str> = v73;
                            let v74: Rc<UH0> = v74;
                            let v75: i32 = kit_agile_sprint_show::kit_agile_sprint_retro(&v71, &v73);
                            v75
                        }
                        UH0::UH0_Nil => {
                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v77: i32 = kit_agile_sprint_show::kit_agile_sprint_retro(&v71, &v76);
                            v77
                        }
                    }
                }
                UH0::UH0_Nil => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintShow => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v24, v25) => {
                    let v24: Rc<str> = v24;
                    let v25: Rc<UH0> = v25;
                    match Rc::unwrap_or_clone(v25) {
                        UH0::UH0_Cons(v26, v27) => {
                            let v26: Rc<str> = v26;
                            let v27: Rc<UH0> = v27;
                            let v28: i32 = kit_agile_sprint_show::kit_agile_sprint_shown(&v24, &v26);
                            v28
                        }
                        UH0::UH0_Nil => {
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v30: i32 = kit_agile_sprint_show::kit_agile_sprint_shown(&v24, &v29);
                            v30
                        }
                    }
                }
                UH0::UH0_Nil => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintStart => {
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
                                    match Rc::unwrap_or_clone(v7) {
                                        UH0::UH0_Cons(v8, v9) => {
                                            let v8: Rc<str> = v8;
                                            let v9: Rc<UH0> = v9;
                                            let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                            let v11: i32 = 0i32;
                                            let v12: i32 = method3(v10.clone(), v9, v11);
                                            let v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
                                            let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
                                            let v15: i32 = kit_agile_sprint_start::kit_agile_sprint_started(&v2, &v4, &v6, &v8, v14);
                                            v15
                                        }
                                        _ => {
                                            sprint_usage_1()
                                        }
                                    }
                                }
                                _ => {
                                    sprint_usage_1()
                                }
                            }
                        }
                        _ => {
                            sprint_usage_1()
                        }
                    }
                }
                _ => {
                    sprint_usage_1()
                }
            }
        }
        US0::US0_SprintVelocity => {
            match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Cons(v83, v84) => {
                    let v83: Rc<str> = v83;
                    let v84: Rc<UH0> = v84;
                    match Rc::unwrap_or_clone(v84) {
                        UH0::UH0_Cons(v85, v86) => {
                            let v85: Rc<str> = v85;
                            let v86: Rc<UH0> = v86;
                            match Rc::unwrap_or_clone(v86) {
                                UH0::UH0_Cons(v87, v88) => {
                                    let v87: Rc<str> = v87;
                                    let v88: Rc<UH0> = v88;
                                    let v89: bool = &*v85 == "--last";
                                    if v89 {
                                        let v90: Option<i32> = (v87).trim().parse::<i32>().ok();
                                        let v91: Option<i32> = method4(v90);
                                        let mut v92: Rc<dyn Fn((i32)) -> US1> = closure1();
                                        let mut v93: Option<US1> = v91.map(|x| v92(x));
                                        let v95: US1 = US1::US1_None;
                                        let v96: US1 = v95.clone();
                                        let v97: US1 = v93.unwrap_or(v96);
                                        let v100: i32 = match v97 {
                                            US1::US1_None => {
                                                3i32
                                            }
                                            US1::US1_Some(v98) => {
                                                let v98: i32 = v98;
                                                v98
                                            }
                                        };
                                        let v101: i32 = kit_agile_sprint_change::kit_agile_sprint_velocity_shown(&v83, v100);
                                        v101
                                    } else {
                                        let v102: i32 = kit_agile_sprint_change::kit_agile_sprint_velocity_shown(&v83, 3i32);
                                        v102
                                    }
                                }
                                _ => {
                                    let v104: i32 = kit_agile_sprint_change::kit_agile_sprint_velocity_shown(&v83, 3i32);
                                    v104
                                }
                            }
                        }
                        _ => {
                            let v106: i32 = kit_agile_sprint_change::kit_agile_sprint_velocity_shown(&v83, 3i32);
                            v106
                        }
                    }
                }
                UH0::UH0_Nil => {
                    sprint_usage_1()
                }
            }
        }
    }
}
fn command_line_argument_count_6() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_5(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_6();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_7(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_5(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn verb_named_8(mut v0: Rc<str>) -> US3 {
    let v1: bool = &*v0 == "start";
    if v1 {
        let v2: US0 = US0::US0_SprintStart;
        US3::US3_Some(v2)
    } else {
        let v4: bool = &*v0 == "show";
        if v4 {
            let v5: US0 = US0::US0_SprintShow;
            US3::US3_Some(v5)
        } else {
            let v7: bool = &*v0 == "retro";
            if v7 {
                let v8: US0 = US0::US0_SprintRetro;
                US3::US3_Some(v8)
            } else {
                let v10: bool = &*v0 == "add";
                if v10 {
                    let v11: US0 = US0::US0_SprintAdd;
                    US3::US3_Some(v11)
                } else {
                    let v13: bool = &*v0 == "drop";
                    if v13 {
                        let v14: US0 = US0::US0_SprintDrop;
                        US3::US3_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "close";
                        if v16 {
                            let v17: US0 = US0::US0_SprintClose;
                            US3::US3_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "velocity";
                            if v19 {
                                let v20: US0 = US0::US0_SprintVelocity;
                                US3::US3_Some(v20)
                            } else {
                                US3::US3_None
                            }
                        }
                    }
                }
            }
        }
    }
}
fn sprint_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_5(v0);
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
                            let v8: US3 = verb_named_8(v6);
                            match v8 {
                                US3::US3_None => {
                                    sprint_usage_1()
                                }
                                US3::US3_Some(v9) => {
                                    let v9: US0 = v9;
                                    sprint_ran_2(v9, v7)
                                }
                            }
                        }
                        _ => {
                            sprint_usage_1()
                        }
                    }
                }
                _ => {
                    sprint_usage_1()
                }
            }
        }
        _ => {
            sprint_usage_1()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        sprint_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_run() -> i32 {
    closure0()()
}
