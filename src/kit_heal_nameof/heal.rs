#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_HealNameof,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
enum US1 {
    US1_Ok(Vec<std::string::String>),
    US1_Error(std::io::Error),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(US0),
    US3_None,
}
fn heal_usage_1() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: heal usage: heal nameof <dir>... (lists matches that map 3 or more cases to their own names as strings, or back, per file and per root; the compiler's tokens command reads each .spi)"); } LIT.with(|lit| lit.clone()) };
    eprintln!("{}", v0);
    2i32
}
fn get_environment_variable_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn compiler_dll_3() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_COMPILER_DLL"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_4(v0);
    let v2: bool = &*v1 != "";
    if v2 {
        v1
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = get_environment_variable_4(v3);
        let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/spiral-bin/bin/single-flight/SpiralCompiler/Release/net11.0/SpiralCompiler.dll"); } LIT.with(|lit| lit.clone()) }));
        v5
    }
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
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
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US1> = Rc::new(move |mut v0: Vec<std::string::String>| -> US1 {
        US1::US1_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method10() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c2()
}
fn directory_exists_13(mut v0: Rc<str>) -> bool {
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
fn skipped_directory_14(mut v0: Rc<str>) -> bool {
    let v1: bool = &*v0 == "target";
    let mut v3: bool = if v1 {
        true
    } else {
        let v2: bool = &*v0 == ".git";
        v2
    };
    let mut v5: bool = if v3 {
        true
    } else {
        let v4: bool = &*v0 == ".cache";
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let mut v6: bool = &*v0 == "node_modules";
        v6
    };
    let mut v9: bool = if v7 {
        true
    } else {
        let mut v8: bool = &*v0 == "deps";
        v8
    };
    if v9 {
        true
    } else {
        let v10: bool = &*v0 == "_build";
        v10
    }
}
fn method12(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> (Rc<UH0>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v4.clone()));
                let mut v8: bool = directory_exists_13(v7.clone());
                let (mut v18, mut v19): (Rc<UH0>, Rc<UH0>) = if v8 {
                    let mut v9: bool = skipped_directory_14(v4);
                    if v9 {
                        (v2, v3)
                    } else {
                        let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v2));
                        (v10, v3)
                    }
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v4.ends_with(&*v13);
                    if v14 {
                        let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v3));
                        (v2, v15)
                    } else {
                        (v2, v3)
                    }
                };
                (v1, v2, v3) = (v5, v18, v19);
                continue;
            }
            UH0::UH0_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn spi_files_from_8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v7: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                let mut v8: Rc<dyn Fn(Vec<std::string::String>) -> US1> = method10();
                let mut v9: Rc<dyn Fn(std::io::Error) -> US1> = method11();
                let v10: US1 = match v7 { Ok(x) => v8(x), Err(e) => v9(e) };
                let v31: Rc<UH0> = match v10 {
                    US1::US1_Error(v28) => {
                        let v28: std::io::Error = v28;
                        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                    }
                    US1::US1_Ok(v11) => {
                        let mut v11: Vec<std::string::String> = v11;
                        let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v14: bool = true; let _vec_fold_ = v11.into_iter().fold(v12, |acc, x| { //;
                        let v16: Rc<UH0> = acc;
                        let v18: std::string::String = x;
                        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v16));
                        let v23: bool = true; v22 });
                        let v25: Rc<UH0> = _vec_fold_;
                        let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        method9(v25, v26)
                    }
                };
                let (mut v32, mut v33): (Rc<UH0>, Rc<UH0>) = method12(v4, v31, v5, v1);
                (v0, v1) = (v32, v33);
                continue;
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method9(v1, v2);
            }
        }
    }
}
fn tokens_of_17(mut v0: Rc<str>, mut v1: Rc<str>) -> US2 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
    drop(v3.borrow_mut().arg(&*v0));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tokens"); } LIT.with(|lit| lit.clone()) };
    drop(v3.borrow_mut().arg(&*v4));
    drop(v3.borrow_mut().arg(&*v1));
    drop(v3.borrow_mut().stdin(std::process::Stdio::null()));
    let mut v5: Rc<std::io::Result<std::process::Output>> = Rc::new(v3.borrow_mut().output());
    let mut v6: bool = v5.is_ok();
    let mut v9: bool = if v6 {
        let v7: i32 = (&*v5).as_ref().unwrap().status.code().unwrap_or(-2);
        let mut v8: bool = v7 == 0i32;
        v8
    } else {
        false
    };
    if v9 {
        let v10: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v5).as_ref().unwrap().stdout).as_ref());
        US2::US2_Some(v10)
    } else {
        US2::US2_None
    }
}
fn method21(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn finding_line_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nameof "); } LIT.with(|lit| lit.clone()) }, v0));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = format_real_20(v2);
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v1));
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" arms="); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = format_real_20(v3);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cases="); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v4));
    v15
}
fn rows_shown_18(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        let v4: i32 = (v1.clone().borrow().len() as i32);
        let mut v5: bool = v2 >= v4;
        if v5 {
            return (v2, v3);
        } else {
            let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, i32, i32, Rc<str>) = v1.borrow()[v2 as usize].clone();
            let v10: Rc<str> = finding_line_19(v0.clone(), v6, v7, v8, v9);
            println!("{}", v10);
            let mut v11: i32 = v2.wrapping_add(1i32);
            let v12: i32 = v3.wrapping_add(v8);
            (v2, v3) = (v11, v12);
            continue;
        }
    }
}
fn file_tally_16(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> (i32, i32, i32, i32) {
    let mut v6: US2 = tokens_of_17(v0, v5.clone());
    let (mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, i32) = match &v6 {
        US2::US2_None => {
            let v7: i32 = v1.wrapping_add(1i32);
            let mut v8: i32 = v2.wrapping_add(1i32);
            (v7, v8, v3, v4)
        }
        _ => {
            let (mut v19, mut v20, mut v21, mut v22): (i32, i32, i32, i32) = match v6 {
                US2::US2_Some(v9) => {
                    let mut v9: Rc<str> = v9;
                    let v10: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v5).unwrap_or_default().as_str());
                    let mut v11: Rc<RefCell<Vec<(Rc<str>, i32, i32, Rc<str>)>>> = kit_nameof_findings::kit_nameof_finding_rows(&v10, &v9);
                    let v12: i32 = 0i32;
                    let v13: i32 = 0i32;
                    let (mut v14, mut v15): (i32, i32) = rows_shown_18(v5, &v11, v12, v13);
                    let v16: i32 = v1.wrapping_add(1i32);
                    let v17: i32 = v3.wrapping_add(v14);
                    let v18: i32 = v4.wrapping_add(v15);
                    (v16, v2, v17, v18)
                }
                _ => unreachable!(),
            };
            (v19, v20, v21, v22)
        }
    };
    (v23, v24, v25, v26)
}
fn method15(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> (i32, i32, i32, i32) {
    let (mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, i32) = match &*v1 {
        UH0::UH0_Cons(v6, v7) => {
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let (mut v8, mut v9, mut v10, mut v11): (i32, i32, i32, i32) = file_tally_16(v0.clone(), v2, v3, v4, v5, v6);
            method15(v0, v7, v8, v9, v10, v11)
        }
        _ => {
            let (mut v16, mut v17, mut v18, mut v19): (i32, i32, i32, i32) = match Rc::unwrap_or_clone(v1) {
                UH0::UH0_Nil => {
                    (v2, v3, v4, v5)
                }
                _ => unreachable!(),
            };
            (v16, v17, v18, v19)
        }
    };
    (v20, v21, v22, v23)
}
fn root_report_7(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v2));
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = spi_files_from_8(v3, v4);
    let mut v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let mut v8: i32 = 0i32;
    let mut v9: i32 = 0i32;
    let (mut v10, mut v11, mut v12, mut v13): (i32, i32, i32, i32) = method15(v0, v5, v6, v7, v8, v9);
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit heal nameof root="); } LIT.with(|lit| lit.clone()) }, v1));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" files="); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = format_real_20(v10);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unreadable="); } LIT.with(|lit| lit.clone()) }));
    let v19: Rc<str> = format_real_20(v11);
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) }));
    let v22: Rc<str> = format_real_20(v12);
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" arms="); } LIT.with(|lit| lit.clone()) }));
    let v25: Rc<str> = format_real_20(v13);
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    println!("{}", v26);
    v13
}
fn method6(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let mut v5: i32 = root_report_7(v0.clone(), v3);
                let mut v6: i32 = v2.wrapping_add(v5);
                (v1, v2) = (v4, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method22(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn heal_ran_2(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    match v0 {
        US0::US0_HealNameof => {
            let v2: Rc<str> = compiler_dll_3();
            let mut v3: bool = match &*v1 {
                UH0::UH0_Nil => {
                    true
                }
                _ => {
                    false
                }
            };
            if v3 {
                heal_usage_1()
            } else {
                let mut v5: bool = file_exists_5(v2.clone());
                let mut v6: bool = v5 == false;
                if v6 {
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: heal nameof: no compiler at "); } LIT.with(|lit| lit.clone()) }, v2));
                    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; set SPIRAL_COMPILER_DLL"); } LIT.with(|lit| lit.clone()) }));
                    eprintln!("{}", v8);
                    2i32
                } else {
                    let mut v9: i32 = 0i32;
                    let v10: i32 = method6(v2, v1.clone(), v9);
                    let mut v11: i32 = 0i32;
                    let v12: i32 = method22(v1, v11);
                    let v13: Rc<str> = format_real_20(v12);
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit heal nameof ok roots="); } LIT.with(|lit| lit.clone()) }, v13));
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" arms="); } LIT.with(|lit| lit.clone()) }));
                    let v16: Rc<str> = format_real_20(v10);
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                    println!("{}", v17);
                    0i32
                }
            }
        }
    }
}
fn command_line_argument_count_24() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_25(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_23(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_24();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US2 = if v4 {
        let mut v5: Rc<str> = command_line_argument_25(v0);
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
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_23(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn heal_named_26(mut v0: Rc<str>) -> US3 {
    let v1: bool = &*v0 == "nameof";
    if v1 {
        let v2: US0 = US0::US0_HealNameof;
        US3::US3_Some(v2)
    } else {
        US3::US3_None
    }
}
fn heal_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_23(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let mut v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
                    let mut v6: US3 = heal_named_26(v4);
                    match v6 {
                        US3::US3_None => {
                            heal_usage_1()
                        }
                        US3::US3_Some(v7) => {
                            let v7: US0 = v7;
                            heal_ran_2(v7, v5)
                        }
                    }
                }
                _ => {
                    heal_usage_1()
                }
            }
        }
        _ => {
            heal_usage_1()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        heal_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_heal_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
