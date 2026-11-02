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
    US1_Ok(std::path::PathBuf),
    US1_Error(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(std::path::PathBuf),
    US2_None,
}
struct Mut0 { l0: i32, l1: i32, l2: Rc<RefCell<Vec<Rc<str>>>> }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
fn method1(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
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
fn method2(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
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
fn method6(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn get_current_directory_5() -> Rc<str> {
    let mut v1: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
    let mut v3: std::path::PathBuf = v1.unwrap();
    let mut v4: std::path::PathBuf = method6(v3);
    let mut v6: std::path::Display = v4.display();
    let mut v8: std::string::String = format!("{}", v6);
    let v10: Rc<str> = Rc::<str>::from(String::as_str(&v8));
    v10
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method6(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn method13(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US0> = Rc::new(move |mut v0: (Rc<str>)| -> US0 {
        let mut v1: Rc<str> = (v0);
        US0::US0_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_file_name_12(mut v0: Rc<str>) -> Rc<str> {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: Option<&std::ffi::OsStr> = v6.file_name();
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let mut v12: &std::ffi::OsStr = x;
    let mut v14: std::ffi::OsString = v12.to_os_string();
    let v16: Option<&str> = v14.to_str();
    let v18: &str = v16.unwrap();
    let v20: std::string::String = String::from(v18);
    let v22: Rc<str> = Rc::<str>::from(String::as_str(&v20));
    let v24: bool = true; v22 });
    let mut v26: Option<Rc<str>> = _optionm_map_;
    let mut v27: Option<Rc<str>> = method13(v26.clone());
    let mut v28: Rc<dyn Fn((Rc<str>)) -> US0> = closure1();
    let mut v29: Option<US0> = v27.map(|x| v28(x));
    let mut v31: US0 = US0::US0_None;
    let mut v32: US0 = v31.clone();
    let v33: US0 = v29.unwrap_or(v32);
    match &v33 {
        US0::US0_None => {
            let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v35.clone()
        }
        US0::US0_Some(v34) => {
            let mut v34: Rc<str> = v34.clone();
            v34.clone()
        }
    }
}
fn directory_get_parent_14(mut v0: Rc<str>) -> Option<Rc<str>> {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let mut v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method6(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let v21: bool = true; v19 });
    let v23: Option<Rc<str>> = _optionm_map_;
    v23.clone()
}
fn closure2() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> Rc<str>> = Rc::new(move |mut v0: std::io::Error| -> Rc<str> {
        let mut v1: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v0)).s() });
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15() -> Rc<dyn Fn(std::io::Error) -> Rc<str>> {
    closure2()
}
fn ok_c3() -> Rc<dyn Fn(std::path::PathBuf) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::path::PathBuf) -> US1> = Rc::new(move |mut v0: std::path::PathBuf| -> US1 {
        US1::US1_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16() -> Rc<dyn Fn(std::path::PathBuf) -> US1> {
    ok_c3()
}
fn error_c4() -> Rc<dyn Fn(Rc<str>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US1> = Rc::new(move |mut v0: Rc<str>| -> US1 {
        US1::US1_Error(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn(Rc<str>) -> US1> {
    error_c4()
}
fn run_11(mut v0: Rc<str>, mut v1: Rc<dyn Fn(u8) -> Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>>>, mut v2: u8, mut v3: Rc<str>, mut v4: Rc<str>) -> Result<std::path::PathBuf, std::io::Error> {
    let mut v5: Rc<str> = get_file_name_12(v4.clone());
    let mut v6: Option<Rc<str>> = directory_get_parent_14(v4.clone());
    let mut v7: Option<Rc<str>> = method13(v6.clone());
    let mut v8: Rc<dyn Fn((Rc<str>)) -> US0> = closure1();
    let mut v9: Option<US0> = v7.map(|x| v8(x));
    let v11: US0 = US0::US0_None;
    let mut v12: US0 = v11.clone();
    let mut v13: US0 = v9.unwrap_or(v12);
    let mut v14: bool = v2 >= 11u8;
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.read_link / "); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = Rc::<str>::from(format!("path: {} / n: {} / path': {} / name: {}", v0, v2, v4, v5));
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
        let v19: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v17);
        let v21: Result<std::path::PathBuf, std::io::Error> = Err(v19);
        v21
    } else {
        match &v13 {
            US0::US0_Some(v22) => {
                let v22: Rc<str> = v22.clone();
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v24: bool = v4 != v23 ;
                if v24 {
                    let v25: u8 = v2.wrapping_add(1u8);
                    let mut v26: Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>> = v1(v25);
                    let mut v27: Result<std::path::PathBuf, std::io::Error> = v26(v22.clone());
                    let mut v28: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method15();
                    let v30: Result<std::path::PathBuf, Rc<str>> = v27.map_err(|x| v28(x));
                    let mut v31: Rc<dyn Fn(std::path::PathBuf) -> US1> = method16();
                    let mut v32: Rc<dyn Fn(Rc<str>) -> US1> = method17();
                    let v33: US1 = match v30 { Ok(x) => v31(x), Err(e) => v32(e) };
                    match &v33 {
                        US1::US1_Error(v48) => {
                            let mut v48: Rc<str> = v48.clone();
                            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.read_link / "); } LIT.with(|lit| lit.clone()) };
                            let v50: Rc<str> = Rc::<str>::from(format!("error': {} / error: {} / name: {}", v48, v3, v5));
                            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                            let v53: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v51);
                            let v55: Result<std::path::PathBuf, std::io::Error> = Err(v53);
                            v55
                        }
                        US1::US1_Ok(v34) => {
                            let mut v34: std::path::PathBuf = v34.clone();
                            let mut v35: std::path::PathBuf = method6(v34.clone());
                            let v37: std::path::Display = v35.display();
                            let v38: Rc<str> = Rc::<str>::from(format!("{}", v37));
                            let v39: Rc<str> = method7(v38.clone(), v5.clone());
                            let v41: &str = &*v39;
                            let v43: std::string::String = String::from(v41);
                            let v45: std::path::PathBuf = std::path::PathBuf::from(v43);
                            let v47: Result<std::path::PathBuf, std::io::Error> = Ok(v45);
                            v47
                        }
                    }
                } else {
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.read_link / run / The file or directory is not a reparse point. / "); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<str> = Rc::<str>::from(format!("path: {} / error: {} / path': {} / name: {}", v0, v3, v4, v5));
                    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                    let v62: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v60);
                    let v64: Result<std::path::PathBuf, std::io::Error> = Err(v62);
                    v64
                }
            }
            _ => {
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.read_link / run / The file or directory is not a reparse point. / "); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("path: {} / error: {} / path': {} / name: {}", v0, v3, v4, v5));
                let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
                let v70: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v68);
                let v72: Result<std::path::PathBuf, std::io::Error> = Err(v70);
                v72
            }
        }
    }
}
fn closure6(mut v0: Rc<str>, mut v1: u8) -> Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>> {
    Rc::new(move |mut v2: Rc<str>| -> Result<std::path::PathBuf, std::io::Error> {
        method10(v0.clone(), v1, v2.clone())
    })
}
fn closure5(mut v0: Rc<str>) -> Rc<dyn Fn(u8) -> Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>>> {
    Rc::new(move |mut v1: u8| -> Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>> {
        closure6(v0.clone(), v1)
    })
}
fn method10(mut v0: Rc<str>, mut v1: u8, mut v2: Rc<str>) -> Result<std::path::PathBuf, std::io::Error> {
    let mut v4: Result<std::path::PathBuf, std::io::Error> = std::fs::read_link(&*v2);
    let mut v5: Rc<dyn Fn(std::io::Error) -> Rc<str>> = method15();
    let mut v7: Result<std::path::PathBuf, Rc<str>> = v4.map_err(|x| v5(x));
    let mut v8: Rc<dyn Fn(std::path::PathBuf) -> US1> = method16();
    let mut v9: Rc<dyn Fn(Rc<str>) -> US1> = method17();
    let v10: US1 = match v7 { Ok(x) => v8(x), Err(e) => v9(e) };
    match &v10 {
        US1::US1_Error(v14) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<dyn Fn(u8) -> Rc<dyn Fn(Rc<str>) -> Result<std::path::PathBuf, std::io::Error>>> = closure5(v0.clone());
            run_11(v0.clone(), v15.clone(), v1, v14.clone(), v2.clone())
        }
        US1::US1_Ok(v11) => {
            let v11: std::path::PathBuf = v11.clone();
            let mut v13: Result<std::path::PathBuf, std::io::Error> = Ok(v11);
            v13
        }
    }
}
fn directory_exists_18(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn read_link_9(mut v0: Rc<str>) -> Result<std::path::PathBuf, std::io::Error> {
    let mut v1: bool = directory_exists_18(v0.clone());
    if v1 {
        let mut v3: Result<std::path::PathBuf, std::io::Error> = std::fs::read_link(&*v0);
        v3
    } else {
        let mut v4: u8 = 0u8;
        method10(v0.clone(), v4, v0)
    }
}
fn method19(mut v0: Option<std::path::PathBuf>) -> Option<std::path::PathBuf> {
    v0
}
fn closure7() -> Rc<dyn Fn((std::path::PathBuf)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((std::path::PathBuf)) -> US2> = Rc::new(move |mut v0: (std::path::PathBuf)| -> US2 {
        let mut v1: std::path::PathBuf = (v0);
        US2::US2_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn normalize_path_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = &*v0 == "";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let mut v3: Result<std::path::PathBuf, std::io::Error> = read_link_9(v0.clone());
        let mut v5: Option<std::path::PathBuf> = v3.ok();
        let mut v6: Option<std::path::PathBuf> = method19(v5.clone());
        let mut v7: Rc<dyn Fn((std::path::PathBuf)) -> US2> = closure7();
        let mut v8: Option<US2> = v6.map(|x| v7(x));
        let v10: US2 = US2::US2_None;
        let v11: US2 = v10.clone();
        let mut v12: US2 = v8.unwrap_or(v11);
        let v19: Rc<str> = match &v12 {
            US2::US2_None => {
                v0.clone()
            }
            US2::US2_Some(v13) => {
                let mut v13: std::path::PathBuf = v13.clone();
                let mut v14: std::path::PathBuf = method6(v13.clone());
                let v16: std::path::Display = v14.display();
                let v17: Rc<str> = Rc::<str>::from(format!("{}", v16));
                v17.clone()
            }
        };
        let v20: bool = &*v19 == "";
        let v21: Rc<str> = if v20 {
            v0.clone()
        } else {
            v19.clone()
        };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\?\\"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v21.starts_with(&*v22);
        let v30: Rc<str> = if v23 {
            let v24: i32 = (v21.clone().len() as i32);
            let v25: i32 = v24.wrapping_sub(4i32);
            let mut v26: usize = ((4i32) as usize);
            let mut v27: i32 = 4i32.wrapping_add(v25);
            let mut v28: usize = ((v27) as usize);
            let mut v29: Rc<str> = Rc::<str>::from(&(v21)[v26..v28]);
            v29.clone()
        } else {
            v21.clone()
        };
        let mut v31: i32 = (v30.clone().len() as i32);
        let mut v32: bool = v31 < 2i32;
        if v32 {
            v0.clone()
        } else {
            let v33: Rc<str> = string_slice(&v30.clone(), 0i32 as i64, 0i32 as i64);
            let mut v34: Rc<str> = Rc::<str>::from(v33.to_lowercase());
            let mut v35: i32 = v31.wrapping_sub(1i32);
            let v36: Rc<str> = string_slice(&v30.clone(), 1i32 as i64, v35 as i64);
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v36));
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = Rc::<str>::from(v37.replace(&*v38, &*v39));
            v40.clone()
        }
    }
}
fn method20(mut v0: Rc<RefCell<Mut0>>) -> bool {
    let mut v1: i32 = v0.borrow().l0;
    let v2: bool = v1 > 0i32;
    v2
}
fn method21(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn directory_separator_char_22() -> u8 {
    let v0: u8 = (std::path::MAIN_SEPARATOR as u8);
    v0
}
fn get_full_path_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = get_current_directory_5();
    let v2: Rc<str> = method7(v1.clone(), v0.clone());
    let mut v3: Rc<str> = normalize_path_8(v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let mut v7: i32 = (v5.clone().borrow().len() as i32);
    let mut v8: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v7, l1: 0i32, l2: v6.clone() }));
    while method20(v8.clone()) {
        let v10: i32 = v8.borrow().l0;
        let v11: i32 = v10.wrapping_sub(1i32);
        let (mut v12, mut v13): (i32, Rc<RefCell<Vec<Rc<str>>>>) = (v8.borrow().l1, v8.borrow().l2.clone());
        let mut v14: Rc<str> = v5.clone().borrow()[v11 as usize].clone();
        let v15: bool = ".." == &*v14;
        let (mut v64, mut v65): (i32, Rc<RefCell<Vec<Rc<str>>>>) = if v15 {
            let v16: i32 = v12.wrapping_add(1i32);
            (v16, v13.clone())
        } else {
            let v17: bool = "." == &*v14;
            if v17 {
                (v12, v13.clone())
            } else {
                let v18: bool = 0i32 == v12;
                if v18 {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    let v20: bool = v14.ends_with(&*v19);
                    if v20 {
                        let v21: Rc<str> = string_slice(&v1.clone(), 0i32 as i64, 0i32 as i64);
                        let v22: Rc<str> = string_slice(&v14.clone(), 0i32 as i64, 0i32 as i64);
                        let v23: Rc<str> = Rc::<str>::from(v22.to_lowercase());
                        let v24: Rc<str> = Rc::<str>::from(v21.to_lowercase());
                        let v25: bool = &*v23 == &*v24;
                        let mut v27: Rc<str> = if v25 {
                            v21.clone()
                        } else {
                            let mut v26: Rc<str> = Rc::<str>::from(v22.to_uppercase());
                            v26.clone()
                        };
                        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v19));
                        let mut v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v28.clone()]));
                        let v30: i32 = (v29.clone().borrow().len() as i32);
                        let mut v31: i32 = (v13.clone().borrow().len() as i32);
                        let mut v32: i32 = v30.wrapping_add(v31);
                        let v33: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v32 as usize]));
                        let mut v34: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                        while method21(v32, v34.clone()) {
                            let v36: i32 = v34.borrow().l0;
                            let v37: bool = v36 < v30;
                            let v41: Rc<str> = if v37 {
                                let v38: Rc<str> = v29.clone().borrow()[v36 as usize].clone();
                                v38.clone()
                            } else {
                                let v39: i32 = v36.wrapping_sub(v30);
                                let v40: Rc<str> = v13.clone().borrow()[v39 as usize].clone();
                                v40.clone()
                            };
                            v33.clone().borrow_mut()[v36 as usize] = v41.clone();
                            let v42: i32 = v36.wrapping_add(1i32);
                            v34.borrow_mut().l0 = v42;
                            ()
                        };
                        (0i32, v33.clone())
                    } else {
                        let v43: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v14.clone()]));
                        let v44: i32 = (v43.clone().borrow().len() as i32);
                        let v45: i32 = (v13.clone().borrow().len() as i32);
                        let v46: i32 = v44.wrapping_add(v45);
                        let v47: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v46 as usize]));
                        let mut v48: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                        while method21(v46, v48.clone()) {
                            let v50: i32 = v48.borrow().l0;
                            let v51: bool = v50 < v44;
                            let v55: Rc<str> = if v51 {
                                let v52: Rc<str> = v43.clone().borrow()[v50 as usize].clone();
                                v52.clone()
                            } else {
                                let v53: i32 = v50.wrapping_sub(v44);
                                let v54: Rc<str> = v13.clone().borrow()[v53 as usize].clone();
                                v54.clone()
                            };
                            v47.clone().borrow_mut()[v50 as usize] = v55.clone();
                            let v56: i32 = v50.wrapping_add(1i32);
                            v48.borrow_mut().l0 = v56;
                            ()
                        };
                        (0i32, v47.clone())
                    }
                } else {
                    let v59: i32 = v12.wrapping_sub(1i32);
                    (v59, v13.clone())
                }
            }
        };
        v8.borrow_mut().l0 = v11;
        v8.borrow_mut().l1 = v64;
        v8.borrow_mut().l2 = v65.clone();
        ()
    };
    let (mut v66, mut v67): (i32, Rc<RefCell<Vec<Rc<str>>>>) = (v8.borrow().l1, v8.borrow().l2.clone());
    let v68: Rc<Vec<Rc<str>>> = Rc::new(v67.borrow().clone());
    let v69: u8 = directory_separator_char_22();
    let v70: Rc<str> = Rc::<str>::from((v69 as char).encode_utf8(&mut [0u8; 4]) as &str);
    let v71: Rc<str> = Rc::<str>::from(v68.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(&*v70));
    v71.clone()
}
fn method23(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b'/';
            if v5 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn delete_comparable_path_3(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = get_full_path_4(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    let mut v5: i32 = (v4.clone().len() as i32);
    let mut v6: i32 = method23(v4.clone(), v5);
    let mut v7: Rc<str> = string_slice(&v4, 0i32 as i64, v6 as i64);
    let mut v8: Rc<str> = Rc::<str>::from(v7.to_lowercase());
    v8
}
fn delete_shape_protected_24(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = 0i32 < v3;
    let mut v7: Rc<str> = if v4 {
        let mut v5: Rc<str> = v2.clone().borrow()[0i32 as usize].clone();
        v5.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6.clone()
    };
    let mut v8: bool = &*v7 == "";
    let v18: bool = if v8 {
        let mut v9: bool = v3 == 2i32;
        if v9 {
            true
        } else {
            let v10: bool = v3 == 3i32;
            if v10 {
                let v11: bool = 1i32 < v3;
                let mut v14: Rc<str> = if v11 {
                    let mut v12: Rc<str> = v2.clone().borrow()[1i32 as usize].clone();
                    v12.clone()
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                };
                let v15: bool = &*v14 == "home";
                v15
            } else {
                false
            }
        }
    } else {
        false
    };
    let v19: bool = 3i32 < v3;
    let v22: Rc<str> = if v19 {
        let v20: Rc<str> = v2.clone().borrow()[3i32 as usize].clone();
        v20.clone()
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    };
    let v23: bool = &*v22 == "appdata";
    let v39: bool = if v23 {
        let v24: bool = v3 <= 5i32;
        if v24 {
            true
        } else {
            let v25: bool = v3 == 6i32;
            let mut v31: bool = if v25 {
                let mut v26: bool = 4i32 < v3;
                let mut v29: Rc<str> = if v26 {
                    let mut v27: Rc<str> = v2.clone().borrow()[4i32 as usize].clone();
                    v27.clone()
                } else {
                    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v28.clone()
                };
                let v30: bool = &*v29 == "local";
                v30
            } else {
                false
            };
            if v31 {
                let mut v32: bool = 5i32 < v3;
                let mut v35: Rc<str> = if v32 {
                    let v33: Rc<str> = v2.clone().borrow()[5i32 as usize].clone();
                    v33.clone()
                } else {
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v34.clone()
                };
                let v36: bool = &*v35 == "temp";
                v36
            } else {
                false
            }
        }
    } else {
        false
    };
    let v40: bool = 1i32 < v3;
    let v43: Rc<str> = if v40 {
        let v41: Rc<str> = v2.clone().borrow()[1i32 as usize].clone();
        v41.clone()
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42.clone()
    };
    let v44: bool = &*v43 == "users";
    let v47: bool = if v44 {
        let v45: bool = v3 <= 3i32;
        let v46: bool = v45 || v39;
        v46
    } else {
        false
    };
    let mut v48: bool = v3 <= 1i32;
    let v49: bool = v48 || v18;
    let v50: bool = v49 || v47;
    if v50 {
        true
    } else {
        let v51: bool = v3 == 2i32;
        if v51 {
            let v54: Rc<str> = if v40 {
                let v52: Rc<str> = v2.clone().borrow()[1i32 as usize].clone();
                v52.clone()
            } else {
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v53.clone()
            };
            let v55: bool = &*v54 == "windows";
            v55
        } else {
            false
        }
    }
}
fn get_environment_variable_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn delete_variable_protected_25(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<str> = get_environment_variable_26(v2.clone());
                let mut v5: bool = &*v4 != "";
                let mut v14: bool = if v5 {
                    let mut v6: Rc<str> = delete_comparable_path_3(v4.clone());
                    let mut v7: bool = &*v6 != "";
                    if v7 {
                        let mut v8: bool = &*v0 == &*v6;
                        if v8 {
                            true
                        } else {
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v9));
                            let v11: bool = v6.starts_with(&*v10);
                            v11
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };
                if v14 {
                    return true;
                } else {
                    (v0, v1) = (v0.clone(), v3.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn delete_target_refusal_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let mut v3: i32 = method1(v0.clone(), v1, v2);
    let mut v4: i32 = v1.wrapping_sub(1i32);
    let mut v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let mut v6: i32 = (v5.clone().len() as i32);
    let mut v7: i32 = method2(v5.clone(), v6);
    let mut v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let mut v12: bool = v11.starts_with(&*v10);
    let v18: bool = if v12 {
        true
    } else {
        let mut v13: i32 = (v11.clone().len() as i32);
        let mut v14: bool = v13 >= 3i32;
        if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v11.contains(&*v15);
            v16
        } else {
            false
        }
    };
    let v19: bool = v18 == false;
    if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit refuses to remove a relative path, which resolves against the current folder: "); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v0));
        v21
    } else {
        let v22: Rc<str> = delete_comparable_path_3(v0.clone());
        let v23: bool = &*v0 == "";
        let v25: bool = if v23 {
            true
        } else {
            delete_shape_protected_24(v22.clone())
        };
        let v46: bool = if v25 {
            true
        } else {
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TEMP"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TMP"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TMPDIR"); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SystemRoot"); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ProgramData"); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v35));
            let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33, v36));
            let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v32, v37));
            let v39: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31, v38));
            let v40: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v39));
            let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v40));
            let v42: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v41));
            let v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v42));
            let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v43));
            delete_variable_protected_25(v22, v44)
        };
        if v46 {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("or an ancestor of one): "); } LIT.with(|lit| lit.clone()) };
            let mut v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v0));
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit refuses to remove a protected folder (a drive or system root, a user profile, its AppData or temp, "); } LIT.with(|lit| lit.clone()) }, v48));
            v49
        } else {
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v50
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        delete_target_refusal_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_delete_target_refusal(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
