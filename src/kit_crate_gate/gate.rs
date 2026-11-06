#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, i32, Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_CrateWithin,
    US0_CrateOver,
    US0_CrateUnbuilt,
}
#[derive(Clone)]
enum US1 {
    US1_GatePassed,
    US1_GateFailedOver(i32),
    US1_GateFailedMissing(i32),
    US1_GateFailedVacuous,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_FlagEnforce,
    US3_FlagLimit,
}
fn method3(mut v0: Option<char>) -> Option<char> {
    v0
}
fn closure1() -> Rc<dyn Fn((char)) -> u32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((char)) -> u32> = Rc::new(move |mut v0: (char)| -> u32 {
        let mut v1: char = (v0);
        let v2: u32 = u32::from(v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn digits_value_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: usize = ((v1) as usize);
            let mut v6: Option<char> = (&*v0)[v5..].chars().next();
            let v7: Option<char> = method3(v6);
            let mut v8: Rc<dyn Fn((char)) -> u32> = closure1();
            let mut v9: Option<u32> = v7.map(|x| v8(x));
            let v11: u32 = v9.unwrap_or(0u32);
            let v12: bool = 48u32 <= v11;
            let v14: bool = if v12 {
                let v13: bool = v11 <= 57u32;
                v13
            } else {
                false
            };
            if v14 {
                let v15: i32 = v1.wrapping_add(1i32);
                let v16: i32 = v2.wrapping_mul(10i32);
                let v17: u32 = v11.wrapping_sub(48u32);
                let v18: i32 = (v17 as i32);
                let v19: i32 = v16.wrapping_add(v18);
                (v1, v2) = (v15, v19);
                continue;
            } else {
                return -1i32;
            }
        }
    }
}
fn directory_exists_4(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn rows_from_6(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6, v7, v8, v2));
            (v1, v2) = (v4, v9);
            continue;
        }
    }
}
fn rows_listed_5(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>>) -> Rc<UH0> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    rows_from_6(v0, v2, v3)
}
fn method10(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_9(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method10(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn verdict_of_12(mut v0: i32, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>) -> US0 {
    let v5: bool = v3 < 0i32;
    if v5 {
        US0::US0_CrateUnbuilt
    } else {
        let v7: bool = v3 > v0;
        if v7 {
            US0::US0_CrateOver
        } else {
            US0::US0_CrateWithin
        }
    }
}
fn verdict_text_11(mut v0: i32, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>) -> Rc<str> {
    let v5: US0 = verdict_of_12(v0, v1, v2, v3, v4);
    match v5 {
        US0::US0_CrateOver => {
            let v7: Rc<str> = format_real_9(v0);
            let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("OVER "); } LIT.with(|lit| lit.clone()) }, v7));
            v8
        }
        US0::US0_CrateUnbuilt => {
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no generated output yet"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US0::US0_CrateWithin => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn measure_line_8(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gate / "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v2.clone()));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let mut v9: Rc<str> = format_real_9(v3);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = verdict_text_11(v0, &v1, &v2, v3, &v4);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v4));
    v14
}
fn lines_printed_7(mut v0: i32, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: Rc<str> = measure_line_8(v0, v2, v3, v4, v5);
                println!("{}", v7);
                v1 = v6;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method14(mut v0: i32, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<UH0> = v7;
                let mut v8: US0 = verdict_of_12(v0, &v3, &v4, v5, &v6);
                let v10: i32 = match v8 {
                    US0::US0_CrateOver => {
                        let mut v9: i32 = v2.wrapping_add(1i32);
                        v9
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v7, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn over_count_13(mut v0: i32, mut v1: Rc<UH0>) -> i32 {
    let v2: i32 = 0i32;
    method14(v0, v1, v2)
}
fn method15(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: i32 = v4;
                let v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method18(mut v0: i32, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: i32 = v5;
                let mut v6: Rc<str> = v6;
                let v7: Rc<UH0> = v7;
                let mut v8: US0 = verdict_of_12(v0, &v3, &v4, v5, &v6);
                let v10: i32 = match v8 {
                    US0::US0_CrateUnbuilt => {
                        let mut v9: i32 = v2.wrapping_add(1i32);
                        v9
                    }
                    _ => {
                        v2
                    }
                };
                (v1, v2) = (v7, v10);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn missing_count_17(mut v0: i32, mut v1: Rc<UH0>) -> i32 {
    let v2: i32 = 0i32;
    method18(v0, v1, v2)
}
fn gate_outcome_of_16(mut v0: bool, mut v1: i32, mut v2: Rc<UH0>) -> US1 {
    let v3: i32 = over_count_13(v1, v2.clone());
    let v4: i32 = missing_count_17(v1, v2.clone());
    let mut v8: bool = if v0 {
        let v5: i32 = 0i32;
        let mut v6: i32 = method15(v2, v5);
        let v7: bool = v6 == 0i32;
        v7
    } else {
        false
    };
    if v8 {
        US1::US1_GateFailedVacuous
    } else {
        let v11: bool = if v0 {
            let v10: bool = v3 > 0i32;
            v10
        } else {
            false
        };
        if v11 {
            US1::US1_GateFailedOver(v3)
        } else {
            let v14: bool = if v0 {
                let v13: bool = v4 > 0i32;
                v13
            } else {
                false
            };
            if v14 {
                US1::US1_GateFailedMissing(v4)
            } else {
                US1::US1_GatePassed
            }
        }
    }
}
fn outcome_text_19(mut v0: US1, mut v1: Rc<str>) -> (i32, Rc<str>) {
    match v0 {
        US1::US1_GateFailedMissing(v8) => {
            let mut v8: i32 = v8;
            let mut v9: Rc<str> = format_real_9(v8);
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate refused: "); } LIT.with(|lit| lit.clone()) }, v9));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crate(s) have no generated output, so generation did not run or failed;"); } LIT.with(|lit| lit.clone()) }));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v1));
            (1i32, v12)
        }
        US1::US1_GateFailedOver(v3) => {
            let v3: i32 = v3;
            let v4: Rc<str> = format_real_9(v3);
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate refused: "); } LIT.with(|lit| lit.clone()) }, v4));
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crate(s) over the limit;"); } LIT.with(|lit| lit.clone()) }));
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v1));
            (1i32, v7)
        }
        US1::US1_GateFailedVacuous => {
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate refused: no crate with [package.metadata.spiral] under the root, so nothing was measured;"); } LIT.with(|lit| lit.clone()) }, v1));
            (1i32, v13)
        }
        US1::US1_GatePassed => {
            let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit crate gate ok"); } LIT.with(|lit| lit.clone()) }, v1));
            (0i32, v2)
        }
    }
}
fn gate_checked_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>) -> i32 {
    let v3: bool = &*v2 == "";
    let v7: i32 = if v3 {
        1000i32
    } else {
        let v4: i32 = 0i32;
        let v5: i32 = 0i32;
        digits_value_2(v2.clone(), v4, v5)
    };
    let mut v8: bool = &*v0 == "";
    let v11: bool = if v8 {
        true
    } else {
        let mut v9: bool = directory_exists_4(v0.clone());
        let v10: bool = v9 == false;
        v10
    };
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: crate usage: crate gate <root> [--enforce] [--limit n] (generated output of every Cargo.toml with [package.metadata.spiral] under root)"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v12);
        2i32
    } else {
        let v13: bool = v7 <= 0i32;
        if v13 {
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: crate gate: --limit expects a positive whole number, got "); } LIT.with(|lit| lit.clone()) }, v2));
            println!("{}", v14);
            2i32
        } else {
            let v15: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, Rc<str>)>>> = kit_crate_walk::kit_crate_measures(&v0);
            let v16: Rc<UH0> = rows_listed_5(&v15);
            lines_printed_7(v7, v16.clone());
            let v17: i32 = over_count_13(v7, v16.clone());
            let v18: i32 = 0i32;
            let v19: i32 = method15(v16.clone(), v18);
            let v20: Rc<str> = format_real_9(v19);
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" crates="); } LIT.with(|lit| lit.clone()) }, v20));
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" over="); } LIT.with(|lit| lit.clone()) }));
            let v23: Rc<str> = format_real_9(v17);
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" limit="); } LIT.with(|lit| lit.clone()) }));
            let v26: Rc<str> = format_real_9(v7);
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" root="); } LIT.with(|lit| lit.clone()) }));
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v0));
            let v30: US1 = gate_outcome_of_16(v1, v7, v16);
            let (mut v31, mut v32): (i32, Rc<str>) = outcome_text_19(v30, v29);
            println!("{}", v32);
            v31
        }
    }
}
fn command_line_argument_count_21() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_20(mut v0: i32) -> Rc<UH1> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_21();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_22(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH1> = arguments_from_20(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn flag_word_24(mut v0: US3) -> Rc<str> {
    let v3: Rc<str> = match v0 {
        US3::US3_FlagEnforce => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("enforce"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US3::US3_FlagLimit => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("limit"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) }, v3));
    v4
}
fn gate_flags_read_23(mut v0: Rc<UH1>, mut v1: bool, mut v2: Rc<str>) -> (bool, Rc<str>) {
    let (mut v30, mut v31): (bool, Rc<str>) = match &*v0 {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH1> = v4.clone();
            let v5: US3 = US3::US3_FlagEnforce;
            let mut v6: Rc<str> = flag_word_24(v5);
            let v7: bool = &*v3 == &*v6;
            if v7 {
                let mut v8: bool = true;
                let (mut v9, mut v10): (bool, Rc<str>) = gate_flags_read_23(v4, v8, v2);
                (v9, v10)
            } else {
                let (mut v24, mut v25): (bool, Rc<str>) = match &*v4 {
                    UH1::UH1_Cons(v11, v12) => {
                        let v11: Rc<str> = v11.clone();
                        let v12: Rc<UH1> = v12.clone();
                        let v13: US3 = US3::US3_FlagLimit;
                        let v14: Rc<str> = flag_word_24(v13);
                        let v15: bool = &*v3 == &*v14;
                        if v15 {
                            let (mut v16, mut v17): (bool, Rc<str>) = gate_flags_read_23(v12, v1, v11);
                            (v16, v17)
                        } else {
                            let (mut v18, mut v19): (bool, Rc<str>) = gate_flags_read_23(v4, v1, v2);
                            (v18, v19)
                        }
                    }
                    _ => {
                        let (mut v22, mut v23): (bool, Rc<str>) = gate_flags_read_23(v4, v1, v2);
                        (v22, v23)
                    }
                };
                (v24, v25)
            }
        }
        _ => {
            let (mut v28, mut v29): (bool, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH1::UH1_Nil => {
                    (v1, v2)
                }
                _ => unreachable!(),
            };
            (v28, v29)
        }
    };
    (v30, v31)
}
fn gate_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH1> = arguments_from_20(v0);
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH1::UH1_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH1> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH1::UH1_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6;
                            let v7: Rc<UH1> = v7;
                            let mut v8: bool = false;
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let (mut v10, mut v11): (bool, Rc<str>) = gate_flags_read_23(v7, v8, v9);
                            gate_checked_1(v6, v10, v11)
                        }
                        _ => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v14: bool = false;
                            gate_checked_1(v13.clone(), v14, v13)
                        }
                    }
                }
                _ => {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = false;
                    gate_checked_1(v17.clone(), v18, v17)
                }
            }
        }
        _ => {
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v22: bool = false;
            gate_checked_1(v21.clone(), v22, v21)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_crate_gate_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
