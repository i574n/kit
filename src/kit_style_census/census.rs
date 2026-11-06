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
    US2_FlagCensus,
    US2_FlagBaseline,
    US2_FlagBaselineMerge,
    US2_FlagRatchet,
    US2_FlagScoreboard,
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US2),
    US4_None,
}
fn stdout_line_4(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn baseline_saved_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union census_row = CensusCount : string * string * i32 * i32\n\n"); } LIT.with(|lit| lit.clone()) }, v1));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    std::fs::write(&*v0, &*v4).unwrap();
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards baseline ok packages="); } LIT.with(|lit| lit.clone()) }, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) }));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
    stdout_line_4(v8);
    0i32
}
fn method7(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(&v2, v3);
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
fn method10(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn census_report_2(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<UH0>) -> i32 {
    let v5: Rc<str> = count_text_5(v1);
    match v0 {
        US0::US0_CheckRatchet(v24) => {
            let v24: Rc<str> = v24;
            let v25: bool = v2 == 0i32;
            let v28: Rc<str> = if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v26
            } else {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                v27
            };
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) }, v5));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) }));
            let v31: Rc<str> = count_text_5(v2);
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards ratchet "); } LIT.with(|lit| lit.clone()) }, v28));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) }));
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v24));
            stdout_line_4(v36);
            if v25 {
                0i32
            } else {
                1i32
            }
        }
        US0::US0_MergeBaseline(v9) => {
            let mut v9: Rc<str> = v9;
            let mut v10: bool = v2 > 0i32;
            if v10 {
                let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" packages="); } LIT.with(|lit| lit.clone()) }, v5));
                let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" regressions="); } LIT.with(|lit| lit.clone()) }));
                let v13: Rc<str> = count_text_5(v2);
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards merge refused; a baseline only shrinks"); } LIT.with(|lit| lit.clone()) }, v14));
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" baseline="); } LIT.with(|lit| lit.clone()) }));
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
                stderr_line_8(v17);
                1i32
            } else {
                let v18: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
                let v19: Rc<RefCell<Vec<Rc<str>>>> = method9(v4, v18);
                let v20: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v9).unwrap_or_default().as_str());
                let v21: Rc<str> = kit_style_baseline::kit_style_baseline_merged(&v20, v19, &v3);
                baseline_saved_3(v9, v21, v5)
            }
        }
        US0::US0_PrintCensus => {
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit standards census ok packages="); } LIT.with(|lit| lit.clone()) }, v5));
            stdout_line_4(v6);
            0i32
        }
        US0::US0_WriteBaseline(v7) => {
            let v7: Rc<str> = v7;
            baseline_saved_3(v7, v3, v5)
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v2]));
                let v5: i32 = (v1.clone().borrow().len() as i32);
                let v6: i32 = (v4.clone().borrow().len() as i32);
                let v7: i32 = v5.wrapping_add(v6);
                let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![<Rc<str>>::default(); v7 as usize]));
                let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v7, &v9) {
                    let mut v11: i32 = v9.borrow().l0;
                    let v12: bool = v11 < v5;
                    let v16: Rc<str> = if v12 {
                        let v13: Rc<str> = v1.borrow()[v11 as usize].clone();
                        v13
                    } else {
                        let v14: i32 = v11.wrapping_sub(v5);
                        let v15: Rc<str> = v4.borrow()[v14 as usize].clone();
                        v15
                    };
                    v8.borrow_mut()[v11 as usize] = v16;
                    let v17: i32 = v11.wrapping_add(1i32);
                    v9.borrow_mut().l0 = v17;
                    ()
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn list_of_base_12(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v4, v6);
            continue;
        }
    }
}
fn counts_from_15(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, i32, i32) = v0.borrow()[v1 as usize].clone();
            let mut v9: bool = kit_style_probes::kit_style_probe_falls(&v6);
            let mut v10: i32 = v1.wrapping_add(1i32);
            let mut v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6, v9, v7, v2));
            (v1, v2) = (v10, v11);
            continue;
        }
    }
}
fn census_of_14(mut v0: Rc<str>) -> (Rc<str>, i32, Rc<UH1>) {
    let v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_measure::kit_style_measured(&v0);
    let (mut v2, mut v3, mut v4, mut v5): (Rc<str>, Rc<str>, i32, i32) = v1.borrow()[0i32 as usize].clone();
    let v6: i32 = 0i32;
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH1> = counts_from_15(&v1, v6, v7);
    (v2, v5, v8)
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: bool = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v2));
                let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) }));
                let mut v8: Rc<str> = count_text_5(v4);
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v1));
                (v0, v1) = (v5, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
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
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: bool = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![(v2, v4)]));
                let v7: i32 = (v1.clone().borrow().len() as i32);
                let mut v8: i32 = (v6.clone().borrow().len() as i32);
                let mut v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32 }));
                while method10(v9, &v11) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, i32) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, i32) = v1.borrow()[v13 as usize].clone();
                        (v15, v16)
                    } else {
                        let v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, i32) = v6.borrow()[v17 as usize].clone();
                        (v18, v19)
                    };
                    v10.borrow_mut()[v13 as usize] = (v20, v21);
                    let v22: i32 = v13.wrapping_add(1i32);
                    v11.borrow_mut().l0 = v22;
                    ()
                };
                (v0, v1) = (v5, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
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
fn entry_from_21(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32) -> (Rc<str>, Rc<str>, i32, i32) {
    let v4: i32 = (v0.clone().borrow().len() as i32);
    let v5: bool = v3 >= v4;
    let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<str>, i32, i32) = if v5 {
        (v1, v2, -1i32, 0i32)
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
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: i32 = v5;
                let v6: Rc<UH1> = v6;
                let v7: bool = &*v3 == &*v0;
                let mut v8: i32 = if v7 {
                    v5
                } else {
                    v2
                };
                (v1, v2) = (v6, v8);
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
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: bool = v3;
                let v4: i32 = v4;
                let v5: Rc<UH1> = v5;
                let v7: i32 = if v3 {
                    let v6: i32 = v1.wrapping_add(v4);
                    v6
                } else {
                    v1
                };
                (v0, v1) = (v5, v7);
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
        v1
    } else {
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
        v4
    }
}
fn method29(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<UH1>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            UH1::UH1_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5;
                let v6: bool = v6;
                let v7: i32 = v7;
                let mut v8: Rc<UH1> = v8;
                let mut v9: i32 = 0i32;
                let (mut v10, mut v11, mut v12, mut v13): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v0, v1.clone(), v5.clone(), v9);
                let v14: Rc<str> = count_text_5(v12);
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let v16: Rc<str> = count_text_5(v13);
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) }));
                let v19: Rc<str> = count_text_5(v7);
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
                let v21: Rc<str> = count_text_5(v2);
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v5.clone()));
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v18));
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v22));
                let v28: bool = v12 >= 0i32;
                let v30: bool = if v28 {
                    let v29: bool = kit_style_probes::kit_style_probe_regressed(&v5, v12, v13, v7, v2);
                    v29
                } else {
                    false
                };
                let v32: Rc<str> = if v30 {
                    appended_30(v4, v27)
                } else {
                    v4
                };
                (v3, v4) = (v8, v32);
                continue;
            }
            UH1::UH1_Nil => {
                return v4;
            }
        }
    }
}
fn probes_compared_28(mut v0: Rc<UH1>, mut v1: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v2: Rc<str>, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    method29(v1, v2, v3, v0, v4)
}
fn census_verdict_22(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>, mut v3: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>) -> US1 {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = 0i32;
    let (mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v3, v0.clone(), v4, v5);
    let mut v10: bool = v8 < 0i32;
    if v10 {
        let mut v11: Rc<str> = new_package_problem_23(v2);
        let v12: bool = &*v11 == "";
        if v12 {
            US1::US1_Held
        } else {
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  "); } LIT.with(|lit| lit.clone()) }));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v11));
            US1::US1_Regressed(v15)
        }
    } else {
        let v18: Rc<str> = probes_compared_28(v2, v3, v0, v1);
        let v19: bool = &*v18 == "";
        if v19 {
            US1::US1_Held
        } else {
            US1::US1_Regressed(v18)
        }
    }
}
fn census_walk_13(mut v0: US0, mut v1: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>>, mut v2: Rc<UH0>, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<UH0>) -> (i32, i32, Rc<str>, Rc<UH0>) {
    let (mut v107, mut v108, mut v109, mut v110): (i32, i32, Rc<str>, Rc<UH0>) = match &*v2 {
        UH0::UH0_Nil => {
            (v3, v4, v5, v6)
        }
        _ => {
            let (mut v103, mut v104, mut v105, mut v106): (i32, i32, Rc<str>, Rc<UH0>) = match Rc::unwrap_or_clone(v2) {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7;
                    let mut v8: Rc<UH0> = v8;
                    let (mut v9, mut v10, mut v11): (Rc<str>, i32, Rc<UH1>) = census_of_14(v7);
                    let v12: i32 = v3.wrapping_add(1i32);
                    match &v0 {
                        US0::US0_PrintCensus => {
                            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines="); } LIT.with(|lit| lit.clone()) }));
                            let v14: Rc<str> = count_text_5(v10);
                            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                            let v16: Rc<str> = counts_text_16(v11);
                            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                            stdout_line_4(v17);
                            census_walk_13(v0, v1, v8, v12, v4, v5, v6)
                        }
                        _ => {
                            let (mut v95, mut v96, mut v97, mut v98): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                US0::US0_WriteBaseline(v22) => {
                                    let v22: Rc<str> = v22.clone();
                                    let v23: Rc<str> = rows_after_18(v9, v10, v11, v5);
                                    census_walk_13(v0, v1, v8, v12, v4, v23, v6)
                                }
                                _ => {
                                    let (mut v91, mut v92, mut v93, mut v94): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                        US0::US0_MergeBaseline(v28) => {
                                            let v28: Rc<str> = v28.clone();
                                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
                                            let v30: i32 = 0i32;
                                            let (mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, i32, i32) = entry_from_21(v1, v9.clone(), v29, v30);
                                            let v35: bool = v33 < 0i32;
                                            if v35 {
                                                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  new  a merge never adds a package that is not in the baseline"); } LIT.with(|lit| lit.clone()) }));
                                                stdout_line_4(v36);
                                                let v37: i32 = v4.wrapping_add(1i32);
                                                let (mut v38, mut v39, mut v40, mut v41): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_13(v0, v1, v8, v12, v37, v5, v6);
                                                (v38, v39, v40, v41)
                                            } else {
                                                let v42: US1 = census_verdict_22(v9.clone(), v10, v11.clone(), v1);
                                                let (mut v59, mut v60, mut v61, mut v62): (i32, i32, Rc<str>, Rc<UH0>) = match &v42 {
                                                    US1::US1_Held => {
                                                        let v43: Rc<str> = rows_after_18(v9.clone(), v10, v11, v5);
                                                        let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v6));
                                                        census_walk_13(v0, v1, v8, v12, v4, v43, v44)
                                                    }
                                                    _ => {
                                                        let (mut v55, mut v56, mut v57, mut v58): (i32, i32, Rc<str>, Rc<UH0>) = match v42 {
                                                            US1::US1_Regressed(v49) => {
                                                                let v49: Rc<str> = v49;
                                                                stdout_line_4(v49);
                                                                let v50: i32 = v4.wrapping_add(1i32);
                                                                census_walk_13(v0, v1, v8, v12, v50, v5, v6)
                                                            }
                                                            _ => unreachable!(),
                                                        };
                                                        (v55, v56, v57, v58)
                                                    }
                                                };
                                                (v59, v60, v61, v62)
                                            }
                                        }
                                        _ => {
                                            let (mut v87, mut v88, mut v89, mut v90): (i32, i32, Rc<str>, Rc<UH0>) = match &v0 {
                                                US0::US0_CheckRatchet(v67) => {
                                                    let v67: Rc<str> = v67.clone();
                                                    let v68: US1 = census_verdict_22(v9, v10, v11, v1);
                                                    match &v68 {
                                                        US1::US1_Held => {
                                                            census_walk_13(v0, v1, v8, v12, v4, v5, v6)
                                                        }
                                                        _ => {
                                                            let (mut v79, mut v80, mut v81, mut v82): (i32, i32, Rc<str>, Rc<UH0>) = match v68 {
                                                                US1::US1_Regressed(v73) => {
                                                                    let v73: Rc<str> = v73;
                                                                    stdout_line_4(v73);
                                                                    let v74: i32 = v4.wrapping_add(1i32);
                                                                    census_walk_13(v0, v1, v8, v12, v74, v5, v6)
                                                                }
                                                                _ => unreachable!(),
                                                            };
                                                            (v79, v80, v81, v82)
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v87, v88, v89, v90)
                                        }
                                    };
                                    (v91, v92, v93, v94)
                                }
                            };
                            (v95, v96, v97, v98)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v103, v104, v105, v106)
        }
    };
    (v107, v108, v109, v110)
}
fn census_started_1(mut v0: US0, mut v1: Rc<UH0>) -> i32 {
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![]));
    let v3: Rc<RefCell<Vec<Rc<str>>>> = method11(v1, v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = kit_style_measure::kit_style_packages_under(v3);
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: i32 = v5.wrapping_sub(1i32);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: Rc<UH0> = list_of_base_12(&v4, v6, v7);
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = Rc::new(RefCell::new(vec![]));
    let v17: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = match &v0 {
        US0::US0_CheckRatchet(v10) => {
            let mut v10: Rc<str> = v10.clone();
            let mut v11: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v10).unwrap_or_default().as_str());
            let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_baseline::kit_style_baseline_entries(&v11);
            v12
        }
        US0::US0_MergeBaseline(v13) => {
            let v13: Rc<str> = v13.clone();
            let v14: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v13).unwrap_or_default().as_str());
            let v15: Rc<RefCell<Vec<(Rc<str>, Rc<str>, i32, i32)>>> = kit_style_baseline::kit_style_baseline_entries(&v14);
            v15
        }
        _ => {
            v9
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
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: standards found no Spiral packages; usage: standards --census <dir>... | standards --baseline <file.spi> <dir>... | standards --baseline-merge <file.spi> <dir>... (replaces only the walked packages' rows) | standards --ratchet <file.spi> <dir>... | standards --scoreboard <root>... (one line per root: lines, then each probe's count and count per 1000 lines) (a directory without .spi files stands for its package subdirectories)"); } LIT.with(|lit| lit.clone()) };
        stderr_line_8(v19);
        2i32
    } else {
        let v20: i32 = 0i32;
        let v21: i32 = 0i32;
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let (mut v24, mut v25, mut v26, mut v27): (i32, i32, Rc<str>, Rc<UH0>) = census_walk_13(v0.clone(), &v17, v8, v20, v21, v22, v23);
        census_report_2(v0, v24, v25, v26, v27)
    }
}
fn with_path_32(mut v0: Rc<dyn Fn(Rc<str>) -> US0>, mut v1: Rc<UH0>) -> i32 {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            let v4: US0 = v0(v2);
            census_started_1(v4, v3)
        }
        UH0::UH0_Nil => {
            let v6: US0 = US0::US0_PrintCensus;
            let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            census_started_1(v6, v7)
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_WriteBaseline(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_MergeBaseline(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> US0> = Rc::new(move |mut v0: Rc<str>| -> US0 {
        US0::US0_CheckRatchet(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn flag_ran_31(mut v0: US2, mut v1: Rc<UH0>) -> i32 {
    match v0 {
        US2::US2_FlagBaseline => {
            let v5: Rc<dyn Fn(Rc<str>) -> US0> = closure1();
            with_path_32(v5, v1)
        }
        US2::US2_FlagBaselineMerge => {
            let v7: Rc<dyn Fn(Rc<str>) -> US0> = closure2();
            with_path_32(v7, v1)
        }
        US2::US2_FlagCensus => {
            let v2: US0 = US0::US0_PrintCensus;
            census_started_1(v2, v1)
        }
        US2::US2_FlagRatchet => {
            let mut v9: Rc<dyn Fn(Rc<str>) -> US0> = closure3();
            with_path_32(v9, v1)
        }
        US2::US2_FlagScoreboard => {
            let v4: i32 = kit_style_scoreboard::kit_style_scoreboard_run();
            v4
        }
    }
}
fn command_line_argument_count_34() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_35(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_33(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_34();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let mut v8: US3 = if v4 {
        let v5: Rc<str> = command_line_argument_35(v0);
        US3::US3_Some(v5)
    } else {
        US3::US3_None
    };
    match v8 {
        US3::US3_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US3::US3_Some(v10) => {
            let mut v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_33(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn flag_named_36(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Census"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = kit_flag_words::kit_flag_word(&v1);
    let v3: bool = &*v0 == &*v2;
    if v3 {
        let v4: US2 = US2::US2_FlagCensus;
        US4::US4_Some(v4)
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Baseline"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = kit_flag_words::kit_flag_word(&v6);
        let mut v8: bool = &*v0 == &*v7;
        if v8 {
            let mut v9: US2 = US2::US2_FlagBaseline;
            US4::US4_Some(v9)
        } else {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BaselineMerge"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = kit_flag_words::kit_flag_word(&v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US2 = US2::US2_FlagBaselineMerge;
                US4::US4_Some(v14)
            } else {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Ratchet"); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = kit_flag_words::kit_flag_word(&v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_FlagRatchet;
                    US4::US4_Some(v19)
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Scoreboard"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = kit_flag_words::kit_flag_word(&v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US2 = US2::US2_FlagScoreboard;
                        US4::US4_Some(v24)
                    } else {
                        US4::US4_None
                    }
                }
            }
        }
    }
}
fn census_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_33(v0);
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<UH0> = v3;
            match Rc::unwrap_or_clone(v3) {
                UH0::UH0_Cons(v4, v5) => {
                    let v4: Rc<str> = v4;
                    let v5: Rc<UH0> = v5;
                    let v6: US4 = flag_named_36(v4.clone());
                    match v6 {
                        US4::US4_None => {
                            let mut v9: US0 = US0::US0_PrintCensus;
                            let mut v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v5));
                            census_started_1(v9, v10)
                        }
                        US4::US4_Some(v7) => {
                            let v7: US2 = v7;
                            flag_ran_31(v7, v5)
                        }
                    }
                }
                _ => {
                    let v14: US0 = US0::US0_PrintCensus;
                    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    census_started_1(v14, v15)
                }
            }
        }
        _ => {
            let v18: US0 = US0::US0_PrintCensus;
            let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            census_started_1(v18, v19)
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
use kit_style_baseline::SpiralShape_97df63ebabc3 as Mut0;
use kit_style_probes::SpiralShape_2617b4e49837 as Mut1;
