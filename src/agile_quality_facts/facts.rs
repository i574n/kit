#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_TaskLine(Rc<str>, Rc<str>, Rc<str>),
    US0_PlanningLine(Rc<str>, i32, Rc<str>, Rc<str>),
    US0_NoteLine(Rc<str>, Rc<str>),
    US0_OtherLine,
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method7(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method7(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method6(v16, v17)
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method8(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let mut v7: i32 = method9(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method10(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH0::UH0_Cons(v12, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let mut v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planning_line_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = v0.borrow()[2i32 as usize].clone();
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v2.replace(&*v3, &*v4));
    let v6: Rc<UH0> = split_list_5(v1, v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = method8(v6, v7);
    match Rc::unwrap_or_clone(v8) {
        UH0::UH0_Cons(v9, v10) => {
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH0> = v10;
            match Rc::unwrap_or_clone(v10) {
                UH0::UH0_Cons(v11, v12) => {
                    let v11: Rc<str> = v11;
                    let v12: Rc<UH0> = v12;
                    match Rc::unwrap_or_clone(v12) {
                        UH0::UH0_Cons(v13, v14) => {
                            let v13: Rc<str> = v13;
                            let v14: Rc<UH0> = v14;
                            match Rc::unwrap_or_clone(v14) {
                                UH0::UH0_Cons(v15, v16) => {
                                    let v15: Rc<str> = v15;
                                    let v16: Rc<UH0> = v16;
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Minutes "); } LIT.with(|lit| lit.clone()) };
                                    let v18: bool = v11.starts_with(&*v17);
                                    let v20: Rc<str> = if v18 {
                                        let v19: Rc<str> = Rc::<str>::from(v11.replace(&*v17, &*v4));
                                        v19
                                    } else {
                                        v4.clone()
                                    };
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                                    let v22: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v4));
                                    let v23: Option<i32> = (v22).trim().parse::<i32>().ok();
                                    let v24: Option<i32> = method11(v23);
                                    let mut v25: Rc<dyn Fn((i32)) -> US1> = closure1();
                                    let mut v26: Option<US1> = v24.map(|x| v25(x));
                                    let v28: US1 = US1::US1_None;
                                    let v29: US1 = v28.clone();
                                    let v30: US1 = v26.unwrap_or(v29);
                                    let v33: i32 = match v30 {
                                        US1::US1_None => {
                                            0i32
                                        }
                                        US1::US1_Some(v31) => {
                                            let v31: i32 = v31;
                                            v31
                                        }
                                    };
                                    let v34: Rc<str> = v0.borrow()[1i32 as usize].clone();
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load "); } LIT.with(|lit| lit.clone()) };
                                    let v36: bool = v13.starts_with(&*v35);
                                    let v38: Rc<str> = if v36 {
                                        let v37: Rc<str> = Rc::<str>::from(v13.replace(&*v35, &*v4));
                                        v37
                                    } else {
                                        v4.clone()
                                    };
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assigned "); } LIT.with(|lit| lit.clone()) };
                                    let v40: bool = v15.starts_with(&*v39);
                                    let v42: Rc<str> = if v40 {
                                        let v41: Rc<str> = Rc::<str>::from(v15.replace(&*v39, &*v4));
                                        v41
                                    } else {
                                        v4
                                    };
                                    US0::US0_PlanningLine(v34, v33, v38, v42)
                                }
                                _ => {
                                    US0::US0_OtherLine
                                }
                            }
                        }
                        _ => {
                            US0::US0_OtherLine
                        }
                    }
                }
                _ => {
                    US0::US0_OtherLine
                }
            }
        }
        _ => {
            US0::US0_OtherLine
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method13(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                (v0, v1) = (v3, v2);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn task_line_12(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> US0 {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = v1.wrapping_sub(9i32);
    let v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let v5: Rc<UH0> = split_list_5(v2, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method8(v5, v6);
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method13(v7, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method14(v9, v10);
    let v12: Rc<str> = v0.borrow()[1i32 as usize].clone();
    let v13: i32 = v1.wrapping_sub(6i32);
    let v14: Rc<str> = v0.borrow()[v13 as usize].clone();
    US0::US0_TaskLine(v12, v11, v14)
}
fn state_line_of_3(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 3i32;
    if v4 {
        US0::US0_OtherLine
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
        let mut v7: bool = v0.starts_with(&*v6);
        if v7 {
            let mut v8: Rc<str> = v2.borrow()[1i32 as usize].clone();
            US0::US0_NoteLine(v8, v0)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
            let v11: bool = v0.starts_with(&*v10);
            if v11 {
                planning_line_4(v2)
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v14: bool = v0.starts_with(&*v13);
                let v17: bool = if v14 {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("() = Task ("); } LIT.with(|lit| lit.clone()) };
                    let v16: bool = v0.contains(&*v15);
                    v16
                } else {
                    false
                };
                let v19: bool = if v17 {
                    let v18: bool = v3 >= 10i32;
                    v18
                } else {
                    false
                };
                if v19 {
                    task_line_12(v2)
                } else {
                    US0::US0_OtherLine
                }
            }
        }
    }
}
fn method16(mut v0: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>) -> Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> {
    v0
}
fn closure2() -> Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> = Rc::new(move |mut v0: ((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))| -> US2 {
        let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = (v0);
        US2::US2_Some(v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folded_15(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: US0) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> {
    match v1 {
        US0::US0_NoteLine(v75, v76) => {
            let v75: Rc<str> = v75;
            let v76: Rc<str> = v76;
            let v77: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v75).cloned();
            let v78: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = method16(v77);
            let mut v79: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v80: Option<US2> = v78.map(|x| v79(x));
            let v82: US2 = US2::US2_None;
            let v83: US2 = v82.clone();
            let v84: US2 = v80.unwrap_or(v83);
            let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = match &v84 {
                US2::US2_Some(v85, v86, v87, v88, v89, v90) => {
                    let v85: Rc<str> = v85.clone();
                    let v86: Rc<str> = v86.clone();
                    let v87: i32 = *v87;
                    let v88: Rc<str> = v88.clone();
                    let v89: Rc<str> = v89.clone();
                    let v90: Rc<str> = v90.clone();
                    (v85, v86, v87, v88, v89, v90)
                }
                _ => {
                    let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = match v84 {
                        US2::US2_None => {
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v91.clone(), v91.clone(), 0i32, v91.clone(), v91.clone(), v91.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v92, v93, v94, v95, v96, v97)
                }
            };
            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v76));
            let mut v106: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v108: bool = true; // 1;
            let mut v106 = v106;
            let v110: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v106.insert(v75, (v98, v99, v100, v101, v102, v105));
            let v111: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v106);
            v111
        }
        US0::US0_OtherLine => {
            v0
        }
        US0::US0_PlanningLine(v38, v39, v40, v41) => {
            let v38: Rc<str> = v38;
            let v39: i32 = v39;
            let v40: Rc<str> = v40;
            let v41: Rc<str> = v41;
            let v42: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v38).cloned();
            let v43: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = method16(v42);
            let mut v44: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v45: Option<US2> = v43.map(|x| v44(x));
            let v47: US2 = US2::US2_None;
            let v48: US2 = v47.clone();
            let v49: US2 = v45.unwrap_or(v48);
            let (mut v63, mut v64, mut v68): (Rc<str>, Rc<str>, Rc<str>) = match &v49 {
                US2::US2_Some(v50, v51, v52, v53, v54, v55) => {
                    let v50: Rc<str> = v50.clone();
                    let v51: Rc<str> = v51.clone();
                    let v52: i32 = *v52;
                    let v53: Rc<str> = v53.clone();
                    let v54: Rc<str> = v54.clone();
                    let v55: Rc<str> = v55.clone();
                    (v50, v51, v55)
                }
                _ => {
                    let (mut v57, mut v58, mut v62): (Rc<str>, Rc<str>, Rc<str>) = match v49 {
                        US2::US2_None => {
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v56.clone(), v56.clone(), v56.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v57, v58, v62)
                }
            };
            let mut v69: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v71: bool = true; // 1;
            let mut v69 = v69;
            let v73: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v69.insert(v38, (v63, v64, v39, v40, v41, v68));
            let v74: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v69);
            v74
        }
        US0::US0_TaskLine(v2, v3, v4) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v2).cloned();
            let v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = method16(v5);
            let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
            let mut v8: Option<US2> = v6.map(|x| v7(x));
            let v10: US2 = US2::US2_None;
            let v11: US2 = v10.clone();
            let v12: US2 = v8.unwrap_or(v11);
            let (mut v28, mut v29, mut v30, mut v31): (i32, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                US2::US2_Some(v13, v14, v15, v16, v17, v18) => {
                    let v13: Rc<str> = v13.clone();
                    let v14: Rc<str> = v14.clone();
                    let v15: i32 = *v15;
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = v17.clone();
                    let v18: Rc<str> = v18.clone();
                    (v15, v16, v17, v18)
                }
                _ => {
                    let (mut v22, mut v23, mut v24, mut v25): (i32, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                        US2::US2_None => {
                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (0i32, v19.clone(), v19.clone(), v19.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v22, v23, v24, v25)
                }
            };
            let mut v32: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = std::rc::Rc::unwrap_or_clone(v0);
            let v34: bool = true; // 1;
            let mut v32 = v32;
            let v36: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v32.insert(v2, (v3, v4, v28, v29, v30, v31));
            let v37: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v32);
            v37
        }
    }
}
fn method2(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: US0 = state_line_of_3(v2);
                let v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = folded_15(v1, v4);
                (v0, v1) = (v3, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn facts_of_1(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH0> = split_list_5(v1, v4);
    let mut v7: std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = std::collections::BTreeMap::new();
    let mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = std::rc::Rc::new(v7);
    method2(v5, v8)
}
fn method17(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = v0.get(&v3).cloned();
                let v6: Option<(Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)> = method16(v5);
                let mut v7: Rc<dyn Fn(((Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>))) -> US2> = closure2();
                let mut v8: Option<US2> = v6.map(|x| v7(x));
                let v10: US2 = US2::US2_None;
                let v11: US2 = v10.clone();
                let v12: US2 = v8.unwrap_or(v11);
                let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                    US2::US2_Some(v13, v14, v15, v16, v17, v18) => {
                        let v13: Rc<str> = v13.clone();
                        let v14: Rc<str> = v14.clone();
                        let v15: i32 = *v15;
                        let v16: Rc<str> = v16.clone();
                        let v17: Rc<str> = v17.clone();
                        let v18: Rc<str> = v18.clone();
                        (v13, v14, v15, v16, v17, v18)
                    }
                    _ => {
                        let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                            US2::US2_None => {
                                std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                            }
                            _ => unreachable!(),
                        };
                        (v25, v26, v27, v28, v29, v30)
                    }
                };
                let v37: bool = &*v31 == "";
                let v39: Rc<UH1> = if v37 {
                    v2
                } else {
                    Rc::new(UH1::UH1_Cons(v3, v31, v32, v33, v34, v35, v36, v2))
                };
                (v1, v2) = (v4, v39);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let v6: Rc<str> = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<UH1> = v9;
                let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v1));
                (v0, v1) = (v9, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<UH1> = v10;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9));
                let v11: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v10, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn fact_rows_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, (Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = facts_of_1(v0);
    let mut v2: Vec<Rc<str>> = v1.keys().cloned().collect::<Vec<_>>();
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: bool = true; let _vec_fold_ = v2.into_iter().fold(v3, |acc, x| { //;
    let mut v7: Rc<UH0> = acc;
    let mut v9: Rc<str> = x;
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v7));
    let v12: bool = true; v11 });
    let v14: Rc<UH0> = _vec_fold_;
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = method6(v14, v15);
    let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH1> = method17(v1, v16, v17);
    let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH1> = method18(v18, v19);
    let v21: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v22: i32 = 0i32;
    let v23: i32 = method19(v21.clone(), v20, v22);
    let v24: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v21.borrow().clone());
    let mut v25: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v24).as_ref().clone()));
    v25
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> {
        fact_rows_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_fact_rows(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_652accfc6a8d = Mut0;
