#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ListingShown(Rc<str>),
    US0_ListingRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_WholeRoot,
    US1_RootedDirectory(Rc<str>),
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn proxy_argument_count_2() -> i32 {
    let v0: i32 = command_line_argument_count_3();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn directory_listing_outcome_1() -> US0 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = proxy_argument_count_2();
    let v2: bool = v1 == 2;
    if v2 {
        let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let v5: bool = &*v4 == ".";
        let v8: US1 = if v5 {
            US1::US1_WholeRoot
        } else {
            US1::US1_RootedDirectory(v4.clone())
        };
        let v12: Rc<str> = match &v8 {
            US1::US1_RootedDirectory(v9) => {
                let v9: Rc<str> = v9.clone();
                let v10: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v9).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                v10.clone()
            }
            US1::US1_WholeRoot => {
                v3.clone()
            }
        };
        let v13: bool = &*v12 == "";
        if v13 {
            let v14: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            US0::US0_ListingRejected(v16.clone())
        } else {
            let v18: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).err().unwrap_or_default().as_str());
            let v19: bool = &*v18 == "";
            if v19 {
                let mut v20: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                let v21: bool = &*v20 == "";
                let v23: i32 = if v21 {
                    0i32
                } else {
                    let v22: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v20.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v22
                };
                let v26: u64 = if v21 {
                    0u64
                } else {
                    let v24: u64 = v20.matches(char::from(10u8)).count() as u64;
                    let v25: u64 = v24 + 1;
                    v25
                };
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v12));
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                let v31: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
                let v35: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let v36: Rc<str> = std::rc::Rc::<str>::from((v35).to_string());
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v36));
                US0::US0_ListingShown(v37.clone())
            } else {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v18));
                US0::US0_ListingRejected(v40.clone())
            }
        }
    } else {
        let v44: bool = v1 == 3;
        if v44 {
            let v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let v47: bool = &*v46 == ".";
            let v50: US1 = if v47 {
                US1::US1_WholeRoot
            } else {
                US1::US1_RootedDirectory(v46.clone())
            };
            let v54: Rc<str> = match &v50 {
                US1::US1_RootedDirectory(v51) => {
                    let v51: Rc<str> = v51.clone();
                    let v52: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v45), &*v51).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    v52.clone()
                }
                US1::US1_WholeRoot => {
                    v45.clone()
                }
            };
            let v55: bool = &*v54 == "";
            if v55 {
                let v56: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v45), &*v46).err().unwrap_or_default().as_str());
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v56));
                US0::US0_ListingRejected(v58.clone())
            } else {
                let v60: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v54)).err().unwrap_or_default().as_str());
                let v61: bool = &*v60 == "";
                if v61 {
                    let mut v62: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v54)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                    let v63: bool = &*v62 == "";
                    let v65: i32 = if v63 {
                        0i32
                    } else {
                        let v64: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v62.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v64
                    };
                    let v68: u64 = if v63 {
                        0u64
                    } else {
                        let v66: u64 = v62.matches(char::from(10u8)).count() as u64;
                        let v67: u64 = v66 + 1;
                        v67
                    };
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) };
                    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v54));
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                    let v73: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
                    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
                    let v77: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let v78: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v78));
                    US0::US0_ListingShown(v79.clone())
                } else {
                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v60));
                    US0::US0_ListingRejected(v82.clone())
                }
            }
        } else {
            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list expects <root> [relative]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ListingRejected(v86.clone())
        }
    }
}
fn directory_listing_report_0() -> (i32, Rc<str>) {
    let v0: US0 = directory_listing_outcome_1();
    match &v0 {
        US0::US0_ListingRejected(v2) => {
            let v2: Rc<str> = v2.clone();
            (2i32, v2)
        }
        US0::US0_ListingShown(v1) => {
            let v1: Rc<str> = v1.clone();
            (0i32, v1)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        directory_listing_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_directory_listing_outcome() -> (i32, Rc<str>) {
    closure0()()
}
