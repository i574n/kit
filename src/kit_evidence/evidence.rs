#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Refreshed(Rc<str>),
    US0_DryRun(Rc<str>),
    US0_NotListed,
    US0_OutsideRoot,
    US0_MissingFile,
    US0_WriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
}
fn stderr_line_1(mut v0: Rc<str>) -> i32 {
    eprintln!("{}", v0);
    2i32
}
fn stdout_line_2(mut v0: Rc<str>) -> i32 {
    println!("{}", v0);
    0i32
}
fn argument_3(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default().as_str());
    v1
}
fn option_after_4(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|value| value.as_str() != &*v0).nth(1).unwrap_or_default().as_str());
    v1.clone()
}
fn has_flag_5(mut v0: Rc<str>) -> bool {
    let v1: bool = std::env::args().skip(0i32 as usize).any(|argument| argument == &*v0);
    v1
}
fn file_length_7(mut v0: Rc<str>) -> u64 {
    let v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn file_sha256_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(kit_proxy_search::inspection_file_sha256(std::path::Path::new(&*v0)).unwrap_or_default().as_str());
    v1
}
fn method9(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn rewritten_row_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef ("); } LIT.with(|lit| lit.clone()) };
    let v5: Option<usize> = v0.find(&*v4);
    let v6: Option<usize> = method13(v5.clone());
    let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v8: Option<i32> = v6.map(|x| v7(x));
    let v10: i32 = v8.unwrap_or(-1i32);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
    let v12: Option<usize> = v0.find(&*v11);
    let v13: Option<usize> = method13(v12.clone());
    let mut v14: Option<i32> = v13.map(|x| v7(x));
    let v16: i32 = v14.unwrap_or(-1i32);
    let v17: bool = v10 < 0i32;
    let v19: bool = if v17 {
        true
    } else {
        let v18: bool = v16 < 0i32;
        v18
    };
    if v19 {
        v0.clone()
    } else {
        let v20: usize = ((0i32) as usize);
        let v21: usize = ((v10) as usize);
        let v22: Rc<str> = Rc::<str>::from(&(v0)[v20..v21]);
        let v23: i32 = v16.wrapping_add(3i32);
        let v24: i32 = (v0.clone().len() as i32);
        let v25: i32 = v24.wrapping_sub(v16);
        let v26: i32 = v25.wrapping_sub(3i32);
        let v27: usize = ((v23) as usize);
        let v28: i32 = v23.wrapping_add(v26);
        let v29: usize = ((v28) as usize);
        let v30: Rc<str> = Rc::<str>::from(&(v0)[v27..v29]);
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v1));
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v22, Rc::<str>::from(format!("{}{}", v32, v33))));
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v2)), v33));
        let v40: Rc<str> = format_real_14(v3);
        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v41, v42)), v30));
        v44.clone()
    }
}
fn joined_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<RefCell<Vec<Rc<str>>>>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool) -> US1 {
    loop {
        let v9: bool = v6 >= v4;
        if v9 {
            if v8 {
                return US1::US1_Some(v7.clone());
            } else {
                return US1::US1_None;
            }
        } else {
            let v13: Rc<str> = v3.clone().borrow()[v6 as usize].clone();
            let mut v14: bool = v8 == false;
            let v16: bool = if v14 {
                let v15: bool = v13.contains(&*v5);
                v15
            } else {
                false
            };
            let v18: Rc<str> = if v16 {
                rewritten_row_12(v13.clone(), v0.clone(), v1.clone(), v2)
            } else {
                v13.clone()
            };
            let v19: bool = v6 == 0i32;
            let v22: Rc<str> = if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v20.clone()
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v21.clone()
            };
            let v23: i32 = v6.wrapping_add(1i32);
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v22)), v18));
            let v26: bool = v8 || v16;
            (v0, v1, v2, v3, v4, v5, v6, v7, v8) = (v0.clone(), v1.clone(), v2, v3.clone(), v4, v5.clone(), v23, v25.clone(), v26);
            continue;
        }
    }
}
fn refreshed_text_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US1 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: i32 = 0i32;
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: bool = false;
    joined_11(v2.clone(), v3.clone(), v4, v6.clone(), v7, v11.clone(), v12, v13.clone(), v14)
}
fn refresh_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> US0 {
    let v4: bool = &*v2 == "";
    let v5: Rc<str> = if v4 {
        v1.clone()
    } else {
        v2.clone()
    };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/evidence.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v6));
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v8)), v5));
    let v11: bool = &*v1 != "";
    let v15: bool = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
        let v13: bool = v1.contains(&*v12);
        let mut v14: bool = v13 == false;
        v14
    } else {
        false
    };
    let v19: bool = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v17: bool = v1.contains(&*v16);
        let v18: bool = v17 == false;
        v18
    } else {
        false
    };
    let v22: bool = if v19 {
        let v20: bool = v1.starts_with(&*v8);
        let v21: bool = v20 == false;
        v21
    } else {
        false
    };
    let v26: bool = if v22 {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v24: bool = v1.starts_with(&*v23);
        let v25: bool = v24 == false;
        v25
    } else {
        false
    };
    let v27: bool = v26 == false;
    let v45: bool = if v27 {
        true
    } else {
        let v28: bool = &*v5 != "";
        let v32: bool = if v28 {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let v30: bool = v5.contains(&*v29);
            let v31: bool = v30 == false;
            v31
        } else {
            false
        };
        let v36: bool = if v32 {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v34: bool = v5.contains(&*v33);
            let v35: bool = v34 == false;
            v35
        } else {
            false
        };
        let v39: bool = if v36 {
            let v37: bool = v5.starts_with(&*v8);
            let v38: bool = v37 == false;
            v38
        } else {
            false
        };
        let v43: bool = if v39 {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let v41: bool = v5.starts_with(&*v40);
            let v42: bool = v41 == false;
            v42
        } else {
            false
        };
        let v44: bool = v43 == false;
        v44
    };
    if v45 {
        US0::US0_OutsideRoot
    } else {
        let v47: u64 = file_length_7(v10.clone());
        let v48: bool = v47 == 0u64;
        if v48 {
            US0::US0_MissingFile
        } else {
            let v50: Rc<str> = file_sha256_8(v10.clone());
            let v51: u64 = file_length_7(v10.clone());
            let v53: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v7);
            let v55: Vec<u8> = v53.unwrap();
            let v56: Vec<u8> = method9(v55.clone());
            let v58: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v56);
            let v60: std::string::String = v58.unwrap();
            let v62: Rc<str> = Rc::<str>::from(String::as_str(&v60));
            let v63: US1 = refreshed_text_10(v62.clone(), v1.clone(), v5.clone(), v50.clone(), v51);
            match &v63 {
                US1::US1_None => {
                    US0::US0_NotListed
                }
                US1::US1_Some(v65) => {
                    let v65: Rc<str> = v65.clone();
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) };
                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v5));
                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) };
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v69, v70)), v50)), v68));
                    let v74: Rc<str> = format_real_14(v51);
                    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
                    let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
                    if v3 {
                        US0::US0_DryRun(v77.clone())
                    } else {
                        let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v79));
                        let v81: bool = std::fs::write(&*v80, &*v65).is_ok();
                        let v93: US2 = if v81 {
                            let v82: bool = std::fs::rename(&*v80, &*v7).is_ok();
                            if v82 {
                                let v83: bool = std::fs::read_to_string(&*v7).is_ok_and(|value| value == *v65);
                                if v83 {
                                    US2::US2_Replaced
                                } else {
                                    US2::US2_ReadbackMismatch
                                }
                            } else {
                                let v87: bool = std::fs::remove_file(&*v80).is_ok() || !std::path::Path::new(&*v80).exists();
                                if v87 {
                                    US2::US2_RenameFailed
                                } else {
                                    US2::US2_RenameFailedTemporaryKept
                                }
                            }
                        } else {
                            US2::US2_TemporaryWriteFailed
                        };
                        match &v93 {
                            US2::US2_Replaced => {
                                US0::US0_Refreshed(v77.clone())
                            }
                            _ => {
                                let v102: Rc<str> = match &v93 {
                                    US2::US2_ReadbackMismatch => {
                                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                        v98.clone()
                                    }
                                    US2::US2_RenameFailed => {
                                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                        v96.clone()
                                    }
                                    US2::US2_RenameFailedTemporaryKept => {
                                        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                        v97.clone()
                                    }
                                    US2::US2_TemporaryWriteFailed => {
                                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                        v95.clone()
                                    }
                                    _ => unreachable!(),
                                };
                                US0::US0_WriteFailed(v102.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
fn evidence_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_3(v0);
    let v2: bool = &*v1 != "refresh";
    let v6: bool = if v2 {
        true
    } else {
        let v3: i32 = 3i32;
        let v4: Rc<str> = argument_3(v3);
        let v5: bool = &*v4 == "";
        v5
    };
    let v10: bool = if v6 {
        true
    } else {
        let mut v7: i32 = 4i32;
        let mut v8: Rc<str> = argument_3(v7);
        let v9: bool = &*v8 == "";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run]"); } LIT.with(|lit| lit.clone()) };
        stderr_line_1(v11.clone())
    } else {
        let v13: i32 = 3i32;
        let mut v14: Rc<str> = argument_3(v13);
        let v15: i32 = 4i32;
        let v16: Rc<str> = argument_3(v15);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = option_after_4(v17.clone());
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
        let v20: bool = has_flag_5(v19.clone());
        let v21: US0 = refresh_6(v14.clone(), v16.clone(), v18.clone(), v20);
        match &v21 {
            US0::US0_DryRun(v26) => {
                let v26: Rc<str> = v26.clone();
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh dry-run (nothing written) "); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
                stdout_line_2(v28.clone())
            }
            US0::US0_MissingFile => {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: the file to hash is missing or empty"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v34.clone())
            }
            US0::US0_NotListed => {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: that path has no EvidenceRef row in state/evidence.spi"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v30.clone())
            }
            US0::US0_OutsideRoot => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: the path must be relative and stay inside the root"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v32.clone())
            }
            US0::US0_Refreshed(v22) => {
                let v22: Rc<str> = v22.clone();
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh ok "); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
                stdout_line_2(v24.clone())
            }
            US0::US0_WriteFailed(v36) => {
                let v36: Rc<str> = v36.clone();
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: "); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v36));
                stderr_line_1(v38.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        evidence_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_evidence_run() -> i32 {
    closure0()()
}
