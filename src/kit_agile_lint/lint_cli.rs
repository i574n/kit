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
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_1(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
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
fn state_text_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) }));
    let v5: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v4).unwrap_or_default().as_str());
    v5
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn census_printed_5(mut v0: &Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: i32) -> () {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 < v2;
        if v3 {
            let (mut v4, mut v5): (Rc<str>, i32) = v0.borrow()[v1 as usize].clone();
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile lint census check="); } LIT.with(|lit| lit.clone()) }, v4));
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) }));
            let v8: Rc<str> = format_real_6(v5);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            println!("{}", v9);
            let v10: i32 = v1.wrapping_add(1i32);
            v1 = v10;
            continue;
        }
        return ();
    }
}
fn printed_8(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32) -> () {
    loop {
        let v2: i32 = (v0.clone().borrow().len() as i32);
        let v3: bool = v1 < v2;
        if v3 {
            let (mut v4, mut v5, mut v6): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v6));
            println!("{}", v10);
            let v11: i32 = v1.wrapping_add(1i32);
            v1 = v11;
            continue;
        }
        return ();
    }
}
fn distinct_ids_9(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<str>, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3;
        } else {
            let (mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v9: i32 = v1.wrapping_add(1i32);
            let v10: bool = &*v6 == &*v2;
            let v12: i32 = if v10 {
                v3
            } else {
                let v11: i32 = v3.wrapping_add(1i32);
                v11
            };
            (v1, v2, v3) = (v9, v6, v12);
            continue;
        }
    }
}
fn lint_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_1(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            match Rc::unwrap_or_clone(v7) {
                                UH0::UH0_Cons(v8, v9) => {
                                    let v8: Rc<str> = v8;
                                    let v9: Rc<UH0> = v9;
                                    let v10: bool = &*v8 == "--census";
                                    if v10 {
                                        let v11: Rc<str> = state_text_4(v6);
                                        let v12: Rc<RefCell<Vec<(Rc<str>, i32)>>> = agile_quality_state::kit_agile_lint_census(&v11);
                                        let v13: i32 = 0i32;
                                        census_printed_5(&v12, v13);
                                        0i32
                                    } else {
                                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                        let v15: bool = v6.starts_with(&*v14);
                                        let v16: bool = v15 == false;
                                        if v16 {
                                            let v23: Rc<str> = match Rc::unwrap_or_clone(v9) {
                                                UH0::UH0_Cons(v17, v18) => {
                                                    let v17: Rc<str> = v17;
                                                    let v18: Rc<UH0> = v18;
                                                    let v19: bool = &*v8 == "--team";
                                                    if v19 {
                                                        v17
                                                    } else {
                                                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        v20
                                                    }
                                                }
                                                _ => {
                                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    v22
                                                }
                                            };
                                            let v24: Rc<str> = state_text_4(v6);
                                            let v25: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::kit_agile_lint_team_failures(&v24, &v23);
                                            let v26: i32 = 0i32;
                                            printed_8(&v25, v26);
                                            let v27: i32 = (v25.clone().borrow().len() as i32);
                                            let v28: i32 = 0i32;
                                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v30: i32 = 0i32;
                                            let v31: i32 = distinct_ids_9(&v25, v28, v29, v30);
                                            let v32: Rc<str> = format_real_6(v31);
                                            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile lint ok failing_items="); } LIT.with(|lit| lit.clone()) }, v32));
                                            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failing_checks="); } LIT.with(|lit| lit.clone()) }));
                                            let v35: Rc<str> = format_real_6(v27);
                                            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (report only; legacy items ratchet)"); } LIT.with(|lit| lit.clone()) }));
                                            println!("{}", v37);
                                            0i32
                                        } else {
                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile lint <root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
                                            eprintln!("{}", v38);
                                            2i32
                                        }
                                    }
                                }
                                _ => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                    let v42: bool = v6.starts_with(&*v41);
                                    let v43: bool = v42 == false;
                                    if v43 {
                                        let v44: Rc<str> = state_text_4(v6);
                                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v46: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_state::kit_agile_lint_team_failures(&v44, &v45);
                                        let v47: i32 = 0i32;
                                        printed_8(&v46, v47);
                                        let v48: i32 = (v46.clone().borrow().len() as i32);
                                        let v49: i32 = 0i32;
                                        let v50: i32 = 0i32;
                                        let v51: i32 = distinct_ids_9(&v46, v49, v45, v50);
                                        let v52: Rc<str> = format_real_6(v51);
                                        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile lint ok failing_items="); } LIT.with(|lit| lit.clone()) }, v52));
                                        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" failing_checks="); } LIT.with(|lit| lit.clone()) }));
                                        let v55: Rc<str> = format_real_6(v48);
                                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
                                        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (report only; legacy items ratchet)"); } LIT.with(|lit| lit.clone()) }));
                                        println!("{}", v57);
                                        0i32
                                    } else {
                                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile lint <root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
                                        eprintln!("{}", v58);
                                        2i32
                                    }
                                }
                            }
                        }
                        _ => {
                            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile lint <root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v61);
                            2i32
                        }
                    }
                }
                _ => {
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile lint <root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v63);
                    2i32
                }
            }
        }
        _ => {
            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile lint <root> [--census | --team <team>]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v65);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lint_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_lint_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
