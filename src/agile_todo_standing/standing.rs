#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_OpenTodo,
    US0_StaleTodo,
    US0_UnknownTodo,
}
fn standings_from_1(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: i32) -> (i32, i32) {
    let v6: i32 = (v3.clone().len() as i32);
    let v7: bool = v6 <= v5;
    let (mut v49, mut v50): (i32, i32) = if v7 {
        (v0, v1)
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v9: i32 = v3.get(v5 as usize..).and_then(|rest| rest.find(&*v8)).map(|found| (found + v5 as usize) as i32).unwrap_or(-1);
        let v10: bool = v9 < 0i32;
        let v11: i32 = if v10 {
            v6
        } else {
            v9
        };
        let v12: Rc<str> = std::rc::Rc::<str>::from(v3.get(v5 as usize..v11 as usize).unwrap_or(""));
        let v13: Rc<str> = agile_todo_lines::kit_agile_todo_id(&*v2, &*v12);
        let v14: bool = &*v13 == "";
        let v24: US0 = if v14 {
            US0::US0_OpenTodo
        } else {
            let v16: Rc<str> = agile_task_census::kit_agile_task_status(&*v2, &*v13);
            let v17: bool = &*v16 == "";
            if v17 {
                US0::US0_UnknownTodo
            } else {
                let v19: bool = &*v16 == "Done";
                if v19 {
                    US0::US0_StaleTodo
                } else {
                    US0::US0_OpenTodo
                }
            }
        };
        let v26: bool = if v4 {
            true
        } else {
            match &v24 {
                US0::US0_OpenTodo => {
                    true
                }
                _ => {
                    false
                }
            }
        };
        let v37: i32 = if v26 {
            0i32
        } else {
            let v32: Rc<str> = match &v24 {
                US0::US0_OpenTodo => {
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v27
                }
                US0::US0_StaleTodo => {
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stale (item is Done): "); } LIT.with(|lit| lit.clone()) };
                    v28
                }
                US0::US0_UnknownTodo => {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown (no backlog item): "); } LIT.with(|lit| lit.clone()) };
                    v29
                }
            };
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v13));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at "); } LIT.with(|lit| lit.clone()) }));
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v12));
            let v36: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v35.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v36
        };
        let (mut v44, mut v45): (i32, i32) = match &v24 {
            US0::US0_OpenTodo => {
                (v0, v1)
            }
            _ => {
                let (mut v42, mut v43): (i32, i32) = match &v24 {
                    US0::US0_StaleTodo => {
                        let v38: i32 = v0.wrapping_add(1i32);
                        (v38, v1)
                    }
                    _ => {
                        let (mut v40, mut v41): (i32, i32) = match v24 {
                            US0::US0_UnknownTodo => {
                                let v39: i32 = v1.wrapping_add(1i32);
                                (v0, v39)
                            }
                            _ => unreachable!(),
                        };
                        (v40, v41)
                    }
                };
                (v42, v43)
            }
        };
        let v46: i32 = v11.wrapping_add(1i32);
        let (mut v47, mut v48): (i32, i32) = standings_from_1(v44, v45, v2, v3, v4, v46);
        (v47, v48)
    };
    (v49, v50)
}
fn standings_of_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> (i32, i32) {
    let v3: i32 = 0i32;
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let (mut v6, mut v7): (i32, i32) = standings_from_1(v3, v4, v0, v1, v2, v5);
    (v6, v7)
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, bool) -> (i32, i32)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, bool) -> (i32, i32)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool| -> (i32, i32) {
        standings_of_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_todo_standings(v0: &str, v1: &str, v2: bool) -> (i32, i32) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), v2)
}
