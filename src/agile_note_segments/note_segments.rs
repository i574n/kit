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
    US0_Replaced,
    US0_TemporaryWriteFailed,
    US0_RenameFailed,
    US0_RenameFailedTemporaryKept,
    US0_ReadbackMismatch,
    US0_ChangedSinceRead,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
fn file_exists_2(mut v0: Rc<str>) -> bool {
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
fn renamed_within_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v5);
                continue;
            }
        }
    }
}
fn written_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = file_exists_2(v0.clone());
    if v3 {
        let v4: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
        let v22: US0 = if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
            let mut v7: bool = std::fs::write(&*v6, &*v2).is_ok();
            if v7 {
                let v8: i32 = 40i32;
                let mut v9: bool = renamed_within_3(v0.clone(), v6.clone(), v8);
                if v9 {
                    let v10: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                    if v10 {
                        US0::US0_Replaced
                    } else {
                        US0::US0_ReadbackMismatch
                    }
                } else {
                    let v14: bool = std::fs::remove_file(&*v6).is_ok() || !std::path::Path::new(&*v6).exists();
                    if v14 {
                        US0::US0_RenameFailed
                    } else {
                        US0::US0_RenameFailedTemporaryKept
                    }
                }
            } else {
                US0::US0_TemporaryWriteFailed
            }
        } else {
            US0::US0_ChangedSinceRead
        };
        match &v22 {
            US0::US0_ChangedSinceRead => {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                v28.clone()
            }
            US0::US0_ReadbackMismatch => {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            }
            US0::US0_RenameFailed => {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                v25.clone()
            }
            US0::US0_RenameFailedTemporaryKept => {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                v26.clone()
            }
            US0::US0_Replaced => {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            }
            US0::US0_TemporaryWriteFailed => {
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            }
        }
    } else {
        std::fs::write(&*v0, &*v2).unwrap();
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v36.clone()
    }
}
fn segment_name_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..4i32 as usize).unwrap_or(""));
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.get(5i32 as usize..7i32 as usize).unwrap_or(""));
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
    v3.borrow_mut().push(v4);
    v3.borrow_mut().push(v1);
    v3.borrow_mut().push(v2);
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    v8
}
fn state_path_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let mut v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    v6
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn text_or_empty_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_2(v0.clone());
    if v1 {
        let mut v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method7(v5);
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        v12
    } else {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    }
}
fn registered_8(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    "); } LIT.with(|lit| lit.clone()) };
    v2.borrow_mut().push(v3);
    v2.borrow_mut().push(v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    v2.borrow_mut().push(v4.clone());
    let v5: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from((*v6.borrow()).join(&*v7));
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v9.borrow_mut().push(v4);
    v9.borrow_mut().push(v0);
    let v10: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v10).as_ref().clone()));
    let v12: Rc<str> = Rc::<str>::from((*v11.borrow()).join(&*v7));
    let mut v13: bool = v12.contains(&*v8);
    v13
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method12(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let mut v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method12(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.clone().borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let mut v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method11(v16.clone(), v17.clone())
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method13(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH0::UH0_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            let v4: Rc<UH0> = method15(v3, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn manifest_lines_with_14(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v17, v18) => {
                let v17: Rc<str> = v17.clone();
                let v18: Rc<UH0> = v18.clone();
                let v19: i32 = (v17.clone().len() as i32);
                let v20: i32 = 0i32;
                let v21: i32 = method16(v17.clone(), v19, v20);
                let v22: i32 = v19.wrapping_sub(1i32);
                let v23: Rc<str> = string_slice(&v17.clone(), v21 as i64, v22 as i64);
                let mut v24: i32 = (v23.clone().len() as i32);
                let v25: i32 = method17(v23.clone(), v24);
                let v26: Rc<str> = string_slice(&v23.clone(), 0i32 as i64, v25 as i64);
                let v27: bool = &*v26 == &*v1;
                if v27 {
                    let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v29: Rc<UH0> = method11(v4.clone(), v28.clone());
                    return method15(v29.clone(), v0.clone());
                } else {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                    let v32: bool = v26.ends_with(&*v31);
                    let v33: bool = v32 && v2;
                    let v35: bool = if v33 {
                        let v34: bool = v3 == false;
                        v34
                    } else {
                        false
                    };
                    if v35 {
                        let v36: bool = &*v26 == "modules:";
                        let v37: bool = true;
                        let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                        v38.borrow_mut().push(v39);
                        v38.borrow_mut().push(v1.clone());
                        let v40: Rc<Vec<Rc<str>>> = Rc::new(v38.borrow().clone());
                        let v41: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v40).as_ref().clone()));
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v43: Rc<str> = Rc::<str>::from((*v41.borrow()).join(&*v42));
                        let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43.clone(), v4.clone()));
                        let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v44.clone()));
                        (v0, v1, v2, v3, v4) = (v18.clone(), v1.clone(), v36, v37, v45.clone());
                        continue;
                    } else {
                        let v47: bool = v26.ends_with(&*v31);
                        if v47 {
                            let v48: bool = &*v26 == "modules:";
                            let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v4.clone()));
                            (v0, v1, v2, v3, v4) = (v18.clone(), v1.clone(), v48, v3, v49.clone());
                            continue;
                        } else {
                            let v52: bool = if v2 {
                                let v51: bool = v3 == false;
                                v51
                            } else {
                                false
                            };
                            let v54: bool = if v52 {
                                let v53: bool = &*v26 != "agile";
                                v53
                            } else {
                                false
                            };
                            let v58: bool = if v54 {
                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
                                let v56: bool = v26.starts_with(&*v55);
                                let v57: bool = v56 == false;
                                v57
                            } else {
                                false
                            };
                            if v58 {
                                let v59: bool = true;
                                let mut v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                                v60.borrow_mut().push(v61);
                                v60.borrow_mut().push(v1.clone());
                                let v62: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
                                let v63: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v62).as_ref().clone()));
                                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v65: Rc<str> = Rc::<str>::from((*v63.borrow()).join(&*v64));
                                let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v65.clone(), v4.clone()));
                                let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v66.clone()));
                                (v0, v1, v2, v3, v4) = (v18.clone(), v1.clone(), v2, v59, v67.clone());
                                continue;
                            } else {
                                let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17.clone(), v4.clone()));
                                (v0, v1, v2, v3, v4) = (v18.clone(), v1.clone(), v2, v3, v69.clone());
                                continue;
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v6: bool = if v3 {
                    true
                } else {
                    let v5: bool = v2 == false;
                    v5
                };
                let v14: Rc<UH0> = if v6 {
                    v4.clone()
                } else {
                    let mut v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    "); } LIT.with(|lit| lit.clone()) };
                    v7.borrow_mut().push(v8);
                    v7.borrow_mut().push(v1);
                    let mut v9: Rc<Vec<Rc<str>>> = Rc::new(v7.borrow().clone());
                    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v12: Rc<str> = Rc::<str>::from((*v10.borrow()).join(&*v11));
                    Rc::new(UH0::UH0_Cons(v12.clone(), v4.clone()))
                };
                let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method11(v14.clone(), v15.clone());
            }
        }
    }
}
fn method18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn manifest_with_segment_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<UH0> = split_list_10(v2.clone(), v0);
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH0> = method13(v3, v4);
    let mut v6: bool = false;
    let mut v7: bool = false;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = manifest_lines_with_14(v5, v1, v6, v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let v12: i32 = method18(v10.clone(), v9, v11);
    let mut v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v2));
    let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v16.borrow_mut().push(v15);
    v16.borrow_mut().push(v2);
    let v17: Rc<Vec<Rc<str>>> = Rc::new(v16.borrow().clone());
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v17).as_ref().clone()));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from((*v18.borrow()).join(&*v19));
    v20
}
fn note_appended_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = segment_name_4(v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = state_path_5(v0.clone(), v4);
    let mut v6: Rc<str> = text_or_empty_6(v5.clone());
    let mut v7: bool = registered_8(v6.clone(), v3.clone());
    let v11: Rc<str> = if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8
    } else {
        let mut v9: Rc<str> = manifest_with_segment_9(v6.clone(), v3.clone());
        written_1(v5, v6, v9)
    };
    let v12: bool = &*v11 != "";
    if v12 {
        v11
    } else {
        let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v13.borrow_mut().push(v3);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        v13.borrow_mut().push(v14);
        let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
        let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v17));
        let v19: Rc<str> = state_path_5(v0, v18);
        let v20: Rc<str> = text_or_empty_6(v19.clone());
        let v21: bool = &*v20 == "";
        let v23: Rc<str> = if v21 {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("open agile\n"); } LIT.with(|lit| lit.clone()) };
            v22
        } else {
            v20.clone()
        };
        let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        v24.borrow_mut().push(v23);
        v24.borrow_mut().push(v1);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v24.borrow_mut().push(v25);
        let v26: Rc<Vec<Rc<str>>> = Rc::new(v24.borrow().clone());
        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v26).as_ref().clone()));
        let v28: Rc<str> = Rc::<str>::from((*v27.borrow()).join(&*v17));
        written_1(v19, v20, v28)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        note_appended_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn declared_from_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let mut v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn last_declared_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_23(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v4);
            continue;
        }
    }
}
fn notes_insertion_22(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let mut v7: i32 = 0i32;
    let v8: i32 = declared_from_23(v0.clone(), v6.clone(), v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v3, &*v1, &*v4, &*v5].concat());
    let v11: i32 = 0i32;
    let v12: i32 = declared_from_23(v0.clone(), v10.clone(), v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v1, &*v4, &*v5].concat());
    let v15: i32 = 0i32;
    let mut v16: i32 = -1i32;
    let v17: i32 = last_declared_24(v0.clone(), v14.clone(), v15, v16);
    let v18: bool = v12 <= v17;
    let v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let v20: bool = v8 <= v19;
    let v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let mut v24: bool = v23 < 0i32;
    if v24 {
        let v25: i32 = (v0.clone().len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: i32 = notes_insertion_22(v0.clone(), v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v5, &*v5].concat());
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    let v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v9, &*v5, &*v5].concat());
    v10
}
fn state_written_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v7));
        let mut v9: bool = std::fs::write(&*v8, &*v2).is_ok();
        let v22: US0 = if v9 {
            let v10: i32 = 40i32;
            let v11: bool = renamed_within_3(v0.clone(), v8.clone(), v10);
            if v11 {
                let v12: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v12 {
                    US0::US0_Replaced
                } else {
                    US0::US0_ReadbackMismatch
                }
            } else {
                let mut v16: bool = std::fs::remove_file(&*v8).is_ok() || !std::path::Path::new(&*v8).exists();
                if v16 {
                    US0::US0_RenameFailed
                } else {
                    US0::US0_RenameFailedTemporaryKept
                }
            }
        } else {
            US0::US0_TemporaryWriteFailed
        };
        match &v22 {
            US0::US0_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v33: Rc<str> = match &v22 {
                    US0::US0_ChangedSinceRead => {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US0::US0_ReadbackMismatch => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    US0::US0_RenameFailed => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    US0::US0_RenameFailedTemporaryKept => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US0::US0_TemporaryWriteFailed => {
                        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v33.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn note_persisted_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, u64) {
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AGILE_NOTE_SEGMENTS"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = get_environment_variable_20(v7);
    let mut v9: bool = &*v8 == "1";
    if v9 {
        let v10: Rc<str> = note_appended_0(v0, v5.clone(), v6);
        let v11: i32 = (v2.clone().len() as i32);
        let v12: u64 = (v11 as u64);
        (v10, v12)
    } else {
        let mut v13: Rc<str> = notes_appended_21(v3, v4, v5);
        let v14: US1 = state_written_25(v1, v2, v13.clone());
        match &v14 {
            US1::US1_StateWritten => {
                let v15: i32 = (v13.len() as i32);
                let mut v16: u64 = (v15 as u64);
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v17, v16)
            }
            _ => {
                let v21: Rc<str> = match &v14 {
                    US1::US1_StateChangedSinceRead => {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v18
                    }
                    US1::US1_StateWriteFailed(v19) => {
                        let v19: Rc<str> = v19.clone();
                        v19
                    }
                    _ => unreachable!(),
                };
                (v21, 0u64)
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (Rc<str>, u64) {
        note_persisted_19(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_note_appended(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_agile_note_persisted(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> (Rc<str>, u64) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
