#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_MayNotFall,
    US0_MayNotRise,
    US0_NeverRises,
    US0_Unranked,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, US0, Rc<UH0>),
}
struct Mut0 { l0: i32 }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, US0),
    US1_None,
}
fn probe_table_1() -> Rc<UH0> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gadt-case"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*[A-Z]\\w*\\s*::[^=]*$"); } LIT.with(|lit| lit.clone()) };
    let v2: US0 = US0::US0_MayNotFall;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("existential"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\\\?\\s*\\w+\\.|\\bexists\\s+\\w+\\."); } LIT.with(|lit| lit.clone()) };
    let v5: US0 = US0::US0_MayNotFall;
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forall"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\\\*\\s*\\w+(\\s+\\w+)*\\.|\\bforall\\s+\\w+"); } LIT.with(|lit| lit.clone()) };
    let v8: US0 = US0::US0_MayNotFall;
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("indexed-union"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*union\\s+(rec\\s+)?(private\\s+)?\\w+\\s+\\w+\\s*="); } LIT.with(|lit| lit.clone()) };
    let mut v11: US0 = US0::US0_MayNotFall;
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prototype"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(prototype|instance)\\b"); } LIT.with(|lit| lit.clone()) };
    let v14: US0 = US0::US0_MayNotFall;
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nominal"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:!\\s+\\w+"); } LIT.with(|lit| lit.clone()) };
    let v17: US0 = US0::US0_MayNotFall;
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict-union"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:\\|\\s+(\\^\\s*)?\\w*(verdict|outcome|refusal|fault|violation|check|change)\\w*\\s"); } LIT.with(|lit| lit.clone()) };
    let v20: US0 = US0::US0_MayNotFall;
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(union|type|nominal)\\b"); } LIT.with(|lit| lit.clone()) };
    let v23: US0 = US0::US0_Unranked;
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-macro"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s+.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v26: US0 = US0::US0_MayNotRise;
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']{200,}'"); } LIT.with(|lit| lit.clone()) };
    let v29: US0 = US0::US0_MayNotRise;
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(^|[^\\w.])(rust_global\\b|global\\s*(\\(|\\x22))"); } LIT.with(|lit| lit.clone()) };
    let v32: US0 = US0::US0_MayNotRise;
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("char::from\\((9|30|31)u8\\)|\\x22\\\\t\\x22"); } LIT.with(|lit| lit.clone()) };
    let v35: US0 = US0::US0_MayNotRise;
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe-table"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe_(index|member|lookup)"); } LIT.with(|lit| lit.clone()) };
    let v38: US0 = US0::US0_MayNotRise;
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("number-arm"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*\\d+(u64|i32|u32)?\\s*=>"); } LIT.with(|lit| lit.clone()) };
    let v41: US0 = US0::US0_MayNotRise;
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("string-concat"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bconcat5\\b"); } LIT.with(|lit| lit.clone()) };
    let v44: US0 = US0::US0_MayNotRise;
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("top-level-macro"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\S.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v47: US0 = US0::US0_MayNotRise;
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rc-wrap"); } LIT.with(|lit| lit.clone()) };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rc::<str>::from"); } LIT.with(|lit| lit.clone()) };
    let v50: US0 = US0::US0_MayNotRise;
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("std-string-use"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_std_string\\."); } LIT.with(|lit| lit.clone()) };
    let v53: US0 = US0::US0_MayNotRise;
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bang-export"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!!!!Export\\("); } LIT.with(|lit| lit.clone()) };
    let v56: US0 = US0::US0_MayNotRise;
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-block"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']*$"); } LIT.with(|lit| lit.clone()) };
    let v59: US0 = US0::US0_NeverRises;
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-statement"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x24'[^']*(;|$)"); } LIT.with(|lit| lit.clone()) };
    let v62: US0 = US0::US0_NeverRises;
    let v63: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v64: Rc<UH0> = Rc::new(UH0::UH0_Cons(v60.clone(), v61.clone(), v62.clone(), v63.clone()));
    let v65: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57.clone(), v58.clone(), v59.clone(), v64.clone()));
    let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v54.clone(), v55.clone(), v56.clone(), v65.clone()));
    let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v51.clone(), v52.clone(), v53.clone(), v66.clone()));
    let v68: Rc<UH0> = Rc::new(UH0::UH0_Cons(v48.clone(), v49.clone(), v50.clone(), v67.clone()));
    let v69: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45.clone(), v46.clone(), v47.clone(), v68.clone()));
    let v70: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42.clone(), v43.clone(), v44.clone(), v69.clone()));
    let v71: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39.clone(), v40.clone(), v41.clone(), v70.clone()));
    let v72: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36.clone(), v37.clone(), v38.clone(), v71.clone()));
    let v73: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33.clone(), v34.clone(), v35.clone(), v72.clone()));
    let v74: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30.clone(), v31.clone(), v32.clone(), v73.clone()));
    let v75: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27.clone(), v28.clone(), v29.clone(), v74.clone()));
    let v76: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24.clone(), v25.clone(), v26.clone(), v75.clone()));
    let v77: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21.clone(), v22.clone(), v23.clone(), v76.clone()));
    let v78: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18.clone(), v19.clone(), v20.clone(), v77.clone()));
    let v79: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15.clone(), v16.clone(), v17.clone(), v78.clone()));
    let v80: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12.clone(), v13.clone(), v14.clone(), v79.clone()));
    let v81: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v10.clone(), v11.clone(), v80.clone()));
    let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v7.clone(), v8.clone(), v81.clone()));
    let v83: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v4.clone(), v5.clone(), v82.clone()));
    Rc::new(UH0::UH0_Cons(v0.clone(), v1.clone(), v2.clone(), v83.clone()))
}
fn method3(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: US0 = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v2.clone(), v3.clone())]));
                let v7: i32 = (v1.clone().borrow().len() as i32);
                let v8: i32 = (v6.clone().borrow().len() as i32);
                let v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method3(v9, v11.clone()) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, Rc<str>) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, Rc<str>) = v1.borrow()[v13 as usize].clone();
                        (v15.clone(), v16.clone())
                    } else {
                        let v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, Rc<str>) = v6.borrow()[v17 as usize].clone();
                        (v18.clone(), v19.clone())
                    };
                    v10.borrow_mut()[v13 as usize] = (v20.clone(), v21.clone());
                    let v22: i32 = v13.wrapping_add(1i32);
                    v11.borrow_mut().l0 = v22;
                    ()
                };
                (v0, v1) = (v5.clone(), v10.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn probe_specs_0() -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    let v0: Rc<UH0> = probe_table_1();
    let mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![]));
    let v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = method2(v0, v1);
    v2
}
fn closure0() -> Rc<dyn Fn() -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>> = Rc::new(move || -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
        probe_specs_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn regressed_5(mut v0: US0, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    let v5: f64 = (v1 as f64);
    let v6: f64 = v5 * 1000.0f64;
    let v7: bool = v2 > 1i32;
    let v8: i32 = if v7 {
        v2
    } else {
        1i32
    };
    let v9: f64 = (v8 as f64);
    let mut v10: f64 = v6 / v9;
    let mut v11: f64 = (v3 as f64);
    let v12: f64 = v11 * 1000.0f64;
    let v13: bool = v4 > 1i32;
    let v14: i32 = if v13 {
        v4
    } else {
        1i32
    };
    let v15: f64 = (v14 as f64);
    let v16: f64 = v12 / v15;
    match &v0 {
        US0::US0_MayNotFall => {
            let v22: bool = v3 < v1;
            let v26: bool = if v22 {
                let v23: f64 = v10 * 0.95f64;
                let v24: f64 = v23 - 0.5f64;
                let v25: bool = v16 < v24;
                v25
            } else {
                false
            };
            if v26 {
                let v27: i32 = v1.wrapping_sub(v3);
                let v28: i32 = v2.wrapping_sub(v4);
                let v29: bool = v27 > v28;
                v29
            } else {
                false
            }
        }
        US0::US0_MayNotRise => {
            let v17: f64 = v10 * 1.05f64;
            let v18: f64 = v17 + 0.5f64;
            let v19: bool = v16 > v18;
            if v19 {
                let v20: bool = v3 > v1;
                v20
            } else {
                false
            }
        }
        US0::US0_NeverRises => {
            let v31: bool = v3 > v1;
            v31
        }
        US0::US0_Unranked => {
            false
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US1 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: US0 = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: bool = &*v3 == &*v0;
                if v7 {
                    return US1::US1_Some(v3.clone(), v4.clone(), v5.clone());
                } else {
                    v1 = v6.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn direction_of_word_6(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<UH0> = probe_table_1();
    let v2: US1 = method7(v0, v1);
    match &v2 {
        US1::US1_None => {
            US0::US0_Unranked
        }
        US1::US1_Some(v3, v4, v5) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: US0 = v5.clone();
            v5
        }
    }
}
fn probe_regressed_4(mut v0: i32, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> bool {
    let v5: US0 = direction_of_word_6(v4);
    regressed_5(v5, v3, v2, v1, v0)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32, i32, i32, i32) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, i32, i32) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32| -> bool {
        probe_regressed_4(v4, v3, v2, v1, v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn probe_falls_8(mut v0: Rc<str>) -> bool {
    let mut v1: US0 = direction_of_word_6(v0);
    match &v1 {
        US0::US0_MayNotFall => {
            true
        }
        _ => {
            false
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        probe_falls_8(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_style_probe_specs() -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    closure0()()
}
pub fn kit_style_probe_regressed(v0: &str, v1: i32, v2: i32, v3: i32, v4: i32) -> bool {
    closure1()(Rc::<str>::from(v0), v1, v2, v3, v4)
}
pub fn kit_style_probe_falls(v0: &str) -> bool {
    closure2()(Rc::<str>::from(v0))
}
