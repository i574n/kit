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
fn method2(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method3(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn binary_of_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v0 != "kit";
    let v12: bool = if v2 {
        true
    } else {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: i32 = 0i32;
        let v5: i32 = method2(v1.clone(), v3, v4);
        let mut v6: i32 = v3.wrapping_sub(1i32);
        let v7: Rc<str> = string_slice(&v1.clone(), v5 as i64, v6 as i64);
        let v8: i32 = (v7.clone().len() as i32);
        let v9: i32 = method3(v7.clone(), v8);
        let v10: Rc<str> = string_slice(&v7, 0i32 as i64, v9 as i64);
        let v11: bool = &*v10 == "";
        v11
    };
    if v12 {
        v0
    } else {
        let v13: i32 = (v1.clone().len() as i32);
        let v14: i32 = 0i32;
        let v15: i32 = method2(v1.clone(), v13, v14);
        let v16: i32 = v13.wrapping_sub(1i32);
        let v17: Rc<str> = string_slice(&v1, v15 as i64, v16 as i64);
        let v18: i32 = (v17.clone().len() as i32);
        let v19: i32 = method3(v17.clone(), v18);
        let v20: Rc<str> = string_slice(&v17, 0i32 as i64, v19 as i64);
        v20
    }
}
fn machines_folder_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/machines"); } LIT.with(|lit| lit.clone()) }));
    v4
}
fn file_exists_5(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method6(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn machine_binary_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = machines_folder_4(v2.clone());
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".bin"); } LIT.with(|lit| lit.clone()) }));
    let v7: bool = &*v0 != "kit";
    if v7 {
        let v8: Rc<str> = machines_folder_4(v2);
        let v10: Result<(), std::io::Error> = std::fs::create_dir_all(&*v8);
        std::fs::write(&*v6, &*v0).unwrap();
        v0
    } else {
        let v12: bool = file_exists_5(v6.clone());
        let v25: Rc<str> = if v12 {
            let v14: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
            let v16: Vec<u8> = v14.unwrap();
            let v17: Vec<u8> = method6(v16);
            let v19: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v17);
            let v21: std::string::String = v19.unwrap();
            let v23: Rc<str> = Rc::<str>::from(String::as_str(&v21));
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        binary_of_1(v0, v25)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        machine_binary_0(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_machine_binary(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
