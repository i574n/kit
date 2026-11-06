#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_CopyReceipt(Rc<str>),
    US0_CopyRejected(Rc<str>),
}
fn file_copy_outcome_1(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US0 {
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: bool = v0 == 4i32;
    if v5 {
        let v6: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).err().unwrap_or_default().as_str());
        let v7: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).err().unwrap_or_default().as_str());
        let v8: bool = &*v6 == "";
        if v8 {
            let v9: bool = &*v7 == "";
            if v9 {
                let v10: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v2).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let v11: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v1), &*v3).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                let v12: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::copy_regular_atomic_preserve(std::path::Path::new(&*v10), std::path::Path::new(&*v11)).err().unwrap_or_default().as_str());
                let v13: bool = &*v12 == "";
                if v13 {
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-copy ok source="); } LIT.with(|lit| lit.clone()) }, v10));
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" target="); } LIT.with(|lit| lit.clone()) }));
                    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v11.clone()));
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes="); } LIT.with(|lit| lit.clone()) }));
                    let v18: u64 = std::fs::symlink_metadata(&*v11).map(|meta| meta.len()).unwrap_or(0);
                    let v19: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v19));
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                    let v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                    let v23: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v23));
                    US0::US0_CopyReceipt(v24)
                } else {
                    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy failed: "); } LIT.with(|lit| lit.clone()) }, v12));
                    US0::US0_CopyRejected(v26)
                }
            } else {
                US0::US0_CopyRejected(v7)
            }
        } else {
            US0::US0_CopyRejected(v6)
        }
    } else {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-copy expects root, source, target"); } LIT.with(|lit| lit.clone()) };
        US0::US0_CopyRejected(v33)
    }
}
fn file_copy_report_0(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let v4: US0 = file_copy_outcome_1(v0, v1, v2, v3);
    match v4 {
        US0::US0_CopyReceipt(v5) => {
            let v5: Rc<str> = v5;
            (0i32, v5)
        }
        US0::US0_CopyRejected(v6) => {
            let v6: Rc<str> = v6;
            (2i32, v6)
        }
    }
}
fn closure0() -> Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (i32, Rc<str>) {
        file_copy_report_0(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_file_copy_outcome(v0: i32, v1: &str, v2: &str, v3: &str) -> (i32, Rc<str>) {
    closure0()(v0, Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
