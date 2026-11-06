#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_Backlog,
    US1_Story,
    US1_Standup,
}
#[derive(Clone)]
enum US0 {
    US0_Some(US1),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn refused_1(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit: "); } LIT.with(|lit| lit.clone()) }, v0));
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a verb of this ceremony; use: "); } LIT.with(|lit| lit.clone()) }));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
    eprintln!("{}", v4);
    false
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match Rc::unwrap_or_clone(v2) {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH0> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH0::UH0_Nil => {
                US2::US2_None
            }
        }
    })
}
fn method3(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    closure1(v0, v1)
}
fn ceremony_of_subcommand_2(mut v0: Rc<str>) -> US0 {
    let v1: i32 = 0i32;
    let mut v2: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sprint"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v15));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v16));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v17));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v18));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v19));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v20));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v21));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v22));
    let v24: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v23));
    let v25: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v24));
    let v26: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v25));
    let v27: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v26));
    let v28: US2 = v2(v27);
    let v29: bool = match v28 {
        US2::US2_None => {
            true
        }
        _ => {
            false
        }
    };
    let v30: bool = v29 != true;
    if v30 {
        let v31: US1 = US1::US1_Backlog;
        US0::US0_Some(v31)
    } else {
        let v33: i32 = 0i32;
        let mut v34: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v33);
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v43, v44));
        let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v42, v45));
        let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v41, v46));
        let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v40, v47));
        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39, v48));
        let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38, v49));
        let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37, v50));
        let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36, v51));
        let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v35, v52));
        let v54: US2 = v34(v53);
        let v55: bool = match v54 {
            US2::US2_None => {
                true
            }
            _ => {
                false
            }
        };
        let v56: bool = v55 != true;
        if v56 {
            let v57: US1 = US1::US1_Story;
            US0::US0_Some(v57)
        } else {
            let v59: i32 = 0i32;
            let mut v60: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0, v59);
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cadence"); } LIT.with(|lit| lit.clone()) };
            let v64: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v65: Rc<UH0> = Rc::new(UH0::UH0_Cons(v63, v64));
            let v66: Rc<UH0> = Rc::new(UH0::UH0_Cons(v62, v65));
            let v67: Rc<UH0> = Rc::new(UH0::UH0_Cons(v61, v66));
            let v68: US2 = v60(v67);
            let v69: bool = match v68 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v70: bool = v69 != true;
            if v70 {
                let v71: US1 = US1::US1_Standup;
                US0::US0_Some(v71)
            } else {
                US0::US0_None
            }
        }
    }
}
fn ceremony_admits_0(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: US0 = ceremony_of_subcommand_2(v1.clone());
    match v2 {
        US0::US0_None => {
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
            refused_1(v1, v18)
        }
        US0::US0_Some(v3) => {
            let v3: US1 = v3;
            let v7: Rc<str> = match &v3 {
                US1::US1_Backlog => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
                    v4
                }
                US1::US1_Standup => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup"); } LIT.with(|lit| lit.clone()) };
                    v6
                }
                US1::US1_Story => {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story"); } LIT.with(|lit| lit.clone()) };
                    v5
                }
            };
            let v8: bool = &*v7 == &*v0;
            if v8 {
                true
            } else {
                let v12: Rc<str> = match v3 {
                    US1::US1_Backlog => {
                        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
                        v9
                    }
                    US1::US1_Standup => {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup"); } LIT.with(|lit| lit.clone()) };
                        v11
                    }
                    US1::US1_Story => {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story"); } LIT.with(|lit| lit.clone()) };
                        v10
                    }
                };
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit "); } LIT.with(|lit| lit.clone()) }, v12));
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v1.clone()));
                refused_1(v1, v15)
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> bool {
        ceremony_admits_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_ceremony_admits(v0: &str, v1: &str) -> bool {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
