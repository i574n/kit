#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(regex::Regex),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Header(Rc<str>),
    US1_Carried,
    US1_Plain,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US1, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_First,
    US2_Last,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, US2, regex::Regex, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, US2, regex::Regex),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(Rc<str>),
    US4_None,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, US2, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<str>, Rc<UH4>),
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, i64, Rc<UH5>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(i64),
    US5_None,
}
#[derive(Clone)]
enum UH7 {
    UH7_Nil,
    UH7_Cons(Rc<str>, i64, Rc<str>, Rc<UH7>),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(Rc<str>, Rc<str>, Rc<UH7>, Rc<UH6>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(Rc<str>, US2),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Some(Rc<str>, Rc<str>, Rc<UH7>),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(i64, Rc<str>),
    US8_None,
}
struct Mut0 { l0: Rc<str> }
fn compiled_pattern_1(mut v0: Rc<str>) -> regex::Regex {
    let v2: Result<regex::Regex, regex::Error> = regex::Regex::new(&v0);
    let v4: regex::Regex = v2.unwrap();
    v4
}
fn method6(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    (v0, v1)
}
fn regex_captures_5(mut v0: Rc<str>, mut v1: regex::Regex) -> Vec<std::collections::HashMap<Rc<str>, Rc<str>>> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: bool = true; let _capture_move = (move || { //;
    let v9: Box<std::string::String> = Box::new(v5);
    let v11: &'static mut std::string::String = Box::leak(v9);
    let mut v13: regex::Regex = v1.clone();
    let v15: regex::CaptureMatches = v13.captures_iter(v11);
    let v17: bool = true; let _regex_captures : Vec<_> = v15.map(|x| { //;
    let v19: regex::Captures<'static> = x;
    let v21: Box<regex::Captures<'static>> = Box::new(v19);
    let mut v23: &'static mut regex::Captures<'static> = Box::leak(v21);
    let v25: regex::CaptureNames = v1.capture_names();
    let v27: bool = true; let _regex_captures : std::collections::HashMap<_, _> = v25.map(|x| { //;
    let v29: Rc<str> = x.unwrap_or("").to_string().into();
    let v31: Rc<str> = v29.clone();
    let v33: Rc<str> = v23.name(&v29).map(|x| x.as_str()).unwrap_or("").to_string().into();
    let (mut v34, mut v35): (Rc<str>, Rc<str>) = method6(v31.clone(), v33.clone());
    let v36: (Rc<str>, Rc<str>) = (v34, v35);
    let v38: bool = true; v36 }).collect();
    let v40: std::collections::HashMap<Rc<str>, Rc<str>> = _regex_captures;
    let v42: bool = true; v40 }).collect::<Vec<_>>();
    let v44: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _regex_captures;
    let v46: bool = true; v44 })();
    let v48: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = _capture_move;
    v48.clone()
}
fn method7(mut v0: Rc<str>) -> Rc<str> {
    v0
}
fn group_in_4(mut v0: Rc<str>, mut v1: regex::Regex, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_5(v0.clone(), v1.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = true; let _vec_fold_ = v3.into_iter().fold(v4, |acc, x| { //;
    let v8: Rc<str> = acc;
    let v10: std::collections::HashMap<Rc<str>, Rc<str>> = x;
    let v11: bool = &*v8 != "";
    let v18: Rc<str> = if v11 {
        v8.clone()
    } else {
        let v12: Rc<str> = method7(v2.clone());
        let mut v14: Option<Rc<str>> = std::collections::HashMap::get(&v10, &v12).map(|x| x).cloned();
        let v16: Rc<str> = v8.clone();
        let v17: Rc<str> = v14.unwrap_or(v16);
        v17.clone()
    };
    let v20: bool = true; v18 });
    let v22: Rc<str> = _vec_fold_;
    v22.clone()
}
fn matched_in_8(mut v0: Rc<str>, mut v1: regex::Regex) -> bool {
    let v2: Vec<std::collections::HashMap<Rc<str>, Rc<str>>> = regex_captures_5(v0, v1);
    let mut v3: usize = ((v2).len() as usize);
    let v4: u64 = (v3 as u64);
    let v5: bool = v4 > 0u64;
    v5
}
fn class_of_3(mut v0: Rc<str>, mut v1: US0, mut v2: US0) -> US1 {
    let v8: Rc<str> = match &v1 {
        US0::US0_None => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        }
        US0::US0_Some(v3) => {
            let mut v3: regex::Regex = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("section"); } LIT.with(|lit| lit.clone()) };
            group_in_4(v0.clone(), v3.clone(), v4.clone())
        }
    };
    let v9: bool = &*v8 != "";
    if v9 {
        US1::US1_Header(v8.clone())
    } else {
        match &v2 {
            US0::US0_None => {
                US1::US1_Plain
            }
            US0::US0_Some(v11) => {
                let v11: regex::Regex = v11.clone();
                let v12: bool = matched_in_8(v0.clone(), v11.clone());
                if v12 {
                    US1::US1_Carried
                } else {
                    US1::US1_Plain
                }
            }
        }
    }
}
fn classified_from_2(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: US0, mut v3: US0, mut v4: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v5: bool = v1 < 0i32;
        if v5 {
            return v4.clone();
        } else {
            let mut v6: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v7: i32 = v1.wrapping_sub(1i32);
            let v8: US1 = class_of_3(v6.clone(), v2.clone(), v3.clone());
            let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v6.clone(), v4.clone()));
            (v0, v1, v2, v3, v4) = (v0.clone(), v7, v2.clone(), v3.clone(), v9.clone());
            continue;
        }
    }
}
fn columns_from_9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        let mut v3: bool = v1 < 0i32;
        if v3 {
            return v2.clone();
        } else {
            let v4: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v7: i32 = (v6.clone().borrow().len() as i32);
            let v8: bool = v7 < 3i32;
            let v49: US3 = if v8 {
                US3::US3_None
            } else {
                let v10: Rc<RefCell<Vec<Rc<str>>>> = v6.clone() ;
                let v11: i32 = (v10.clone().borrow().len() as i32);
                let v12: bool = 0i32 < v11;
                let v16: US4 = if v12 {
                    let mut v13: Rc<str> = v10.clone().borrow()[0i32 as usize].clone();
                    US4::US4_Some(v13.clone())
                } else {
                    US4::US4_None
                };
                let v20: Rc<str> = match &v16 {
                    US4::US4_None => {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    }
                    US4::US4_Some(v17) => {
                        let v17: Rc<str> = v17.clone();
                        v17.clone()
                    }
                };
                let v21: bool = &*v20 == "last";
                let v24: US2 = if v21 {
                    US2::US2_Last
                } else {
                    US2::US2_First
                };
                let v25: Rc<RefCell<Vec<Rc<str>>>> = v6.clone() ;
                let v26: i32 = (v25.clone().borrow().len() as i32);
                let v27: bool = 1i32 < v26;
                let v31: US4 = if v27 {
                    let v28: Rc<str> = v25.clone().borrow()[1i32 as usize].clone();
                    US4::US4_Some(v28.clone())
                } else {
                    US4::US4_None
                };
                let v35: Rc<str> = match &v31 {
                    US4::US4_None => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v33.clone()
                    }
                    US4::US4_Some(v32) => {
                        let v32: Rc<str> = v32.clone();
                        v32.clone()
                    }
                };
                let v36: Rc<RefCell<Vec<Rc<str>>>> = v6 ;
                let v37: i32 = (v36.clone().borrow().len() as i32);
                let v38: bool = 2i32 < v37;
                let v42: US4 = if v38 {
                    let v39: Rc<str> = v36.clone().borrow()[2i32 as usize].clone();
                    US4::US4_Some(v39.clone())
                } else {
                    US4::US4_None
                };
                let v46: Rc<str> = match &v42 {
                    US4::US4_None => {
                        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v44.clone()
                    }
                    US4::US4_Some(v43) => {
                        let v43: Rc<str> = v43.clone();
                        v43.clone()
                    }
                };
                let v47: regex::Regex = compiled_pattern_1(v46.clone());
                US3::US3_Some(v35.clone(), v24.clone(), v47.clone())
            };
            match &v49 {
                US3::US3_None => {
                    let v56: i32 = v1.wrapping_sub(1i32);
                    (v0, v1, v2) = (v0.clone(), v56, v2.clone());
                    continue;
                }
                US3::US3_Some(v50, v51, v52) => {
                    let v50: Rc<str> = v50.clone();
                    let v51: US2 = v51.clone();
                    let v52: regex::Regex = v52.clone();
                    let v53: i32 = v1.wrapping_sub(1i32);
                    let v54: Rc<UH1> = Rc::new(UH1::UH1_Cons(v50.clone(), v51.clone(), v52.clone(), v2.clone()));
                    (v0, v1, v2) = (v0.clone(), v53, v54.clone());
                    continue;
                }
            }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: US2 = v3.clone();
            let v4: regex::Regex = v4.clone();
            let v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<UH2> = method10(v5, v1.clone());
            Rc::new(UH2::UH2_Cons(v2, v3, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method12(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH3> = v3.clone();
                let v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method13(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v1 {
            UH3::UH3_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH3> = v4.clone();
                let v5: Rc<UH4> = Rc::new(UH4::UH4_Cons(v0.clone(), v3.clone(), v2.clone()));
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v1 {
            UH3::UH3_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH3> = v4.clone();
                let v5: Rc<UH4> = Rc::new(UH4::UH4_Cons(v0.clone(), v3.clone(), v2.clone()));
                (v0, v1, v2) = (v0.clone(), v4.clone(), v5.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method15(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v0 {
            UH4::UH4_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH4> = v4.clone();
                let v5: Rc<UH4> = Rc::new(UH4::UH4_Cons(v2.clone(), v3.clone(), v1.clone()));
                (v0, v1) = (v4.clone(), v5.clone());
                continue;
            }
            UH4::UH4_Nil => {
                return v1.clone();
            }
        }
    }
}
fn placed_from_11(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<UH3>, mut v3: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v9, v10, v11) => {
                let v9: US1 = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: Rc<UH0> = v11.clone();
                match &v9 {
                    US1::US1_Carried => {
                        let v12: Rc<UH3> = Rc::new(UH3::UH3_Cons(v10.clone(), v2.clone()));
                        (v0, v1, v2, v3) = (v11.clone(), v1.clone(), v12.clone(), v3.clone());
                        continue;
                    }
                    US1::US1_Header(v14) => {
                        let mut v14: Rc<str> = v14.clone();
                        let v15: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v16: Rc<UH3> = method12(v2.clone(), v15.clone());
                        let v17: Rc<UH4> = method13(v14.clone(), v16.clone(), v3.clone());
                        let v18: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                        let v19: Rc<UH4> = Rc::new(UH4::UH4_Cons(v14.clone(), v10.clone(), v17.clone()));
                        (v0, v1, v2, v3) = (v11.clone(), v14.clone(), v18.clone(), v19.clone());
                        continue;
                    }
                    US1::US1_Plain => {
                        let v21: Rc<UH4> = Rc::new(UH4::UH4_Cons(v1.clone(), v10.clone(), v3.clone()));
                        (v0, v1, v2, v3) = (v11.clone(), v1.clone(), v2.clone(), v21.clone());
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v5: Rc<UH3> = method12(v2.clone(), v4.clone());
                let mut v6: Rc<UH4> = method14(v1.clone(), v5.clone(), v3.clone());
                let mut v7: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                return method15(v6.clone(), v7.clone());
            }
        }
    }
}
fn method18(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure1() -> Rc<dyn Fn((i64)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US5> = Rc::new(move |mut v0: (i64)| -> US5 {
        let mut v1: i64 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn signed_ms_in_17(mut v0: Rc<str>, mut v1: regex::Regex) -> i64 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ms"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = group_in_4(v0.clone(), v1.clone(), v2.clone());
    let v4: Option<i64> = (v3).trim().parse::<i64>().ok();
    let v5: Option<i64> = method18(v4.clone());
    let mut v6: Rc<dyn Fn((i64)) -> US5> = closure1();
    let mut v7: Option<US5> = v5.map(|x| v6(x));
    let v9: US5 = US5::US5_None;
    let v10: US5 = v9.clone();
    let v11: US5 = v7.unwrap_or(v10);
    match &v11 {
        US5::US5_None => {
            0i64
        }
        US5::US5_Some(v12) => {
            let v12: i64 = *v12;
            v12
        }
    }
}
fn method19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: Rc<UH1>, mut v4: Rc<UH5>) -> Rc<UH5> {
    loop {
        match &*v3 {
            UH1::UH1_Cons(v5, v6, v7, v8) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: US2 = v6.clone();
                let mut v7: regex::Regex = v7.clone();
                let v8: Rc<UH1> = v8.clone();
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = group_in_4(v1.clone(), v7.clone(), v9.clone());
                let v11: bool = &*v10 == "";
                let v15: Rc<UH5> = if v11 {
                    v4.clone()
                } else {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("value"); } LIT.with(|lit| lit.clone()) };
                    let mut v13: Rc<str> = group_in_4(v1.clone(), v7.clone(), v12.clone());
                    Rc::new(UH5::UH5_Cons(v0.clone(), v5.clone(), v10.clone(), v13.clone(), v2, v4.clone()))
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2, v8.clone(), v15.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v4.clone();
            }
        }
    }
}
fn method16(mut v0: Rc<UH1>, mut v1: regex::Regex, mut v2: Rc<UH4>, mut v3: Rc<UH5>) -> Rc<UH5> {
    loop {
        match &*v2 {
            UH4::UH4_Cons(v4, v5, v6) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH4> = v6.clone();
                let mut v7: i64 = signed_ms_in_17(v5.clone(), v1.clone());
                let v8: Rc<UH5> = method19(v4.clone(), v5.clone(), v7, v0.clone(), v3.clone());
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6.clone(), v8.clone());
                continue;
            }
            UH4::UH4_Nil => {
                return v3.clone();
            }
        }
    }
}
fn method20(mut v0: Rc<UH5>, mut v1: Rc<UH5>) -> Rc<UH5> {
    loop {
        match &*v0 {
            UH5::UH5_Cons(v2, v3, v4, v5, v6, v7) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: i64 = *v6;
                let mut v7: Rc<UH5> = v7.clone();
                let v8: Rc<UH5> = Rc::new(UH5::UH5_Cons(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v1.clone()));
                (v0, v1) = (v7.clone(), v8.clone());
                continue;
            }
            UH5::UH5_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method22(mut v0: Rc<str>, mut v1: Rc<UH2>) -> US6 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4, v5) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: US2 = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let mut v6: bool = &*v3 == &*v0;
                if v6 {
                    return US6::US6_Some(v3.clone(), v4.clone());
                } else {
                    (v0, v1) = (v0.clone(), v5.clone());
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return US6::US6_None;
            }
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH6>) -> US7 {
    loop {
        match &*v2 {
            UH6::UH6_Cons(v4, v5, v6, v7) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH7> = v6.clone();
                let mut v7: Rc<UH6> = v7.clone();
                let v8: bool = &*v4 == &*v0;
                let v10: bool = if v8 {
                    let v9: bool = &*v5 == &*v1;
                    v9
                } else {
                    false
                };
                if v10 {
                    return US7::US7_Some(v4.clone(), v5.clone(), v6.clone());
                } else {
                    (v0, v1, v2) = (v0.clone(), v1.clone(), v7.clone());
                    continue;
                }
            }
            UH6::UH6_Nil => {
                return US7::US7_None;
            }
        }
    }
}
fn cell_of_25(mut v0: Rc<UH7>, mut v1: Rc<str>) -> US8 {
    loop {
        match &*v0 {
            UH7::UH7_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: i64 = *v4;
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH7> = v6.clone();
                let mut v7: bool = &*v3 == &*v1;
                if v7 {
                    return US8::US8_Some(v4, v5.clone());
                } else {
                    (v0, v1) = (v6.clone(), v1.clone());
                    continue;
                }
            }
            UH7::UH7_Nil => {
                return US8::US8_None;
            }
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<UH7>, mut v2: Rc<UH7>) -> Rc<UH7> {
    match &*v1 {
        UH7::UH7_Cons(v3, v4, v5, v6) => {
            let mut v3: Rc<str> = v3.clone();
            let v4: i64 = *v4;
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH7> = v6.clone();
            let mut v7: Rc<UH7> = method26(v0.clone(), v6, v2.clone());
            let v8: bool = &*v3 != &*v0;
            if v8 {
                Rc::new(UH7::UH7_Cons(v3, v4, v5, v7.clone()))
            } else {
                v7
            }
        }
        UH7::UH7_Nil => {
            v2
        }
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i64, mut v5: i64, mut v6: Rc<str>, mut v7: US2, mut v8: Rc<UH6>, mut v9: Rc<UH6>) -> Rc<UH6> {
    match &*v8 {
        UH6::UH6_Cons(v10, v11, v12, v13) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = v11.clone();
            let v12: Rc<UH7> = v12.clone();
            let mut v13: Rc<UH6> = v13.clone();
            let mut v14: Rc<UH6> = method24(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4, v5, v6.clone(), v7.clone(), v13.clone(), v9.clone());
            let v15: bool = &*v10 == &*v0;
            let v17: bool = if v15 {
                let v16: bool = &*v11 == &*v2;
                v16
            } else {
                false
            };
            let (mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<UH7>) = if v17 {
                let v18: US8 = cell_of_25(v12.clone(), v1.clone());
                let (mut v23, mut v24): (i64, Rc<str>) = match &v18 {
                    US8::US8_Some(v19, v20) => {
                        let v19: i64 = *v19;
                        let v20: Rc<str> = v20.clone();
                        match &v7 {
                            US2::US2_First => {
                                (v19, v20.clone())
                            }
                            _ => {
                                (v5, v6.clone())
                            }
                        }
                    }
                    _ => {
                        (v5, v6.clone())
                    }
                };
                let v25: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                let v26: Rc<UH7> = method26(v1.clone(), v12.clone(), v25.clone());
                let v27: Rc<UH7> = Rc::new(UH7::UH7_Cons(v1.clone(), v23, v24.clone(), v26.clone()));
                (v10.clone(), v11.clone(), v27.clone())
            } else {
                (v10.clone(), v11.clone(), v12.clone())
            };
            Rc::new(UH6::UH6_Cons(v28.clone(), v29.clone(), v30.clone(), v14.clone()))
        }
        UH6::UH6_Nil => {
            v9.clone()
        }
    }
}
fn method27(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match &*v0 {
            UH6::UH6_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH7> = v4.clone();
                let v5: Rc<UH6> = v5.clone();
                let mut v6: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2.clone(), v3.clone(), v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH6::UH6_Nil => {
                return v1.clone();
            }
        }
    }
}
fn rows_from_21(mut v0: Rc<UH2>, mut v1: Rc<UH5>, mut v2: Rc<UH6>) -> Rc<UH6> {
    loop {
        match &*v1 {
            UH5::UH5_Cons(v5, v6, v7, v8, v9, v10) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: Rc<str> = v6.clone();
                let mut v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: i64 = *v9;
                let v10: Rc<UH5> = v10.clone();
                let v11: US6 = method22(v6.clone(), v0.clone());
                let v16: US2 = match &v11 {
                    US6::US6_None => {
                        US2::US2_First
                    }
                    US6::US6_Some(v12, v13) => {
                        let v12: Rc<str> = v12.clone();
                        let mut v13: US2 = v13.clone();
                        v13.clone()
                    }
                };
                let v17: US7 = method23(v5.clone(), v7.clone(), v2.clone());
                let v27: Rc<UH6> = match &v17 {
                    US7::US7_None => {
                        let v18: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
                        let v19: Rc<UH7> = Rc::new(UH7::UH7_Cons(v6.clone(), v9, v8.clone(), v18.clone()));
                        Rc::new(UH6::UH6_Cons(v5.clone(), v7.clone(), v19.clone(), v2.clone()))
                    }
                    US7::US7_Some(v21, v22, v23) => {
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let mut v23: Rc<UH7> = v23.clone();
                        let v24: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                        method24(v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9, v9, v8.clone(), v16.clone(), v2.clone(), v24.clone())
                    }
                };
                (v0, v1, v2) = (v0.clone(), v10.clone(), v27.clone());
                continue;
            }
            UH5::UH5_Nil => {
                let mut v3: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                return method27(v2.clone(), v3.clone());
            }
        }
    }
}
fn method31(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_30(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH7>, mut v3: Rc<UH2>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        match &*v3 {
            UH2::UH2_Cons(v5, v6, v7) => {
                let v5: Rc<str> = v5.clone();
                let mut v6: US2 = v6.clone();
                let mut v7: Rc<UH2> = v7.clone();
                let v8: US8 = cell_of_25(v2.clone(), v5.clone());
                let v20: Rc<str> = match &v8 {
                    US8::US8_None => {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t-\t-"); } LIT.with(|lit| lit.clone()) };
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v17));
                        v18.clone()
                    }
                    US8::US8_Some(v9, v10) => {
                        let v9: i64 = *v9;
                        let v10: Rc<str> = v10.clone();
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v11));
                        let mut v13: Rc<str> = format_real_30(v9);
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v11)), v10));
                        v16.clone()
                    }
                };
                (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v2.clone(), v7.clone(), v20.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v4.clone();
            }
        }
    }
}
fn method28(mut v0: Rc<UH2>, mut v1: Rc<UH6>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v1 {
            UH6::UH6_Cons(v3, v4, v5, v6) => {
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH7> = v5.clone();
                let mut v6: Rc<UH6> = v6.clone();
                let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v7)), v4));
                let v10: Rc<str> = method29(v3.clone(), v4.clone(), v5.clone(), v0.clone(), v9.clone());
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v10));
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                (v0, v1, v2) = (v0.clone(), v6.clone(), v13.clone());
                continue;
            }
            UH6::UH6_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: u64) -> u64 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4) => {
                let v2: US1 = v2.clone();
                let mut v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let mut v7: u64 = match &v2 {
                    US1::US1_Header(v5) => {
                        let v5: Rc<str> = v5.clone();
                        let mut v6: u64 = v1.wrapping_add(1u64);
                        v6
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v4.clone(), v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_33(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn sections_text_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let v8: i32 = v7.wrapping_sub(1i32);
    let v9: bool = &*v2 == "";
    let mut v13: US0 = if v9 {
        US0::US0_None
    } else {
        let v11: regex::Regex = compiled_pattern_1(v2.clone());
        US0::US0_Some(v11.clone())
    };
    let mut v14: bool = &*v3 == "";
    let v18: US0 = if v14 {
        US0::US0_None
    } else {
        let v16: regex::Regex = compiled_pattern_1(v3.clone());
        US0::US0_Some(v16.clone())
    };
    let v19: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH0> = classified_from_2(v6.clone(), v8, v13.clone(), v18.clone(), v19.clone());
    let v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v22: i32 = (v21.clone().borrow().len() as i32);
    let mut v23: i32 = v22.wrapping_sub(1i32);
    let v24: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v25: Rc<UH1> = columns_from_9(v21.clone(), v23, v24.clone());
    let v26: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH2> = method10(v25.clone(), v26.clone());
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v30: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v31: Rc<UH4> = placed_from_11(v20.clone(), v28.clone(), v29.clone(), v30.clone());
    let v32: regex::Regex = compiled_pattern_1(v1.clone());
    let v33: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v34: Rc<UH5> = method16(v25.clone(), v32.clone(), v31.clone(), v33.clone());
    let v35: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
    let v36: Rc<UH5> = method20(v34.clone(), v35.clone());
    let v37: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
    let v38: Rc<UH6> = rows_from_21(v27.clone(), v36.clone(), v37.clone());
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = method28(v27.clone(), v38.clone(), v39.clone());
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("#sections\t"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
    let v43: u64 = 0u64;
    let v44: u64 = method32(v20.clone(), v43);
    let v45: Rc<str> = format_real_33(v44);
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v45));
    v46.clone()
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>| -> Rc<str> {
        sections_text_0(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method35(mut v0: Rc<UH2>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3, v4) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: US2 = v3.clone();
                let v4: Rc<UH2> = v4.clone();
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v5)), v2));
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_time\t"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v8)), v2));
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_value"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                (v0, v1) = (v4.clone(), v12.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method36(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: US2 = v3.clone();
            let v4: regex::Regex = v4.clone();
            let v5: Rc<UH1> = v5.clone();
            let mut v6: Rc<UH2> = method36(v5, v1.clone());
            Rc::new(UH2::UH2_Cons(v2, v3, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn sections_header_34(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<UH1> = columns_from_9(v2.clone(), v4, v5.clone());
    let mut v7: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH2> = method36(v6.clone(), v7.clone());
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("section\tid"); } LIT.with(|lit| lit.clone()) };
    method35(v8.clone(), v9.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        sections_header_34(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_sequence_sections(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4))
}
pub fn kit_sequence_sections_header(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
