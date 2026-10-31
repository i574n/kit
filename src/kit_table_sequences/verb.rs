#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
fn command_line_argument_count_4() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_7(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_6(mut v0: i32) -> Rc<str> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_4();
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
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_Some(v9) => {
            let v9: Rc<str> = v9.clone();
            v9
        }
    }
}
fn has_counts_5(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let v2: bool = v0 >= v1;
        if v2 {
            return false;
        } else {
            let v3: Rc<str> = argument_6(v0);
            let v4: bool = &*v3 == "--count";
            if v4 {
                return true;
            } else {
                let v5: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v5, v1);
                continue;
            }
        }
    }
}
fn has_columns_8(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let v2: bool = v0 >= v1;
        if v2 {
            return false;
        } else {
            let v3: Rc<str> = argument_6(v0);
            let v4: bool = &*v3 == "--first";
            let v7: bool = if v4 {
                true
            } else {
                let v5: Rc<str> = argument_6(v0);
                let v6: bool = &*v5 == "--last";
                v6
            };
            if v7 {
                return true;
            } else {
                let v8: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v8, v1);
                continue;
            }
        }
    }
}
fn sequences_run_0() -> i32 {
    let v0: i32 = command_line_argument_count_4();
    let v1: i32 = 2i32;
    let v2: bool = has_counts_5(v1, v0);
    if v2 {
        counts_run_1()
    } else {
        let v4: i32 = 2i32;
        let v5: bool = has_columns_8(v4, v0);
        if v5 {
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
