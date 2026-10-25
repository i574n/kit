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
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ToolRust,
    US1_ToolZig,
    US1_ToolOtp,
    US1_ToolElixir,
    US1_ToolGleam,
    US1_ToolRebar,
    US1_ToolPython,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US1, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_OtherAction,
    US2_SetupRust,
    US2_SetupZig,
    US2_SetupBeam,
    US2_SetupPython,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US1),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US1, Rc<str>),
    US4_None,
}
struct Mut0 { l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_2();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_3(v0);
        US0::US0_Some(v5.clone())
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_Some(v9) => {
            let mut v9: Rc<str> = v9.clone();
            v9.clone()
        }
    }
}
fn method5(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0.clone()
}
fn method4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method5(v13.clone());
    let mut v16: std::path::Display = v14.display();
    let mut v18: std::string::String = format!("{}", v16);
    let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20.clone()
}
fn directory_exists_6(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method11(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n- uses: workflow-boundary\n"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v5));
                let mut v7: Rc<str> = method4(v0.clone(), v3.clone());
                let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v7);
                let mut v11: Vec<u8> = v9.unwrap();
                let mut v12: Vec<u8> = method11(v11.clone());
                let mut v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
                let mut v16: std::string::String = v14.unwrap();
                let mut v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
                let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v18));
                (v0, v1, v2) = (v0.clone(), v4.clone(), v19.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v2.clone();
            }
        }
    }
}
fn names_in_12(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: bool = directory_exists_6(v0.clone());
    let mut v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let mut v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let mut v12: Rc<UH1> = acc;
        let mut v14: std::string::String = x;
        let mut v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let mut v18: Rc<UH1> = Rc::new(UH1::UH1_Cons(v16.clone(), v12.clone()));
        let mut v19: bool = true; v18 });
        let mut v21: Rc<UH1> = _vec_fold_;
        v21.clone()
    }
}
fn workflow_file_14(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".yml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    if v2 {
        true
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".yaml"); } LIT.with(|lit| lit.clone()) };
        let mut v4: bool = v0.ends_with(&*v3);
        v4
    }
}
fn method13(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH1> = v3.clone();
            let mut v4: Rc<UH1> = method13(v3.clone(), v1.clone());
            let mut v5: bool = workflow_file_14(v2.clone());
            if v5 {
                Rc::new(UH1::UH1_Cons(v2.clone(), v4.clone()))
            } else {
                v4.clone()
            }
        }
        UH1::UH1_Nil => {
            v1.clone()
        }
    }
}
fn workflows_text_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH1> = names_in_12(v0.clone());
    let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<UH1> = method13(v1.clone(), v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method10(v0.clone(), v3.clone(), v4.clone())
}
fn action_of_17(mut v0: Rc<str>, mut v1: US2) -> US2 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("uses:"); } LIT.with(|lit| lit.clone()) };
    let mut v3: bool = v0.contains(&*v2);
    let mut v4: bool = v3 == false;
    if v4 {
        v1.clone()
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("setup-rust"); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.contains(&*v5);
        let mut v9: bool = if v6 {
            true
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0.contains(&*v7);
            v8
        };
        if v9 {
            US2::US2_SetupRust
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("setup-zig"); } LIT.with(|lit| lit.clone()) };
            let mut v12: bool = v0.contains(&*v11);
            if v12 {
                US2::US2_SetupZig
            } else {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("setup-beam"); } LIT.with(|lit| lit.clone()) };
                let mut v15: bool = v0.contains(&*v14);
                if v15 {
                    US2::US2_SetupBeam
                } else {
                    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("setup-python"); } LIT.with(|lit| lit.clone()) };
                    let mut v18: bool = v0.contains(&*v17);
                    if v18 {
                        US2::US2_SetupPython
                    } else {
                        US2::US2_OtherAction
                    }
                }
            }
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn key_of_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method19(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method20(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v11: Rc<str> = v10.clone().borrow()[0i32 as usize].clone();
    v11.clone()
}
fn beam_tool_22(mut v0: Rc<str>) -> US3 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("otp-version"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US1 = US1::US1_ToolOtp;
        US3::US3_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir-version"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US1 = US1::US1_ToolElixir;
            US3::US3_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam-version"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US1 = US1::US1_ToolGleam;
                US3::US3_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3-version"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US1 = US1::US1_ToolRebar;
                    US3::US3_Some(v11.clone())
                } else {
                    US3::US3_None
                }
            }
        }
    }
}
fn key_tool_21(mut v0: US2, mut v1: Rc<str>) -> US3 {
    match &v0 {
        US2::US2_OtherAction => {
            US3::US3_None
        }
        US2::US2_SetupBeam => {
            beam_tool_22(v1.clone())
        }
        US2::US2_SetupPython => {
            let mut v15: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python-version"); } LIT.with(|lit| lit.clone()) };
            if v15 {
                let mut v16: US1 = US1::US1_ToolPython;
                US3::US3_Some(v16.clone())
            } else {
                US3::US3_None
            }
        }
        US2::US2_SetupRust => {
            let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("channel"); } LIT.with(|lit| lit.clone()) };
            let mut v4: bool = if v2 {
                true
            } else {
                let mut v3: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
                v3
            };
            if v4 {
                let mut v5: US1 = US1::US1_ToolRust;
                US3::US3_Some(v5.clone())
            } else {
                US3::US3_None
            }
        }
        US2::US2_SetupZig => {
            let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: US1 = US1::US1_ToolZig;
                US3::US3_Some(v10.clone())
            } else {
                US3::US3_None
            }
        }
    }
}
fn unquoted_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method19(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method20(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v10));
    v13.clone()
}
fn value_of_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = method19(v0.clone(), v2, v3);
    let mut v5: i32 = v2.wrapping_sub(1i32);
    let mut v6: Rc<str> = string_slice(&v0.clone(), v4 as i64, v5 as i64);
    let mut v7: i32 = (v6.clone().len() as i32);
    let mut v8: i32 = method20(v6.clone(), v7);
    let mut v9: Rc<str> = string_slice(&v6.clone(), 0i32 as i64, v8 as i64);
    let mut v10: i32 = (v1.clone().len() as i32);
    let mut v11: i32 = v10.wrapping_add(1i32);
    let mut v12: i32 = (v9.clone().len() as i32);
    let mut v13: i32 = v12.wrapping_sub(v11);
    let mut v14: usize = ((v11) as usize);
    let mut v15: i32 = v11.wrapping_add(v13);
    let mut v16: usize = ((v15) as usize);
    let mut v17: Rc<str> = Rc::<str>::from(&(v9)[v14..v16]);
    unquoted_24(v17.clone())
}
fn method26(mut v0: US1, mut v1: Rc<str>, mut v2: Rc<UH0>) -> US4 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6) => {
                let mut v4: US1 = v4.clone();
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let mut v7: bool = match (&v4, &v0) {
                    (US1::US1_ToolElixir, US1::US1_ToolElixir) => {
                        true
                    }
                    (US1::US1_ToolGleam, US1::US1_ToolGleam) => {
                        true
                    }
                    (US1::US1_ToolOtp, US1::US1_ToolOtp) => {
                        true
                    }
                    (US1::US1_ToolPython, US1::US1_ToolPython) => {
                        true
                    }
                    (US1::US1_ToolRebar, US1::US1_ToolRebar) => {
                        true
                    }
                    (US1::US1_ToolRust, US1::US1_ToolRust) => {
                        true
                    }
                    (US1::US1_ToolZig, US1::US1_ToolZig) => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let mut v9: bool = if v7 {
                    let mut v8: bool = v5.clone() == v1.clone();
                    v8
                } else {
                    false
                };
                if v9 {
                    return US4::US4_Some(v4.clone(), v5.clone());
                } else {
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v6.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn already_pinned_25(mut v0: Rc<UH0>, mut v1: US1, mut v2: Rc<str>) -> bool {
    let mut v3: US4 = method26(v1.clone(), v2.clone(), v0.clone());
    let mut v4: bool = match &v3 {
        US4::US4_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v5: bool = v4 != true;
    v5
}
fn pins_from_16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: US2, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let mut v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method8(v3.clone(), v6.clone());
        } else {
            let mut v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v9: US2 = action_of_17(v8.clone(), v2.clone());
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let mut v11: bool = v8.contains(&*v10);
            let mut v14: Rc<str> = if v11 {
                key_of_18(v8.clone())
            } else {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v13.clone()
            };
            let mut v15: US3 = key_tool_21(v9.clone(), v14.clone());
            match &v15 {
                US3::US3_None => {
                    let mut v23: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v23, v9.clone(), v3.clone());
                    continue;
                }
                US3::US3_Some(v16) => {
                    let mut v16: US1 = v16.clone();
                    let mut v17: Rc<str> = value_of_23(v8.clone(), v14.clone());
                    let mut v18: bool = already_pinned_25(v3.clone(), v16.clone(), v17.clone());
                    let mut v20: Rc<UH0> = if v18 {
                        v3.clone()
                    } else {
                        Rc::new(UH0::UH0_Cons(v16.clone(), v17.clone(), v3.clone()))
                    };
                    let mut v21: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v21, v9.clone(), v20.clone());
                    continue;
                }
            }
        }
    }
}
fn workflow_pins_15(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: US2 = US2::US2_OtherAction;
    let mut v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    pins_from_16(v5.clone(), v6, v7.clone(), v8.clone())
}
fn method27(mut v0: Rc<UH0>) -> US4 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = match &v2 {
                    US1::US1_ToolRust => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                if v5 {
                    return US4::US4_Some(v2.clone(), v3.clone());
                } else {
                    v0 = v4.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn channel_line_30(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US0 {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let mut v3: bool = v1 >= v2;
        if v3 {
            return US0::US0_None;
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method19(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method20(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("channel"); } LIT.with(|lit| lit.clone()) };
            let mut v15: bool = v13.starts_with(&*v14);
            let mut v18: bool = if v15 {
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let mut v17: bool = v13.contains(&*v16);
                v17
            } else {
                false
            };
            if v18 {
                let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let mut v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v13.split(&*v19).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let mut v21: Rc<str> = v20.clone().borrow()[1i32 as usize].clone();
                let mut v22: Rc<str> = unquoted_24(v21.clone());
                let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v23 {
                    return US0::US0_None;
                } else {
                    return US0::US0_Some(v22.clone());
                }
            } else {
                let mut v27: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v27);
                continue;
            }
        }
    }
}
fn toolchain_file_channel_29(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method19(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method20(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[toolchain]"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v11.contains(&*v12);
    if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v14).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v16: i32 = 0i32;
        channel_line_30(v15.clone(), v16)
    } else {
        let mut v18: bool = v11.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v21: bool = if v18 {
            true
        } else {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v20: bool = v11.contains(&*v19);
            v20
        };
        if v21 {
            US0::US0_None
        } else {
            US0::US0_Some(v11.clone())
        }
    }
}
fn file_exists_32(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn text_at_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_32(v0.clone());
    if v1 {
        let mut v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method11(v5.clone());
        let mut v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let mut v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12.clone()
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    }
}
fn repo_toolchain_28(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain.toml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method4(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_at_31(v2.clone());
    let mut v4: US0 = toolchain_file_channel_29(v3.clone());
    match &v4 {
        US0::US0_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = method4(v0.clone(), v7.clone());
            let mut v9: Rc<str> = text_at_31(v8.clone());
            toolchain_file_channel_29(v9.clone())
        }
        US0::US0_Some(v5) => {
            let mut v5: Rc<str> = v5.clone();
            US0::US0_Some(v5.clone())
        }
    }
}
fn workflow_and_file_pins_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<str> = workflows_text_9(v1.clone());
    let mut v3: Rc<UH0> = workflow_pins_15(v2.clone());
    let mut v4: US4 = method27(v3.clone());
    let mut v5: bool = match &v4 {
        US4::US4_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v6: bool = v5 != true;
    let mut v7: US0 = repo_toolchain_28(v0.clone());
    match &v7 {
        US0::US0_None => {
            v3.clone()
        }
        US0::US0_Some(v8) => {
            let mut v8: Rc<str> = v8.clone();
            if v6 {
                v3.clone()
            } else {
                let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v10: Rc<UH0> = method8(v3.clone(), v9.clone());
                let mut v11: US1 = US1::US1_ToolRust;
                let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v8.clone(), v12.clone()));
                method8(v10.clone(), v13.clone())
            }
        }
    }
}
fn tool_name_34(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_ToolElixir => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US1::US1_ToolGleam => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US1::US1_ToolOtp => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("otp"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US1::US1_ToolPython => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
        US1::US1_ToolRebar => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US1::US1_ToolRust => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_ToolZig => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
    }
}
fn probe_row_33(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>) -> (Rc<str>, bool) {
    let mut v3: Rc<str> = tool_name_34(v1.clone());
    let (mut v4, mut v5): (Rc<str>, bool) = eoie_ci_verdict::eoie_ci_parity_row(&*v3, &*v2, &*v0);
    (v4.clone(), v5)
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(US1, Rc<str>) -> (Rc<str>, bool)> {
    Rc::new(move |mut v1: US1, mut v2: Rc<str>| -> (Rc<str>, bool) {
        probe_row_33(v0.clone(), v1.clone(), v2.clone())
    })
}
fn rows_printed_35(mut v0: Rc<dyn Fn(US1, Rc<str>) -> (Rc<str>, bool)>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5) => {
                let mut v3: US1 = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let (mut v6, mut v7): (Rc<str>, bool) = v0(v3.clone(), v4.clone());
                println!("{}", v6);
                let mut v9: i32 = if v7 {
                    let mut v8: i32 = v2.wrapping_add(1i32);
                    v8
                } else {
                    v2
                };
                (v0, v1, v2) = (v0.clone(), v5.clone(), v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method36(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: US1 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method38(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_37(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method38(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn parity_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".github"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = method4(v1.clone(), v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("workflows"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = method4(v3.clone(), v4.clone());
    let mut v6: i32 = 1i32;
    let mut v7: Rc<str> = argument_1(v6);
    let mut v8: bool = v7.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("parity"); } LIT.with(|lit| lit.clone()) };
    let mut v10: bool = if v8 {
        true
    } else {
        let mut v9: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    };
    if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: ci usage: "); } LIT.with(|lit| lit.clone()) };
        let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-* pins in <repo>/.github/workflows and its rust-toolchain file against the local toolchain; one row per tool, a fix line per mismatch, exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: ci usage: ci parity <repo> (setup-* pins in <repo>/.github/workflows and its rust-toolchain file against the local toolchain; one row per tool, a fix line per mismatch, exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v13);
        2i32
    } else {
        let mut v14: bool = directory_exists_6(v5.clone());
        let mut v15: bool = v14 == false;
        if v15 {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie ci parity refused: no workflow folder at "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v5));
            println!("{}", v17);
            1i32
        } else {
            let mut v18: Rc<UH0> = workflow_and_file_pins_7(v1.clone(), v5.clone());
            let mut v19: Rc<dyn Fn(US1, Rc<str>) -> (Rc<str>, bool)> = closure1(v1.clone());
            let mut v20: i32 = 0i32;
            let mut v21: i32 = rows_printed_35(v19.clone(), v18.clone(), v20);
            let mut v22: i32 = 0i32;
            let mut v23: i32 = method36(v18.clone(), v22);
            let mut v24: Rc<str> = format_real_37(v23);
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tools="); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" differs="); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            let mut v29: Rc<str> = format_real_37(v21);
            let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" repo="); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
            let mut v34: bool = v23 == 0i32;
            if v34 {
                let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie ci parity refused: no setup-* or rust-toolchain pin found;"); } LIT.with(|lit| lit.clone()) };
                let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v33));
                println!("{}", v36);
                1i32
            } else {
                let mut v37: bool = v21 > 0i32;
                if v37 {
                    let mut v38: Rc<str> = format_real_37(v21);
                    let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie ci parity refused: "); } LIT.with(|lit| lit.clone()) };
                    let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v38));
                    let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tool(s) differ from CI;"); } LIT.with(|lit| lit.clone()) };
                    let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v33));
                    println!("{}", v43);
                    1i32
                } else {
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie ci parity ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v33));
                    println!("{}", v45);
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        parity_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_ci_parity_run() -> i32 {
    closure0()()
}
