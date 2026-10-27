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
fn method2() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0.clone()
}
fn codec_empty_1() -> Rc<str> {
    method2()
}
fn literal_loop_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (Rc<str>, i32, bool) {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return (v6.clone(), v1, false);
        } else {
            let v7: u8 = v0.clone().as_bytes()[v1 as usize];
            let v8: bool = v7 == b'"' ;
            if v8 {
                let v9: bool = v2 == v1 ;
                let v13: Rc<str> = if v9 {
                    codec_empty_1()
                } else {
                    let v11: i32 = v1 - 1i32 ;
                    let v12: Rc<str> = string_slice(&v0.clone(), v2 as i64, v11 as i64);
                    v12.clone()
                };
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v13.clone()));
                let v15: i32 = v1 + 1i32 ;
                return (v14.clone(), v15, true);
            } else {
                let v16: bool = v7 == b'\\' ;
                if v16 {
                    let v17: i32 = v1 + 1i32 ;
                    let v18: bool = v17 >= v4;
                    if v18 {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return (v19.clone(), v1, false);
                    } else {
                        let v20: u8 = v0.clone().as_bytes()[v17 as usize];
                        let v21: bool = v20 == b'n' ;
                        let v36: Rc<str> = if v21 {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                            v22.clone()
                        } else {
                            let v23: bool = v20 == b'r' ;
                            if v23 {
                                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                v24.clone()
                            } else {
                                let v25: bool = v20 == b't' ;
                                if v25 {
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                    v26.clone()
                                } else {
                                    let v27: bool = v20 == b'"' ;
                                    if v27 {
                                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        v28.clone()
                                    } else {
                                        let v29: bool = v20 == b'\\' ;
                                        if v29 {
                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                            v30.clone()
                                        } else {
                                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            v31.clone()
                                        }
                                    }
                                }
                            }
                        };
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v38: bool = v36 == v37 ;
                        if v38 {
                            return (v37.clone(), v1, false);
                        } else {
                            let v39: i32 = v1 + 2i32 ;
                            let v40: bool = v2 == v1 ;
                            let v44: Rc<str> = if v40 {
                                codec_empty_1()
                            } else {
                                let v42: i32 = v1 - 1i32 ;
                                let v43: Rc<str> = string_slice(&v0.clone(), v2 as i64, v42 as i64);
                                v43.clone()
                            };
                            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v44.clone()));
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45.clone(), v36.clone()));
                            (v0, v1, v2, v3) = (v0.clone(), v39, v39, v46.clone());
                            continue;
                        }
                    }
                } else {
                    let v56: i32 = v1 + 1i32 ;
                    (v0, v1, v2, v3) = (v0.clone(), v56, v2, v3.clone());
                    continue;
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32) -> (Rc<str>, i32, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> (Rc<str>, i32, bool)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> (Rc<str>, i32, bool) {
        let v2: bool = 0i32 <= v1;
        let v5: bool = if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let v4: bool = v1 < v3;
            v4
        } else {
            false
        };
        let v8: bool = if v5 {
            let v6: u8 = v0.clone().as_bytes()[v1 as usize];
            let v7: bool = v6 == b'"' ;
            v7
        } else {
            false
        };
        if v8 {
            let v9: i32 = v1 + 1i32 ;
            let v10: i32 = v1 + 1i32 ;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            literal_loop_0(v0.clone(), v9, v10, v11.clone())
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v15.clone(), v1, false)
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn encode_loop_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 == v4 ;
        if v5 {
            let v6: bool = v2 == v1 ;
            let v10: Rc<str> = if v6 {
                codec_empty_1()
            } else {
                let v8: i32 = v1 - 1i32 ;
                let v9: Rc<str> = string_slice(&v0.clone(), v2 as i64, v8 as i64);
                v9.clone()
            };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v10.clone()));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
            return v12.clone();
        } else {
            let v13: u8 = v0.clone().as_bytes()[v1 as usize];
            let v14: bool = v13 == b'"' ;
            let v29: Rc<str> = if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let v16: bool = v13 == b'\\' ;
                if v16 {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    v17.clone()
                } else {
                    let v18: bool = v13 == b'\n' ;
                    if v18 {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        v19.clone()
                    } else {
                        let v20: bool = v13 == b'\r' ;
                        if v20 {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                            v21.clone()
                        } else {
                            let v22: bool = v13 == b'\t' ;
                            if v22 {
                                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                v23.clone()
                            } else {
                                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v24.clone()
                            }
                        }
                    }
                }
            };
            let v30: i32 = v1 + 1i32 ;
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v32: bool = v29 == v31 ;
            if v32 {
                (v0, v1, v2, v3) = (v0.clone(), v30, v2, v3.clone());
                continue;
            } else {
                let v34: bool = v2 == v1 ;
                let v38: Rc<str> = if v34 {
                    codec_empty_1()
                } else {
                    let v36: i32 = v1 - 1i32 ;
                    let v37: Rc<str> = string_slice(&v0.clone(), v2 as i64, v36 as i64);
                    v37.clone()
                };
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v3.clone(), v38.clone()));
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39.clone(), v29.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v30, v30, v40.clone());
                continue;
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: i32 = 0i32;
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        encode_loop_3(v0.clone(), v1, v2, v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_predicate_literal(v0: &str, v1: i32) -> (Rc<str>, i32, bool) {
    closure0()(Rc::<str>::from(v0), v1)
}
pub fn kit_predicate_encode(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
