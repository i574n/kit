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
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table sequences "); } LIT.with(|lit| lit.clone()) };
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
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_6(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method5(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn loop_7(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
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
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
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
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v22)), v1));
        eprintln!("{}", v24);
        ()
    };
    0i32
}
fn command_line_argument_count_8() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn command_line_argument_12(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_11(mut v0: i32) -> Rc<str> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_8();
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
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
                (v0, v1) = (v0.clone(), v3);
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
            let mut v5: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let v7: i32 = 0i32;
            let v8: i32 = method15(v5.clone(), v6, v7);
            let v9: i32 = v6.wrapping_sub(1i32);
            let v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = method16(v10.clone(), v11);
            let v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let v14: bool = &*v13 == &*v0;
            if v14 {
                return true;
            } else {
                let mut v15: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v2, v15);
                continue;
            }
        }
    }
}
fn known_flag_13(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time|--interval|--window|--order|--latency|--between|--from|--to|--ms|--modulo|--out|--section-on|--carry|--first|--last|--count|--count-in|--in-set"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
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
                    (v0, v1, v2, v3) = (v11, v1, v2.clone(), v3.clone());
                    continue;
                } else {
                    let v14: bool = &*v3 == "";
                    let v17: Rc<str> = if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v7));
                        v16.clone()
                    } else {
                        v3.clone()
                    };
                    (v0, v1, v2, v3) = (v1, v1, v2.clone(), v17.clone());
                    continue;
                }
            } else {
                let v22: i32 = v0.wrapping_add(1i32);
                let mut v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()));
                (v0, v1, v2, v3) = (v22, v1, v23.clone(), v3.clone());
                continue;
            }
        }
    }
}
fn value_after_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
                (v0, v1, v2) = (v0.clone(), v3, v2);
                continue;
            }
        }
    }
}
fn method20(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn split_pair_19(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method20(v2.clone());
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
        let v14: i32 = v7.wrapping_add(v13);
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
fn method22(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_21(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method22(v1.clone());
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
fn measure_from_17(mut v0: i32) -> (Rc<str>, US1) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--interval"); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = 2i32;
    let mut v3: Rc<str> = value_after_18(v1, v2, v0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--window"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = 2i32;
    let mut v6: Rc<str> = value_after_18(v4, v5, v0);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--order"); } LIT.with(|lit| lit.clone()) };
    let v8: i32 = 2i32;
    let v9: Rc<str> = value_after_18(v7, v8, v0);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--latency"); } LIT.with(|lit| lit.clone()) };
    let v11: i32 = 2i32;
    let v12: Rc<str> = value_after_18(v10, v11, v0);
    let v13: bool = &*v3 != "";
    if v13 {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let (mut v15, mut v16): (Rc<str>, Rc<str>) = split_pair_19(v3, v14);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--between"); } LIT.with(|lit| lit.clone()) };
        let v18: i32 = 2i32;
        let v19: Rc<str> = value_after_18(v17, v18, v0);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--modulo"); } LIT.with(|lit| lit.clone()) };
        let v21: i32 = 2i32;
        let v22: Rc<str> = value_after_18(v20, v21, v0);
        let mut v23: u64 = decimal_21(v22);
        let v24: US2 = US2::US2_IntervalOf(v16, v19, v23);
        let v25: US1 = US1::US1_Some(v24);
        (v15, v25)
    } else {
        let v26: bool = &*v6 != "";
        if v26 {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let (mut v28, mut v29): (Rc<str>, Rc<str>) = split_pair_19(v6, v27);
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--from"); } LIT.with(|lit| lit.clone()) };
            let v31: i32 = 2i32;
            let mut v32: Rc<str> = value_after_18(v30, v31, v0);
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ms"); } LIT.with(|lit| lit.clone()) };
            let v34: i32 = 2i32;
            let v35: Rc<str> = value_after_18(v33, v34, v0);
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let (mut v37, mut v38): (Rc<str>, Rc<str>) = split_pair_19(v35, v36);
            let v39: bool = &*v32 == "";
            let v45: US1 = if v39 {
                US1::US1_None
            } else {
                let v41: u64 = decimal_21(v37);
                let v42: u64 = decimal_21(v38);
                let v43: US2 = US2::US2_WindowOf(v29, v32, v41, v42);
                US1::US1_Some(v43)
            };
            (v28, v45)
        } else {
            let v46: bool = &*v9 != "";
            if v46 {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let (mut v48, mut v49): (Rc<str>, Rc<str>) = split_pair_19(v9, v47);
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let (mut v51, mut v52): (Rc<str>, Rc<str>) = split_pair_19(v49, v50);
                let v53: bool = &*v52 == "";
                let v57: US1 = if v53 {
                    US1::US1_None
                } else {
                    let v55: US2 = US2::US2_OrderOf(v51, v52);
                    US1::US1_Some(v55)
                };
                (v48, v57)
            } else {
                let v58: bool = &*v12 != "";
                if v58 {
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let (mut v60, mut v61): (Rc<str>, Rc<str>) = split_pair_19(v12, v59);
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--to"); } LIT.with(|lit| lit.clone()) };
                    let v63: i32 = 2i32;
                    let v64: Rc<str> = value_after_18(v62, v63, v0);
                    let v65: bool = &*v64 == "";
                    let v69: US1 = if v65 {
                        US1::US1_None
                    } else {
                        let v67: US2 = US2::US2_LatencyOf(v61, v64);
                        US1::US1_Some(v67)
                    };
                    (v60, v69)
                } else {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v71: US1 = US1::US1_None;
                    (v70, v71)
                }
            }
        }
    }
}
fn file_exists_24(mut v0: Rc<str>) -> bool {
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
fn directory_exists_25(mut v0: Rc<str>) -> bool {
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
fn missing_path_23(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
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
                let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1.clone();
            }
        }
    }
}
fn method27(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0
}
fn method28(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let mut v3: bool = v2 < v0;
    v3
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let v1: Rc<str> = (v0);
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
            let v7: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let v9: i32 = v2.wrapping_sub(1i32);
            let v10: bool = file_exists_24(v8.clone());
            let v12: Rc<UH0> = if v10 {
                Rc::new(UH0::UH0_Cons(v8.clone(), v3.clone()))
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v9, v12.clone());
            continue;
        }
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
                    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v13 as usize]));
                    let mut v15: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method28(v13, v15.clone()) {
                        let v17: i32 = v15.borrow().l0;
                        let v18: std::string::String = v12.clone().borrow()[v17 as usize].clone();
                        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        v14.clone().borrow_mut()[v17 as usize] = v20.clone();
                        let v21: i32 = v17.wrapping_add(1i32);
                        v15.borrow_mut().l0 = v21;
                        ()
                    };
                    let v22: Rc<RefCell<Vec<Rc<str>>>> = v14 ;
                    let mut v23: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method29();
                    let v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().clone()));
                    let v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().iter().map(|x| v23(x.clone())).collect::<Vec<_>>()));
                    let v26: i32 = (v22.clone().borrow().len() as i32);
                    let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
                    while method28(v26, v27.clone()) {
                        let v29: i32 = v27.borrow().l0;
                        let v30: Rc<str> = v24.clone().borrow()[v29 as usize].clone();
                        let v31: Rc<str> = v25.clone().borrow()[v29 as usize].clone();
                        let mut v32: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v29 }));
                        while method30(v25.clone(), v31.clone(), v32.clone()) {
                            let v34: i32 = v32.borrow().l0;
                            let v35: i32 = v34.wrapping_sub(1i32);
                            let v36: Rc<str> = v24.clone().borrow()[v35 as usize].clone();
                            v24.clone().borrow_mut()[v34 as usize] = v36.clone();
                            let v37: Rc<str> = v25.clone().borrow()[v35 as usize].clone();
                            v25.clone().borrow_mut()[v34 as usize] = v37.clone();
                            v32.borrow_mut().l0 = v35;
                            ()
                        };
                        let v38: i32 = v32.borrow().l0;
                        v24.clone().borrow_mut()[v38 as usize] = v30.clone();
                        let v39: i32 = v32.borrow().l0;
                        v25.clone().borrow_mut()[v39 as usize] = v31.clone();
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
                let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method10(v1.clone(), v2.clone());
            }
        }
    }
}
fn method35(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn data_rows_36(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    let mut v4: i32 = (v0.clone().borrow().len() as i32);
    let mut v5: bool = v1 >= v4;
    let (mut v23, mut v24): (Rc<str>, u64) = if v5 {
        (v2.clone(), v3)
    } else {
        let mut v6: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
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
            let v14: i32 = v1.wrapping_add(1i32);
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
fn file_table_of_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2) -> (Rc<str>, u64) {
    let (mut v21, mut v22): (Rc<str>, Rc<str>) = match &v3 {
        US2::US2_IntervalOf(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: u64 = *v6;
            (v4.clone(), v5.clone())
        }
        US2::US2_LatencyOf(v13, v14) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = v14.clone();
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
    let v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let v26: Vec<u8> = v24.unwrap();
    let mut v27: Vec<u8> = method35(v26.clone());
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
    let (mut v63, mut v64): (Rc<str>, u64) = data_rows_36(v59.clone(), v60, v61.clone(), v62);
    (v63.clone(), v64)
}
fn tables_of_33(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: Rc<str>, mut v5: u64) -> (Rc<str>, u64) {
    let (mut v16, mut v17): (Rc<str>, u64) = match &*v0 {
        UH0::UH0_Nil => {
            (v4.clone(), v5)
        }
        _ => {
            let (mut v14, mut v15): (Rc<str>, u64) = match &*v0 {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6.clone();
                    let v7: Rc<UH0> = v7.clone();
                    let (mut v8, mut v9): (Rc<str>, u64) = file_table_of_34(v6, v1.clone(), v2.clone(), v3.clone());
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v8));
                    let v11: u64 = v5.wrapping_add(v9);
                    tables_of_33(v7, v1, v2, v3, v10, v11)
                }
                _ => unreachable!(),
            };
            (v14, v15)
        }
    };
    (v16, v17)
}
fn measures_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_8();
    let v1: i32 = 2i32;
    let v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v4, mut v5): (Rc<UH0>, Rc<str>) = paths_from_9(v1, v0, v2, v3.clone());
    let (mut v6, mut v7): (Rc<str>, US1) = measure_from_17(v0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
    let v9: i32 = 2i32;
    let v10: Rc<str> = value_after_18(v8, v9, v0);
    let v11: bool = &*v5 != "";
    if v11 {
        refused_1(v5)
    } else {
        match &v7 {
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
                    let v17: bool = &*v6 == "";
                    v17
                };
                if v18 {
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --time and a measure name"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v19)
                } else {
                    match &*v4 {
                        UH0::UH0_Nil => {
                            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory"); } LIT.with(|lit| lit.clone()) };
                            refused_1(v21)
                        }
                        _ => {
                            let mut v23: Rc<str> = missing_path_23(v4.clone());
                            let v24: bool = &*v23 != "";
                            if v24 {
                                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
                                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v23));
                                refused_1(v26)
                            } else {
                                let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let v29: Rc<UH0> = files_in_26(v4, v28);
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                                let v31: i32 = 2i32;
                                let mut v32: Rc<str> = value_after_18(v30, v31, v0);
                                let v33: u64 = 0u64;
                                let (mut v34, mut v35): (Rc<str>, u64) = tables_of_33(v29.clone(), v6.clone(), v10, v15, v3, v33);
                                table_written_2(v6, v32, v29, v34, v35)
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
