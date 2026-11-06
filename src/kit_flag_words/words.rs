#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn kebab_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: usize = ((v2) as usize);
            let v6: i32 = v2.wrapping_add(1i32);
            let v7: usize = ((v6) as usize);
            let v8: Rc<str> = Rc::<str>::from(&(v0)[v5..v7]);
            let v9: Rc<str> = Rc::<str>::from(v8.to_lowercase());
            let v10: bool = v2 > 0i32;
            let v12: bool = if v10 {
                let v11: bool = &*v8 != &*v9;
                v11
            } else {
                false
            };
            let v15: Rc<str> = if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v15));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
            (v2, v3) = (v6, v17);
            continue;
        }
    }
}
fn flag_word_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = 0i32;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = kebab_from_1(v0, v1, v2, v3);
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) }, v4));
    v5
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        flag_word_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_flag_word(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
