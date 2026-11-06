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
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: Rc<str> }
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
fn flag_value_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let v6: bool = &*v2 == &*v1;
                        if v6 {
                            return v4;
                        } else {
                            v0 = v3;
                            continue;
                        }
                    }
                    _ => {
                        v0 = v3;
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v11;
            }
        }
    }
}
fn terms_of_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--team"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = flag_value_5(v1.clone(), v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = flag_value_5(v1, v4);
    (v0, v3, v5)
}
fn method6(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let v5: Rc<str> = (v0)[v4].clone();
            let v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method7(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH0> = method7(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: Rc<UH0> = method9(v3, v1);
            Rc::new(UH0::UH0_Cons(v2, v4))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn ids_where_8(mut v0: &Rc<RefCell<Vec<(Rc<str>, bool)>>>, mut v1: bool, mut v2: i32) -> Rc<UH0> {
    let v3: bool = v2 < 0i32;
    if v3 {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let (mut v5, mut v6): (Rc<str>, bool) = v0.borrow()[v2 as usize].clone();
        let v7: i32 = v2.wrapping_sub(1i32);
        let v8: Rc<UH0> = ids_where_8(v0, v1, v7);
        let v9: bool = v6 == v1;
        if v9 {
            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
            method9(v8, v11)
        } else {
            v8
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
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
fn method13(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_12(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method13(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method15(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn ready_line_14(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile claim "); } LIT.with(|lit| lit.clone()) }, v0));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <ID> --as "); } LIT.with(|lit| lit.clone()) }));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v2));
        let v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v10: i32 = 0i32;
        let v11: i32 = method15(v9.clone(), v3, v10);
        let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
        let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Your team's top ready light items: "); } LIT.with(|lit| lit.clone()) }, v15));
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(". Claim one explicitly ("); } LIT.with(|lit| lit.clone()) }));
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v8));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(") and start it now; while a run or train you started is pending, work another of these"); } LIT.with(|lit| lit.clone()) }));
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" instead of stopping."); } LIT.with(|lit| lit.clone()) }));
        v20
    }
}
fn parked_line_16(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: Rc<UH0>) -> Rc<str> {
    let v4: bool = match &*v3 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Paused P0/P1 items for team "); } LIT.with(|lit| lit.clone()) }, v1));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (re-decide each: resume as Planned, or keep Paused only with a reason AND a resume condition): "); } LIT.with(|lit| lit.clone()) }));
        let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v9: i32 = 0i32;
        let v10: i32 = method15(v8.clone(), v3, v9);
        let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
        let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from((*v12.borrow()).join(&*v13));
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v14));
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) }));
        v16
    }
}
fn gate_lines_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>, mut v4: Rc<UH0>) -> Rc<UH0> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile gate ok team="); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ready="); } LIT.with(|lit| lit.clone()) }));
    let v7: i32 = 0i32;
    let v8: i32 = method11(v3.clone(), v7);
    let v9: Rc<str> = format_real_12(v8);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v9));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" paused="); } LIT.with(|lit| lit.clone()) }));
    let v12: i32 = 0i32;
    let v13: i32 = method11(v4.clone(), v12);
    let v14: Rc<str> = format_real_12(v13);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v14));
    let v16: Rc<str> = ready_line_14(v0.clone(), &v1, v2.clone(), v3);
    let v17: Rc<str> = parked_line_16(&v0, v1, &v2, v4);
    let v18: bool = &*v15 != "";
    let v22: Rc<UH0> = if v18 {
        let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v15, v19))
    } else {
        { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
    };
    let v23: bool = &*v17 != "";
    let v25: Rc<UH0> = if v23 {
        Rc::new(UH0::UH0_Cons(v17, v22))
    } else {
        v22
    };
    let v26: bool = &*v16 != "";
    if v26 {
        Rc::new(UH0::UH0_Cons(v16, v25))
    } else {
        v25
    }
}
fn method17(mut v0: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v1, v2) => {
                let v1: Rc<str> = v1;
                let v2: Rc<UH0> = v2;
                println!("{}", v1);
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
fn gate_run_0() -> i32 {
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
                    match Rc::unwrap_or_clone(v5) {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6;
                            let v7: Rc<UH0> = v7;
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                            let v9: bool = v6.starts_with(&*v8);
                            let v10: bool = v9 == false;
                            if v10 {
                                let (mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, Rc<str>) = terms_of_4(v6.clone(), v7);
                                let v14: bool = &*v12 == "";
                                let v16: bool = if v14 {
                                    true
                                } else {
                                    let v15: bool = &*v13 == "";
                                    v15
                                };
                                if v16 {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                    eprintln!("{}", v17);
                                    2i32
                                } else {
                                    let v18: Rc<RefCell<Vec<(Rc<str>, bool)>>> = agile_team_work::kit_agile_gate_rows(&v6, &v12);
                                    let v19: i32 = (v18.clone().borrow().len() as i32);
                                    let v20: i32 = v19.wrapping_sub(1i32);
                                    let v21: Rc<RefCell<Vec<Rc<str>>>> = agile_team_lane::kit_agile_team_paused(&v6, &v12);
                                    let v22: Rc<Vec<Rc<str>>> = Rc::new((v21).borrow().clone());
                                    let v23: usize = (v22).len();
                                    let v24: i32 = ((v23) as i32);
                                    let v25: i32 = v24.wrapping_sub(1i32);
                                    let v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                                    let v27: Rc<UH1> = method6(v22, v25, v26);
                                    let v28: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                    let v29: Rc<UH0> = method7(v27, v28);
                                    let v30: bool = false;
                                    let v31: Rc<UH0> = ids_where_8(&v18, v30, v20);
                                    let v32: Rc<UH0> = gate_lines_10(v11, v12, v13, v31, v29);
                                    method17(v32);
                                    0i32
                                }
                            } else {
                                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                eprintln!("{}", v34);
                                2i32
                            }
                        }
                        _ => {
                            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                            eprintln!("{}", v36);
                            2i32
                        }
                    }
                }
                _ => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                    eprintln!("{}", v38);
                    2i32
                }
            }
        }
        _ => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit agile gate <root> --team <team> --as <lane>"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", v40);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        gate_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_gate_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
