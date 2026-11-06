#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_RecordCatalogComplete,
    US0_RecordCatalogPartial(Rc<str>),
}
fn agile_record_catalog_text_0(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile records catalog="); } LIT.with(|lit| lit.clone()) }, v0));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" present="); } LIT.with(|lit| lit.clone()) }));
    let v6: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let v9: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: bool = v1 == v2;
    let v14: US0 = if v11 {
        US0::US0_RecordCatalogComplete
    } else {
        US0::US0_RecordCatalogPartial(v3)
    };
    match v14 {
        US0::US0_RecordCatalogComplete => {
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing=none"); } LIT.with(|lit| lit.clone()) }));
            v15
        }
        US0::US0_RecordCatalogPartial(v16) => {
            let v16: Rc<str> = v16;
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" missing="); } LIT.with(|lit| lit.clone()) }));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (agile check does not need them)"); } LIT.with(|lit| lit.clone()) }));
            v19
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, i32, i32, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>| -> Rc<str> {
        agile_record_catalog_text_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_record_catalog_text(v0: &str, v1: i32, v2: i32, v3: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), v1, v2, Rc::<str>::from(v3))
}
