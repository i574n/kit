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
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
fn tool_of_name_1(mut v0: Rc<str>) -> US0 {
    let mut v1: bool = &*v0 == "rust";
    if v1 {
        let mut v2: US1 = US1::US1_ToolRust;
        US0::US0_Some(v2)
    } else {
        let mut v4: bool = &*v0 == "zig";
        if v4 {
            let mut v5: US1 = US1::US1_ToolZig;
            US0::US0_Some(v5)
        } else {
            let mut v7: bool = &*v0 == "otp";
            if v7 {
                let v8: US1 = US1::US1_ToolOtp;
                US0::US0_Some(v8)
            } else {
                let v10: bool = &*v0 == "elixir";
                if v10 {
                    let v11: US1 = US1::US1_ToolElixir;
                    US0::US0_Some(v11)
                } else {
                    let mut v13: bool = &*v0 == "gleam";
                    if v13 {
                        let mut v14: US1 = US1::US1_ToolGleam;
                        US0::US0_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "rebar3";
                        if v16 {
                            let v17: US1 = US1::US1_ToolRebar;
                            US0::US0_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "python";
                            if v19 {
                                let v20: US1 = US1::US1_ToolPython;
                                US0::US0_Some(v20)
                            } else {
                                US0::US0_None
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                        let v8: bool = v4 == b'\n';
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
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                        let v8: bool = v4 == b'\n';
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
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method5(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0, v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method6(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v10));
    v13
}
fn channel_line_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US2 {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return US2::US2_None;
        } else {
            let mut v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let v8: i32 = method5(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method6(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("channel"); } LIT.with(|lit| lit.clone()) };
            let v15: bool = v13.starts_with(&*v14);
            let v18: bool = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v17: bool = v13.contains(&*v16);
                v17
            } else {
                false
            };
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v13.split(&*v19).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                let v21: Rc<str> = v20.borrow()[1i32 as usize].clone();
                let v22: Rc<str> = unquoted_7(v21);
                let v23: bool = &*v22 == "";
                if v23 {
                    return US2::US2_None;
                } else {
                    return US2::US2_Some(v22);
                }
            } else {
                let v27: i32 = v1.wrapping_add(1i32);
                v1 = v27;
                continue;
            }
        }
    }
}
fn toolchain_file_channel_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method5(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method6(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[toolchain]"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v11.contains(&*v12);
    if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v14).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v16: i32 = 0i32;
        channel_line_4(v15, v16)
    } else {
        let v18: bool = &*v11 == "";
        let v21: bool = if v18 {
            true
        } else {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v20: bool = v11.contains(&*v19);
            v20
        };
        if v21 {
            US2::US2_None
        } else {
            US2::US2_Some(v11)
        }
    }
}
fn method9(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method9(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method12(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_at_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_11(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method12(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn repo_toolchain_2(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain.toml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method8(v0.clone(), v1);
    let v3: Rc<str> = text_at_10(v2);
    let mut v4: US2 = toolchain_file_channel_3(v3);
    match v4 {
        US2::US2_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = method8(v0, v7);
            let mut v9: Rc<str> = text_at_10(v8);
            toolchain_file_channel_3(v9)
        }
        US2::US2_Some(v5) => {
            let mut v5: Rc<str> = v5;
            US2::US2_Some(v5)
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn word_at_17(mut v0: i32, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: bool = v0 < v4;
    if v5 {
        let mut v6: Rc<str> = v3.borrow()[v0 as usize].clone();
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    }
}
fn method19(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn without_host_triple_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-x86_64-"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method19(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 > 0i32;
    let mut v9: i32 = if v8 {
        v7
    } else {
        -1i32
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-aarch64-"); } LIT.with(|lit| lit.clone()) };
    let v11: Option<usize> = v0.find(&*v10);
    let v12: Option<usize> = method19(v11);
    let mut v13: Option<i32> = v12.map(|x| v4(x));
    let v15: i32 = v13.unwrap_or(-1i32);
    let v16: bool = v15 > 0i32;
    let v20: bool = if v16 {
        let v17: bool = v9 < 0i32;
        if v17 {
            true
        } else {
            let v18: bool = v15 < v9;
            v18
        }
    } else {
        false
    };
    let v21: i32 = if v20 {
        v15
    } else {
        v9
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-i686-"); } LIT.with(|lit| lit.clone()) };
    let v23: Option<usize> = v0.find(&*v22);
    let v24: Option<usize> = method19(v23);
    let mut v25: Option<i32> = v24.map(|x| v4(x));
    let v27: i32 = v25.unwrap_or(-1i32);
    let v28: bool = v27 > 0i32;
    let v32: bool = if v28 {
        let v29: bool = v21 < 0i32;
        if v29 {
            true
        } else {
            let mut v30: bool = v27 < v21;
            v30
        }
    } else {
        false
    };
    let v33: i32 = if v32 {
        v27
    } else {
        v21
    };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-arm"); } LIT.with(|lit| lit.clone()) };
    let v35: Option<usize> = v0.find(&*v34);
    let v36: Option<usize> = method19(v35);
    let mut v37: Option<i32> = v36.map(|x| v4(x));
    let v39: i32 = v37.unwrap_or(-1i32);
    let v40: bool = v39 > 0i32;
    let v44: bool = if v40 {
        let v41: bool = v33 < 0i32;
        if v41 {
            true
        } else {
            let v42: bool = v39 < v33;
            v42
        }
    } else {
        false
    };
    let v45: i32 = if v44 {
        v39
    } else {
        v33
    };
    let v46: bool = v45 < 0i32;
    if v46 {
        v0
    } else {
        let v47: usize = ((0i32) as usize);
        let mut v48: usize = ((v45) as usize);
        let v49: Rc<str> = Rc::<str>::from(&(v0)[v47..v48]);
        v49
    }
}
fn channels_from_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method16(v2, v5);
        } else {
            let mut v7: i32 = 0i32;
            let v8: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v9: i32 = (v8.clone().len() as i32);
            let v10: i32 = 0i32;
            let v11: i32 = method5(v8.clone(), v9, v10);
            let v12: i32 = v9.wrapping_sub(1i32);
            let mut v13: Rc<str> = string_slice(&v8, v11 as i64, v12 as i64);
            let mut v14: i32 = (v13.clone().len() as i32);
            let v15: i32 = method6(v13.clone(), v14);
            let v16: Rc<str> = string_slice(&v13, 0i32 as i64, v15 as i64);
            let v17: Rc<str> = word_at_17(v7, v16);
            let v18: Rc<str> = without_host_triple_18(v17);
            let v19: i32 = v1.wrapping_add(1i32);
            let v20: bool = &*v18 == "";
            let v22: Rc<UH0> = if v20 {
                v2
            } else {
                Rc::new(UH0::UH0_Cons(v18, v2))
            };
            (v1, v2) = (v19, v22);
            continue;
        }
    }
}
fn installed_channels_14(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    channels_from_15(v5, v6, v7)
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method22(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
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
fn captured_20(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cmd"); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".bat"); } LIT.with(|lit| lit.clone()) }));
    let mut v4: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v0)));
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = method21(v1.clone(), v5);
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method16(v6, v7);
    method22(v4.clone(), v8);
    let mut v9: Rc<std::io::Result<std::process::Output>> = Rc::new(v4.borrow_mut().output());
    let v10: bool = v9.is_ok();
    let (mut v16, mut v17, mut v18): (bool, i32, Rc<str>) = if v10 {
        let v11: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stderr).as_ref());
        let v12: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v9).as_ref().unwrap().stdout).as_ref());
        let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
        let mut v14: i32 = (&*v9).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v14, v13)
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v15)
    };
    let v20: bool = if v16 {
        let v19: bool = v17 == 0i32;
        v19
    } else {
        false
    };
    let v23: US2 = if v20 {
        US2::US2_Some(v18)
    } else {
        US2::US2_None
    };
    let v46: US2 = match &v23 {
        US2::US2_None => {
            let mut v25: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v2)));
            let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v27: Rc<UH0> = method21(v1.clone(), v26);
            let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v29: Rc<UH0> = method16(v27, v28);
            method22(v25.clone(), v29);
            let mut v30: Rc<std::io::Result<std::process::Output>> = Rc::new(v25.borrow_mut().output());
            let v31: bool = v30.is_ok();
            let (mut v37, mut v38, mut v39): (bool, i32, Rc<str>) = if v31 {
                let v32: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v30).as_ref().unwrap().stderr).as_ref());
                let v33: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v30).as_ref().unwrap().stdout).as_ref());
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
                let v35: i32 = (&*v30).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v35, v34)
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v36)
            };
            let v41: bool = if v37 {
                let v40: bool = v38 == 0i32;
                v40
            } else {
                false
            };
            if v41 {
                US2::US2_Some(v39)
            } else {
                US2::US2_None
            }
        }
        US2::US2_Some(v24) => {
            let v24: Rc<str> = v24.clone();
            v23
        }
    };
    match &v46 {
        US2::US2_None => {
            let mut v48: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v3)));
            let v49: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v50: Rc<UH0> = method21(v1, v49);
            let v51: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v52: Rc<UH0> = method16(v50, v51);
            method22(v48.clone(), v52);
            let mut v53: Rc<std::io::Result<std::process::Output>> = Rc::new(v48.borrow_mut().output());
            let v54: bool = v53.is_ok();
            let (mut v60, mut v61, mut v62): (bool, i32, Rc<str>) = if v54 {
                let v55: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stderr).as_ref());
                let v56: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v53).as_ref().unwrap().stdout).as_ref());
                let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v55));
                let v58: i32 = (&*v53).as_ref().unwrap().status.code().unwrap_or(-2);
                (true, v58, v57)
            } else {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (false, -1i32, v59)
            };
            let v64: bool = if v60 {
                let v63: bool = v61 == 0i32;
                v63
            } else {
                false
            };
            if v64 {
                US2::US2_Some(v62)
            } else {
                US2::US2_None
            }
        }
        US2::US2_Some(v47) => {
            let v47: Rc<str> = v47.clone();
            v46
        }
    }
}
fn rust_installed_13() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v4));
    let mut v6: US2 = captured_20(v0, v5);
    match v6 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v7) => {
            let mut v7: Rc<str> = v7;
            installed_channels_14(v7)
        }
    }
}
fn first_line_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method5(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3, v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method6(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v14: Rc<str> = v13.borrow()[0i32 as usize].clone();
    let v15: i32 = (v14.clone().len() as i32);
    let v16: i32 = 0i32;
    let v17: i32 = method5(v14.clone(), v15, v16);
    let v18: i32 = v15.wrapping_sub(1i32);
    let v19: Rc<str> = string_slice(&v14, v17 as i64, v18 as i64);
    let v20: i32 = (v19.clone().len() as i32);
    let v21: i32 = method6(v19.clone(), v20);
    let v22: Rc<str> = string_slice(&v19, 0i32 as i64, v21 as i64);
    v22
}
fn method26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 1i32;
    word_at_17(v1, v0)
}
fn version_from_output_24(mut v0: US1, mut v1: Rc<str>) -> US2 {
    let mut v2: Rc<str> = first_line_25(v1);
    let v15: Rc<str> = match v0 {
        US1::US1_ToolElixir => {
            v2
        }
        US1::US1_ToolGleam => {
            method26(v2)
        }
        US1::US1_ToolOtp => {
            v2
        }
        US1::US1_ToolPython => {
            method26(v2)
        }
        US1::US1_ToolRebar => {
            method26(v2)
        }
        US1::US1_ToolRust => {
            let v3: i32 = 0i32;
            let mut v4: Rc<str> = word_at_17(v3, v2);
            without_host_triple_18(v4)
        }
        US1::US1_ToolZig => {
            v2
        }
    };
    let v16: bool = &*v15 == "";
    if v16 {
        US2::US2_None
    } else {
        US2::US2_Some(v15)
    }
}
fn probe_of_27(mut v0: US1) -> (Rc<str>, Rc<UH0>, US2) {
    let (mut v61, mut v62, mut v63): (Rc<str>, Rc<UH0>, US2) = match &v0 {
        US1::US1_ToolRust => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4));
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v5));
            let mut v7: US2 = US2::US2_None;
            (v1, v6, v7)
        }
        _ => {
            let (mut v58, mut v59, mut v60): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                US1::US1_ToolZig => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v10));
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_ZIG"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: US2 = US2::US2_Some(v12);
                    (v8, v11, v13)
                }
                _ => {
                    let (mut v55, mut v56, mut v57): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                        US1::US1_ToolOtp => {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl"); } LIT.with(|lit| lit.clone()) };
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-noshell"); } LIT.with(|lit| lit.clone()) };
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-eval"); } LIT.with(|lit| lit.clone()) };
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("io:format(\"~s\",[element(2,file:read_file(filename:join([code:root_dir(),\"releases\",erlang:system_info(otp_release),\"OTP_VERSION\"])))]),halt()."); } LIT.with(|lit| lit.clone()) };
                            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v18));
                            let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v19));
                            let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v20));
                            let v22: US2 = US2::US2_None;
                            (v14, v21, v22)
                        }
                        _ => {
                            let (mut v52, mut v53, mut v54): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                US1::US1_ToolElixir => {
                                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--short-version"); } LIT.with(|lit| lit.clone()) };
                                    let mut v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v25));
                                    let v27: US2 = US2::US2_None;
                                    (v23, v26, v27)
                                }
                                _ => {
                                    let (mut v49, mut v50, mut v51): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                        US1::US1_ToolGleam => {
                                            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                            let mut v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let v31: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30));
                                            let v32: US2 = US2::US2_None;
                                            (v28, v31, v32)
                                        }
                                        _ => {
                                            let (mut v46, mut v47, mut v48): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                                US1::US1_ToolRebar => {
                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
                                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                                                    let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v35));
                                                    let mut v37: US2 = US2::US2_None;
                                                    (v33, v36, v37)
                                                }
                                                _ => {
                                                    let (mut v43, mut v44, mut v45): (Rc<str>, Rc<UH0>, US2) = match v0 {
                                                        US1::US1_ToolPython => {
                                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
                                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                                            let v40: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                            let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v40));
                                                            let v42: US2 = US2::US2_None;
                                                            (v38, v41, v42)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v43, v44, v45)
                                                }
                                            };
                                            (v46, v47, v48)
                                        }
                                    };
                                    (v49, v50, v51)
                                }
                            };
                            (v52, v53, v54)
                        }
                    };
                    (v55, v56, v57)
                }
            };
            (v58, v59, v60)
        }
    };
    (v61, v62, v63)
}
fn get_environment_variable_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn probe_program_28(mut v0: Rc<str>, mut v1: &Rc<UH0>, mut v2: US2) -> Rc<str> {
    let v15: Rc<str> = match v2 {
        US2::US2_None => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v13
        }
        US2::US2_Some(v3) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = get_environment_variable_29(v3);
            let mut v5: bool = &*v4 == "";
            let v8: US2 = if v5 {
                US2::US2_None
            } else {
                US2::US2_Some(v4)
            };
            match v8 {
                US2::US2_None => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                US2::US2_Some(v9) => {
                    let mut v9: Rc<str> = v9;
                    v9
                }
            }
        }
    };
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = 0i32;
    let v18: i32 = method5(v15.clone(), v16, v17);
    let v19: i32 = v16.wrapping_sub(1i32);
    let v20: Rc<str> = string_slice(&v15.clone(), v18 as i64, v19 as i64);
    let v21: i32 = (v20.clone().len() as i32);
    let v22: i32 = method6(v20.clone(), v21);
    let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
    let v24: bool = &*v23 == "";
    if v24 {
        v0
    } else {
        v15
    }
}
fn probed_version_23(mut v0: US1) -> US2 {
    let (mut v1, mut v2, mut v3): (Rc<str>, Rc<UH0>, US2) = probe_of_27(v0.clone());
    let mut v4: Rc<str> = probe_program_28(v1, &v2, v3);
    let mut v5: US2 = captured_20(v4, v2);
    match v5 {
        US2::US2_None => {
            US2::US2_None
        }
        US2::US2_Some(v6) => {
            let mut v6: Rc<str> = v6;
            version_from_output_24(v0, v6)
        }
    }
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == &*v0;
                if v5 {
                    return US2::US2_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_local_30(mut v0: US2, mut v1: Rc<UH0>, mut v2: US2) -> US2 {
    match v0 {
        US2::US2_None => {
            v2
        }
        US2::US2_Some(v3) => {
            let v3: Rc<str> = v3;
            let mut v4: US2 = method31(v3.clone(), v1);
            let mut v5: bool = match v4 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v6: bool = v5 != true;
            if v6 {
                US2::US2_Some(v3)
            } else {
                US2::US2_None
            }
        }
    }
}
fn local_version_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let mut v2: US0 = tool_of_name_1(v0);
    let mut v13: US2 = match v2 {
        US0::US0_None => {
            US2::US2_None
        }
        US0::US0_Some(v3) => {
            let v3: US1 = v3;
            match &v3 {
                US1::US1_ToolRust => {
                    let mut v4: US2 = repo_toolchain_2(v1);
                    let mut v5: Rc<UH0> = rust_installed_13();
                    let mut v6: US1 = US1::US1_ToolRust;
                    let mut v7: US2 = probed_version_23(v6);
                    rust_local_30(v4, v5, v7)
                }
                _ => {
                    probed_version_23(v3)
                }
            }
        }
    };
    match v13 {
        US2::US2_None => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v15)
        }
        US2::US2_Some(v14) => {
            let mut v14: Rc<str> = v14;
            (true, v14)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        local_version_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method33(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = &*v3 == &*v0;
                if v5 {
                    return US2::US2_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_channel_installed_32(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<UH0> = rust_installed_13();
    let mut v2: US2 = method33(v0, v1);
    let v3: bool = match v2 {
        US2::US2_None => {
            true
        }
        _ => {
            false
        }
    };
    let mut v4: bool = v3 != true;
    v4
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        rust_channel_installed_32(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_ci_local_version(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_ci_rust_installed(v0: &str) -> bool {
    closure2()(Rc::<str>::from(v0))
}
