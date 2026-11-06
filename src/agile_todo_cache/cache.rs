#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(i32),
    US1_None,
}
fn ids_in_0(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id\":\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v2;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            if v9 {
                return v2;
            } else {
                let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
                let v11: i32 = v8.wrapping_add(1i32);
                let v12: bool = &*v2 == "";
                let v15: Rc<str> = if v12 {
                    v10
                } else {
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v10));
                    v14
                };
                (v1, v2) = (v11, v15);
                continue;
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        ids_in_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn method4(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_3(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_5(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method4(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn cache_row_1(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v0.replace(&*v6, &*v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(v8.replace(&*v5, &*v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v16.replace(&*v17, &*v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = concat5_2(v5.clone(), v19, v5.clone(), v20.clone(), v20.clone());
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl c () = Cached ("); } LIT.with(|lit| lit.clone()) }, v21));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v24: Rc<str> = format_real_3(v1);
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) }));
    let v27: Rc<str> = format_real_3(v2);
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) }));
    let v30: Rc<str> = Rc::<str>::from(v3.replace(&*v6, &*v7));
    let v31: Rc<str> = Rc::<str>::from(v30.replace(&*v5, &*v9));
    let v32: Rc<str> = Rc::<str>::from(v31.replace(&*v11, &*v12));
    let v33: Rc<str> = Rc::<str>::from(v32.replace(&*v14, &*v15));
    let v34: Rc<str> = Rc::<str>::from(v33.replace(&*v17, &*v18));
    let v35: Rc<str> = concat5_2(v5.clone(), v34, v5, v20.clone(), v20);
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v35));
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v38: Rc<str> = format_real_5(v4);
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) }));
    v40
}
fn closure1() -> Rc<dyn Fn(Rc<str>, u64, u64, Rc<str>, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, Rc<str>, i32) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32| -> Rc<str> {
        cache_row_1(v0, v1, v2, v3, v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn skipped_row_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v4.replace(&*v1, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v6.replace(&*v7, &*v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v9.replace(&*v10, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = concat5_2(v1.clone(), v15, v1, v16.clone(), v16);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl s () = Skipped ("); } LIT.with(|lit| lit.clone()) }, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
    v19
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        skipped_row_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method9(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_between_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (u64, i32) {
    let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
    let v4: bool = v3 < 0i32;
    if v4 {
        (0u64, -1i32)
    } else {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v3 as usize).unwrap_or(""));
        let v6: Option<u64> = (v5).trim().parse::<u64>().ok();
        let v7: Option<u64> = method9(v6);
        let mut v8: Rc<dyn Fn((u64)) -> US0> = closure4();
        let mut v9: Option<US0> = v7.map(|x| v8(x));
        let v11: US0 = US0::US0_None;
        let v12: US0 = v11.clone();
        let v13: US0 = v9.unwrap_or(v12);
        let v16: u64 = match v13 {
            US0::US0_None => {
                0u64
            }
            US0::US0_Some(v14) => {
                let v14: u64 = v14;
                v14
            }
        };
        (v16, v3)
    }
}
fn method10(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure5() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn row_of_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32) {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v2.replace(&*v4, &*v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v6.replace(&*v3, &*v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(v8.replace(&*v9, &*v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(v14.replace(&*v15, &*v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = concat5_2(v3.clone(), v17, v3.clone(), v18.clone(), v18.clone());
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) }, v19));
    let v21: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v20)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
    let v22: bool = v21 < 0i32;
    let v24: i32 = if v22 {
        let v23: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v20)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
        v23
    } else {
        v21
    };
    let v25: i32 = (v20.len() as i32);
    let v26: i32 = v24.wrapping_add(v25);
    let v27: bool = v24 < 8i32;
    if v27 {
        (0i32, 0u64, 0u64, v18, 0i32, -1i32)
    } else {
        let v28: i32 = v24.wrapping_sub(8i32);
        let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v24 as usize).unwrap_or(""));
        let v30: bool = &*v29 == "Skipped ";
        if v30 {
            let v31: i32 = v26.wrapping_add(1i32);
            (1i32, 0u64, 0u64, v18, 0i32, v31)
        } else {
            let v32: i32 = v24.wrapping_sub(7i32);
            let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v32 as usize..v24 as usize).unwrap_or(""));
            let v34: bool = &*v33 != "Cached ";
            if v34 {
                (0i32, 0u64, 0u64, v18, 0i32, -1i32)
            } else {
                let v35: i32 = v26.wrapping_add(2i32);
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
                let (mut v37, mut v38): (u64, i32) = number_between_8(v0.clone(), v35, v36);
                let v39: bool = v38 < 0i32;
                let (mut v44, mut v45): (u64, i32) = if v39 {
                    (0u64, -1i32)
                } else {
                    let v40: i32 = v38.wrapping_add(5i32);
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, \""); } LIT.with(|lit| lit.clone()) };
                    number_between_8(v0.clone(), v40, v41)
                };
                let v46: i32 = v45.wrapping_add(6i32);
                let v47: bool = v45 < 0i32;
                let v50: i32 = if v47 {
                    -1i32
                } else {
                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
                    let v49: i32 = v0.get(v46 as usize..).and_then(|rest| rest.find(&*v48)).map(|found| (found + v46 as usize) as i32).unwrap_or(-1);
                    v49
                };
                let v51: bool = v50 < 0i32;
                let v54: i32 = if v51 {
                    -1i32
                } else {
                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
                    let v53: i32 = v0.get(v50 as usize..).and_then(|rest| rest.find(&*v52)).map(|found| (found + v50 as usize) as i32).unwrap_or(-1);
                    v53
                };
                let v55: bool = v54 < 0i32;
                if v55 {
                    (0i32, 0u64, 0u64, v18, 0i32, -1i32)
                } else {
                    let v56: i32 = v50.wrapping_add(3i32);
                    let v57: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v54 as usize).unwrap_or(""));
                    let v58: Option<i32> = (v57).trim().parse::<i32>().ok();
                    let v59: Option<i32> = method10(v58);
                    let mut v60: Rc<dyn Fn((i32)) -> US1> = closure5();
                    let mut v61: Option<US1> = v59.map(|x| v60(x));
                    let v63: US1 = US1::US1_None;
                    let v64: US1 = v63.clone();
                    let v65: US1 = v61.unwrap_or(v64);
                    let v68: i32 = match v65 {
                        US1::US1_None => {
                            0i32
                        }
                        US1::US1_Some(v66) => {
                            let v66: i32 = v66;
                            v66
                        }
                    };
                    let v69: Rc<str> = std::rc::Rc::<str>::from(v0.get(v46 as usize..v50 as usize).unwrap_or(""));
                    (2i32, v37, v44, v69, v68, v54)
                }
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>| -> (i32, u64, u64, Rc<str>, i32, i32) {
        row_of_7(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn head_row_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Option<char> = char::from_u32(0u32);
    let v5: char = v3.unwrap();
    let v6: Rc<str> = Rc::<str>::from(format!("{}", v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v2.replace(&*v6, &*v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v0.replace(&*v10, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(v12.replace(&*v9, &*v13));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v14.replace(&*v7, &*v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(v16.replace(&*v17, &*v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(v19.replace(&*v20, &*v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = concat5_2(v9.clone(), v22, v9.clone(), v23.clone(), v23.clone());
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl h () = RootHead ("); } LIT.with(|lit| lit.clone()) }, v24));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v27: Rc<str> = Rc::<str>::from(v1.replace(&*v10, &*v11));
    let v28: Rc<str> = Rc::<str>::from(v27.replace(&*v9, &*v13));
    let v29: Rc<str> = Rc::<str>::from(v28.replace(&*v7, &*v15));
    let v30: Rc<str> = Rc::<str>::from(v29.replace(&*v17, &*v18));
    let v31: Rc<str> = Rc::<str>::from(v30.replace(&*v20, &*v21));
    let v32: Rc<str> = concat5_2(v9.clone(), v31, v9.clone(), v23.clone(), v23.clone());
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v32));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v35: Rc<str> = Rc::<str>::from(v8.replace(&*v10, &*v11));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v9, &*v13));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v7, &*v15));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v17, &*v18));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v20, &*v21));
    let v40: Rc<str> = concat5_2(v9.clone(), v39, v9, v23.clone(), v23);
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v40));
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
    v42
}
fn closure6() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        head_row_11(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn head_of_12(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(v1.replace(&*v3, &*v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(v5.replace(&*v2, &*v6));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v13.replace(&*v14, &*v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = concat5_2(v2.clone(), v16, v2.clone(), v17.clone(), v17.clone());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RootHead ("); } LIT.with(|lit| lit.clone()) }, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \""); } LIT.with(|lit| lit.clone()) }));
    let v21: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v20)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v22: bool = v21 < 0i32;
    if v22 {
        (v17.clone(), v17.clone())
    } else {
        let v23: i32 = (v20.len() as i32);
        let v24: i32 = v21.wrapping_add(v23);
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
        let v26: i32 = v0.get(v24 as usize..).and_then(|rest| rest.find(&*v25)).map(|found| (found + v24 as usize) as i32).unwrap_or(-1);
        let v27: bool = v26 < 0i32;
        let v31: i32 = if v27 {
            -1i32
        } else {
            let v28: i32 = v26.wrapping_add(4i32);
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
            let v30: i32 = v0.get(v28 as usize..).and_then(|rest| rest.find(&*v29)).map(|found| (found + v28 as usize) as i32).unwrap_or(-1);
            v30
        };
        let v32: bool = v31 < 0i32;
        if v32 {
            (v17.clone(), v17.clone())
        } else {
            let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v24 as usize..v26 as usize).unwrap_or(""));
            let v34: Option<char> = char::from_u32(0u32);
            let v36: char = v34.unwrap();
            let v37: Rc<str> = Rc::<str>::from(format!("{}", v36));
            let v38: i32 = v26.wrapping_add(4i32);
            let v39: Rc<str> = std::rc::Rc::<str>::from(v0.get(v38 as usize..v31 as usize).unwrap_or(""));
            let v40: Rc<str> = Rc::<str>::from(v39.replace(&*v9, &*v37));
            (v33, v40)
        }
    }
}
fn closure7() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (Rc<str>, Rc<str>) {
        head_of_12(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_ids_in(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_todo_cache_row(v0: &str, v1: u64, v2: u64, v3: &str, v4: i32) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3), v4)
}
pub fn kit_agile_todo_skipped_row(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn kit_agile_todo_row_of(v0: &str, v1: i32, v2: &str) -> (i32, u64, u64, Rc<str>, i32, i32) {
    closure3()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2))
}
pub fn kit_agile_todo_head_row(v0: &str, v1: &str, v2: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_agile_todo_head_of(v0: &str, v1: &str) -> (Rc<str>, Rc<str>) {
    closure7()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
