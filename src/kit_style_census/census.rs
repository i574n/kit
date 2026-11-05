#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_PrintCensus,
    US0_WriteBaseline(Rc<str>),
    US0_MergeBaseline(Rc<str>),
    US0_CheckRatchet(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: Rc<str> }
struct Mut1 { l0: i32 }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, bool, i32, Rc<UH1>),
}
#[derive(Clone)]
enum US1 {
    US1_Held,
    US1_Regressed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
fn stdout_line_4(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn baseline_saved_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union census_row = CensusCount : string * string * i32 * i32\n\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    std::fs::write(&*v0, &*v6).unwrap();
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards baseline ok packages="); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v0));
    stdout_line_4(v12);
    0i32
}
fn method7(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn count_text_5(mut v0: i32) -> Rc<str> {
    format_real_6(v0)
}
fn stderr_line_8(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn method10(mut v0: i32, mut v1: Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v7, v9.clone()) {
                    let mut v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn census_report_2(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let v5: Rc<str> = count_text_5(v1);
    match &v0 {
        US0::US0_CheckRatchet(v29) => {
            let v29: Rc<str> = v29.clone();
            let v30: bool = v2 == 0i32;
            let v33: Rc<str> = if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v31
            } else {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                v32
            };
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v5));
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
            let v38: Rc<str> = count_text_5(v2);
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards ratchet "); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v40, v33)), v39));
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v42, v43)), v29));
            stdout_line_4(v45);
            if v30 {
                0i32
            } else {
                1i32
            }
        }
        US0::US0_MergeBaseline(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: bool = v2 > 0i32;
            if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v5));
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                let v16: Rc<str> = count_text_5(v2);
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards merge refused; a baseline only shrinks"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v10));
                stderr_line_8(v22);
                1i32
            } else {
                let v23: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
                let v24: Rc<RefCell<Vec<Rc<str>>>> = method9(v4, v23);
                let v25: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
                let v26: Rc<str> = kit_style_baseline::kit_style_baseline_merged(&v25, v24, &v3);
                baseline_saved_3(v10, v26, v5.clone())
            }
        }
        US0::US0_PrintCensus => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards census ok packages="); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
            stdout_line_4(v7);
            0i32
        }
        US0::US0_WriteBaseline(v8) => {
            let mut v8: Rc<str> = v8.clone();
            baseline_saved_3(v8, v3, v5)
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2.clone()]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v7, v9.clone()) {
                    let mut v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13.clone()
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15.clone()
                    };
                    v8.borrow_mut()[v11 as usize] = v16.clone();
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn list_of_base_12(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v1, v2) = (v4, v6.clone());
            continue;
        }
    }
}
fn counts_from_15(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v1 as usize].clone();
            let mut v9: bool = kit_style_probes::kit_style_probe_falls(&v6);
            let mut v10: i32 = v1.wrapping_add(1i32);
            let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6.clone(), v9, v7, v2.clone()));
            (v1, v2) = (v10, v11.clone());
            continue;
        }
    }
}
fn census_of_14(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<UH1>) {
    let v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_measure::kit_style_measured(&v0);
    let (mut v2, mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, i32, i32) = v1.borrow()[0i32 as usize].clone();
    let v6: i32 = 0i32;
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH1> = counts_from_15(v1, v6, v7);
    (v2, v5, v8)
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: bool = *v3;
                let v4: i32 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v2));
                let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) };
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                let mut v10: Rc<str> = count_text_5(v4);
                let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v1));
                (v0, v1) = (v5.clone(), v12.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn counts_text_16(mut v0: Rc<UH1>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method17(v0, v1)
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32)>>>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: bool = *v3;
                let v4: i32 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![(v2.clone(), v4)]));
                let v7: i32 = (v1.clone().borrow().len() as i32);
                let mut v8: i32 = (v6.clone().borrow().len() as i32);
                let mut v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v9, v11.clone()) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, i32) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, i32) = v1.borrow()[v13 as usize].clone();
                        (v15.clone(), v16)
                    } else {
                        let v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, i32) = v6.borrow()[v17 as usize].clone();
                        (v18.clone(), v19)
                    };
                    v10.borrow_mut()[v13 as usize] = (v20.clone(), v21);
                    let v22: i32 = v13.wrapping_add(1i32);
                    v11.borrow_mut().l0 = v22;
                    ()
                };
                (v0, v1) = (v5.clone(), v10.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn readings_of_19(mut v0: Rc<UH1>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    let v1: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<(Rc<str>, i32)>>> = method20(v0, v1);
    v2
}
fn rows_after_18(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<RefCell<Vec<(Rc<str>, i32)>>> = readings_of_19(v2);
    let v5: Rc<str> = kit_style_baseline::kit_style_baseline_rendered(&v0, v1, v4, &v3);
    v5
}
fn entry_from_21(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> (Rc<str>, Rc<str>, i32, i32) {
    let v4: i32 = (v0.clone().borrow().len() as i32);
    let v5: bool = v3 >= v4;
    let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, i32, i32) = if v5 {
        (v1.clone(), v2.clone(), -1i32, 0i32)
    } else {
        let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v3 as usize].clone();
        let mut v10: bool = &*v6 == &*v1;
        let v12: bool = if v10 {
            let mut v11: bool = &*v7 == &*v2;
            v11
        } else {
            false
        };
        let (mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, i32, i32) = if v12 {
            (v6, v7, v8, v9)
        } else {
            let v13: i32 = v3.wrapping_add(1i32);
            let (mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v0, v1, v2, v13);
            (v14, v15, v16, v17)
        };
        (v18, v19, v20, v21)
    };
    (v22, v23, v24, v25)
}
fn method25(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3.clone();
                let v4: bool = *v4;
                let v5: i32 = *v5;
                let v6: Rc<UH1> = v6.clone();
                let v7: bool = &*v3 == &*v0;
                let mut v8: i32 = if v7 {
                    v5
                } else {
                    v2
                };
                (v1, v2) = (v6.clone(), v8);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn count_of_24(mut v0: Rc<UH1>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = 0i32;
    method25(v1, v0, v2)
}
fn method27(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: bool = *v3;
                let v4: i32 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let v7: i32 = if v3 {
                    let v6: i32 = v1.wrapping_add(v4);
                    v6
                } else {
                    v1
                };
                (v0, v1) = (v5.clone(), v7);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn feature_total_26(mut v0: Rc<UH1>) -> i32 {
    let v1: i32 = 0i32;
    method27(v0, v1)
}
fn new_package_problem_23(mut v0: Rc<UH1>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = count_of_24(v0.clone(), v1);
    let v3: bool = v2 > 0i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package carries rust_global"); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
        let v6: i32 = count_of_24(v0.clone(), v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-statement"); } LIT.with(|lit| lit.clone()) };
        let mut v8: i32 = count_of_24(v0.clone(), v7);
        let mut v9: i32 = v6.wrapping_add(v8);
        let mut v10: bool = v9 > 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package has host strings over 200 characters, multi-line or with statements; name one-expression primitives in a boundary package"); } LIT.with(|lit| lit.clone()) };
            v11
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
            let v13: i32 = count_of_24(v0.clone(), v12);
            let v14: bool = v13 > 0i32;
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package encodes rows with tab or record separators"); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
                let v17: i32 = count_of_24(v0.clone(), v16);
                let v18: bool = v17 >= 2i32;
                let v21: bool = if v18 {
                    let v19: i32 = feature_total_26(v0);
                    let v20: bool = v19 == 0i32;
                    v20
                } else {
                    false
                };
                if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a new package declares types but uses none of the typefeatureset (GADT cases, existentials, foralls, indexed unions, prototypes)"); } LIT.with(|lit| lit.clone()) };
                    v22
                } else {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23
                }
            }
        }
    }
}
fn appended_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v3)), v1));
        v5
    }
}
fn method29(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH1::UH1_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let v6: bool = *v6;
                let v7: i32 = *v7;
                let mut v8: Rc<UH1> = v8.clone();
                let mut v9: i32 = 0i32;
                let (mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v0.clone(), v1.clone(), v5.clone(), v9);
                let v14: Rc<str> = count_text_5(v12);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                let v17: Rc<str> = count_text_5(v13);
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                let v21: Rc<str> = count_text_5(v7);
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v15));
                let v23: Rc<str> = count_text_5(v2);
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v25)), v5)), v25)), v20)), v24));
                let v31: bool = v12 >= 0i32;
                let v33: bool = if v31 {
                    let v32: bool = kit_style_probes::kit_style_probe_regressed(&v5, v12, v13, v7, v2);
                    v32
                } else {
                    false
                };
                let v35: Rc<str> = if v33 {
                    appended_30(v4.clone(), v30.clone())
                } else {
                    v4.clone()
                };
                (v3, v4) = (v8.clone(), v35.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v4.clone();
            }
        }
    }
}
fn probes_compared_28(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method29(v1, v2, v3, v0, v4)
}
fn census_verdict_22(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> US1 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = 0i32;
    let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v3.clone(), v0.clone(), v4, v5);
    let mut v10: bool = v8 < 0i32;
    if v10 {
        let mut v11: Rc<str> = new_package_problem_23(v2.clone());
        let v12: bool = &*v11 == "";
        if v12 {
            US1::US1_Held
        } else {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v14)), v11));
            US1::US1_Regressed(v16)
        }
    } else {
        let v19: Rc<str> = probes_compared_28(v2, v3, v0, v1);
        let v20: bool = &*v19 == "";
        if v20 {
            US1::US1_Held
        } else {
            US1::US1_Regressed(v19)
        }
    }
}
fn census_walk_13(mut v0: US0, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v2: Rc<UH0>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<UH0>) -> (i32, i32, Rc<str>, Rc<UH0>) {
    let (mut v109, mut v110, mut v111, mut v112): (i32, i32, Rc<str>, Rc<UH0>) = match &*v2 {
        UH0::UH0_Nil => {
            (v3, v4, v5.clone(), v6.clone())
        }
        _ => {
            let (mut v105, mut v106, mut v107, mut v108): (i32, i32, Rc<str>, Rc<UH0>) = match &*v2 {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7.clone();
                    let mut v8: Rc<UH0> = v8.clone();
                    let (mut v9, mut v10, mut v11): (Rc<str>, i32, Rc<UH1>) = census_of_14(v7);
                    let v12: i32 = v3.wrapping_add(1i32);
                    match &v0 {
                        US0::US0_PrintCensus => {
                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines="); } LIT.with(|lit| lit.clone()) };
                            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v13));
                            let v15: Rc<str> = count_text_5(v10);
                            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                            let v17: Rc<str> = counts_text_16(v11.clone());
                            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                            stdout_line_4(v18);
                            census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v4, v5.clone(), v6.clone())
                        }
                        _ => {
                            let (mut v97, mut v98, mut v99, mut v100): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                US0::US0_WriteBaseline(v23) => {
                                    let v23: Rc<str> = v23.clone();
                                    let v24: Rc<str> = rows_after_18(v9.clone(), v10, v11.clone(), v5.clone());
                                    census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v4, v24, v6.clone())
                                }
                                _ => {
                                    let (mut v93, mut v94, mut v95, mut v96): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                        US0::US0_MergeBaseline(v29) => {
                                            let v29: Rc<str> = v29.clone();
                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
                                            let v31: i32 = 0i32;
                                            let (mut v32, mut v33, mut v34, mut v35): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v1.clone(), v9.clone(), v30, v31);
                                            let v36: bool = v34 < 0i32;
                                            if v36 {
                                                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  a merge never adds a package that is not in the baseline"); } LIT.with(|lit| lit.clone()) };
                                                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v37));
                                                stdout_line_4(v38);
                                                let v39: i32 = v4.wrapping_add(1i32);
                                                let (mut v40, mut v41, mut v42, mut v43): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v39, v5.clone(), v6.clone());
                                                (v40, v41, v42, v43)
                                            } else {
                                                let v44: US1 = census_verdict_22(v9.clone(), v10, v11.clone(), v1.clone());
                                                let (mut v61, mut v62, mut v63, mut v64): (i32, i32, Rc<str>, Rc<UH0>) = match &v44 {
                                                    US1::US1_Held => {
                                                        let v45: Rc<str> = rows_after_18(v9.clone(), v10, v11.clone(), v5.clone());
                                                        let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v6.clone()));
                                                        census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v4, v45, v46)
                                                    }
                                                    _ => {
                                                        let (mut v57, mut v58, mut v59, mut v60): (i32, i32, Rc<str>, Rc<UH0>) = match &v44 {
                                                            US1::US1_Regressed(v51) => {
                                                                let v51: Rc<str> = v51.clone();
                                                                stdout_line_4(v51);
                                                                let v52: i32 = v4.wrapping_add(1i32);
                                                                census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v52, v5.clone(), v6.clone())
                                                            }
                                                            _ => unreachable!(),
                                                        };
                                                        (v57, v58, v59, v60)
                                                    }
                                                };
                                                (v61, v62, v63, v64)
                                            }
                                        }
                                        _ => {
                                            let (mut v89, mut v90, mut v91, mut v92): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                                US0::US0_CheckRatchet(v69) => {
                                                    let v69: Rc<str> = v69.clone();
                                                    let v70: US1 = census_verdict_22(v9, v10, v11, v1.clone());
                                                    match &v70 {
                                                        US1::US1_Held => {
                                                            census_walk_13(v0.clone(), v1.clone(), v8.clone(), v12, v4, v5.clone(), v6.clone())
                                                        }
                                                        _ => {
                                                            let (mut v81, mut v82, mut v83, mut v84): (i32, i32, Rc<str>, Rc<UH0>) = match &v70 {
                                                                US1::US1_Regressed(v75) => {
                                                                    let v75: Rc<str> = v75.clone();
                                                                    stdout_line_4(v75);
                                                                    let v76: i32 = v4.wrapping_add(1i32);
                                                                    census_walk_13(v0, v1, v8, v12, v76, v5, v6)
                                                                }
                                                                _ => unreachable!(),
                                                            };
                                                            (v81, v82, v83, v84)
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v89, v90, v91, v92)
                                        }
                                    };
                                    (v93, v94, v95, v96)
                                }
                            };
                            (v97, v98, v99, v100)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v105, v106, v107, v108)
        }
    };
    (v109, v110, v111, v112)
}
fn census_started_1(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = method11(v1.clone(), v2.clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = kit_style_measure::kit_style_packages_under(v3);
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: i32 = v5.wrapping_sub(1i32);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = list_of_base_12(v4.clone(), v6, v7.clone());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![]));
    let v17: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = match &v0 {
        US0::US0_CheckRatchet(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
            let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_baseline::kit_style_baseline_entries(&v11);
            v12.clone()
        }
        US0::US0_MergeBaseline(v13) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
            let v15: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_baseline::kit_style_baseline_entries(&v14);
            v15.clone()
        }
        _ => {
            v9.clone()
        }
    };
    let v18: bool = match &*v8 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards found no Spiral packages; "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: standards --census <dir>... | standards --baseline <file.spi> <dir>... | standards --baseline-merge <file.spi> <dir>... (replaces only the walked packages' rows) | standards --ratchet <file.spi> <dir>... (a directory without .spi files stands for its package subdirectories)"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards found no Spiral packages; usage: standards --census <dir>... | standards --baseline <file.spi> <dir>... | standards --baseline-merge <file.spi> <dir>... (replaces only the walked packages' rows) | standards --ratchet <file.spi> <dir>... (a directory without .spi files stands for its package subdirectories)"); } LIT.with(|lit| lit.clone()) };
        stderr_line_8(v21.clone());
        2i32
    } else {
        let v22: i32 = 0i32;
        let v23: i32 = 0i32;
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v26, mut v27, mut v28, mut v29): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_13(v0.clone(), v17.clone(), v8.clone(), v22, v23, v24.clone(), v25.clone());
        census_report_2(v0.clone(), v26, v27, v28.clone(), v29.clone())
    }
}
fn command_line_argument_count_32() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_33(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_31(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_32();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_33(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match &v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_31(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn mode_of_34(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: bool = &*v0 == "--baseline";
    if v2 {
        US0::US0_WriteBaseline(v1.clone())
    } else {
        let v4: bool = &*v0 == "--baseline-merge";
        if v4 {
            US0::US0_MergeBaseline(v1.clone())
        } else {
            let v6: bool = &*v0 == "--ratchet";
            if v6 {
                US0::US0_CheckRatchet(v1)
            } else {
                US0::US0_PrintCensus
            }
        }
    }
}
fn census_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_31(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: Rc<UH0> = v5.clone();
                    let v6: bool = &*v4 == "--census";
                    if v6 {
                        let v7: US0 = US0::US0_PrintCensus;
                        census_started_1(v7, v5.clone())
                    } else {
                        match &*v5 {
                            UH0::UH0_Cons(v9, v10) => {
                                let mut v9: Rc<str> = v9.clone();
                                let mut v10: Rc<UH0> = v10.clone();
                                let mut v11: US0 = mode_of_34(v4, v9.clone());
                                let v12: bool = match &v11 {
                                    US0::US0_PrintCensus => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let v14: Rc<UH0> = if v12 {
                                    Rc::new(UH0::UH0_Cons(v9, v10.clone()))
                                } else {
                                    v10
                                };
                                census_started_1(v11, v14)
                            }
                            _ => {
                                let v16: US0 = US0::US0_PrintCensus;
                                let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                census_started_1(v16, v17)
                            }
                        }
                    }
                }
                _ => {
                    let v21: US0 = US0::US0_PrintCensus;
                    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    census_started_1(v21, v22)
                }
            }
        }
        _ => {
            let v25: US0 = US0::US0_PrintCensus;
            let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            census_started_1(v25, v26)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        census_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_census_run() -> i32 {
    closure0()()
}
