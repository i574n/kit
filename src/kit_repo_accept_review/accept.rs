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
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_LiteralChange,
    US0_BooleanChange,
    US0_ComparisonChange,
    US0_ShapeChange,
}
#[derive(Clone)]
enum US1 {
    US1_WordToken,
    US1_NumberToken,
    US1_OperatorToken,
    US1_MarkToken,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US1, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
fn method2(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method5(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn token_change_9(mut v0: US1, mut v1: Rc<str>, mut v2: US1, mut v3: Rc<str>) -> US0 {
    match &v0 {
        US1::US1_NumberToken => {
            match &v2 {
                US1::US1_NumberToken => {
                    US0::US0_LiteralChange
                }
                _ => {
                    US0::US0_ShapeChange
                }
            }
        }
        US1::US1_OperatorToken => {
            match &v2 {
                US1::US1_OperatorToken => {
                    let mut v19: bool = &*v1 == "<";
                    let v21: bool = if v19 {
                        true
                    } else {
                        let mut v20: bool = &*v1 == ">";
                        v20
                    };
                    let v23: bool = if v21 {
                        true
                    } else {
                        let v22: bool = &*v1 == "<=";
                        v22
                    };
                    let v25: bool = if v23 {
                        true
                    } else {
                        let v24: bool = &*v1 == ">=";
                        v24
                    };
                    let v27: bool = if v25 {
                        true
                    } else {
                        let v26: bool = &*v1 == "=";
                        v26
                    };
                    let v29: bool = if v27 {
                        true
                    } else {
                        let v28: bool = &*v1 == "<>";
                        v28
                    };
                    let v31: bool = if v29 {
                        true
                    } else {
                        let v30: bool = &*v1 == "!=";
                        v30
                    };
                    let v33: bool = if v31 {
                        true
                    } else {
                        let v32: bool = &*v1 == "==";
                        v32
                    };
                    let v49: bool = if v33 {
                        true
                    } else {
                        let v34: bool = &*v3 == "<";
                        let v36: bool = if v34 {
                            true
                        } else {
                            let v35: bool = &*v3 == ">";
                            v35
                        };
                        let v38: bool = if v36 {
                            true
                        } else {
                            let v37: bool = &*v3 == "<=";
                            v37
                        };
                        let v40: bool = if v38 {
                            true
                        } else {
                            let v39: bool = &*v3 == ">=";
                            v39
                        };
                        let v42: bool = if v40 {
                            true
                        } else {
                            let v41: bool = &*v3 == "=";
                            v41
                        };
                        let v44: bool = if v42 {
                            true
                        } else {
                            let v43: bool = &*v3 == "<>";
                            v43
                        };
                        let v46: bool = if v44 {
                            true
                        } else {
                            let v45: bool = &*v3 == "!=";
                            v45
                        };
                        if v46 {
                            true
                        } else {
                            let v47: bool = &*v3 == "==";
                            v47
                        }
                    };
                    if v49 {
                        US0::US0_ComparisonChange
                    } else {
                        US0::US0_ShapeChange
                    }
                }
                _ => {
                    US0::US0_ShapeChange
                }
            }
        }
        US1::US1_WordToken => {
            match &v2 {
                US1::US1_WordToken => {
                    let v7: bool = &*v1 == "true";
                    let mut v9: bool = if v7 {
                        true
                    } else {
                        let mut v8: bool = &*v1 == "false";
                        v8
                    };
                    let v13: bool = if v9 {
                        let mut v10: bool = &*v3 == "true";
                        if v10 {
                            true
                        } else {
                            let mut v11: bool = &*v3 == "false";
                            v11
                        }
                    } else {
                        false
                    };
                    if v13 {
                        US0::US0_BooleanChange
                    } else {
                        US0::US0_ShapeChange
                    }
                }
                _ => {
                    US0::US0_ShapeChange
                }
            }
        }
        _ => {
            US0::US0_ShapeChange
        }
    }
}
fn tokens_change_8(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: US0) -> US0 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v5, v6, v7) => {
                let v5: US1 = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let v7: Rc<UH0> = v7.clone();
                match &*v1 {
                    UH0::UH0_Cons(v8, v9, v10) => {
                        let mut v8: US1 = v8.clone();
                        let mut v9: Rc<str> = v9.clone();
                        let mut v10: Rc<UH0> = v10.clone();
                        let mut v11: bool = &*v6 == &*v9;
                        let v23: US0 = if v11 {
                            v2.clone()
                        } else {
                            let v12: US0 = token_change_9(v5.clone(), v6.clone(), v8.clone(), v9.clone());
                            let v16: i32 = match &v12 {
                                US0::US0_BooleanChange => {
                                    2i32
                                }
                                US0::US0_ComparisonChange => {
                                    3i32
                                }
                                US0::US0_LiteralChange => {
                                    1i32
                                }
                                US0::US0_ShapeChange => {
                                    4i32
                                }
                            };
                            let mut v20: i32 = match &v2 {
                                US0::US0_BooleanChange => {
                                    2i32
                                }
                                US0::US0_ComparisonChange => {
                                    3i32
                                }
                                US0::US0_LiteralChange => {
                                    1i32
                                }
                                US0::US0_ShapeChange => {
                                    4i32
                                }
                            };
                            let v21: bool = v16 > v20;
                            if v21 {
                                v12.clone()
                            } else {
                                v2.clone()
                            }
                        };
                        (v0, v1, v2) = (v7.clone(), v10.clone(), v23.clone());
                        continue;
                    }
                    _ => {
                        return US0::US0_ShapeChange;
                    }
                }
            }
            UH0::UH0_Nil => {
                match &*v1 {
                    UH0::UH0_Nil => {
                        return v2.clone();
                    }
                    _ => {
                        return US0::US0_ShapeChange;
                    }
                }
            }
        }
    }
}
fn method12(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let v2: US1 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn((char)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> i32> = Rc::new(move |mut v0: (char)| -> i32 {
        let mut v1: char = (v0);
        let v2: usize = v1.len_utf8();
        let mut v3: i32 = ((v2) as i32);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_end_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<dyn Fn(u32) -> bool>) -> i32 {
    loop {
        let v4: bool = v1 < v2;
        let v13: bool = if v4 {
            let v5: usize = ((v1) as usize);
            let mut v6: Option<char> = (&*v0)[v5..].chars().next();
            let v7: Option<char> = method13(v6.clone());
            let mut v8: Rc<dyn Fn((char)) -> u32> = closure1();
            let mut v9: Option<u32> = v7.map(|x| v8(x));
            let mut v11: u32 = v9.unwrap_or(0u32);
            v3(v11)
        } else {
            false
        };
        if v13 {
            let v14: usize = ((v1) as usize);
            let v15: Option<char> = (&*v0)[v14..].chars().next();
            let v16: Option<char> = method13(v15.clone());
            let mut v17: Rc<dyn Fn((char)) -> i32> = closure2();
            let mut v18: Option<i32> = v16.map(|x| v17(x));
            let mut v20: i32 = v18.unwrap_or(1i32);
            let v21: i32 = v1.wrapping_add(v20);
            (v0, v1, v2, v3) = (v0.clone(), v21, v2, v3.clone());
            continue;
        } else {
            return v1;
        }
    }
}
fn closure3() -> Rc<dyn Fn(u32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u32) -> bool> = Rc::new(move |mut v0: u32| -> bool {
        let mut v1: bool = 48u32 <= v0;
        if v1 {
            let v2: bool = v0 <= 57u32;
            v2
        } else {
            false
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(u32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u32) -> bool> = Rc::new(move |mut v0: u32| -> bool {
        let mut v1: bool = 65u32 <= v0;
        let mut v3: bool = if v1 {
            let v2: bool = v0 <= 90u32;
            v2
        } else {
            false
        };
        let v7: bool = if v3 {
            true
        } else {
            let v4: bool = 97u32 <= v0;
            if v4 {
                let v5: bool = v0 <= 122u32;
                v5
            } else {
                false
            }
        };
        let mut v9: bool = if v7 {
            true
        } else {
            let mut v8: bool = v0 == 95u32;
            v8
        };
        if v9 {
            true
        } else {
            let mut v10: bool = v0 > 127u32;
            v10
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_end_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    let mut v3: Rc<dyn Fn(u32) -> bool> = closure3();
    let v4: i32 = run_end_15(v0.clone(), v1, v2, v3.clone());
    let v5: i32 = v4.wrapping_add(1i32);
    let mut v6: bool = v5 < v2;
    let v15: bool = if v6 {
        let v7: usize = ((v4) as usize);
        let mut v8: Option<char> = (&*v0)[v7..].chars().next();
        let mut v9: Option<char> = method13(v8.clone());
        let mut v10: Rc<dyn Fn((char)) -> u32> = closure1();
        let mut v11: Option<u32> = v9.map(|x| v10(x));
        let v13: u32 = v11.unwrap_or(0u32);
        let v14: bool = v13 == 46u32;
        v14
    } else {
        false
    };
    let v26: bool = if v15 {
        let v16: usize = ((v5) as usize);
        let mut v17: Option<char> = (&*v0)[v16..].chars().next();
        let mut v18: Option<char> = method13(v17.clone());
        let mut v19: Rc<dyn Fn((char)) -> u32> = closure1();
        let mut v20: Option<u32> = v18.map(|x| v19(x));
        let v22: u32 = v20.unwrap_or(0u32);
        let v23: bool = 48u32 <= v22;
        if v23 {
            let v24: bool = v22 <= 57u32;
            v24
        } else {
            false
        }
    } else {
        false
    };
    let v28: i32 = if v26 {
        run_end_15(v0.clone(), v5, v2, v3.clone())
    } else {
        v4
    };
    let v29: Rc<dyn Fn(u32) -> bool> = closure4();
    let v30: i32 = run_end_15(v0.clone(), v28, v2, v29.clone());
    run_end_15(v0.clone(), v30, v2, v3.clone())
}
fn closure5() -> Rc<dyn Fn(u32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u32) -> bool> = Rc::new(move |mut v0: u32| -> bool {
        let mut v1: bool = 65u32 <= v0;
        let mut v3: bool = if v1 {
            let v2: bool = v0 <= 90u32;
            v2
        } else {
            false
        };
        let v7: bool = if v3 {
            true
        } else {
            let v4: bool = 97u32 <= v0;
            if v4 {
                let v5: bool = v0 <= 122u32;
                v5
            } else {
                false
            }
        };
        let mut v9: bool = if v7 {
            true
        } else {
            let mut v8: bool = v0 == 95u32;
            v8
        };
        let mut v11: bool = if v9 {
            true
        } else {
            let mut v10: bool = v0 > 127u32;
            v10
        };
        let v15: bool = if v11 {
            true
        } else {
            let v12: bool = 48u32 <= v0;
            if v12 {
                let v13: bool = v0 <= 57u32;
                v13
            } else {
                false
            }
        };
        if v15 {
            true
        } else {
            let v16: bool = v0 == 39u32;
            v16
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(u32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u32) -> bool> = Rc::new(move |mut v0: u32| -> bool {
        let mut v1: bool = v0 == 60u32;
        let mut v3: bool = if v1 {
            true
        } else {
            let v2: bool = v0 == 62u32;
            v2
        };
        let v5: bool = if v3 {
            true
        } else {
            let v4: bool = v0 == 61u32;
            v4
        };
        if v5 {
            true
        } else {
            let mut v6: bool = v0 == 33u32;
            v6
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tokens_from_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method12(v3.clone(), v5.clone());
        } else {
            let v7: usize = ((v1) as usize);
            let mut v8: Option<char> = (&*v0)[v7..].chars().next();
            let mut v9: Option<char> = method13(v8.clone());
            let mut v10: Rc<dyn Fn((char)) -> u32> = closure1();
            let mut v11: Option<u32> = v9.map(|x| v10(x));
            let v13: u32 = v11.unwrap_or(0u32);
            let v14: bool = v13 == 32u32;
            let v16: bool = if v14 {
                true
            } else {
                let v15: bool = v13 == 9u32;
                v15
            };
            let mut v18: bool = if v16 {
                true
            } else {
                let mut v17: bool = v13 == 13u32;
                v17
            };
            if v18 {
                let mut v19: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v19, v2, v3.clone());
                continue;
            } else {
                let v21: bool = 48u32 <= v13;
                let v23: bool = if v21 {
                    let v22: bool = v13 <= 57u32;
                    v22
                } else {
                    false
                };
                let (mut v63, mut v64): (US1, i32) = if v23 {
                    let v24: i32 = number_end_14(v0.clone(), v1, v2);
                    let v25: US1 = US1::US1_NumberToken;
                    (v25.clone(), v24)
                } else {
                    let v26: bool = 65u32 <= v13;
                    let v28: bool = if v26 {
                        let v27: bool = v13 <= 90u32;
                        v27
                    } else {
                        false
                    };
                    let v32: bool = if v28 {
                        true
                    } else {
                        let v29: bool = 97u32 <= v13;
                        if v29 {
                            let v30: bool = v13 <= 122u32;
                            v30
                        } else {
                            false
                        }
                    };
                    let v34: bool = if v32 {
                        true
                    } else {
                        let v33: bool = v13 == 95u32;
                        v33
                    };
                    let v36: bool = if v34 {
                        true
                    } else {
                        let v35: bool = v13 > 127u32;
                        v35
                    };
                    if v36 {
                        let v37: Rc<dyn Fn(u32) -> bool> = closure5();
                        let v38: i32 = run_end_15(v0.clone(), v1, v2, v37.clone());
                        let v39: US1 = US1::US1_WordToken;
                        (v39.clone(), v38)
                    } else {
                        let v40: bool = v13 == 60u32;
                        let v42: bool = if v40 {
                            true
                        } else {
                            let v41: bool = v13 == 62u32;
                            v41
                        };
                        let v44: bool = if v42 {
                            true
                        } else {
                            let v43: bool = v13 == 61u32;
                            v43
                        };
                        let v46: bool = if v44 {
                            true
                        } else {
                            let v45: bool = v13 == 33u32;
                            v45
                        };
                        if v46 {
                            let v47: Rc<dyn Fn(u32) -> bool> = closure6();
                            let v48: i32 = run_end_15(v0.clone(), v1, v2, v47.clone());
                            let v49: US1 = US1::US1_OperatorToken;
                            (v49.clone(), v48)
                        } else {
                            let v50: usize = ((v1) as usize);
                            let v51: Option<char> = (&*v0)[v50..].chars().next();
                            let v52: Option<char> = method13(v51.clone());
                            let mut v53: Rc<dyn Fn((char)) -> i32> = closure2();
                            let mut v54: Option<i32> = v52.map(|x| v53(x));
                            let v56: i32 = v54.unwrap_or(1i32);
                            let v57: i32 = v1.wrapping_add(v56);
                            let v58: US1 = US1::US1_MarkToken;
                            (v58.clone(), v57)
                        }
                    }
                };
                let v65: i32 = v64.wrapping_sub(v1);
                let v66: usize = ((v1) as usize);
                let v67: i32 = v1.wrapping_add(v65);
                let v68: usize = ((v67) as usize);
                let v69: Rc<str> = Rc::<str>::from(&(v0)[v66..v68]);
                let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63.clone(), v69.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v64, v2, v70.clone());
                continue;
            }
        }
    }
}
fn tokens_of_10(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: i32 = 0i32;
    let v2: i32 = (v0.clone().len() as i32);
    let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    tokens_from_11(v0.clone(), v1, v2, v3.clone())
}
fn line_change_7(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: Rc<UH0> = tokens_of_10(v0);
    let mut v3: Rc<UH0> = tokens_of_10(v1);
    let v4: US0 = US0::US0_LiteralChange;
    tokens_change_8(v2, v3, v4)
}
fn method16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                        let mut v8: bool = v4 == b'\n';
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
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
fn reviewed_lines_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: bool, mut v4: Rc<str>) -> (bool, Rc<str>) {
    let v5: i32 = (v0.clone().borrow().len() as i32);
    let mut v6: bool = v5 <= v2;
    let (mut v55, mut v56): (bool, Rc<str>) = if v6 {
        (v3, v4.clone())
    } else {
        let v7: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
        let mut v8: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
        let mut v9: bool = &*v7 == &*v8;
        let (mut v53, mut v54): (bool, Rc<str>) = if v9 {
            let mut v10: i32 = v2.wrapping_add(1i32);
            let (mut v11, mut v12): (bool, Rc<str>) = reviewed_lines_6(v0.clone(), v1.clone(), v10, v3, v4.clone());
            (v11, v12.clone())
        } else {
            let v13: US0 = line_change_7(v7.clone(), v8.clone());
            let v14: i32 = v2.wrapping_add(1i32);
            let v15: Rc<str> = format_real_4(v14);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let v27: Rc<str> = match &v13 {
                US0::US0_BooleanChange => {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BOOLEAN"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
                US0::US0_ComparisonChange => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("COMPARISON"); } LIT.with(|lit| lit.clone()) };
                    v22.clone()
                }
                US0::US0_LiteralChange => {
                    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("literal"); } LIT.with(|lit| lit.clone()) };
                    v20.clone()
                }
                US0::US0_ShapeChange => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SHAPE"); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
            };
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v27));
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let v31: i32 = (v8.clone().len() as i32);
            let v32: i32 = 0i32;
            let v33: i32 = method16(v8.clone(), v31, v32);
            let v34: i32 = v31.wrapping_sub(1i32);
            let v35: Rc<str> = string_slice(&v8.clone(), v33 as i64, v34 as i64);
            let v36: i32 = (v35.clone().len() as i32);
            let v37: i32 = method17(v35.clone(), v36);
            let v38: Rc<str> = string_slice(&v35.clone(), 0i32 as i64, v37 as i64);
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v38));
            let v45: bool = if v3 {
                let v43: i32 = match &v13 {
                    US0::US0_BooleanChange => {
                        2i32
                    }
                    US0::US0_ComparisonChange => {
                        3i32
                    }
                    US0::US0_LiteralChange => {
                        1i32
                    }
                    US0::US0_ShapeChange => {
                        4i32
                    }
                };
                let v44: bool = v43 == 1i32;
                v44
            } else {
                false
            };
            let v46: bool = &*v4 == "";
            let v50: Rc<str> = if v46 {
                v39.clone()
            } else {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v47)), v39));
                v49.clone()
            };
            let (mut v51, mut v52): (bool, Rc<str>) = reviewed_lines_6(v0.clone(), v1.clone(), v14, v45, v50.clone());
            (v51, v52.clone())
        };
        (v53, v54.clone())
    };
    (v55, v56.clone())
}
fn accept_review_3(mut v0: Rc<str>, mut v1: Rc<str>) -> (bool, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v3.clone().borrow().len() as i32);
    let mut v6: i32 = (v4.clone().borrow().len() as i32);
    let v7: bool = v5 != v6;
    let (mut v20, mut v21): (bool, Rc<str>) = if v7 {
        let mut v8: Rc<str> = format_real_4(v5);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SHAPE: line count "); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v8));
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
        let v13: Rc<str> = format_real_4(v6);
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        (false, v14.clone())
    } else {
        let v15: i32 = 0i32;
        let v16: bool = true;
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let (mut v18, mut v19): (bool, Rc<str>) = reviewed_lines_6(v3.clone(), v4.clone(), v15, v16, v17.clone());
        (v18, v19.clone())
    };
    (v20, v21.clone())
}
fn reviewed_files_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let v5: Vec<u8> = v3.unwrap();
    let mut v6: Vec<u8> = method2(v5.clone());
    let mut v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
    let mut v10: std::string::String = v8.unwrap();
    let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
    let v14: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v1);
    let v16: Vec<u8> = v14.unwrap();
    let mut v17: Vec<u8> = method2(v16.clone());
    let mut v19: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v17);
    let v21: std::string::String = v19.unwrap();
    let v23: Rc<str> = Rc::<str>::from(String::as_str(&v21));
    let (mut v24, mut v25): (bool, Rc<str>) = accept_review_3(v12.clone(), v23.clone());
    let v26: bool = &*v25 == "";
    if v26 {
        ()
    } else {
        println!("{}", v25);
        ()
    };
    let v29: Rc<str> = if v24 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v27.clone()
    } else {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("refused"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit repo accept-review "); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" before="); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v31, v32)), v0));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v34, v35)), v1));
    println!("{}", v37);
    if v24 {
        0i32
    } else {
        1i32
    }
}
fn command_line_argument_count_18() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_19(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn file_exists_20(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn accept_review_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_18();
    let mut v1: bool = 2i32 < v0;
    let mut v6: US2 = if v1 {
        let v2: i32 = 2i32;
        let mut v3: Rc<str> = command_line_argument_19(v2);
        US2::US2_Some(v3.clone())
    } else {
        US2::US2_None
    };
    let mut v10: Rc<str> = match &v6 {
        US2::US2_None => {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US2::US2_Some(v7) => {
            let v7: Rc<str> = v7.clone();
            v7.clone()
        }
    };
    let mut v11: i32 = command_line_argument_count_18();
    let v12: bool = 3i32 < v11;
    let mut v17: US2 = if v12 {
        let v13: i32 = 3i32;
        let v14: Rc<str> = command_line_argument_19(v13);
        US2::US2_Some(v14.clone())
    } else {
        US2::US2_None
    };
    let v21: Rc<str> = match &v17 {
        US2::US2_None => {
            let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US2::US2_Some(v18) => {
            let mut v18: Rc<str> = v18.clone();
            v18.clone()
        }
    };
    let v22: bool = &*v10 == "";
    let v24: bool = if v22 {
        true
    } else {
        let v23: bool = &*v21 == "";
        v23
    };
    if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: repo usage: repo accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v25);
        2i32
    } else {
        let v26: bool = file_exists_20(v10.clone());
        let v27: bool = v26 == false;
        if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: repo accept-review: no file at "); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v10));
            println!("{}", v29);
            2i32
        } else {
            let v30: bool = file_exists_20(v21.clone());
            let v31: bool = v30 == false;
            if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: repo accept-review: no file at "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v21));
                println!("{}", v33);
                2i32
            } else {
                reviewed_files_1(v10.clone(), v21.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        accept_review_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_repo_accept_review_run() -> i32 {
    closure0()()
}
