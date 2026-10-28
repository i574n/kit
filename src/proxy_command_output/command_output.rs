#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ChildSucceeded,
    US0_ChildFailed(i32),
}
fn command_output_receipt_text_0(mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64) -> Rc<str> {
    let v4: bool = v0 == 0i32;
    let v7: US0 = if v4 {
        US0::US0_ChildSucceeded
    } else {
        US0::US0_ChildFailed(v0)
    };
    let v12: Rc<str> = match &v7 {
        US0::US0_ChildFailed(v9) => {
            let v9: i32 = *v9;
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("child-failed"); } LIT.with(|lit| lit.clone()) };
            v10.clone()
        }
        US0::US0_ChildSucceeded => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy command-output "); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
    let v17: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stdout_bytes="); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v21));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr_bytes="); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
    let v28: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v28)), v27));
    v30.clone()
}
fn closure0() -> Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: i32, mut v1: u64, mut v2: u64, mut v3: u64| -> Rc<str> {
        command_output_receipt_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_command_output_receipt(v0: i32, v1: u64, v2: u64, v3: u64) -> Rc<str> {
    closure0()(v0, v1, v2, v3)
}
