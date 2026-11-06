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
pub struct Mut0 { pub l0: i32 }
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
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\\\?(\\s*(\\w+|\\([^)]*\\)|\\{[^}]*\\}))+\\s*\\.|\\\\\\?\\s*\\(|\\bexists\\s+\\w+\\."); } LIT.with(|lit| lit.clone()) };
    let v5: US0 = US0::US0_MayNotFall;
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("forall"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\\\*(\\s*(\\w+|\\([^)]*\\)|\\{[^}]*\\}))+\\s*\\.|\\bforall\\s+\\w+"); } LIT.with(|lit| lit.clone()) };
    let v8: US0 = US0::US0_MayNotFall;
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("indexed-union"); } LIT.with(|lit| lit.clone()) };
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*union\\s+(rec\\s+)?(private\\s+)?\\w+\\s+\\w+\\s*=|^\\s*:\\|\\s+(\\^\\s*)?\\w+(\\s+\\w+)+\\s*="); } LIT.with(|lit| lit.clone()) };
    let mut v11: US0 = US0::US0_MayNotFall;
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("prototype"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(prototype|instance)\\b|^\\s*:[?+]\\s"); } LIT.with(|lit| lit.clone()) };
    let v14: US0 = US0::US0_MayNotFall;
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nominal"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(:!|nominal)\\s+\\w+"); } LIT.with(|lit| lit.clone()) };
    let v17: US0 = US0::US0_MayNotFall;
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("verdict-union"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:\\|\\s+(\\^\\s*)?\\w*(verdict|outcome|refusal|fault|violation|check|change)\\w*\\s"); } LIT.with(|lit| lit.clone()) };
    let v20: US0 = US0::US0_MayNotFall;
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("reflection"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bunion_reflection\\.|\\bunion_case_\\w+|\\bnameof\\b|\\breflection(::|\\.)"); } LIT.with(|lit| lit.clone()) };
    let v23: US0 = US0::US0_MayNotFall;
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("phantom-type"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(:!|nominal)\\s+\\w+(\\s+\\w+)*\\s*=\\s*\\(\\s*\\)\\s*$"); } LIT.with(|lit| lit.clone()) };
    let v26: US0 = US0::US0_MayNotFall;
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("higher-kinded"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\(\\s*\\w+\\s*:\\s*\\*\\s*->"); } LIT.with(|lit| lit.clone()) };
    let v29: US0 = US0::US0_MayNotFall;
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type-decl"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*(union|type|nominal)\\b|^\\s*:(\\||==|!)\\s"); } LIT.with(|lit| lit.clone()) };
    let v32: US0 = US0::US0_Unranked;
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-macro"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s+.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v35: US0 = US0::US0_MayNotRise;
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wide-host-macro"); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']{200,}'"); } LIT.with(|lit| lit.clone()) };
    let v38: US0 = US0::US0_MayNotRise;
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust-global"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(^|[^\\w.])(rust_global\\b|global\\s*(\\(|\\x22))"); } LIT.with(|lit| lit.clone()) };
    let v41: US0 = US0::US0_MayNotRise;
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("encoded-row"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("char::from\\((9|30|31)u8\\)|\\x22\\\\t\\x22"); } LIT.with(|lit| lit.clone()) };
    let v44: US0 = US0::US0_MayNotRise;
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe-table"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pipe_(index|member|lookup)"); } LIT.with(|lit| lit.clone()) };
    let v47: US0 = US0::US0_MayNotRise;
    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("number-arm"); } LIT.with(|lit| lit.clone()) };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\s*\\|\\s*\\d+(u64|i32|u32)?\\s*=>"); } LIT.with(|lit| lit.clone()) };
    let v50: US0 = US0::US0_MayNotRise;
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("string-concat"); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\bconcat5\\b"); } LIT.with(|lit| lit.clone()) };
    let v53: US0 = US0::US0_MayNotRise;
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("top-level-macro"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^\\S.*(\\$'|\\$\\x22)"); } LIT.with(|lit| lit.clone()) };
    let v56: US0 = US0::US0_MayNotRise;
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rc-wrap"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Rc::<str>::from"); } LIT.with(|lit| lit.clone()) };
    let v59: US0 = US0::US0_MayNotRise;
    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("std-string-use"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_std_string\\."); } LIT.with(|lit| lit.clone()) };
    let v62: US0 = US0::US0_MayNotRise;
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bang-export"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!!!!Export\\("); } LIT.with(|lit| lit.clone()) };
    let v65: US0 = US0::US0_MayNotRise;
    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-block"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\$'[^']*$"); } LIT.with(|lit| lit.clone()) };
    let v68: US0 = US0::US0_NeverRises;
    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-statement"); } LIT.with(|lit| lit.clone()) };
    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\x24'[^']*(;|$)"); } LIT.with(|lit| lit.clone()) };
    let v71: US0 = US0::US0_NeverRises;
    let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("banned-name-token"); } LIT.with(|lit| lit.clone()) };
    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^(:~|:=|inl|let)\\s+(\\^\\s*)?([a-z0-9]+_)*(length|len|size|dir|ctx)(_[a-z0-9]+)*'*([\\s(]|$)"); } LIT.with(|lit| lit.clone()) };
    let v74: US0 = US0::US0_NeverRises;
    let v75: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v76: Rc<UH0> = Rc::new(UH0::UH0_Cons(v72, v73, v74, v75));
    let v77: Rc<UH0> = Rc::new(UH0::UH0_Cons(v69, v70, v71, v76));
    let v78: Rc<UH0> = Rc::new(UH0::UH0_Cons(v66, v67, v68, v77));
    let v79: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v64, v65, v78));
    let v80: Rc<UH0> = Rc::new(UH0::UH0_Cons(v60, v61, v62, v79));
    let v81: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57, v58, v59, v80));
    let v82: Rc<UH0> = Rc::new(UH0::UH0_Cons(v54, v55, v56, v81));
    let v83: Rc<UH0> = Rc::new(UH0::UH0_Cons(v51, v52, v53, v82));
    let v84: Rc<UH0> = Rc::new(UH0::UH0_Cons(v48, v49, v50, v83));
    let v85: Rc<UH0> = Rc::new(UH0::UH0_Cons(v45, v46, v47, v84));
    let v86: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v43, v44, v85));
    let v87: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v40, v41, v86));
    let v88: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36, v37, v38, v87));
    let v89: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33, v34, v35, v88));
    let v90: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v31, v32, v89));
    let v91: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v28, v29, v90));
    let v92: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v25, v26, v91));
    let v93: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v22, v23, v92));
    let v94: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v19, v20, v93));
    let v95: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v16, v17, v94));
    let v96: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v13, v14, v95));
    let v97: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v10, v11, v96));
    let v98: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v7, v8, v97));
    let v99: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v4, v5, v98));
    Rc::new(UH0::UH0_Cons(v0, v1, v2, v99))
}
fn method3(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: US0 = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v2, v3)]));
                let v7: i32 = (v1.clone().borrow().len() as i32);
                let v8: i32 = (v6.clone().borrow().len() as i32);
                let v9: i32 = v7.wrapping_add(v8);
                let mut v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, Rc<str>)>::default(); v9 as usize]));
                let mut v11: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method3(v9, &v11) {
                    let v13: i32 = v11.borrow().l0;
                    let v14: bool = v13 < v7;
                    let (mut v20, mut v21): (Rc<str>, Rc<str>) = if v14 {
                        let (mut v15, mut v16): (Rc<str>, Rc<str>) = v1.borrow()[v13 as usize].clone();
                        (v15, v16)
                    } else {
                        let v17: i32 = v13.wrapping_sub(v7);
                        let (mut v18, mut v19): (Rc<str>, Rc<str>) = v6.borrow()[v17 as usize].clone();
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
            UH0::UH0_Nil => {
                return v1;
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
    match v0 {
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
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: US0 = v5;
                let v6: Rc<UH0> = v6;
                let v7: bool = &*v3 == &*v0;
                if v7 {
                    return US1::US1_Some(v3, v4, v5);
                } else {
                    v1 = v6;
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
    match v2 {
        US1::US1_None => {
            US0::US0_Unranked
        }
        US1::US1_Some(v3, v4, v5) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: US0 = v5;
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
        probe_regressed_4(v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn probe_falls_8(mut v0: Rc<str>) -> bool {
    let mut v1: US0 = direction_of_word_6(v0);
    match v1 {
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
        probe_falls_8(v0)
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
pub type SpiralShape_2617b4e49837 = Mut0;
