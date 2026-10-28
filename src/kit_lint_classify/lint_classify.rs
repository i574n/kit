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
    US0_TermMacro,
    US0_TypeMacro,
}
#[derive(Clone)]
enum US1 {
    US1_InlineMacro,
    US1_GlobalDeclaration,
    US1_GlobalBody,
}
#[derive(Clone)]
enum US3 {
    US3_MacroLong,
    US3_MacroBlock,
    US3_MacroFlow,
    US3_MacroCast,
    US3_MacroArith,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US3),
    US2_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>, Rc<str>, Rc<str>),
    US5_None,
}
fn words_from_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: i32 = v1.wrapping_add(1i32);
            let v7: bool = &*v5 == "";
            let v9: Rc<UH0> = if v7 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v6, v9.clone());
            continue;
        }
    }
}
fn words_of_3(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v2));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v2));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v2));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v2));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v2));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v2));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v2));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v2));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v2));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v2));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v2));
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(v25.replace(&*v26, &*v2));
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v27.replace(&*v28, &*v2));
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(v29.replace(&*v30, &*v2));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(v31.replace(&*v32, &*v2));
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v33.replace(&*v34, &*v2));
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(v35.replace(&*v36, &*v2));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(v37.replace(&*v38, &*v2));
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v39.replace(&*v40, &*v2));
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = Rc::<str>::from(v41.replace(&*v42, &*v2));
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("?"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = Rc::<str>::from(v43.replace(&*v44, &*v2));
    let v46: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v45.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v47: i32 = 0i32;
    let v48: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    words_from_4(v46.clone(), v47, v48.clone())
}
fn method5(mut v0: Rc<UH0>) -> US4 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v2 == "if";
                let v6: bool = if v4 {
                    true
                } else {
                    let v5: bool = &*v2 == "else";
                    v5
                };
                let v8: bool = if v6 {
                    true
                } else {
                    let v7: bool = &*v2 == "match";
                    v7
                };
                let v10: bool = if v8 {
                    true
                } else {
                    let v9: bool = &*v2 == "for";
                    v9
                };
                let v12: bool = if v10 {
                    true
                } else {
                    let v11: bool = &*v2 == "while";
                    v11
                };
                let mut v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = &*v2 == "loop";
                    v13
                };
                let v16: bool = if v14 {
                    true
                } else {
                    let v15: bool = &*v2 == "let";
                    v15
                };
                let v18: bool = if v16 {
                    true
                } else {
                    let v17: bool = &*v2 == "return";
                    v17
                };
                let v20: bool = if v18 {
                    true
                } else {
                    let v19: bool = &*v2 == "fn";
                    v19
                };
                if v20 {
                    return US4::US4_Some(v2.clone());
                } else {
                    v0 = v3.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn method6(mut v0: Rc<UH0>) -> US4 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v2 == "as";
                if v4 {
                    return US4::US4_Some(v2.clone());
                } else {
                    v0 = v3.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US4::US4_None;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method10(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn operator_between_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> bool {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: i32 = v1.wrapping_add(1i32);
        let v4: bool = v2 <= v3;
        if v4 {
            return false;
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method9(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method10(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: i32 = (v13.clone().len() as i32);
            let v15: bool = v14 == 0i32;
            let v22: Rc<str> = if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v16.clone()
            } else {
                let v17: i32 = v14.wrapping_sub(1i32);
                let v18: usize = ((v17) as usize);
                let v19: i32 = v17.wrapping_add(1i32);
                let v20: usize = ((v19) as usize);
                let v21: Rc<str> = Rc::<str>::from(&(v13)[v18..v20]);
                v21.clone()
            };
            let mut v23: Rc<str> = v0.clone().borrow()[v3 as usize].clone();
            let mut v24: i32 = (v23.clone().len() as i32);
            let v25: i32 = 0i32;
            let v26: i32 = method9(v23.clone(), v24, v25);
            let v27: i32 = v24.wrapping_sub(1i32);
            let v28: Rc<str> = string_slice(&v23.clone(), v26 as i64, v27 as i64);
            let v29: i32 = (v28.clone().len() as i32);
            let v30: i32 = method10(v28.clone(), v29);
            let v31: Rc<str> = string_slice(&v28.clone(), 0i32 as i64, v30 as i64);
            let v32: bool = &*v31 == "";
            let v37: Rc<str> = if v32 {
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v33.clone()
            } else {
                let v34: usize = ((0i32) as usize);
                let v35: usize = ((1i32) as usize);
                let v36: Rc<str> = Rc::<str>::from(&(v31)[v34..v35]);
                v36.clone()
            };
            let v38: bool = &*v22 != "";
            let v41: bool = if v38 {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"); } LIT.with(|lit| lit.clone()) };
                let v40: bool = v39.contains(&*v22);
                v40
            } else {
                false
            };
            let v43: bool = if v41 {
                true
            } else {
                let v42: bool = &*v22 == ")";
                v42
            };
            let v52: bool = if v43 {
                let v44: bool = &*v37 != "";
                let v47: bool = if v44 {
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"); } LIT.with(|lit| lit.clone()) };
                    let v46: bool = v45.contains(&*v37);
                    v46
                } else {
                    false
                };
                let v49: bool = if v47 {
                    true
                } else {
                    let v48: bool = &*v37 == "!";
                    v48
                };
                if v49 {
                    true
                } else {
                    let v50: bool = &*v37 == "(";
                    v50
                }
            } else {
                false
            };
            if v52 {
                return true;
            } else {
                (v0, v1) = (v0.clone(), v3);
                continue;
            }
        }
    }
}
fn arith_in_7(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: bool = operator_between_8(v2.clone(), v3);
    let v9: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = 0i32;
        operator_between_8(v6.clone(), v7)
    };
    let mut v14: bool = if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v10).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v12: i32 = 0i32;
        operator_between_8(v11.clone(), v12)
    };
    let v19: bool = if v14 {
        true
    } else {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v17: i32 = 0i32;
        operator_between_8(v16.clone(), v17)
    };
    if v19 {
        true
    } else {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v20).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v22: i32 = 0i32;
        operator_between_8(v21.clone(), v22)
    }
}
fn term_class_2(mut v0: Rc<str>) -> US2 {
    let v1: Rc<UH0> = words_of_3(v0.clone());
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("=>"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v0.contains(&*v2);
    let v6: bool = if v3 {
        true
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0.contains(&*v4);
        v5
    };
    let v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v0.contains(&*v7);
        v8
    };
    let v13: bool = if v9 {
        true
    } else {
        let v10: US4 = method5(v1.clone());
        let v11: bool = match &v10 {
            US4::US4_None => {
                true
            }
            _ => {
                false
            }
        };
        let v12: bool = v11 != true;
        v12
    };
    if v13 {
        let mut v14: US3 = US3::US3_MacroFlow;
        US2::US2_Some(v14.clone())
    } else {
        let v16: US4 = method6(v1.clone());
        let v17: bool = match &v16 {
            US4::US4_None => {
                true
            }
            _ => {
                false
            }
        };
        let v18: bool = v17 != true;
        if v18 {
            let v19: US3 = US3::US3_MacroCast;
            US2::US2_Some(v19.clone())
        } else {
            let v21: bool = arith_in_7(v0.clone());
            let mut v24: bool = if v21 {
                true
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("=="); } LIT.with(|lit| lit.clone()) };
                let mut v23: bool = v0.contains(&*v22);
                v23
            };
            let v27: bool = if v24 {
                true
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!="); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v0.contains(&*v25);
                v26
            };
            if v27 {
                let v28: US3 = US3::US3_MacroArith;
                US2::US2_Some(v28.clone())
            } else {
                US2::US2_None
            }
        }
    }
}
fn outside_quotes_12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            (v0, v1, v2) = (v0.clone(), v5, v9.clone());
            continue;
        }
    }
}
fn unquoted_body_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 >= 3i32;
    let v9: Rc<str> = if v2 {
        let v3: i32 = v1.wrapping_sub(3i32);
        let v4: usize = ((2i32) as usize);
        let v5: i32 = 2i32.wrapping_add(v3);
        let v6: usize = ((v5) as usize);
        let v7: Rc<str> = Rc::<str>::from(&(v0)[v4..v6]);
        v7.clone()
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v8.clone()
    };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v9.split(&*v10).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v12: i32 = 0i32;
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    outside_quotes_12(v11.clone(), v12, v13.clone())
}
fn class_of_1(mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1) -> US2 {
    let v6: i32 = (v3.clone().len() as i32);
    let v7: bool = v6 > 120i32;
    match &v5 {
        US1::US1_GlobalBody => {
            let v8: US3 = US3::US3_MacroBlock;
            US2::US2_Some(v8)
        }
        US1::US1_GlobalDeclaration => {
            if v7 {
                let v10: US3 = US3::US3_MacroLong;
                US2::US2_Some(v10)
            } else {
                US2::US2_None
            }
        }
        US1::US1_InlineMacro => {
            if v7 {
                let mut v14: US3 = US3::US3_MacroLong;
                US2::US2_Some(v14)
            } else {
                let v16: bool = v4 == false;
                if v16 {
                    let v17: US3 = US3::US3_MacroBlock;
                    US2::US2_Some(v17)
                } else {
                    match &v2 {
                        US0::US0_TermMacro => {
                            let v20: Rc<str> = unquoted_body_11(v3);
                            term_class_2(v20)
                        }
                        US0::US0_TypeMacro => {
                            US2::US2_None
                        }
                    }
                }
            }
        }
    }
}
fn classified_0(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> i32 {
    let v4: bool = v2 == 1i32;
    let v7: US0 = if v4 {
        US0::US0_TypeMacro
    } else {
        US0::US0_TermMacro
    };
    let v8: bool = v3 == 1i32;
    let mut v14: US1 = if v8 {
        US1::US1_GlobalDeclaration
    } else {
        let v10: bool = v3 == 2i32;
        if v10 {
            US1::US1_GlobalBody
        } else {
            US1::US1_InlineMacro
        }
    };
    let v15: i32 = 0i32;
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: US2 = class_of_1(v15, v16.clone(), v7.clone(), v0.clone(), v1, v14.clone());
    match &v17 {
        US2::US2_None => {
            -1i32
        }
        US2::US2_Some(v18) => {
            let v18: US3 = v18.clone();
            match &v18 {
                US3::US3_MacroArith => {
                    4i32
                }
                US3::US3_MacroBlock => {
                    1i32
                }
                US3::US3_MacroCast => {
                    3i32
                }
                US3::US3_MacroFlow => {
                    2i32
                }
                US3::US3_MacroLong => {
                    0i32
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, bool, i32, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, bool, i32, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32| -> i32 {
        classified_0(v0.clone(), v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn class_word_16(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_MacroArith => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US3::US3_MacroBlock => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US3::US3_MacroCast => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_MacroFlow => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US3::US3_MacroLong => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: US3, mut v4: Rc<str>, mut v5: Rc<UH1>) -> US5 {
    loop {
        match &*v5 {
            UH1::UH1_Cons(v7, v8, v9, v10) => {
                let v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let v10: Rc<UH1> = v10.clone();
                let v11: bool = &*v7 == &*v0;
                let v13: bool = if v11 {
                    let v12: bool = &*v8 == &*v2;
                    v12
                } else {
                    false
                };
                let v16: bool = if v13 {
                    let mut v14: Rc<str> = class_word_16(v3.clone());
                    let v15: bool = &*v9 == &*v14;
                    v15
                } else {
                    false
                };
                if v16 {
                    return US5::US5_Some(v7.clone(), v8.clone(), v9.clone());
                } else {
                    (v0, v1, v2, v3, v4, v5) = (v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v10.clone());
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US5::US5_None;
            }
        }
    }
}
fn known_14(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US3, mut v5: Rc<str>) -> bool {
    let v6: US5 = method15(v1, v2, v3, v4, v5, v0);
    let v7: bool = match &v6 {
        US5::US5_None => {
            true
        }
        _ => {
            false
        }
    };
    let v8: bool = v7 != true;
    v8
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
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
fn words_in_20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method21(v2.clone(), v5.clone());
        } else {
            let v7: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v8: i32 = v1.wrapping_add(1i32);
            let v9: bool = &*v7 == "";
            let v11: Rc<UH0> = if v9 {
                v2.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()))
            };
            (v0, v1, v2) = (v0.clone(), v8, v11.clone());
            continue;
        }
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn known_of_19(mut v0: Rc<str>) -> US5 {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: i32 = method9(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method10(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v7 as i64);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v8.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v11: i32 = 0i32;
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = words_in_20(v10.clone(), v11, v12.clone());
    match &*v13 {
        UH0::UH0_Cons(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let v15: Rc<UH0> = v15.clone();
            match &*v15 {
                UH0::UH0_Cons(v16, v17) => {
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<UH0> = v17.clone();
                    match &*v17 {
                        UH0::UH0_Cons(v18, v19) => {
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH0> = v19.clone();
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                            let v21: Option<usize> = v14.rfind(&*v20);
                            let v22: Option<usize> = method22(v21.clone());
                            let mut v23: Rc<dyn Fn((usize)) -> i32> = closure2();
                            let mut v24: Option<i32> = v22.map(|x| v23(x));
                            let v26: i32 = v24.unwrap_or(-1i32);
                            let v27: bool = v26 <= 0i32;
                            if v27 {
                                US5::US5_None
                            } else {
                                let v29: usize = ((0i32) as usize);
                                let v30: usize = ((v26) as usize);
                                let v31: Rc<str> = Rc::<str>::from(&(v14)[v29..v30]);
                                US5::US5_Some(v31.clone(), v16.clone(), v18.clone())
                            }
                        }
                        _ => {
                            US5::US5_None
                        }
                    }
                }
                _ => {
                    US5::US5_None
                }
            }
        }
        _ => {
            US5::US5_None
        }
    }
}
fn baseline_from_18(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: US5 = known_of_19(v5.clone());
            match &v6 {
                US5::US5_None => {
                    let v13: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v13, v2.clone());
                    continue;
                }
                US5::US5_Some(v7, v8, v9) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = v9.clone();
                    let v10: i32 = v1.wrapping_add(1i32);
                    let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v7.clone(), v8.clone(), v9.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v10, v11.clone());
                    continue;
                }
            }
        }
    }
}
fn baseline_of_17(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v6: i32 = 0i32;
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    baseline_from_18(v5.clone(), v6, v7.clone())
}
fn known_in_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> bool {
    let v4: bool = v3 == 0i32;
    let mut v24: US2 = if v4 {
        let v5: US3 = US3::US3_MacroLong;
        US2::US2_Some(v5.clone())
    } else {
        let v7: bool = v3 == 1i32;
        if v7 {
            let v8: US3 = US3::US3_MacroBlock;
            US2::US2_Some(v8.clone())
        } else {
            let v10: bool = v3 == 2i32;
            if v10 {
                let v11: US3 = US3::US3_MacroFlow;
                US2::US2_Some(v11.clone())
            } else {
                let v13: bool = v3 == 3i32;
                if v13 {
                    let mut v14: US3 = US3::US3_MacroCast;
                    US2::US2_Some(v14.clone())
                } else {
                    let v16: bool = v3 == 4i32;
                    if v16 {
                        let v17: US3 = US3::US3_MacroArith;
                        US2::US2_Some(v17.clone())
                    } else {
                        US2::US2_None
                    }
                }
            }
        }
    };
    match &v24 {
        US2::US2_None => {
            false
        }
        US2::US2_Some(v25) => {
            let v25: US3 = v25.clone();
            let v26: Rc<UH1> = baseline_of_17(v0.clone());
            let v27: i32 = 0i32;
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            known_14(v26.clone(), v1.clone(), v27, v2.clone(), v25.clone(), v28.clone())
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, i32) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32| -> bool {
        known_in_13(v0.clone(), v1.clone(), v2.clone(), v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_class(v0: &str, v1: bool, v2: i32, v3: i32) -> i32 {
    closure0()(Rc::<str>::from(v0), v1, v2, v3)
}
pub fn kit_lint_known(v0: &str, v1: &str, v2: &str, v3: i32) -> bool {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3)
}
