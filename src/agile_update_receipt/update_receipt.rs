#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn agile_clock_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    v0
}
fn closure0() -> Rc<dyn Fn() -> u64> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> u64> = Rc::new(move || -> u64 {
        agile_clock_ms_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_set_receipt_text_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64) -> Rc<str> {
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile set ok id="); } LIT.with(|lit| lit.clone()) }, v0));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" progress="); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status="); } LIT.with(|lit| lit.clone()) }));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v2));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" validation=runtime-attested compiler=not-required"); } LIT.with(|lit| lit.clone()) }));
    let v13: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
    let v14: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v14));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
    let v17: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
    let v20: Rc<str> = std::rc::Rc::<str>::from((v13).to_string());
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v21));
    v22
}
fn closure1() -> Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, Rc<str>, u64, u64, u64) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: u64| -> Rc<str> {
        agile_set_receipt_text_1(v0, v1, v2, v3, v4, v5)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_clock_ms() -> u64 {
    closure0()()
}
pub fn kit_agile_set_receipt_text(v0: &str, v1: i32, v2: &str, v3: u64, v4: u64, v5: u64) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), v3, v4, v5)
}
