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
    US0_MacroLong,
    US0_MacroBlock,
    US0_MacroFlow,
    US0_MacroCast,
    US0_MacroArith,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<str>, US0, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US0),
    US1_None,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn lint_args_2() -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) {
    let mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>)>>> = kit_lint_args::kit_lint_args_read(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = v0.borrow()[0i32 as usize].clone();
    (v1, v2, v3, v4, v5, v6, v7)
}
fn file_exists_3(mut v0: Rc<str>) -> bool {
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
fn method5(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method5(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn listed_from_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v1, v2) = (v4, v6.clone());
            continue;
        }
    }
}
fn listed_paths_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    listed_from_9(v0, v2, v3)
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let v5: Rc<UH0> = method11(v0.clone(), v4, v2.clone());
            let mut v6: Rc<str> = method4(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method12(v3, v1.clone());
            let v5: bool = file_exists_3(v2.clone());
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method13(v3, v1.clone());
            let v5: bool = file_exists_3(v2.clone());
            let mut v6: bool = v5 == false;
            if v6 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn walked_from_16(mut v0: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v1: std::rc::Rc<Vec<std::rc::Rc<str>>> = kit_lint_walk::kit_lint_walk_files(&*v0);
    v1
}
fn listed_paths_17(mut v0: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: usize = ((v1) as usize);
            let mut v6: Rc<str> = v0[v5].clone();
            let mut v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v2.clone()));
            (v1, v2) = (v4, v7.clone());
            continue;
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method18(v3, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: std::rc::Rc<Vec<std::rc::Rc<str>>> = walked_from_16(v2.clone());
                let v5: usize = v4.len();
                let mut v6: i32 = ((v5) as i32);
                let mut v7: i32 = v6.wrapping_sub(1i32);
                let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v9: Rc<UH0> = listed_paths_17(v4.clone(), v7, v8.clone());
                let v10: Rc<UH0> = method18(v1.clone(), v9.clone());
                (v0, v1) = (v3.clone(), v10.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn files_from_14(mut v0: Rc<UH0>) -> Rc<UH0> {
    let v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method15(v0, v1)
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method20(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn added_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_lint_scan::kit_lint_added_text(&*v0);
    v1
}
fn relative_to_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
    let mut v6: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v7: i32 = (v5.clone().len() as i32);
    let v8: bool = v6.starts_with(&*v5);
    if v8 {
        let mut v9: i32 = (v6.clone().len() as i32);
        let v10: i32 = v9.wrapping_sub(v7);
        let v11: usize = ((v7) as usize);
        let v12: i32 = v7.wrapping_add(v10);
        let v13: usize = ((v12) as usize);
        let mut v14: Rc<str> = Rc::<str>::from(&(v6)[v11..v13]);
        v14
    } else {
        v6
    }
}
fn scan_of_25(mut v0: Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6): (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = kit_lint_scan::kit_lint_scan(&*v0);
    (v1, v2, v3, v4, v5, v6)
}
fn method27(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<str> = v4.clone();
                let v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v1.clone()));
                (v0, v1) = (v7.clone(), v8.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn flow_at_29(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_lint_flow::kit_lint_flow(&*v0);
    v1
}
fn construct_at_30(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_lint_flow_constructs::kit_lint_flow_construct(&*v0);
    v1
}
fn idiom_at_31(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_lint_flow_idioms::kit_lint_flow_idiom(&*v0);
    v1
}
fn flow_candidate_at_28(mut v0: Rc<str>) -> bool {
    let v1: bool = flow_at_29(v0.clone());
    let v4: bool = if v1 {
        let v2: bool = construct_at_30(v0.clone());
        let v3: bool = v2 == false;
        v3
    } else {
        false
    };
    if v4 {
        let v5: bool = idiom_at_31(v0);
        let mut v6: bool = v5 == false;
        v6
    } else {
        false
    }
}
fn class_at_32(mut v0: Rc<str>, mut v1: bool, mut v2: bool, mut v3: i32, mut v4: i32) -> i32 {
    let v5: i32 = kit_lint_classify::kit_lint_class(&*v0, v1, v2, v3, v4);
    v5
}
fn walk_26(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: Rc<RefCell<Vec<i32>>>, mut v3: Rc<RefCell<Vec<bool>>>, mut v4: Rc<RefCell<Vec<Rc<str>>>>, mut v5: Rc<RefCell<Vec<Rc<str>>>>, mut v6: Rc<RefCell<Vec<i32>>>, mut v7: i32, mut v8: Rc<RefCell<Vec<Rc<str>>>>, mut v9: i32, mut v10: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v11: bool = v9 >= v7;
        if v11 {
            let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method27(v10.clone(), v12.clone());
        } else {
            let mut v14: Rc<str> = v4.borrow()[v9 as usize].clone();
            let v15: i32 = v6.borrow()[v9 as usize];
            let v16: bool = v15 >= 1i32;
            let v19: bool = if v16 {
                let v17: i32 = (v8.clone().borrow().len() as i32);
                let v18: bool = v15 <= v17;
                v18
            } else {
                false
            };
            let v23: Rc<str> = if v19 {
                let v20: i32 = v15.wrapping_sub(1i32);
                let v21: Rc<str> = v8.borrow()[v20 as usize].clone();
                v21.clone()
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22.clone()
            };
            let v24: bool = flow_candidate_at_28(v23.clone());
            let v25: bool = v3.borrow()[v9 as usize];
            let v26: i32 = v2.borrow()[v9 as usize];
            let v27: i32 = v1.borrow()[v9 as usize];
            let v28: i32 = class_at_32(v14.clone(), v25, v24, v26, v27);
            let v29: bool = v28 == 0i32;
            let v49: US1 = if v29 {
                let v30: US0 = US0::US0_MacroLong;
                US1::US1_Some(v30.clone())
            } else {
                let v32: bool = v28 == 1i32;
                if v32 {
                    let v33: US0 = US0::US0_MacroBlock;
                    US1::US1_Some(v33.clone())
                } else {
                    let v35: bool = v28 == 2i32;
                    if v35 {
                        let v36: US0 = US0::US0_MacroFlow;
                        US1::US1_Some(v36.clone())
                    } else {
                        let v38: bool = v28 == 3i32;
                        if v38 {
                            let v39: US0 = US0::US0_MacroCast;
                            US1::US1_Some(v39.clone())
                        } else {
                            let v41: bool = v28 == 4i32;
                            if v41 {
                                let v42: US0 = US0::US0_MacroArith;
                                US1::US1_Some(v42.clone())
                            } else {
                                US1::US1_None
                            }
                        }
                    }
                }
            };
            match &v49 {
                US1::US1_None => {
                    let v56: i32 = v9.wrapping_add(1i32);
                    v9 = v56;
                    continue;
                }
                US1::US1_Some(v50) => {
                    let v50: US0 = v50.clone();
                    let v51: i32 = v6.borrow()[v9 as usize];
                    let v52: Rc<str> = v5.borrow()[v9 as usize].clone();
                    let v53: i32 = v9.wrapping_add(1i32);
                    let v54: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0.clone(), v51, v52.clone(), v50.clone(), v14.clone(), v10.clone()));
                    (v9, v10) = (v53, v54.clone());
                    continue;
                }
            }
        }
    }
}
fn file_findings_24(mut v0: Rc<str>, mut v1: Rc<str>) -> (i32, Rc<UH1>) {
    let (mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = scan_of_25(v1.clone());
    let v8: i32 = (v2.clone().borrow().len() as i32);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v1.replace(&*v9, &*v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v14: i32 = 0i32;
    let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH1> = walk_26(v0.clone(), v7.clone(), v6.clone(), v5.clone(), v4.clone(), v3.clone(), v2.clone(), v8, v13.clone(), v14, v15.clone());
    (v8, v16.clone())
}
fn method33(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<str> = v4.clone();
                let v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3, v4.clone(), v5.clone(), v6.clone(), v1.clone()));
                (v0, v1) = (v7.clone(), v8.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<UH1>) -> (i32, Rc<UH1>) {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v4);
                let mut v9: Vec<u8> = v7.unwrap();
                let v10: Vec<u8> = method7(v9.clone());
                let v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
                let mut v14: std::string::String = v12.unwrap();
                let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".diff"); } LIT.with(|lit| lit.clone()) };
                let v18: bool = v4.ends_with(&*v17);
                let v21: bool = if v18 {
                    true
                } else {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".patch"); } LIT.with(|lit| lit.clone()) };
                    let v20: bool = v4.ends_with(&*v19);
                    v20
                };
                let v23: Rc<str> = if v21 {
                    added_22(v16.clone())
                } else {
                    v16.clone()
                };
                let v24: Rc<str> = relative_to_23(v0.clone(), v4.clone());
                let (mut v25, mut v26): (i32, Rc<UH1>) = file_findings_24(v24.clone(), v23.clone());
                let v27: i32 = v2.wrapping_add(v25);
                let v28: Rc<UH1> = method33(v26.clone(), v3.clone());
                (v1, v2, v3) = (v5.clone(), v27, v28.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3.clone());
            }
        }
    }
}
fn known_in_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> bool {
    let v10: i32 = match &v4 {
        US0::US0_MacroArith => {
            4i32
        }
        US0::US0_MacroBlock => {
            1i32
        }
        US0::US0_MacroCast => {
            3i32
        }
        US0::US0_MacroFlow => {
            2i32
        }
        US0::US0_MacroLong => {
            0i32
        }
    };
    let v11: bool = kit_lint_walk::kit_lint_known(&*v0, &*v1, &*v3, v10);
    v11
}
fn method34(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match &*v1 {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3.clone();
            let v4: i32 = *v4;
            let v5: Rc<str> = v5.clone();
            let mut v6: US0 = v6.clone();
            let mut v7: Rc<str> = v7.clone();
            let v8: Rc<UH1> = v8.clone();
            let mut v9: Rc<UH1> = method34(v0.clone(), v8, v2.clone());
            let v10: bool = known_in_35(v0, v3.clone(), v4, v5.clone(), v6.clone(), v7.clone());
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v7, v9.clone()))
            } else {
                v9
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn method37(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<str> = v4.clone();
                let v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: i32 = (v4.clone().len() as i32);
                let mut v9: bool = v8 > v1;
                let v10: i32 = if v9 {
                    v8
                } else {
                    v1
                };
                (v0, v1) = (v7.clone(), v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn name_width_36(mut v0: Rc<UH1>) -> i32 {
    let v1: i32 = 0i32;
    method37(v0, v1)
}
fn method38(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let v3: i32 = *v3;
                let v4: Rc<str> = v4.clone();
                let v5: US0 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<UH1> = v7.clone();
                let v8: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v7.clone(), v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method39(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn tsv_row_41(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>) -> Rc<str> {
    let mut v9: i32 = match &v3 {
        US0::US0_MacroArith => {
            4i32
        }
        US0::US0_MacroBlock => {
            1i32
        }
        US0::US0_MacroCast => {
            3i32
        }
        US0::US0_MacroFlow => {
            2i32
        }
        US0::US0_MacroLong => {
            0i32
        }
    };
    let v10: Rc<str> = kit_lint_classify::kit_lint_tsv_row(&*v0, v1, &*v2, v9, &*v4);
    v10
}
fn row_of_42(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US0, mut v5: Rc<str>) -> Rc<str> {
    let v10: i32 = match &v4 {
        US0::US0_MacroArith => {
            4i32
        }
        US0::US0_MacroBlock => {
            1i32
        }
        US0::US0_MacroCast => {
            3i32
        }
        US0::US0_MacroFlow => {
            2i32
        }
        US0::US0_MacroLong => {
            0i32
        }
    };
    let v11: Rc<str> = kit_lint_scan::kit_lint_row(v0, &*v1, v2, &*v3, v10, &*v5);
    v11
}
fn method40(mut v0: bool, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match &*v2 {
        UH1::UH1_Cons(v4, v5, v6, v7, v8, v9) => {
            let v4: Rc<str> = v4.clone();
            let v5: i32 = *v5;
            let mut v6: Rc<str> = v6.clone();
            let mut v7: US0 = v7.clone();
            let v8: Rc<str> = v8.clone();
            let mut v9: Rc<UH1> = v9.clone();
            let v10: Rc<UH0> = method40(v0, v1, v9, v3.clone());
            let v13: Rc<str> = if v0 {
                tsv_row_41(v4.clone(), v5, v6.clone(), v7.clone(), v8.clone())
            } else {
                row_of_42(v1, v4, v5, v6, v7, v8)
            };
            Rc::new(UH0::UH0_Cons(v13, v10))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn lint_report_10(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: bool) -> (Rc<UH0>, i32, i32, i32, i32) {
    let v8: Rc<UH0> = match &*v1 {
        UH0::UH0_Nil => {
            let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v0.clone(), v4))
        }
        _ => {
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method11(v0.clone(), v1, v6)
        }
    };
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method12(v8.clone(), v9);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = method13(v8, v11);
    let v13: Rc<UH0> = files_from_14(v12);
    let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = method19(v10, v14);
    let v16: Rc<UH0> = method20(v13, v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH0> = method19(v16, v17);
    let v19: i32 = 0i32;
    let v20: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v21, mut v22): (i32, Rc<UH1>) = method21(v0, v18.clone(), v19, v20);
    let v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v24: Rc<UH1> = method27(v22, v23);
    let v25: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v26: Rc<UH1> = method34(v2, v24.clone(), v25);
    let v27: i32 = name_width_36(v26.clone());
    let v28: i32 = 0i32;
    let v29: i32 = method38(v24, v28);
    let v30: i32 = 0i32;
    let v31: i32 = method39(v18, v30);
    let v32: i32 = 0i32;
    let v33: i32 = method38(v26.clone(), v32);
    let v34: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH0> = method40(v3, v27, v26, v34);
    let v38: Rc<UH0> = if v3 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file\tline\tname\tclass\ttext"); } LIT.with(|lit| lit.clone()) };
        Rc::new(UH0::UH0_Cons(v36, v35.clone()))
    } else {
        v35
    };
    (v38, v31, v21, v29, v33)
}
fn method43(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
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
fn method44(mut v0: Rc<UH0>) -> () {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1.clone();
                let v2: Rc<UH0> = v2.clone();
                println!("{}", v1);
                v0 = v2.clone();
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn elapsed_text_45(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_47() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_46() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_47();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn tally_text_48(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = kit_lint_scan::kit_lint_tally(v0, v1, v2, v3);
    v4
}
fn lint_run_0() -> i32 {
    let mut v0: u64 = now_ms_1();
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, bool, Rc<RefCell<Vec<Rc<str>>>>) = lint_args_2();
    let v8: bool = &*v2 == "";
    let v10: bool = if v8 {
        true
    } else {
        file_exists_3(v2.clone())
    };
    let v12: Rc<str> = if v10 {
        v2.clone()
    } else {
        method4(v1.clone(), v2.clone())
    };
    let v13: bool = &*v1 == "";
    let v16: bool = if v13 {
        true
    } else {
        let mut v14: bool = directory_exists_6(v1.clone());
        let v15: bool = v14 == false;
        v15
    };
    if v16 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint usage: lint macros <root> [<path>...] [--baseline <rows|file.spi>] [--tsv]"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" [--write-baseline <file.spi>]"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint usage: lint macros <root> [<path>...] [--baseline <rows|file.spi>] [--tsv] [--write-baseline <file.spi>]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v19);
        2i32
    } else {
        let v20: bool = &*v12 != "";
        let v23: bool = if v20 {
            let v21: bool = file_exists_3(v12.clone());
            let v22: bool = v21 == false;
            v22
        } else {
            false
        };
        if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint macros: no baseline file at "); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v2));
            eprintln!("{}", v25);
            2i32
        } else {
            let v26: bool = &*v12 == "";
            let v39: Rc<str> = if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v27
            } else {
                let v29: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v12);
                let v31: Vec<u8> = v29.unwrap();
                let v32: Vec<u8> = method7(v31);
                let v34: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v32);
                let v36: std::string::String = v34.unwrap();
                let v38: Rc<str> = Rc::<str>::from(String::as_str(&v36));
                v38
            };
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            let v41: bool = v12.ends_with(&*v40);
            let v43: Rc<str> = if v41 {
                let v42: Rc<str> = kit_lint_baseline::kit_lint_baseline_rows(&*v39);
                v42
            } else {
                v39
            };
            let v44: bool = &*v3 != "";
            if v44 {
                let v45: Rc<UH0> = listed_paths_8(v7.clone());
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v47: bool = false;
                let (mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<UH0>, i32, i32, i32, i32) = lint_report_10(v1.clone(), v45, v46, v47);
                let v53: bool = &*v43 != "";
                let v61: Rc<str> = if v53 {
                    v43.clone()
                } else {
                    let v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v55: i32 = 0i32;
                    let v56: i32 = method43(v54.clone(), v48, v55);
                    let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                    let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v60: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v59));
                    v60
                };
                let v62: Rc<str> = kit_lint_baseline::kit_lint_baseline_write(&*v3, &*v61);
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit lint baseline written "); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v3));
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  rows "); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v64, v65)), v62));
                println!("{}", v67);
                0i32
            } else {
                let v68: Rc<UH0> = listed_paths_8(v7);
                let (mut v69, mut v70, mut v71, mut v72, mut v73): (Rc<UH0>, i32, i32, i32, i32) = lint_report_10(v1, v68, v43, v5);
                method44(v69);
                let v74: Rc<str> = elapsed_text_45(v0);
                let v75: Rc<str> = clock_text_46();
                let v76: Rc<str> = tally_text_48(v70, v71, v72, v73);
                let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v81: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v76, v77)), v74)), v77)), v75));
                println!("{}", v81);
                let v82: bool = v73 == 0i32;
                if v82 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lint_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn shadows_run_49() -> i32 {
    let mut v0: i32 = kit_lint_shadows::kit_lint_shadows_run();
    v0
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        shadows_run_49()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_run() -> i32 {
    closure0()()
}
pub fn kit_lint_shadows_routed() -> i32 {
    closure1()()
}
