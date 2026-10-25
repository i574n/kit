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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, i32),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_SpiralCrate(Rc<str>, Rc<str>),
    US2_NotSpiralCrate,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_CrateWithin,
    US3_CrateOver,
    US3_CrateUnbuilt,
}
#[derive(Clone)]
enum US4 {
    US4_GatePassed,
    US4_GateFailedOver(i32),
    US4_GateFailedMissing(i32),
    US4_GateFailedVacuous,
}
fn command_line_argument_count_1() -> i32 {
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
fn argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_1();
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
fn value_after_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v2 <= v3;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_2(v1);
            let mut v7: bool = v6.clone() == v0.clone();
            if v7 {
                return argument_2(v3);
            } else {
                (v0, v1, v2) = (v0.clone(), v3, v2);
                continue;
            }
        }
    }
}
fn method6(mut v0: Option<char>) -> Option<char> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let mut v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn digits_value_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: usize = ((v1) as usize);
            let mut v6: Option<char> = (&*v0)[v5..].chars().next();
            let mut v7: Option<char> = method6(v6.clone());
            let mut v8: Rc<dyn Fn((char)) -> u32> = closure1();
            let mut v9: Option<u32> = v7.map(|x| v8(x));
            let mut v11: u32 = v9.unwrap_or(0u32);
            let mut v12: bool = 48u32 <= v11;
            let mut v14: bool = if v12 {
                let mut v13: bool = v11 <= 57u32;
                v13
            } else {
                false
            };
            if v14 {
                let mut v15: i32 = v1.wrapping_add(1i32);
                let mut v16: i32 = v2.wrapping_mul(10i32);
                let mut v17: u32 = v11.wrapping_sub(48u32);
                let mut v18: i32 = (v17 as i32);
                let mut v19: i32 = v16.wrapping_add(v18);
                (v0, v1, v2) = (v0.clone(), v15, v19);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn directory_exists_7(mut v0: Rc<str>) -> bool {
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
fn method11(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0.clone()
}
fn method10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method11(v13.clone());
    let mut v16: std::path::Display = v14.display();
    let mut v18: std::string::String = format!("{}", v16);
    let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20.clone()
}
fn file_exists_12(mut v0: Rc<str>) -> bool {
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
fn method13(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn method17(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method18(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn quoted_value_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 >= 3i32;
    if v4 {
        let mut v5: Rc<str> = v2.clone().borrow()[1i32 as usize].clone();
        v5.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6.clone()
    }
}
fn section_fields_16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> US2 {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let mut v5: bool = v4 <= v1;
        if v5 {
            let mut v6: bool = v2.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = if v6 {
                let mut v7: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v7
            } else {
                false
            };
            if v8 {
                return US2::US2_SpiralCrate(v2.clone(), v3.clone());
            } else {
                return US2::US2_NotSpiralCrate;
            }
        } else {
            let mut v12: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v13: i32 = (v12.clone().len() as i32);
            let mut v14: i32 = 0i32;
            let mut v15: i32 = method17(v12.clone(), v13, v14);
            let mut v16: i32 = v13.wrapping_sub(1i32);
            let mut v17: Rc<str> = string_slice(&v12.clone(), v15 as i64, v16 as i64);
            let mut v18: i32 = (v17.clone().len() as i32);
            let mut v19: i32 = method18(v17.clone(), v18);
            let mut v20: Rc<str> = string_slice(&v17.clone(), 0i32 as i64, v19 as i64);
            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
            let mut v22: bool = v20.starts_with(&*v21);
            if v22 {
                (v0, v1, v2, v3) = (v0.clone(), v4, v2.clone(), v3.clone());
                continue;
            } else {
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("entry"); } LIT.with(|lit| lit.clone()) };
                let mut v25: bool = v20.starts_with(&*v24);
                if v25 {
                    let mut v26: i32 = v1.wrapping_add(1i32);
                    let mut v27: Rc<str> = quoted_value_19(v20.clone());
                    (v0, v1, v2, v3) = (v0.clone(), v26, v27.clone(), v3.clone());
                    continue;
                } else {
                    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output"); } LIT.with(|lit| lit.clone()) };
                    let mut v30: bool = v20.starts_with(&*v29);
                    if v30 {
                        let mut v31: i32 = v1.wrapping_add(1i32);
                        let mut v32: Rc<str> = quoted_value_19(v20.clone());
                        (v0, v1, v2, v3) = (v0.clone(), v31, v2.clone(), v32.clone());
                        continue;
                    } else {
                        let mut v34: i32 = v1.wrapping_add(1i32);
                        (v0, v1, v2, v3) = (v0.clone(), v34, v2.clone(), v3.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn manifest_from_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US2 {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let mut v3: bool = v2 <= v1;
        if v3 {
            return US2::US2_NotSpiralCrate;
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method17(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method18(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("[package.metadata.spiral]"); } LIT.with(|lit| lit.clone()) };
            if v14 {
                let mut v15: i32 = v1.wrapping_add(1i32);
                let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return section_fields_16(v0.clone(), v15, v16.clone(), v16.clone());
            } else {
                let mut v18: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v18);
                continue;
            }
        }
    }
}
fn manifest_of_14(mut v0: Rc<str>) -> US2 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    manifest_from_15(v5.clone(), v6)
}
fn line_count_20(mut v0: Rc<str>) -> i32 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v1 {
        0i32
    } else {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v4: i32 = (v3.clone().borrow().len() as i32);
        let mut v5: bool = v0.ends_with(&*v2);
        if v5 {
            let mut v6: i32 = v4.wrapping_sub(1i32);
            v6
        } else {
            v4
        }
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn leaf_name_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Option<usize> = v3.rfind(&*v2);
    let mut v5: Option<usize> = method22(v4.clone());
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let mut v9: i32 = v7.unwrap_or(-1i32);
    let mut v10: bool = v9 < 0i32;
    if v10 {
        v3.clone()
    } else {
        let mut v11: i32 = v9.wrapping_add(1i32);
        let mut v12: i32 = (v3.clone().len() as i32);
        let mut v13: i32 = v12.wrapping_sub(v9);
        let mut v14: i32 = v13.wrapping_sub(1i32);
        let mut v15: usize = ((v11) as usize);
        let mut v16: i32 = v11.wrapping_add(v14);
        let mut v17: usize = ((v16) as usize);
        let mut v18: Rc<str> = Rc::<str>::from(&(v3)[v15..v17]);
        v18.clone()
    }
}
fn measured_9(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method10(v0.clone(), v1.clone());
    let mut v3: bool = file_exists_12(v2.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        US1::US1_None
    } else {
        let mut v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v2);
        let mut v9: Vec<u8> = v7.unwrap();
        let mut v10: Vec<u8> = method13(v9.clone());
        let mut v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
        let mut v14: std::string::String = v12.unwrap();
        let mut v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let mut v17: US2 = manifest_of_14(v16.clone());
        match &v17 {
            US2::US2_NotSpiralCrate => {
                US1::US1_None
            }
            US2::US2_SpiralCrate(v19, v20) => {
                let mut v19: Rc<str> = v19.clone();
                let mut v20: Rc<str> = v20.clone();
                let mut v21: Rc<str> = method10(v0.clone(), v20.clone());
                let mut v22: bool = file_exists_12(v21.clone());
                let mut v35: i32 = if v22 {
                    let mut v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v21);
                    let mut v26: Vec<u8> = v24.unwrap();
                    let mut v27: Vec<u8> = method13(v26.clone());
                    let mut v29: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v27);
                    let mut v31: std::string::String = v29.unwrap();
                    let mut v33: Rc<str> = Rc::<str>::from(String::as_str(&v31));
                    line_count_20(v33.clone())
                } else {
                    -1i32
                };
                let mut v36: Rc<str> = leaf_name_21(v0.clone());
                US1::US1_Some(v36.clone(), v20.clone(), v35)
            }
        }
    }
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn dir_names_23(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let mut v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let mut v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let mut v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v9.clone()));
    let mut v16: bool = true; v15 });
    let mut v18: Rc<UH0> = _vec_fold_;
    let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method24(v18.clone(), v19.clone())
}
fn method25(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                let mut v6: bool = v3.starts_with(&*v5);
                let mut v8: bool = if v6 {
                    true
                } else {
                    let mut v7: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) };
                    v7
                };
                let mut v10: bool = if v8 {
                    true
                } else {
                    let mut v9: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node_modules"); } LIT.with(|lit| lit.clone()) };
                    v9
                };
                let mut v12: bool = if v10 {
                    true
                } else {
                    let mut v11: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_build"); } LIT.with(|lit| lit.clone()) };
                    v11
                };
                let mut v14: bool = if v12 {
                    true
                } else {
                    let mut v13: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
                    v13
                };
                let mut v16: bool = if v14 {
                    true
                } else {
                    let mut v15: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lab"); } LIT.with(|lit| lit.clone()) };
                    v15
                };
                let mut v17: bool = v16 == false;
                let mut v20: bool = if v17 {
                    let mut v18: Rc<str> = method10(v0.clone(), v3.clone());
                    directory_exists_7(v18.clone())
                } else {
                    false
                };
                let mut v23: Rc<UH0> = if v20 {
                    let mut v21: Rc<str> = method10(v0.clone(), v3.clone());
                    Rc::new(UH0::UH0_Cons(v21.clone(), v2.clone()))
                } else {
                    v2.clone()
                };
                (v0, v1, v2) = (v0.clone(), v4.clone(), v23.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: i32 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v3.clone(), v4, v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn walked_8(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: US1 = measured_9(v4.clone());
                let mut v12: Rc<UH1> = match &v6 {
                    US1::US1_None => {
                        v1.clone()
                    }
                    US1::US1_Some(v7, v8, v9) => {
                        let mut v7: Rc<str> = v7.clone();
                        let mut v8: Rc<str> = v8.clone();
                        let mut v9: i32 = *v9;
                        Rc::new(UH1::UH1_Cons(v7.clone(), v8.clone(), v9, v1.clone()))
                    }
                };
                let mut v13: Rc<UH0> = dir_names_23(v4.clone());
                let mut v14: Rc<UH0> = method25(v4.clone(), v13.clone(), v5.clone());
                (v0, v1) = (v14.clone(), v12.clone());
                continue;
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method26(v1.clone(), v2.clone());
            }
        }
    }
}
fn method30(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_29(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method30(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn verdict_of_32(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> US3 {
    let mut v4: bool = v3 < 0i32;
    if v4 {
        US3::US3_CrateUnbuilt
    } else {
        let mut v6: bool = v3 > v0;
        if v6 {
            US3::US3_CrateOver
        } else {
            US3::US3_CrateWithin
        }
    }
}
fn verdict_text_31(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let mut v4: US3 = verdict_of_32(v0, v1.clone(), v2.clone(), v3);
    match &v4 {
        US3::US3_CrateOver => {
            let mut v6: Rc<str> = format_real_29(v0);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("OVER "); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
            v8.clone()
        }
        US3::US3_CrateUnbuilt => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no generated output yet"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US3::US3_CrateWithin => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    }
}
fn measure_line_28(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate / "); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v2));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v6));
    let mut v10: Rc<str> = format_real_29(v3);
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = verdict_text_31(v0, v1.clone(), v2.clone(), v3);
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    v15.clone()
}
fn lines_printed_27(mut v0: i32, mut v1: Rc<UH1>) -> () {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: i32 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: Rc<str> = measure_line_28(v0, v2.clone(), v3.clone(), v4);
                println!("{}", v6);
                (v0, v1) = (v0, v5.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method34(mut v0: i32, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: i32 = *v5;
                let mut v6: Rc<UH1> = v6.clone();
                let mut v7: US3 = verdict_of_32(v0, v3.clone(), v4.clone(), v5);
                let mut v9: i32 = match &v7 {
                    US3::US3_CrateOver => {
                        let mut v8: i32 = v2.wrapping_add(1i32);
                        v8
                    }
                    _ => {
                        v2
                    }
                };
                (v0, v1, v2) = (v0, v6.clone(), v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn over_count_33(mut v0: i32, mut v1: Rc<UH1>) -> i32 {
    let mut v2: i32 = 0i32;
    method34(v0, v1.clone(), v2)
}
fn method35(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let mut v4: i32 = *v4;
                let mut v5: Rc<UH1> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn flag_given_36(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v2 <= v1;
        if v3 {
            return false;
        } else {
            let mut v4: Rc<str> = argument_2(v1);
            let mut v5: bool = v4.clone() == v0.clone();
            if v5 {
                return true;
            } else {
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        }
    }
}
fn method39(mut v0: i32, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let mut v5: i32 = *v5;
                let mut v6: Rc<UH1> = v6.clone();
                let mut v7: US3 = verdict_of_32(v0, v3.clone(), v4.clone(), v5);
                let mut v9: i32 = match &v7 {
                    US3::US3_CrateUnbuilt => {
                        let mut v8: i32 = v2.wrapping_add(1i32);
                        v8
                    }
                    _ => {
                        v2
                    }
                };
                (v0, v1, v2) = (v0, v6.clone(), v9);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn missing_count_38(mut v0: i32, mut v1: Rc<UH1>) -> i32 {
    let mut v2: i32 = 0i32;
    method39(v0, v1.clone(), v2)
}
fn gate_outcome_of_37(mut v0: bool, mut v1: i32, mut v2: Rc<UH1>) -> US4 {
    let mut v3: i32 = over_count_33(v1, v2.clone());
    let mut v4: i32 = missing_count_38(v1, v2.clone());
    let mut v8: bool = if v0 {
        let mut v5: i32 = 0i32;
        let mut v6: i32 = method35(v2.clone(), v5);
        let mut v7: bool = v6 == 0i32;
        v7
    } else {
        false
    };
    if v8 {
        US4::US4_GateFailedVacuous
    } else {
        let mut v11: bool = if v0 {
            let mut v10: bool = v3 > 0i32;
            v10
        } else {
            false
        };
        if v11 {
            US4::US4_GateFailedOver(v3)
        } else {
            let mut v14: bool = if v0 {
                let mut v13: bool = v4 > 0i32;
                v13
            } else {
                false
            };
            if v14 {
                US4::US4_GateFailedMissing(v4)
            } else {
                US4::US4_GatePassed
            }
        }
    }
}
fn gate_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_1();
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = argument_2(v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--limit"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = 3i32;
    let mut v5: Rc<str> = value_after_4(v3.clone(), v4, v0);
    let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v10: i32 = if v6 {
        1000i32
    } else {
        let mut v7: i32 = 0i32;
        let mut v8: i32 = 0i32;
        digits_value_5(v5.clone(), v7, v8)
    };
    let mut v11: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: bool = if v11 {
        true
    } else {
        let mut v12: bool = directory_exists_7(v2.clone());
        let mut v13: bool = v12 == false;
        v13
    };
    if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: crate usage: crate gate <root> [--enforce] [--limit n] (generated output of every Cargo.toml with [package.metadata.spiral] under root)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v15);
        2i32
    } else {
        let mut v16: bool = v10 <= 0i32;
        if v16 {
            let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: crate gate: --limit expects a positive whole number, got "); } LIT.with(|lit| lit.clone()) };
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v5));
            println!("{}", v18);
            2i32
        } else {
            let mut v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v19.clone()));
            let mut v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let mut v22: Rc<UH1> = walked_8(v20.clone(), v21.clone());
            lines_printed_27(v10, v22.clone());
            let mut v23: i32 = over_count_33(v10, v22.clone());
            let mut v24: i32 = 0i32;
            let mut v25: i32 = method35(v22.clone(), v24);
            let mut v26: Rc<str> = format_real_29(v25);
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crates="); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" over="); } LIT.with(|lit| lit.clone()) };
            let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let mut v31: Rc<str> = format_real_29(v23);
            let mut v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" limit="); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
            let mut v35: Rc<str> = format_real_29(v10);
            let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" root="); } LIT.with(|lit| lit.clone()) };
            let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v2));
            let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--enforce"); } LIT.with(|lit| lit.clone()) };
            let mut v41: i32 = 3i32;
            let mut v42: bool = flag_given_36(v40.clone(), v41, v0);
            let mut v43: US4 = gate_outcome_of_37(v42, v10, v22.clone());
            match &v43 {
                US4::US4_GateFailedMissing(v53) => {
                    let mut v53: i32 = *v53;
                    let mut v54: Rc<str> = format_real_29(v53);
                    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie crate gate refused: "); } LIT.with(|lit| lit.clone()) };
                    let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v54));
                    let mut v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crate(s) have no generated output, so generation did not run or failed;"); } LIT.with(|lit| lit.clone()) };
                    let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
                    let mut v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v39));
                    println!("{}", v59);
                    1i32
                }
                US4::US4_GateFailedOver(v46) => {
                    let mut v46: i32 = *v46;
                    let mut v47: Rc<str> = format_real_29(v46);
                    let mut v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie crate gate refused: "); } LIT.with(|lit| lit.clone()) };
                    let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v47));
                    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crate(s) over the limit;"); } LIT.with(|lit| lit.clone()) };
                    let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                    let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v39));
                    println!("{}", v52);
                    1i32
                }
                US4::US4_GateFailedVacuous => {
                    let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie crate gate refused: no crate with [package.metadata.spiral] under the root, so nothing was measured;"); } LIT.with(|lit| lit.clone()) };
                    let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v39));
                    println!("{}", v61);
                    1i32
                }
                US4::US4_GatePassed => {
                    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie crate gate ok"); } LIT.with(|lit| lit.clone()) };
                    let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v39));
                    println!("{}", v45);
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_crate_gate_run() -> i32 {
    closure0()()
}
