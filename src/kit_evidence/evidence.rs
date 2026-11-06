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
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
    US2_ChangedSinceRead,
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
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    v1
}
fn option_after_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip_while(|value| value.as_str() != &*v0).nth(1).unwrap_or_default().as_str());
    v1
}
fn has_flag_5(mut v0: Rc<str>) -> bool {
    let mut v1: bool = std::env::args().skip(0i32 as usize).any(|argument| argument == &*v0);
    v1
}
fn file_byte_count_7(mut v0: Rc<str>) -> u64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let v3: bool = v2 == false;
    let mut v7: i64 = if v3 {
        0i64
    } else {
        let v4: std::fs::Metadata = v1.unwrap();
        let v5: u64 = v4.len();
        let v6: i64 = (v5 as i64);
        v6
    };
    let mut v8: u64 = (v7 as u64);
    v8
}
fn file_sha256_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(kit_proxy_search::inspection_file_sha256(std::path::Path::new(&*v0)).unwrap_or_default().as_str());
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
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_14(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method15(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn rewritten_row_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef ("); } LIT.with(|lit| lit.clone()) };
    let v5: Option<usize> = v0.find(&*v4);
    let v6: Option<usize> = method13(v5);
    let mut v7: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v8: Option<i32> = v6.map(|x| v7(x));
    let v10: i32 = v8.unwrap_or(-1i32);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
    let v12: Option<usize> = v0.find(&*v11);
    let v13: Option<usize> = method13(v12);
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
        v0
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
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) }, v1));
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v32));
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v2));
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
        let v37: Rc<str> = format_real_14(v3);
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) }));
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v30));
        v40
    }
}
fn joined_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<RefCell<Vec<Rc<str>>>>, mut v4: i32, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool) -> US1 {
    loop {
        let v9: bool = v6 >= v4;
        if v9 {
            if v8 {
                return US1::US1_Some(v7);
            } else {
                return US1::US1_None;
            }
        } else {
            let v13: Rc<str> = v3.borrow()[v6 as usize].clone();
            let mut v14: bool = v8 == false;
            let v16: bool = if v14 {
                let v15: bool = v13.contains(&*v5);
                v15
            } else {
                false
            };
            let v18: Rc<str> = if v16 {
                rewritten_row_12(v13, v0.clone(), v1.clone(), v2)
            } else {
                v13
            };
            let v19: bool = v6 == 0i32;
            let v22: Rc<str> = if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v20
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                v21
            };
            let v23: i32 = v6.wrapping_add(1i32);
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v22));
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v18));
            let v26: bool = v8 || v16;
            (v6, v7, v8) = (v23, v25, v26);
            continue;
        }
    }
}
fn refreshed_text_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> US1 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v5).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v7: i32 = (v6.clone().borrow().len() as i32);
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) }, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
    let v10: i32 = 0i32;
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: bool = false;
    joined_11(v2, v3, v4, v6, v7, v9, v10, v11, v12)
}
fn renamed_within_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn refresh_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool) -> US0 {
    let v4: bool = &*v2 == "";
    let v5: Rc<str> = if v4 {
        v1.clone()
    } else {
        v2
    };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/evidence.spi"); } LIT.with(|lit| lit.clone()) }));
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5.clone()));
    let v9: bool = &*v1 != "";
    let v13: bool = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v1.contains(&*v10);
        let v12: bool = v11 == false;
        v12
    } else {
        false
    };
    let v17: bool = if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v15: bool = v1.contains(&*v14);
        let v16: bool = v15 == false;
        v16
    } else {
        false
    };
    let v21: bool = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v1.starts_with(&*v18);
        let v20: bool = v19 == false;
        v20
    } else {
        false
    };
    let v25: bool = if v21 {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v23: bool = v1.starts_with(&*v22);
        let v24: bool = v23 == false;
        v24
    } else {
        false
    };
    let v26: bool = v25 == false;
    let v45: bool = if v26 {
        true
    } else {
        let v27: bool = &*v5 != "";
        let v31: bool = if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".."); } LIT.with(|lit| lit.clone()) };
            let v29: bool = v5.contains(&*v28);
            let v30: bool = v29 == false;
            v30
        } else {
            false
        };
        let v35: bool = if v31 {
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v33: bool = v5.contains(&*v32);
            let v34: bool = v33 == false;
            v34
        } else {
            false
        };
        let v39: bool = if v35 {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v37: bool = v5.starts_with(&*v36);
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
        let v47: u64 = file_byte_count_7(v8.clone());
        let v48: bool = v47 == 0u64;
        if v48 {
            US0::US0_MissingFile
        } else {
            let v50: Rc<str> = file_sha256_8(v8.clone());
            let v51: u64 = file_byte_count_7(v8);
            let v53: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
            let v55: Vec<u8> = v53.unwrap();
            let v56: Vec<u8> = method9(v55);
            let v58: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v56);
            let v60: std::string::String = v58.unwrap();
            let v62: Rc<str> = Rc::<str>::from(String::as_str(&v60));
            let v63: US1 = refreshed_text_10(v62, v1, v5.clone(), v50.clone(), v51);
            match v63 {
                US1::US1_None => {
                    US0::US0_NotListed
                }
                US1::US1_Some(v65) => {
                    let v65: Rc<str> = v65;
                    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("core.EvidenceRef (\""); } LIT.with(|lit| lit.clone()) }, v5));
                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v50));
                    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\", "); } LIT.with(|lit| lit.clone()) }));
                    let v71: Rc<str> = format_real_14(v51);
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) }));
                    if v3 {
                        US0::US0_DryRun(v73)
                    } else {
                        let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v6.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
                        let v76: bool = std::fs::write(&*v75, &*v65).is_ok();
                        let v89: US2 = if v76 {
                            let v77: i32 = 40i32;
                            let v78: bool = renamed_within_16(v6.clone(), v75.clone(), v77);
                            if v78 {
                                let v79: bool = std::fs::read_to_string(&*v6).is_ok_and(|value| value == *v65);
                                if v79 {
                                    US2::US2_Replaced
                                } else {
                                    US2::US2_ReadbackMismatch
                                }
                            } else {
                                let v83: bool = std::fs::remove_file(&*v75).is_ok() || !std::path::Path::new(&*v75).exists();
                                if v83 {
                                    US2::US2_RenameFailed
                                } else {
                                    US2::US2_RenameFailedTemporaryKept
                                }
                            }
                        } else {
                            US2::US2_TemporaryWriteFailed
                        };
                        match &v89 {
                            US2::US2_Replaced => {
                                US0::US0_Refreshed(v73)
                            }
                            _ => {
                                let v100: Rc<str> = match v89 {
                                    US2::US2_ChangedSinceRead => {
                                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                        v95
                                    }
                                    US2::US2_ReadbackMismatch => {
                                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                        v94
                                    }
                                    US2::US2_RenameFailed => {
                                        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                        v92
                                    }
                                    US2::US2_RenameFailedTemporaryKept => {
                                        let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                        v93
                                    }
                                    US2::US2_TemporaryWriteFailed => {
                                        let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                        v91
                                    }
                                    _ => unreachable!(),
                                };
                                US0::US0_WriteFailed(v100)
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
    let mut v1: Rc<str> = argument_3(v0);
    let mut v2: bool = &*v1 != "refresh";
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
        stderr_line_1(v11)
    } else {
        let v13: i32 = 3i32;
        let mut v14: Rc<str> = argument_3(v13);
        let v15: i32 = 4i32;
        let v16: Rc<str> = argument_3(v15);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = option_after_4(v17);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dry-run"); } LIT.with(|lit| lit.clone()) };
        let v20: bool = has_flag_5(v19);
        let v21: US0 = refresh_6(v14, v16, v18, v20);
        match v21 {
            US0::US0_DryRun(v25) => {
                let v25: Rc<str> = v25;
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh dry-run (nothing written) "); } LIT.with(|lit| lit.clone()) }, v25));
                stdout_line_2(v26)
            }
            US0::US0_MissingFile => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: the file to hash is missing or empty"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v32)
            }
            US0::US0_NotListed => {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: that path has no EvidenceRef row in state/evidence.spi"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v28)
            }
            US0::US0_OutsideRoot => {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: the path must be relative and stay inside the root"); } LIT.with(|lit| lit.clone()) };
                stderr_line_1(v30)
            }
            US0::US0_Refreshed(v22) => {
                let v22: Rc<str> = v22;
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh ok "); } LIT.with(|lit| lit.clone()) }, v22));
                stdout_line_2(v23)
            }
            US0::US0_WriteFailed(v34) => {
                let v34: Rc<str> = v34;
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit evidence refresh: "); } LIT.with(|lit| lit.clone()) }, v34));
                stderr_line_1(v35)
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
pub type SpiralShape_97df63ebabc3 = Mut0;
