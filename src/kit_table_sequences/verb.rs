#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
fn counts_run_1() -> i32 {
    let v0: i32 = kit_table_counts::kit_table_counts_run();
    v0
}
fn sections_run_2() -> i32 {
    let v0: i32 = kit_table_sections::kit_table_sections_run();
    v0
}
fn measures_run_3() -> i32 {
    let v0: i32 = kit_table_sequence_measures::kit_table_sequence_measures_run();
    v0
}
fn command_line_argument_count_6() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_5(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_6();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_7(v0);
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
            let v12: Rc<UH0> = arguments_from_5(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn sequence_words_4() -> Rc<UH0> {
    let v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_5(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
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
fn has_flag_8(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    v1 = v3.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn sequences_run_0() -> i32 {
    let v0: Rc<UH0> = sequence_words_4();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("first"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--first"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = has_flag_8(v3, v0.clone());
    let v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
        has_flag_8(v6, v0.clone())
    };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count"); } LIT.with(|lit| lit.clone()) };
    let v11: bool = has_flag_8(v10, v0);
    if v11 {
        counts_run_1()
    } else {
        if v8 {
            sections_run_2()
        } else {
            measures_run_3()
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        sequences_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_sequences_run() -> i32 {
    closure0()()
}
