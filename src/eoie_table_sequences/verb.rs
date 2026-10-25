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
fn sections_run_1() -> i32 {
    let mut v0: i32 = eoie_table_sections::eoie_table_sections_run();
    v0
}
fn refused_3(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: table sequences "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table sequences <file|dir>... --time <regex with (?<ms>digits)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex with (?<section>)>] [--carry <regex>] (--first|--last) name=<regex with (?<id>) and (?<value>)>...) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    eprintln!("{}", v6);
    2i32
}
fn method5(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method7(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn format_real_8(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn loop_9(mut v0: Rc<str>) -> Rc<str> {
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
fn table_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file\tline\t"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v0));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v3));
    let mut v10: i32 = 0i32;
    let mut v11: i32 = method5(v2.clone(), v10);
    let mut v12: Rc<str> = format_real_6(v11);
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie table sequences ok files="); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let mut v17: Rc<str> = format_real_8(v4);
    let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let mut v19: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v19 {
        let mut v20: Rc<str> = loop_9(v9.clone());
        println!("{}", v20);
        eprintln!("{}", v18);
        ()
    } else {
        std::fs::write(&*v1, &*v9).unwrap();
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v22));
        let mut v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v1));
        eprintln!("{}", v24);
        ()
    };
    0i32
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn command_line_argument_count_13() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_14(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn argument_12(mut v0: i32) -> Rc<str> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_13();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_14(v0);
        US0::US0_Some(v5.clone())
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_Some(v9) => {
            let mut v9: Rc<str> = v9.clone();
            v9.clone()
        }
    }
}
fn method17(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method18(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
fn loop_16(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v2;
        if v4 {
            return false;
        } else {
            let mut v5: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: i32 = 0i32;
            let mut v8: i32 = method17(v5.clone(), v6, v7);
            let mut v9: i32 = v6.wrapping_sub(1i32);
            let mut v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
            let mut v11: i32 = (v10.clone().len() as i32);
            let mut v12: i32 = method18(v10.clone(), v11);
            let mut v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
            let mut v14: bool = v13.clone() == v0.clone();
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
fn known_flag_15(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time|--interval|--window|--order|--latency|--between|--from|--to|--ms|--modulo|--out|--section-on|--carry|--first|--last"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v2).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v4: i32 = (v3.clone().borrow().len() as i32);
    let mut v5: i32 = 0i32;
    loop_16(v0.clone(), v3.clone(), v4, v5)
}
fn paths_from_10(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>, mut v3: Rc<str>) -> (Rc<UH0>, Rc<str>) {
    loop {
        let mut v4: bool = v0 >= v1;
        if v4 {
            let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<UH0> = method11(v2.clone(), v5.clone());
            return (v6.clone(), v3.clone());
        } else {
            let mut v7: Rc<str> = argument_12(v0);
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v7.starts_with(&*v8);
            if v9 {
                let mut v10: bool = known_flag_15(v7.clone());
                if v10 {
                    let mut v11: i32 = v0.wrapping_add(2i32);
                    (v0, v1, v2, v3) = (v11, v1, v2.clone(), v3.clone());
                    continue;
                } else {
                    let mut v14: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v17: Rc<str> = if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                        let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v7));
                        v16.clone()
                    } else {
                        v3.clone()
                    };
                    (v0, v1, v2, v3) = (v1, v1, v2.clone(), v17.clone());
                    continue;
                }
            } else {
                let mut v22: i32 = v0.wrapping_add(1i32);
                let mut v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v2.clone()));
                (v0, v1, v2, v3) = (v22, v1, v23.clone(), v3.clone());
                continue;
            }
        }
    }
}
fn value_after_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v3 >= v2;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let mut v6: Rc<str> = argument_12(v1);
            let mut v7: bool = v6.clone() == v0.clone();
            if v7 {
                return argument_12(v3);
            } else {
                (v0, v1, v2) = (v0.clone(), v3, v2);
                continue;
            }
        }
    }
}
fn method22(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn split_pair_21(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let mut v2: Option<usize> = v0.find(&*v1);
    let mut v3: Option<usize> = method22(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let mut v8: bool = v7 < 0i32;
    if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v0.clone(), v9.clone())
    } else {
        let mut v10: usize = ((0i32) as usize);
        let mut v11: usize = ((v7) as usize);
        let mut v12: Rc<str> = Rc::<str>::from(&(v0)[v10..v11]);
        let mut v13: i32 = (v1.clone().len() as i32);
        let mut v14: i32 = v7.wrapping_add(v13);
        let mut v15: i32 = (v0.clone().len() as i32);
        let mut v16: i32 = v15.wrapping_sub(v7);
        let mut v17: i32 = v16.wrapping_sub(v13);
        let mut v18: usize = ((v14) as usize);
        let mut v19: i32 = v14.wrapping_add(v17);
        let mut v20: usize = ((v19) as usize);
        let mut v21: Rc<str> = Rc::<str>::from(&(v0)[v18..v20]);
        (v12.clone(), v21.clone())
    }
}
fn method24(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure2() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_23(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method24(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure2();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let mut v7: US3 = v6.clone();
    let mut v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn measure_from_19(mut v0: i32) -> (Rc<str>, US1) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--interval"); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = 2i32;
    let mut v3: Rc<str> = value_after_20(v1.clone(), v2, v0);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--window"); } LIT.with(|lit| lit.clone()) };
    let mut v5: i32 = 2i32;
    let mut v6: Rc<str> = value_after_20(v4.clone(), v5, v0);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--order"); } LIT.with(|lit| lit.clone()) };
    let mut v8: i32 = 2i32;
    let mut v9: Rc<str> = value_after_20(v7.clone(), v8, v0);
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--latency"); } LIT.with(|lit| lit.clone()) };
    let mut v11: i32 = 2i32;
    let mut v12: Rc<str> = value_after_20(v10.clone(), v11, v0);
    let mut v13: bool = v3.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
        let (mut v15, mut v16): (Rc<str>, Rc<str>) = split_pair_21(v3.clone(), v14.clone());
        let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--between"); } LIT.with(|lit| lit.clone()) };
        let mut v18: i32 = 2i32;
        let mut v19: Rc<str> = value_after_20(v17.clone(), v18, v0);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--modulo"); } LIT.with(|lit| lit.clone()) };
        let mut v21: i32 = 2i32;
        let mut v22: Rc<str> = value_after_20(v20.clone(), v21, v0);
        let mut v23: u64 = decimal_23(v22.clone());
        let mut v24: US2 = US2::US2_IntervalOf(v16.clone(), v19.clone(), v23);
        let mut v25: US1 = US1::US1_Some(v24.clone());
        (v15.clone(), v25.clone())
    } else {
        let mut v26: bool = v6.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v26 {
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
            let (mut v28, mut v29): (Rc<str>, Rc<str>) = split_pair_21(v6.clone(), v27.clone());
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--from"); } LIT.with(|lit| lit.clone()) };
            let mut v31: i32 = 2i32;
            let mut v32: Rc<str> = value_after_20(v30.clone(), v31, v0);
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ms"); } LIT.with(|lit| lit.clone()) };
            let mut v34: i32 = 2i32;
            let mut v35: Rc<str> = value_after_20(v33.clone(), v34, v0);
            let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let (mut v37, mut v38): (Rc<str>, Rc<str>) = split_pair_21(v35.clone(), v36.clone());
            let mut v39: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v45: US1 = if v39 {
                US1::US1_None
            } else {
                let mut v41: u64 = decimal_23(v37.clone());
                let mut v42: u64 = decimal_23(v38.clone());
                let mut v43: US2 = US2::US2_WindowOf(v29.clone(), v32.clone(), v41, v42);
                US1::US1_Some(v43.clone())
            };
            (v28.clone(), v45.clone())
        } else {
            let mut v46: bool = v9.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v46 {
                let mut v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let (mut v48, mut v49): (Rc<str>, Rc<str>) = split_pair_21(v9.clone(), v47.clone());
                let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let (mut v51, mut v52): (Rc<str>, Rc<str>) = split_pair_21(v49.clone(), v50.clone());
                let mut v53: bool = v52.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v57: US1 = if v53 {
                    US1::US1_None
                } else {
                    let mut v55: US2 = US2::US2_OrderOf(v51.clone(), v52.clone());
                    US1::US1_Some(v55.clone())
                };
                (v48.clone(), v57.clone())
            } else {
                let mut v58: bool = v12.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                if v58 {
                    let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                    let (mut v60, mut v61): (Rc<str>, Rc<str>) = split_pair_21(v12.clone(), v59.clone());
                    let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--to"); } LIT.with(|lit| lit.clone()) };
                    let mut v63: i32 = 2i32;
                    let mut v64: Rc<str> = value_after_20(v62.clone(), v63, v0);
                    let mut v65: bool = v64.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v69: US1 = if v65 {
                        US1::US1_None
                    } else {
                        let mut v67: US2 = US2::US2_LatencyOf(v61.clone(), v64.clone());
                        US1::US1_Some(v67.clone())
                    };
                    (v60.clone(), v69.clone())
                } else {
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let mut v71: US1 = US1::US1_None;
                    (v70.clone(), v71.clone())
                }
            }
        }
    }
}
fn file_exists_26(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn directory_exists_27(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let mut v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn missing_path_25(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = file_exists_26(v2.clone());
                let mut v6: bool = if v4 {
                    true
                } else {
                    directory_exists_27(v2.clone())
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
fn method29(mut v0: Vec<std::string::String>) -> Vec<std::string::String> {
    v0.clone()
}
fn method30(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let mut v2: i32 = v1.borrow().l0.clone();
    let mut v3: bool = v2 < v0;
    v3
}
fn closure3() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = Rc::new(move |mut v0: (Rc<str>)| -> Rc<str> {
        let mut v1: Rc<str> = (v0);
        v1.clone()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method31() -> Rc<dyn Fn((Rc<str>)) -> Rc<str>> {
    closure3()
}
fn method32(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<str>, mut v2: Rc<RefCell<Mut2>>) -> bool {
    let mut v3: i32 = v2.borrow().l0.clone();
    let mut v4: bool = v3 > 0i32;
    if v4 {
        let mut v5: i32 = v2.borrow().l0.clone();
        let mut v6: i32 = v5.wrapping_sub(1i32);
        let mut v7: Rc<str> = v0.clone().borrow()[v6 as usize].clone();
        let mut v8: bool = v7.clone() < v1.clone();
        let mut v11: i32 = if v8 {
            -1i32
        } else {
            let mut v9: bool = v7.clone() > v1.clone();
            if v9 {
                1i32
            } else {
                0i32
            }
        };
        let mut v12: bool = v11 == 1i32;
        v12
    } else {
        false
    }
}
fn named_files_33(mut v0: Rc<str>, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3.clone();
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
            let mut v7: Rc<str> = v1.clone().borrow()[v2 as usize].clone();
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: i32 = v2.wrapping_sub(1i32);
            let mut v10: bool = file_exists_26(v8.clone());
            let mut v12: Rc<UH0> = if v10 {
                Rc::new(UH0::UH0_Cons(v8.clone(), v3.clone()))
            } else {
                v3.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v9, v12.clone());
            continue;
        }
    }
}
fn method34(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
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
fn files_in_28(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let mut v5: Rc<UH0> = v5.clone();
                let mut v6: bool = directory_exists_27(v4.clone());
                if v6 {
                    let mut v8: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
                    let mut v10: Vec<std::string::String> = v8.unwrap();
                    let mut v11: Vec<std::string::String> = method29(v10.clone());
                    let mut v12: Rc<RefCell<Vec<std::string::String>>> = Rc::new(RefCell::new(v11));
                    let mut v13: i32 = (v12.clone().borrow().len() as i32);
                    let mut v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v13 as usize]));
                    let mut v15: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                    while method30(v13, v15.clone()) {
                        let mut v17: i32 = v15.borrow().l0.clone();
                        let mut v18: std::string::String = v12.clone().borrow()[v17 as usize].clone();
                        let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
                        v14.clone().borrow_mut()[v17 as usize] = v20.clone();
                        let mut v21: i32 = v17.wrapping_add(1i32);
                        v15.borrow_mut().l0 = v21;
                        ()
                    };
                    let mut v22: Rc<RefCell<Vec<Rc<str>>>> = v14 ;
                    let mut v23: Rc<dyn Fn((Rc<str>)) -> Rc<str>> = method31();
                    let mut v24: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().clone()));
                    let mut v25: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v22.borrow().iter().map(|x| v23(x.clone())).collect::<Vec<_>>()));
                    let mut v26: i32 = (v22.clone().borrow().len() as i32);
                    let mut v27: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 1i32 }));
                    while method30(v26, v27.clone()) {
                        let mut v29: i32 = v27.borrow().l0.clone();
                        let mut v30: Rc<str> = v24.clone().borrow()[v29 as usize].clone();
                        let mut v31: Rc<str> = v25.clone().borrow()[v29 as usize].clone();
                        let mut v32: Rc<RefCell<Mut2>> = Rc::new(RefCell::new(Mut2 { l0: v29 }));
                        while method32(v25.clone(), v31.clone(), v32.clone()) {
                            let mut v34: i32 = v32.borrow().l0.clone();
                            let mut v35: i32 = v34.wrapping_sub(1i32);
                            let mut v36: Rc<str> = v24.clone().borrow()[v35 as usize].clone();
                            v24.clone().borrow_mut()[v34 as usize] = v36.clone();
                            let mut v37: Rc<str> = v25.clone().borrow()[v35 as usize].clone();
                            v25.clone().borrow_mut()[v34 as usize] = v37.clone();
                            v32.borrow_mut().l0 = v35;
                            ()
                        };
                        let mut v38: i32 = v32.borrow().l0.clone();
                        v24.clone().borrow_mut()[v38 as usize] = v30.clone();
                        let mut v39: i32 = v32.borrow().l0.clone();
                        v25.clone().borrow_mut()[v39 as usize] = v31.clone();
                        let mut v40: i32 = v29.wrapping_add(1i32);
                        v27.borrow_mut().l0 = v40;
                        ()
                    };
                    let mut v41: Rc<RefCell<Vec<Rc<str>>>> = v24 ;
                    let mut v42: i32 = (v41.clone().borrow().len() as i32);
                    let mut v43: i32 = v42.wrapping_sub(1i32);
                    let mut v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    let mut v45: Rc<UH0> = named_files_33(v4.clone(), v41.clone(), v43, v44.clone());
                    let mut v46: Rc<UH0> = method34(v45.clone(), v1.clone());
                    (v0, v1) = (v5.clone(), v46.clone());
                    continue;
                } else {
                    let mut v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v1.clone()));
                    (v0, v1) = (v5.clone(), v48.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method11(v1.clone(), v2.clone());
            }
        }
    }
}
fn method37(mut v0: Vec<u8>) -> Vec<u8> {
    v0.clone()
}
fn data_rows_38(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>, mut v3: u64) -> (Rc<str>, u64) {
    let mut v4: i32 = (v0.clone().borrow().len() as i32);
    let mut v5: bool = v1 >= v4;
    let (mut v23, mut v24): (Rc<str>, u64) = if v5 {
        (v2.clone(), v3)
    } else {
        let mut v6: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
        let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v10: bool = if v7 {
            true
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#"); } LIT.with(|lit| lit.clone()) };
            let mut v9: bool = v6.starts_with(&*v8);
            v9
        };
        let (mut v21, mut v22): (Rc<str>, u64) = if v10 {
            let mut v11: i32 = v1.wrapping_add(1i32);
            let (mut v12, mut v13): (Rc<str>, u64) = data_rows_38(v0.clone(), v11, v2.clone(), v3);
            (v12.clone(), v13)
        } else {
            let mut v14: i32 = v1.wrapping_add(1i32);
            let mut v15: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v6));
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let mut v18: u64 = v3.wrapping_add(1u64);
            let (mut v19, mut v20): (Rc<str>, u64) = data_rows_38(v0.clone(), v14, v17.clone(), v18);
            (v19.clone(), v20)
        };
        (v21.clone(), v22)
    };
    (v23.clone(), v24)
}
fn file_table_of_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2) -> (Rc<str>, u64) {
    let (mut v21, mut v22): (Rc<str>, Rc<str>) = match &v3 {
        US2::US2_IntervalOf(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = v5.clone();
            let mut v6: u64 = *v6;
            (v4.clone(), v5.clone())
        }
        US2::US2_LatencyOf(v13, v14) => {
            let mut v13: Rc<str> = v13.clone();
            let mut v14: Rc<str> = v14.clone();
            (v13.clone(), v14.clone())
        }
        US2::US2_OrderOf(v11, v12) => {
            let mut v11: Rc<str> = v11.clone();
            let mut v12: Rc<str> = v12.clone();
            (v11.clone(), v12.clone())
        }
        US2::US2_WindowOf(v7, v8, v9, v10) => {
            let mut v7: Rc<str> = v7.clone();
            let mut v8: Rc<str> = v8.clone();
            let mut v9: u64 = *v9;
            let mut v10: u64 = *v10;
            (v7.clone(), v8.clone())
        }
    };
    let mut v24: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let mut v26: Vec<u8> = v24.unwrap();
    let mut v27: Vec<u8> = method37(v26.clone());
    let mut v29: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v27);
    let mut v31: std::string::String = v29.unwrap();
    let mut v33: Rc<str> = Rc::<str>::from(String::as_str(&v31));
    let mut v34: Rc<str> = eoie_log_sequences::eoie_sequence_events(&*v33, &*v2, &*v21, &*v22);
    let mut v57: Rc<str> = match &v3 {
        US2::US2_IntervalOf(v35, v36, v37) => {
            let mut v35: Rc<str> = v35.clone();
            let mut v36: Rc<str> = v36.clone();
            let mut v37: u64 = *v37;
            let mut v38: bool = v36.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v41: Rc<str> = if v38 {
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v39.clone()
            } else {
                let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("second"); } LIT.with(|lit| lit.clone()) };
                v40.clone()
            };
            let mut v42: Rc<str> = eoie_log_sequences::eoie_sequence_interval(&*v0, &*v34, "first", &*v41, v37);
            v42.clone()
        }
        US2::US2_LatencyOf(v51, v52) => {
            let mut v51: Rc<str> = v51.clone();
            let mut v52: Rc<str> = v52.clone();
            let mut v53: Rc<str> = eoie_log_sequences::eoie_sequence_latency(&*v0, &*v34, "first", "second");
            v53.clone()
        }
        US2::US2_OrderOf(v48, v49) => {
            let mut v48: Rc<str> = v48.clone();
            let mut v49: Rc<str> = v49.clone();
            let mut v50: Rc<str> = eoie_log_sequences::eoie_sequence_order(&*v0, &*v34, "first", "second");
            v50.clone()
        }
        US2::US2_WindowOf(v43, v44, v45, v46) => {
            let mut v43: Rc<str> = v43.clone();
            let mut v44: Rc<str> = v44.clone();
            let mut v45: u64 = *v45;
            let mut v46: u64 = *v46;
            let mut v47: Rc<str> = eoie_log_sequences::eoie_sequence_window(&*v0, &*v34, "first", "second", v45, v46);
            v47.clone()
        }
    };
    let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v57.split(&*v58).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v60: i32 = 0i32;
    let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v62: u64 = 0u64;
    let (mut v63, mut v64): (Rc<str>, u64) = data_rows_38(v59.clone(), v60, v61.clone(), v62);
    (v63.clone(), v64)
}
fn tables_of_35(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: Rc<str>, mut v5: u64) -> (Rc<str>, u64) {
    let (mut v16, mut v17): (Rc<str>, u64) = match &*v0 {
        UH0::UH0_Nil => {
            (v4.clone(), v5)
        }
        _ => {
            let (mut v14, mut v15): (Rc<str>, u64) = match &*v0 {
                UH0::UH0_Cons(v6, v7) => {
                    let mut v6: Rc<str> = v6.clone();
                    let mut v7: Rc<UH0> = v7.clone();
                    let (mut v8, mut v9): (Rc<str>, u64) = file_table_of_36(v6.clone(), v1.clone(), v2.clone(), v3.clone());
                    let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v8));
                    let mut v11: u64 = v5.wrapping_add(v9);
                    tables_of_35(v7.clone(), v1.clone(), v2.clone(), v3.clone(), v10.clone(), v11)
                }
                _ => unreachable!(),
            };
            (v14.clone(), v15)
        }
    };
    (v16.clone(), v17)
}
fn measures_run_2(mut v0: i32) -> i32 {
    let mut v1: i32 = 2i32;
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v4, mut v5): (Rc<UH0>, Rc<str>) = paths_from_10(v1, v0, v2.clone(), v3.clone());
    let (mut v6, mut v7): (Rc<str>, US1) = measure_from_19(v0);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--time"); } LIT.with(|lit| lit.clone()) };
    let mut v9: i32 = 2i32;
    let mut v10: Rc<str> = value_after_20(v8.clone(), v9, v0);
    let mut v11: bool = v5.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v11 {
        refused_3(v5.clone())
    } else {
        match &v7 {
            US1::US1_None => {
                let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs one measure with both its patterns"); } LIT.with(|lit| lit.clone()) };
                refused_3(v13.clone())
            }
            US1::US1_Some(v15) => {
                let mut v15: US2 = v15.clone();
                let mut v16: bool = v10.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v18: bool = if v16 {
                    true
                } else {
                    let mut v17: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v17
                };
                if v18 {
                    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs --time and a measure name"); } LIT.with(|lit| lit.clone()) };
                    refused_3(v19.clone())
                } else {
                    match &*v4 {
                        UH0::UH0_Cons(v23, v24) => {
                            let mut v23: Rc<str> = v23.clone();
                            let mut v24: Rc<UH0> = v24.clone();
                            let mut v25: Rc<str> = missing_path_25(v4.clone());
                            let mut v26: bool = v25.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            if v26 {
                                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) };
                                let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v25));
                                refused_3(v28.clone())
                            } else {
                                let mut v30: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                let mut v31: Rc<UH0> = files_in_28(v4.clone(), v30.clone());
                                let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--out"); } LIT.with(|lit| lit.clone()) };
                                let mut v33: i32 = 2i32;
                                let mut v34: Rc<str> = value_after_20(v32.clone(), v33, v0);
                                let mut v35: u64 = 0u64;
                                let (mut v36, mut v37): (Rc<str>, u64) = tables_of_35(v31.clone(), v6.clone(), v10.clone(), v15.clone(), v3.clone(), v35);
                                table_written_4(v6.clone(), v34.clone(), v31.clone(), v36.clone(), v37)
                            }
                        }
                        UH0::UH0_Nil => {
                            let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory"); } LIT.with(|lit| lit.clone()) };
                            refused_3(v21.clone())
                        }
                    }
                }
            }
        }
    }
}
fn has_columns_39(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 >= v1;
        if v2 {
            return false;
        } else {
            let mut v3: Rc<str> = argument_12(v0);
            let mut v4: bool = v3.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--first"); } LIT.with(|lit| lit.clone()) };
            let mut v7: bool = if v4 {
                true
            } else {
                let mut v5: Rc<str> = argument_12(v0);
                let mut v6: bool = v5.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
                v6
            };
            if v7 {
                return true;
            } else {
                let mut v8: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v8, v1);
                continue;
            }
        }
    }
}
fn sequences_run_0() -> i32 {
    let mut v0: i32 = command_line_argument_count_13();
    let mut v1: i32 = 2i32;
    let mut v2: bool = has_columns_39(v1, v0);
    if v2 {
        sections_run_1()
    } else {
        measures_run_2(v0)
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        sequences_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_table_sequences_run() -> i32 {
    closure0()()
}
