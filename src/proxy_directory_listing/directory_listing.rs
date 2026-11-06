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
        let v12: Rc<str> = match v8 {
            US1::US1_RootedDirectory(v9) => {
                let v9: Rc<str> = v9;
                let v10: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v9).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                v10
            }
            US1::US1_WholeRoot => {
                v3.clone()
            }
        };
        let v13: bool = &*v12 == "";
        if v13 {
            let v14: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) }, v14));
            US0::US0_ListingRejected(v15)
        } else {
            let v17: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).err().unwrap_or_default().as_str());
            let v18: bool = &*v17 == "";
            if v18 {
                let mut v19: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v12)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                let v20: bool = &*v19 == "";
                let v22: i32 = if v20 {
                    0i32
                } else {
                    let v21: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v21
                };
                let v25: u64 = if v20 {
                    0u64
                } else {
                    let v23: u64 = v19.matches(char::from(10u8)).count() as u64;
                    let v24: u64 = v23 + 1;
                    v24
                };
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) }, v12));
                let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) }));
                let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                let v31: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let v32: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v32));
                US0::US0_ListingShown(v33)
            } else {
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) }, v17));
                US0::US0_ListingRejected(v35)
            }
        }
    } else {
        let v39: bool = v1 == 3;
        if v39 {
            let v40: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let v41: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let v42: bool = &*v41 == ".";
            let v45: US1 = if v42 {
                US1::US1_WholeRoot
            } else {
                US1::US1_RootedDirectory(v41.clone())
            };
            let v49: Rc<str> = match v45 {
                US1::US1_RootedDirectory(v46) => {
                    let v46: Rc<str> = v46;
                    let v47: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v40), &*v46).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    v47
                }
                US1::US1_WholeRoot => {
                    v40.clone()
                }
            };
            let v50: bool = &*v49 == "";
            if v50 {
                let v51: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v40), &*v41).err().unwrap_or_default().as_str());
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list path rejected: "); } LIT.with(|lit| lit.clone()) }, v51));
                US0::US0_ListingRejected(v52)
            } else {
                let v54: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v49)).err().unwrap_or_default().as_str());
                let v55: bool = &*v54 == "";
                if v55 {
                    let mut v56: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_list_names(std::path::Path::new(&*v49)).unwrap_or_default().join(&char::from(10u8).to_string()).as_str());
                    let v57: bool = &*v56 == "";
                    let v59: i32 = if v57 {
                        0i32
                    } else {
                        let v58: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v56.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v58
                    };
                    let v62: u64 = if v57 {
                        0u64
                    } else {
                        let v60: u64 = v56.matches(char::from(10u8)).count() as u64;
                        let v61: u64 = v60 + 1;
                        v61
                    };
                    let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-list ok path="); } LIT.with(|lit| lit.clone()) }, v49));
                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" entries="); } LIT.with(|lit| lit.clone()) }));
                    let v65: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
                    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                    let v68: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                    let v69: Rc<str> = std::rc::Rc::<str>::from((v68).to_string());
                    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v69));
                    US0::US0_ListingShown(v70)
                } else {
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list failed: "); } LIT.with(|lit| lit.clone()) }, v54));
                    US0::US0_ListingRejected(v72)
                }
            }
        } else {
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-list expects <root> [relative]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ListingRejected(v76)
        }
    }
}
fn directory_listing_report_0() -> (i32, Rc<str>) {
    let v0: US0 = directory_listing_outcome_1();
    match v0 {
        US0::US0_ListingRejected(v2) => {
            let v2: Rc<str> = v2;
            (2i32, v2)
        }
        US0::US0_ListingShown(v1) => {
            let v1: Rc<str> = v1;
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
