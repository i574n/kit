#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_SprintOpen,
    US0_SprintClosed,
}
#[derive(Clone)]
enum US1 {
    US1_Committed,
    US1_Dropped,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, i32, US1, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US0, Rc<UH1>, Rc<UH0>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool, Rc<UH2>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>, Rc<str>, Rc<str>, Rc<str>, US0, Rc<UH1>),
    US2_None,
}
pub struct Mut0 { pub l0: i32 }
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(i32, i32, i32, bool, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(i32, i32, i32, Rc<UH4>),
}
fn method3(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US0 = v6;
                let v7: Rc<UH1> = v7;
                let v8: Rc<UH0> = v8;
                let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v1));
                (v0, v1) = (v8, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method4(mut v0: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>, mut v1: i32, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool) = (v0)[v4].clone();
            let v13: i32 = v1.wrapping_sub(1i32);
            let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v5, v6, v7, v8, v9, v10, v11, v12, v2));
            (v1, v2) = (v13, v14);
            continue;
        }
    }
}
fn method6(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3, v4, v5) => {
            let v2: Rc<str> = v2;
            let v3: i32 = v3;
            let v4: US1 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH1> = method6(v5, v1);
            Rc::new(UH1::UH1_Cons(v2, v3, v4, v6))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn method5(mut v0: Rc<UH2>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3, v4, v5, v6, v7, v8, v9, v10) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: bool = v6;
                let v7: Rc<str> = v7;
                let v8: i32 = v8;
                let v9: bool = v9;
                let v10: Rc<UH2> = v10;
                let v13: US1 = if v9 {
                    US1::US1_Committed
                } else {
                    US1::US1_Dropped
                };
                let v14: bool = &*v7 == "";
                let v18: Rc<UH1> = if v14 {
                    { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
                } else {
                    let v16: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    Rc::new(UH1::UH1_Cons(v7, v8, v13, v16))
                };
                let v38: Rc<UH0> = match &*v1 {
                    UH0::UH0_Cons(v19, v20, v21, v22, v23, v24, v25) => {
                        let v19: Rc<str> = v19.clone();
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let v23: US0 = v23.clone();
                        let v24: Rc<UH1> = v24.clone();
                        let v25: Rc<UH0> = v25.clone();
                        let v26: bool = &*v19 == &*v2;
                        if v26 {
                            let v27: Rc<UH1> = method6(v24, v18);
                            Rc::new(UH0::UH0_Cons(v19, v20, v21, v22, v23, v27, v25))
                        } else {
                            let v31: US0 = if v6 {
                                US0::US0_SprintOpen
                            } else {
                                US0::US0_SprintClosed
                            };
                            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v31, v18, v1))
                        }
                    }
                    _ => {
                        let v36: US0 = if v6 {
                            US0::US0_SprintOpen
                        } else {
                            US0::US0_SprintClosed
                        };
                        Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v36, v18, v1))
                    }
                };
                (v0, v1) = (v10, v38);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn sprints_from_2(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>>) -> Rc<UH0> {
    let mut v1: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>> = Rc::new((v0).borrow().clone());
    let v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH2> = method4(v1, v4, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = method5(v6, v7);
    let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method3(v8, v9)
}
fn sprints_read_1(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, i32, bool)>>> = kit_agile_sprint_read::kit_agile_sprint_flat_rows(&v0);
    sprints_from_2(v1)
}
fn method8(mut v0: Rc<UH0>, mut v1: US2) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6, v7, v8) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: US0 = v6;
                let v7: Rc<UH1> = v7;
                let v8: Rc<UH0> = v8;
                let v20: US2 = match &v1 {
                    US2::US2_None => {
                        let v15: bool = match &v6 {
                            US0::US0_SprintOpen => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v15 {
                            US2::US2_Some(v2, v3, v4, v5, v6, v7)
                        } else {
                            US2::US2_None
                        }
                    }
                    US2::US2_Some(v9, v10, v11, v12, v13, v14) => {
                        let v9: Rc<str> = v9.clone();
                        let v10: Rc<str> = v10.clone();
                        let mut v11: Rc<str> = v11.clone();
                        let mut v12: Rc<str> = v12.clone();
                        let v13: US0 = v13.clone();
                        let v14: Rc<UH1> = v14.clone();
                        v1
                    }
                };
                (v0, v1) = (v8, v20);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn first_open_7(mut v0: Rc<UH0>) -> US2 {
    let mut v1: US2 = US2::US2_None;
    method8(v0, v1)
}
fn open_sprint_of_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<UH0> = sprints_read_1(v0);
    let v2: US2 = first_open_7(v1);
    match v2 {
        US2::US2_None => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US2::US2_Some(v3, v4, v5, v6, v7, v8) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: US0 = v7;
            let v8: Rc<UH1> = v8;
            v3
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        open_sprint_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US2 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6, v7, v8, v9) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<str> = v6;
                let v7: US0 = v7;
                let v8: Rc<UH1> = v8;
                let v9: Rc<UH0> = v9;
                let v10: bool = &*v3 == &*v0;
                if v10 {
                    return US2::US2_Some(v3, v4, v5, v6, v7, v8);
                } else {
                    v1 = v9;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return US2::US2_None;
            }
        }
    }
}
fn chosen_10(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US2 {
    let v2: bool = &*v1 == "";
    if v2 {
        first_open_7(v0)
    } else {
        method11(v1, v0)
    }
}
fn sprint_head_of_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> {
    let v2: Rc<UH0> = sprints_read_1(v1);
    let v3: US2 = chosen_10(v2, v0);
    match v3 {
        US2::US2_None => {
            Rc::new(RefCell::new(vec![]))
        }
        US2::US2_Some(v4, v5, v6, v7, v8, v9) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: US0 = v8;
            let v9: Rc<UH1> = v9;
            let v10: bool = match v8 {
                US0::US0_SprintOpen => {
                    true
                }
                _ => {
                    false
                }
            };
            Rc::new(RefCell::new(vec![(v4, v5, v6, v7, v10)]))
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> {
        sprint_head_of_9(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn method13(mut v0: Rc<UH1>, mut v1: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>) -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2;
                let v3: i32 = v3;
                let v4: US1 = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = match v4 {
                    US1::US1_Committed => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v7: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = Rc::new(RefCell::new(vec![(v2, v3, v6)]));
                let v8: i32 = (v1.clone().borrow().len() as i32);
                let v9: i32 = (v7.clone().borrow().len() as i32);
                let v10: i32 = v8.wrapping_add(v9);
                let mut v11: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = Rc::new(RefCell::new(vec![<(Rc<str>, i32, bool)>::default(); v10 as usize]));
                let mut v12: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32 }));
                while method14(v10, &v12) {
                    let v14: i32 = v12.borrow().l0;
                    let v15: bool = v14 < v8;
                    let (mut v23, mut v24, mut v25): (Rc<str>, i32, bool) = if v15 {
                        let (mut v16, mut v17, mut v18): (Rc<str>, i32, bool) = v1.borrow()[v14 as usize].clone();
                        (v16, v17, v18)
                    } else {
                        let v19: i32 = v14.wrapping_sub(v8);
                        let (mut v20, mut v21, mut v22): (Rc<str>, i32, bool) = v7.borrow()[v19 as usize].clone();
                        (v20, v21, v22)
                    };
                    v11.borrow_mut()[v14 as usize] = (v23, v24, v25);
                    let v26: i32 = v14.wrapping_add(1i32);
                    v12.borrow_mut().l0 = v26;
                    ()
                };
                (v0, v1) = (v5, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn sprint_items_of_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> {
    let v2: Rc<UH0> = sprints_read_1(v1);
    let v3: US2 = chosen_10(v2, v0);
    match v3 {
        US2::US2_None => {
            Rc::new(RefCell::new(vec![]))
        }
        US2::US2_Some(v4, v5, v6, v7, v8, v9) => {
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: US0 = v8;
            let v9: Rc<UH1> = v9;
            let v10: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = Rc::new(RefCell::new(vec![]));
            let mut v11: Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> = method13(v9, v10);
            v11
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> {
        sprint_items_of_12(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17(mut v0: i32, mut v1: Rc<UH3>, mut v2: i32, mut v3: i32) -> (i32, i32) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH3::UH3_Cons(v4, v5, v6, v7, v8) => {
                let v4: i32 = v4;
                let v5: i32 = v5;
                let v6: i32 = v6;
                let v7: bool = v7;
                let v8: Rc<UH3> = v8;
                let v9: bool = v2 < v0;
                let (mut v12, mut v13): (i32, i32) = if v9 {
                    let v10: i32 = v2.wrapping_add(1i32);
                    let mut v11: i32 = v3.wrapping_add(v4);
                    (v10, v11)
                } else {
                    (v2, v3)
                };
                (v1, v2, v3) = (v8, v12, v13);
                continue;
            }
            UH3::UH3_Nil => {
                return (v2, v3);
            }
        }
    }
}
fn velocity_16(mut v0: Rc<UH3>, mut v1: i32) -> i32 {
    let v2: i32 = 0i32;
    let v3: i32 = 0i32;
    let (mut v4, mut v5): (i32, i32) = method17(v1, v0, v2, v3);
    let v6: bool = v4 == 0i32;
    if v6 {
        0i32
    } else {
        let v7: i32 = v5.wrapping_div(v4);
        v7
    }
}
fn method19(mut v0: Rc<UH4>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match Rc::unwrap_or_clone(v0) {
        UH4::UH4_Cons(v2, v3, v4, v5) => {
            let v2: i32 = v2;
            let v3: i32 = v3;
            let v4: i32 = v4;
            let v5: Rc<UH4> = v5;
            let v6: Rc<UH3> = method19(v5, v1);
            let v7: bool = v3 == 0i32;
            Rc::new(UH3::UH3_Cons(v2, v3, v4, v7, v6))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn method20(mut v0: Rc<Vec<(i32, i32, i32)>>, mut v1: i32, mut v2: Rc<UH4>) -> Rc<UH4> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let v4: usize = ((v1) as usize);
            let (mut v5, mut v6, mut v7): (i32, i32, i32) = (v0)[v4].clone();
            let v8: i32 = v1.wrapping_sub(1i32);
            let v9: Rc<UH4> = Rc::new(UH4::UH4_Cons(v5, v6, v7, v2));
            (v1, v2) = (v8, v9);
            continue;
        }
    }
}
fn reviews_from_18(mut v0: Rc<RefCell<Vec<(i32, i32, i32)>>>) -> Rc<UH3> {
    let mut v1: Rc<Vec<(i32, i32, i32)>> = Rc::new((v0).borrow().clone());
    let v2: usize = (v1).len();
    let v3: i32 = ((v2) as i32);
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH4> = method20(v1, v4, v5);
    let v7: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method19(v6, v7)
}
fn method21(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH3::UH3_Cons(v2, v3, v4, v5, v6) => {
                let v2: i32 = v2;
                let v3: i32 = v3;
                let v4: i32 = v4;
                let v5: bool = v5;
                let v6: Rc<UH3> = v6;
                let v7: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH3::UH3_Nil => {
                return v1;
            }
        }
    }
}
fn velocity_of_15(mut v0: i32, mut v1: Rc<str>) -> i32 {
    let v2: Rc<RefCell<Vec<(i32, i32, i32)>>> = kit_agile_sprint_read::kit_agile_sprint_review_rows(&v1);
    let v3: Rc<UH3> = reviews_from_18(v2);
    let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH3> = method21(v3, v4);
    velocity_16(v5, v0)
}
fn closure3() -> Rc<dyn Fn(Rc<str>, i32) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: i32| -> i32 {
        velocity_of_15(v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_open_sprint(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_agile_sprint_head(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool)>>> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_sprint_items(v0: &str, v1: &str) -> Rc<RefCell<Vec<(Rc<str>, i32, bool)>>> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_sprint_velocity(v0: &str, v1: i32) -> i32 {
    closure3()(Rc::<str>::from(v0), v1)
}
pub type SpiralShape_2617b4e49837 = Mut0;
