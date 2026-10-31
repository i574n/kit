#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
struct Mut0 { l0: Rc<str> }
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
            return v2.clone();
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let mut v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: bool = v8 < 0i32;
            if v9 {
                return v2.clone();
            } else {
                let v10: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
                let v11: i32 = v8.wrapping_add(1i32);
                let v12: bool = &*v2 == "";
                let v16: Rc<str> = if v12 {
                    v10.clone()
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v13)), v10));
                    v15.clone()
                };
                (v0, v1, v2) = (v0.clone(), v11, v16.clone());
                continue;
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: i32 = 0i32;
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        ids_in_0(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_2(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn format_real_4(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method3(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn cache_row_1(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v6, &v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from(v8.replace(&*v5, &v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from(v10.replace(&*v11, &v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from(v13.replace(&*v14, &v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v17, &v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v19, &*v5, &*v20, &*v20].concat());
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl c () = Cached ("); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = format_real_2(v1);
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
    let v30: Rc<str> = format_real_2(v2);
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v30)), v28));
    let v33: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v6, &v7));
    let v34: Rc<str> = std::rc::Rc::<str>::from(v33.replace(&*v5, &v9));
    let v35: Rc<str> = std::rc::Rc::<str>::from(v34.replace(&*v11, &v12));
    let v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v14, &v15));
    let v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v17, &v18));
    let v38: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v37, &*v5, &*v20, &*v20].concat());
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v38)), v24));
    let v41: Rc<str> = format_real_4(v4);
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v43));
    v44
}
fn closure1() -> Rc<dyn Fn(Rc<str>, u64, u64, Rc<str>, i32) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, Rc<str>, i32) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: i32| -> Rc<str> {
        cache_row_1(v0.clone(), v1, v2, v3.clone(), v4)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn skipped_row_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v2, &v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v1, &v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v7, &v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = std::rc::Rc::<str>::from(v9.replace(&*v10, &v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v13, &v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v15, &*v1, &*v16, &*v16].concat());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl s () = Skipped ("); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v17));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    v21
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        skipped_row_5(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure4() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn number_between_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (u64, i32) {
    let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
    let v4: bool = v3 < 0i32;
    if v4 {
        (0u64, -1i32)
    } else {
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v3 as usize).unwrap_or(""));
        let v6: Option<u64> = (v5).trim().parse::<u64>().ok();
        let v7: Option<u64> = method8(v6.clone());
        let mut v8: Rc<dyn Fn((u64)) -> US0> = closure4();
        let mut v9: Option<US0> = v7.map(|x| v8(x));
        let v11: US0 = US0::US0_None;
        let v12: US0 = v11.clone();
        let v13: US0 = v9.unwrap_or(v12);
        let v16: u64 = match &v13 {
            US0::US0_None => {
                0u64
            }
            US0::US0_Some(v14) => {
                let v14: u64 = *v14;
                v14
            }
        };
        (v16, v3)
    }
}
fn method9(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure5() -> Rc<dyn Fn((i32)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US1> = Rc::new(move |mut v0: (i32)| -> US1 {
        let v1: i32 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn row_of_6(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32) {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v4, &v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v6.replace(&*v3, &v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = std::rc::Rc::<str>::from(v8.replace(&*v9, &v10));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = std::rc::Rc::<str>::from(v11.replace(&*v12, &v13));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from(v14.replace(&*v15, &v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v17, &*v3, &*v18, &*v18].concat());
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("("); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
    let v22: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v21)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
    let v23: bool = v22 < 0i32;
    let v25: i32 = if v23 {
        let v24: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v21)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
        v24
    } else {
        v22
    };
    let v26: i32 = (v21.clone().len() as i32);
    let v27: i32 = v25.wrapping_add(v26);
    let v28: bool = v25 < 8i32;
    if v28 {
        (0i32, 0u64, 0u64, v18.clone(), 0i32, -1i32)
    } else {
        let v29: i32 = v25.wrapping_sub(8i32);
        let v30: Rc<str> = std::rc::Rc::<str>::from(v0.get(v29 as usize..v25 as usize).unwrap_or(""));
        let v31: bool = &*v30 == "Skipped ";
        if v31 {
            let v32: i32 = v27.wrapping_add(1i32);
            (1i32, 0u64, 0u64, v18.clone(), 0i32, v32)
        } else {
            let v33: i32 = v25.wrapping_sub(7i32);
            let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v33 as usize..v25 as usize).unwrap_or(""));
            let v35: bool = &*v34 != "Cached ";
            if v35 {
                (0i32, 0u64, 0u64, v18.clone(), 0i32, -1i32)
            } else {
                let v36: i32 = v27.wrapping_add(2i32);
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
                let (mut v38, mut v39): (u64, i32) = number_between_7(v0.clone(), v36, v37.clone());
                let v40: bool = v39 < 0i32;
                let (mut v45, mut v46): (u64, i32) = if v40 {
                    (0u64, -1i32)
                } else {
                    let v41: i32 = v39.wrapping_add(5i32);
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, \""); } LIT.with(|lit| lit.clone()) };
                    number_between_7(v0.clone(), v41, v42.clone())
                };
                let v47: i32 = v46.wrapping_add(6i32);
                let v48: bool = v46 < 0i32;
                let v51: i32 = if v48 {
                    -1i32
                } else {
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
                    let v50: i32 = v0.get(v47 as usize..).and_then(|rest| rest.find(&*v49)).map(|found| (found + v47 as usize) as i32).unwrap_or(-1);
                    v50
                };
                let v52: bool = v51 < 0i32;
                let v55: i32 = if v52 {
                    -1i32
                } else {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("i32)"); } LIT.with(|lit| lit.clone()) };
                    let v54: i32 = v0.get(v51 as usize..).and_then(|rest| rest.find(&*v53)).map(|found| (found + v51 as usize) as i32).unwrap_or(-1);
                    v54
                };
                let v56: bool = v55 < 0i32;
                if v56 {
                    (0i32, 0u64, 0u64, v18.clone(), 0i32, -1i32)
                } else {
                    let v57: i32 = v51.wrapping_add(3i32);
                    let v58: Rc<str> = std::rc::Rc::<str>::from(v0.get(v57 as usize..v55 as usize).unwrap_or(""));
                    let v59: Option<i32> = (v58).trim().parse::<i32>().ok();
                    let v60: Option<i32> = method9(v59.clone());
                    let mut v61: Rc<dyn Fn((i32)) -> US1> = closure5();
                    let mut v62: Option<US1> = v60.map(|x| v61(x));
                    let v64: US1 = US1::US1_None;
                    let v65: US1 = v64.clone();
                    let v66: US1 = v62.unwrap_or(v65);
                    let v69: i32 = match &v66 {
                        US1::US1_None => {
                            0i32
                        }
                        US1::US1_Some(v67) => {
                            let v67: i32 = *v67;
                            v67
                        }
                    };
                    let v70: Rc<str> = std::rc::Rc::<str>::from(v0.get(v47 as usize..v51 as usize).unwrap_or(""));
                    (2i32, v38, v45, v70.clone(), v69, v55)
                }
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>) -> (i32, u64, u64, Rc<str>, i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>| -> (i32, u64, u64, Rc<str>, i32, i32) {
        row_of_6(v0.clone(), v1, v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn head_row_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Option<char> = char::from_u32(0u32);
    let v5: char = v3.unwrap();
    let v6: Rc<str> = Rc::<str>::from(format!("{}", v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = Rc::<str>::from(v2.replace(&*v6, &*v7));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v10, &v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = std::rc::Rc::<str>::from(v12.replace(&*v9, &v13));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from(v14.replace(&*v7, &v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = std::rc::Rc::<str>::from(v16.replace(&*v17, &v18));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = std::rc::Rc::<str>::from(v19.replace(&*v20, &v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v22, &*v9, &*v23, &*v23].concat());
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl h () = RootHead ("); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let v29: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v10, &v11));
    let v30: Rc<str> = std::rc::Rc::<str>::from(v29.replace(&*v9, &v13));
    let v31: Rc<str> = std::rc::Rc::<str>::from(v30.replace(&*v7, &v15));
    let v32: Rc<str> = std::rc::Rc::<str>::from(v31.replace(&*v17, &v18));
    let v33: Rc<str> = std::rc::Rc::<str>::from(v32.replace(&*v20, &v21));
    let v34: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v33, &*v9, &*v23, &*v23].concat());
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v28, v34)), v27));
    let v37: Rc<str> = std::rc::Rc::<str>::from(v8.replace(&*v10, &v11));
    let v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v9, &v13));
    let v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v7, &v15));
    let v40: Rc<str> = std::rc::Rc::<str>::from(v39.replace(&*v17, &v18));
    let v41: Rc<str> = std::rc::Rc::<str>::from(v40.replace(&*v20, &v21));
    let v42: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v41, &*v9, &*v23, &*v23].concat());
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v42));
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
    v45
}
fn closure6() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> Rc<str> {
        head_row_10(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn head_of_11(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v3, &v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from(v5.replace(&*v2, &v6));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.replace(&*v8, &v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from(v10.replace(&*v11, &v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = std::rc::Rc::<str>::from(v13.replace(&*v14, &v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v16, &*v2, &*v17, &*v17].concat());
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("RootHead ("); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \""); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v24: bool = v23 < 0i32;
    if v24 {
        (v17.clone(), v17.clone())
    } else {
        let v25: i32 = (v22.clone().len() as i32);
        let v26: i32 = v23.wrapping_add(v25);
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", \""); } LIT.with(|lit| lit.clone()) };
        let v28: i32 = v0.get(v26 as usize..).and_then(|rest| rest.find(&*v27)).map(|found| (found + v26 as usize) as i32).unwrap_or(-1);
        let v29: bool = v28 < 0i32;
        let v33: i32 = if v29 {
            -1i32
        } else {
            let v30: i32 = v28.wrapping_add(4i32);
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\")"); } LIT.with(|lit| lit.clone()) };
            let v32: i32 = v0.get(v30 as usize..).and_then(|rest| rest.find(&*v31)).map(|found| (found + v30 as usize) as i32).unwrap_or(-1);
            v32
        };
        let v34: bool = v33 < 0i32;
        if v34 {
            (v17.clone(), v17.clone())
        } else {
            let v35: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
            let v36: Option<char> = char::from_u32(0u32);
            let v38: char = v36.unwrap();
            let v39: Rc<str> = Rc::<str>::from(format!("{}", v38));
            let v40: i32 = v28.wrapping_add(4i32);
            let v41: Rc<str> = std::rc::Rc::<str>::from(v0.get(v40 as usize..v33 as usize).unwrap_or(""));
            let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v9, &*v39));
            (v35.clone(), v42.clone())
        }
    }
}
fn closure7() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (Rc<str>, Rc<str>) {
        head_of_11(v0.clone(), v1.clone())
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
