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
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, Rc<UH1>),
}
pub struct Mut1 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_2();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_3(v0);
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
            let v12: Rc<UH0> = arguments_from_1(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method6(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn list_of_base_7(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn row_count_11(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v2 as usize].clone();
            let v10: i32 = v2.wrapping_add(1i32);
            let v11: bool = &*v7 == &*v1;
            let v13: i32 = if v11 {
                let v12: i32 = v3.wrapping_add(v8);
                v12
            } else {
                v3
            };
            (v2, v3) = (v10, v13);
            continue;
        }
    }
}
fn method10(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let v4: i32 = v4;
            let v5: Rc<UH1> = v5;
            let mut v6: Rc<UH1> = method10(v0, v5, v2);
            let mut v7: i32 = 0i32;
            let v8: i32 = 0i32;
            let v9: i32 = row_count_11(v0, v3.clone(), v7, v8);
            let v10: i32 = v4.wrapping_add(v9);
            Rc::new(UH1::UH1_Cons(v3, v10, v6))
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn first_totals_12(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v1 as usize].clone();
            let v9: i32 = 0i32;
            let v10: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v9, v2));
            (v1, v2) = (v4, v10);
            continue;
        }
    }
}
fn totals_added_9(mut v0: Rc<UH1>, mut v1: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> Rc<UH1> {
    let v2: bool = match &*v0 {
        UH1::UH1_Nil => {
            true
        }
        _ => {
            false
        }
    };
    let mut v7: Rc<UH1> = if v2 {
        let v3: i32 = (v1.clone().borrow().len() as i32);
        let v4: i32 = v3.wrapping_sub(1i32);
        let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        first_totals_12(v1, v4, v5)
    } else {
        v0
    };
    let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method10(v1, v7, v8)
}
fn method8(mut v0: Rc<UH0>, mut v1: i32, mut v2: Rc<UH1>) -> (i32, Rc<UH1>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_measure::kit_style_measured(&v3);
                let mut v6: i32 = (v5.clone().borrow().len() as i32);
                let mut v7: bool = v6 > 0i32;
                let v12: i32 = if v7 {
                    let (mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, i32, i32) = v5.borrow()[0i32 as usize].clone();
                    v11
                } else {
                    0i32
                };
                let v13: i32 = v1.wrapping_add(v12);
                let v14: Rc<UH1> = totals_added_9(v2, &v5);
                (v0, v1, v2) = (v4, v13, v14);
                continue;
            }
            UH0::UH0_Nil => {
                return (v1, v2);
            }
        }
    }
}
fn method13(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_18(mut v0: i64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn per_kloc_17(mut v0: i32, mut v1: i32) -> Rc<str> {
    let v2: i64 = (v0 as i64);
    let v3: i64 = v2.wrapping_mul(10000i64);
    let v4: bool = v1 > 0i32;
    let v5: i32 = if v4 {
        v1
    } else {
        1i32
    };
    let mut v6: i64 = (v5 as i64);
    let mut v7: i64 = v3.wrapping_div(v6);
    let v8: i64 = v7.wrapping_div(10i64);
    let v9: Rc<str> = format_real_18(v8);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
    let v11: i64 = v7.wrapping_rem(10i64);
    let v12: Rc<str> = format_real_18(v11);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v12));
    v13
}
fn method16(mut v0: i32, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) }));
                let v9: Rc<str> = format_real_14(v4);
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) }));
                let v12: Rc<str> = per_kloc_17(v4, v0);
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
                (v1, v2) = (v5, v14);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn scoreboard_line_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v0.clone()]));
    let v3: i32 = (v1.clone().borrow().len() as i32);
    let v4: i32 = (v2.clone().borrow().len() as i32);
    let v5: i32 = v3.wrapping_add(v4);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v5 as usize]));
    let mut v7: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
    while method6(v5, &v7) {
        let v9: i32 = v7.borrow().l0;
        let v10: bool = v9 < v3;
        let v14: Rc<str> = if v10 {
            let v11: Rc<str> = v1.borrow()[v9 as usize].clone();
            v11
        } else {
            let v12: i32 = v9.wrapping_sub(v3);
            let v13: Rc<str> = v2.borrow()[v12 as usize].clone();
            v13
        };
        v6.borrow_mut()[v9 as usize] = v14;
        let v15: i32 = v9.wrapping_add(1i32);
        v7.borrow_mut().l0 = v15;
        ()
    };
    let v16: Rc<RefCell<Vec<Rc<str>>>> = kit_style_measure::kit_style_packages_under(v6);
    let v17: i32 = (v16.clone().borrow().len() as i32);
    let v18: i32 = v17.wrapping_sub(1i32);
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH0> = list_of_base_7(&v16, v18, v19);
    let v21: i32 = 0i32;
    let v22: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let (mut v23, mut v24): (i32, Rc<UH1>) = method8(v20.clone(), v21, v22);
    let v25: i32 = 0i32;
    let v26: i32 = method13(v20, v25);
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("scoreboard "); } LIT.with(|lit| lit.clone()) }, v0));
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) }));
    let v29: Rc<str> = format_real_14(v26);
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines="); } LIT.with(|lit| lit.clone()) }));
    let v32: Rc<str> = format_real_14(v23);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = method16(v23, v24, v34);
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v35));
    v36
}
fn method4(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                let v3: Rc<str> = scoreboard_line_5(v1);
                println!("{}", v3);
                v0 = v2;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn scoreboard_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let mut v6: bool = match &*v5 {
                        UH0::UH0_Nil => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let mut v7: bool = v6 != true;
                    if v7 {
                        method4(v5.clone());
                        let v8: i32 = 0i32;
                        let v9: i32 = method13(v5, v8);
                        let v10: Rc<str> = format_real_14(v9);
                        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards scoreboard ok roots="); } LIT.with(|lit| lit.clone()) }, v10));
                        println!("{}", v11);
                        0i32
                    } else {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards --scoreboard needs at least one root directory"); } LIT.with(|lit| lit.clone()) };
                        eprintln!("{}", v12);
                        2i32
                    }
                }
                _ => {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards --scoreboard needs at least one root directory"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v14);
                    2i32
                }
            }
        }
        _ => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards --scoreboard needs at least one root directory"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v16);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        scoreboard_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_style_scoreboard_run() -> i32 {
    closure0()()
}
use kit_style_probes::SpiralShape_2617b4e49837 as Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
