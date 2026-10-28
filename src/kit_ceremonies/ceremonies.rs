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
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit: "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a verb of this ceremony; use: "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v1));
    eprintln!("{}", v6);
    false
}
fn closure1(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH0>) -> US2> {
    Rc::new(move |mut v2: Rc<UH0>| -> US2 {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US2::US2_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v8);
                    v9(v5.clone())
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
    let v13: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12.clone(), v13.clone()));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11.clone(), v14.clone()));
    let v16: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10.clone(), v15.clone()));
    let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9.clone(), v16.clone()));
    let v18: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8.clone(), v17.clone()));
    let v19: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7.clone(), v18.clone()));
    let v20: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v19.clone()));
    let v21: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v20.clone()));
    let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v21.clone()));
    let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3.clone(), v22.clone()));
    let v24: US2 = v2(v23.clone());
    let v25: bool = match &v24 {
        US2::US2_None => {
            true
        }
        _ => {
            false
        }
    };
    let v26: bool = v25 != true;
    if v26 {
        let v27: US1 = US1::US1_Backlog;
        US0::US0_Some(v27.clone())
    } else {
        let v29: i32 = 0i32;
        let mut v30: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v29);
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v39.clone(), v40.clone()));
        let v42: Rc<UH0> = Rc::new(UH0::UH0_Cons(v38.clone(), v41.clone()));
        let v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v37.clone(), v42.clone()));
        let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v36.clone(), v43.clone()));
        let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v35.clone(), v44.clone()));
        let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34.clone(), v45.clone()));
        let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33.clone(), v46.clone()));
        let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v32.clone(), v47.clone()));
        let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31.clone(), v48.clone()));
        let v50: US2 = v30(v49.clone());
        let v51: bool = match &v50 {
            US2::US2_None => {
                true
            }
            _ => {
                false
            }
        };
        let v52: bool = v51 != true;
        if v52 {
            let v53: US1 = US1::US1_Story;
            US0::US0_Some(v53.clone())
        } else {
            let v55: i32 = 0i32;
            let mut v56: Rc<dyn Fn(Rc<UH0>) -> US2> = method3(v0.clone(), v55);
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
            let v59: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v60: Rc<UH0> = Rc::new(UH0::UH0_Cons(v58.clone(), v59.clone()));
            let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v57.clone(), v60.clone()));
            let v62: US2 = v56(v61.clone());
            let v63: bool = match &v62 {
                US2::US2_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v64: bool = v63 != true;
            if v64 {
                let v65: US1 = US1::US1_Standup;
                US0::US0_Some(v65.clone())
            } else {
                US0::US0_None
            }
        }
    }
}
fn ceremony_admits_0(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: US0 = ceremony_of_subcommand_2(v1.clone());
    match &v2 {
        US0::US0_None => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile "); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v1));
            refused_1(v1.clone(), v25.clone())
        }
        US0::US0_Some(v3) => {
            let v3: US1 = v3.clone();
            let mut v9: Rc<str> = match &v3 {
                US1::US1_Backlog => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
                    v4.clone()
                }
                US1::US1_Standup => {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup"); } LIT.with(|lit| lit.clone()) };
                    v6.clone()
                }
                US1::US1_Story => {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story"); } LIT.with(|lit| lit.clone()) };
                    v5.clone()
                }
            };
            let v10: bool = &*v9 == &*v0;
            if v10 {
                true
            } else {
                let v16: Rc<str> = match &v3 {
                    US1::US1_Backlog => {
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog"); } LIT.with(|lit| lit.clone()) };
                        v11.clone()
                    }
                    US1::US1_Standup => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup"); } LIT.with(|lit| lit.clone()) };
                        v13.clone()
                    }
                    US1::US1_Story => {
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story"); } LIT.with(|lit| lit.clone()) };
                        v12.clone()
                    }
                };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit "); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v1));
                refused_1(v1.clone(), v21.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> bool {
        ceremony_admits_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_ceremony_admits(v0: &str, v1: &str) -> bool {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
