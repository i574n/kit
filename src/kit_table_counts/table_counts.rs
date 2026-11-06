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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
pub struct Mut1 { pub l0: i32 }
pub struct Mut2 { pub l0: i32 }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table sequences "); } LIT.with(|lit| lit.clone()) }, v0));
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table sequences <file|dir>... --time <regex with (?<ms>digits)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex with (?<section>)>] [--carry <regex>] (--first|--last) name=<regex with (?<id>) and (?<value>)>... | [--section-on <regex>] (--count name=<regex [(?<value>)]>... [--count-in name=<regex with (?<rest>)>] [--in-set name=<file>[:<regex with (?<value>)>]])) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) }));
    eprintln!("{}", v3);
    2i32
}
fn method3(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method5(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let mut v5: i32 = v4.wrapping_sub(1i32);
            let mut v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn counts_written_2(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = kit_log_counts::kit_sequence_counts_header();
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v2));
    let v8: i32 = 0i32;
    let v9: i32 = method3(v1, v8);
    let v10: Rc<str> = format_real_4(v9);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table sequences ok files="); } LIT.with(|lit| lit.clone()) }, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sections="); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = format_real_6(v4);
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = format_real_6(v3);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: bool = &*v0 == "";
    if v18 {
        let v19: Rc<str> = loop_7(v7);
        println!("{}", v19);
        eprintln!("{}", v17);
        ()
    } else {
        std::fs::write(&*v0, &*v7).unwrap();
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) }));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v0));
        eprintln!("{}", v22);
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
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_9(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_10();
        let v3: bool = v0 < v2;
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
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_9(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn sequence_words_8() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_9(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4;
                    let mut v5: Rc<UH0> = v5;
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
    let mut v1: bool = &*v0 == "--time";
    if v1 {
        true
    } else {
        let v2: bool = &*v0 == "--interval";
        if v2 {
            true
        } else {
            let v3: bool = &*v0 == "--window";
            if v3 {
                true
            } else {
                let mut v4: bool = &*v0 == "--order";
                if v4 {
                    true
                } else {
                    let mut v5: bool = &*v0 == "--latency";
                    if v5 {
                        true
                    } else {
                        let mut v6: bool = &*v0 == "--between";
                        if v6 {
                            true
                        } else {
                            let v7: bool = &*v0 == "--from";
                            if v7 {
                                true
                            } else {
                                let v8: bool = &*v0 == "--to";
                                if v8 {
                                    true
                                } else {
                                    let v9: bool = &*v0 == "--ms";
                                    if v9 {
                                        true
                                    } else {
                                        let v10: bool = &*v0 == "--modulo";
                                        if v10 {
                                            true
                                        } else {
                                            let v11: bool = &*v0 == "--out";
                                            if v11 {
                                                true
                                            } else {
                                                let v12: bool = &*v0 == "--section-on";
                                                if v12 {
                                                    true
                                                } else {
                                                    let v13: bool = &*v0 == "--carry";
                                                    if v13 {
                                                        true
                                                    } else {
                                                        let mut v14: bool = &*v0 == "--first";
                                                        if v14 {
                                                            true
                                                        } else {
                                                            let mut v15: bool = &*v0 == "--last";
                                                            if v15 {
                                                                true
                                                            } else {
                                                                let v16: bool = &*v0 == "--count";
                                                                if v16 {
                                                                    true
                                                                } else {
                                                                    let v17: bool = &*v0 == "--count-in";
                                                                    if v17 {
                                                                        true
                                                                    } else {
                                                                        let v18: bool = &*v0 == "--in-set";
                                                                        v18
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
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn paths_from_12(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                let v8: bool = v5.starts_with(&*v7);
                let v9: bool = v8 == false;
                if v9 {
                    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v1));
                    (v0, v1) = (v6, v10);
                    continue;
                } else {
                    let v13: bool = known_flag_13(v5.clone());
                    let mut v14: bool = v13 == false;
                    if v14 {
                        let mut v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH0> = method14(v1, v15);
                        let v17: bool = &*v2 == "";
                        let v19: Rc<str> = if v17 {
                            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v5));
                            v18
                        } else {
                            v2
                        };
                        return (v16, v19);
                    } else {
                        match Rc::unwrap_or_clone(v6) {
                            UH0::UH0_Cons(v20, v21) => {
                                let v20: Rc<str> = v20;
                                let v21: Rc<UH0> = v21;
                                v0 = v21;
                                continue;
                            }
                            UH0::UH0_Nil => {
                                let mut v24: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let mut v25: Rc<UH0> = method14(v1, v24);
                                return (v25, v2);
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v4: Rc<UH0> = method14(v1, v3);
                return (v4, v2);
            }
        }
    }
}
fn method17(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn split_pair_16(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method17(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v0, v9)
    } else {
        let v10: usize = ((0i32) as usize);
        let v11: usize = ((v7) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v0)[v10..v11]);
        let v13: i32 = (v1.len() as i32);
        let mut v14: i32 = v7.wrapping_add(v13);
        let mut v15: i32 = (v0.clone().len() as i32);
        let v16: i32 = v15.wrapping_sub(v7);
        let v17: i32 = v16.wrapping_sub(v13);
        let v18: usize = ((v14) as usize);
        let v19: i32 = v14.wrapping_add(v17);
        let v20: usize = ((v19) as usize);
        let v21: Rc<str> = Rc::<str>::from(&(v0)[v18..v20]);
        (v12, v21)
    }
}
fn counted_names_15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4;
                        let mut v5: Rc<UH0> = v5;
                        let mut v6: bool = &*v2 == "--count";
                        if v6 {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                            let (mut v8, mut v9): (Rc<str>, Rc<str>) = split_pair_16(v4, v7);
                            let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v1));
                            (v0, v1) = (v5, v10);
                            continue;
                        } else {
                            let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                            v0 = v12;
                            continue;
                        }
                    }
                    _ => {
                        return v1;
                    }
                }
            }
            _ => {
                return v1;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let v2: bool = &*v1 == &*v0;
        v2
    })
}
fn method19(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method20(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                let mut v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method19(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn stray_name_18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4;
                        let mut v5: Rc<UH0> = v5;
                        let mut v6: bool = &*v2 == "--count-in";
                        let v8: bool = if v6 {
                            true
                        } else {
                            let v7: bool = &*v2 == "--in-set";
                            v7
                        };
                        if v8 {
                            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                            let (mut v10, mut v11): (Rc<str>, Rc<str>) = split_pair_16(v4, v9);
                            let v12: Rc<dyn Fn(Rc<str>) -> bool> = method19(v10.clone());
                            let v13: bool = method20(v10.clone(), v12, v0.clone());
                            if v13 {
                                v1 = v5;
                                continue;
                            } else {
                                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v10));
                                return v16;
                            }
                        } else {
                            let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                            v1 = v18;
                            continue;
                        }
                    }
                    _ => {
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v21;
                    }
                }
            }
            _ => {
                let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v23;
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
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = file_exists_22(v2.clone());
                let mut v6: bool = if v4 {
                    true
                } else {
                    directory_exists_23(v2.clone())
                };
                if v6 {
                    v0 = v3;
                    continue;
                } else {
                    return v2;
                }
            }
            UH0::UH0_Nil => {
                let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1;
            }
        }
    }
}
fn method25(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method26(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method27() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method28(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: &Rc<RefCell<Mut2>>) -> bool {
    let v3: i32 = v2.borrow().l0;
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
fn named_files_29(mut v0: Rc<str>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
            let mut v6: Rc<str> = v1.borrow()[v2 as usize].clone();
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
            let v8: i32 = v2.wrapping_sub(1i32);
            let v9: bool = file_exists_22(v7.clone());
            let v11: Rc<UH0> = if v9 {
                Rc::new(UH0::UH0_Cons(v7, v3))
            } else {
                v3
            };
            (v2, v3) = (v8, v11);
            continue;
        }
    }
}
fn method30(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn files_in_24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let mut v6: bool = directory_exists_23(v4.clone());
                if v6 {
                    let v8: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                    let v10: Vec<std::string::String> = v8.unwrap();
                    let v11: Vec<std::string::String> = method25(v10);
                    let v12: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v11));
                    let v13: i32 = (v12.clone().borrow().len() as i32);
                    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v13 as usize]));
                    let mut v15: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method26(v13, &v15) {
                        let v17: i32 = v15.borrow().l0;
                        let v18: std::string::String = v12.borrow()[v17 as usize].clone();
                        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        v14.borrow_mut()[v17 as usize] = v20;
                        let v21: i32 = v17.wrapping_add(1i32);
                        v15.borrow_mut().l0 = v21;
                        ()
                    };
                    let v22: Rc<RefCell<Vec<Rc<str>>>> = v14 ;
                    let mut v23: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method27();
                    let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().clone()));
                    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().iter().map(|x| v23(x.clone())).collect::<Vec<_>>()));
                    let v26: i32 = (v22.borrow().len() as i32);
                    let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
                    while method26(v26, &v27) {
                        let v29: i32 = v27.borrow().l0;
                        let v30: Rc<str> = v24.borrow()[v29 as usize].clone();
                        let v31: Rc<str> = v25.borrow()[v29 as usize].clone();
                        let mut v32: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v29 }));
                        while method28(&v25, v31.clone(), &v32) {
                            let v34: i32 = v32.borrow().l0;
                            let v35: i32 = v34.wrapping_sub(1i32);
                            let v36: Rc<str> = v24.borrow()[v35 as usize].clone();
                            v24.borrow_mut()[v34 as usize] = v36;
                            let v37: Rc<str> = v25.borrow()[v35 as usize].clone();
                            v25.borrow_mut()[v34 as usize] = v37;
                            v32.borrow_mut().l0 = v35;
                            ()
                        };
                        let v38: i32 = v32.borrow().l0;
                        v24.borrow_mut()[v38 as usize] = v30;
                        let v39: i32 = v32.borrow().l0;
                        v25.borrow_mut()[v39 as usize] = v31;
                        let v40: i32 = v29.wrapping_add(1i32);
                        v27.borrow_mut().l0 = v40;
                        ()
                    };
                    let v41: Rc<RefCell<Vec<Rc<str>>>> = v24 ;
                    let v42: i32 = (v41.clone().borrow().len() as i32);
                    let v43: i32 = v42.wrapping_sub(1i32);
                    let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let v45: Rc<UH0> = named_files_29(v4, &v41, v43, v44);
                    let v46: Rc<UH0> = method30(v45, v1);
                    (v0, v1) = (v5, v46);
                    continue;
                } else {
                    let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v1));
                    (v0, v1) = (v5, v48);
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method14(v1, v2);
            }
        }
    }
}
fn value_after_31(mut v0: Rc<str>, mut v1: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match Rc::unwrap_or_clone(v3) {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4;
                        let mut v5: Rc<UH0> = v5;
                        let mut v6: bool = &*v2 == &*v0;
                        if v6 {
                            return v4;
                        } else {
                            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                            v1 = v7;
                            continue;
                        }
                    }
                    _ => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        return v10;
                    }
                }
            }
            _ => {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v12;
            }
        }
    }
}
fn data_rows_34(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    let mut v4: i32 = (v0.clone().borrow().len() as i32);
    let mut v5: bool = v1 >= v4;
    let (mut v22, mut v23): (Rc<str>, u64) = if v5 {
        (v2, v3)
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
        let (mut v20, mut v21): (Rc<str>, u64) = if v10 {
            let v11: i32 = v1.wrapping_add(1i32);
            let (mut v12, mut v13): (Rc<str>, u64) = data_rows_34(v0, v11, v2, v3);
            (v12, v13)
        } else {
            let mut v14: i32 = v1.wrapping_add(1i32);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v17: u64 = v3.wrapping_add(1u64);
            let (mut v18, mut v19): (Rc<str>, u64) = data_rows_34(v0, v14, v16, v17);
            (v18, v19)
        };
        (v20, v21)
    };
    (v22, v23)
}
fn counts_of_file_33(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, u64, u64) {
    let (mut v2, mut v3): (Rc<str>, u64) = kit_log_counts::kit_sequence_counts(&*v0, &*v1);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: u64 = 0u64;
    let (mut v9, mut v10): (Rc<str>, u64) = data_rows_34(&v5, v6, v7, v8);
    (v9, v10, v3)
}
fn counts_of_files_32(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> (Rc<str>, u64, u64) {
    let (mut v19, mut v20, mut v21): (Rc<str>, u64, u64) = match &*v0 {
        UH0::UH0_Nil => {
            (v2, v3, v4)
        }
        _ => {
            let (mut v16, mut v17, mut v18): (Rc<str>, u64, u64) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5;
                    let mut v6: Rc<UH0> = v6;
                    let (mut v7, mut v8, mut v9): (Rc<str>, u64, u64) = counts_of_file_33(v5, v1.clone());
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
                    let v11: u64 = v3.wrapping_add(v8);
                    let v12: u64 = v4.wrapping_add(v9);
                    counts_of_files_32(v6, v1, v10, v11, v12)
                }
                _ => unreachable!(),
            };
            (v16, v17, v18)
        }
    };
    (v19, v20, v21)
}
fn counts_run_0() -> i32 {
    let mut v0: Rc<UH0> = sequence_words_8();
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v3, mut v4): (Rc<UH0>, Rc<str>) = paths_from_12(v0.clone(), v1, v2.clone());
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH0> = counted_names_15(v0.clone(), v5);
    let v7: Rc<str> = stray_name_18(v6, v0.clone());
    let v8: Rc<str> = kit_log_counts::kit_sequence_counts_missing_set();
    let v9: bool = &*v4 != "";
    if v9 {
        refused_1(v4)
    } else {
        let v11: bool = &*v7 != "";
        if v11 {
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" names no --count column"); } LIT.with(|lit| lit.clone()) }));
            refused_1(v12)
        } else {
            let mut v14: bool = &*v8 != "";
            if v14 {
                let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such set file: "); } LIT.with(|lit| lit.clone()) }, v8));
                refused_1(v15)
            } else {
                match &*v3 {
                    UH0::UH0_Nil => {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory"); } LIT.with(|lit| lit.clone()) };
                        refused_1(v17)
                    }
                    _ => {
                        let v19: Rc<str> = missing_path_21(v3.clone());
                        let v20: bool = &*v19 != "";
                        if v20 {
                            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) }, v19));
                            refused_1(v21)
                        } else {
                            let mut v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let mut v24: Rc<UH0> = files_in_24(v3, v23);
                            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                            let v26: Rc<str> = value_after_31(v25, v0.clone());
                            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--section-on"); } LIT.with(|lit| lit.clone()) };
                            let v28: Rc<str> = value_after_31(v27, v0);
                            let v29: u64 = 0u64;
                            let v30: u64 = 0u64;
                            let (mut v31, mut v32, mut v33): (Rc<str>, u64, u64) = counts_of_files_32(v24.clone(), v28, v2, v29, v30);
                            counts_written_2(v26, v24, v31, v32, v33)
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        counts_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_counts_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_2617b4e49837 = Mut1;
