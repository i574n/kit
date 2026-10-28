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
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
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
                let v25: US0 = match &v3 {
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
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US0 = US0::US0_ScanPlain;
            let v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3.clone(), v4, v5, v6, v7.clone())
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn deps_renamed_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: bool) -> (Rc<str>, bool) {
    loop {
        let v7: bool = v2 <= v1;
        if v7 {
            return (v5.clone(), v6);
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v9: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
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
            let v17: i32 = method10(v14.clone(), v15, v16);
            let v18: i32 = v15.wrapping_sub(1i32);
            let v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
            let v20: i32 = (v19.clone().len() as i32);
            let v21: i32 = method11(v19.clone(), v20);
            let v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
            let v23: bool = &*v22 == &*v3;
            let v24: Rc<str> = if v23 {
                v4.clone()
            } else {
                v22.clone()
            };
            let v25: bool = &*v5 == "";
            let v28: Rc<str> = if v25 {
                v24.clone()
            } else {
                let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v8)), v24));
                v27.clone()
            };
            let v29: bool = v23 || v6;
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v13, v2, v3.clone(), v4.clone(), v28.clone(), v29);
            continue;
        }
    }
}
fn dependent_line_3(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let v9: i32 = (v0.clone().len() as i32);
    let v10: i32 = v1.wrapping_add(6i32);
    let v11: i32 = 0i32;
    let v12: i32 = field_start_4(v0.clone(), v10, v9, v11);
    let v13: US0 = US0::US0_ScanPlain;
    let v14: i32 = field_end_5(v0.clone(), v12, v9, v13.clone());
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: Rc<str> = unquoted_7(v15.clone());
    let v17: i32 = 1i32;
    let v18: i32 = field_start_4(v0.clone(), v10, v9, v17);
    let v19: US0 = US0::US0_ScanPlain;
    let v20: i32 = field_end_5(v0.clone(), v18, v9, v19.clone());
    let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v18 as usize..v20 as usize).unwrap_or(""));
    let v22: i32 = v21.find(&*v2).map(|offset| offset as i32).unwrap_or(-1);
    let v23: bool = v22 < 0i32;
    let v24: bool = v23 == false;
    let (mut v31, mut v32, mut v33, mut v34, mut v35): (i32, Rc<str>, i32, Rc<str>, i32) = if v24 {
        let v25: bool = &*v7 == "";
        let v29: Rc<str> = if v25 {
            v16.clone()
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v26)), v16));
            v28.clone()
        };
        let v30: i32 = v8.wrapping_add(1i32);
        (v4, v5.clone(), v6, v29.clone(), v30)
    } else {
        (v4, v5.clone(), v6, v7.clone(), v8)
    };
    let v36: i32 = 7i32;
    let v37: i32 = field_start_4(v0.clone(), v10, v9, v36);
    let v38: US0 = US0::US0_ScanPlain;
    let v39: i32 = field_end_5(v0.clone(), v37, v9, v38.clone());
    let v40: Rc<str> = std::rc::Rc::<str>::from(v0.get(v37 as usize..v39 as usize).unwrap_or(""));
    let v41: Rc<str> = unquoted_7(v40.clone());
    let v42: i32 = 0i32;
    let v43: i32 = (v41.clone().len() as i32);
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v45: bool = false;
    let (mut v46, mut v47): (Rc<str>, bool) = deps_renamed_9(v41.clone(), v42, v43, v2.clone(), v3.clone(), v44.clone(), v45);
    if v47 {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = std::rc::Rc::<str>::from(v46.replace(&*v49, &v50));
        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v48, &v52));
        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let v56: Rc<str> = std::rc::Rc::<str>::from(v53.replace(&*v54, &v55));
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let v59: Rc<str> = std::rc::Rc::<str>::from(v56.replace(&*v57, &v58));
        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let v62: Rc<str> = std::rc::Rc::<str>::from(v59.replace(&*v60, &v61));
        let mut v63: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v62, &*v48, &*v44, &*v44].concat());
        let v64: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v37 as usize).unwrap_or(""));
        let v65: Rc<str> = std::rc::Rc::<str>::from(v0.get(v39 as usize..v9 as usize).unwrap_or(""));
        let v66: Rc<str> = std::rc::Rc::<str>::from([&*v64, &*v63, &*v65, &*v44, &*v44].concat());
        let v67: bool = &*v32 == "";
        let v71: Rc<str> = if v67 {
            v16.clone()
        } else {
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v70: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v68)), v16));
            v70.clone()
        };
        let v72: i32 = v33.wrapping_add(1i32);
        (v66.clone(), v31, v71.clone(), v72, v34.clone(), v35)
    } else {
        (v0.clone(), v31, v32.clone(), v33, v34.clone(), v35)
    }
}
fn renamed_line_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v0.starts_with(&*v8);
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
        let v15: i32 = v0.find(&*v14).map(|offset| offset as i32).unwrap_or(-1);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
        let v18: i32 = v0.find(&*v17).map(|offset| offset as i32).unwrap_or(-1);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
        let v21: i32 = v0.find(&*v20).map(|offset| offset as i32).unwrap_or(-1);
        let v22: i32 = v3.wrapping_add(1i32);
        let v23: bool = v15 < 0i32;
        let v24: bool = v23 == false;
        if v24 {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v25, &v26));
            let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            let v29: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v1, &*v12, &*v13].concat());
            let v30: i32 = (v29.clone().len() as i32);
            let v31: i32 = v15.wrapping_add(v30);
            let v32: i32 = (v0.clone().len() as i32);
            let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v32 as usize).unwrap_or(""));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v28));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let v37: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v2, &*v12, &*v13].concat());
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v33));
            (v39.clone(), v22, v4.clone(), v5, v6.clone(), v7)
        } else {
            let v40: bool = v18 < 0i32;
            let v41: bool = v40 == false;
            if v41 {
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                let mut v45: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v43, &v44));
                let v46: Rc<str> = std::rc::Rc::<str>::from(v45.to_ascii_lowercase());
                let v47: Rc<str> = std::rc::Rc::<str>::from([&*v42, &*v46, &*v13, &*v13, &*v13].concat());
                let v48: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v1, &*v12, &*v13].concat());
                let v49: i32 = (v48.clone().len() as i32);
                let v50: i32 = v18.wrapping_add(v49);
                let v51: i32 = (v0.clone().len() as i32);
                let v52: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v51 as usize).unwrap_or(""));
                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v47));
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                let v56: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v11, &*v2, &*v12, &*v13].concat());
                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v55, v56)), v52));
                (v58.clone(), v22, v4.clone(), v5, v6.clone(), v7)
            } else {
                let v59: bool = v21 < 0i32;
                let v60: bool = v59 == false;
                if v60 {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v63: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v61, &v62));
                    let v64: Rc<str> = std::rc::Rc::<str>::from(v63.to_ascii_lowercase());
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v65, v64)), v62));
                    let v68: i32 = v21.wrapping_add(6i32);
                    let v69: i32 = (v0.clone().len() as i32);
                    let v70: i32 = 1i32;
                    let v71: i32 = field_start_4(v0.clone(), v68, v69, v70);
                    let v72: US0 = US0::US0_ScanPlain;
                    let v73: i32 = field_end_5(v0.clone(), v71, v69, v72.clone());
                    let v74: Rc<str> = std::rc::Rc::<str>::from(v0.get(v71 as usize..v73 as usize).unwrap_or(""));
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = Rc::<str>::from(v74.replace(&*v75, &*v13));
                    let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v76));
                    let v78: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v1, &*v12, &*v13].concat());
                    let v79: i32 = (v78.clone().len() as i32);
                    let v80: i32 = v21.wrapping_add(v79);
                    let v81: Rc<str> = std::rc::Rc::<str>::from(v0.get(v80 as usize..v69 as usize).unwrap_or(""));
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v77));
                    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v83));
                    let v85: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v11, &*v2, &*v12, &*v13].concat());
                    let v87: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v84, v85)), v81));
                    (v87.clone(), v22, v4.clone(), v5, v6.clone(), v7)
                } else {
                    let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                    let v89: i32 = v0.find(&*v88).map(|offset| offset as i32).unwrap_or(-1);
                    let v90: bool = v89 < 0i32;
                    if v90 {
                        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    } else {
                        dependent_line_3(v0.clone(), v89, v1.clone(), v2.clone(), v3, v4.clone(), v5, v6.clone(), v7)
                    }
                }
            }
        }
    } else {
        (v0.clone(), v3, v4.clone(), v5, v6.clone(), v7)
    }
}
fn renamed_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: i32, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) {
    loop {
        let v11: bool = v2 <= v1;
        if v11 {
            return (v5.clone(), v6, v7.clone(), v8, v9.clone(), v10);
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v13: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v12)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let v14: bool = v13 < 0i32;
            let v15: i32 = if v14 {
                v2
            } else {
                v13
            };
            let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v15 as usize).unwrap_or(""));
            let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_line_2(v16.clone(), v3.clone(), v4.clone(), v6, v7.clone(), v8, v9.clone(), v10);
            let v24: Rc<str> = if v14 {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23.clone()
            } else {
                v12.clone()
            };
            let v25: i32 = v15.wrapping_add(1i32);
            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v17)), v24));
            (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = (v0.clone(), v25, v2, v3.clone(), v4.clone(), v27.clone(), v18, v19.clone(), v20, v21.clone(), v22);
            continue;
        }
    }
}
fn renamed_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    let v3: i32 = 0i32;
    let v4: i32 = (v0.clone().len() as i32);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, Rc<str>, i32, Rc<str>, i32) = renamed_from_1(v0.clone(), v3, v4, v1.clone(), v2.clone(), v5.clone(), v6, v5.clone(), v7, v5.clone(), v8);
    (v9.clone(), v10, v12, v11.clone(), v14, v13.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
        renamed_text_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_renamed_text(v0: &str, v1: &str, v2: &str) -> (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
