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
enum UH0 {
    UH0_Nil,
    UH0_Cons(i32, Rc<str>, US0, Rc<str>, bool, US1, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_MacroLong,
    US2_MacroBlock,
    US2_MacroFlow,
    US2_MacroCast,
    US2_MacroArith,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US3 {
    US3_Some(US2),
    US3_None,
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let v4: US0 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: bool = *v6;
                let v7: US1 = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v1.clone()));
                (v0, v1) = (v8.clone(), v9.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn definition_name_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    let v5: bool = if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.starts_with(&*v3);
        v4
    };
    if v5 {
        v1.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v8: i32 = (v7.clone().borrow().len() as i32);
        let v9: bool = v8 < 2i32;
        let v25: bool = if v9 {
            true
        } else {
            let v10: Rc<str> = v7.clone().borrow()[0i32 as usize].clone();
            let mut v11: bool = &*v10 == "inl";
            let mut v13: bool = if v11 {
                true
            } else {
                let v12: bool = &*v10 == "let";
                v12
            };
            let v15: bool = if v13 {
                true
            } else {
                let mut v14: bool = &*v10 == ":=";
                v14
            };
            let mut v17: bool = if v15 {
                true
            } else {
                let mut v16: bool = &*v10 == ":~";
                v16
            };
            let v19: bool = if v17 {
                true
            } else {
                let v18: bool = &*v10 == "nominal";
                v18
            };
            let v21: bool = if v19 {
                true
            } else {
                let mut v20: bool = &*v10 == "type";
                v20
            };
            let v23: bool = if v21 {
                true
            } else {
                let v22: bool = &*v10 == "union";
                v22
            };
            let mut v24: bool = v23 == false;
            v24
        };
        if v25 {
            v1.clone()
        } else {
            let v26: Rc<str> = v7.clone().borrow()[1i32 as usize].clone();
            let v27: bool = &*v26 == "rec";
            let v29: bool = if v27 {
                let v28: bool = v8 >= 3i32;
                v28
            } else {
                false
            };
            if v29 {
                let v30: Rc<str> = v7.clone().borrow()[2i32 as usize].clone();
                v30.clone()
            } else {
                let v31: Rc<str> = v7.clone().borrow()[1i32 as usize].clone();
                v31.clone()
            }
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
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
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
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
fn method8(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn global_macro_7(mut v0: i32, mut v1: Rc<str>) -> (i32, Rc<str>, US0, Rc<str>, bool, US1) {
    let v2: i32 = (v1.clone().len() as i32);
    let v3: i32 = 0i32;
    let v4: i32 = method5(v1.clone(), v2, v3);
    let v5: i32 = v2.wrapping_sub(1i32);
    let mut v6: Rc<str> = string_slice(&v1.clone(), v4 as i64, v5 as i64);
    let v7: i32 = (v6.clone().len() as i32);
    let v8: i32 = method6(v6.clone(), v7);
    let v9: Rc<str> = string_slice(&v6.clone(), 0i32 as i64, v8 as i64);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v11: Option<usize> = v9.find(&*v10);
    let v12: Option<usize> = method8(v11.clone());
    let mut v13: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v14: Option<i32> = v12.map(|x| v13(x));
    let mut v16: i32 = v14.unwrap_or(-1i32);
    let mut v17: i32 = (v9.clone().len() as i32);
    let v18: bool = v16 < 0i32;
    let v27: Rc<str> = if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19.clone()
    } else {
        let mut v20: i32 = v16.wrapping_add(1i32);
        let v21: i32 = v17.wrapping_sub(v16);
        let v22: i32 = v21.wrapping_sub(1i32);
        let v23: usize = ((v20) as usize);
        let mut v24: i32 = v20.wrapping_add(v22);
        let v25: usize = ((v24) as usize);
        let v26: Rc<str> = Rc::<str>::from(&(v9)[v23..v25]);
        v26.clone()
    };
    let v28: bool = v16 >= 0i32;
    let v31: bool = if v28 {
        let v29: bool = v27.starts_with(&*v10);
        let v30: bool = v29 == false;
        v30
    } else {
        false
    };
    let v46: bool = if v31 {
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("use "); } LIT.with(|lit| lit.clone()) };
        let v33: bool = v27.starts_with(&*v32);
        let v36: bool = if v33 {
            true
        } else {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import "); } LIT.with(|lit| lit.clone()) };
            let v35: bool = v27.starts_with(&*v34);
            v35
        };
        let v39: bool = if v36 {
            true
        } else {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("extern "); } LIT.with(|lit| lit.clone()) };
            let v38: bool = v27.starts_with(&*v37);
            v38
        };
        let v42: bool = if v39 {
            true
        } else {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mod "); } LIT.with(|lit| lit.clone()) };
            let v41: bool = v27.starts_with(&*v40);
            v41
        };
        if v42 {
            true
        } else {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            let v44: bool = v27.starts_with(&*v43);
            v44
        }
    } else {
        false
    };
    let v49: US1 = if v46 {
        US1::US1_GlobalDeclaration
    } else {
        US1::US1_GlobalBody
    };
    let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global"); } LIT.with(|lit| lit.clone()) };
    let v51: US0 = US0::US0_TermMacro;
    (v0, v50.clone(), v51.clone(), v9.clone(), true, v49.clone())
}
fn place_of_10(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: i32 = (v1.clone().len() as i32);
    let v3: i32 = 0i32;
    let v4: i32 = method5(v1.clone(), v2, v3);
    let v5: i32 = v2.wrapping_sub(1i32);
    let mut v6: Rc<str> = string_slice(&v1.clone(), v4 as i64, v5 as i64);
    let v7: i32 = (v6.clone().len() as i32);
    let v8: i32 = method6(v6.clone(), v7);
    let v9: Rc<str> = string_slice(&v6.clone(), 0i32 as i64, v8 as i64);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nominal "); } LIT.with(|lit| lit.clone()) };
    let mut v11: bool = v0.starts_with(&*v10);
    let mut v14: bool = if v11 {
        true
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type "); } LIT.with(|lit| lit.clone()) };
        let mut v13: bool = v0.starts_with(&*v12);
        v13
    };
    if v14 {
        US0::US0_TypeMacro
    } else {
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let mut v17: bool = v9.ends_with(&*v16);
        let mut v20: bool = if v17 {
            true
        } else {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v9.ends_with(&*v18);
            v19
        };
        if v20 {
            US0::US0_TypeMacro
        } else {
            US0::US0_TermMacro
        }
    }
}
fn line_macros_9(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v5: i32 = (v2.clone().len() as i32);
        let mut v6: i32 = v5.wrapping_sub(v3);
        let v7: usize = ((v3) as usize);
        let v8: i32 = v3.wrapping_add(v6);
        let v9: usize = ((v8) as usize);
        let v10: Rc<str> = Rc::<str>::from(&(v2)[v7..v9]);
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$'"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Option<usize> = v10.find(&*v13);
        let v15: Option<usize> = method8(v14.clone());
        let mut v16: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v17: Option<i32> = v15.map(|x| v16(x));
        let v19: i32 = v17.unwrap_or(-1i32);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$\""); } LIT.with(|lit| lit.clone()) };
        let v22: Option<usize> = v10.find(&*v21);
        let v23: Option<usize> = method8(v22.clone());
        let mut v24: Option<i32> = v23.map(|x| v16(x));
        let v26: i32 = v24.unwrap_or(-1i32);
        let v27: bool = v19 < 0i32;
        let v32: i32 = if v27 {
            v26
        } else {
            let v28: bool = v26 < 0i32;
            let v30: bool = if v28 {
                true
            } else {
                let v29: bool = v19 < v26;
                v29
            };
            if v30 {
                v19
            } else {
                v26
            }
        };
        let v33: bool = v32 == v19;
        let v34: Rc<str> = if v33 {
            v12.clone()
        } else {
            v20.clone()
        };
        let v35: bool = v32 < 0i32;
        if v35 {
            return v4.clone();
        } else {
            let v36: i32 = v3.wrapping_add(v32);
            let v37: usize = ((0i32) as usize);
            let v38: usize = ((v36) as usize);
            let v39: Rc<str> = Rc::<str>::from(&(v2)[v37..v38]);
            let v40: US0 = place_of_10(v2.clone(), v39.clone());
            let v41: i32 = v36.wrapping_add(2i32);
            let v42: i32 = v5.wrapping_sub(v36);
            let v43: i32 = v42.wrapping_sub(2i32);
            let v44: usize = ((v41) as usize);
            let v45: i32 = v41.wrapping_add(v43);
            let v46: usize = ((v45) as usize);
            let v47: Rc<str> = Rc::<str>::from(&(v2)[v44..v46]);
            let v48: Option<usize> = v47.find(&*v34);
            let v49: Option<usize> = method8(v48.clone());
            let mut v50: Option<i32> = v49.map(|x| v16(x));
            let v52: i32 = v50.unwrap_or(-1i32);
            let v53: bool = v52 < 0i32;
            if v53 {
                let v54: usize = ((v36) as usize);
                let v55: i32 = v36.wrapping_add(v42);
                let v56: usize = ((v55) as usize);
                let v57: Rc<str> = Rc::<str>::from(&(v2)[v54..v56]);
                let v58: US1 = US1::US1_InlineMacro;
                return Rc::new(UH0::UH0_Cons(v0, v1.clone(), v40.clone(), v57.clone(), false, v58.clone(), v4.clone()));
            } else {
                let v60: i32 = v41.wrapping_add(v52);
                let v61: i32 = v60.wrapping_sub(v36);
                let v62: i32 = v61.wrapping_add(1i32);
                let v63: usize = ((v36) as usize);
                let v64: i32 = v36.wrapping_add(v62);
                let v65: usize = ((v64) as usize);
                let v66: Rc<str> = Rc::<str>::from(&(v2)[v63..v65]);
                let v67: i32 = v60.wrapping_add(1i32);
                let v68: bool = true;
                let v69: US1 = US1::US1_InlineMacro;
                let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v1.clone(), v40.clone(), v66.clone(), v68, v69.clone(), v4.clone()));
                (v0, v1, v2, v3, v4) = (v0, v1.clone(), v2.clone(), v67, v70.clone());
                continue;
            }
        }
    }
}
fn macros_from_2(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v4 <= v1;
        if v5 {
            let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method3(v3.clone(), v6.clone());
        } else {
            let v8: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v9: Rc<str> = definition_name_4(v8.clone(), v2.clone());
            let v10: i32 = (v8.clone().len() as i32);
            let mut v11: i32 = 0i32;
            let v12: i32 = method5(v8.clone(), v10, v11);
            let mut v13: i32 = v10.wrapping_sub(1i32);
            let mut v14: Rc<str> = string_slice(&v8.clone(), v12 as i64, v13 as i64);
            let v15: i32 = (v14.clone().len() as i32);
            let mut v16: i32 = method6(v14.clone(), v15);
            let mut v17: Rc<str> = string_slice(&v14.clone(), 0i32 as i64, v16 as i64);
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global "); } LIT.with(|lit| lit.clone()) };
            let v19: bool = v17.starts_with(&*v18);
            let v22: bool = if v19 {
                true
            } else {
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("global("); } LIT.with(|lit| lit.clone()) };
                let v21: bool = v17.starts_with(&*v20);
                v21
            };
            let v27: bool = if v22 {
                true
            } else {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_global"); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_global"); } LIT.with(|lit| lit.clone()) };
                let v26: bool = v17.starts_with(&*v25);
                v26
            };
            if v27 {
                let v28: i32 = v1.wrapping_add(1i32);
                let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (i32, Rc<str>, US0, Rc<str>, bool, US1) = global_macro_7(v28, v8.clone());
                let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v30.clone(), v31.clone(), v32.clone(), v33, v34.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v28, v9.clone(), v35.clone());
                continue;
            } else {
                let v37: i32 = v1.wrapping_add(1i32);
                let v38: i32 = 0i32;
                let v39: Rc<UH0> = line_macros_9(v37, v9.clone(), v8.clone(), v38, v3.clone());
                (v0, v1, v2, v3) = (v0.clone(), v37, v9.clone(), v39.clone());
                continue;
            }
        }
    }
}
fn file_macros_1(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    macros_from_2(v5.clone(), v6, v2.clone(), v7.clone())
}
fn closure2() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: i32 = *v2;
                let v3: Rc<str> = v3.clone();
                let v4: US0 = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: bool = *v6;
                let v7: US1 = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let v9: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v8.clone(), v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32>, mut v1: Rc<RefCell<Vec<i32>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = *v4;
                let v5: Rc<str> = v5.clone();
                let mut v6: US0 = v6.clone();
                let v7: Rc<str> = v7.clone();
                let v8: bool = *v8;
                let v9: US1 = v9.clone();
                let v10: Rc<UH0> = v10.clone();
                let mut v11: i32 = v0(v4, v5.clone(), v6.clone(), v7.clone(), v8, v9.clone());
                v1.clone().borrow_mut()[v3 as usize] = v11;
                let v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v10.clone(), v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_11(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<i32>>> {
    let v2: i32 = 0i32;
    let v3: i32 = method12(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![<i32>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method13(v0, v4.clone(), v1, v5);
    v4
}
fn closure3() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> Rc<str> {
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = *v4;
                let v5: Rc<str> = v5.clone();
                let mut v6: US0 = v6.clone();
                let v7: Rc<str> = v7.clone();
                let v8: bool = *v8;
                let v9: US1 = v9.clone();
                let v10: Rc<UH0> = v10.clone();
                let mut v11: Rc<str> = v0(v4, v5.clone(), v6.clone(), v7.clone(), v8, v9.clone());
                v1.clone().borrow_mut()[v3 as usize] = v11.clone();
                let v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v10.clone(), v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_14(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let v2: i32 = 0i32;
    let v3: i32 = method12(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method15(v0, v4.clone(), v1, v5);
    v4
}
fn closure4() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> Rc<str> {
        v3.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> bool {
        v4
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool>, mut v1: Rc<RefCell<Vec<bool>>>, mut v2: Rc<UH0>, mut v3: i32) -> i32 {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5, v6, v7, v8, v9, v10) => {
                let v4: i32 = *v4;
                let v5: Rc<str> = v5.clone();
                let mut v6: US0 = v6.clone();
                let v7: Rc<str> = v7.clone();
                let v8: bool = *v8;
                let v9: US1 = v9.clone();
                let v10: Rc<UH0> = v10.clone();
                let mut v11: bool = v0(v4, v5.clone(), v6.clone(), v7.clone(), v8, v9.clone());
                v1.clone().borrow_mut()[v3 as usize] = v11;
                let v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v10.clone(), v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v3;
            }
        }
    }
}
fn filled_16(mut v0: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool>, mut v1: Rc<UH0>) -> Rc<RefCell<Vec<bool>>> {
    let v2: i32 = 0i32;
    let v3: i32 = method12(v1.clone(), v2);
    let v4: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![<bool>::default(); v3 as usize]));
    let v5: i32 = 0i32;
    let mut v6: i32 = method17(v0, v4.clone(), v1, v5);
    v4
}
fn closure6() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        match &v2 {
            US0::US0_TermMacro => {
                0i32
            }
            US0::US0_TypeMacro => {
                1i32
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: Rc<str>, mut v4: bool, mut v5: US1| -> i32 {
        match &v5 {
            US1::US1_GlobalBody => {
                2i32
            }
            US1::US1_GlobalDeclaration => {
                1i32
            }
            US1::US1_InlineMacro => {
                0i32
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn scanned_0(mut v0: Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    let v1: Rc<UH0> = file_macros_1(v0);
    let v3: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure2();
    let v4: Rc<RefCell<Vec<i32>>> = filled_11(v3, v1.clone());
    let mut v6: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = closure3();
    let v7: Rc<RefCell<Vec<Rc<str>>>> = filled_14(v6, v1.clone());
    let v8: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> Rc<str>> = closure4();
    let v9: Rc<RefCell<Vec<Rc<str>>>> = filled_14(v8, v1.clone());
    let mut v11: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> bool> = closure5();
    let v12: Rc<RefCell<Vec<bool>>> = filled_16(v11, v1.clone());
    let mut v14: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure6();
    let v15: Rc<RefCell<Vec<i32>>> = filled_11(v14, v1.clone());
    let mut v17: Rc<dyn Fn(i32, Rc<str>, US0, Rc<str>, bool, US1) -> i32> = closure7();
    let v18: Rc<RefCell<Vec<i32>>> = filled_11(v17, v1);
    (v4, v7, v9, v12, v15, v18)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>)> = Rc::new(move |mut v0: Rc<str>| -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
        scanned_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn added_from_19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            return method20(v2.clone(), v5.clone());
        } else {
            let v7: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v8: i32 = (v7.clone().len() as i32);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v7.starts_with(&*v9);
            let mut v14: bool = if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("+++"); } LIT.with(|lit| lit.clone()) };
                let v12: bool = v7.starts_with(&*v11);
                let mut v13: bool = v12 == false;
                v13
            } else {
                false
            };
            let v15: i32 = v1.wrapping_add(1i32);
            let v22: Rc<UH1> = if v14 {
                let mut v16: i32 = v8.wrapping_sub(1i32);
                let mut v17: usize = ((1i32) as usize);
                let v18: i32 = 1i32.wrapping_add(v16);
                let v19: usize = ((v18) as usize);
                let mut v20: Rc<str> = Rc::<str>::from(&(v7)[v17..v19]);
                Rc::new(UH1::UH1_Cons(v20.clone(), v2.clone()))
            } else {
                v2.clone()
            };
            (v0, v1, v2) = (v0.clone(), v15, v22.clone());
            continue;
        }
    }
}
fn method21(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn added_text_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v3.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH1> = added_from_19(v5.clone(), v6, v7.clone());
    let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let mut v11: i32 = method21(v9.clone(), v8.clone(), v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let mut v14: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v4));
    v14.clone()
}
fn closure8() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        added_text_18(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure10(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let v4: bool = v2 >= v0;
        if v4 {
            v3.clone()
        } else {
            let v5: i32 = v2.wrapping_add(1i32);
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method24(v0, v1.clone(), v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
            v6(v7.clone())
        }
    })
}
fn method24(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure10(v0, v1, v2)
}
fn class_word_25(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_MacroArith => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("arith"); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
        US2::US2_MacroBlock => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("block"); } LIT.with(|lit| lit.clone()) };
            v2.clone()
        }
        US2::US2_MacroCast => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cast"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_MacroFlow => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("flow"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
        US2::US2_MacroLong => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("long"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
    }
}
fn method27(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_26(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method27(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn violation_row_23(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: US2, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: i32 = v0.wrapping_add(2i32);
    let v7: Rc<str> = Rc::<str>::from((b' ' as char).encode_utf8(&mut [0u8; 4]) as &str);
    let v8: i32 = (v3.clone().len() as i32);
    let v9: i32 = v6.wrapping_sub(v8);
    let v10: i32 = 0i32;
    let mut v11: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method24(v9, v7.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = v11(v12.clone());
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v13));
    let v15: Rc<str> = class_word_25(v4.clone());
    let mut v16: Rc<str> = Rc::<str>::from((b' ' as char).encode_utf8(&mut [0u8; 4]) as &str);
    let mut v17: i32 = (v15.clone().len() as i32);
    let v18: i32 = 7i32.wrapping_sub(v17);
    let v19: i32 = 0i32;
    let mut v20: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method24(v18, v16.clone(), v19);
    let v21: Rc<str> = v20(v12.clone());
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v23));
    let v25: Rc<str> = format_real_26(v2);
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v27)), v14)), v22)), v5));
    v31.clone()
}
fn row_of_22(mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>) -> Rc<str> {
    let mut v6: bool = v4 == 0i32;
    let v26: US3 = if v6 {
        let v7: US2 = US2::US2_MacroLong;
        US3::US3_Some(v7.clone())
    } else {
        let v9: bool = v4 == 1i32;
        if v9 {
            let v10: US2 = US2::US2_MacroBlock;
            US3::US3_Some(v10.clone())
        } else {
            let v12: bool = v4 == 2i32;
            if v12 {
                let mut v13: US2 = US2::US2_MacroFlow;
                US3::US3_Some(v13.clone())
            } else {
                let v15: bool = v4 == 3i32;
                if v15 {
                    let mut v16: US2 = US2::US2_MacroCast;
                    US3::US3_Some(v16.clone())
                } else {
                    let v18: bool = v4 == 4i32;
                    if v18 {
                        let v19: US2 = US2::US2_MacroArith;
                        US3::US3_Some(v19.clone())
                    } else {
                        US3::US3_None
                    }
                }
            }
        }
    };
    match &v26 {
        US3::US3_None => {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29.clone()
        }
        US3::US3_Some(v27) => {
            let v27: US2 = v27.clone();
            violation_row_23(v0, v1.clone(), v2, v3.clone(), v27.clone(), v5.clone())
        }
    }
}
fn closure9() -> Rc<dyn Fn(i32, Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, i32, Rc<str>, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>| -> Rc<str> {
        row_of_22(v0, v1.clone(), v2, v3.clone(), v4, v5.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn tally_head_29(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: bool = v3 == 0i32;
    let v7: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v6.clone()
    };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  files "); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = format_real_26(v0);
    let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  macros "); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: Rc<str> = format_real_26(v1);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  violations "); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = format_real_26(v2);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new "); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: Rc<str> = format_real_26(v3);
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    v23.clone()
}
fn tally_text_28(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    tally_head_29(v0, v1, v2, v3)
}
fn closure11() -> Rc<dyn Fn(i32, i32, i32, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, i32, i32, i32) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32| -> Rc<str> {
        tally_text_28(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lint_scan(v0: &str) -> (Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<bool>>>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<i32>>>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_lint_added_text(v0: &str) -> Rc<str> {
    closure8()(Rc::<str>::from(v0))
}
pub fn kit_lint_row(v0: i32, v1: &str, v2: i32, v3: &str, v4: i32, v5: &str) -> Rc<str> {
    closure9()(v0, Rc::<str>::from(v1), v2, Rc::<str>::from(v3), v4, Rc::<str>::from(v5))
}
pub fn kit_lint_tally(v0: i32, v1: i32, v2: i32, v3: i32) -> Rc<str> {
    closure11()(v0, v1, v2, v3)
}
