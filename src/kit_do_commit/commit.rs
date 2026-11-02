#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Committed,
    US0_WriteFailed(Rc<str>, Rc<str>),
    US0_ChangedSinceRead(Rc<str>),
    US0_PromoteFailed(Rc<str>, bool),
}
fn commit_problem_1(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_ChangedSinceRead(v9) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" changed since it was read; run the program again"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            v11
        }
        US0::US0_Committed => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US0::US0_PromoteFailed(v12, v13) => {
            let v12: Rc<str> = v12.clone();
            let v13: bool = *v13;
            let v16: Rc<str> = if v13 {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("every earlier file was restored"); } LIT.with(|lit| lit.clone()) };
                v14
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("some earlier files could not be restored"); } LIT.with(|lit| lit.clone()) };
                v15
            };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v12));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" still failed after 40 tries; "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v16));
            v21
        }
        US0::US0_WriteFailed(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2));
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v6)), v3));
            v8
        }
    }
}
fn target_at_4(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.borrow()[v2 as usize].0.clone();
    let v4: Rc<str> = std::path::Path::new(&*v1).join(&*v3).display().to_string().into();
    v4
}
fn temporary_at_5(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = target_at_4(v0.clone(), v1.clone(), v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".kit-do.tmp"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::path::Path::new(&*v3).with_file_name(format!(".{}{}", std::path::Path::new(&*v3).file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(), &*v4)).display().to_string().into();
    v5.clone()
}
fn renamed_within_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn discarded_7(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> () {
    loop {
        let v4: bool = v2 >= v3;
        if v4 {
            return ();
        } else {
            let v5: Rc<str> = temporary_at_5(v0.clone(), v1.clone(), v2);
            let v6: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            let v7: i32 = v2.wrapping_add(1i32);
            v2 = v7;
            continue;
        }
        return ();
    }
}
fn restored_8(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: bool) -> bool {
    loop {
        let v5: bool = v2 >= v3;
        if v5 {
            return v4;
        } else {
            let v6: Rc<str> = temporary_at_5(v0.clone(), v1.clone(), v2);
            let v7: Rc<str> = v0.borrow()[v2 as usize].2.clone();
            let v8: Rc<str> = std::fs::write(&*v6, &*v7).err().map(|error| error.to_string()).unwrap_or_default().into();
            let v9: bool = &*v8 == "";
            let v13: bool = if v9 {
                let v10: Rc<str> = target_at_4(v0.clone(), v1.clone(), v2);
                let v11: i32 = 40i32;
                renamed_within_6(v10.clone(), v6.clone(), v11)
            } else {
                false
            };
            let v15: bool = if v13 {
                true
            } else {
                let v14: bool = std::fs::remove_file(&*v6).is_ok() || !std::path::Path::new(&*v6).exists();
                v14
            };
            let v16: i32 = v2.wrapping_add(1i32);
            let v17: bool = v4 && v13;
            (v2, v4) = (v16, v17);
            continue;
        }
    }
}
fn promoted_3(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    loop {
        let v3: i32 = v0.borrow().len() as i32;
        let v4: bool = v2 >= v3;
        if v4 {
            return US0::US0_Committed;
        } else {
            let v6: Rc<str> = target_at_4(v0.clone(), v1.clone(), v2);
            let v7: Rc<str> = temporary_at_5(v0.clone(), v1.clone(), v2);
            let v8: i32 = 40i32;
            let v9: bool = renamed_within_6(v6.clone(), v7.clone(), v8);
            if v9 {
                let v10: i32 = v2.wrapping_add(1i32);
                v2 = v10;
                continue;
            } else {
                let v12: i32 = v0.borrow().len() as i32;
                discarded_7(v0.clone(), v1.clone(), v2, v12);
                let v13: Rc<str> = target_at_4(v0.clone(), v1.clone(), v2);
                let v14: i32 = 0i32;
                let v15: bool = true;
                let v16: bool = restored_8(v0.clone(), v1.clone(), v14, v2, v15);
                return US0::US0_PromoteFailed(v13.clone(), v16);
            }
        }
    }
}
fn written_9(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    loop {
        let v3: i32 = v0.borrow().len() as i32;
        let v4: bool = v2 >= v3;
        if v4 {
            return US0::US0_Committed;
        } else {
            let v6: Rc<str> = temporary_at_5(v0.clone(), v1.clone(), v2);
            let v7: Rc<str> = v0.borrow()[v2 as usize].1.clone();
            let v8: Rc<str> = std::fs::write(&*v6, &*v7).err().map(|error| error.to_string()).unwrap_or_default().into();
            let v9: bool = &*v8 == "";
            if v9 {
                let v10: i32 = v2.wrapping_add(1i32);
                v2 = v10;
                continue;
            } else {
                let v12: i32 = 0i32;
                discarded_7(v0.clone(), v1.clone(), v12, v2);
                let v13: Rc<str> = v0.borrow()[v2 as usize].0.clone();
                return US0::US0_WriteFailed(v13.clone(), v8.clone());
            }
        }
    }
}
fn unread_changes_10(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    loop {
        let v3: i32 = v0.borrow().len() as i32;
        let v4: bool = v2 >= v3;
        if v4 {
            return US0::US0_Committed;
        } else {
            let v6: Rc<str> = v0.borrow()[v2 as usize].2.clone();
            let v7: Rc<str> = target_at_4(v0.clone(), v1.clone(), v2);
            let v8: bool = std::fs::read_to_string(&*v7).is_ok_and(|value| value == *v6);
            if v8 {
                let v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            } else {
                let v11: Rc<str> = v0.borrow()[v2 as usize].0.clone();
                return US0::US0_ChangedSinceRead(v11.clone());
            }
        }
    }
}
fn committed_2(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>) -> US0 {
    let mut v2: i32 = 0i32;
    let v3: US0 = written_9(v0.clone(), v1.clone(), v2);
    match &v3 {
        US0::US0_Committed => {
            let v4: i32 = 0i32;
            let v5: US0 = unread_changes_10(v0.clone(), v1.clone(), v4);
            match &v5 {
                US0::US0_Committed => {
                    let v6: i32 = 0i32;
                    promoted_3(v0.clone(), v1.clone(), v6)
                }
                _ => {
                    let v8: i32 = 0i32;
                    let v9: i32 = v0.borrow().len() as i32;
                    discarded_7(v0, v1, v8, v9);
                    v5
                }
            }
        }
        _ => {
            v3
        }
    }
}
fn committed_problem_0(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: US0 = committed_2(v0, v1);
    commit_problem_1(v2)
}
fn closure0() -> Rc<dyn Fn(std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>| -> Rc<str> {
        committed_problem_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_committed(v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, v1: &str) -> Rc<str> {
    closure0()(v0, Rc::<str>::from(v1))
}
