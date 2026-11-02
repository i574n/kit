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
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(regex::Regex),
    US1_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, regex::Regex, US1, US0, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(u64, Rc<UH3>),
}
struct Mut0 { l0: Rc<str> }
fn method2(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn named_after_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                match &*v4 {
                    UH0::UH0_Cons(v5, v6) => {
                        let v5: Rc<str> = v5.clone();
                        let mut v6: Rc<UH0> = v6.clone();
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                        let v8: Option<usize> = v5.find(&*v7);
                        let v9: Option<usize> = method2(v8.clone());
                        let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
                        let mut v11: Option<i32> = v9.map(|x| v10(x));
                        let mut v13: i32 = v11.unwrap_or(-1i32);
                        let mut v14: bool = v13 < 0i32;
                        let (mut v27, mut v28): (Rc<str>, Rc<str>) = if v14 {
                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v5.clone(), v15.clone())
                        } else {
                            let mut v16: usize = ((0i32) as usize);
                            let v17: usize = ((v13) as usize);
                            let v18: Rc<str> = Rc::<str>::from(&(v5)[v16..v17]);
                            let v19: i32 = v13.wrapping_add(1i32);
                            let v20: i32 = (v5.clone().len() as i32);
                            let v21: i32 = v20.wrapping_sub(v13);
                            let mut v22: i32 = v21.wrapping_sub(1i32);
                            let mut v23: usize = ((v19) as usize);
                            let v24: i32 = v19.wrapping_add(v22);
                            let v25: usize = ((v24) as usize);
                            let v26: Rc<str> = Rc::<str>::from(&(v5)[v23..v25]);
                            (v18.clone(), v26.clone())
                        };
                        let v29: bool = &*v3 == &*v0;
                        let v31: bool = if v29 {
                            let v30: bool = &*v27 == &*v1;
                            v30
                        } else {
                            false
                        };
                        if v31 {
                            return v28.clone();
                        } else {
                            let v32: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v6.clone()));
                            v2 = v32.clone();
                            continue;
                        }
                    }
                    _ => {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v35.clone();
                    }
                }
            }
            _ => {
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v37.clone();
            }
        }
    }
}
fn method4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return -1i32;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = b':' == v4;
            if v5 {
                let mut v6: i32 = v2.wrapping_sub(2i32);
                return v6;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        }
    }
}
fn specs_from_1(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == "--count";
                        if v6 {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                            let v8: Option<usize> = v4.find(&*v7);
                            let v9: Option<usize> = method2(v8.clone());
                            let mut v10: Rc<dyn Fn((usize)) -> i32> = closure1();
                            let mut v11: Option<i32> = v9.map(|x| v10(x));
                            let mut v13: i32 = v11.unwrap_or(-1i32);
                            let mut v14: bool = v13 < 0i32;
                            let (mut v27, mut v28): (Rc<str>, Rc<str>) = if v14 {
                                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v4.clone(), v15.clone())
                            } else {
                                let mut v16: usize = ((0i32) as usize);
                                let v17: usize = ((v13) as usize);
                                let v18: Rc<str> = Rc::<str>::from(&(v4)[v16..v17]);
                                let v19: i32 = v13.wrapping_add(1i32);
                                let v20: i32 = (v4.clone().len() as i32);
                                let v21: i32 = v20.wrapping_sub(v13);
                                let mut v22: i32 = v21.wrapping_sub(1i32);
                                let mut v23: usize = ((v19) as usize);
                                let v24: i32 = v19.wrapping_add(v22);
                                let v25: usize = ((v24) as usize);
                                let v26: Rc<str> = Rc::<str>::from(&(v4)[v23..v25]);
                                (v18.clone(), v26.clone())
                            };
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--in-set"); } LIT.with(|lit| lit.clone()) };
                            let v30: Rc<str> = named_after_3(v29.clone(), v27.clone(), v1.clone());
                            let v31: i32 = (v30.clone().len() as i32);
                            let v32: i32 = 2i32;
                            let v33: i32 = method4(v30.clone(), v31, v32);
                            let v34: i32 = v33.wrapping_add(2i32);
                            let v35: bool = v33 < 0i32;
                            let (mut v47, mut v48): (Rc<str>, Rc<str>) = if v35 {
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v30.clone(), v36.clone())
                            } else {
                                let v37: usize = ((0i32) as usize);
                                let v38: usize = ((v34) as usize);
                                let v39: Rc<str> = Rc::<str>::from(&(v30)[v37..v38]);
                                let v40: i32 = v34.wrapping_add(1i32);
                                let v41: i32 = v31.wrapping_sub(v34);
                                let v42: i32 = v41.wrapping_sub(1i32);
                                let v43: usize = ((v40) as usize);
                                let mut v44: i32 = v40.wrapping_add(v42);
                                let v45: usize = ((v44) as usize);
                                let v46: Rc<str> = Rc::<str>::from(&(v30)[v43..v45]);
                                (v39.clone(), v46.clone())
                            };
                            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count-in"); } LIT.with(|lit| lit.clone()) };
                            let v50: Rc<str> = named_after_3(v49.clone(), v27.clone(), v1.clone());
                            let v51: Rc<UH1> = specs_from_1(v5.clone(), v1.clone());
                            return Rc::new(UH1::UH1_Cons(v27.clone(), v28.clone(), v50.clone(), v47.clone(), v48.clone(), v51.clone()));
                        } else {
                            let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                            v0 = v53.clone();
                            continue;
                        }
                    }
                    _ => {
                        return { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    }
                }
            }
            _ => {
                return { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            }
        }
    }
}
fn command_line_argument_count_5() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn words_from_6(mut v0: i32, mut v1: i32) -> Rc<UH0> {
    let mut v2: bool = v0 >= v1;
    if v2 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v4: bool = v0 >= 0i32;
        let v7: bool = if v4 {
            let v5: i32 = command_line_argument_count_5();
            let mut v6: bool = v0 < v5;
            v6
        } else {
            false
        };
        let mut v11: US0 = if v7 {
            let v8: Rc<str> = command_line_argument_7(v0);
            US0::US0_Some(v8)
        } else {
            US0::US0_None
        };
        let v15: Rc<str> = match &v11 {
            US0::US0_None => {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v13
            }
            US0::US0_Some(v12) => {
                let v12: Rc<str> = v12.clone();
                v12
            }
        };
        let mut v16: i32 = v0.wrapping_add(1i32);
        let v17: Rc<UH0> = words_from_6(v16, v1);
        Rc::new(UH0::UH0_Cons(v15, v17))
    }
}
fn command_specs_0() -> Rc<UH1> {
    let mut v0: i32 = 2i32;
    let mut v1: i32 = command_line_argument_count_5();
    let mut v2: Rc<UH0> = words_from_6(v0, v1);
    specs_from_1(v2.clone(), v2)
}
fn compiled_pattern_12(mut v0: Rc<str>) -> regex::Regex {
    let mut v2: Result<regex::Regex, regex::Error> = regex::Regex::new(&v0);
    let v4: regex::Regex = v2.unwrap();
    v4
}
fn method13(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn loop_16(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    (v0, v1)
}
fn regex_captures_18(mut v0: Rc<str>, mut v1: regex::Regex) -> Vec<std::collections::HashMap<Rc<str>, Rc<str>>> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let v7: bool = true; let _capture_move = (move || { //;
    let v9: Box<std::string::String> = Box::new(v5);
    let mut v11: &'static mut std::string::String = Box::leak(v9);
    let mut v13: regex::Regex = v1.clone();
    let v15: regex::CaptureMatches = v13.captures_iter(v11);
    let v17: bool = true; let _regex_captures : Vec<_> = v15.map(|x| { //;
    let v19: regex::Captures<'static> = x;
    let v21: Box<regex::Captures<'static>> = Box::new(v19);
    let mut v23: &'static mut regex::Captures<'static> = Box::leak(v21);
    let v25: regex::CaptureNames = v1.capture_names();
    let v27: bool = true; let _regex_captures : std::collections::HashMap<_, _> = v25.map(|x| { //;
    let v29: Rc<str> = x.unwrap_or("").to_string().into();
    let v31: Rc<str> = v29.clone();
    let v33: Rc<str> = v23.name(&v29).map(|x| x.as_str()).unwrap_or("").to_string().into();
    let (mut v34, mut v35): (Rc<str>, Rc<str>) = method19(v31.clone(), v33.clone());
    let v36: (Rc<str>, Rc<str>) = (v34, v35);
    let v38: bool = true; v36 }).collect();
    let v40: std::collections::HashMap<Rc<str>, Rc<str>> = _regex_captures;
    let v42: bool = true; v40 }).collect::<Vec<_>>();
    let mut v44: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _regex_captures;
    let v46: bool = true; v44 })();
    let v48: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _capture_move;
    v48.clone()
}
fn method20(mut v0: Rc<str>) -> Rc<str> {
    v0
}
fn group_in_17(mut v0: Rc<str>, mut v1: regex::Regex, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_18(v0.clone(), v1.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let v8: Rc<str> = acc;
    let mut v10: std::collections::HashMap<Rc<str>, Rc<str>> = x;
    let mut v11: bool = &*v8 != "";
    let v18: Rc<str> = if v11 {
        v8.clone()
    } else {
        let v12: Rc<str> = method20(v2.clone());
        let mut v14: Option<Rc<str>> = std::collections::HashMap::get(&v10, &v12).map(|x| x).cloned();
        let mut v16: Rc<str> = v8.clone();
        let v17: Rc<str> = v14.unwrap_or(v16);
        v17.clone()
    };
    let v20: bool = true; v18 });
    let mut v22: Rc<str> = _vec_fold_;
    v22.clone()
}
fn set_values_15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: regex::Regex, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v7: Rc<str> = loop_16(v6.clone());
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("value"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = group_in_17(v7.clone(), v2.clone(), v8.clone());
            let mut v10: i32 = v1.wrapping_add(1i32);
            let mut v11: bool = &*v9 == "";
            let mut v13: Rc<UH0> = if v11 {
                v3.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v9.clone(), v3.clone()))
            };
            (v1, v3) = (v10, v13.clone());
            continue;
        }
    }
}
fn method21(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn set_text_of_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v4: bool = &*v1 == "";
    let mut v6: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^(?<value>.+)$"); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        v1.clone()
    };
    let v7: regex::Regex = compiled_pattern_12(v6.clone());
    let v8: i32 = 0i32;
    let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v10: Rc<UH0> = set_values_15(v3.clone(), v8, v7.clone(), v9.clone());
    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v12: i32 = 0i32;
    let mut v13: i32 = method21(v11.clone(), v10.clone(), v12);
    let mut v14: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
    let v15: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v14).as_ref().clone()));
    let mut v16: Rc<str> = Rc::<str>::from((*v15.borrow()).join(&*v2));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v16)), v2));
    v18.clone()
}
fn count_column_of_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> (Rc<str>, regex::Regex, US1, US0) {
    let v5: regex::Regex = compiled_pattern_12(v1);
    let mut v6: bool = &*v2 == "";
    let mut v10: US1 = if v6 {
        US1::US1_None
    } else {
        let v8: regex::Regex = compiled_pattern_12(v2);
        US1::US1_Some(v8)
    };
    let mut v11: bool = &*v3 == "";
    let v26: US0 = if v11 {
        US0::US0_None
    } else {
        let mut v14: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v3);
        let mut v16: Vec<u8> = v14.unwrap();
        let v17: Vec<u8> = method13(v16);
        let v19: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v17);
        let v21: std::string::String = v19.unwrap();
        let mut v23: Rc<str> = Rc::<str>::from(String::as_str(&v21));
        let v24: Rc<str> = set_text_of_14(v23, v4);
        US0::US0_Some(v24)
    };
    (v0, v5, v10, v26)
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<UH1> = v7.clone();
            let v8: Rc<UH2> = method10(v7, v1.clone());
            let (mut v9, mut v10, mut v11, mut v12): (Rc<str>, regex::Regex, US1, US0) = count_column_of_11(v2, v3, v4, v5, v6);
            Rc::new(UH2::UH2_Cons(v9, v10, v11, v12, v8))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method22(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match &*v0 {
        UH2::UH2_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: regex::Regex = v3.clone();
            let v4: US1 = v4.clone();
            let v5: US0 = v5.clone();
            let mut v6: Rc<UH2> = v6.clone();
            let v7: Rc<UH3> = method22(v6, v1.clone());
            Rc::new(UH3::UH3_Cons(0u64, v7))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method26(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_25(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method26(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method24(mut v0: Rc<UH3>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH3::UH3_Cons(v2, v3) => {
            let mut v2: u64 = *v2;
            let mut v3: Rc<UH3> = v3.clone();
            let v4: Rc<UH0> = method24(v3, v1.clone());
            let v5: Rc<str> = format_real_25(v2);
            Rc::new(UH0::UH0_Cons(v5, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn method29() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("value"); } LIT.with(|lit| lit.clone()) };
    v0
}
fn matches_in_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: regex::Regex, mut v3: US1, mut v4: US0) -> u64 {
    let v9: Rc<str> = match &v3 {
        US1::US1_None => {
            v0.clone()
        }
        US1::US1_Some(v5) => {
            let v5: regex::Regex = v5.clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rest"); } LIT.with(|lit| lit.clone()) };
            group_in_17(v0.clone(), v5.clone(), v6.clone())
        }
    };
    let mut v10: bool = &*v9 == "";
    if v10 {
        0u64
    } else {
        let mut v11: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_18(v9.clone(), v2.clone());
        let mut v13: bool = true; let _vec_fold_ = v11.into_iter().fold(0u64, |acc, x| { //;
        let v15: u64 = acc;
        let v17: std::collections::HashMap<Rc<str>, Rc<str>> = x;
        let v36: u64 = match &v4 {
            US0::US0_None => {
                let v18: u64 = v15.wrapping_add(1u64);
                v18
            }
            US0::US0_Some(v19) => {
                let v19: Rc<str> = v19.clone();
                let v20: Rc<str> = method29();
                let mut v22: Option<Rc<str>> = std::collections::HashMap::get(&v17, &v20).map(|x| x).cloned();
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = v24.clone();
                let v26: Rc<str> = v22.unwrap_or(v25);
                let v27: bool = &*v26 != "";
                let v32: bool = if v27 {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v26)), v28));
                    let v31: bool = v19.contains(&*v30);
                    v31
                } else {
                    false
                };
                if v32 {
                    let v33: u64 = v15.wrapping_add(1u64);
                    v33
                } else {
                    v15
                }
            }
        };
        let v38: bool = true; v36 });
        let v40: u64 = _vec_fold_;
        v40
    }
}
fn counts_added_27(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: Rc<UH3>) -> Rc<UH3> {
    match &*v1 {
        UH2::UH2_Cons(v3, v4, v5, v6, v7) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: regex::Regex = v4.clone();
            let v5: US1 = v5.clone();
            let mut v6: US0 = v6.clone();
            let v7: Rc<UH2> = v7.clone();
            match &*v2 {
                UH3::UH3_Cons(v8, v9) => {
                    let v8: u64 = *v8;
                    let v9: Rc<UH3> = v9.clone();
                    let mut v10: u64 = matches_in_28(v0.clone(), v3, v4, v5, v6);
                    let mut v11: u64 = v8.wrapping_add(v10);
                    let v12: Rc<UH3> = counts_added_27(v0, v7, v9);
                    Rc::new(UH3::UH3_Cons(v11, v12))
                }
                _ => {
                    { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
                }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn counted_from_23(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: US1, mut v3: Rc<UH2>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: Rc<UH3>, mut v8: Rc<UH0>, mut v9: u64) -> (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) {
    let mut v10: i32 = (v0.clone().borrow().len() as i32);
    let mut v11: bool = v1 >= v10;
    let (mut v83, mut v84, mut v85, mut v86, mut v87, mut v88): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = if v11 {
        let mut v13: bool = if v5 {
            true
        } else {
            let v12: bool = v6 > 0u64;
            v12
        };
        let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = if v13 {
            let mut v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v15: Rc<UH0> = method24(v7.clone(), v14);
            let mut v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v16.borrow_mut().push(v4.clone());
            let v17: i32 = 1i32;
            let v18: i32 = method21(v16.clone(), v15, v17);
            let v19: Rc<Vec<Rc<str>>> = Rc::new(v16.borrow().clone());
            let v20: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v19).as_ref().clone()));
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v22: Rc<str> = Rc::<str>::from((*v20.borrow()).join(&*v21));
            let mut v23: u64 = v9.wrapping_add(1u64);
            let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v8.clone()));
            (v4.clone(), v5, v6, v7.clone(), v24, v23)
        } else {
            (v4.clone(), v5, v6, v7.clone(), v8.clone(), v9)
        };
        (v25, v26, v27, v28, v29, v30)
    } else {
        let v31: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v32: Rc<str> = loop_16(v31);
        let v38: Rc<str> = match &v2 {
            US1::US1_None => {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36
            }
            US1::US1_Some(v33) => {
                let v33: regex::Regex = v33.clone();
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("section"); } LIT.with(|lit| lit.clone()) };
                group_in_17(v32.clone(), v33, v34)
            }
        };
        let v39: bool = &*v38 != "";
        let (mut v70, mut v71, mut v72, mut v73, mut v74, mut v75): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = if v39 {
            let v41: bool = if v5 {
                true
            } else {
                let v40: bool = v6 > 0u64;
                v40
            };
            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = if v41 {
                let v42: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v43: Rc<UH0> = method24(v7.clone(), v42);
                let mut v44: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                v44.borrow_mut().push(v4.clone());
                let v45: i32 = 1i32;
                let v46: i32 = method21(v44.clone(), v43, v45);
                let v47: Rc<Vec<Rc<str>>> = Rc::new(v44.borrow().clone());
                let v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v47).as_ref().clone()));
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v50: Rc<str> = Rc::<str>::from((*v48.borrow()).join(&*v49));
                let v51: u64 = v9.wrapping_add(1u64);
                let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v50, v8.clone()));
                (v4.clone(), v5, v6, v7.clone(), v52, v51)
            } else {
                (v4.clone(), v5, v6, v7.clone(), v8.clone(), v9)
            };
            let v59: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            let v60: Rc<UH3> = method22(v3.clone(), v59);
            (v38, true, 0u64, v60, v57, v58)
        } else {
            let v61: bool = &*v32 == "";
            let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = if v61 {
                (v4.clone(), v5, v6, v7.clone(), v8.clone(), v9)
            } else {
                let v62: Rc<UH3> = counts_added_27(v32, v3.clone(), v7);
                let v63: u64 = v6.wrapping_add(1u64);
                (v4, v5, v63, v62, v8, v9)
            };
            (v64, v65, v66, v67, v68, v69)
        };
        let v76: i32 = v1.wrapping_add(1i32);
        let (mut v77, mut v78, mut v79, mut v80, mut v81, mut v82): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = counted_from_23(v0, v76, v2, v3, v70, v71, v72, v73, v74, v75);
        (v77, v78, v79, v80, v81, v82)
    };
    (v83, v84, v85, v86, v87, v88)
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn method31(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v2));
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
                (v0, v1) = (v3.clone(), v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn counts_text_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH1>) -> (Rc<str>, u64) {
    let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH2> = method10(v3.clone(), v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v6).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH3> = method22(v5.clone(), v8.clone());
    let mut v10: i32 = 0i32;
    let mut v11: bool = &*v2 == "";
    let v15: US1 = if v11 {
        US1::US1_None
    } else {
        let mut v13: regex::Regex = compiled_pattern_12(v2.clone());
        US1::US1_Some(v13.clone())
    };
    let mut v16: bool = false;
    let v17: u64 = 0u64;
    let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v19: u64 = 0u64;
    let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, Rc<UH3>, Rc<UH0>, u64) = counted_from_23(v7.clone(), v10, v15.clone(), v5.clone(), v1.clone(), v16, v17, v9.clone(), v18.clone(), v19);
    let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH0> = method30(v24.clone(), v26.clone());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = method31(v27.clone(), v28.clone());
    (v29.clone(), v25)
}
fn counts_of_file_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH1>) -> (Rc<str>, u64) {
    let v4: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let mut v6: Vec<u8> = v4.unwrap();
    let v7: Vec<u8> = method13(v6.clone());
    let v9: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v7);
    let mut v11: std::string::String = v9.unwrap();
    let mut v13: Rc<str> = Rc::<str>::from(String::as_str(&v11));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(v0.replace(&*v14, &*v15));
    let v17: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v16.split(&*v15).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v18: i32 = (v17.clone().borrow().len() as i32);
    let v19: i32 = v18.wrapping_sub(1i32);
    let v20: Rc<RefCell<Vec<Rc<str>>>> = v17 ;
    let v21: bool = v19 >= 0i32;
    let v24: bool = if v21 {
        let mut v22: i32 = (v20.clone().borrow().len() as i32);
        let mut v23: bool = v19 < v22;
        v23
    } else {
        false
    };
    let v28: US0 = if v24 {
        let v25: Rc<str> = v20.borrow()[v19 as usize].clone();
        US0::US0_Some(v25.clone())
    } else {
        US0::US0_None
    };
    let v32: Rc<str> = match &v28 {
        US0::US0_None => {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        }
        US0::US0_Some(v29) => {
            let v29: Rc<str> = v29.clone();
            v29.clone()
        }
    };
    counts_text_9(v13.clone(), v32.clone(), v1.clone(), v2.clone())
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (Rc<str>, u64) {
        let mut v2: Rc<UH1> = command_specs_0();
        counts_of_file_8(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method33(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = v6.clone();
            let v7: Rc<UH1> = v7.clone();
            let v8: Rc<UH0> = method33(v7, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v8))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn counts_header_32(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH0> = method33(v0, v1);
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("section"); } LIT.with(|lit| lit.clone()) };
    v3.borrow_mut().push(v4);
    let v5: i32 = 1i32;
    let mut v6: i32 = method21(v3.clone(), v2, v5);
    let v7: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    v10
}
fn closure2() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<UH1> = command_specs_0();
        counts_header_32(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_35(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn missing_set_34(mut v0: Rc<UH1>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v1, v2, v3, v4, v5, v6) => {
                let mut v1: Rc<str> = v1.clone();
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH1> = v6.clone();
                let v7: bool = &*v4 != "";
                let mut v10: bool = if v7 {
                    let v8: bool = file_exists_35(v4.clone());
                    let v9: bool = v8 == false;
                    v9
                } else {
                    false
                };
                if v10 {
                    return v4.clone();
                } else {
                    v0 = v6.clone();
                    continue;
                }
            }
            UH1::UH1_Nil => {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v13.clone();
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<UH1> = command_specs_0();
        missing_set_34(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_sequence_counts(v0: &str, v1: &str) -> (Rc<str>, u64) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_sequence_counts_header() -> Rc<str> {
    closure2()()
}
pub fn kit_sequence_counts_missing_set() -> Rc<str> {
    closure3()()
}
