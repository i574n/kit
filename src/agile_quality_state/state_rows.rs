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
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, i32, Rc<UH2>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(i32),
    US0_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<str>, Rc<str>, i64, Rc<UH4>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH3> }
#[derive(Clone)]
enum US2 {
    US2_Some(i64),
    US2_None,
}
fn method3(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7): (Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let mut v8: i32 = v1.wrapping_sub(1i32);
            let mut v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6, v7, v2));
            (v1, v2) = (v8, v9);
            continue;
        }
    }
}
fn method4(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<UH1> = v9;
            let v10: Rc<UH1> = method4(v9, v1);
            let v11: bool = &*v3 != "Done";
            if v11 {
                Rc::new(UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v10))
            } else {
                v10
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method5(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<UH1> = v10;
                v0.borrow_mut().push((v3, v4, v5, v6, v7, v8, v9));
                let v11: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v10, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn open_failures_in_2(mut v0: Rc<UH1>) -> Rc<UH0> {
    let mut v1: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH1> = method4(v0, v1);
    let v3: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method5(v3.clone(), v2, v4);
    let v6: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v3.borrow().clone());
    let mut v7: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let mut v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_lint::kit_agile_lint_failing_rows(v7);
    let mut v9: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v8).borrow().clone());
    let v10: usize = (v9).len();
    let v11: i32 = ((v10) as i32);
    let v12: i32 = v11.wrapping_sub(1i32);
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v9, v12, v13)
}
fn method7(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = (v0)[v4].clone();
            let v12: i32 = v1.wrapping_sub(1i32);
            let v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v5, v6, v7, v8, v9, v10, v11, v2));
            (v1, v2) = (v12, v13);
            continue;
        }
    }
}
fn facts_list_6(mut v0: Rc<str>) -> Rc<UH1> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_facts::kit_agile_fact_rows(&v0);
    let mut v2: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>> = Rc::new((v1).borrow().clone());
    let v3: usize = (v2).len();
    let mut v4: i32 = ((v3) as i32);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method7(v2, v5, v6)
}
fn open_failures_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<UH1> = facts_list_6(v0);
    open_failures_in_2(v1)
}
fn method8(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let mut v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn failures_of_0(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: Rc<UH0> = open_failures_1(v0);
    let mut v2: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v3: i32 = 0i32;
    let mut v4: i32 = method8(v2.clone(), v1, v3);
    let mut v5: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v2.borrow().clone());
    let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v5).as_ref().clone()));
    v6
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        failures_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4) => {
                let mut v2: Rc<str> = v2;
                let v3: i32 = v3;
                let mut v4: Rc<UH2> = v4;
                let mut v5: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3, v1));
                (v0, v1) = (v4, v5);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method13(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure2() -> Rc<dyn Fn((i32)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US0> = Rc::new(move |mut v0: (i32)| -> US0 {
        let mut v1: i32 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12(mut v0: Rc<UH0>, mut v1: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH0> = v5;
                let v6: Option<i32> = v1.get(&v4).cloned();
                let mut v7: Option<i32> = method13(v6);
                let mut v8: Rc<dyn Fn((i32)) -> US0> = closure2();
                let mut v9: Option<US0> = v7.map(|x| v8(x));
                let v11: US0 = US0::US0_None;
                let v12: US0 = v11.clone();
                let v13: US0 = v9.unwrap_or(v12);
                let v16: i32 = match v13 {
                    US0::US0_None => {
                        0i32
                    }
                    US0::US0_Some(v14) => {
                        let v14: i32 = v14;
                        v14
                    }
                };
                let v17: i32 = v16.wrapping_add(1i32);
                let mut v18: std::collections::BTreeMap<Rc<str>, i32> = std::rc::Rc::unwrap_or_clone(v1);
                let v20: bool = true; // 1;
                let mut v18 = v18;
                let mut v22: Option<i32> = v18.insert(v4, v17);
                let v23: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = std::rc::Rc::new(v18);
                (v0, v1) = (v5, v23);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method14(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn method15(mut v0: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>>, mut v1: Rc<UH3>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH3> = v4;
                let mut v5: Option<i32> = v0.get(&v3).cloned();
                let v6: Option<i32> = method13(v5);
                let mut v7: Rc<dyn Fn((i32)) -> US0> = closure2();
                let mut v8: Option<US0> = v6.map(|x| v7(x));
                let v10: US0 = US0::US0_None;
                let v11: US0 = v10.clone();
                let v12: US0 = v8.unwrap_or(v11);
                let v16: i32 = match v12 {
                    US0::US0_None => {
                        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dict.present: the key is not in the dict"); } LIT.with(|lit| lit.clone()) }))
                    }
                    US0::US0_Some(v13) => {
                        let v13: i32 = v13;
                        v13
                    }
                };
                let v17: Rc<UH2> = Rc::new(UH2::UH2_Cons(v3, v16, v2));
                (v1, v2) = (v4, v17);
                continue;
            }
            UH3::UH3_Nil => {
                return v2;
            }
        }
    }
}
fn census_of_10(mut v0: Rc<UH0>) -> Rc<UH2> {
    let mut v2: std::collections::BTreeMap<Rc<str>, i32> = std::collections::BTreeMap::new();
    let v3: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = std::rc::Rc::new(v2);
    let mut v4: std::rc::Rc<std::collections::BTreeMap<Rc<str>, i32>> = method12(v0, v3);
    let mut v5: Vec<Rc<str>> = v4.keys().cloned().collect::<Vec<_>>();
    let v6: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let mut v8: bool = true; let _vec_fold_ = v5.into_iter().fold(v6, |acc, x| { //;
    let v10: Rc<UH3> = acc;
    let v12: Rc<str> = x;
    let v14: Rc<UH3> = Rc::new(UH3::UH3_Cons(v12, v10));
    let mut v15: bool = true; v14 });
    let v17: Rc<UH3> = _vec_fold_;
    let mut v18: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v19: Rc<UH3> = method14(v17, v18);
    let v20: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v21: Rc<UH2> = method15(v4, v19, v20);
    let mut v22: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method11(v21, v22)
}
fn method16(mut v0: Rc<RefCell<Vec<(Rc<str>, i32)>>>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v3, v4, v5) => {
                let v3: Rc<str> = v3;
                let mut v4: i32 = v4;
                let mut v5: Rc<UH2> = v5;
                v0.borrow_mut().push((v3, v4));
                let v6: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v5, v6);
                continue;
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn census_text_of_9(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    let mut v1: Rc<UH0> = open_failures_1(v0);
    let mut v2: Rc<UH2> = census_of_10(v1);
    let v3: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method16(v3.clone(), v2, v4);
    let v6: Rc<Vec<(Rc<str>, i32)>> = Rc::new(v3.borrow().clone());
    let mut v7: Rc<RefCell<Vec<(Rc<str>, i32)>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    v7
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
        census_text_of_9(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18(mut v0: Rc<str>, mut v1: Rc<UH1>) -> US1 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: i32 = v6;
                let mut v7: Rc<str> = v7;
                let mut v8: Rc<str> = v8;
                let mut v9: Rc<str> = v9;
                let v10: Rc<UH1> = v10;
                let v11: bool = &*v3 == &*v0;
                if v11 {
                    return US1::US1_Some(v3, v4, v5, v6, v7, v8, v9);
                } else {
                    v1 = v10;
                    continue;
                }
            }
            UH1::UH1_Nil => {
                return US1::US1_None;
            }
        }
    }
}
fn item_failures_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let v3: Rc<UH1> = facts_list_6(v2);
    let mut v4: US1 = method18(v1, v3);
    match v4 {
        US1::US1_None => {
            Rc::new(RefCell::new(vec![]))
        }
        US1::US1_Some(v5, v6, v7, v8, v9, v10, v11) => {
            let mut v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: i32 = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<str> = v11;
            let v12: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(vec![(v5, v0, v7, v8, v9, v10, v11)]));
            let v13: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = agile_quality_lint::kit_agile_lint_failing_rows(v12);
            v13
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        item_failures_17(v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method24(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH3> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH3::UH3_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let mut v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method24(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH3> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH3> = Rc::new(UH3::UH3_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH3> = v9.borrow().l1.clone();
        let v17: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method14(v16, v17)
    }
}
fn method26(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure5() -> Rc<dyn Fn((i64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US2> = Rc::new(move |mut v0: (i64)| -> US2 {
        let mut v1: i64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn stamp_hour_25(mut v0: Rc<str>) -> i64 {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: bool = v1 < 13i32;
    if v2 {
        0i64
    } else {
        let v3: usize = ((0i32) as usize);
        let mut v4: usize = ((4i32) as usize);
        let mut v5: Rc<str> = Rc::<str>::from(&(v0)[v3..v4]);
        let v6: Option<i64> = (v5).trim().parse::<i64>().ok();
        let mut v7: Option<i64> = method26(v6);
        let mut v8: Rc<dyn Fn((i64)) -> US2> = closure5();
        let mut v9: Option<US2> = v7.map(|x| v8(x));
        let v11: US2 = US2::US2_None;
        let v12: US2 = v11.clone();
        let v13: US2 = v9.unwrap_or(v12);
        let v16: i64 = match v13 {
            US2::US2_None => {
                0i64
            }
            US2::US2_Some(v14) => {
                let v14: i64 = v14;
                v14
            }
        };
        let v17: usize = ((5i32) as usize);
        let mut v18: usize = ((7i32) as usize);
        let v19: Rc<str> = Rc::<str>::from(&(v0)[v17..v18]);
        let v20: Option<i64> = (v19).trim().parse::<i64>().ok();
        let v21: Option<i64> = method26(v20);
        let mut v22: Option<US2> = v21.map(|x| v8(x));
        let v24: US2 = US2::US2_None;
        let v25: US2 = v24.clone();
        let v26: US2 = v22.unwrap_or(v25);
        let v29: i64 = match v26 {
            US2::US2_None => {
                0i64
            }
            US2::US2_Some(v27) => {
                let v27: i64 = v27;
                v27
            }
        };
        let v30: usize = ((8i32) as usize);
        let v31: usize = ((10i32) as usize);
        let v32: Rc<str> = Rc::<str>::from(&(v0)[v30..v31]);
        let v33: Option<i64> = (v32).trim().parse::<i64>().ok();
        let v34: Option<i64> = method26(v33);
        let mut v35: Option<US2> = v34.map(|x| v8(x));
        let v37: US2 = US2::US2_None;
        let v38: US2 = v37.clone();
        let v39: US2 = v35.unwrap_or(v38);
        let v42: i64 = match v39 {
            US2::US2_None => {
                0i64
            }
            US2::US2_Some(v40) => {
                let v40: i64 = v40;
                v40
            }
        };
        let v43: bool = v29 <= 2i64;
        let v45: i64 = if v43 {
            let v44: i64 = v16.wrapping_sub(1i64);
            v44
        } else {
            v16
        };
        let v46: i64 = v45.wrapping_div(400i64);
        let v47: i64 = v45.wrapping_rem(400i64);
        let v48: bool = v47 != 0i64;
        let v50: bool = if v48 {
            let v49: bool = v45 < 0i64;
            v49
        } else {
            false
        };
        let v52: i64 = if v50 {
            let v51: i64 = v46.wrapping_sub(1i64);
            v51
        } else {
            v46
        };
        let v53: i64 = v52.wrapping_mul(400i64);
        let v54: i64 = v45.wrapping_sub(v53);
        let v55: i64 = v54.wrapping_mul(365i64);
        let v56: i64 = v54.wrapping_div(4i64);
        let v57: i64 = v55.wrapping_add(v56);
        let v58: i64 = v54.wrapping_div(100i64);
        let v59: i64 = v57.wrapping_sub(v58);
        let v60: bool = v29 > 2i64;
        let v63: i64 = if v60 {
            let v61: i64 = v29.wrapping_sub(3i64);
            v61
        } else {
            let v62: i64 = v29.wrapping_add(9i64);
            v62
        };
        let v64: i64 = 153i64.wrapping_mul(v63);
        let v65: i64 = v64.wrapping_add(2i64);
        let v66: i64 = v65.wrapping_div(5i64);
        let v67: i64 = v66.wrapping_add(v42);
        let v68: i64 = v67.wrapping_sub(1i64);
        let v69: i64 = v59.wrapping_add(v68);
        let v70: i64 = v52.wrapping_mul(146097i64);
        let v71: i64 = v70.wrapping_add(v69);
        let v72: i64 = v71.wrapping_sub(719468i64);
        let v73: i64 = v72.wrapping_mul(24i64);
        let v74: usize = ((11i32) as usize);
        let v75: usize = ((13i32) as usize);
        let v76: Rc<str> = Rc::<str>::from(&(v0)[v74..v75]);
        let v77: Option<i64> = (v76).trim().parse::<i64>().ok();
        let v78: Option<i64> = method26(v77);
        let mut v79: Option<US2> = v78.map(|x| v8(x));
        let v81: US2 = US2::US2_None;
        let v82: US2 = v81.clone();
        let v83: US2 = v79.unwrap_or(v82);
        let v86: i64 = match v83 {
            US2::US2_None => {
                0i64
            }
            US2::US2_Some(v84) => {
                let v84: i64 = v84;
                v84
            }
        };
        let v87: i64 = v73.wrapping_add(v86);
        v87
    }
}
fn method22(mut v0: Rc<UH3>, mut v1: i64) -> i64 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH3> = v3;
                let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let mut v5: Rc<UH3> = split_list_23(v4, v2);
                let mut v18: i64 = match Rc::unwrap_or_clone(v5) {
                    UH3::UH3_Cons(v6, v7) => {
                        let v6: Rc<str> = v6;
                        let mut v7: Rc<UH3> = v7;
                        match Rc::unwrap_or_clone(v7) {
                            UH3::UH3_Cons(v8, v9) => {
                                let mut v8: Rc<str> = v8;
                                let mut v9: Rc<UH3> = v9;
                                match Rc::unwrap_or_clone(v9) {
                                    UH3::UH3_Cons(v10, v11) => {
                                        let v10: Rc<str> = v10;
                                        let v11: Rc<UH3> = v11;
                                        match Rc::unwrap_or_clone(v11) {
                                            UH3::UH3_Cons(v12, v13) => {
                                                let v12: Rc<str> = v12;
                                                let v13: Rc<UH3> = v13;
                                                stamp_hour_25(v12)
                                            }
                                            _ => {
                                                0i64
                                            }
                                        }
                                    }
                                    _ => {
                                        0i64
                                    }
                                }
                            }
                            _ => {
                                0i64
                            }
                        }
                    }
                    _ => {
                        0i64
                    }
                };
                let v19: bool = v18 > v1;
                let v20: i64 = if v19 {
                    v18
                } else {
                    v1
                };
                (v0, v1) = (v3, v20);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn last_note_hour_21(mut v0: Rc<str>) -> i64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH3> = split_list_23(v1, v0);
    let v3: i64 = 0i64;
    method22(v2, v3)
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH4>) -> Rc<UH4> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5, v6, v7, v8, v9) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: i32 = v5;
            let v6: Rc<str> = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<UH1> = v9;
            let v10: Rc<UH4> = method20(v9, v1);
            let v11: i64 = last_note_hour_21(v8);
            Rc::new(UH4::UH4_Cons(v2, v3, v4, v11, v10))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>, mut v1: Rc<UH4>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH4::UH4_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<str> = v5;
                let v6: i64 = v6;
                let mut v7: Rc<UH4> = v7;
                v0.borrow_mut().push((v3, v4, v5, v6));
                let mut v8: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v7, v8);
                continue;
            }
            UH4::UH4_Nil => {
                return v2;
            }
        }
    }
}
fn cadence_of_19(mut v0: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> {
    let mut v1: Rc<UH1> = facts_list_6(v0);
    let mut v2: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<UH4> = method20(v1, v2);
    let mut v4: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v5: i32 = 0i32;
    let v6: i32 = method27(v4.clone(), v3, v5);
    let mut v7: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>> = Rc::new(v4.borrow().clone());
    let mut v8: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    v8
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> {
        cadence_of_19(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> i64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i64> = Rc::new(move |mut v0: Rc<str>| -> i64 {
        stamp_hour_25(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method32(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method33(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
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
fn method31(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH3> = v3;
            let mut v4: Rc<UH3> = method31(v3, v1);
            let mut v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let mut v7: i32 = method32(v2.clone(), v5, v6);
            let mut v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method33(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH3::UH3_Cons(v12, v4))
        }
        UH3::UH3_Nil => {
            v1
        }
    }
}
fn closure8(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH3>) -> US0> {
    Rc::new(move |mut v2: Rc<UH3>| -> US0 {
        match Rc::unwrap_or_clone(v2) {
            UH3::UH3_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH3> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US0::US0_Some(v1)
                } else {
                    let mut v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH3>) -> US0> = method34(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH3::UH3_Nil => {
                US0::US0_None
            }
        }
    })
}
fn method34(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH3>) -> US0> {
    closure8(v0, v1)
}
fn of_team_30(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>) -> bool {
    let mut v8: bool = &*v0 == "";
    if v8 {
        true
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<UH3> = split_list_23(v9, v3);
        let v11: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v12: Rc<UH3> = method31(v10, v11);
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) }, v0));
        let v14: i32 = 0i32;
        let mut v15: Rc<dyn Fn(Rc<UH3>) -> US0> = method34(v13, v14);
        let v16: US0 = v15(v12);
        let v17: bool = match v16 {
            US0::US0_None => {
                true
            }
            _ => {
                false
            }
        };
        let mut v18: bool = v17 != true;
        v18
    }
}
fn method29(mut v0: Rc<str>, mut v1: Rc<UH1>, mut v2: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v1) {
        UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v10) => {
            let v3: Rc<str> = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<str> = v5;
            let v6: i32 = v6;
            let mut v7: Rc<str> = v7;
            let mut v8: Rc<str> = v8;
            let mut v9: Rc<str> = v9;
            let v10: Rc<UH1> = v10;
            let v11: Rc<UH1> = method29(v0.clone(), v10, v2);
            let v12: bool = of_team_30(v0, &v3, &v4, v5.clone(), v6, &v7, &v8, &v9);
            if v12 {
                Rc::new(UH1::UH1_Cons(v3, v4, v5, v6, v7, v8, v9, v11))
            } else {
                v11
            }
        }
        UH1::UH1_Nil => {
            v2
        }
    }
}
fn team_failures_of_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v2: Rc<UH1> = facts_list_6(v1);
    let v3: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH1> = method29(v0, v2, v3);
    let mut v5: Rc<UH0> = open_failures_in_2(v4);
    let v6: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v7: i32 = 0i32;
    let mut v8: i32 = method8(v6.clone(), v5, v7);
    let mut v9: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v6.borrow().clone());
    let v10: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v9).as_ref().clone()));
    v10
}
fn closure7() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        team_failures_of_28(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_lint_failures(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_lint_census(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, i32)>>> {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_agile_lint_item_failures(v0: &str, v1: &str, v2: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_agile_cadence_rows(v0: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i64)>>> {
    closure4()(Rc::<str>::from(v0))
}
pub fn kit_agile_stamp_hour(v0: &str) -> i64 {
    closure6()(Rc::<str>::from(v0))
}
pub fn kit_agile_lint_team_failures(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure7()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_4fa472513c6d = Mut0;
