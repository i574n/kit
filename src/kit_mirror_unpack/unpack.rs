#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RemovalInside(Rc<str>),
    US0_RemovalRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Ok,
    US2_Error(Rc<str>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn closure1(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v1: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == "..";
                if v5 {
                    US1::US1_Some(v0)
                } else {
                    let v7: i32 = v0.wrapping_add(1i32);
                    let mut v8: Rc<dyn Fn(Rc<UH0>) -> US1> = method5(v7);
                    v8(v4)
                }
            }
            UH0::UH0_Nil => {
                US1::US1_None
            }
        }
    })
}
fn method5(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    closure1(v0)
}
fn removal_of_1(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v1));
    let v4: Rc<UH0> = split_list_2(v1.clone(), v3);
    let v5: bool = &*v0 == "";
    let v7: bool = if v5 {
        true
    } else {
        let mut v6: bool = v0.starts_with(&*v1);
        v6
    };
    let v10: bool = if v7 {
        true
    } else {
        let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = v0.contains(&*v8);
        v9
    };
    let v16: bool = if v10 {
        true
    } else {
        let v11: i32 = 0i32;
        let mut v12: Rc<dyn Fn(Rc<UH0>) -> US1> = method5(v11);
        let v13: US1 = v12(v4);
        let v14: bool = match v13 {
            US1::US1_None => {
                true
            }
            _ => {
                false
            }
        };
        let v15: bool = v14 != true;
        v15
    };
    if v16 {
        US0::US0_RemovalRefused(v0)
    } else {
        US0::US0_RemovalInside(v0)
    }
}
fn file_exists_6(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn tar_program_7() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_8(v0);
    let v2: bool = &*v1 == "";
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tar"); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/System32/tar.exe"); } LIT.with(|lit| lit.clone()) }));
        v4
    }
}
fn method9(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method10(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
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
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method13() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn() -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> US2> = Rc::new(move || -> US2 {
        US2::US2_Ok
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14() -> Rc<dyn Fn() -> US2> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US2> = Rc::new(move |mut v0: Rc<str>| -> US2 {
        US2::US2_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15() -> Rc<dyn Fn(Rc<str>) -> US2> {
    error_c4()
}
fn removed_inside_12(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32, mut v3: Rc<UH0>) -> (i32, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v6, v7) => {
                let mut v6: Rc<str> = v6;
                let v7: Rc<UH0> = v7;
                let mut v8: US0 = removal_of_1(v6);
                match v8 {
                    US0::US0_RemovalInside(v13) => {
                        let v13: Rc<str> = v13;
                        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
                        let v16: bool = file_exists_6(v15.clone());
                        if v16 {
                            let v17: bool = file_exists_6(v15.clone());
                            let mut v21: Result<(), std::io::Error> = if v17 {
                                let v19: Result<(), std::io::Error> = std::fs::remove_file(&*v15);
                                v19
                            } else {
                                let mut v20: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                                v20
                            };
                            let mut v22: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method13();
                            let v24: Result<(), Rc<str>> = v21.map_err(|x| v22(x));
                            let mut v25: Rc<dyn Fn() -> US2> = method14();
                            let mut v26: Rc<dyn Fn(Rc<str>) -> US2> = method15();
                            let v27: US2 = match v24 { Ok(()) => v25(), Err(e) => v26(e) };
                            match v27 {
                                US2::US2_Error(v28) => {
                                    let v28: Rc<str> = v28;
                                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) }, v28));
                                    eprintln!("{}", v29);
                                    ()
                                }
                                US2::US2_Ok => {
                                    ()
                                }
                            };
                            let v30: i32 = v2.wrapping_add(1i32);
                            (v1, v2) = (v7, v30);
                            continue;
                        } else {
                            v1 = v7;
                            continue;
                        }
                    }
                    US0::US0_RemovalRefused(v9) => {
                        let mut v9: Rc<str> = v9;
                        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v3));
                        (v1, v3) = (v7, v10);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH0> = method3(v3, v4);
                return (v2, v5);
            }
        }
    }
}
fn method16(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror unpack refused "); } LIT.with(|lit| lit.clone()) }, v1));
                eprintln!("{}", v3);
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
fn method18(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_17(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method19(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn unpack_printed_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".kit-mirror-incoming.tar"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v0.clone()));
    let mut v8: bool = &*v0 == "";
    let v14: bool = if v8 {
        true
    } else {
        let mut v9: US0 = removal_of_1(v0.clone());
        let mut v12: bool = match v9 {
            US0::US0_RemovalInside(v10) => {
                let v10: Rc<str> = v10;
                let v11: bool = &*v10 == &*v0;
                v11
            }
            _ => {
                false
            }
        };
        let v13: bool = v12 != true;
        v13
    };
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror unpack: the removal list name must be a plain relative name"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v15);
        2i32
    } else {
        let v16: bool = file_exists_6(v6.clone());
        let v17: bool = v16 == false;
        if v17 {
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror unpack: no .kit-mirror-incoming.tar under "); } LIT.with(|lit| lit.clone()) }, v4));
            eprintln!("{}", v18);
            2i32
        } else {
            let v19: Rc<str> = tar_program_7();
            let mut v20: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v19)));
            let mut v21: bool = &*v6 == "";
            let v29: Rc<UH0> = if v21 {
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-xf"); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH0::UH0_Cons(v22, v23))
            } else {
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-xf"); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25, v26));
                Rc::new(UH0::UH0_Cons(v6.clone(), v27))
            };
            let v30: bool = &*v4 == "";
            let v36: Rc<UH0> = if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
                Rc::new(UH0::UH0_Cons(v31, v29))
            } else {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33, v29));
                Rc::new(UH0::UH0_Cons(v4.clone(), v34))
            };
            let v37: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v38: Rc<UH0> = method3(v36, v37);
            method9(v20.clone(), v38);
            let mut v39: Rc<std::io::Result<std::process::Output>> = Rc::new(v20.borrow_mut().output());
            let v40: bool = v39.is_ok();
            let (mut v46, mut v47, mut v48): (bool, i32, Rc<str>) = if v40 {
                let v41: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v39).as_ref().unwrap().stderr).as_ref());
                let v42: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v39).as_ref().unwrap().stdout).as_ref());
                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v41));
                let v44: i32 = (&*v39).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v44, v43)
            } else {
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v45)
            };
            let v50: bool = if v46 {
                let v49: bool = v47 == 0i32;
                v49
            } else {
                false
            };
            let v51: bool = v50 == false;
            if v51 {
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: mirror unpack: tar could not extract "); } LIT.with(|lit| lit.clone()) }, v6));
                eprintln!("{}", v52);
                2i32
            } else {
                let v53: bool = file_exists_6(v7.clone());
                let v68: Rc<UH0> = if v53 {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v56: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v7);
                    let v58: Vec<u8> = v56.unwrap();
                    let v59: Vec<u8> = method10(v58);
                    let v61: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v59);
                    let v63: std::string::String = v61.unwrap();
                    let v65: Rc<str> = Rc::<str>::from(String::as_str(&v63));
                    split_list_2(v54, v65)
                } else {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                };
                let v69: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v70: Rc<UH0> = method11(v68, v69);
                let v71: i32 = 0i32;
                let v72: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let (mut v73, mut v74): (i32, Rc<UH0>) = removed_inside_12(v4, v70, v71, v72);
                let v75: bool = file_exists_6(v6.clone());
                let mut v79: Result<(), std::io::Error> = if v75 {
                    let v77: Result<(), std::io::Error> = std::fs::remove_file(&*v6);
                    v77
                } else {
                    let v78: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                    v78
                };
                let mut v80: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method13();
                let v82: Result<(), Rc<str>> = v79.map_err(|x| v80(x));
                let mut v83: Rc<dyn Fn() -> US2> = method14();
                let mut v84: Rc<dyn Fn(Rc<str>) -> US2> = method15();
                let v85: US2 = match v82 { Ok(()) => v83(), Err(e) => v84(e) };
                match v85 {
                    US2::US2_Error(v86) => {
                        let v86: Rc<str> = v86;
                        let v87: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) }, v86));
                        eprintln!("{}", v87);
                        ()
                    }
                    US2::US2_Ok => {
                        ()
                    }
                };
                let v88: bool = file_exists_6(v7.clone());
                if v88 {
                    let v89: bool = file_exists_6(v7.clone());
                    let mut v93: Result<(), std::io::Error> = if v89 {
                        let v91: Result<(), std::io::Error> = std::fs::remove_file(&*v7);
                        v91
                    } else {
                        let v92: Result<(), std::io::Error> = Ok::<(), std::io::Error>(());
                        v92
                    };
                    let mut v94: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method13();
                    let v96: Result<(), Rc<str>> = v93.map_err(|x| v94(x));
                    let mut v97: Rc<dyn Fn() -> US2> = method14();
                    let mut v98: Rc<dyn Fn(Rc<str>) -> US2> = method15();
                    let v99: US2 = match v96 { Ok(()) => v97(), Err(e) => v98(e) };
                    match v99 {
                        US2::US2_Error(v100) => {
                            let v100: Rc<str> = v100;
                            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.file_delete: "); } LIT.with(|lit| lit.clone()) }, v100));
                            eprintln!("{}", v101);
                            ()
                        }
                        US2::US2_Ok => {
                            ()
                        }
                    }
                };
                method16(v74.clone());
                let v102: Rc<str> = format_real_17(v73);
                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("removed="); } LIT.with(|lit| lit.clone()) }, v102));
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v103, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused="); } LIT.with(|lit| lit.clone()) }));
                let v105: i32 = 0i32;
                let v106: i32 = method19(v74.clone(), v105);
                let v107: Rc<str> = format_real_17(v106);
                let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v107));
                let v109: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror unpack ok "); } LIT.with(|lit| lit.clone()) }, v108));
                eprintln!("{}", v109);
                let v110: bool = match Rc::unwrap_or_clone(v74) {
                    UH0::UH0_Nil => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v110 {
                    0i32
                } else {
                    1i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        unpack_printed_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_unpack_printed(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_652accfc6a8d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
