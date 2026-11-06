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
    US0_TypeTwice,
    US0_ShadowedUnused,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, i32, US0, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
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
    let mut v4: std::string::String = String::from(v2);
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
    let mut v5: std::string::String = String::from(v3);
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
    let mut v4: std::string::String = String::from(v2);
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
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn listed_from_10(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn listed_paths_9(mut v0: &Rc<RefCell<Vec<Rc<str>>>>) -> Rc<UH0> {
    let v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    listed_from_10(v0, v2, v3)
}
fn method12(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            let mut v5: Rc<UH0> = method12(v0.clone(), v4, v2);
            let mut v6: Rc<str> = method4(v0, v3);
            Rc::new(UH0::UH0_Cons(v6, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method13(v3, v1);
            let mut v5: bool = file_exists_3(v2.clone());
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
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method14(v3, v1);
            let mut v5: bool = file_exists_3(v2.clone());
            let mut v6: bool = v5 == false;
            if v6 {
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
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn entry_names_17(mut v0: Rc<str>) -> Rc<UH0> {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method16(v18, v19)
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let v8: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = &*v3 == "target";
                    v7
                };
                let v10: bool = if v8 {
                    true
                } else {
                    let mut v9: bool = &*v3 == "node_modules";
                    v9
                };
                let v12: bool = if v10 {
                    true
                } else {
                    let v11: bool = &*v3 == "_build";
                    v11
                };
                let mut v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = &*v3 == "deps";
                    v13
                };
                let v16: bool = if v14 {
                    true
                } else {
                    let v15: bool = &*v3 == "bin";
                    v15
                };
                let v18: bool = if v16 {
                    true
                } else {
                    let v17: bool = &*v3 == "obj";
                    v17
                };
                let v21: Rc<UH0> = if v18 {
                    v2
                } else {
                    let v19: Rc<str> = method4(v0.clone(), v3);
                    Rc::new(UH0::UH0_Cons(v19, v2))
                };
                (v1, v2) = (v4, v21);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn spi_files_15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = file_exists_3(v4.clone());
                if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v8: bool = v4.ends_with(&*v7);
                    let v10: Rc<UH0> = if v8 {
                        Rc::new(UH0::UH0_Cons(v4, v1))
                    } else {
                        v1
                    };
                    (v0, v1) = (v5, v10);
                    continue;
                } else {
                    let v12: bool = directory_exists_6(v4.clone());
                    if v12 {
                        let v13: Rc<UH0> = entry_names_17(v4.clone());
                        let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v15: Rc<UH0> = method16(v13, v14);
                        let v16: Rc<UH0> = method18(v4, v15, v5);
                        v0 = v16;
                        continue;
                    } else {
                        v0 = v5;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method16(v1, v2);
            }
        }
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let mut v4: Rc<UH0> = method19(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn relative_to_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
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
fn walk_23(mut v0: Rc<RefCell<Vec<i32>>>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: Rc<RefCell<Vec<i32>>>, mut v3: Rc<RefCell<Vec<Rc<str>>>>, mut v4: i32, mut v5: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v6: bool = v4 < 0i32;
        if v6 {
            return v5;
        } else {
            let mut v7: i32 = v0.borrow()[v4 as usize];
            let v8: bool = v7 == 0i32;
            let v11: US0 = if v8 {
                US0::US0_TypeTwice
            } else {
                US0::US0_ShadowedUnused
            };
            let v12: Rc<str> = v3.borrow()[v4 as usize].clone();
            let v13: i32 = v2.borrow()[v4 as usize];
            let mut v14: i32 = v1.borrow()[v4 as usize];
            let v15: i32 = v4.wrapping_sub(1i32);
            let v16: Rc<UH1> = Rc::new(UH1::UH1_Cons(v12, v13, v14, v11, v5));
            (v4, v5) = (v15, v16);
            continue;
        }
    }
}
fn scan_of_24(mut v0: Rc<str>) -> (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = kit_lint_shadows_core::kit_lint_shadows_scan(&*v0);
    (v1, v2, v3, v4)
}
fn scanned_shadows_22(mut v0: Rc<str>) -> Rc<UH1> {
    let (mut v1, mut v2, mut v3, mut v4): (Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) = scan_of_24(v0);
    let mut v5: i32 = (v1.clone().borrow().len() as i32);
    let mut v6: i32 = v5.wrapping_sub(1i32);
    let mut v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    walk_23(v4, v3, v2, v1, v6, v7)
}
fn closure1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: US0) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v5: Rc<str>| -> bool {
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
        let mut v7: bool = v5.starts_with(&*v6);
        let v11: bool = if v7 {
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v10: bool = v5.contains(&*v9);
            v10
        } else {
            false
        };
        if v11 {
            let v15: Rc<str> = match &v4 {
                US0::US0_ShadowedUnused => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shadowed-unused"); } LIT.with(|lit| lit.clone()) };
                    v13
                }
                US0::US0_TypeTwice => {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-twice"); } LIT.with(|lit| lit.clone()) };
                    v12
                }
            };
            let v16: bool = v5.contains(&*v15);
            v16
        } else {
            false
        }
    })
}
fn method28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: US0) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1(v0, v1, v2, v3, v4)
}
fn method27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: US0, mut v5: Rc<dyn Fn(Rc<str>) -> bool>, mut v6: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v6) {
            UH0::UH0_Cons(v7, v8) => {
                let mut v7: Rc<str> = v7;
                let v8: Rc<UH0> = v8;
                let mut v9: bool = v5(v7);
                if v9 {
                    return true;
                } else {
                    let v10: Rc<dyn Fn(Rc<str>) -> bool> = method28(v0.clone(), v1.clone(), v2, v3, v4.clone());
                    (v5, v6) = (v10, v8);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn method30(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method30(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method16(v16, v17)
    }
}
fn known_in_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: i32, mut v5: US0) -> bool {
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<UH0> = split_list_29(v6, v0);
    let v8: Rc<dyn Fn(Rc<str>) -> bool> = method28(v1.clone(), v2.clone(), v3, v4, v5.clone());
    method27(v1, v2, v3, v4, v5, v8, v7)
}
fn method25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6, v7, v8) => {
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let mut v6: i32 = v6;
            let mut v7: US0 = v7;
            let v8: Rc<UH1> = v8;
            let mut v9: Rc<UH1> = method25(v0.clone(), v1.clone(), v8, v3);
            let v10: bool = known_in_26(v0, v1, v4.clone(), v5, v6, v7.clone());
            let v11: bool = v10 == false;
            if v11 {
                Rc::new(UH1::UH1_Cons(v4, v5, v6, v7, v9))
            } else {
                v9
            }
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let mut v4: i32 = v4;
            let mut v5: i32 = v5;
            let mut v6: US0 = v6;
            let mut v7: Rc<UH1> = v7;
            let v8: Rc<UH0> = method31(v0.clone(), v7, v2);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = Rc::<str>::from((v5).to_string());
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v3));
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v18: Rc<str> = match v6 {
                US0::US0_ShadowedUnused => {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("shadowed-unused"); } LIT.with(|lit| lit.clone()) };
                    v16
                }
                US0::US0_TypeTwice => {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-twice"); } LIT.with(|lit| lit.clone()) };
                    v15
                }
            };
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v18));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  first at line "); } LIT.with(|lit| lit.clone()) }));
            let v21: Rc<str> = Rc::<str>::from((v4).to_string());
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
            Rc::new(UH0::UH0_Cons(v22, v8))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn listed_33(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: i32 = v1.wrapping_sub(1i32);
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn open_rows_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> (Rc<UH0>, i32, i32) {
    let (mut v6, mut v7, mut v8): (Rc<RefCell<Vec<Rc<str>>>>, i32, i32) = kit_lint_opens::kit_lint_open_rows(&*v0, &*v1, &*v2, &*v3, &*v4, &*v5);
    let mut v9: i32 = (v6.clone().borrow().len() as i32);
    let v10: i32 = v9.wrapping_sub(1i32);
    let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v12: Rc<UH0> = listed_33(&v6, v10, v11);
    (v12, v7, v8)
}
fn method34(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: i32 = v4;
                let mut v5: US0 = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<UH0>, mut v5: Rc<UH0>, mut v6: i32, mut v7: i32) -> (Rc<UH0>, i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v4) {
            UH0::UH0_Cons(v8, v9) => {
                let v8: Rc<str> = v8;
                let mut v9: Rc<UH0> = v9;
                let v10: Rc<str> = relative_to_21(v0.clone(), v8.clone());
                let v12: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v8);
                let mut v14: Vec<u8> = v12.unwrap();
                let v15: Vec<u8> = method7(v14);
                let v17: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v15);
                let v19: std::string::String = v17.unwrap();
                let v21: Rc<str> = Rc::<str>::from(String::as_str(&v19));
                let v22: Rc<UH1> = scanned_shadows_22(v21.clone());
                let v23: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v24: Rc<UH1> = method25(v1.clone(), v10.clone(), v22.clone(), v23);
                let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v26: Rc<UH0> = method31(v10.clone(), v24.clone(), v25);
                let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v28: Rc<UH0> = method16(v26, v27);
                let (mut v33, mut v34, mut v35): (Rc<UH0>, i32, i32) = if v2 {
                    open_rows_32(v0.clone(), v3.clone(), v8, v10, v21, v1.clone())
                } else {
                    let v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    (v32, 0i32, 0i32)
                };
                let v36: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v37: Rc<UH0> = method16(v33, v36);
                let v38: Rc<UH0> = method19(v37, v28);
                let v39: i32 = 0i32;
                let v40: i32 = method34(v22, v39);
                let v41: i32 = v6.wrapping_add(v40);
                let v42: i32 = v41.wrapping_add(v34);
                let v43: Rc<UH0> = method19(v38, v5);
                let v44: i32 = 0i32;
                let v45: i32 = method34(v24, v44);
                let v46: i32 = v7.wrapping_add(v45);
                let v47: i32 = v46.wrapping_add(v35);
                (v4, v5, v6, v7) = (v9, v43, v42, v47);
                continue;
            }
            UH0::UH0_Nil => {
                return (v5, v6, v7);
            }
        }
    }
}
fn method35(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn shadows_report_11(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: bool, mut v4: Rc<str>) -> (Rc<UH0>, i32, i32, i32) {
    let mut v9: Rc<UH0> = match &*v1 {
        UH0::UH0_Nil => {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_Cons(v0.clone(), v5))
        }
        _ => {
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            method12(v0.clone(), v1, v7)
        }
    };
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method13(v9.clone(), v10);
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = method14(v9, v12);
    let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH0> = spi_files_15(v13, v14);
    let v16: Rc<UH0> = method19(v11, v15);
    let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v18: i32 = 0i32;
    let v19: i32 = 0i32;
    let (mut v20, mut v21, mut v22): (Rc<UH0>, i32, i32) = method20(v0, v2, v3, v4, v16.clone(), v17, v18, v19);
    let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v24: Rc<UH0> = method16(v20, v23);
    let v25: i32 = 0i32;
    let v26: i32 = method35(v16, v25);
    (v24, v26, v21, v22)
}
fn method36(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                println!("{}", v1);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn elapsed_text_37(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_39() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_38() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_39();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn shadows_run_0() -> i32 {
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
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint usage: lint shadows <root> [<path>...] [--baseline <rows>] [--opens [--lib <dir>]]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v17);
        2i32
    } else {
        let v18: bool = &*v12 != "";
        let v21: bool = if v18 {
            let v19: bool = file_exists_3(v12.clone());
            let v20: bool = v19 == false;
            v20
        } else {
            false
        };
        if v21 {
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: lint shadows: no baseline file at "); } LIT.with(|lit| lit.clone()) }, v2));
            eprintln!("{}", v22);
            2i32
        } else {
            let v23: bool = &*v12 == "";
            let v36: Rc<str> = if v23 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24
            } else {
                let v26: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v12);
                let v28: Vec<u8> = v26.unwrap();
                let v29: Vec<u8> = method7(v28);
                let v31: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v29);
                let v33: std::string::String = v31.unwrap();
                let v35: Rc<str> = Rc::<str>::from(String::as_str(&v33));
                v35
            };
            let v37: bool = &*v4 != "";
            let v40: Rc<str> = if v37 {
                v4
            } else {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_LIB"); } LIT.with(|lit| lit.clone()) };
                get_environment_variable_8(v38)
            };
            let v41: Rc<UH0> = listed_paths_9(&v7);
            let (mut v42, mut v43, mut v44, mut v45): (Rc<UH0>, i32, i32, i32) = shadows_report_11(v1, v41, v36, v6, v40);
            method36(v42);
            let v46: Rc<str> = elapsed_text_37(v0);
            let v47: Rc<str> = clock_text_38();
            let v48: bool = v45 == 0i32;
            let v51: Rc<str> = if v48 {
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v49
            } else {
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                v50
            };
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  files "); } LIT.with(|lit| lit.clone()) }));
            let v53: Rc<str> = Rc::<str>::from((v43).to_string());
            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  shadows "); } LIT.with(|lit| lit.clone()) }));
            let v56: Rc<str> = Rc::<str>::from((v44).to_string());
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new "); } LIT.with(|lit| lit.clone()) }));
            let v59: Rc<str> = Rc::<str>::from((v45).to_string());
            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v46));
            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v47));
            println!("{}", v64);
            if v48 {
                0i32
            } else {
                1i32
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        shadows_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_shadows_run() -> i32 {
    closure0()()
}
pub type SpiralShape_652accfc6a8d = Mut0;
