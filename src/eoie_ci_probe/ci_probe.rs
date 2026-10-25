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
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US1 = US1::US1_ToolRust;
        US0::US0_Some(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US1 = US1::US1_ToolZig;
            US0::US0_Some(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("otp"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US1 = US1::US1_ToolOtp;
                US0::US0_Some(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US1 = US1::US1_ToolElixir;
                    US0::US0_Some(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US1 = US1::US1_ToolGleam;
                        US0::US0_Some(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US1 = US1::US1_ToolRebar;
                            US0::US0_Some(v17.clone())
                        } else {
                            let mut v19: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
                            if v19 {
                                let mut v20: US1 = US1::US1_ToolPython;
                                US0::US0_Some(v20.clone())
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
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let mut v3: i32 = method5(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method6(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v10));
    v13.clone()
}
fn channel_line_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US2 {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let mut v3: bool = v1 >= v2;
        if v3 {
            return US2::US2_None;
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method5(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method6(v10.clone(), v11);
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
                let mut v22: Rc<str> = unquoted_7(v21.clone());
                let mut v23: bool = v22.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v23 {
                    return US2::US2_None;
                } else {
                    return US2::US2_Some(v22.clone());
                }
            } else {
                let mut v27: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v27);
                continue;
            }
        }
    }
}
fn toolchain_file_channel_3(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method5(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method6(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[toolchain]"); } LIT.with(|lit| lit.clone()) };
    let mut v13: bool = v11.contains(&*v12);
    if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v14).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v16: i32 = 0i32;
        channel_line_4(v15.clone(), v16)
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
            US2::US2_None
        } else {
            US2::US2_Some(v11.clone())
        }
    }
}
fn method9(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0.clone()
}
fn method8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method9(v13.clone());
    let mut v16: std::path::Display = v14.display();
    let mut v18: std::string::String = format!("{}", v16);
    let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20.clone()
}
fn file_exists_11(mut v0: Rc<str>) -> bool {
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
fn method12(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn text_at_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = file_exists_11(v0.clone());
    if v1 {
        let mut v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method12(v5.clone());
        let mut v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let mut v10: std::string::String = v8.unwrap();
        let mut v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12.clone()
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    }
}
fn repo_toolchain_2(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain.toml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method8(v0.clone(), v1.clone());
    let mut v3: Rc<str> = text_at_10(v2.clone());
    let mut v4: US2 = toolchain_file_channel_3(v3.clone());
    match &v4 {
        US2::US2_None => {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = method8(v0.clone(), v7.clone());
            let mut v9: Rc<str> = text_at_10(v8.clone());
            toolchain_file_channel_3(v9.clone())
        }
        US2::US2_Some(v5) => {
            let mut v5: Rc<str> = v5.clone();
            US2::US2_Some(v5.clone())
        }
    }
}
fn method16(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn word_at_17(mut v0: i32, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: bool = v0 < v4;
    if v5 {
        let mut v6: Rc<str> = v3.clone().borrow()[v0 as usize].clone();
        v6.clone()
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7.clone()
    }
}
fn method19(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
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
    let mut v3: Option<usize> = method19(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 > 0i32;
    let mut v9: i32 = if v8 {
        v7
    } else {
        -1i32
    };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-aarch64-"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Option<usize> = v0.find(&*v10);
    let mut v12: Option<usize> = method19(v11.clone());
    let mut v13: Option<i32> = v12.map(|x| v4(x));
    let mut v15: i32 = v13.unwrap_or(-1i32);
    let mut v16: bool = v15 > 0i32;
    let mut v20: bool = if v16 {
        let mut v17: bool = v9 < 0i32;
        if v17 {
            true
        } else {
            let mut v18: bool = v15 < v9;
            v18
        }
    } else {
        false
    };
    let mut v21: i32 = if v20 {
        v15
    } else {
        v9
    };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-i686-"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Option<usize> = v0.find(&*v22);
    let mut v24: Option<usize> = method19(v23.clone());
    let mut v25: Option<i32> = v24.map(|x| v4(x));
    let mut v27: i32 = v25.unwrap_or(-1i32);
    let mut v28: bool = v27 > 0i32;
    let mut v32: bool = if v28 {
        let mut v29: bool = v21 < 0i32;
        if v29 {
            true
        } else {
            let mut v30: bool = v27 < v21;
            v30
        }
    } else {
        false
    };
    let mut v33: i32 = if v32 {
        v27
    } else {
        v21
    };
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-arm"); } LIT.with(|lit| lit.clone()) };
    let mut v35: Option<usize> = v0.find(&*v34);
    let mut v36: Option<usize> = method19(v35.clone());
    let mut v37: Option<i32> = v36.map(|x| v4(x));
    let mut v39: i32 = v37.unwrap_or(-1i32);
    let mut v40: bool = v39 > 0i32;
    let mut v44: bool = if v40 {
        let mut v41: bool = v33 < 0i32;
        if v41 {
            true
        } else {
            let mut v42: bool = v39 < v33;
            v42
        }
    } else {
        false
    };
    let mut v45: i32 = if v44 {
        v39
    } else {
        v33
    };
    let mut v46: bool = v45 < 0i32;
    if v46 {
        v0.clone()
    } else {
        let mut v47: usize = ((0i32) as usize);
        let mut v48: usize = ((v45) as usize);
        let mut v49: Rc<str> = Rc::<str>::from(&(v0)[v47..v48]);
        v49.clone()
    }
}
fn channels_from_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method16(v2.clone(), v5.clone());
        } else {
            let mut v7: i32 = 0i32;
            let mut v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v9: i32 = (v8.clone().len() as i32);
            let mut v10: i32 = 0i32;
            let mut v11: i32 = method5(v8.clone(), v9, v10);
            let mut v12: i32 = v9.wrapping_sub(1i32);
            let mut v13: Rc<str> = string_slice(&v8.clone(), v11 as i64, v12 as i64);
            let mut v14: i32 = (v13.clone().len() as i32);
            let mut v15: i32 = method6(v13.clone(), v14);
            let mut v16: Rc<str> = string_slice(&v13.clone(), 0i32 as i64, v15 as i64);
            let mut v17: Rc<str> = word_at_17(v7, v16.clone());
            let mut v18: Rc<str> = without_host_triple_18(v17.clone());
            let mut v19: i32 = v1.wrapping_add(1i32);
            let mut v20: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<UH0> = if v20 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v18.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v19, v22.clone());
            continue;
        }
    }
}
fn installed_channels_14(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    channels_from_15(v5.clone(), v6, v7.clone())
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, Rc<str>) {
    let (mut v12, mut v13): (Rc<str>, Rc<str>) = match &*v1 {
        UH0::UH0_Cons(v4, v5) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<UH0> = v5.clone();
            let (mut v6, mut v7): (Rc<str>, Rc<str>) = method21(v0.clone(), v5.clone(), v2.clone(), v3.clone());
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v6));
            (v9.clone(), v0.clone())
        }
        _ => {
            let (mut v10, mut v11): (Rc<str>, Rc<str>) = match &*v1 {
                UH0::UH0_Nil => {
                    (v2.clone(), v3.clone())
                }
                _ => unreachable!(),
            };
            (v10.clone(), v11.clone())
        }
    };
    (v12.clone(), v13.clone())
}
fn captured_20(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cmd"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".bat"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v8, mut v9): (Rc<str>, Rc<str>) = method21(v6.clone(), v1.clone(), v7.clone(), v7.clone());
    let (mut v10, mut v11): (bool, Rc<str>) = match std::process::Command::new(&*v0).args((&*v8).split(&*v6).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
    let mut v14: US2 = if v10 {
        US2::US2_Some(v11.clone())
    } else {
        US2::US2_None
    };
    let mut v25: US2 = match &v14 {
        US2::US2_None => {
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v17, mut v18): (Rc<str>, Rc<str>) = method21(v16.clone(), v1.clone(), v7.clone(), v7.clone());
            let (mut v19, mut v20): (bool, Rc<str>) = match std::process::Command::new(&*v3).args((&*v17).split(&*v16).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v19 {
                US2::US2_Some(v20.clone())
            } else {
                US2::US2_None
            }
        }
        US2::US2_Some(v15) => {
            let mut v15: Rc<str> = v15.clone();
            v14.clone()
        }
    };
    match &v25 {
        US2::US2_None => {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v28, mut v29): (Rc<str>, Rc<str>) = method21(v27.clone(), v1.clone(), v7.clone(), v7.clone());
            let (mut v30, mut v31): (bool, Rc<str>) = match std::process::Command::new(&*v5).args((&*v28).split(&*v27).filter(|a| a.len() > 0)).output() { Ok(out) if out.status.success() => { let mut text = String::from_utf8_lossy(&out.stdout).into_owned(); text.push_str(&String::from_utf8_lossy(&out.stderr)); (true, Rc::<str>::from(text)) } _ => (false, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }) };
            if v30 {
                US2::US2_Some(v31.clone())
            } else {
                US2::US2_None
            }
        }
        US2::US2_Some(v26) => {
            let mut v26: Rc<str> = v26.clone();
            v25.clone()
        }
    }
}
fn rust_installed_13() -> Rc<UH0> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toolchain"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone()));
    let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()));
    let mut v6: US2 = captured_20(v0.clone(), v5.clone());
    match &v6 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v7) => {
            let mut v7: Rc<str> = v7.clone();
            installed_channels_14(v7.clone())
        }
    }
}
fn first_line_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method5(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method6(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v11.split(&*v12).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v14: Rc<str> = v13.clone().borrow()[0i32 as usize].clone();
    let mut v15: i32 = (v14.clone().len() as i32);
    let mut v16: i32 = 0i32;
    let mut v17: i32 = method5(v14.clone(), v15, v16);
    let mut v18: i32 = v15.wrapping_sub(1i32);
    let mut v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
    let mut v20: i32 = (v19.clone().len() as i32);
    let mut v21: i32 = method6(v19.clone(), v20);
    let mut v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
    v22.clone()
}
fn version_from_output_23(mut v0: US1, mut v1: Rc<str>) -> US2 {
    let mut v2: Rc<str> = first_line_24(v1.clone());
    let mut v18: Rc<str> = match &v0 {
        US1::US1_ToolElixir => {
            v2.clone()
        }
        US1::US1_ToolGleam => {
            let mut v6: i32 = 1i32;
            word_at_17(v6, v2.clone())
        }
        US1::US1_ToolOtp => {
            v2.clone()
        }
        US1::US1_ToolPython => {
            let mut v10: i32 = 1i32;
            word_at_17(v10, v2.clone())
        }
        US1::US1_ToolRebar => {
            let mut v8: i32 = 1i32;
            word_at_17(v8, v2.clone())
        }
        US1::US1_ToolRust => {
            let mut v3: i32 = 0i32;
            let mut v4: Rc<str> = word_at_17(v3, v2.clone());
            without_host_triple_18(v4.clone())
        }
        US1::US1_ToolZig => {
            v2.clone()
        }
    };
    let mut v19: bool = v18.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v19 {
        US2::US2_None
    } else {
        US2::US2_Some(v18.clone())
    }
}
fn probe_of_25(mut v0: US1) -> (Rc<str>, Rc<UH0>, US2) {
    let (mut v63, mut v64, mut v65): (Rc<str>, Rc<UH0>, US2) = match &v0 {
        US1::US1_ToolRust => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustup"); } LIT.with(|lit| lit.clone()) };
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("active-toolchain"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v4.clone()));
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v5.clone()));
            let mut v7: US2 = US2::US2_None;
            (v1.clone(), v6.clone(), v7.clone())
        }
        _ => {
            let (mut v60, mut v61, mut v62): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                US1::US1_ToolZig => {
                    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                    let mut v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v10.clone()));
                    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SPIRAL_ZIG"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: US2 = US2::US2_Some(v12.clone());
                    (v8.clone(), v11.clone(), v13.clone())
                }
                _ => {
                    let (mut v57, mut v58, mut v59): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                        US1::US1_ToolOtp => {
                            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("io:format(\"~s\",[element(2,file:read_file(filename:join([code:root_dir(),\"releases\","); } LIT.with(|lit| lit.clone()) };
                            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erlang:system_info(otp_release),\"OTP_VERSION\"])))]),halt()."); } LIT.with(|lit| lit.clone()) };
                            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("io:format(\"~s\",[element(2,file:read_file(filename:join([code:root_dir(),\"releases\",erlang:system_info(otp_release),\"OTP_VERSION\"])))]),halt()."); } LIT.with(|lit| lit.clone()) };
                            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("erl"); } LIT.with(|lit| lit.clone()) };
                            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-noshell"); } LIT.with(|lit| lit.clone()) };
                            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-eval"); } LIT.with(|lit| lit.clone()) };
                            let mut v20: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16.clone(), v20.clone()));
                            let mut v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19.clone(), v21.clone()));
                            let mut v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18.clone(), v22.clone()));
                            let mut v24: US2 = US2::US2_None;
                            (v17.clone(), v23.clone(), v24.clone())
                        }
                        _ => {
                            let (mut v54, mut v55, mut v56): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                US1::US1_ToolElixir => {
                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elixir"); } LIT.with(|lit| lit.clone()) };
                                    let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--short-version"); } LIT.with(|lit| lit.clone()) };
                                    let mut v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let mut v28: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26.clone(), v27.clone()));
                                    let mut v29: US2 = US2::US2_None;
                                    (v25.clone(), v28.clone(), v29.clone())
                                }
                                _ => {
                                    let (mut v51, mut v52, mut v53): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                        US1::US1_ToolGleam => {
                                            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                            let mut v32: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                            let mut v33: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31.clone(), v32.clone()));
                                            let mut v34: US2 = US2::US2_None;
                                            (v30.clone(), v33.clone(), v34.clone())
                                        }
                                        _ => {
                                            let (mut v48, mut v49, mut v50): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                                US1::US1_ToolRebar => {
                                                    let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rebar3"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("version"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v37: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    let mut v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36.clone(), v37.clone()));
                                                    let mut v39: US2 = US2::US2_None;
                                                    (v35.clone(), v38.clone(), v39.clone())
                                                }
                                                _ => {
                                                    let (mut v45, mut v46, mut v47): (Rc<str>, Rc<UH0>, US2) = match &v0 {
                                                        US1::US1_ToolPython => {
                                                            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("python"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--version"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v42: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                            let mut v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41.clone(), v42.clone()));
                                                            let mut v44: US2 = US2::US2_None;
                                                            (v40.clone(), v43.clone(), v44.clone())
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v45.clone(), v46.clone(), v47.clone())
                                                }
                                            };
                                            (v48.clone(), v49.clone(), v50.clone())
                                        }
                                    };
                                    (v51.clone(), v52.clone(), v53.clone())
                                }
                            };
                            (v54.clone(), v55.clone(), v56.clone())
                        }
                    };
                    (v57.clone(), v58.clone(), v59.clone())
                }
            };
            (v60.clone(), v61.clone(), v62.clone())
        }
    };
    (v63.clone(), v64.clone(), v65.clone())
}
fn get_environment_variable_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1.clone()
}
fn probe_program_26(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: US2) -> Rc<str> {
    let mut v15: Rc<str> = match &v2 {
        US2::US2_None => {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v13.clone()
        }
        US2::US2_Some(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: Rc<str> = get_environment_variable_27(v3.clone());
            let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: US2 = if v5 {
                US2::US2_None
            } else {
                US2::US2_Some(v4.clone())
            };
            match &v8 {
                US2::US2_None => {
                    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US2::US2_Some(v9) => {
                    let mut v9: Rc<str> = v9.clone();
                    v9.clone()
                }
            }
        }
    };
    let mut v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = 0i32;
    let mut v18: i32 = method5(v15.clone(), v16, v17);
    let mut v19: i32 = v16.wrapping_sub(1i32);
    let mut v20: Rc<str> = string_slice(&v15.clone(), v18 as i64, v19 as i64);
    let mut v21: i32 = (v20.clone().len() as i32);
    let mut v22: i32 = method6(v20.clone(), v21);
    let mut v23: Rc<str> = string_slice(&v20.clone(), 0i32 as i64, v22 as i64);
    let mut v24: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v24 {
        v0.clone()
    } else {
        v15.clone()
    }
}
fn probed_version_22(mut v0: US1) -> US2 {
    let (mut v1, mut v2, mut v3): (Rc<str>, Rc<UH0>, US2) = probe_of_25(v0.clone());
    let mut v4: Rc<str> = probe_program_26(v1.clone(), v2.clone(), v3.clone());
    let mut v5: US2 = captured_20(v4.clone(), v2.clone());
    match &v5 {
        US2::US2_None => {
            US2::US2_None
        }
        US2::US2_Some(v6) => {
            let mut v6: Rc<str> = v6.clone();
            version_from_output_23(v0.clone(), v6.clone())
        }
    }
}
fn method29(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v3.clone() == v0.clone();
                if v5 {
                    return US2::US2_Some(v3.clone());
                } else {
                    (v0, v1) = (v0.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_local_28(mut v0: US2, mut v1: Rc<UH0>, mut v2: US2) -> US2 {
    match &v0 {
        US2::US2_None => {
            v2.clone()
        }
        US2::US2_Some(v3) => {
            let mut v3: Rc<str> = v3.clone();
            let mut v4: US2 = method29(v3.clone(), v1.clone());
            let mut v5: bool = match &v4 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let mut v6: bool = v5 != true;
            if v6 {
                US2::US2_Some(v3.clone())
            } else {
                US2::US2_None
            }
        }
    }
}
fn local_version_0(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let mut v2: US0 = tool_of_name_1(v0.clone());
    let mut v13: US2 = match &v2 {
        US0::US0_None => {
            US2::US2_None
        }
        US0::US0_Some(v3) => {
            let mut v3: US1 = v3.clone();
            match &v3 {
                US1::US1_ToolRust => {
                    let mut v4: US2 = repo_toolchain_2(v1.clone());
                    let mut v5: Rc<UH0> = rust_installed_13();
                    let mut v6: US1 = US1::US1_ToolRust;
                    let mut v7: US2 = probed_version_22(v6.clone());
                    rust_local_28(v4.clone(), v5.clone(), v7.clone())
                }
                _ => {
                    probed_version_22(v3.clone())
                }
            }
        }
    };
    match &v13 {
        US2::US2_None => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v15.clone())
        }
        US2::US2_Some(v14) => {
            let mut v14: Rc<str> = v14.clone();
            (true, v14.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (bool, Rc<str>) {
        local_version_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method31(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v3.clone() == v0.clone();
                if v5 {
                    return US2::US2_Some(v3.clone());
                } else {
                    (v0, v1) = (v0.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn rust_channel_installed_30(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<UH0> = rust_installed_13();
    let mut v2: US2 = method31(v0.clone(), v1.clone());
    let mut v3: bool = match &v2 {
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
        rust_channel_installed_30(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_ci_local_version(v0: &str, v1: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn eoie_ci_rust_installed(v0: &str) -> bool {
    closure2()(Rc::<str>::from(v0))
}
