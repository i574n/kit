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
struct Mut1 { l0: i32 }
struct Mut2 { l0: i32 }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table sequences "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
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
                let mut v2: Rc<str> = v2.clone();
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
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.ends_with(&*v1);
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
fn counts_written_2(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = kit_log_counts::kit_sequence_counts_header();
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v6)), v2));
    let v9: i32 = 0i32;
    let v10: i32 = method3(v1, v9);
    let v11: Rc<str> = format_real_4(v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table sequences ok files="); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sections="); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let v16: Rc<str> = format_real_6(v4);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = format_real_6(v3);
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: bool = &*v0 == "";
    if v22 {
        let mut v23: Rc<str> = loop_7(v8.clone());
        println!("{}", v23);
        eprintln!("{}", v21);
        ()
    } else {
        std::fs::write(&*v0, &*v8).unwrap();
        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v25)), v0));
        eprintln!("{}", v27);
        ()
    };
    0i32
}
fn command_line_argument_count_8() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn command_line_argument_12(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_11(mut v0: i32) -> Rc<str> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_8();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_12(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_Some(v9) => {
            let v9: Rc<str> = v9.clone();
            v9
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn loop_14(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method15(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method16(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: bool = &*v13 == &*v0;
            if v14 {
                return true;
            } else {
                let mut v15: i32 = v3.wrapping_add(1i32);
                v3 = v15;
                continue;
            }
        }
    }
}
fn known_flag_13(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time|--interval|--window|--order|--latency|--between|--from|--to|--ms|--modulo|--out|--section-on|--carry|--first|--last|--count|--count-in|--in-set"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: i32 = 0i32;
    loop_14(v0.clone(), v3.clone(), v4, v5)
}
fn paths_from_9(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        let mut v4: bool = v0 >= v1;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<UH0> = method10(v2.clone(), v5.clone());
            return (v6.clone(), v3.clone());
        } else {
            let v7: Rc<str> = argument_11(v0);
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let v9: bool = v7.starts_with(&*v8);
            if v9 {
                let v10: bool = known_flag_13(v7.clone());
                if v10 {
                    let v11: i32 = v0.wrapping_add(2i32);
                    v0 = v11;
                    continue;
                } else {
                    let mut v14: bool = &*v3 == "";
                    let v17: Rc<str> = if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v7));
                        v16.clone()
                    } else {
                        v3.clone()
                    };
                    (v0, v3) = (v1, v17.clone());
                    continue;
                }
            } else {
                let v22: i32 = v0.wrapping_add(1i32);
                let mut v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()));
                (v0, v2) = (v22, v23.clone());
                continue;
            }
        }
    }
}
fn method19(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn split_pair_18(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method19(v2.clone());
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
fn counted_names_17(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v3: i32 = v0.wrapping_add(1i32);
        let mut v4: bool = v3 >= v1;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = argument_11(v0);
            let mut v6: bool = &*v5 == "--count";
            if v6 {
                let v7: Rc<str> = argument_11(v3);
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let (mut v9, mut v10): (Rc<str>, Rc<str>) = split_pair_18(v7.clone(), v8.clone());
                let v11: i32 = v0.wrapping_add(2i32);
                let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v2.clone()));
                (v0, v2) = (v11, v12.clone());
                continue;
            } else {
                v0 = v3;
                continue;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = &*v1 == &*v0;
        v2
    })
}
fn method21(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method22(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: bool = v1(v3.clone());
                if v5 {
                    return true;
                } else {
                    let mut v6: Rc<dyn Fn(Rc<str>) -> bool> = method21(v0.clone());
                    (v1, v2) = (v6.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn stray_name_20(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v3 >= v2;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_11(v1);
            let v7: bool = &*v6 == "--count-in";
            let v10: bool = if v7 {
                true
            } else {
                let v8: Rc<str> = argument_11(v1);
                let v9: bool = &*v8 == "--in-set";
                v9
            };
            if v10 {
                let v11: Rc<str> = argument_11(v3);
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let (mut v13, mut v14): (Rc<str>, Rc<str>) = split_pair_18(v11.clone(), v12.clone());
                let mut v15: Rc<dyn Fn(Rc<str>) -> bool> = method21(v13.clone());
                let v16: bool = method22(v13.clone(), v15.clone(), v0.clone());
                if v16 {
                    let v17: i32 = v1.wrapping_add(2i32);
                    v1 = v17;
                    continue;
                } else {
                    let v19: Rc<str> = argument_11(v1);
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v13));
                    return v22.clone();
                }
            } else {
                v1 = v3;
                continue;
            }
        }
    }
}
fn file_exists_24(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn directory_exists_25(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn missing_path_23(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = file_exists_24(v2.clone());
                let mut v6: bool = if v4 {
                    true
                } else {
                    directory_exists_25(v2.clone())
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
fn method27(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method28(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method30(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut2>>) -> bool {
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
fn named_files_31(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
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
            let v10: bool = file_exists_24(v8.clone());
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
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
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
fn files_in_26(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = directory_exists_25(v4.clone());
                if v6 {
                    let v8: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                    let v10: Vec<std::string::String> = v8.unwrap();
                    let v11: Vec<std::string::String> = method27(v10.clone());
                    let v12: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v11));
                    let v13: i32 = (v12.clone().borrow().len() as i32);
                    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v13 as usize]));
                    let mut v15: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method28(v13, v15.clone()) {
                        let v17: i32 = v15.borrow().l0;
                        let v18: std::string::String = v12.borrow()[v17 as usize].clone();
                        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        v14.borrow_mut()[v17 as usize] = v20.clone();
                        let v21: i32 = v17.wrapping_add(1i32);
                        v15.borrow_mut().l0 = v21;
                        ()
                    };
                    let v22: Rc<RefCell<Vec<Rc<str>>>> = v14 ;
                    let mut v23: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method29();
                    let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().clone()));
                    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().iter().map(|x| v23(x.clone())).collect::<Vec<_>>()));
                    let v26: i32 = (v22.clone().borrow().len() as i32);
                    let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
                    while method28(v26, v27.clone()) {
                        let v29: i32 = v27.borrow().l0;
                        let v30: Rc<str> = v24.borrow()[v29 as usize].clone();
                        let v31: Rc<str> = v25.borrow()[v29 as usize].clone();
                        let mut v32: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v29 }));
                        while method30(v25.clone(), v31.clone(), v32.clone()) {
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
                    let v45: Rc<UH0> = named_files_31(v4.clone(), v41.clone(), v43, v44.clone());
                    let v46: Rc<UH0> = method32(v45.clone(), v1.clone());
                    (v0, v1) = (v5.clone(), v46.clone());
                    continue;
                } else {
                    let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v1.clone()));
                    (v0, v1) = (v5.clone(), v48.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method10(v1.clone(), v2.clone());
            }
        }
    }
}
fn value_after_33(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v3 >= v2;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_11(v1);
            let v7: bool = &*v6 == &*v0;
            if v7 {
                return argument_11(v3);
            } else {
                v1 = v3;
                continue;
            }
        }
    }
}
fn data_rows_36(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
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
            let (mut v12, mut v13): (Rc<str>, u64) = data_rows_36(v0.clone(), v11, v2.clone(), v3);
            (v12, v13)
        } else {
            let mut v14: i32 = v1.wrapping_add(1i32);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let v18: u64 = v3.wrapping_add(1u64);
            let (mut v19, mut v20): (Rc<str>, u64) = data_rows_36(v0, v14, v17, v18);
            (v19, v20)
        };
        (v21, v22)
    };
    (v23, v24)
}
fn counts_of_file_35(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, u64, u64) {
    let (mut v2, mut v3): (Rc<str>, u64) = kit_log_counts::kit_sequence_counts(&*v0, &*v1);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v4).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v6: i32 = 0i32;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: u64 = 0u64;
    let (mut v9, mut v10): (Rc<str>, u64) = data_rows_36(v5.clone(), v6, v7.clone(), v8);
    (v9.clone(), v10, v3)
}
fn counts_of_files_34(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> (Rc<str>, u64, u64) {
    let (mut v19, mut v20, mut v21): (Rc<str>, u64, u64) = match &*v0 {
        UH0::UH0_Nil => {
            (v2.clone(), v3, v4)
        }
        _ => {
            let (mut v16, mut v17, mut v18): (Rc<str>, u64, u64) = match &*v0 {
                UH0::UH0_Cons(v5, v6) => {
                    let mut v5: Rc<str> = v5.clone();
                    let mut v6: Rc<UH0> = v6.clone();
                    let (mut v7, mut v8, mut v9): (Rc<str>, u64, u64) = counts_of_file_35(v5, v1.clone());
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
                    let v11: u64 = v3.wrapping_add(v8);
                    let v12: u64 = v4.wrapping_add(v9);
                    counts_of_files_34(v6, v1, v10, v11, v12)
                }
                _ => unreachable!(),
            };
            (v16, v17, v18)
        }
    };
    (v19, v20, v21)
}
fn counts_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_8();
    let mut v1: i32 = 2i32;
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v4, mut v5): (Rc<UH0>, Rc<str>) = paths_from_9(v1, v0, v2, v3.clone());
    let mut v6: i32 = 2i32;
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = counted_names_17(v6, v0, v7);
    let v9: i32 = 2i32;
    let v10: Rc<str> = stray_name_20(v8, v9, v0);
    let v11: Rc<str> = kit_log_counts::kit_sequence_counts_missing_set();
    let v12: bool = &*v5 != "";
    if v12 {
        refused_1(v5)
    } else {
        let mut v14: bool = &*v10 != "";
        if v14 {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" names no --count column"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v15));
            refused_1(v16)
        } else {
            let v18: bool = &*v11 != "";
            if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such set file: "); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v11));
                refused_1(v20)
            } else {
                match &*v4 {
                    UH0::UH0_Nil => {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory"); } LIT.with(|lit| lit.clone()) };
                        refused_1(v22)
                    }
                    _ => {
                        let mut v24: Rc<str> = missing_path_23(v4.clone());
                        let mut v25: bool = &*v24 != "";
                        if v25 {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
                            let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v24));
                            refused_1(v27)
                        } else {
                            let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            let v30: Rc<UH0> = files_in_26(v4, v29);
                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                            let mut v32: i32 = 2i32;
                            let v33: Rc<str> = value_after_33(v31, v32, v0);
                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--section-on"); } LIT.with(|lit| lit.clone()) };
                            let v35: i32 = 2i32;
                            let v36: Rc<str> = value_after_33(v34, v35, v0);
                            let v37: u64 = 0u64;
                            let v38: u64 = 0u64;
                            let (mut v39, mut v40, mut v41): (Rc<str>, u64, u64) = counts_of_files_34(v30.clone(), v36, v3, v37, v38);
                            counts_written_2(v33, v30, v39, v40, v41)
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
