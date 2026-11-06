#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ReadShown(i32),
    US0_ReadRejected(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_Streamed(u64),
    US1_StreamFailed,
}
#[derive(Clone)]
enum US2 {
    US2_WholeFileShown,
    US2_LeadingBytesShown,
}
#[derive(Clone)]
enum US3 {
    US3_ContentClosed,
    US3_ContentOpen,
}
#[derive(Clone)]
enum US5 {
    US5_WholeFile,
    US5_LeadingBytes(u64),
}
#[derive(Clone)]
enum US4 {
    US4_Valid(US5),
    US4_Invalid,
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
fn read_receipt_text_4(mut v0: Rc<str>, mut v1: u64, mut v2: i32, mut v3: u64) -> Rc<str> {
    let v4: u64 = kit_rust_std_fs::rooted_regular_size(std::path::Path::new(&*v0)).unwrap_or(0);
    let v5: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy fs-read ok bytes="); } LIT.with(|lit| lit.clone()) }, v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" shown="); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: bool = v1 < v4;
    let v13: US2 = if v10 {
        US2::US2_LeadingBytesShown
    } else {
        US2::US2_WholeFileShown
    };
    let v27: Rc<str> = match v13 {
        US2::US2_LeadingBytesShown => {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" truncated=true"); } LIT.with(|lit| lit.clone()) };
            v25
        }
        US2::US2_WholeFileShown => {
            let v14: bool = v2 < 0i32;
            let v20: US3 = if v14 {
                US3::US3_ContentClosed
            } else {
                let v16: bool = v2 == 10i32;
                if v16 {
                    US3::US3_ContentClosed
                } else {
                    US3::US3_ContentOpen
                }
            };
            match v20 {
                US3::US3_ContentClosed => {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v21
                }
                US3::US3_ContentOpen => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" no_final_newline=true"); } LIT.with(|lit| lit.clone()) };
                    v22
                }
            }
        }
    };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v27));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
    let v30: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
    let v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v31));
    v32
}
fn file_reading_outcome_1() -> US0 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = proxy_argument_count_2();
    let v2: bool = v1 == 3;
    if v2 {
        let v3: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
        let v4: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
        let v5: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        let v6: bool = &*v5 == "";
        if v6 {
            let v7: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v3), &*v4).err().unwrap_or_default().as_str());
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) }, v7));
            US0::US0_ReadRejected(v8)
        } else {
            let v10: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).err().unwrap_or_default().as_str());
            let v11: bool = &*v10 == "";
            if v11 {
                let v12: u64 = u64::MAX;
                let v13: u64 = kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v12), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                let v14: u64 = u64::MAX;
                let v15: bool = v13 == v14;
                let v18: US1 = if v15 {
                    US1::US1_StreamFailed
                } else {
                    US1::US1_Streamed(v13)
                };
                match v18 {
                    US1::US1_StreamFailed => {
                        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) }, v5));
                        US0::US0_ReadRejected(v19)
                    }
                    US1::US1_Streamed(v21) => {
                        let v21: u64 = v21;
                        let v22: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                        let v23: bool = v22 == 0i32;
                        if v23 {
                            let v24: i32 = kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v5)).ok().and_then(|mut file| { let offset = (v21).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                            let v25: Rc<str> = read_receipt_text_4(v5, v21, v24, v0);
                            let v26: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v25.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            US0::US0_ReadShown(v26)
                        } else {
                            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) }, v5));
                            US0::US0_ReadRejected(v28)
                        }
                    }
                }
            } else {
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) }, v10));
                US0::US0_ReadRejected(v33)
            }
        }
    } else {
        let v37: bool = v1 == 4;
        if v37 {
            let v38: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
            let v39: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
            let v40: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
            let v41: bool = v40.parse::<u64>().is_ok();
            let v46: US4 = if v41 {
                let v42: u64 = v40.parse::<u64>().unwrap_or(0);
                let v43: US5 = US5::US5_LeadingBytes(v42);
                US4::US4_Valid(v43)
            } else {
                US4::US4_Invalid
            };
            match v46 {
                US4::US4_Invalid => {
                    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid byte limit"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReadRejected(v83)
                }
                US4::US4_Valid(v47) => {
                    let v47: US5 = v47;
                    let v48: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v38), &*v39).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                    let v49: bool = &*v48 == "";
                    if v49 {
                        let v50: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_path(std::path::Path::new(&*v38), &*v39).err().unwrap_or_default().as_str());
                        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read path rejected: "); } LIT.with(|lit| lit.clone()) }, v50));
                        US0::US0_ReadRejected(v51)
                    } else {
                        let v53: Rc<str> = std::rc::Rc::<str>::from(kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v48)).err().unwrap_or_default().as_str());
                        let v54: bool = &*v53 == "";
                        if v54 {
                            let v58: u64 = match v47 {
                                US5::US5_LeadingBytes(v56) => {
                                    let v56: u64 = v56;
                                    v56
                                }
                                US5::US5_WholeFile => {
                                    let v55: u64 = u64::MAX;
                                    v55
                                }
                            };
                            let v59: u64 = kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v48)).ok().and_then(|file| std::io::copy(&mut std::io::Read::take(file, v58), &mut std::io::stdout().lock()).ok()).unwrap_or(u64::MAX);
                            let v60: u64 = u64::MAX;
                            let v61: bool = v59 == v60;
                            let v64: US1 = if v61 {
                                US1::US1_StreamFailed
                            } else {
                                US1::US1_Streamed(v59)
                            };
                            match v64 {
                                US1::US1_StreamFailed => {
                                    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed while streaming: "); } LIT.with(|lit| lit.clone()) }, v48));
                                    US0::US0_ReadRejected(v65)
                                }
                                US1::US1_Streamed(v67) => {
                                    let v67: u64 = v67;
                                    let v68: i32 = std::io::Write::flush(&mut std::io::stdout()).map(|_| 0i32).unwrap_or(2i32);
                                    let v69: bool = v68 == 0i32;
                                    if v69 {
                                        let v70: i32 = kit_rust_std_fs::rooted_open_regular_read(std::path::Path::new(&*v48)).ok().and_then(|mut file| { let offset = (v67).checked_sub(1)?; std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(offset)).ok()?; let mut byte = [0u8; 1]; std::io::Read::read_exact(&mut file, &mut byte).ok()?; Some(i32::from(byte[0])) }).unwrap_or(-1);
                                        let v71: Rc<str> = read_receipt_text_4(v48, v67, v70, v0);
                                        let v72: i32 = std::io::Write::write_all(&mut std::io::stderr(), (v71.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                        US0::US0_ReadShown(v72)
                                    } else {
                                        let v74: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read could not flush stdout: "); } LIT.with(|lit| lit.clone()) }, v48));
                                        US0::US0_ReadRejected(v74)
                                    }
                                }
                            }
                        } else {
                            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read failed: "); } LIT.with(|lit| lit.clone()) }, v53));
                            US0::US0_ReadRejected(v79)
                        }
                    }
                }
            }
        } else {
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs-read expects <root> <relative> [max-bytes]"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReadRejected(v87)
        }
    }
}
fn file_reading_report_0() -> (i32, Rc<str>) {
    let v0: US0 = file_reading_outcome_1();
    match v0 {
        US0::US0_ReadRejected(v3) => {
            let v3: Rc<str> = v3;
            (2i32, v3)
        }
        US0::US0_ReadShown(v1) => {
            let v1: i32 = v1;
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (0i32, v2)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        file_reading_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_file_reading_outcome() -> (i32, Rc<str>) {
    closure0()()
}
