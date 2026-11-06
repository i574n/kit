#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
struct SpiralUse(std::rc::Rc<dyn Fn()>);
impl Drop for SpiralUse {
    fn drop(&mut self) { (self.0)() }
}
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
    UH0_Cons(Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i64, Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<UH1>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, i64, Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(i64),
    US2_None,
}
pub struct Mut1 { pub l0: i32, pub l1: Rc<UH0> }
#[derive(Clone)]
enum US3 {
    US3_Added(Rc<str>),
    US3_Changed(Rc<str>),
    US3_Removed(Rc<str>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(US3, Rc<UH2>),
}
fn remote_arguments_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<UH0> {
    let v3: bool = &*v2 == "";
    let mut v6: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" --skip "); } LIT.with(|lit| lit.clone()) }, v2));
        v5
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit mirror manifest "); } LIT.with(|lit| lit.clone()) }, v1));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v6));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-o"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BatchMode=yes"); } LIT.with(|lit| lit.clone()) };
    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ConnectTimeout=12"); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v12));
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v0, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v14));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v15));
    let mut v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v16));
    Rc::new(UH0::UH0_Cons(v9, v17))
}
fn method2(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: bool = &*v2 == "";
                let mut v6: Rc<UH0> = if v4 {
                    v1
                } else {
                    Rc::new(UH0::UH0_Cons(v2, v1))
                };
                (v0, v1) = (v3, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Rc<RefCell<std::process::Command>>, mut v1: Rc<UH0>) -> () {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH0> = v3;
                drop(v0.borrow_mut().arg(&*v2));
                v1 = v3;
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method9(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn paths_8(mut v0: Rc<UH0>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v2: i32 = 0i32;
    let v3: i32 = method9(v1.clone(), v0, v2);
    let mut v4: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v4).as_ref().clone()));
    v5
}
fn refusal_7(mut v0: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v1);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v3);
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, v2, v4, v6, v7)
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Option<i64>) -> Option<i64> {
    v0
}
fn closure2() -> Rc<dyn Fn((i64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i64)) -> US2> = Rc::new(move |mut v0: (i64)| -> US2 {
        let mut v1: i64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn entry_of_line_12(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method13(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    let v14: Rc<str> = if v8 {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        let v10: i32 = v7.wrapping_add(1i32);
        let mut v11: i32 = (v0.clone().len() as i32);
        let mut v12: i32 = v11.wrapping_sub(1i32);
        let v13: Rc<str> = string_slice(&v0.clone(), v10 as i64, v12 as i64);
        v13
    };
    let v15: Option<usize> = v14.find(&*v1);
    let v16: Option<usize> = method13(v15);
    let mut v17: Option<i32> = v16.map(|x| v4(x));
    let v19: i32 = v17.unwrap_or(-1i32);
    let v20: bool = v7 <= 0i32;
    let v22: bool = if v20 {
        true
    } else {
        let v21: bool = v19 <= 0i32;
        v21
    };
    let v26: bool = if v22 {
        true
    } else {
        let v23: i32 = v19.wrapping_add(1i32);
        let v24: i32 = (v14.clone().len() as i32);
        let v25: bool = v23 >= v24;
        v25
    };
    if v26 {
        US1::US1_None
    } else {
        let v28: i32 = v7.wrapping_sub(1i32);
        let v29: Rc<str> = string_slice(&v0, 0i32 as i64, v28 as i64);
        let v30: i32 = v19.wrapping_add(1i32);
        let v31: i32 = (v14.clone().len() as i32);
        let v32: i32 = v31.wrapping_sub(1i32);
        let v33: Rc<str> = string_slice(&v14.clone(), v30 as i64, v32 as i64);
        let v34: i32 = v19.wrapping_sub(1i32);
        let v35: Rc<str> = string_slice(&v14, 0i32 as i64, v34 as i64);
        let v36: Option<i64> = (v35).trim().parse::<i64>().ok();
        let v37: Option<i64> = method14(v36);
        let mut v38: Rc<dyn Fn((i64)) -> US2> = closure2();
        let mut v39: Option<US2> = v37.map(|x| v38(x));
        let v41: US2 = US2::US2_None;
        let v42: US2 = v41.clone();
        let v43: US2 = v39.unwrap_or(v42);
        match v43 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v44) => {
                let v44: i64 = v44;
                US1::US1_Some(v33, v44, v29)
            }
        }
    }
}
fn method15(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i64 = v3;
                let mut v4: Rc<str> = v4;
                let mut v5: Rc<UH1> = v5;
                let mut v6: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn entries_of_11(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5;
                let mut v6: Rc<UH0> = v6;
                let v7: bool = &*v5 == "";
                if v7 {
                    v0 = v6;
                    continue;
                } else {
                    let mut v9: US1 = entry_of_line_12(v5);
                    match v9 {
                        US1::US1_None => {
                            return US0::US0_None;
                        }
                        US1::US1_Some(v10, v11, v12) => {
                            let v10: Rc<str> = v10;
                            let mut v11: i64 = v11;
                            let mut v12: Rc<str> = v12;
                            let v13: Rc<UH1> = Rc::new(UH1::UH1_Cons(v10, v11, v12, v1));
                            (v0, v1) = (v6, v13);
                            continue;
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v3: Rc<UH1> = method15(v1, v2);
                return US0::US0_Some(v3);
            }
        }
    }
}
fn method17(mut v0: i32, mut v1: &Rc<RefCell<Mut1>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1, v4))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: 0i32, l1: v8 }));
        while method17(v7, &v9) {
            let mut v11: i32 = v9.borrow().l0;
            let mut v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let mut v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn manifest_of_text_10(mut v0: Rc<str>) -> US0 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let mut v5: Rc<UH0> = split_list_16(v1, v4);
    let mut v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    entries_of_11(v5, v6)
}
fn method21(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let v2: US3 = v2;
                let v3: Rc<UH2> = v3;
                let mut v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn changes_from_20(mut v0: Rc<UH1>, mut v1: Rc<UH1>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v15, v16, v17, v18) => {
                let v15: Rc<str> = v15.clone();
                let v16: i64 = *v16;
                let mut v17: Rc<str> = v17.clone();
                let v18: Rc<UH1> = v18.clone();
                match &*v1 {
                    UH1::UH1_Cons(v23, v24, v25, v26) => {
                        let v23: Rc<str> = v23.clone();
                        let v24: i64 = *v24;
                        let v25: Rc<str> = v25.clone();
                        let v26: Rc<UH1> = v26.clone();
                        let v27: bool = &*v15 == &*v23;
                        if v27 {
                            let v28: bool = &*v17 == &*v25;
                            let v30: bool = if v28 {
                                let v29: bool = v16 == v24;
                                v29
                            } else {
                                false
                            };
                            let v33: Rc<UH2> = if v30 {
                                v2
                            } else {
                                let v31: US3 = US3::US3_Changed(v23);
                                Rc::new(UH2::UH2_Cons(v31, v2))
                            };
                            (v0, v1, v2) = (v18, v26, v33);
                            continue;
                        } else {
                            let v35: bool = &*v15 < &*v23;
                            let mut v38: i32 = if v35 {
                                -1i32
                            } else {
                                let v36: bool = &*v15 > &*v23;
                                if v36 {
                                    1i32
                                } else {
                                    0i32
                                }
                            };
                            let mut v39: bool = v38 == -1i32;
                            if v39 {
                                let v40: US3 = US3::US3_Removed(v15);
                                let v41: Rc<UH2> = Rc::new(UH2::UH2_Cons(v40, v2));
                                (v0, v2) = (v18, v41);
                                continue;
                            } else {
                                let v43: US3 = US3::US3_Added(v23);
                                let v44: Rc<UH2> = Rc::new(UH2::UH2_Cons(v43, v2));
                                (v1, v2) = (v26, v44);
                                continue;
                            }
                        }
                    }
                    UH1::UH1_Nil => {
                        let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v20: US3 = US3::US3_Removed(v15);
                        let v21: Rc<UH2> = Rc::new(UH2::UH2_Cons(v20, v2));
                        (v0, v1, v2) = (v18, v19, v21);
                        continue;
                    }
                }
            }
            UH1::UH1_Nil => {
                match Rc::unwrap_or_clone(v1) {
                    UH1::UH1_Cons(v5, v6, v7, v8) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: i64 = v6;
                        let v7: Rc<str> = v7;
                        let v8: Rc<UH1> = v8;
                        let mut v9: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                        let v10: US3 = US3::US3_Added(v5);
                        let mut v11: Rc<UH2> = Rc::new(UH2::UH2_Cons(v10, v2));
                        (v0, v1, v2) = (v9, v8, v11);
                        continue;
                    }
                    UH1::UH1_Nil => {
                        let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                        return method21(v2, v3);
                    }
                }
            }
        }
    }
}
fn manifest_changes_19(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH2> {
    let v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    changes_from_20(v0, v1, v2)
}
fn sorted_changes_22(mut v0: Rc<UH2>, mut v1: Rc<UH0>, mut v2: Rc<UH0>, mut v3: Rc<UH0>) -> (Rc<UH0>, Rc<UH0>, Rc<UH0>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v10, v11) => {
                let v10: US3 = v10;
                let mut v11: Rc<UH2> = v11;
                match v10 {
                    US3::US3_Added(v12) => {
                        let mut v12: Rc<str> = v12;
                        let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v1));
                        (v0, v1) = (v11, v13);
                        continue;
                    }
                    US3::US3_Changed(v17) => {
                        let mut v17: Rc<str> = v17;
                        let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v2));
                        (v0, v2) = (v11, v18);
                        continue;
                    }
                    US3::US3_Removed(v22) => {
                        let v22: Rc<str> = v22;
                        let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v3));
                        (v0, v3) = (v11, v23);
                        continue;
                    }
                }
            }
            UH2::UH2_Nil => {
                let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v5: Rc<UH0> = method3(v1, v4);
                let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let v7: Rc<UH0> = method3(v2, v6);
                let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                let mut v9: Rc<UH0> = method3(v3, v8);
                return (v5, v7, v9);
            }
        }
    }
}
fn closure3() -> Rc<dyn Fn((u8)) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u8)) -> Rc<str>> = Rc::new(move |mut v0: (u8)| -> Rc<str> {
        let mut v1: u8 = (v0);
        let v3: std::string::String = format!("{:02x}", v1);
        let mut v5: Rc<str> = Rc::<str>::from(String::as_str(&v3));
        v5
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method25(mut v0: Vec<Rc<str>>) -> Vec<Rc<str>> {
    v0
}
fn method26(mut v0: Rc<Vec<Rc<str>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: usize = ((v1) as usize);
            let mut v5: Rc<str> = (v0)[v4].clone();
            let mut v6: i32 = v1.wrapping_sub(1i32);
            let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v2));
            (v1, v2) = (v6, v7);
            continue;
        }
    }
}
fn method27() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    v0
}
fn hash_text_24(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(v0.as_bytes().to_vec()));
    let v2: Vec<u8> = (v1).borrow().clone();
    let mut v4: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(v2);
    let mut v6: std::io::BufReader<std::io::Cursor<Vec<u8>>> = std::io::BufReader::new(v4);
    let v8: bool = true; let mut v6 = v6;
    let v10: bool = true; let result : sha2::Sha256 = sha2::Digest::new();
    let mut v12: sha2::Sha256 = result;
    let v14: bool = true; let mut v12 = v12;
    let v15: usize = ((0i32) as usize);
    let mut v17: _ = [0u8; 1024i32 as usize];
    let v19: bool = true; loop { // rust.loop 1;
    let v21: bool = true; let mut v17 = v17;
    let v23: Result<usize, std::io::Error> = std::io::Read::read(&mut v6, &mut v17);
    let v25: usize = v23.unwrap();
    let v26: bool = v25 == v15 ;
    let v29: bool = if v26 {
        let v28: bool = true; break ();
        true
    } else {
        false
    };
    let v30: usize = ((v25) as usize);
    let v31: usize = (v30.clone());
    let v33: usize = v17.len();
    let v34: bool = v31 == v33 ;
    let mut v39: &_ = if v34 {
        let v36: &_ = &v17[v15..];
        v36
    } else {
        let mut v38: &_ = &v17[v15..v30];
        v38
    };
    let v41: bool = true; sha2::Digest::update(&mut v12, v39);
    let v43: bool = true; } // rust.loop 3;
    let v45: &[u8] = &sha2::Digest::finalize(v12);
    let v47: Vec<u8> = v45.iter().map(|x| *x).collect::<Vec<_>>();
    let mut v49: Rc<dyn Fn((u8)) -> Rc<str>> = closure3();
    let v50: Vec<Rc<str>> = v47.iter().map(|x| v49(x.clone())).collect::<Vec<_>>();
    let v51: Vec<Rc<str>> = method25(v50);
    let v52: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v51));
    let v53: Rc<Vec<Rc<str>>> = Rc::new((v52).borrow().clone());
    let v54: usize = (v53).len();
    let v55: i32 = ((v54) as i32);
    let v56: i32 = v55.wrapping_sub(1i32);
    let v57: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v58: Rc<UH0> = method26(v53, v56, v57);
    let v59: Rc<str> = method27();
    let v60: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v61: i32 = 0i32;
    let v62: i32 = method9(v60.clone(), v58, v61);
    let v63: Rc<Vec<Rc<str>>> = Rc::new(v60.borrow().clone());
    let v64: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v63).as_ref().clone()));
    let v65: Rc<str> = Rc::<str>::from((*v64.borrow()).join(&*v59));
    let v66: Result<Rc<str>, std::io::Error> = Ok::<Rc<str>, std::io::Error>(v65);
    let v68: Rc<str> = v66.unwrap();
    v68
}
fn format_real_30(mut v0: i64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method29(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: i64 = v3;
            let mut v4: Rc<str> = v4;
            let mut v5: Rc<UH1> = v5;
            let mut v6: Rc<UH0> = method29(v5, v1);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v8: Rc<str> = format_real_30(v3);
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let mut v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            Rc::new(UH0::UH0_Cons(v12, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn manifest_text_28(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v2: Rc<UH0> = method29(v0, v1);
    let v3: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = method9(v3.clone(), v2, v4);
    let mut v6: Rc<Vec<Rc<str>>> = Rc::new(v3.borrow().clone());
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v6).as_ref().clone()));
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from((*v7.borrow()).join(&*v8));
    v9
}
fn manifest_fingerprint_23(mut v0: Rc<UH1>) -> Rc<str> {
    let mut v1: Rc<str> = manifest_text_28(v0);
    hash_text_24(v1)
}
fn compared_18(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) {
    let v2: Rc<UH2> = manifest_changes_19(v0.clone(), v1);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7, mut v8): (Rc<UH0>, Rc<UH0>, Rc<UH0>) = sorted_changes_22(v2, v3, v4, v5);
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v6);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v7);
    let mut v11: Rc<RefCell<Vec<Rc<str>>>> = paths_8(v8);
    let mut v12: Rc<str> = manifest_fingerprint_23(v0);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v13, v9, v10, v11, v12)
}
fn plan_of_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
    let mut v4: Rc<UH0> = remote_arguments_1(v3, v2, v1);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<RefCell<std::process::Command>> = Rc::new(RefCell::new(std::process::Command::new(&*v5)));
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method2(v4, v7);
    let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method3(v8, v9);
    method4(v6.clone(), v10);
    let mut v11: Rc<std::io::Result<std::process::Output>> = Rc::new(v6.borrow_mut().output());
    let mut v12: bool = v11.is_ok();
    let (mut v17, mut v18, mut v19): (bool, i32, Rc<str>) = if v12 {
        let v13: Rc<str> = Rc::<str>::from(String::from_utf8_lossy(&(&*v11).as_ref().unwrap().stdout).as_ref());
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }));
        let v15: i32 = (&*v11).as_ref().unwrap().status.code().unwrap_or(-2);
        (true, v15, v14)
    } else {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, -1i32, v16)
    };
    let v21: bool = if v17 {
        let v20: bool = v18 == 0i32;
        v20
    } else {
        false
    };
    let v22: bool = v21 == false;
    let (mut v71, mut v72, mut v73, mut v74, mut v75): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = if v22 {
        let v23: Rc<str> = format_real_5(v18);
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ssh or the remote kit failed, exit "); } LIT.with(|lit| lit.clone()) }, v23));
        let (mut v25, mut v26, mut v27, mut v28, mut v29): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = refusal_7(v24);
        (v25, v26, v27, v28, v29)
    } else {
        let v30: US0 = manifest_of_text_10(v19);
        let (mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = match &v30 {
            US0::US0_None => {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the remote manifest has a line that is not '<sha256> <bytes> <path>'"); } LIT.with(|lit| lit.clone()) };
                refusal_7(v31)
            }
            _ => {
                let (mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = match v30 {
                    US0::US0_Some(v37) => {
                        let v37: Rc<UH1> = v37;
                        let mut v38: US0 = manifest_of_text_10(v0);
                        match &v38 {
                            US0::US0_Some(v39) => {
                                let mut v39: Rc<UH1> = v39.clone();
                                compared_18(v37, v39)
                            }
                            _ => {
                                let (mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>) = match v38 {
                                    US0::US0_None => {
                                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the local manifest did not read back"); } LIT.with(|lit| lit.clone()) };
                                        refusal_7(v45)
                                    }
                                    _ => unreachable!(),
                                };
                                (v51, v52, v53, v54, v55)
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                (v61, v62, v63, v64, v65)
            }
        };
        (v66, v67, v68, v69, v70)
    };
    Rc::new(RefCell::new(vec![(v71, v72, v73, v74, v75)]))
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
        plan_of_0(v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_mirror_plan_of(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>, Rc<str>)>>> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
pub type SpiralShape_652accfc6a8d = Mut1;
