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
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_IntervalOf(Rc<str>, Rc<str>, u64),
    US2_WindowOf(Rc<str>, Rc<str>, u64, u64),
    US2_OrderOf(Rc<str>, Rc<str>),
    US2_LatencyOf(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
struct Mut1 { l0: i32 }
struct Mut2 { l0: i32 }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table sequences "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table sequences <file|dir>... --time <regex with (?<ms>digits)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex with (?<section>)>] [--carry <regex>] (--first|--last) name=<regex with (?<id>) and (?<value>)>... | [--section-on <regex>] (--count name=<regex [(?<value>)]>... [--count-in name=<regex with (?<rest>)>] [--in-set name=<file>[:<regex with (?<value>)>]])) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    eprintln!("{}", v6);
    2i32
}
fn method3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn table_written_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file\tline\t"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v0));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v3));
    let v10: i32 = 0i32;
    let v11: i32 = method3(v2, v10);
    let v12: Rc<str> = format_real_4(v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table sequences ok files="); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let v17: Rc<str> = format_real_6(v4);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: bool = &*v1 == "";
    if v19 {
        let v20: Rc<str> = loop_7(v9.clone());
        println!("{}", v20);
        eprintln!("{}", v18);
        ()
    } else {
        std::fs::write(&*v1, &*v9).unwrap();
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v22)), v1));
        eprintln!("{}", v24);
        ()
    };
    0i32
}
fn command_line_argument_count_10() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_11(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_9(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_10();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_11(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_9(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn sequence_words_8() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_9(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let mut v5: Rc<UH0> = v5.clone();
                    v5
                }
                _ => {
                    { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
                }
            }
        }
        _ => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn known_flag_13(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = &*v0 == &*v3;
    if v4 {
        true
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("interval"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--interval"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = &*v0 == &*v6;
        if v7 {
            true
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--window"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = &*v0 == &*v9;
            if v10 {
                true
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("order"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--order"); } LIT.with(|lit| lit.clone()) };
                let v13: bool = &*v0 == &*v12;
                if v13 {
                    true
                } else {
                    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("latency"); } LIT.with(|lit| lit.clone()) };
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--latency"); } LIT.with(|lit| lit.clone()) };
                    let v16: bool = &*v0 == &*v15;
                    if v16 {
                        true
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("between"); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--between"); } LIT.with(|lit| lit.clone()) };
                        let v19: bool = &*v0 == &*v18;
                        if v19 {
                            true
                        } else {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from"); } LIT.with(|lit| lit.clone()) };
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--from"); } LIT.with(|lit| lit.clone()) };
                            let v22: bool = &*v0 == &*v21;
                            if v22 {
                                true
                            } else {
                                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
                                let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--to"); } LIT.with(|lit| lit.clone()) };
                                let mut v25: bool = &*v0 == &*v24;
                                if v25 {
                                    true
                                } else {
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ms"); } LIT.with(|lit| lit.clone()) };
                                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ms"); } LIT.with(|lit| lit.clone()) };
                                    let v28: bool = &*v0 == &*v27;
                                    if v28 {
                                        true
                                    } else {
                                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modulo"); } LIT.with(|lit| lit.clone()) };
                                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--modulo"); } LIT.with(|lit| lit.clone()) };
                                        let v31: bool = &*v0 == &*v30;
                                        if v31 {
                                            true
                                        } else {
                                            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("out"); } LIT.with(|lit| lit.clone()) };
                                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                                            let v34: bool = &*v0 == &*v33;
                                            if v34 {
                                                true
                                            } else {
                                                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("section-on"); } LIT.with(|lit| lit.clone()) };
                                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--section-on"); } LIT.with(|lit| lit.clone()) };
                                                let v37: bool = &*v0 == &*v36;
                                                if v37 {
                                                    true
                                                } else {
                                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("carry"); } LIT.with(|lit| lit.clone()) };
                                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--carry"); } LIT.with(|lit| lit.clone()) };
                                                    let v40: bool = &*v0 == &*v39;
                                                    if v40 {
                                                        true
                                                    } else {
                                                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
                                                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--first"); } LIT.with(|lit| lit.clone()) };
                                                        let v43: bool = &*v0 == &*v42;
                                                        if v43 {
                                                            true
                                                        } else {
                                                            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last"); } LIT.with(|lit| lit.clone()) };
                                                            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
                                                            let v46: bool = &*v0 == &*v45;
                                                            if v46 {
                                                                true
                                                            } else {
                                                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                                                                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count"); } LIT.with(|lit| lit.clone()) };
                                                                let v49: bool = &*v0 == &*v48;
                                                                if v49 {
                                                                    true
                                                                } else {
                                                                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count-in"); } LIT.with(|lit| lit.clone()) };
                                                                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count-in"); } LIT.with(|lit| lit.clone()) };
                                                                    let v52: bool = &*v0 == &*v51;
                                                                    if v52 {
                                                                        true
                                                                    } else {
                                                                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in-set"); } LIT.with(|lit| lit.clone()) };
                                                                        let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--in-set"); } LIT.with(|lit| lit.clone()) };
                                                                        let v55: bool = &*v0 == &*v54;
                                                                        v55
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn paths_from_12(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                let v8: bool = v5.starts_with(&*v7);
                let v9: bool = v8 == false;
                if v9 {
                    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v1.clone()));
                    (v0, v1) = (v6.clone(), v10.clone());
                    continue;
                } else {
                    let v13: bool = known_flag_13(v5.clone());
                    let mut v14: bool = v13 == false;
                    if v14 {
                        let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH0> = method14(v1.clone(), v15.clone());
                        let v17: bool = &*v2 == "";
                        let v20: Rc<str> = if v17 {
                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v5));
                            v19.clone()
                        } else {
                            v2.clone()
                        };
                        return (v16.clone(), v20.clone());
                    } else {
                        match &*v6 {
                            UH0::UH0_Cons(v21, v22) => {
                                let v21: Rc<str> = v21.clone();
                                let v22: Rc<UH0> = v22.clone();
                                v0 = v22.clone();
                                continue;
                            }
                            UH0::UH0_Nil => {
                                let mut v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v26: Rc<UH0> = method14(v1.clone(), v25.clone());
                                return (v26.clone(), v2.clone());
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let mut v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH0> = method14(v1.clone(), v3.clone());
                return (v4.clone(), v2.clone());
            }
        }
    }
}
fn value_after_16(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v0;
                        if v6 {
                            return v4.clone();
                        } else {
                            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                            v1 = v7.clone();
                            continue;
                        }
                    }
                    _ => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v10.clone();
                    }
                }
            }
            _ => {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v12.clone();
            }
        }
    }
}
fn method18(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn split_pair_17(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method18(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v0.clone(), v9.clone())
    } else {
        let v10: usize = ((0i32) as usize);
        let v11: usize = ((v7) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v0)[v10..v11]);
        let v13: i32 = (v1.clone().len() as i32);
        let mut v14: i32 = v7.wrapping_add(v13);
        let mut v15: i32 = (v0.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(v7);
        let v17: i32 = v16.wrapping_sub(v13);
        let v18: usize = ((v14) as usize);
        let v19: i32 = v14.wrapping_add(v17);
        let v20: usize = ((v19) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v0)[v18..v20]);
        (v12.clone(), v21.clone())
    }
}
fn method20(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_19(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method20(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure2();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn measure_from_15(mut v0: Rc<UH0>) -> (Rc<str>, US1) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("interval"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--interval"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = value_after_16(v3, v0.clone());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--window"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = value_after_16(v6, v0.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("order"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--order"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = value_after_16(v9, v0.clone());
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("latency"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--latency"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = value_after_16(v12, v0.clone());
    let mut v14: bool = &*v4 != "";
    if v14 {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let (mut v16, mut v17): (Rc<str>, Rc<str>) = split_pair_17(v4, v15);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("between"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--between"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = value_after_16(v19, v0.clone());
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modulo"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--modulo"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = value_after_16(v22, v0.clone());
        let mut v24: u64 = decimal_19(v23);
        let mut v25: US2 = US2::US2_IntervalOf(v17, v20, v24);
        let v26: US1 = US1::US1_Some(v25);
        (v16, v26)
    } else {
        let mut v27: bool = &*v7 != "";
        if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let (mut v29, mut v30): (Rc<str>, Rc<str>) = split_pair_17(v7, v28);
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from"); } LIT.with(|lit| lit.clone()) };
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--from"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = value_after_16(v32, v0.clone());
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ms"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ms"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = value_after_16(v35, v0.clone());
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let (mut v38, mut v39): (Rc<str>, Rc<str>) = split_pair_17(v36, v37);
            let v40: bool = &*v33 == "";
            let v46: US1 = if v40 {
                US1::US1_None
            } else {
                let v42: u64 = decimal_19(v38);
                let v43: u64 = decimal_19(v39);
                let v44: US2 = US2::US2_WindowOf(v30, v33, v42, v43);
                US1::US1_Some(v44)
            };
            (v29, v46)
        } else {
            let v47: bool = &*v10 != "";
            if v47 {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let (mut v49, mut v50): (Rc<str>, Rc<str>) = split_pair_17(v10, v48);
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let (mut v52, mut v53): (Rc<str>, Rc<str>) = split_pair_17(v50, v51);
                let v54: bool = &*v53 == "";
                let v58: US1 = if v54 {
                    US1::US1_None
                } else {
                    let v56: US2 = US2::US2_OrderOf(v52, v53);
                    US1::US1_Some(v56)
                };
                (v49, v58)
            } else {
                let v59: bool = &*v13 != "";
                if v59 {
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let (mut v61, mut v62): (Rc<str>, Rc<str>) = split_pair_17(v13, v60);
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--to"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<str> = value_after_16(v64, v0);
                    let v66: bool = &*v65 == "";
                    let v70: US1 = if v66 {
                        US1::US1_None
                    } else {
                        let v68: US2 = US2::US2_LatencyOf(v62, v65);
                        US1::US1_Some(v68)
                    };
                    (v61, v70)
                } else {
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v72: US1 = US1::US1_None;
                    (v71, v72)
                }
            }
        }
    }
}
fn file_exists_22(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn directory_exists_23(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn missing_path_21(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = file_exists_22(v2.clone());
                let mut v6: bool = if v4 {
                    true
                } else {
                    directory_exists_23(v2.clone())
                };
                if v6 {
                    v0 = v3.clone();
                    continue;
                } else {
                    return v2.clone();
                }
            }
            UH0::UH0_Nil => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1.clone();
            }
        }
    }
}
fn method25(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method26(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method27() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method28(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut2>>) -> bool {
    let mut v3: i32 = v2.borrow().l0;
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0;
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let v7: Rc<str> = v0.borrow()[v6 as usize].clone();
        let v8: bool = &*v7 < &*v1;
        let v11: i32 = if v8 {
            -1i32
        } else {
            let v9: bool = &*v7 > &*v1;
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn named_files_29(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
            let v7: Rc<str> = v1.borrow()[v2 as usize].clone();
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let v9: i32 = v2.wrapping_sub(1i32);
            let v10: bool = file_exists_22(v8.clone());
            let v12: Rc<UH0> = if v10 {
                Rc::new(UH0::UH0_Cons(v8.clone(), v3.clone()))
            } else {
                v3.clone()
            };
            (v2, v3) = (v9, v12.clone());
            continue;
        }
    }
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn files_in_24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = directory_exists_23(v4.clone());
                if v6 {
                    let v8: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                    let v10: Vec<std::string::String> = v8.unwrap();
                    let v11: Vec<std::string::String> = method25(v10.clone());
                    let v12: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v11));
                    let v13: i32 = (v12.clone().borrow().len() as i32);
                    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v13 as usize]));
                    let mut v15: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method26(v13, v15.clone()) {
                        let v17: i32 = v15.borrow().l0;
                        let v18: std::string::String = v12.borrow()[v17 as usize].clone();
                        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        v14.borrow_mut()[v17 as usize] = v20.clone();
                        let v21: i32 = v17.wrapping_add(1i32);
                        v15.borrow_mut().l0 = v21;
                        ()
                    };
                    let v22: Rc<RefCell<Vec<Rc<str>>>> = v14 ;
                    let mut v23: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method27();
                    let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().clone()));
                    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().iter().map(|x| v23(x.clone())).collect::<Vec<_>>()));
                    let v26: i32 = (v22.clone().borrow().len() as i32);
                    let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
                    while method26(v26, v27.clone()) {
                        let v29: i32 = v27.borrow().l0;
                        let v30: Rc<str> = v24.borrow()[v29 as usize].clone();
                        let v31: Rc<str> = v25.borrow()[v29 as usize].clone();
                        let mut v32: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v29 }));
                        while method28(v25.clone(), v31.clone(), v32.clone()) {
                            let v34: i32 = v32.borrow().l0;
                            let v35: i32 = v34.wrapping_sub(1i32);
                            let v36: Rc<str> = v24.borrow()[v35 as usize].clone();
                            v24.borrow_mut()[v34 as usize] = v36.clone();
                            let v37: Rc<str> = v25.borrow()[v35 as usize].clone();
                            v25.borrow_mut()[v34 as usize] = v37.clone();
                            v32.borrow_mut().l0 = v35;
                            ()
                        };
                        let v38: i32 = v32.borrow().l0;
                        v24.borrow_mut()[v38 as usize] = v30.clone();
                        let v39: i32 = v32.borrow().l0;
                        v25.borrow_mut()[v39 as usize] = v31.clone();
                        let v40: i32 = v29.wrapping_add(1i32);
                        v27.borrow_mut().l0 = v40;
                        ()
                    };
                    let v41: Rc<RefCell<Vec<Rc<str>>>> = v24 ;
                    let v42: i32 = (v41.clone().borrow().len() as i32);
                    let v43: i32 = v42.wrapping_sub(1i32);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = named_files_29(v4.clone(), v41.clone(), v43, v44.clone());
                    let v46: Rc<UH0> = method30(v45.clone(), v1.clone());
                    (v0, v1) = (v5.clone(), v46.clone());
                    continue;
                } else {
                    let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v1.clone()));
                    (v0, v1) = (v5.clone(), v48.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method14(v1.clone(), v2.clone());
            }
        }
    }
}
fn method33(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn data_rows_34(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    let mut v4: i32 = (v0.clone().borrow().len() as i32);
    let mut v5: bool = v1 >= v4;
    let (mut v23, mut v24): (Rc<str>, u64) = if v5 {
        (v2.clone(), v3)
    } else {
        let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v7: bool = &*v6 == "";
        let v10: bool = if v7 {
            true
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            let v9: bool = v6.starts_with(&*v8);
            v9
        };
        let (mut v21, mut v22): (Rc<str>, u64) = if v10 {
            let v11: i32 = v1.wrapping_add(1i32);
            let (mut v12, mut v13): (Rc<str>, u64) = data_rows_34(v0.clone(), v11, v2.clone(), v3);
            (v12, v13)
        } else {
            let mut v14: i32 = v1.wrapping_add(1i32);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let v18: u64 = v3.wrapping_add(1u64);
            let (mut v19, mut v20): (Rc<str>, u64) = data_rows_34(v0, v14, v17, v18);
            (v19, v20)
        };
        (v21, v22)
    };
    (v23, v24)
}
fn file_table_of_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2) -> (Rc<str>, u64) {
    let (mut v21, mut v22): (Rc<str>, Rc<str>) = match &v3 {
        US2::US2_IntervalOf(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: u64 = *v6;
            (v4.clone(), v5.clone())
        }
        US2::US2_LatencyOf(v13, v14) => {
            let v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            (v13.clone(), v14.clone())
        }
        US2::US2_OrderOf(v11, v12) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            (v11.clone(), v12.clone())
        }
        US2::US2_WindowOf(v7, v8, v9, v10) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = v8.clone();
            let v9: u64 = *v9;
            let v10: u64 = *v10;
            (v7.clone(), v8.clone())
        }
    };
    let mut v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let v26: Vec<u8> = v24.unwrap();
    let mut v27: Vec<u8> = method33(v26.clone());
    let v29: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v27);
    let v31: std::string::String = v29.unwrap();
    let v33: Rc<str> = Rc::<str>::from(String::as_str(&v31));
    let v34: Rc<str> = kit_log_sequences::kit_sequence_events(&*v33, &*v2, &*v21, &*v22);
    let v57: Rc<str> = match &v3 {
        US2::US2_IntervalOf(v35, v36, v37) => {
            let v35: Rc<str> = v35.clone();
            let v36: Rc<str> = v36.clone();
            let v37: u64 = *v37;
            let v38: bool = &*v36 == "";
            let v41: Rc<str> = if v38 {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39.clone()
            } else {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("second"); } LIT.with(|lit| lit.clone()) };
                v40.clone()
            };
            let v42: Rc<str> = kit_log_sequences::kit_sequence_interval(&*v0, &*v34, "first", &*v41, v37);
            v42.clone()
        }
        US2::US2_LatencyOf(v51, v52) => {
            let v51: Rc<str> = v51.clone();
            let v52: Rc<str> = v52.clone();
            let v53: Rc<str> = kit_log_sequences::kit_sequence_latency(&*v0, &*v34, "first", "second");
            v53.clone()
        }
        US2::US2_OrderOf(v48, v49) => {
            let v48: Rc<str> = v48.clone();
            let v49: Rc<str> = v49.clone();
            let v50: Rc<str> = kit_log_sequences::kit_sequence_order(&*v0, &*v34, "first", "second");
            v50.clone()
        }
        US2::US2_WindowOf(v43, v44, v45, v46) => {
            let v43: Rc<str> = v43.clone();
            let v44: Rc<str> = v44.clone();
            let v45: u64 = *v45;
            let v46: u64 = *v46;
            let v47: Rc<str> = kit_log_sequences::kit_sequence_window(&*v0, &*v34, "first", "second", v45, v46);
            v47.clone()
        }
    };
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v57.split(&*v58).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v60: i32 = 0i32;
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v62: u64 = 0u64;
    let (mut v63, mut v64): (Rc<str>, u64) = data_rows_34(v59.clone(), v60, v61.clone(), v62);
    (v63.clone(), v64)
}
fn tables_of_31(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: Rc<str>, mut v5: u64) -> (Rc<str>, u64) {
    let (mut v16, mut v17): (Rc<str>, u64) = match &*v0 {
        UH0::UH0_Nil => {
            (v4.clone(), v5)
        }
        _ => {
            let (mut v14, mut v15): (Rc<str>, u64) = match &*v0 {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6.clone();
                    let v7: Rc<UH0> = v7.clone();
                    let (mut v8, mut v9): (Rc<str>, u64) = file_table_of_32(v6, v1.clone(), v2.clone(), v3.clone());
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v8));
                    let v11: u64 = v5.wrapping_add(v9);
                    tables_of_31(v7, v1, v2, v3, v10, v11)
                }
                _ => unreachable!(),
            };
            (v14, v15)
        }
    };
    (v16, v17)
}
fn measures_run_0() -> i32 {
    let mut v0: Rc<UH0> = sequence_words_8();
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v3, mut v4): (Rc<UH0>, Rc<str>) = paths_from_12(v0.clone(), v1, v2.clone());
    let (mut v5, mut v6): (Rc<str>, US1) = measure_from_15(v0.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("time"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = value_after_16(v9, v0.clone());
    let v11: bool = &*v4 != "";
    if v11 {
        refused_1(v4)
    } else {
        match &v6 {
            US1::US1_None => {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs one measure with both its patterns"); } LIT.with(|lit| lit.clone()) };
                refused_1(v13)
            }
            US1::US1_Some(v15) => {
                let mut v15: US2 = v15.clone();
                let v16: bool = &*v10 == "";
                let v18: bool = if v16 {
                    true
                } else {
                    let v17: bool = &*v5 == "";
                    v17
                };
                if v18 {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --time and a measure name"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v19)
                } else {
                    match &*v3 {
                        UH0::UH0_Nil => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v21)
                        }
                        _ => {
                            let mut v23: Rc<str> = missing_path_21(v3.clone());
                            let mut v24: bool = &*v23 != "";
                            if v24 {
                                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
                                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v23));
                                refused_1(v26)
                            } else {
                                let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v29: Rc<UH0> = files_in_24(v3, v28);
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("out"); } LIT.with(|lit| lit.clone()) };
                                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                                let mut v32: Rc<str> = value_after_16(v31, v0);
                                let v33: u64 = 0u64;
                                let (mut v34, mut v35): (Rc<str>, u64) = tables_of_31(v29.clone(), v5.clone(), v10, v15, v2, v33);
                                table_written_2(v5, v32, v29, v34, v35)
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        measures_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_sequence_measures_run() -> i32 {
    closure0()()
}
