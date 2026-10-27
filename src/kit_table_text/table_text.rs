#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn file_exists_1(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn table_problem_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_1(v0.clone());
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v2.clone()
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": file not found"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
        v4.clone()
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        table_problem_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn quoted_as_tsv_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: bool = v1 >= v2;
        if v4 {
            return v3.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: i32 = v1.wrapping_rem(2i32);
            let v7: bool = v6 == 1i32;
            let v28: Rc<str> = if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(v5.replace(&*v8, &*v9));
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = Rc::<str>::from(v10.replace(&*v11, &*v9));
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v9));
                v14.clone()
            } else {
                let v15: bool = &*v5 == "";
                let v17: bool = if v15 {
                    let v16: bool = v1 > 0i32;
                    v16
                } else {
                    false
                };
                let v20: bool = if v17 {
                    let v18: i32 = v2.wrapping_sub(1i32);
                    let v19: bool = v1 < v18;
                    v19
                } else {
                    false
                };
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                } else {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(v5.replace(&*v22, &*v23));
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = Rc::<str>::from(v24.replace(&*v25, &*v22));
                    v26.clone()
                }
            };
            let v29: i32 = v1.wrapping_add(1i32);
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v28));
            (v0, v1, v2, v3) = (v0.clone(), v29, v2, v30.clone());
            continue;
        }
    }
}
fn csv_line_as_tsv_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: i32 = (v2.clone().borrow().len() as i32);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    quoted_as_tsv_6(v2.clone(), v3, v4, v5.clone())
}
fn tsv_lines_4(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(v5.replace(&*v6, &*v7));
            let v9: Rc<str> = csv_line_as_tsv_5(v8.clone());
            let v10: i32 = v1.wrapping_add(1i32);
            let v11: bool = v1 == 0i32;
            let v15: Rc<str> = if v11 {
                v9.clone()
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v12)), v9));
                v14.clone()
            };
            (v0, v1, v2) = (v0.clone(), v10, v15.clone());
            continue;
        }
    }
}
fn tsv_of_text_3(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = Rc::<str>::from(v0.to_lowercase());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".csv"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = v2.ends_with(&*v3);
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = 0i32;
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        tsv_lines_4(v6.clone(), v7, v8.clone())
    } else {
        v1.clone()
    }
}
fn method7(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn table_tsv_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: bool = file_exists_1(v0.clone());
    if v1 {
        let v3: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
        let v5: Vec<u8> = v3.unwrap();
        let mut v6: Vec<u8> = method7(v5.clone());
        let v8: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v6);
        let v10: std::string::String = v8.unwrap();
        let v12: Rc<str> = Rc::<str>::from(String::as_str(&v10));
        tsv_of_text_3(v0.clone(), v12.clone())
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v14.clone()
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        table_tsv_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_problem(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_table_tsv(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
