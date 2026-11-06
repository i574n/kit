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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method3(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method2(v16, v17)
    }
}
fn field_end_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US0::US0_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US0 = match v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US0 = US0::US0_ScanPlain;
            let v6: i32 = field_end_8(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_9(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn concat5_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn unescaped_from_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_12(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_12(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_11(v3, v4, v5, v6, v7)
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                let v6: bool = v4 == b'\t';
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
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                let v6: bool = v4 == b'\t';
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
fn deps_renamed_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> (Rc<str>, bool) {
    loop {
        let v7: bool = v2 <= v1;
        if v7 {
            return (v5, v6);
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let mut v9: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let v10: bool = v9 < 0i32;
            let v11: i32 = if v10 {
                v2
            } else {
                v9
            };
            let v13: i32 = if v10 {
                v2
            } else {
                let v12: i32 = v9.wrapping_add(1i32);
                v12
            };
            let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v11 as usize).unwrap_or(""));
            let v15: i32 = (v14.clone().len() as i32);
            let v16: i32 = 0i32;
            let v17: i32 = method14(v14.clone(), v15, v16);
            let v18: i32 = v15.wrapping_sub(1i32);
            let v19: Rc<str> = string_slice(&v14, v17 as i64, v18 as i64);
            let v20: i32 = (v19.clone().len() as i32);
            let v21: i32 = method15(v19.clone(), v20);
            let v22: Rc<str> = string_slice(&v19, 0i32 as i64, v21 as i64);
            let v23: bool = &*v22 == &*v3;
            let v24: Rc<str> = if v23 {
                v4.clone()
            } else {
                v22
            };
            let v25: bool = &*v5 == "";
            let v28: Rc<str> = if v25 {
                v24
            } else {
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v24));
                v27
            };
            let v29: bool = v23 || v6;
            (v1, v5, v6) = (v13, v28, v29);
            continue;
        }
    }
}
fn dependent_line_6(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let mut v9: i32 = (v0.clone().len() as i32);
    let v10: i32 = v1.wrapping_add(6i32);
    let v11: i32 = 0i32;
    let v12: i32 = field_start_7(v0.clone(), v10, v9, v11);
    let v13: US0 = US0::US0_ScanPlain;
    let v14: i32 = field_end_8(v0.clone(), v12, v9, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: Rc<str> = unquoted_10(v15);
    let v17: i32 = 1i32;
    let v18: i32 = field_start_7(v0.clone(), v10, v9, v17);
    let v19: US0 = US0::US0_ScanPlain;
    let v20: i32 = field_end_8(v0.clone(), v18, v9, v19);
    let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v18 as usize..v20 as usize).unwrap_or(""));
    let v22: i32 = v21.find(&*v2).map(|offset| offset as i32).unwrap_or(-1);
    let v23: bool = v22 < 0i32;
    let v24: bool = v23 == false;
    let (mut v30, mut v31, mut v32, mut v33, mut v34): (i32, Rc<str>, i32, Rc<str>, i32) = if v24 {
        let v25: bool = &*v7 == "";
        let v28: Rc<str> = if v25 {
            v16.clone()
        } else {
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v16.clone()));
            v27
        };
        let v29: i32 = v8.wrapping_add(1i32);
        (v4, v5, v6, v28, v29)
    } else {
        (v4, v5, v6, v7, v8)
    };
    let v35: i32 = 7i32;
    let v36: i32 = field_start_7(v0.clone(), v10, v9, v35);
    let v37: US0 = US0::US0_ScanPlain;
    let v38: i32 = field_end_8(v0.clone(), v36, v9, v37);
    let v39: Rc<str> = std::rc::Rc::<str>::from(v0.get(v36 as usize..v38 as usize).unwrap_or(""));
    let v40: Rc<str> = unquoted_10(v39);
    let v41: i32 = 0i32;
    let v42: i32 = (v40.clone().len() as i32);
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v44: bool = false;
    let (mut v45, mut v46): (Rc<str>, bool) = deps_renamed_13(v40, v41, v42, v2, v3, v43.clone(), v44);
    if v46 {
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = Rc::<str>::from(v45.replace(&*v48, &*v49));
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = Rc::<str>::from(v50.replace(&*v47, &*v51));
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let v55: Rc<str> = Rc::<str>::from(v52.replace(&*v53, &*v54));
        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = Rc::<str>::from(v55.replace(&*v56, &*v57));
        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let mut v61: Rc<str> = Rc::<str>::from(v58.replace(&*v59, &*v60));
        let v62: Rc<str> = concat5_12(v47.clone(), v61, v47.clone(), v43.clone(), v43.clone());
        let v63: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v36 as usize).unwrap_or(""));
        let v64: Rc<str> = std::rc::Rc::<str>::from(v0.get(v38 as usize..v9 as usize).unwrap_or(""));
        let v65: Rc<str> = concat5_12(v63, v62, v64, v43.clone(), v43.clone());
        let v66: bool = &*v31 == "";
        let v69: Rc<str> = if v66 {
            v16
        } else {
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v16));
            v68
        };
        let v70: i32 = v32.wrapping_add(1i32);
        (v65, v30, v69, v70, v33, v34)
    } else {
        (v0, v30, v31, v32, v33, v34)
    }
}
fn renamed_line_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v9: bool = v0.starts_with(&*v8);
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = concat5_12(v10.clone(), v11.clone(), v1.clone(), v12.clone(), v13.clone());
        let v15: i32 = v0.find(&*v14).map(|offset| offset as i32).unwrap_or(-1);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = concat5_12(v16.clone(), v11.clone(), v1.clone(), v12.clone(), v13.clone());
        let v18: i32 = v0.find(&*v17).map(|offset| offset as i32).unwrap_or(-1);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = concat5_12(v19.clone(), v11.clone(), v1.clone(), v12.clone(), v13.clone());
        let v21: i32 = v0.find(&*v20).map(|offset| offset as i32).unwrap_or(-1);
        let v22: i32 = v3.wrapping_add(1i32);
        let v23: bool = v15 < 0i32;
        let v24: bool = v23 == false;
        if v24 {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(v2.replace(&*v25, &*v26));
            let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            let v29: Rc<str> = concat5_12(v10.clone(), v11.clone(), v1, v12.clone(), v13.clone());
            let v30: i32 = (v29.len() as i32);
            let v31: i32 = v15.wrapping_add(v30);
            let v32: i32 = (v0.clone().len() as i32);
            let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v32 as usize).unwrap_or(""));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v28));
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
            let v36: Rc<str> = concat5_12(v10, v11, v2, v12, v13);
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v33));
            (v38, v22, v4, v5, v6, v7)
        } else {
            let v39: bool = v18 < 0i32;
            let v40: bool = v39 == false;
            if v40 {
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v44: Rc<str> = Rc::<str>::from(v2.replace(&*v42, &*v43));
                let v45: Rc<str> = std::rc::Rc::<str>::from(v44.to_ascii_lowercase());
                let v46: Rc<str> = concat5_12(v41, v45, v13.clone(), v13.clone(), v13.clone());
                let v47: Rc<str> = concat5_12(v16.clone(), v11.clone(), v1, v12.clone(), v13.clone());
                let v48: i32 = (v47.len() as i32);
                let v49: i32 = v18.wrapping_add(v48);
                let v50: i32 = (v0.clone().len() as i32);
                let v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v49 as usize..v50 as usize).unwrap_or(""));
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v46));
                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
                let v54: Rc<str> = concat5_12(v16, v11, v2, v12, v13);
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v51));
                (v56, v22, v4, v5, v6, v7)
            } else {
                let v57: bool = v21 < 0i32;
                let v58: bool = v57 == false;
                if v58 {
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v61: Rc<str> = Rc::<str>::from(v2.replace(&*v59, &*v60));
                    let v62: Rc<str> = std::rc::Rc::<str>::from(v61.to_ascii_lowercase());
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) }, v62));
                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) }));
                    let v65: i32 = v21.wrapping_add(6i32);
                    let v66: i32 = (v0.clone().len() as i32);
                    let v67: i32 = 1i32;
                    let v68: i32 = field_start_7(v0.clone(), v65, v66, v67);
                    let v69: US0 = US0::US0_ScanPlain;
                    let v70: i32 = field_end_8(v0.clone(), v68, v66, v69);
                    let v71: Rc<str> = std::rc::Rc::<str>::from(v0.get(v68 as usize..v70 as usize).unwrap_or(""));
                    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                    let v73: Rc<str> = Rc::<str>::from(v71.replace(&*v72, &*v13));
                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v73));
                    let v75: Rc<str> = concat5_12(v19.clone(), v11.clone(), v1, v12.clone(), v13.clone());
                    let v76: i32 = (v75.len() as i32);
                    let v77: i32 = v21.wrapping_add(v76);
                    let v78: Rc<str> = std::rc::Rc::<str>::from(v0.get(v77 as usize..v66 as usize).unwrap_or(""));
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v74));
                    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
                    let v81: Rc<str> = concat5_12(v19, v11, v2, v12, v13);
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v81));
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v78));
                    (v83, v22, v4, v5, v6, v7)
                } else {
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let v85: i32 = v0.find(&*v84).map(|offset| offset as i32).unwrap_or(-1);
                    let v86: bool = v85 < 0i32;
                    if v86 {
                        (v0, v3, v4, v5, v6, v7)
                    } else {
                        dependent_line_6(v0, v85, v1, v2, v3, v4, v5, v6, v7)
                    }
                }
            }
        }
    } else {
        (v0, v3, v4, v5, v6, v7)
    }
}
fn method4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH0>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<UH0>, i32, Rc<str>, i32, Rc<str>, i32) {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v9, v10) => {
                let mut v9: Rc<str> = v9;
                let v10: Rc<UH0> = v10;
                let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_5(v9, v0.clone(), v1.clone(), v4, v5, v6, v7, v8);
                let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v3));
                (v2, v3, v4, v5, v6, v7, v8) = (v10, v17, v12, v13, v14, v15, v16);
                continue;
            }
            UH0::UH0_Nil => {
                return (v3, v4, v5, v6, v7, v8);
            }
        }
    }
}
fn method16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn renamed_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH0> = split_list_1(v3.clone(), v0);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: i32 = 0i32;
    let mut v9: i32 = 0i32;
    let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<UH0>, i32, Rc<str>, i32, Rc<str>, i32) = method4(v1, v2, v4, v5, v6, v7.clone(), v8, v7, v9);
    let v16: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v17: Rc<UH0> = method2(v10, v16);
    let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v19: i32 = 0i32;
    let v20: i32 = method16(v18.clone(), v17, v19);
    let v21: Rc<Vec<Rc<str>>> = Rc::new(v18.borrow().clone());
    let v22: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v21).as_ref().clone()));
    let v23: Rc<str> = Rc::<str>::from((*v22.borrow()).join(&*v3));
    (v23, v11, v13, v12, v15, v14)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
        renamed_text_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_renamed_text(v0: &str, v1: &str, v2: &str) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub type SpiralShape_652accfc6a8d = Mut0;
