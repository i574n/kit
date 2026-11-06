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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_TrainPending,
    US0_TrainPassed,
    US0_TrainTimedOut,
    US0_TrainFailed,
}
#[derive(Clone)]
enum US1 {
    US1_SeenWritten(u64),
    US1_SeenPending,
    US1_SeenShared,
}
fn method4(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_3(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn train_progress_text_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> Rc<str> {
    let mut v5: i32 = v2.wrapping_add(v3);
    let mut v6: i32 = v5.wrapping_add(v4);
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release train "); } LIT.with(|lit| lit.clone()) }, v0));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
    let mut v9: Rc<str> = format_real_3(v6);
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = format_real_3(v1);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notebooks done, "); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = format_real_3(v2);
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pass, "); } LIT.with(|lit| lit.clone()) }));
    let v18: Rc<str> = format_real_3(v3);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timed out (a starved run, retried), "); } LIT.with(|lit| lit.clone()) }));
    let mut v21: Rc<str> = format_real_3(v4);
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failed"); } LIT.with(|lit| lit.clone()) }));
    v23
}
fn method6(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method8(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn train_name_of_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -Name "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method6(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release-train.ps1"); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v0.contains(&*v8);
    let mut v10: bool = v9 == false;
    let v12: bool = if v10 {
        true
    } else {
        let mut v11: bool = v7 < 0i32;
        v11
    };
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let mut v14: i32 = v7.wrapping_add(7i32);
        let v15: i32 = (v0.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(v7);
        let mut v17: i32 = v16.wrapping_sub(7i32);
        let v18: usize = ((v14) as usize);
        let v19: i32 = v14.wrapping_add(v17);
        let v20: usize = ((v19) as usize);
        let mut v21: Rc<str> = Rc::<str>::from(&(v0)[v18..v20]);
        let mut v22: i32 = (v21.clone().len() as i32);
        let v23: i32 = 0i32;
        let v24: i32 = method7(v21.clone(), v22, v23);
        let v25: i32 = v22.wrapping_sub(1i32);
        let v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
        let v27: i32 = (v26.clone().len() as i32);
        let v28: i32 = method8(v26.clone(), v27);
        let v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v31: Option<usize> = v29.find(&*v30);
        let v32: Option<usize> = method6(v31);
        let mut v33: Option<i32> = v32.map(|x| v4(x));
        let v35: i32 = v33.unwrap_or(-1i32);
        let v36: bool = v35 < 0i32;
        let v40: Rc<str> = if v36 {
            v29
        } else {
            let v37: usize = ((0i32) as usize);
            let v38: usize = ((v35) as usize);
            let v39: Rc<str> = Rc::<str>::from(&(v29)[v37..v38]);
            v39
        };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(v40.replace(&*v41, &*v42));
        v43
    }
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn train_root_of_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/scripts/release-train.ps1"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Option<usize> = v6.find(&*v7);
    let mut v9: Option<usize> = method6(v8);
    let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v11: Option<i32> = v9.map(|x| v10(x));
    let v13: i32 = v11.unwrap_or(-1i32);
    let mut v14: bool = v13 < 0i32;
    if v14 {
        v2
    } else {
        let v15: usize = ((0i32) as usize);
        let v16: usize = ((v13) as usize);
        let mut v17: Rc<str> = Rc::<str>::from(&(v6)[v15..v16]);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v19: Option<usize> = v17.rfind(&*v18);
        let v20: Option<usize> = method6(v19);
        let mut v21: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v22: Option<i32> = v20.map(|x| v21(x));
        let v24: i32 = v22.unwrap_or(-1i32);
        let v25: i32 = v24.wrapping_add(1i32);
        let v26: i32 = (v17.clone().len() as i32);
        let v27: i32 = v26.wrapping_sub(v24);
        let v28: i32 = v27.wrapping_sub(1i32);
        let v29: usize = ((v25) as usize);
        let v30: i32 = v25.wrapping_add(v28);
        let v31: usize = ((v30) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v17)[v29..v31]);
        v32
    }
}
fn method11(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let mut v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method11(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
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
    v0
}
fn log_name_of_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".log"); } LIT.with(|lit| lit.clone()) }));
    v6
}
fn logs_from_14(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: Rc<str> = v1.borrow()[v2 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method7(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5, v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method8(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10, 0i32 as i64, v12 as i64);
            let mut v14: Rc<str> = log_name_of_15(v13.clone());
            let v15: Rc<str> = method10(v0.clone(), v14);
            let v16: bool = &*v13 != "";
            let v18: bool = if v16 {
                file_exists_12(v15.clone())
            } else {
                false
            };
            let v31: Rc<str> = if v18 {
                let v20: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v15);
                let mut v22: Vec<u8> = v20.unwrap();
                let v23: Vec<u8> = method13(v22);
                let v25: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v23);
                let v27: std::string::String = v25.unwrap();
                let v29: Rc<str> = Rc::<str>::from(String::as_str(&v27));
                v29
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v30
            };
            let v32: i32 = v2.wrapping_sub(1i32);
            let mut v33: bool = &*v13 == "";
            let v35: Rc<UH0> = if v33 {
                v3
            } else {
                Rc::new(UH0::UH0_Cons(v31, v3))
            };
            (v2, v3) = (v32, v35);
            continue;
        }
    }
}
fn train_verdict_of_18(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Spiral cell timed out"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    if v2 {
        US0::US0_TrainTimedOut
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-tests total"); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.contains(&*v4);
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"failed\":0,"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = v0.contains(&*v6);
            if v7 {
                US0::US0_TrainPassed
            } else {
                US0::US0_TrainFailed
            }
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("** (Mix)"); } LIT.with(|lit| lit.clone()) };
            let v12: bool = v0.contains(&*v11);
            if v12 {
                US0::US0_TrainFailed
            } else {
                US0::US0_TrainPending
            }
        }
    }
}
fn train_counted_19(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: US0) -> (i32, i32, i32, i32) {
    let mut v5: i32 = v0.wrapping_add(1i32);
    let (mut v21, mut v22, mut v23, mut v24): (i32, i32, i32, i32) = match &v4 {
        US0::US0_TrainPending => {
            (v5, v1, v2, v3)
        }
        _ => {
            let (mut v17, mut v18, mut v19, mut v20): (i32, i32, i32, i32) = match &v4 {
                US0::US0_TrainPassed => {
                    let mut v6: i32 = v1.wrapping_add(1i32);
                    (v5, v6, v2, v3)
                }
                _ => {
                    let (mut v13, mut v14, mut v15, mut v16): (i32, i32, i32, i32) = match &v4 {
                        US0::US0_TrainTimedOut => {
                            let mut v7: i32 = v2.wrapping_add(1i32);
                            (v5, v1, v7, v3)
                        }
                        _ => {
                            let (mut v9, mut v10, mut v11, mut v12): (i32, i32, i32, i32) = match v4 {
                                US0::US0_TrainFailed => {
                                    let mut v8: i32 = v3.wrapping_add(1i32);
                                    (v5, v1, v2, v8)
                                }
                                _ => unreachable!(),
                            };
                            (v9, v10, v11, v12)
                        }
                    };
                    (v13, v14, v15, v16)
                }
            };
            (v17, v18, v19, v20)
        }
    };
    (v21, v22, v23, v24)
}
fn method17(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> (i32, i32, i32, i32) {
    let (mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, i32) = match &*v0 {
        UH0::UH0_Cons(v5, v6) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH0> = v6.clone();
            let mut v7: US0 = train_verdict_of_18(v5);
            let (mut v8, mut v9, mut v10, mut v11): (i32, i32, i32, i32) = train_counted_19(v1, v2, v3, v4, v7);
            method17(v6, v8, v9, v10, v11)
        }
        _ => {
            let (mut v16, mut v17, mut v18, mut v19): (i32, i32, i32, i32) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Nil => {
                    (v1, v2, v3, v4)
                }
                _ => unreachable!(),
            };
            (v16, v17, v18, v19)
        }
    };
    (v20, v21, v22, v23)
}
fn train_progress_of_16(mut v0: Rc<UH0>) -> (i32, i32, i32, i32) {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = 0i32;
    let mut v3: i32 = 0i32;
    let mut v4: i32 = 0i32;
    let (mut v5, mut v6, mut v7, mut v8): (i32, i32, i32, i32) = method17(v0, v1, v2, v3, v4);
    (v5, v6, v7, v8)
}
fn train_line_1(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = train_name_of_5(v0.clone());
    let mut v2: Rc<str> = train_root_of_9(v0);
    let mut v3: bool = &*v1 == "";
    let mut v5: bool = if v3 {
        true
    } else {
        let mut v4: bool = &*v2 == "";
        v4
    };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cache/release-train"); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = method10(v2, v7);
        let mut v9: Rc<str> = method10(v8, v1.clone());
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notebooks.txt"); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = method10(v9.clone(), v10);
        let v12: bool = file_exists_12(v11.clone());
        let v13: bool = v12 == false;
        if v13 {
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release train "); } LIT.with(|lit| lit.clone()) }, v1));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no notebook set yet"); } LIT.with(|lit| lit.clone()) }));
            v15
        } else {
            let mut v17: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v11);
            let v19: Vec<u8> = v17.unwrap();
            let v20: Vec<u8> = method13(v19);
            let mut v22: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v20);
            let v24: std::string::String = v22.unwrap();
            let v26: Rc<str> = Rc::<str>::from(String::as_str(&v24));
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v26.split(&*v27).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v29: i32 = (v28.clone().borrow().len() as i32);
            let v30: i32 = v29.wrapping_sub(1i32);
            let v31: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v32: Rc<UH0> = logs_from_14(v9, &v28, v30, v31);
            let (mut v33, mut v34, mut v35, mut v36): (i32, i32, i32, i32) = train_progress_of_16(v32);
            train_progress_text_2(v1, v33, v34, v35, v36)
        }
    }
}
fn shared_text_21(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: &Rc<str>, mut v4: u64) -> Rc<str> {
    let mut v5: bool = v2 > 0i32;
    if v5 {
        let mut v6: Rc<str> = format_real_3(v2);
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) }, v6));
        let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" share a report name, not counted)"); } LIT.with(|lit| lit.clone()) }));
        v8
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    }
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn minutes_text_22(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(60000u64);
    format_real_23(v1)
}
fn pace_text_24(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: &Rc<str>, mut v4: u64, mut v5: u64) -> Rc<str> {
    let mut v6: u64 = v4.wrapping_sub(v5);
    let mut v7: u64 = (v0 as u64);
    let mut v8: u64 = v6.wrapping_div(v7);
    let mut v9: u64 = v8.wrapping_div(1000u64);
    let mut v10: Rc<str> = format_real_23(v9);
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" s per notebook"); } LIT.with(|lit| lit.clone()) }));
    v11
}
fn batch_progress_text_20(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64) -> Rc<str> {
    let mut v7: i32 = v0.wrapping_add(v1);
    let mut v8: i32 = v7.wrapping_add(v2);
    let mut v9: Rc<str> = format_real_3(v0);
    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notebook batch: "); } LIT.with(|lit| lit.clone()) }, v9));
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = format_real_3(v8);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written"); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = shared_text_21(v0, v1, v2, &v3, v4);
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: bool = v0 == 0i32;
    if v17 {
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", none yet after "); } LIT.with(|lit| lit.clone()) }));
        let v19: u64 = v6.wrapping_sub(v5);
        let v20: Rc<str> = minutes_text_22(v19);
        let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min"); } LIT.with(|lit| lit.clone()) }));
        v22
    } else {
        let v23: u64 = v6.wrapping_sub(v4);
        let v24: Rc<str> = minutes_text_22(v23);
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", last "); } LIT.with(|lit| lit.clone()) }));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v3.clone()));
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v24));
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min ago, ~"); } LIT.with(|lit| lit.clone()) }));
        let v30: Rc<str> = pace_text_24(v0, v1, v2, &v3, v4, v5);
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
        v31
    }
}
fn flag_value_until_next_flag_26(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method6(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    let v12: Rc<str> = if v8 {
        v0
    } else {
        let mut v9: usize = ((0i32) as usize);
        let mut v10: usize = ((v7) as usize);
        let mut v11: Rc<str> = Rc::<str>::from(&(v0)[v9..v10]);
        v11
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: i32 = (v15.clone().len() as i32);
    let mut v17: i32 = 0i32;
    let v18: i32 = method7(v15.clone(), v16, v17);
    let v19: i32 = v16.wrapping_sub(1i32);
    let v20: Rc<str> = string_slice(&v15, v18 as i64, v19 as i64);
    let mut v21: i32 = (v20.clone().len() as i32);
    let mut v22: i32 = method8(v20.clone(), v21);
    let v23: Rc<str> = string_slice(&v20, 0i32 as i64, v22 as i64);
    v23
}
fn batch_output_directory_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --output-dir "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method6(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let mut v10: i32 = v7.wrapping_add(14i32);
        let mut v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v10);
        let v13: usize = ((v10) as usize);
        let mut v14: i32 = v10.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        flag_value_until_next_flag_26(v16)
    }
}
fn last_write_ms_27(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        0u64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: std::io::Result<std::time::SystemTime> = v4.modified();
        let mut v6: bool = v5.is_ok();
        if v6 {
            let mut v7: std::time::SystemTime = v5.unwrap();
            let mut v8: std::time::Duration = v7.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let mut v9: u64 = v8.as_secs();
            let mut v10: u64 = v9.wrapping_mul(1000u64);
            let mut v11: u32 = v8.subsec_millis();
            let v12: u64 = (v11 as u64);
            let v13: u64 = v10.wrapping_add(v12);
            v13
        } else {
            0u64
        }
    }
}
fn closure3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> u64> {
    Rc::new(move |mut v1: Rc<str>| -> u64 {
        let mut v2: Rc<str> = method10(v0.clone(), v1);
        last_write_ms_27(v2)
    })
}
fn batch_paths_29(mut v0: Rc<str>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --path "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    v2
}
fn file_name_of_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let mut v7: Option<usize> = v6.rfind(&*v5);
    let mut v8: Option<usize> = method6(v7);
    let mut v9: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v10: Option<i32> = v8.map(|x| v9(x));
    let v12: i32 = v10.unwrap_or(-1i32);
    let v13: bool = v12 < 0i32;
    if v13 {
        v6
    } else {
        let mut v14: i32 = v12.wrapping_add(1i32);
        let v15: i32 = (v6.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(v12);
        let mut v17: i32 = v16.wrapping_sub(1i32);
        let v18: usize = ((v14) as usize);
        let v19: i32 = v14.wrapping_add(v17);
        let v20: usize = ((v19) as usize);
        let mut v21: Rc<str> = Rc::<str>::from(&(v6)[v18..v20]);
        v21
    }
}
fn report_name_of_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = file_name_of_33(v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".ipynb"); } LIT.with(|lit| lit.clone()) }));
    v2
}
fn chunk_name_31(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = v0.borrow()[v3 as usize].clone();
    let mut v5: Rc<str> = flag_value_until_next_flag_26(v4);
    report_name_of_32(v5)
}
fn names_matching_35(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: Rc<str>, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        let mut v6: i32 = (v0.clone().borrow().len() as i32);
        let mut v7: bool = v6 <= v4;
        if v7 {
            return v5;
        } else {
            let mut v8: i32 = v4.wrapping_add(1i32);
            let mut v9: Rc<str> = chunk_name_31(v0, v1.clone(), v2, v4);
            let mut v10: bool = &*v9 == &*v3;
            let v12: i32 = if v10 {
                let mut v11: i32 = v5.wrapping_add(1i32);
                v11
            } else {
                v5
            };
            (v4, v5) = (v8, v12);
            continue;
        }
    }
}
fn notebook_seen_of_34(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: Rc<str>) -> US1 {
    let mut v4: i32 = 1i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = names_matching_35(v0, v1.clone(), v2, v3.clone(), v4, v5);
    let mut v7: bool = v6 > 1i32;
    if v7 {
        US1::US1_SeenShared
    } else {
        let mut v9: u64 = v1(v3);
        let mut v10: bool = v9 > 0u64;
        let v12: bool = if v10 {
            let mut v11: bool = v2 <= v9;
            v11
        } else {
            false
        };
        if v12 {
            US1::US1_SeenWritten(v9)
        } else {
            US1::US1_SeenPending
        }
    }
}
fn progress_counted_36(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: US1) -> (i32, i32, i32, Rc<str>, u64) {
    let (mut v27, mut v28, mut v29, mut v30, mut v31): (i32, i32, i32, Rc<str>, u64) = match &v6 {
        US1::US1_SeenWritten(v7) => {
            let mut v7: u64 = *v7;
            let mut v8: i32 = v0.wrapping_add(1i32);
            let mut v9: bool = v4 <= v7;
            if v9 {
                (v8, v1, v2, v5, v7)
            } else {
                (v8, v1, v2, v3, v4)
            }
        }
        _ => {
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (i32, i32, i32, Rc<str>, u64) = match &v6 {
                US1::US1_SeenPending => {
                    let v15: i32 = v1.wrapping_add(1i32);
                    (v0, v15, v2, v3, v4)
                }
                _ => {
                    let (mut v17, mut v18, mut v19, mut v20, mut v21): (i32, i32, i32, Rc<str>, u64) = match v6 {
                        US1::US1_SeenShared => {
                            let v16: i32 = v2.wrapping_add(1i32);
                            (v0, v1, v16, v3, v4)
                        }
                        _ => unreachable!(),
                    };
                    (v17, v18, v19, v20, v21)
                }
            };
            (v22, v23, v24, v25, v26)
        }
    };
    (v27, v28, v29, v30, v31)
}
fn progress_from_30(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<str>, mut v8: u64) -> (i32, i32, i32, Rc<str>, u64) {
    let mut v9: i32 = (v0.clone().borrow().len() as i32);
    let mut v10: bool = v9 <= v3;
    let (mut v24, mut v25, mut v26, mut v27, mut v28): (i32, i32, i32, Rc<str>, u64) = if v10 {
        (v4, v5, v6, v7, v8)
    } else {
        let mut v11: Rc<str> = chunk_name_31(v0, v1.clone(), v2, v3);
        let v12: i32 = v3.wrapping_add(1i32);
        let v13: US1 = notebook_seen_of_34(v0, v1.clone(), v2, v11.clone());
        let (mut v14, mut v15, mut v16, mut v17, mut v18): (i32, i32, i32, Rc<str>, u64) = progress_counted_36(v4, v5, v6, v7, v8, v11, v13);
        let (mut v19, mut v20, mut v21, mut v22, mut v23): (i32, i32, i32, Rc<str>, u64) = progress_from_30(v0, v1, v2, v12, v14, v15, v16, v17, v18);
        (v19, v20, v21, v22, v23)
    };
    (v24, v25, v26, v27, v28)
}
fn batch_progress_of_28(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> u64>, mut v2: u64) -> (i32, i32, i32, Rc<str>, u64) {
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = batch_paths_29(v0);
    let mut v4: i32 = 1i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = 0i32;
    let mut v7: i32 = 0i32;
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: u64 = 0u64;
    let (mut v10, mut v11, mut v12, mut v13, mut v14): (i32, i32, i32, Rc<str>, u64) = progress_from_30(&v3, v1, v2, v4, v5, v6, v7, v8, v9);
    (v10, v11, v12, v13, v14)
}
fn notebook_batch_line_0(mut v0: Rc<str>, mut v1: u64, mut v2: u64) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run_notebook.ps1"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = v0.contains(&*v3);
    let mut v7: bool = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --output-dir "); } LIT.with(|lit| lit.clone()) };
        let mut v6: bool = v0.contains(&*v5);
        v6
    } else {
        false
    };
    let mut v8: bool = v7 == false;
    if v8 {
        train_line_1(v0)
    } else {
        let mut v10: Rc<str> = batch_output_directory_25(v0.clone());
        let mut v11: u64 = v1.wrapping_add(1u64);
        let v12: u64 = v11.wrapping_mul(60000u64);
        let v13: u64 = v2.wrapping_sub(v12);
        let mut v14: Rc<dyn Fn(Rc<str>) -> u64> = closure3(v10);
        let (mut v15, mut v16, mut v17, mut v18, mut v19): (i32, i32, i32, Rc<str>, u64) = batch_progress_of_28(v0, v14, v13);
        batch_progress_text_20(v15, v16, v17, v18, v19, v13, v2)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64| -> Rc<str> {
        let mut v2: std::time::SystemTime = std::time::SystemTime::now();
        let mut v3: std::time::Duration = v2.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let mut v4: u64 = v3.as_secs();
        let mut v5: u64 = v4.wrapping_mul(1000000000u64);
        let mut v6: u32 = v3.subsec_nanos();
        let mut v7: u64 = (v6 as u64);
        let mut v8: u64 = v5.wrapping_add(v7);
        let mut v9: u64 = v8.wrapping_div(1000u64);
        let mut v10: i64 = (v9 as i64);
        let mut v11: i64 = v10.wrapping_div(1000i64);
        let v12: u64 = (v11 as u64);
        notebook_batch_line_0(v0, v1, v12)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_notebook_batch_line(v0: &str, v1: u64) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
