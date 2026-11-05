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
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
struct Mut0 { l0: i32, l1: Rc<UH1> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ZeroOnly,
    US1_AnyExit,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(i32),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_RowForOldCode,
    US4_RowForNewFile,
    US4_RowForNewCode,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US4, Rc<str>, Rc<str>, Rc<str>, Rc<UH2>),
}
struct Mut1 { l0: Rc<str> }
fn method1(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method6(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method6(v7, v9.clone()) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH1> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method5(v16.clone(), v17.clone())
    }
}
fn spi_known_8(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 < 7i32;
    if v4 {
        US0::US0_None
    } else {
        let mut v6: Rc<str> = v2.borrow()[1i32 as usize].clone();
        let v7: Rc<str> = v2.borrow()[3i32 as usize].clone();
        let mut v8: Rc<str> = v2.borrow()[5i32 as usize].clone();
        US0::US0_Some(v6.clone(), v7.clone(), v8.clone())
    }
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: US0 = spi_known_8(v2.clone());
                let mut v10: Rc<UH0> = match &v4 {
                    US0::US0_None => {
                        v1.clone()
                    }
                    US0::US0_Some(v5, v6, v7) => {
                        let v5: Rc<str> = v5.clone();
                        let mut v6: Rc<str> = v6.clone();
                        let v7: Rc<str> = v7.clone();
                        Rc::new(UH0::UH0_Cons(v5.clone(), v6.clone(), v7.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v3.clone(), v10.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn knowns_of_2(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl lint_known_"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v4.replace(&*v5, &*v6));
    let mut v8: Rc<UH1> = split_list_4(v1, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH0> = method7(v8, v9);
    let mut v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v10, v11)
}
fn method11(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH1>) -> () {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH1> = v3.clone();
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3.clone();
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn command_text_10(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<UH1>) -> US2 {
    let mut v3: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v1)));
    method11(v3.clone(), v2);
    drop(v3.borrow_mut().stdin(std::process::Stdio::null()));
    let mut v4: Rc<std::io::Result<std::process::Output>> = Rc::new(v3.borrow_mut().output());
    let mut v8: bool = match &v0 {
        US1::US1_AnyExit => {
            true
        }
        US1::US1_ZeroOnly => {
            let v5: i32 = (&*v4).as_ref().unwrap().status.code().unwrap_or(-2);
            let mut v6: bool = v5 == 0i32;
            v6
        }
    };
    let mut v9: bool = v4.is_ok();
    let mut v10: bool = v9 && v8;
    if v10 {
        let mut v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v4).as_ref().unwrap().stdout).as_ref());
        US2::US2_Some(v11)
    } else {
        US2::US2_None
    }
}
fn head_text_9(mut v0: Rc<str>, mut v1: Rc<str>) -> US2 {
    let mut v2: US1 = US1::US1_ZeroOnly;
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HEAD:"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-C"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v8));
    let mut v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7, v9));
    let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v0, v10));
    let mut v12: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v11));
    command_text_10(v2, v3, v12)
}
fn same_row_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> bool {
    let mut v6: bool = &*v0 == &*v3;
    let mut v8: bool = if v6 {
        let v7: bool = &*v1 == &*v4;
        v7
    } else {
        false
    };
    if v8 {
        let mut v9: bool = &*v2 == &*v5;
        v9
    } else {
        false
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> US0 {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let v7: Rc<str> = v7.clone();
                let mut v8: Rc<UH0> = v8.clone();
                let mut v9: bool = same_row_15(v0.clone(), v1.clone(), v2.clone(), v5.clone(), v6.clone(), v7.clone());
                if v9 {
                    return US0::US0_Some(v5.clone(), v6.clone(), v7.clone());
                } else {
                    v3 = v8.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US0::US0_None;
            }
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let v7: US0 = method14(v3.clone(), v4.clone(), v5.clone(), v0.clone());
                let mut v8: bool = match &v7 {
                    US0::US0_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = v8 != true;
                let mut v11: Rc<UH0> = if v9 {
                    v2.clone()
                } else {
                    Rc::new(UH0::UH0_Cons(v3.clone(), v4.clone(), v5.clone(), v2.clone()))
                };
                (v1, v2) = (v6.clone(), v11.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn added_rows_12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH0> = method13(v1, v0, v2);
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v3, v4)
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    Rc::new(move |mut v2: Rc<UH1>| -> US3 {
        match &*v2 {
            UH1::UH1_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH1> = v5.clone();
                let mut v6: bool = &*v4 == &*v0;
                if v6 {
                    US3::US3_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US3> = method18(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH1::UH1_Nil => {
                US3::US3_None
            }
        }
    })
}
fn method18(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US3> {
    closure1(v0, v1)
}
fn method20(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn directory_get_parent_19(mut v0: Rc<str>) -> Option<Rc<str>> {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let mut v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let mut v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method20(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let mut v21: bool = true; v19 });
    let mut v23: Option<Rc<str>> = _optionm_map_;
    v23.clone()
}
fn method21(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure2() -> Rc<dyn Fn((Rc<str>)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US2> = Rc::new(move |mut v0: (Rc<str>)| -> US2 {
        let mut v1: Rc<str> = (v0);
        US2::US2_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn staged_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH1>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> Rc<UH1> {
    let v7: i32 = 0i32;
    let mut v8: Rc<dyn Fn(Rc<UH1>) -> US3> = method18(v4.clone(), v7);
    let mut v9: US3 = v8(v3.clone());
    let mut v10: bool = match &v9 {
        US3::US3_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v11: bool = v10 != true;
    if v11 {
        v3.clone()
    } else {
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v12)), v4));
        let v15: US2 = head_text_9(v1.clone(), v14.clone());
        match &v15 {
            US2::US2_None => {
                v3.clone()
            }
            US2::US2_Some(v16) => {
                let v16: Rc<str> = v16.clone();
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v12)), v4));
                let v19: Option<Rc<str>> = directory_get_parent_19(v18.clone());
                let v20: Option<Rc<str>> = method21(v19.clone());
                let mut v21: Rc<dyn Fn((Rc<str>)) -> US2> = closure2();
                let mut v22: Option<US2> = v20.map(|x| v21(x));
                let mut v24: US2 = US2::US2_None;
                let v25: US2 = v24.clone();
                let v26: US2 = v22.unwrap_or(v25);
                let v29: Rc<str> = match &v26 {
                    US2::US2_None => {
                        v0.clone()
                    }
                    US2::US2_Some(v27) => {
                        let v27: Rc<str> = v27.clone();
                        v27.clone()
                    }
                };
                let v31: Result<(), std::io::Error> = std::fs::create_dir_all(&*v29);
                std::fs::write(&*v18, &*v16).unwrap();
                Rc::new(UH1::UH1_Cons(v4.clone(), v3.clone()))
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v3 {
            UH0::UH0_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let v7: Rc<str> = v7.clone();
                let mut v8: Rc<UH0> = v8.clone();
                let mut v9: Rc<UH1> = staged_17(v0.clone(), v1.clone(), v2.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone());
                (v3, v4) = (v8.clone(), v9.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v4.clone();
            }
        }
    }
}
fn current_executable_22() -> Rc<str> {
    let mut v0: Option<std::path::PathBuf> = std::env::current_exe().ok();
    let mut v2: bool = true; let _optionm_map_ = v0.map(|x| { //;
    let mut v4: std::path::PathBuf = x;
    let v5: Rc<str> = Rc::<str>::from(v4.to_string_lossy().as_ref());
    let v7: bool = true; v5 });
    let mut v9: Option<Rc<str>> = _optionm_map_;
    let mut v10: Option<Rc<str>> = method21(v9.clone());
    let mut v11: Rc<dyn Fn((Rc<str>)) -> US2> = closure2();
    let mut v12: Option<US2> = v10.map(|x| v11(x));
    let mut v14: US2 = US2::US2_None;
    let v15: US2 = v14.clone();
    let v16: US2 = v12.unwrap_or(v15);
    match &v16 {
        US2::US2_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
        US2::US2_Some(v17) => {
            let v17: Rc<str> = v17.clone();
            v17.clone()
        }
    }
}
fn method27(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method28(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
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
fn words_in_29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method5(v2.clone(), v5.clone());
        } else {
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v8: i32 = v1.wrapping_add(1i32);
            let mut v9: bool = &*v7 == "";
            let mut v11: Rc<UH1> = if v9 {
                v2.clone()
            } else {
                Rc::new(UH1::UH1_Cons(v7.clone(), v2.clone()))
            };
            (v1, v2) = (v8, v11.clone());
            continue;
        }
    }
}
fn method30(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_26(mut v0: Rc<str>) -> US0 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method27(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method28(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v11: i32 = 0i32;
    let mut v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v13: Rc<UH1> = words_in_29(v10.clone(), v11, v12.clone());
    match &*v13 {
        UH1::UH1_Cons(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<UH1> = v15.clone();
            match &*v15 {
                UH1::UH1_Cons(v16, v17) => {
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<UH1> = v17.clone();
                    match &*v17 {
                        UH1::UH1_Cons(v18, v19) => {
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH1> = v19.clone();
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let mut v21: Option<usize> = v14.rfind(&*v20);
                            let mut v22: Option<usize> = method30(v21.clone());
                            let mut v23: Rc<dyn Fn((usize)) -> i32> = closure3();
                            let mut v24: Option<i32> = v22.map(|x| v23(x));
                            let v26: i32 = v24.unwrap_or(-1i32);
                            let v27: bool = &*v16 == "long";
                            let v35: bool = if v27 {
                                true
                            } else {
                                let v28: bool = &*v16 == "block";
                                if v28 {
                                    true
                                } else {
                                    let v29: bool = &*v16 == "flow";
                                    if v29 {
                                        true
                                    } else {
                                        let v30: bool = &*v16 == "cast";
                                        if v30 {
                                            true
                                        } else {
                                            let v31: bool = &*v16 == "arith";
                                            v31
                                        }
                                    }
                                }
                            };
                            let v46: bool = if v35 {
                                let v36: bool = &*v18 == "long";
                                let v44: bool = if v36 {
                                    true
                                } else {
                                    let v37: bool = &*v18 == "block";
                                    if v37 {
                                        true
                                    } else {
                                        let v38: bool = &*v18 == "flow";
                                        if v38 {
                                            true
                                        } else {
                                            let v39: bool = &*v18 == "cast";
                                            if v39 {
                                                true
                                            } else {
                                                let v40: bool = &*v18 == "arith";
                                                v40
                                            }
                                        }
                                    }
                                };
                                let v45: bool = v44 == false;
                                v45
                            } else {
                                false
                            };
                            let v47: bool = v26 <= 0i32;
                            if v47 {
                                US0::US0_None
                            } else {
                                let v49: usize = ((0i32) as usize);
                                let v50: usize = ((v26) as usize);
                                let v51: Rc<str> = Rc::<str>::from(&(v14)[v49..v50]);
                                if v46 {
                                    let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    US0::US0_Some(v51.clone(), v52.clone(), v16.clone())
                                } else {
                                    US0::US0_Some(v51.clone(), v16.clone(), v18.clone())
                                }
                            }
                        }
                        _ => {
                            US0::US0_None
                        }
                    }
                }
                _ => {
                    US0::US0_None
                }
            }
        }
        _ => {
            US0::US0_None
        }
    }
}
fn baseline_from_25(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: US0 = known_of_26(v5.clone());
            match &v6 {
                US0::US0_None => {
                    let mut v13: i32 = v1.wrapping_add(1i32);
                    v1 = v13;
                    continue;
                }
                US0::US0_Some(v7, v8, v9) => {
                    let v7: Rc<str> = v7.clone();
                    let mut v8: Rc<str> = v8.clone();
                    let mut v9: Rc<str> = v9.clone();
                    let mut v10: i32 = v1.wrapping_add(1i32);
                    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v8.clone(), v9.clone(), v2.clone()));
                    (v1, v2) = (v10, v11.clone());
                    continue;
                }
            }
        }
    }
}
fn baseline_of_24(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    baseline_from_25(v5.clone(), v6, v7.clone())
}
fn head_knowns_23(mut v0: Rc<str>) -> Rc<UH0> {
    baseline_of_24(v0)
}
fn change_of_32(mut v0: bool, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> (US4, Rc<str>, Rc<str>, Rc<str>) {
    let v5: bool = v0 == false;
    let mut v13: US4 = if v5 {
        US4::US4_RowForNewFile
    } else {
        let v7: US0 = method14(v2.clone(), v3.clone(), v4.clone(), v1);
        let mut v8: bool = match &v7 {
            US0::US0_None => {
                true
            }
            _ => {
                false
            }
        };
        let mut v9: bool = v8 != true;
        if v9 {
            US4::US4_RowForOldCode
        } else {
            US4::US4_RowForNewCode
        }
    };
    (v13, v2, v3, v4)
}
fn method31(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match &*v2 {
        UH0::UH0_Cons(v4, v5, v6, v7) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let mut v8: Rc<UH2> = method31(v0.clone(), v1.clone(), v7, v3.clone());
            let mut v9: i32 = 0i32;
            let mut v10: Rc<dyn Fn(Rc<UH1>) -> US3> = method18(v4.clone(), v9);
            let mut v11: US3 = v10(v0);
            let mut v12: bool = match &v11 {
                US3::US3_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v13: bool = v12 != true;
            let (mut v14, mut v15, mut v16, mut v17): (US4, Rc<str>, Rc<str>, Rc<str>) = change_of_32(v13, v1, v4, v5, v6);
            Rc::new(UH2::UH2_Cons(v14, v15, v16, v17, v8))
        }
        UH0::UH0_Nil => {
            v3
        }
    }
}
fn change_line_34(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v7: Rc<str> = match &v0 {
        US4::US4_RowForNewCode => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new-code"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US4::US4_RowForNewFile => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("new-file"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US4::US4_RowForOldCode => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("old-code"); } LIT.with(|lit| lit.clone()) };
            v4
        }
    };
    let mut v8: bool = &*v2 == "";
    let mut v10: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        v2
    };
    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v11.borrow_mut().push(v7);
    v11.borrow_mut().push(v1);
    v11.borrow_mut().push(v10);
    v11.borrow_mut().push(v3);
    let mut v12: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    v15
}
fn method33(mut v0: Rc<UH2>) -> () {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v1, v2, v3, v4, v5) => {
                let mut v1: US4 = v1.clone();
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let mut v6: Rc<str> = change_line_34(v1.clone(), v2.clone(), v3.clone(), v4.clone());
                println!("{}", v6);
                v0 = v5.clone();
                continue;
            }
            UH2::UH2_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn refused_36(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> bool {
    let mut v4: bool = match &v0 {
        US4::US4_RowForOldCode => {
            true
        }
        _ => {
            false
        }
    };
    let v5: bool = v4 != true;
    v5
}
fn method35(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: US4 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH2> = v6.clone();
                let v7: bool = refused_36(v2.clone(), v3.clone(), v4.clone(), v5.clone());
                let mut v9: i32 = if v7 {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    v8
                } else {
                    v1
                };
                (v0, v1) = (v6.clone(), v9);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method37(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method39(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_38(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method39(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn growth_run_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v2));
    let mut v8: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
    let mut v10: Vec<u8> = v8.unwrap();
    let mut v11: Vec<u8> = method1(v10);
    let mut v13: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v11);
    let v15: std::string::String = v13.unwrap();
    let v17: Rc<str> = Rc::<str>::from(String::as_str(&v15));
    let v18: Rc<UH0> = knowns_of_2(v17);
    let v19: US2 = head_text_9(v3.clone(), v2);
    let mut v23: Rc<str> = match &v19 {
        US2::US2_None => {
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v21
        }
        US2::US2_Some(v20) => {
            let v20: Rc<str> = v20.clone();
            v20
        }
    };
    let mut v24: Rc<UH0> = knowns_of_2(v23);
    let v25: Rc<UH0> = added_rows_12(v18, v24);
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.cache/lint-growth"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v26));
    let v28: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v29: Rc<UH1> = method16(v27.clone(), v1, v0, v25.clone(), v28);
    let v30: Rc<str> = current_executable_22();
    let v31: US1 = US1::US1_AnyExit;
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("macros"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v35: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27, v34));
    let v36: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33, v35));
    let v37: Rc<UH1> = Rc::new(UH1::UH1_Cons(v32, v36));
    let v38: US2 = command_text_10(v31, v30, v37);
    let v42: Rc<str> = match &v38 {
        US2::US2_None => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v40
        }
        US2::US2_Some(v39) => {
            let v39: Rc<str> = v39.clone();
            v39
        }
    };
    let v43: Rc<UH0> = head_knowns_23(v42);
    let v44: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v45: Rc<UH2> = method31(v29, v43, v25.clone(), v44);
    method33(v45.clone());
    let v46: i32 = 0i32;
    let v47: i32 = method35(v45, v46);
    let v48: i32 = 0i32;
    let v49: i32 = method37(v25, v48);
    let v50: Rc<str> = format_real_38(v49);
    let v51: Rc<str> = format_real_38(v47);
    let mut v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" added="); } LIT.with(|lit| lit.clone()) };
    v52.borrow_mut().push(v53);
    v52.borrow_mut().push(v50);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" refused="); } LIT.with(|lit| lit.clone()) };
    v52.borrow_mut().push(v54);
    v52.borrow_mut().push(v51);
    let v55: Rc<Vec<Rc<str>>> = Rc::new(v52.borrow().clone());
    let v56: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v55).as_ref().clone()));
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = Rc::<str>::from((*v56.borrow()).join(&*v57));
    let v59: bool = v47 == 0i32;
    let v62: Rc<str> = if v59 {
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v60
    } else {
        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("refused"); } LIT.with(|lit| lit.clone()) };
        v61
    };
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit lint growth "); } LIT.with(|lit| lit.clone()) };
    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v63, v62)), v58));
    println!("{}", v65);
    if v59 {
        0i32
    } else {
        1i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> i32 {
        growth_run_0(v3.clone(), v2.clone(), v1.clone(), v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_growth_run(v0: &str, v1: &str, v2: &str, v3: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
