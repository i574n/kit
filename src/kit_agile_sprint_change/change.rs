#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ItemOpen,
    US0_ItemDone,
    US0_ItemUnknown,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, i32, US0, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_SprintOpen,
    US3_SprintClosed,
}
#[derive(Clone)]
enum US4 {
    US4_Committed,
    US4_Dropped,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US4, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Started(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US3, Rc<UH1>),
    US2_AnotherSprintOpen(Rc<str>),
    US2_ItemNotOpen(Rc<str>),
    US2_ItemUnestimated(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
fn sprint_has_2(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let (mut v5, mut v6, mut v7): (Rc<str>, i32, bool) = v0.borrow()[v2 as usize].clone();
            let v8: bool = &*v5 == &*v1;
            if v8 {
                return true;
            } else {
                let v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn standing_of_4(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "";
    if v1 {
        US0::US0_ItemUnknown
    } else {
        let v3: bool = &*v0 == "Done";
        if v3 {
            US0::US0_ItemDone
        } else {
            US0::US0_ItemOpen
        }
    }
}
fn candidate_of_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, i32, US0) {
    let mut v2: Rc<str> = agile_task_census::kit_agile_task_status_of(&v0, &v1);
    let v3: i32 = agile_planning_fields::kit_agile_planning_minutes_of(&v0, &v1);
    let v4: US0 = standing_of_4(v2);
    (v1, v3, v4)
}
fn method6(mut v0: Rc<UH0>, mut v1: US1) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let v18: US1 = match &v1 {
                    US1::US1_None => {
                        let v7: bool = match v4 {
                            US0::US0_ItemOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v8: bool = v7 != true;
                        if v8 {
                            let v9: US2 = US2::US2_ItemNotOpen(v2);
                            US1::US1_Some(v9)
                        } else {
                            let v11: bool = v3 <= 0i32;
                            if v11 {
                                let v12: US2 = US2::US2_ItemUnestimated(v2);
                                US1::US1_Some(v12)
                            } else {
                                US1::US1_None
                            }
                        }
                    }
                    US1::US1_Some(v6) => {
                        let v6: US2 = v6.clone();
                        v1
                    }
                };
                (v0, v1) = (v5, v18);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn first_refusal_5(mut v0: Rc<UH0>) -> US1 {
    let v1: US1 = US1::US1_None;
    method6(v0, v1)
}
fn sprint_changed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> i32 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/sprints.spi"); } LIT.with(|lit| lit.clone()) }));
    let v9: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v8).unwrap_or_default().as_str());
    let v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_head(&v9, &v1);
    let v13: Rc<str> = if v3 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("drop"); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        v12
    };
    let v14: i32 = (v10.clone().borrow().len() as i32);
    let v15: bool = v14 == 0i32;
    if v15 {
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no sprint "); } LIT.with(|lit| lit.clone()) }, v1));
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint "); } LIT.with(|lit| lit.clone()) }, v13));
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused: "); } LIT.with(|lit| lit.clone()) }));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v16));
        eprintln!("{}", v19);
        1i32
    } else {
        let (mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v10.borrow()[0i32 as usize].clone();
        let v25: bool = v24 == false;
        if v25 {
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint "); } LIT.with(|lit| lit.clone()) }, v1));
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is closed"); } LIT.with(|lit| lit.clone()) }));
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint "); } LIT.with(|lit| lit.clone()) }, v13));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused: "); } LIT.with(|lit| lit.clone()) }));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v27));
            eprintln!("{}", v30);
            1i32
        } else {
            let v31: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_items(&v9, &v1);
            let v32: i32 = 0i32;
            let v33: bool = sprint_has_2(&v31, v2.clone(), v32);
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
            let v35: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v34).unwrap_or_default().as_str());
            let (mut v36, mut v37, mut v38): (Rc<str>, i32, US0) = candidate_of_3(v35, v2.clone());
            let v59: Rc<str> = if v3 {
                if v33 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v39
                } else {
                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item "); } LIT.with(|lit| lit.clone()) }, v2.clone()));
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not in sprint "); } LIT.with(|lit| lit.clone()) }));
                    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v1.clone()));
                    v42
                }
            } else {
                let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36, v37, v38, v44));
                let v46: US1 = first_refusal_5(v45);
                match v46 {
                    US1::US1_Some(v47) => {
                        let v47: US2 = v47;
                        match v47 {
                            US2::US2_ItemNotOpen(v48) => {
                                let v48: Rc<str> = v48;
                                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item "); } LIT.with(|lit| lit.clone()) }, v2.clone()));
                                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is unknown or Done"); } LIT.with(|lit| lit.clone()) }));
                                v50
                            }
                            US2::US2_ItemUnestimated(v51) => {
                                let v51: Rc<str> = v51;
                                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("item "); } LIT.with(|lit| lit.clone()) }, v2.clone()));
                                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" has no estimate"); } LIT.with(|lit| lit.clone()) }));
                                v53
                            }
                            _ => {
                                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v54
                            }
                        }
                    }
                    _ => {
                        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v57
                    }
                }
            };
            let v60: bool = &*v59 != "";
            if v60 {
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint "); } LIT.with(|lit| lit.clone()) }, v13));
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused: "); } LIT.with(|lit| lit.clone()) }));
                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v59));
                eprintln!("{}", v63);
                1i32
            } else {
                let v64: Rc<str> = kit_agile_sprint_append::kit_agile_sprint_item_appended(&v9, &v1, &v2, v37, v3);
                std::fs::write(&*v8, &*v64).unwrap();
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile sprint "); } LIT.with(|lit| lit.clone()) }, v13));
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok id="); } LIT.with(|lit| lit.clone()) }));
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v1));
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" item="); } LIT.with(|lit| lit.clone()) }));
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v2));
                println!("{}", v70);
                0i32
            }
        }
    }
}
fn changed_0(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    sprint_changed_1(v3, v2, v1, v0)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool| -> i32 {
        changed_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn finished_from_9(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>, mut v2: i32, mut v3: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        let v4: i32 = (v1.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, i32, bool) = v1.borrow()[v2 as usize].clone();
            let v9: Rc<str> = agile_task_census::kit_agile_task_status_of(&v0, &v6);
            let v10: bool = &*v9 == "Done";
            let v11: i32 = v2.wrapping_add(1i32);
            let v26: Rc<RefCell<Vec<Rc<str>>>> = if v10 {
                let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v6]));
                let v13: i32 = (v3.clone().borrow().len() as i32);
                let v14: i32 = (v12.clone().borrow().len() as i32);
                let v15: i32 = v13.wrapping_add(v14);
                let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v15 as usize]));
                let mut v17: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method10(v15, &v17) {
                    let v19: i32 = v17.borrow().l0;
                    let v20: bool = v19 < v13;
                    let v24: Rc<str> = if v20 {
                        let v21: Rc<str> = v3.borrow()[v19 as usize].clone();
                        v21
                    } else {
                        let v22: i32 = v19.wrapping_sub(v13);
                        let v23: Rc<str> = v12.borrow()[v22 as usize].clone();
                        v23
                    };
                    v16.borrow_mut()[v19 as usize] = v24;
                    let v25: i32 = v19.wrapping_add(1i32);
                    v17.borrow_mut().l0 = v25;
                    ()
                };
                v16
            } else {
                v3
            };
            (v2, v3) = (v11, v26);
            continue;
        }
    }
}
fn method12(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_11(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method12(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn sprint_closed_8(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/sprints.spi"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v6).unwrap_or_default().as_str());
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_head(&v7, &v1);
    let v9: i32 = (v8.clone().borrow().len() as i32);
    let v10: bool = v9 == 0i32;
    let mut v17: bool = if v10 {
        true
    } else {
        let (mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool) = v8.borrow()[0i32 as usize].clone();
        let mut v16: bool = v15 == false;
        v16
    };
    if v17 {
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: agile sprint close refused: no open sprint "); } LIT.with(|lit| lit.clone()) }, v1));
        eprintln!("{}", v18);
        1i32
    } else {
        let v19: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = kit_agile_sprint_state::kit_agile_sprint_items(&v7, &v1);
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/agile.spi"); } LIT.with(|lit| lit.clone()) }));
        let v21: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v20).unwrap_or_default().as_str());
        let v22: i32 = 0i32;
        let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
        let v24: Rc<RefCell<Vec<Rc<str>>>> = finished_from_9(v21, &v19, v22, v23);
        let v25: Rc<RefCell<Vec<(i32, i32, i32)>>> = kit_agile_sprint_review::kit_agile_sprint_review(&v7, &v1, v24.clone());
        let v26: Rc<str> = kit_agile_sprint_review::kit_agile_sprint_closed(&v7, &v1, v24);
        std::fs::write(&*v6, &*v26).unwrap();
        let (mut v28, mut v29, mut v30): (i32, i32, i32) = v25.borrow()[0i32 as usize].clone();
        let v31: Rc<str> = format_real_11(v28);
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done="); } LIT.with(|lit| lit.clone()) }, v31));
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m carried="); } LIT.with(|lit| lit.clone()) }));
        let v34: Rc<str> = format_real_11(v29);
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m dropped="); } LIT.with(|lit| lit.clone()) }));
        let v37: Rc<str> = format_real_11(v30);
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
        let v40: bool = v29 == 0i32;
        let v43: Rc<str> = if v40 {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("yes"); } LIT.with(|lit| lit.clone()) };
            v41
        } else {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no"); } LIT.with(|lit| lit.clone()) };
            v42
        };
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile sprint close ok id="); } LIT.with(|lit| lit.clone()) }, v1));
        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v39));
        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" goal_met="); } LIT.with(|lit| lit.clone()) }));
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v43));
        println!("{}", v47);
        0i32
    }
}
fn closed_7(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    sprint_closed_8(v1, v0)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        closed_7(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn sprint_velocity_14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/sprints.spi"); } LIT.with(|lit| lit.clone()) }));
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
    let v7: i32 = kit_agile_sprint_state::kit_agile_sprint_velocity(&v6, v1);
    let v8: Rc<str> = format_real_11(v1);
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile sprint velocity ok last="); } LIT.with(|lit| lit.clone()) }, v8));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" minutes="); } LIT.with(|lit| lit.clone()) }));
    let v11: Rc<str> = format_real_11(v7);
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    println!("{}", v12);
    0i32
}
fn velocity_shown_13(mut v0: i32, mut v1: Rc<str>) -> i32 {
    sprint_velocity_14(v1, v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        velocity_shown_13(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_sprint_changed(v0: &str, v1: &str, v2: &str, v3: bool) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
pub fn kit_agile_sprint_closed_run(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_sprint_velocity_shown(v0: &str, v1: i32) -> i32 {
    closure2()(Rc::<str>::from(v0), v1)
}
use kit_agile_sprint_state::SpiralShape_2617b4e49837 as Mut0;
use agile_task_census::SpiralShape_97df63ebabc3 as Mut1;
