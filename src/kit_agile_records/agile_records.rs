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
fn literals_merged_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\" ^^ \""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    v3
}
fn method5(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let mut v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn((char)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> i32> = Rc::new(move |mut v0: (char)| -> i32 {
        let mut v1: char = (v0);
        let mut v2: usize = v1.len_utf8();
        let v3: i32 = ((v2) as i32);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn depth_after_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: bool) -> i32 {
    loop {
        let v5: bool = v2 <= v1;
        if v5 {
            return v3;
        } else {
            let mut v6: usize = ((v1) as usize);
            let v7: Option<char> = (&*v0)[v6..].chars().next();
            let v8: Option<char> = method5(v7);
            let mut v9: Rc<dyn Fn((char)) -> u32> = closure1();
            let mut v10: Option<u32> = v8.map(|x| v9(x));
            let v12: u32 = v10.unwrap_or(0u32);
            let v13: usize = ((v1) as usize);
            let v14: Option<char> = (&*v0)[v13..].chars().next();
            let v15: Option<char> = method5(v14);
            let mut v16: Rc<dyn Fn((char)) -> i32> = closure2();
            let mut v17: Option<i32> = v15.map(|x| v16(x));
            let v19: i32 = v17.unwrap_or(1i32);
            let v20: i32 = v1.wrapping_add(v19);
            if v4 {
                let v21: bool = v12 == 92u32;
                if v21 {
                    let v22: i32 = v20.wrapping_add(1i32);
                    let v23: bool = true;
                    (v1, v4) = (v22, v23);
                    continue;
                } else {
                    let v25: bool = v12 == 34u32;
                    if v25 {
                        let v26: bool = false;
                        (v1, v4) = (v20, v26);
                        continue;
                    } else {
                        let v28: bool = true;
                        (v1, v4) = (v20, v28);
                        continue;
                    }
                }
            } else {
                let v32: bool = v12 == 34u32;
                if v32 {
                    let v33: bool = true;
                    (v1, v4) = (v20, v33);
                    continue;
                } else {
                    let v35: bool = v12 == 40u32;
                    if v35 {
                        let v36: i32 = v3.wrapping_add(1i32);
                        let v37: bool = false;
                        (v1, v3, v4) = (v20, v36, v37);
                        continue;
                    } else {
                        let v39: bool = v12 == 41u32;
                        if v39 {
                            let v40: i32 = v3.wrapping_sub(1i32);
                            let v41: bool = false;
                            (v1, v3, v4) = (v20, v40, v41);
                            continue;
                        } else {
                            let v43: bool = false;
                            (v1, v4) = (v20, v43);
                            continue;
                        }
                    }
                }
            }
        }
    }
}
fn paren_depth_3(mut v0: Rc<str>) -> i32 {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: i32 = 0i32;
    let v4: bool = false;
    depth_after_4(v0, v1, v2, v3, v4)
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: i32 = (v1.clone().len() as i32);
    let v3: i32 = 0i32;
    let v4: i32 = method6(v1.clone(), v2, v3);
    let v5: i32 = v2.wrapping_sub(1i32);
    let mut v6: Rc<str> = string_slice(&v1, v4 as i64, v5 as i64);
    let v7: i32 = (v6.clone().len() as i32);
    let v8: i32 = method7(v6.clone(), v7);
    let mut v9: Rc<str> = string_slice(&v6, 0i32 as i64, v8 as i64);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v0.ends_with(&*v10);
    let v14: bool = if v11 {
        true
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v13: bool = v9.starts_with(&*v12);
        v13
    };
    if v14 {
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v9));
        v15
    } else {
        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
        let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
        v17
    }
}
fn records_into_1(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<RefCell<Vec<Rc<str>>>>) -> () {
    loop {
        let v5: i32 = (v0.clone().borrow().len() as i32);
        let mut v6: bool = v5 <= v1;
        if v6 {
            let v7: bool = &*v2 != "";
            if v7 {
                let v8: Rc<str> = literals_merged_2(v2);
                (v4).borrow_mut().push(v8);
                return ();
            }
        } else {
            let mut v9: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v10: bool = &*v2 == "";
            if v10 {
                let v11: i32 = paren_depth_3(v9.clone());
                let v12: i32 = (v9.clone().len() as i32);
                let v13: i32 = 0i32;
                let v14: i32 = method6(v9.clone(), v12, v13);
                let v15: i32 = v12.wrapping_sub(1i32);
                let mut v16: Rc<str> = string_slice(&v9.clone(), v14 as i64, v15 as i64);
                let mut v17: i32 = (v16.clone().len() as i32);
                let v18: i32 = method7(v16.clone(), v17);
                let v19: Rc<str> = string_slice(&v16, 0i32 as i64, v18 as i64);
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v19.starts_with(&*v20);
                let v30: bool = if v21 {
                    let v22: i32 = 0i32;
                    let v23: i32 = method6(v9.clone(), v12, v22);
                    let v24: Rc<str> = string_slice(&v9.clone(), v23 as i64, v15 as i64);
                    let v25: i32 = (v24.clone().len() as i32);
                    let v26: i32 = method7(v24.clone(), v25);
                    let v27: Rc<str> = string_slice(&v24, 0i32 as i64, v26 as i64);
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let v29: bool = v27.ends_with(&*v28);
                    v29
                } else {
                    false
                };
                let v31: bool = v11 > 0i32;
                let v32: bool = v31 || v30;
                if v32 {
                    let v33: i32 = v1.wrapping_add(1i32);
                    (v1, v2, v3) = (v33, v9, v11);
                    continue;
                } else {
                    (v4).borrow_mut().push(v9);
                    let v34: i32 = v1.wrapping_add(1i32);
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v36: i32 = 0i32;
                    (v1, v2, v3) = (v34, v35, v36);
                    continue;
                }
            } else {
                let v37: Rc<str> = joined_8(v2, v9.clone());
                let v38: i32 = paren_depth_3(v9);
                let v39: i32 = v3.wrapping_add(v38);
                let v40: bool = v39 > 0i32;
                if v40 {
                    let v41: i32 = v1.wrapping_add(1i32);
                    (v1, v2, v3) = (v41, v37, v39);
                    continue;
                } else {
                    let v42: Rc<str> = literals_merged_2(v37);
                    (v4).borrow_mut().push(v42);
                    let v43: i32 = v1.wrapping_add(1i32);
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v45: i32 = 0i32;
                    (v1, v2, v3) = (v43, v44, v45);
                    continue;
                }
            }
        }
        return ();
    }
}
fn records_as_lines_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v4: i32 = 0i32;
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: i32 = 0i32;
    records_into_1(&v3, v4, v5, v6, v1.clone());
    let v7: Rc<str> = Rc::<str>::from((*v1).borrow().iter().map(|x| &**x).collect::<Vec<&str>>().join(&*v2).as_str());
    v7
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        records_as_lines_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_10(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method11(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn records_unchanged_9(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = file_exists_10(v0.clone());
    if v2 {
        let v4: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let mut v6: Vec<u8> = v4.unwrap();
        let v7: Vec<u8> = method11(v6);
        let mut v9: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v7);
        let v11: std::string::String = v9.unwrap();
        let v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
        let v14: Rc<str> = records_as_lines_0(v13);
        let v15: bool = &*v14 == &*v1;
        v15
    } else {
        false
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> bool {
        records_unchanged_9(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_records_as_lines(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_records_unchanged(v0: &str, v1: &str) -> bool {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
