#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_BuiltReady(Rc<str>),
    US0_BuildFailed(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
pub struct Mut2 { pub l0: i32 }
fn method4(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_3(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn step_failed_2(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed with exit "); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = format_real_3(v1);
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    US0::US0_BuildFailed(v4)
}
fn fnv_bytes_6(mut v0: &Rc<RefCell<Vec<u8>>>, mut v1: i32, mut v2: u64) -> u64 {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let mut v6: u8 = v0.borrow()[v1 as usize];
            let v7: u64 = (v6 as u64);
            let mut v8: u64 = v2 ^ v7;
            let mut v9: u64 = v8.wrapping_mul(16777619u64);
            let v10: u64 = v9 & 4294967295u64;
            (v1, v2) = (v5, v10);
            continue;
        }
    }
}
fn format_real_7(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn key_of_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: &Rc<str>, mut v4: Rc<str>) -> (Rc<str>, Rc<UH0>, Rc<str>) {
    let v5: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v4.as_bytes().to_vec()));
    let mut v6: i32 = 0i32;
    let v7: u64 = 2166136261u64;
    let mut v8: u64 = fnv_bytes_6(&v5, v6, v7);
    let mut v9: Rc<str> = format_real_7(v8);
    let v10: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v2.as_bytes().to_vec()));
    let v11: i32 = 0i32;
    let v12: u64 = 2166136261u64;
    let mut v13: u64 = fnv_bytes_6(&v10, v11, v12);
    let v14: Rc<str> = format_real_7(v13);
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v16));
    (v9, v17, v14)
}
fn method8(mut v0: Rc<UH0>, mut v1: u64) -> u64 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v2.as_bytes().to_vec()));
                let v5: i32 = 0i32;
                let mut v6: u64 = 2166136261u64;
                let v7: u64 = fnv_bytes_6(&v4, v5, v6);
                let mut v8: u64 = v1.wrapping_add(v7);
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn stem_of_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v1));
    let mut v4: Rc<UH0> = split_list_10(v1, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = method13(v4, v5.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v5));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let v12: Rc<str> = Rc::<str>::from(v11.to_lowercase());
    v12
}
fn windows_14() -> bool {
    let mut v0: bool = cfg!(windows);
    v0
}
fn paths_of_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/pkgs"); } LIT.with(|lit| lit.clone()) }));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1.clone()));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1.clone()));
    let v10: Rc<str> = if v2 {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".exe"); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v1));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".rs"); } LIT.with(|lit| lit.clone()) }));
    (v3, v5, v14, v15, v11)
}
fn file_exists_16(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn manifest_of_17(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("packages:\n    |core-\n    spiral-\nmodules:\n    "); } LIT.with(|lit| lit.clone()) }, v0));
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn directory_exists_19(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn native_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = windows_14();
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
        v4
    } else {
        v0
    }
}
fn method23(mut v0: i32, mut v1: &Rc<RefCell<Mut2>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let mut v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: 0i32 }));
                while method23(v7, &v9) {
                    let v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let mut v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn step_code_21(mut v0: Rc<str>, mut v1: Rc<UH0>) -> i32 {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = method22(v1, v2);
    let mut v4: Option<std::process::ExitStatus> = std::process::Command::new(&*v0).args((v3).borrow().iter().map(|a| &**a)).status().ok();
    let v5: i32 = v4.and_then(|s| s.code()).unwrap_or(127);
    v5
}
fn linked_18(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = directory_exists_19(v0.clone());
    if v2 {
        true
    } else {
        let v3: bool = windows_14();
        if v3 {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = native_20(v0);
            let mut v6: Rc<str> = native_20(v1);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/c"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mklink"); } LIT.with(|lit| lit.clone()) };
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/J"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v10));
            let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v11));
            let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v12));
            let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v13));
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v14));
            let v16: i32 = step_code_21(v4, v15);
            let v17: bool = v16 == 0i32;
            v17
        } else {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ln"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-s"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v20));
            let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v21));
            let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v22));
            let v24: i32 = step_code_21(v18, v23);
            let v25: bool = v24 == 0i32;
            v25
        }
    }
}
fn program_built_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> US0 {
    let v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let (mut v7, mut v8, mut v9): (Rc<str>, Rc<UH0>, Rc<str>) = key_of_5(v0.clone(), v1.clone(), v2.clone(), &v3, v5.clone());
    let v10: u64 = 0u64;
    let v11: u64 = method8(v8, v10);
    let v12: Rc<str> = format_real_7(v11);
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v13.borrow_mut().push(v9);
    v13.borrow_mut().push(v7);
    v13.borrow_mut().push(v12);
    let v14: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from((*v15.borrow()).join(&*v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v17));
    let v19: Rc<str> = stem_of_9(v4.clone());
    let v20: bool = windows_14();
    let (mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = paths_of_15(v18, v19, v20);
    let v26: bool = &*v5 == "";
    if v26 {
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cannot read "); } LIT.with(|lit| lit.clone()) }, v4));
        US0::US0_BuildFailed(v27)
    } else {
        let v29: bool = file_exists_16(v25.clone());
        if v29 {
            US0::US0_BuiltReady(v25)
        } else {
            let v32: Result<(), std::io::Error> = std::fs::create_dir_all(&*v22);
            std::fs::write(&*v23, &*v5).unwrap();
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
            let v35: Rc<str> = stem_of_9(v4);
            let v36: Rc<str> = manifest_of_17(v35);
            std::fs::write(&*v34, &*v36).unwrap();
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/spiral"); } LIT.with(|lit| lit.clone()) }));
            let v39: bool = linked_18(v38, v0);
            let v40: bool = v39 == false;
            let v44: bool = if v40 {
                true
            } else {
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/core"); } LIT.with(|lit| lit.clone()) }));
                let v42: bool = linked_18(v41, v1);
                let v43: bool = v42 == false;
                v43
            };
            if v44 {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not link the lib and core packages beside the script"); } LIT.with(|lit| lit.clone()) };
                US0::US0_BuildFailed(v45)
            } else {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--backend"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rust"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24.clone(), v50));
                let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v51));
                let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v49, v52));
                let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v48, v53));
                let v55: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v54));
                let v56: i32 = step_code_21(v47, v55);
                let v57: bool = v56 != 0i32;
                if v57 {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                    step_failed_2(v58, v56)
                } else {
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-O"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--edition"); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2021"); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25.clone(), v65));
                    let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v64, v66));
                    let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v67));
                    let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v68));
                    let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v62, v69));
                    let v71: Rc<UH0> = Rc::new(UH0::UH0_Cons(v61, v70));
                    let v72: i32 = step_code_21(v60.clone(), v71);
                    let v73: bool = v72 == 0i32;
                    if v73 {
                        US0::US0_BuiltReady(v25)
                    } else {
                        step_failed_2(v60, v72)
                    }
                }
            }
        }
    }
}
fn built_exe_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v5: US0 = program_built_1(v4, v3, v2, v1, v0);
    match v5 {
        US0::US0_BuildFailed(v8) => {
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v9, v8)
        }
        US0::US0_BuiltReady(v6) => {
            let mut v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v6, v7)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>| -> (Rc<str>, Rc<str>) {
        built_exe_0(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_run_built_exe(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str) -> (Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
pub type SpiralShape_2617b4e49837 = Mut2;
