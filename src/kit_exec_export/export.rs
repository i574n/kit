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
    US1_Some(i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(u64),
    US2_None,
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
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
    let mut v8: US0 = if v4 {
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
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_value_4(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn closure1(mut v0: i32) -> Rc<dyn Fn(Rc<UH0>) -> US1> {
    Rc::new(move |mut v1: Rc<UH0>| -> US1 {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: bool = &*v3 == "--jobs";
                if v5 {
                    US1::US1_Some(v0)
                } else {
                    let mut v7: i32 = v0.wrapping_add(1i32);
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
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let mut v6: Rc<UH0> = method8(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
            let v10: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v8);
            let v12: Vec<u8> = v10.unwrap();
            let v13: Vec<u8> = method9(v12);
            let v15: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v13);
            let v17: std::string::String = v15.unwrap();
            let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
            let v20: Rc<str> = kit_exec_cells::kit_exec_notebook_prom(&v0, &v19);
            Rc::new(UH0::UH0_Cons(v20, v6))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method11(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".ipynb"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.ends_with(&*v5);
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
fn directory_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn notebooks_in_10(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: bool = directory_exists_12(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let v12: Rc<UH0> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v12));
        let v19: bool = true; v18 });
        let v21: Rc<UH0> = _vec_fold_;
        let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v21, v22)
    }
}
fn directory_prom_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: Rc<UH0> = notebooks_in_10(v1.clone());
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method8(v0, v1, v2, v3)
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method13(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
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
            let v4: Rc<UH0> = method14(v3, v1);
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
fn method15(mut v0: Rc<UH0>) -> () {
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
fn method16(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method18(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_17(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method18(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn export_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let mut v6: bool = &*v4 == "export";
                    let v16: bool = if v6 {
                        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--notebooks"); } LIT.with(|lit| lit.clone()) };
                        let mut v8: Rc<str> = flag_value_4(v5.clone(), v7);
                        let v9: bool = &*v8 != "";
                        if v9 {
                            true
                        } else {
                            let v10: i32 = 0i32;
                            let mut v11: Rc<dyn Fn(Rc<UH0>) -> US1> = method5(v10);
                            let v12: US1 = v11(v5.clone());
                            let v13: bool = match v12 {
                                US1::US1_None => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v14: bool = v13 != true;
                            v14
                        }
                    } else {
                        false
                    };
                    if v16 {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--machine"); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = flag_value_4(v5.clone(), v17);
                        let v19: bool = &*v18 == "node";
                        let v22: Rc<str> = if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v21
                        };
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--notebooks"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = flag_value_4(v5.clone(), v23);
                        let v25: i32 = 0i32;
                        let mut v26: Rc<dyn Fn(Rc<UH0>) -> US1> = method5(v25);
                        let v27: US1 = v26(v5.clone());
                        let v28: bool = match v27 {
                            US1::US1_None => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        let v29: bool = v28 != true;
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--since"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = flag_value_4(v5, v30);
                        let v32: Option<u64> = (v31).trim().parse::<u64>().ok();
                        let v33: Option<u64> = method6(v32);
                        let mut v34: Rc<dyn Fn((u64)) -> US2> = closure2();
                        let mut v35: Option<US2> = v33.map(|x| v34(x));
                        let v37: US2 = US2::US2_None;
                        let v38: US2 = v37.clone();
                        let v39: US2 = v35.unwrap_or(v38);
                        let v42: u64 = match v39 {
                            US2::US2_None => {
                                0u64
                            }
                            US2::US2_Some(v40) => {
                                let v40: u64 = v40;
                                v40
                            }
                        };
                        let v43: bool = &*v24 == "";
                        let v46: Rc<UH0> = if v43 {
                            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                        } else {
                            directory_prom_7(v22.clone(), v24)
                        };
                        let v51: Rc<UH0> = if v29 {
                            let v47: Rc<str> = kit_exec_jobs::kit_exec_jobs_prom(&v22, v42);
                            let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            Rc::new(UH0::UH0_Cons(v47, v48))
                        } else {
                            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                        };
                        let v52: Rc<UH0> = method13(v46, v51);
                        let v53: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v54: Rc<UH0> = method14(v52, v53);
                        method15(v54.clone());
                        let v55: i32 = 0i32;
                        let v56: i32 = method16(v54, v55);
                        let v57: Rc<str> = format_real_17(v56);
                        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit metrics export ok parts="); } LIT.with(|lit| lit.clone()) }, v57));
                        eprintln!("{}", v58);
                        0i32
                    } else {
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit metrics export [--notebooks <dir>] [--jobs [--since <unix ms>]] [--machine box|node]"); } LIT.with(|lit| lit.clone()) };
                        eprintln!("{}", v59);
                        2i32
                    }
                }
                _ => {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit metrics export [--notebooks <dir>] [--jobs [--since <unix ms>]] [--machine box|node]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v61);
                    2i32
                }
            }
        }
        _ => {
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit metrics export [--notebooks <dir>] [--jobs [--since <unix ms>]] [--machine box|node]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v63);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        export_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_exec_export_run() -> i32 {
    closure0()()
}
use kit_exec_cells::SpiralShape_97df63ebabc3 as Mut0;
