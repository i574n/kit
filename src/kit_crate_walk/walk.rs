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
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, i32, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>, Rc<str>, i32, Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_SpiralCrate(Rc<str>, Rc<str>),
    US1_NotSpiralCrate,
}
pub struct Mut0 { pub l0: i32 }
fn method2(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method5(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn file_exists_6(mut v0: Rc<str>) -> bool {
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
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let mut v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn quoted_value_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 >= 3i32;
    if v4 {
        let v5: Rc<str> = v2.borrow()[1i32 as usize].clone();
        v5
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    }
}
fn section_fields_10(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v4 <= v1;
        if v5 {
            let mut v6: bool = &*v2 != "";
            let v8: bool = if v6 {
                let mut v7: bool = &*v3 != "";
                v7
            } else {
                false
            };
            if v8 {
                return US1::US1_SpiralCrate(v2, v3);
            } else {
                return US1::US1_NotSpiralCrate;
            }
        } else {
            let mut v12: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v13: i32 = (v12.clone().len() as i32);
            let mut v14: i32 = 0i32;
            let v15: i32 = method11(v12.clone(), v13, v14);
            let v16: i32 = v13.wrapping_sub(1i32);
            let v17: Rc<str> = string_slice(&v12, v15 as i64, v16 as i64);
            let v18: i32 = (v17.clone().len() as i32);
            let v19: i32 = method12(v17.clone(), v18);
            let v20: Rc<str> = string_slice(&v17, 0i32 as i64, v19 as i64);
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
            let v22: bool = v20.starts_with(&*v21);
            if v22 {
                v1 = v4;
                continue;
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("entry"); } LIT.with(|lit| lit.clone()) };
                let v25: bool = v20.starts_with(&*v24);
                if v25 {
                    let v26: i32 = v1.wrapping_add(1i32);
                    let v27: Rc<str> = quoted_value_13(v20);
                    (v1, v2) = (v26, v27);
                    continue;
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("output"); } LIT.with(|lit| lit.clone()) };
                    let v30: bool = v20.starts_with(&*v29);
                    if v30 {
                        let v31: i32 = v1.wrapping_add(1i32);
                        let v32: Rc<str> = quoted_value_13(v20);
                        (v1, v3) = (v31, v32);
                        continue;
                    } else {
                        let v34: i32 = v1.wrapping_add(1i32);
                        v1 = v34;
                        continue;
                    }
                }
            }
        }
    }
}
fn manifest_from_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> US1 {
    loop {
        let mut v2: i32 = (v0.clone().borrow().len() as i32);
        let mut v3: bool = v2 <= v1;
        if v3 {
            return US1::US1_NotSpiralCrate;
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let v8: i32 = method11(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method12(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let mut v14: bool = &*v13 == "[package.metadata.spiral]";
            if v14 {
                let v15: i32 = v1.wrapping_add(1i32);
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return section_fields_10(v0, v15, v16.clone(), v16.clone());
            } else {
                let v18: i32 = v1.wrapping_add(1i32);
                v1 = v18;
                continue;
            }
        }
    }
}
fn manifest_of_8(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    manifest_from_9(v5, v6)
}
fn line_count_14(mut v0: Rc<str>) -> i32 {
    let mut v1: bool = &*v0 == "";
    if v1 {
        0i32
    } else {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v4: i32 = (v3.borrow().len() as i32);
        let v5: bool = v0.ends_with(&*v2);
        if v5 {
            let mut v6: i32 = v4.wrapping_sub(1i32);
            v6
        } else {
            v4
        }
    }
}
fn largest_of_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_crate_largest::kit_crate_largest(&*v0);
    v1
}
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn leaf_name_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Option<usize> = v3.rfind(&*v2);
    let v5: Option<usize> = method17(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let v9: i32 = v7.unwrap_or(-1i32);
    let v10: bool = v9 < 0i32;
    if v10 {
        v3
    } else {
        let mut v11: i32 = v9.wrapping_add(1i32);
        let mut v12: i32 = (v3.clone().len() as i32);
        let v13: i32 = v12.wrapping_sub(v9);
        let mut v14: i32 = v13.wrapping_sub(1i32);
        let v15: usize = ((v11) as usize);
        let v16: i32 = v11.wrapping_add(v14);
        let v17: usize = ((v16) as usize);
        let v18: Rc<str> = Rc::<str>::from(&(v3)[v15..v17]);
        v18
    }
}
fn measured_3(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Cargo.toml"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method4(v0.clone(), v1);
    let mut v3: bool = file_exists_6(v2.clone());
    let mut v4: bool = v3 == false;
    if v4 {
        US0::US0_None
    } else {
        let mut v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v2);
        let v9: Vec<u8> = v7.unwrap();
        let v10: Vec<u8> = method7(v9);
        let mut v12: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v10);
        let mut v14: std::string::String = v12.unwrap();
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v17: US1 = manifest_of_8(v16);
        match v17 {
            US1::US1_NotSpiralCrate => {
                US0::US0_None
            }
            US1::US1_SpiralCrate(v19, v20) => {
                let v19: Rc<str> = v19;
                let v20: Rc<str> = v20;
                let v21: Rc<str> = method4(v0.clone(), v20.clone());
                let v22: bool = file_exists_6(v21.clone());
                let v35: Rc<str> = if v22 {
                    let v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v21);
                    let v26: Vec<u8> = v24.unwrap();
                    let v27: Vec<u8> = method7(v26);
                    let v29: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v27);
                    let v31: std::string::String = v29.unwrap();
                    let v33: Rc<str> = Rc::<str>::from(String::as_str(&v31));
                    v33
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v34
                };
                let v37: i32 = if v22 {
                    line_count_14(v35.clone())
                } else {
                    -1i32
                };
                let v38: bool = v37 > 0i32;
                let v41: Rc<str> = if v38 {
                    largest_of_15(v35)
                } else {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v40
                };
                let v42: Rc<str> = leaf_name_16(v0);
                US0::US0_Some(v42, v20, v37, v41)
            }
        }
    }
}
fn method19(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
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
fn directory_names_18(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v4: Vec<std::string::String> = v2.unwrap();
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: bool = true; let _vec_fold_ = v4.into_iter().fold(v5, |acc, x| { //;
    let v9: Rc<UH0> = acc;
    let mut v11: std::string::String = x;
    let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v9));
    let v16: bool = true; v15 });
    let v18: Rc<UH0> = _vec_fold_;
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method19(v18, v19)
}
fn directory_exists_21(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
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
                    let v9: bool = &*v3 == "node_modules";
                    v9
                };
                let mut v12: bool = if v10 {
                    true
                } else {
                    let mut v11: bool = &*v3 == "_build";
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
                    let v15: bool = &*v3 == "lab";
                    v15
                };
                let v17: bool = v16 == false;
                let v20: bool = if v17 {
                    let v18: Rc<str> = method4(v0.clone(), v3.clone());
                    directory_exists_21(v18)
                } else {
                    false
                };
                let v23: Rc<UH0> = if v20 {
                    let v21: Rc<str> = method4(v0.clone(), v3);
                    Rc::new(UH0::UH0_Cons(v21, v2))
                } else {
                    v2
                };
                (v1, v2) = (v4, v23);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn walked_1(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let mut v6: US0 = measured_3(v4.clone());
                let v13: Rc<UH1> = match v6 {
                    US0::US0_None => {
                        v1
                    }
                    US0::US0_Some(v7, v8, v9, v10) => {
                        let mut v7: Rc<str> = v7;
                        let v8: Rc<str> = v8;
                        let v9: i32 = v9;
                        let v10: Rc<str> = v10;
                        Rc::new(UH1::UH1_Cons(v7, v8, v9, v10, v1))
                    }
                };
                let mut v14: Rc<UH0> = directory_names_18(v4.clone());
                let v15: Rc<UH0> = method20(v4, v14, v5);
                (v0, v1) = (v15, v13);
                continue;
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                return method2(v1, v2);
            }
        }
    }
}
fn method23(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn method22(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let mut v3: Rc<str> = v3;
                let mut v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH1> = v6;
                let mut v7: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v2, v3, v4, v5)]));
                let v8: i32 = (v1.clone().borrow().len() as i32);
                let v9: i32 = (v7.clone().borrow().len() as i32);
                let v10: i32 = v8.wrapping_add(v9);
                let mut v11: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>, i32, Rc<str>)>::default(); v10 as usize]));
                let mut v12: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method23(v10, &v12) {
                    let mut v14: i32 = v12.borrow().l0;
                    let v15: bool = v14 < v8;
                    let (mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, i32, Rc<str>) = if v15 {
                        let (mut v16, mut v17, mut v18, mut v19): (Rc<str>, Rc<str>, i32, Rc<str>) = v1.borrow()[v14 as usize].clone();
                        (v16, v17, v18, v19)
                    } else {
                        let v20: i32 = v14.wrapping_sub(v8);
                        let (mut v21, mut v22, mut v23, mut v24): (Rc<str>, Rc<str>, i32, Rc<str>) = v7.borrow()[v20 as usize].clone();
                        (v21, v22, v23, v24)
                    };
                    v11.borrow_mut()[v14 as usize] = (v25, v26, v27, v28);
                    let v29: i32 = v14.wrapping_add(1i32);
                    v12.borrow_mut().l0 = v29;
                    ()
                };
                (v0, v1) = (v6, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn crate_measures_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v1));
    let mut v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = walked_1(v2, v3);
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
    let mut v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = method22(v4, v5);
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> {
        crate_measures_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_crate_measures(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub type SpiralShape_2617b4e49837 = Mut0;
