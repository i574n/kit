#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Box,
    US0_Node,
}
#[derive(Clone)]
enum US1 {
    US1_Compile,
    US1_Rustc,
    US1_Run,
    US1_SlotWait,
    US1_DotnetStartup,
    US1_Mount,
    US1_LockWait,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US1, u64, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<UH1>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Rust,
    US4_Gleam,
    US4_Zig,
    US4_CBackend,
    US4_Lua,
    US4_Wasm,
    US4_Fsharp,
}
#[derive(Clone)]
enum US3 {
    US3_Duration(US4, US1, u64),
    US3_JobDuration(Rc<str>, u64),
}
#[derive(Clone)]
enum US5 {
    US5_PeakBytes(Rc<str>, u64),
}
#[derive(Clone)]
enum US6 {
    US6_Count(Rc<str>, u64),
}
#[derive(Clone)]
enum US2 {
    US2_TimedSample(US0, US3),
    US2_SizedSample(US0, US5),
    US2_CountedSample(US0, US6),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US2, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH3> }
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(US4, Rc<UH4>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(US4),
    US7_None,
}
pub struct Mut1 { pub l0: Rc<str> }
fn method4(mut v0: US0, mut v1: US4, mut v2: Rc<UH1>, mut v3: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5, v6) => {
            let v4: US1 = v4;
            let v5: u64 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH2> = method4(v0.clone(), v1.clone(), v6, v3);
            let v8: US3 = US3::US3_Duration(v1, v4, v5);
            let mut v9: US2 = US2::US2_TimedSample(v0, v8);
            Rc::new(UH2::UH2_Cons(v9, v7))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn method5(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method8(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH3> = v3;
                let v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH3> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method9(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH3> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH3> = v9.borrow().l1.clone();
        let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method8(v16, v17)
    }
}
fn method10(mut v0: Rc<str>, mut v1: Rc<UH4>) -> US7 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v3, v4) => {
                let v3: US4 = v3;
                let v4: Rc<UH4> = v4;
                let v18: Rc<str> = match &v3 {
                    US4::US4_CBackend => {
                        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                        v8
                    }
                    US4::US4_Fsharp => {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                        v11
                    }
                    US4::US4_Gleam => {
                        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                        v6
                    }
                    US4::US4_Lua => {
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                        v9
                    }
                    US4::US4_Rust => {
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                        v5
                    }
                    US4::US4_Wasm => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                        v10
                    }
                    US4::US4_Zig => {
                        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                        v7
                    }
                };
                let v19: bool = &*v18 == &*v0;
                if v19 {
                    return US7::US7_Some(v3);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            UH4::UH4_Nil => {
                return US7::US7_None;
            }
        }
    }
}
fn cell_samples_3(mut v0: US0, mut v1: Rc<UH1>, mut v2: Rc<str>) -> Rc<UH2> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = (v2.clone().len() as i32);
    let v5: i32 = 0i32;
    let v6: i32 = method5(v2.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v2, v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method6(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8, 0i32 as i64, v10 as i64);
    let v12: Rc<UH3> = split_list_7(v3, v11);
    let v17: Rc<str> = match Rc::unwrap_or_clone(v12) {
        UH3::UH3_Cons(v13, v14) => {
            let v13: Rc<str> = v13;
            let v14: Rc<UH3> = v14;
            v13
        }
        UH3::UH3_Nil => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15
        }
    };
    let v18: US4 = US4::US4_Rust;
    let v19: US4 = US4::US4_Gleam;
    let v20: US4 = US4::US4_Zig;
    let v21: US4 = US4::US4_CBackend;
    let v22: US4 = US4::US4_Lua;
    let v23: US4 = US4::US4_Wasm;
    let v24: US4 = US4::US4_Fsharp;
    let v25: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v26: Rc<UH4> = Rc::new(UH4::UH4_Cons(v24, v25));
    let v27: Rc<UH4> = Rc::new(UH4::UH4_Cons(v23, v26));
    let v28: Rc<UH4> = Rc::new(UH4::UH4_Cons(v22, v27));
    let v29: Rc<UH4> = Rc::new(UH4::UH4_Cons(v21, v28));
    let v30: Rc<UH4> = Rc::new(UH4::UH4_Cons(v20, v29));
    let v31: Rc<UH4> = Rc::new(UH4::UH4_Cons(v19, v30));
    let v32: Rc<UH4> = Rc::new(UH4::UH4_Cons(v18, v31));
    let v33: US7 = method10(v17, v32);
    match v33 {
        US7::US7_None => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        US7::US7_Some(v35) => {
            let v35: US4 = v35;
            let v36: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            method4(v0, v35, v1, v36)
        }
    }
}
fn method11(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US2 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method11(v3, v1);
            Rc::new(UH2::UH2_Cons(v2, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method2(mut v0: US0, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5) => {
                let v3: Rc<UH1> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH2> = cell_samples_3(v0.clone(), v3, v4);
                let v7: Rc<UH2> = method11(v2, v6);
                (v1, v2) = (v5, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4) => {
                let mut v2: Rc<UH1> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<UH0> = v4;
                let v5: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method16(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_15(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method16(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn records_from_13(mut v0: std::rc::Rc<serde_json::Value>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v1 >= v2;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method14(v3, v5);
        } else {
            let v7: Rc<str> = format_real_15(v1);
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/cells/"); } LIT.with(|lit| lit.clone()) }, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/metadata"); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/spiral_test/targets/0/target"); } LIT.with(|lit| lit.clone()) }));
            let v11: Rc<str> = std::rc::Rc::<str>::from(v0.pointer(&*v10).and_then(serde_json::Value::as_str).unwrap_or(""));
            let v12: i32 = v1.wrapping_add(1i32);
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/phases_ms/"); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target"); } LIT.with(|lit| lit.clone()) }));
            let v15: u64 = v0.pointer(&*v14).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust_lock_wait"); } LIT.with(|lit| lit.clone()) }));
            let v17: u64 = v0.pointer(&*v16).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_wait"); } LIT.with(|lit| lit.clone()) }));
            let v19: u64 = v0.pointer(&*v18).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v13.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mount"); } LIT.with(|lit| lit.clone()) }));
            let v21: u64 = v0.pointer(&*v20).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) }));
            let v23: u64 = v0.pointer(&*v22).and_then(serde_json::Value::as_u64).unwrap_or(0);
            let v24: US1 = US1::US1_Compile;
            let v25: US1 = US1::US1_Mount;
            let v26: US1 = US1::US1_SlotWait;
            let v27: US1 = US1::US1_LockWait;
            let v28: US1 = US1::US1_Run;
            let v29: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v30: Rc<UH1> = Rc::new(UH1::UH1_Cons(v28, v15, v29));
            let v31: Rc<UH1> = Rc::new(UH1::UH1_Cons(v27, v17, v30));
            let v32: Rc<UH1> = Rc::new(UH1::UH1_Cons(v26, v19, v31));
            let v33: Rc<UH1> = Rc::new(UH1::UH1_Cons(v25, v21, v32));
            let v34: Rc<UH1> = Rc::new(UH1::UH1_Cons(v24, v23, v33));
            let v35: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v11, v3));
            (v1, v3) = (v12, v35);
            continue;
        }
    }
}
fn cell_records_of_12(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Option<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&*v0).ok();
    let mut v2: std::rc::Rc<serde_json::Value> = std::rc::Rc::new(v1.unwrap_or_default());
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/cells"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = v2.pointer(&*v4).and_then(serde_json::Value::as_array).map_or(0, |items| items.len() as i32);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    records_from_13(v2, v3, v5, v6)
}
fn notebook_samples_1(mut v0: US0, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: Rc<UH0> = cell_records_of_12(v1);
    let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method2(v0, v2, v3)
}
fn format_real_18(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method16(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method17(mut v0: Rc<UH2>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: US2 = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH3> = method17(v3, v1);
            let v122: Rc<str> = match v2 {
                US2::US2_CountedSample(v98, v99) => {
                    let v98: US0 = v98;
                    let v99: US6 = v99;
                    let v103: Rc<str> = match v98 {
                        US0::US0_Box => {
                            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v100
                        }
                        US0::US0_Node => {
                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v101
                        }
                    };
                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v103));
                    let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v99 {
                        US6::US6_Count(v106, v107) => {
                            let v106: Rc<str> = v106;
                            let v107: u64 = v107;
                            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("what=\""); } LIT.with(|lit| lit.clone()) }, v106));
                            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v108, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v110: Rc<str> = format_real_18(v107);
                            let mut v111: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_count{"); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v112);
                            v111.borrow_mut().push(v105);
                            let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v113);
                            v111.borrow_mut().push(v109);
                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v111.borrow_mut().push(v114);
                            v111.borrow_mut().push(v110);
                            let v115: Rc<Vec<Rc<str>>> = Rc::new(v111.borrow().clone());
                            let v116: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v115).as_ref().clone()));
                            let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v118: Rc<str> = Rc::<str>::from((*v116.borrow()).join(&*v117));
                            v118
                        }
                    }
                }
                US2::US2_SizedSample(v76, v77) => {
                    let v76: US0 = v76;
                    let v77: US5 = v77;
                    let v81: Rc<str> = match v76 {
                        US0::US0_Box => {
                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v78
                        }
                        US0::US0_Node => {
                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v79
                        }
                    };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v81));
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v77 {
                        US5::US5_PeakBytes(v84, v85) => {
                            let v84: Rc<str> = v84;
                            let v85: u64 = v85;
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("what=\""); } LIT.with(|lit| lit.clone()) }, v84));
                            let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v88: Rc<str> = format_real_18(v85);
                            let mut v89: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_peak_bytes{"); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v90);
                            v89.borrow_mut().push(v83);
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v91);
                            v89.borrow_mut().push(v87);
                            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v89.borrow_mut().push(v92);
                            v89.borrow_mut().push(v88);
                            let v93: Rc<Vec<Rc<str>>> = Rc::new(v89.borrow().clone());
                            let v94: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v93).as_ref().clone()));
                            let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v96: Rc<str> = Rc::<str>::from((*v94.borrow()).join(&*v95));
                            v96
                        }
                    }
                }
                US2::US2_TimedSample(v5, v6) => {
                    let v5: US0 = v5;
                    let v6: US3 = v6;
                    let v10: Rc<str> = match v5 {
                        US0::US0_Box => {
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("box"); } LIT.with(|lit| lit.clone()) };
                            v7
                        }
                        US0::US0_Node => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("node"); } LIT.with(|lit| lit.clone()) };
                            v8
                        }
                    };
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine=\""); } LIT.with(|lit| lit.clone()) }, v10));
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    match v6 {
                        US3::US3_Duration(v13, v14, v15) => {
                            let v13: US4 = v13;
                            let v14: US1 = v14;
                            let v15: u64 = v15;
                            let v29: Rc<str> = match v13 {
                                US4::US4_CBackend => {
                                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
                                    v19
                                }
                                US4::US4_Fsharp => {
                                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsharp"); } LIT.with(|lit| lit.clone()) };
                                    v22
                                }
                                US4::US4_Gleam => {
                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
                                    v17
                                }
                                US4::US4_Lua => {
                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lua"); } LIT.with(|lit| lit.clone()) };
                                    v20
                                }
                                US4::US4_Rust => {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rust"); } LIT.with(|lit| lit.clone()) };
                                    v16
                                }
                                US4::US4_Wasm => {
                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wasm"); } LIT.with(|lit| lit.clone()) };
                                    v21
                                }
                                US4::US4_Zig => {
                                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
                                    v18
                                }
                            };
                            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backend=\""); } LIT.with(|lit| lit.clone()) }, v29));
                            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v45: Rc<str> = match v14 {
                                US1::US1_Compile => {
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                    v32
                                }
                                US1::US1_DotnetStartup => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet_startup"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                                US1::US1_LockWait => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock_wait"); } LIT.with(|lit| lit.clone()) };
                                    v38
                                }
                                US1::US1_Mount => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mount"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                                US1::US1_Run => {
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("run"); } LIT.with(|lit| lit.clone()) };
                                    v34
                                }
                                US1::US1_Rustc => {
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rustc"); } LIT.with(|lit| lit.clone()) };
                                    v33
                                }
                                US1::US1_SlotWait => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_wait"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                            };
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("phase=\""); } LIT.with(|lit| lit.clone()) }, v45));
                            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let mut v48: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            v48.borrow_mut().push(v12);
                            v48.borrow_mut().push(v31);
                            v48.borrow_mut().push(v47);
                            let v49: Rc<Vec<Rc<str>>> = Rc::new(v48.borrow().clone());
                            let v50: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v49).as_ref().clone()));
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = Rc::<str>::from((*v50.borrow()).join(&*v51));
                            let v53: Rc<str> = format_real_18(v15);
                            let mut v54: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_exec_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v55);
                            v54.borrow_mut().push(v52);
                            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v54.borrow_mut().push(v56);
                            v54.borrow_mut().push(v53);
                            let v57: Rc<Vec<Rc<str>>> = Rc::new(v54.borrow().clone());
                            let v58: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v57).as_ref().clone()));
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = Rc::<str>::from((*v58.borrow()).join(&*v59));
                            v60
                        }
                        US3::US3_JobDuration(v61, v62) => {
                            let v61: Rc<str> = v61;
                            let v62: u64 = v62;
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lane=\""); } LIT.with(|lit| lit.clone()) }, v61));
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                            let v65: Rc<str> = format_real_18(v62);
                            let mut v66: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit_job_duration_ms{"); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v67);
                            v66.borrow_mut().push(v12);
                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v68);
                            v66.borrow_mut().push(v64);
                            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("} "); } LIT.with(|lit| lit.clone()) };
                            v66.borrow_mut().push(v69);
                            v66.borrow_mut().push(v65);
                            let v70: Rc<Vec<Rc<str>>> = Rc::new(v66.borrow().clone());
                            let v71: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v70).as_ref().clone()));
                            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v73: Rc<str> = Rc::<str>::from((*v71.borrow()).join(&*v72));
                            v73
                        }
                    }
                }
            };
            Rc::new(UH3::UH3_Cons(v122, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn method19(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH3>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH3> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn notebook_prom_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "node";
    let v5: US0 = if v2 {
        US0::US0_Node
    } else {
        US0::US0_Box
    };
    let v6: Rc<UH2> = notebook_samples_1(v5, v0);
    let v7: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH3> = method17(v6, v7);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v10: i32 = 0i32;
    let v11: i32 = method19(v9.clone(), v8, v10);
    let v12: Rc<Vec<Rc<str>>> = Rc::new(v9.borrow().clone());
    let v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v12).as_ref().clone()));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from((*v13.borrow()).join(&*v14));
    v15
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notebook_prom_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_exec_notebook_prom(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_4fa472513c6d = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
