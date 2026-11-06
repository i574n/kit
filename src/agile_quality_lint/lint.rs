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
enum US0 {
    US0_PlanningFields,
    US0_DoneDefinition,
    US0_NextStep,
    US0_TypeFeatures,
    US0_DoneEvidence,
    US0_BlockerNamed,
    US0_PauseResume,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US0, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<UH2>),
}
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH2> }
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("team:"); } LIT.with(|lit| lit.clone()) };
        let mut v2: bool = v0.starts_with(&*v1);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure1()
}
fn method5(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method6();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = v0.contains(&*v1);
        v2
    })
}
fn method9(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method8(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v2) {
            UH2::UH2_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let v4: Rc<UH2> = v4;
                let v5: bool = v1(v3);
                if v5 {
                    return true;
                } else {
                    let v6: Rc<dyn Fn(Rc<str>) -> bool> = method9(v0.clone());
                    (v1, v2) = (v6, v4);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn says_7(mut v0: Rc<UH2>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(v1.to_lowercase());
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method9(v2.clone());
    method8(v2, v3, v0)
}
fn method11(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn method12(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH2::UH2_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method12(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH2> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH2> = Rc::new(UH2::UH2_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH2> = v9.borrow().l1.clone();
        let v17: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
        method11(v16, v17)
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
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
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn method13(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match Rc::unwrap_or_clone(v0) {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH2> = v3;
            let v4: Rc<UH2> = method13(v3, v1);
            let v5: i32 = (v2.clone().len() as i32);
            let v6: i32 = 0i32;
            let v7: i32 = method14(v2.clone(), v5, v6);
            let v8: i32 = v5.wrapping_sub(1i32);
            let mut v9: Rc<str> = string_slice(&v2, v7 as i64, v8 as i64);
            let v10: i32 = (v9.clone().len() as i32);
            let v11: i32 = method15(v9.clone(), v10);
            let v12: Rc<str> = string_slice(&v9, 0i32 as i64, v11 as i64);
            Rc::new(UH2::UH2_Cons(v12, v4))
        }
        UH2::UH2_Nil => {
            v1
        }
    }
}
fn markers_of_16(mut v0: US0) -> Rc<UH2> {
    match v0 {
        US0::US0_BlockerNamed => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocked on"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("blocker:"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits on"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v31: Rc<UH2> = Rc::new(UH2::UH2_Cons(v29, v30));
            let v32: Rc<UH2> = Rc::new(UH2::UH2_Cons(v28, v31));
            Rc::new(UH2::UH2_Cons(v27, v32))
        }
        US0::US0_DoneDefinition => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dod:"); } LIT.with(|lit| lit.clone()) };
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("definition of done"); } LIT.with(|lit| lit.clone()) };
            let v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v4: Rc<UH2> = Rc::new(UH2::UH2_Cons(v2, v3));
            Rc::new(UH2::UH2_Cons(v1, v4))
        }
        US0::US0_DoneEvidence => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit 0"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" pass"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("green"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proof:"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("proven"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH2> = Rc::new(UH2::UH2_Cons(v20, v21));
            let v23: Rc<UH2> = Rc::new(UH2::UH2_Cons(v19, v22));
            let v24: Rc<UH2> = Rc::new(UH2::UH2_Cons(v18, v23));
            let v25: Rc<UH2> = Rc::new(UH2::UH2_Cons(v17, v24));
            Rc::new(UH2::UH2_Cons(v16, v25))
        }
        US0::US0_NextStep => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next:"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next step"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let mut v9: Rc<UH2> = Rc::new(UH2::UH2_Cons(v7, v8));
            Rc::new(UH2::UH2_Cons(v6, v9))
        }
        US0::US0_PauseResume => {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume when"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume:"); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resumes on"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume from"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v39: Rc<UH2> = Rc::new(UH2::UH2_Cons(v37, v38));
            let v40: Rc<UH2> = Rc::new(UH2::UH2_Cons(v36, v39));
            let v41: Rc<UH2> = Rc::new(UH2::UH2_Cons(v35, v40));
            Rc::new(UH2::UH2_Cons(v34, v41))
        }
        US0::US0_PlanningFields => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_TypeFeatures => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typefeatures:"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("typefeatureset plan"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            let v14: Rc<UH2> = Rc::new(UH2::UH2_Cons(v12, v13));
            Rc::new(UH2::UH2_Cons(v11, v14))
        }
    }
}
fn passes_4(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: US0) -> bool {
    match &v6 {
        US0::US0_PlanningFields => {
            let v7: bool = v2 > 0i32;
            let mut v9: bool = if v7 {
                let v8: bool = &*v3 != "";
                v8
            } else {
                false
            };
            let v11: bool = if v9 {
                let v10: bool = &*v4 != "";
                v10
            } else {
                false
            };
            if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<UH2> = split_list_10(v12, v1);
                let v14: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v15: Rc<UH2> = method13(v13, v14);
                let v16: Rc<dyn Fn(Rc<str>) -> bool> = method6();
                method5(v16, v15)
            } else {
                false
            }
        }
        _ => {
            let v19: Rc<UH2> = markers_of_16(v6);
            says_7(v19, v5)
        }
    }
}
fn method3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<UH1>, mut v7: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v6) {
        UH1::UH1_Cons(v8, v9) => {
            let v8: US0 = v8;
            let mut v9: Rc<UH1> = v9;
            let v10: Rc<UH1> = method3(v0.clone(), v1.clone(), v2, v3.clone(), v4.clone(), v5.clone(), v9, v7);
            let v11: bool = passes_4(&v0, v1, v2, v3, v4, v5, v8.clone());
            let v12: bool = v11 == false;
            if v12 {
                Rc::new(UH1::UH1_Cons(v8, v10))
            } else {
                v10
            }
        }
        UH1::UH1_Nil => {
            v7
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: bool = &*v0 == "kit";
        let v3: bool = if v1 {
            true
        } else {
            let mut v2: bool = &*v0 == "lib";
            v2
        };
        if v3 {
            true
        } else {
            let v4: bool = &*v0 == "compiler";
            v4
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure3()
}
fn method19(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<UH2>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH2> = v3;
                let v4: bool = v0(v2);
                if v4 {
                    return true;
                } else {
                    let v5: Rc<dyn Fn(Rc<str>) -> bool> = method18();
                    (v0, v1) = (v5, v3);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return false;
            }
        }
    }
}
fn checks_for_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: bool = &*v0 == "Done";
    if v2 {
        let v3: US0 = US0::US0_DoneEvidence;
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v3, v4))
    } else {
        let v6: bool = &*v0 == "Blocked";
        if v6 {
            let v7: US0 = US0::US0_BlockerNamed;
            let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH1::UH1_Cons(v7, v8))
        } else {
            let v10: bool = &*v0 == "Paused";
            if v10 {
                let v11: US0 = US0::US0_PauseResume;
                let v12: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(UH1::UH1_Cons(v11, v12))
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<UH2> = split_list_10(v14, v1);
                let v16: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
                let v17: Rc<UH2> = method13(v15, v16);
                let v18: Rc<dyn Fn(Rc<str>) -> bool> = method18();
                let v19: bool = method19(v18, v17);
                if v19 {
                    let v20: US0 = US0::US0_PlanningFields;
                    let v21: US0 = US0::US0_DoneDefinition;
                    let v22: US0 = US0::US0_NextStep;
                    let v23: US0 = US0::US0_TypeFeatures;
                    let v24: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v25: Rc<UH1> = Rc::new(UH1::UH1_Cons(v23, v24));
                    let v26: Rc<UH1> = Rc::new(UH1::UH1_Cons(v22, v25));
                    let v27: Rc<UH1> = Rc::new(UH1::UH1_Cons(v21, v26));
                    Rc::new(UH1::UH1_Cons(v20, v27))
                } else {
                    let v29: US0 = US0::US0_PlanningFields;
                    let v30: US0 = US0::US0_DoneDefinition;
                    let v31: US0 = US0::US0_NextStep;
                    let v32: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    let v33: Rc<UH1> = Rc::new(UH1::UH1_Cons(v31, v32));
                    let v34: Rc<UH1> = Rc::new(UH1::UH1_Cons(v30, v33));
                    Rc::new(UH1::UH1_Cons(v29, v34))
                }
            }
        }
    }
}
fn failing_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>) -> Rc<UH1> {
    let v6: Rc<UH1> = checks_for_17(v0.clone(), v1.clone());
    let v7: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    method3(v0, v1, v2, v3, v4, v5, v6, v7)
}
fn method20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: &Rc<str>, mut v3: i32, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: Rc<UH1>, mut v8: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v7) {
        UH1::UH1_Cons(v9, v10) => {
            let mut v9: US0 = v9;
            let v10: Rc<UH1> = v10;
            let v11: Rc<UH0> = method20(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v10, v8);
            ;
            ;
            ;
            ;
            ;
            ;
            ;
            let v12: bool = match &v9 {
                US0::US0_PlanningFields => {
                    true
                }
                _ => {
                    false
                }
            };
            let v16: US1 = if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PlanningFields"); } LIT.with(|lit| lit.clone()) };
                US1::US1_Some(v13)
            } else {
                US1::US1_None
            };
            let v75: US1 = match v16 {
                US1::US1_None => {
                    let v19: bool = match &v9 {
                        US0::US0_DoneDefinition => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    let v23: US1 = if v19 {
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneDefinition"); } LIT.with(|lit| lit.clone()) };
                        US1::US1_Some(v20)
                    } else {
                        US1::US1_None
                    };
                    match v23 {
                        US1::US1_None => {
                            let v26: bool = match &v9 {
                                US0::US0_NextStep => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            };
                            let v30: US1 = if v26 {
                                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NextStep"); } LIT.with(|lit| lit.clone()) };
                                US1::US1_Some(v27)
                            } else {
                                US1::US1_None
                            };
                            match v30 {
                                US1::US1_None => {
                                    let v33: bool = match &v9 {
                                        US0::US0_TypeFeatures => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v37: US1 = if v33 {
                                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TypeFeatures"); } LIT.with(|lit| lit.clone()) };
                                        US1::US1_Some(v34)
                                    } else {
                                        US1::US1_None
                                    };
                                    match v37 {
                                        US1::US1_None => {
                                            let v40: bool = match &v9 {
                                                US0::US0_DoneEvidence => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            };
                                            let v44: US1 = if v40 {
                                                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneEvidence"); } LIT.with(|lit| lit.clone()) };
                                                US1::US1_Some(v41)
                                            } else {
                                                US1::US1_None
                                            };
                                            match v44 {
                                                US1::US1_None => {
                                                    let v47: bool = match &v9 {
                                                        US0::US0_BlockerNamed => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    };
                                                    let v51: US1 = if v47 {
                                                        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("BlockerNamed"); } LIT.with(|lit| lit.clone()) };
                                                        US1::US1_Some(v48)
                                                    } else {
                                                        US1::US1_None
                                                    };
                                                    match v51 {
                                                        US1::US1_None => {
                                                            let v54: bool = match v9 {
                                                                US0::US0_PauseResume => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            };
                                                            let v58: US1 = if v54 {
                                                                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PauseResume"); } LIT.with(|lit| lit.clone()) };
                                                                US1::US1_Some(v55)
                                                            } else {
                                                                US1::US1_None
                                                            };
                                                            match v58 {
                                                                US1::US1_None => {
                                                                    US1::US1_None
                                                                }
                                                                US1::US1_Some(v59) => {
                                                                    let v59: Rc<str> = v59;
                                                                    US1::US1_Some(v59)
                                                                }
                                                            }
                                                        }
                                                        US1::US1_Some(v52) => {
                                                            let v52: Rc<str> = v52;
                                                            US1::US1_Some(v52)
                                                        }
                                                    }
                                                }
                                                US1::US1_Some(v45) => {
                                                    let v45: Rc<str> = v45;
                                                    US1::US1_Some(v45)
                                                }
                                            }
                                        }
                                        US1::US1_Some(v38) => {
                                            let v38: Rc<str> = v38;
                                            US1::US1_Some(v38)
                                        }
                                    }
                                }
                                US1::US1_Some(v31) => {
                                    let v31: Rc<str> = v31;
                                    US1::US1_Some(v31)
                                }
                            }
                        }
                        US1::US1_Some(v24) => {
                            let v24: Rc<str> = v24;
                            US1::US1_Some(v24)
                        }
                    }
                }
                US1::US1_Some(v17) => {
                    let v17: Rc<str> = v17;
                    US1::US1_Some(v17)
                }
            };
            let v79: Rc<str> = match v75 {
                US1::US1_None => {
                    std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Option does not have a value."); } LIT.with(|lit| lit.clone()) }))
                }
                US1::US1_Some(v76) => {
                    let v76: Rc<str> = v76;
                    v76
                }
            };
            Rc::new(UH0::UH0_Cons(v0, v1, v79, v11))
        }
        UH1::UH1_Nil => {
            v8
        }
    }
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<UH0> = v5;
            let v6: Rc<UH0> = method21(v5, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v6))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn rows_from_1(mut v0: &Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < 0i32;
        if v3 {
            return v2;
        } else {
            let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>) = v0.borrow()[v1 as usize].clone();
            let v11: Rc<UH1> = failing_2(v5.clone(), v6.clone(), v7, v8.clone(), v9.clone(), v10.clone());
            let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v13: Rc<UH0> = method20(v4, v5, &v6, v7, &v8, &v9, &v10, v11, v12);
            let v14: i32 = v1.wrapping_sub(1i32);
            let v15: Rc<UH0> = method21(v13, v2);
            (v1, v2) = (v14, v15);
            continue;
        }
    }
}
fn method22(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                v0.borrow_mut().push((v3, v4, v5));
                let v7: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn failing_rows_0(mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    let mut v1: i32 = (v0.clone().borrow().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH0> = rows_from_1(&v0, v2, v3);
    let v5: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new(Vec::new()));
    let v6: i32 = 0i32;
    let v7: i32 = method22(v5.clone(), v4, v6);
    let v8: Rc<Vec<(Rc<str>, Rc<str>, Rc<str>)>> = Rc::new(v5.borrow().clone());
    let mut v9: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    v9
}
fn closure0() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>>> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>| -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
        failing_rows_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_lint_failing_rows(v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, i32, Rc<str>, Rc<str>, Rc<str>)>>>) -> Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>)>>> {
    closure0()(v0)
}
pub type SpiralShape_0408665020a9 = Mut0;
