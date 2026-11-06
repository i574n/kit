#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, bool, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_NodeBusy(Rc<UH0>),
    US0_NodeIdle(i32),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>, Rc<str>, bool),
    US1_None,
}
pub struct Mut1 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn method3(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_2(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method4(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method3(v16, v17)
    }
}
fn text_field_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("="); } LIT.with(|lit| lit.clone()) }));
    let v4: i32 = v0.get(0i32 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + 0i32 as usize) as i32).unwrap_or(-1);
    let v5: bool = v4 < 0i32;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let mut v7: i32 = (v1.len() as i32);
        let v8: i32 = v4.wrapping_add(v7);
        let mut v9: i32 = v8.wrapping_add(2i32);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v11: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
        let v12: bool = v11 < 0i32;
        let v14: i32 = if v12 {
            let v13: i32 = (v0.clone().len() as i32);
            v13
        } else {
            v11
        };
        let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v9 as usize..v14 as usize).unwrap_or(""));
        v15
    }
}
fn lane_of_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<UH1> = split_list_2(v1, v0.clone());
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH1> = v6;
                    v3
                }
                _ => {
                    v0
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn job_of_6(mut v0: Rc<str>) -> US1 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = text_field_7(v0.clone(), v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = text_field_7(v0.clone(), v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job list ok "); } LIT.with(|lit| lit.clone()) };
    let v6: bool = v0.starts_with(&*v5);
    let v10: bool = if v6 {
        let mut v7: bool = &*v2 == "running";
        if v7 {
            true
        } else {
            let v8: bool = &*v2 == "queued";
            v8
        }
    } else {
        false
    };
    if v10 {
        let v11: Rc<str> = lane_of_8(v4.clone());
        let v12: bool = &*v2 == "running";
        US1::US1_Some(v11, v4, v12)
    } else {
        US1::US1_None
    }
}
fn method5(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US1 = job_of_6(v2);
                let v10: Rc<UH0> = match v4 {
                    US1::US1_None => {
                        v1
                    }
                    US1::US1_Some(v5, v6, v7) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let mut v7: bool = v7;
                        Rc::new(UH0::UH0_Cons(v5, v6, v7, v1))
                    }
                };
                (v0, v1) = (v3, v10);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method9(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<UH0> = v5;
                let v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v1));
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: bool = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method10(v5, v1);
            if v4 {
                Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method11(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<UH0> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5, v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn activity_of_1(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_2(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method5(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method9(v7, v8);
    let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v11: Rc<UH0> = method10(v9.clone(), v10);
    let v12: i32 = 0i32;
    let v13: i32 = method11(v11, v12);
    let v14: i32 = 0i32;
    let v15: i32 = method11(v9.clone(), v14);
    let v16: bool = v13 > 0i32;
    if v16 {
        US0::US0_NodeBusy(v9)
    } else {
        US0::US0_NodeIdle(v15)
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: bool = v5;
            let v6: Rc<UH0> = v6;
            let mut v7: Rc<UH0> = method14(v0.clone(), v6, v2);
            let v8: bool = &*v3 == &*v0;
            if v8 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v7))
            } else {
                v7
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method15(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: bool = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method15(v5, v1);
            if v4 {
                Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
            } else {
                v6
            }
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method17(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_16(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn method13(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH1>, mut v3: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v4, v5) => {
            let v4: Rc<str> = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH1> = method13(v0.clone(), v1.clone(), v5, v3);
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = method14(v4.clone(), v1, v7);
            let mut v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v10: Rc<UH0> = method15(v8.clone(), v9);
            let v11: i32 = 0i32;
            let v12: i32 = method11(v10, v11);
            let v13: i32 = 0i32;
            let v14: i32 = method11(v8, v13);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status jobs machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lane="); } LIT.with(|lit| lit.clone()) }));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v4));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" running="); } LIT.with(|lit| lit.clone()) }));
            let v19: Rc<str> = format_real_16(v12);
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" queued="); } LIT.with(|lit| lit.clone()) }));
            let v22: i32 = v14.wrapping_sub(v12);
            let v23: Rc<str> = format_real_16(v22);
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v23));
            Rc::new(UH1::UH1_Cons(v24, v6))
        }
        UH1::UH1_Nil => {
            v3
        }
    }
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US2> {
    Rc::new(move |mut v2: Rc<UH1>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US2> = method19(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method19(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US2> {
    closure1(v0, v1)
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: bool = v4;
                let v5: Rc<UH0> = v5;
                let v6: i32 = 0i32;
                let mut v7: Rc<dyn Fn(Rc<UH1>) -> US2> = method19(v2.clone(), v6);
                let v8: US2 = v7(v1.clone());
                let mut v9: bool = match v8 {
                    US2::US2_None => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v10: bool = v9 != true;
                let v12: Rc<UH1> = if v10 {
                    v1
                } else {
                    Rc::new(UH1::UH1_Cons(v2, v1))
                };
                (v0, v1) = (v5, v12);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn activity_lines_12(mut v0: Rc<str>, mut v1: US0) -> Rc<UH1> {
    match v1 {
        US0::US0_NodeBusy(v10) => {
            let v10: Rc<UH0> = v10;
            let v11: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v12: Rc<UH1> = method18(v10.clone(), v11);
            let v13: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH1> = method3(v12, v13);
            let v15: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            method13(v0, v10, v14, v15)
        }
        US0::US0_NodeIdle(v2) => {
            let v2: i32 = v2;
            let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit remote status IDLE machine="); } LIT.with(|lit| lit.clone()) }, v0));
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" running=0 queued="); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<str> = format_real_16(v2);
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (start the top node-runnable item: kit agile next <backlog root> --tag node --open 3)"); } LIT.with(|lit| lit.clone()) }));
            let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v7, v8))
        }
    }
}
fn method20(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH1> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn activity_text_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = activity_of_1(v0);
    let v3: Rc<UH1> = activity_lines_12(v1, v2);
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v5: i32 = 0i32;
    let v6: i32 = method20(v4.clone(), v3, v5);
    let mut v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
    v10
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        activity_text_0(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn field_text_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    text_field_7(v1, v0)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        field_text_21(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_remote_activity_text(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_remote_field_text(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
