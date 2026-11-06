#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_TrackedChange,
    US1_RenamedChange,
    US1_UntrackedEntry,
    US1_UnmergedEntry,
    US1_HeaderLine,
    US1_OtherLine,
}
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_3();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_4(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn git_output_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::process::Command::new("git").arg("-C").arg(&*v0).args(v1.split(char::from(10u8))).output().ok().filter(|output| output.status.success()).map(|output| String::from_utf8_lossy(&output.stdout).into_owned()).unwrap_or_default());
    v2
}
fn path_of_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v0.starts_with(&*v13);
                    if v14 {
                        US1::US1_HeaderLine
                    } else {
                        US1::US1_OtherLine
                    }
                }
            }
        }
    };
    match v21 {
        US1::US1_HeaderLine => {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30
        }
        US1::US1_OtherLine => {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v31
        }
        US1::US1_RenamedChange => {
            let v24: u64 = 9u64;
            let v25: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v24 as usize) + 1, char::from(32u8)).nth(v24 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v25
        }
        US1::US1_TrackedChange => {
            let v22: u64 = 8u64;
            let v23: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v22 as usize) + 1, char::from(32u8)).nth(v22 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v23
        }
        US1::US1_UnmergedEntry => {
            let v28: u64 = 10u64;
            let v29: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v28 as usize) + 1, char::from(32u8)).nth(v28 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v29
        }
        US1::US1_UntrackedEntry => {
            let v26: u64 = 1u64;
            let v27: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v26 as usize) + 1, char::from(32u8)).nth(v26 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v27
        }
    }
}
fn counts_of_9(mut v0: Rc<str>) -> (u64, u64, u64, u64) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let v14: bool = v0.starts_with(&*v13);
                    if v14 {
                        US1::US1_HeaderLine
                    } else {
                        US1::US1_OtherLine
                    }
                }
            }
        }
    };
    let (mut v54, mut v55, mut v56, mut v57): (u64, u64, u64, u64) = match &v21 {
        US1::US1_TrackedChange => {
            let v22: u64 = 1u64;
            let v23: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v22 as usize).unwrap_or(""));
            let v24: bool = v23.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let v25: u64 = if v24 {
                1u64
            } else {
                0u64
            };
            let v26: bool = v23.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let v27: u64 = if v26 {
                1u64
            } else {
                0u64
            };
            (v25, v27, 0u64, 0u64)
        }
        _ => {
            let (mut v50, mut v51, mut v52, mut v53): (u64, u64, u64, u64) = match &v21 {
                US1::US1_RenamedChange => {
                    let v28: u64 = 1u64;
                    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v28 as usize).unwrap_or(""));
                    let v30: bool = v29.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let v31: u64 = if v30 {
                        1u64
                    } else {
                        0u64
                    };
                    let v32: bool = v29.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let v33: u64 = if v32 {
                        1u64
                    } else {
                        0u64
                    };
                    (v31, v33, 0u64, 0u64)
                }
                _ => {
                    let (mut v46, mut v47, mut v48, mut v49): (u64, u64, u64, u64) = match &v21 {
                        US1::US1_UntrackedEntry => {
                            (0u64, 0u64, 1u64, 0u64)
                        }
                        _ => {
                            let (mut v42, mut v43, mut v44, mut v45): (u64, u64, u64, u64) = match &v21 {
                                US1::US1_UnmergedEntry => {
                                    (0u64, 0u64, 0u64, 1u64)
                                }
                                _ => {
                                    let (mut v38, mut v39, mut v40, mut v41): (u64, u64, u64, u64) = match &v21 {
                                        US1::US1_HeaderLine => {
                                            (0u64, 0u64, 0u64, 0u64)
                                        }
                                        _ => {
                                            let (mut v34, mut v35, mut v36, mut v37): (u64, u64, u64, u64) = match v21 {
                                                US1::US1_OtherLine => {
                                                    (0u64, 0u64, 0u64, 0u64)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v34, v35, v36, v37)
                                        }
                                    };
                                    (v38, v39, v40, v41)
                                }
                            };
                            (v42, v43, v44, v45)
                        }
                    };
                    (v46, v47, v48, v49)
                }
            };
            (v50, v51, v52, v53)
        }
    };
    (v54, v55, v56, v57)
}
fn tally_status_7(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, [u64; 4]>>>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: u64) -> () {
    loop {
        let v5: bool = v2 < v3;
        if v5 {
            let v6: i32 = (v2 as i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let v9: bool = v6 >= 0i32;
            let v19: bool = if v9 {
                let v10: i32 = (v8.clone().borrow().len() as i32);
                let v11: bool = v10 > 0i32;
                let v15: bool = if v11 {
                    let v12: i32 = v10.wrapping_sub(1i32);
                    let v13: Rc<str> = v8.borrow()[v12 as usize].clone();
                    let v14: bool = &*v13 == "";
                    v14
                } else {
                    false
                };
                let v17: i32 = if v15 {
                    let v16: i32 = v10.wrapping_sub(1i32);
                    v16
                } else {
                    v10
                };
                let v18: bool = v6 < v17;
                v18
            } else {
                false
            };
            let v32: Rc<str> = if v19 {
                let v20: Rc<str> = v8.borrow()[v6 as usize].clone();
                let v21: i32 = (v20.clone().len() as i32);
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v20.ends_with(&*v22);
                let v24: bool = v23 == false;
                if v24 {
                    v20
                } else {
                    let v25: bool = v21 == 1i32;
                    if v25 {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v26
                    } else {
                        let v27: i32 = v21.wrapping_sub(2i32);
                        let v28: Rc<str> = string_slice(&v20, 0i32 as i64, v27 as i64);
                        v28
                    }
                }
            } else {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v31
            };
            let v33: Rc<str> = path_of_8(v32.clone());
            let v34: bool = &*v33 != "";
            if v34 {
                let v35: Rc<str> = { let parts = v33.split(char::from(47u8)).collect::<Vec<_>>(); let keep = (v4 as usize).min(parts.len().saturating_sub(1)).max(1).min(parts.len()); std::rc::Rc::<str>::from(parts[..keep].join("/")) };
                let (mut v36, mut v37, mut v38, mut v39): (u64, u64, u64, u64) = counts_of_9(v32);
                { let mut map = v0.borrow_mut(); let entry = map.entry(v35.to_string()).or_insert([0; 4]); entry[0] += v36; entry[1] += v37; entry[2] += v38; entry[3] += v39; };
                ()
            };
            let v40: u64 = v2.wrapping_add(1u64);
            v2 = v40;
            continue;
        }
        return ();
    }
}
fn method11(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_10(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method11(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn repo_state_of_5(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let mut v2: bool = std::path::Path::new(&*v0).join(".git").exists();
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain=v2\n--branch\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = git_output_6(v0.clone(), v3);
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n-1\n--format=%h %cI %s"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = git_output_6(v0.clone(), v5);
        let v7: std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, [u64; 4]>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let v8: u64 = 0u64;
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v11: i32 = (v10.clone().borrow().len() as i32);
        let v12: bool = v11 > 0i32;
        let v16: bool = if v12 {
            let v13: i32 = v11.wrapping_sub(1i32);
            let v14: Rc<str> = v10.borrow()[v13 as usize].clone();
            let v15: bool = &*v14 == "";
            v15
        } else {
            false
        };
        let v18: i32 = if v16 {
            let v17: i32 = v11.wrapping_sub(1i32);
            v17
        } else {
            v11
        };
        let v19: u64 = (v18 as u64);
        tally_status_7(v7.clone(), v4.clone(), v8, v19, v1);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.head "); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v20)).unwrap_or("").trim());
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.ab "); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v22)).unwrap_or("").trim());
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit repo state ok root="); } LIT.with(|lit| lit.clone()) }, v0));
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" branch="); } LIT.with(|lit| lit.clone()) }));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v21));
        let v27: bool = &*v23 == "";
        let v29: Rc<str> = if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            v28
        } else {
            v23
        };
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ab="); } LIT.with(|lit| lit.clone()) }, v29));
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v30));
        let v32: u64 = 0u64;
        let v33: u64 = v7.borrow().values().map(|counts| counts[v32 as usize]).sum::<u64>();
        let v34: Rc<str> = format_real_10(v33);
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" staged="); } LIT.with(|lit| lit.clone()) }, v34));
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" modified="); } LIT.with(|lit| lit.clone()) }));
        let v37: u64 = 1u64;
        let v38: u64 = v7.borrow().values().map(|counts| counts[v37 as usize]).sum::<u64>();
        let v39: Rc<str> = format_real_10(v38);
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v39));
        let v41: u64 = 2u64;
        let v42: u64 = v7.borrow().values().map(|counts| counts[v41 as usize]).sum::<u64>();
        let v43: Rc<str> = format_real_10(v42);
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" untracked="); } LIT.with(|lit| lit.clone()) }, v43));
        let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unmerged="); } LIT.with(|lit| lit.clone()) }));
        let v46: u64 = 3u64;
        let v47: u64 = v7.borrow().values().map(|counts| counts[v46 as usize]).sum::<u64>();
        let v48: Rc<str> = format_real_10(v47);
        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v48));
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v49));
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v50));
        let v52: u64 = v7.borrow().len() as u64;
        let v53: Rc<str> = format_real_10(v52);
        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" areas="); } LIT.with(|lit| lit.clone()) }, v53));
        let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v54));
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" last="); } LIT.with(|lit| lit.clone()) }));
        let v57: u64 = 0u64;
        let v58: i32 = (v57 as i32);
        let v59: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v60: bool = v58 >= 0i32;
        let v70: bool = if v60 {
            let v61: i32 = (v59.clone().borrow().len() as i32);
            let v62: bool = v61 > 0i32;
            let v66: bool = if v62 {
                let v63: i32 = v61.wrapping_sub(1i32);
                let v64: Rc<str> = v59.borrow()[v63 as usize].clone();
                let v65: bool = &*v64 == "";
                v65
            } else {
                false
            };
            let v68: i32 = if v66 {
                let v67: i32 = v61.wrapping_sub(1i32);
                v67
            } else {
                v61
            };
            let v69: bool = v58 < v68;
            v69
        } else {
            false
        };
        let v83: Rc<str> = if v70 {
            let v71: Rc<str> = v59.borrow()[v58 as usize].clone();
            let v72: i32 = (v71.clone().len() as i32);
            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v74: bool = v71.ends_with(&*v73);
            let v75: bool = v74 == false;
            if v75 {
                v71
            } else {
                let v76: bool = v72 == 1i32;
                if v76 {
                    let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v77
                } else {
                    let v78: i32 = v72.wrapping_sub(2i32);
                    let v79: Rc<str> = string_slice(&v71, 0i32 as i64, v78 as i64);
                    v79
                }
            }
        } else {
            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v82
        };
        let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v83));
        println!("{}", v84);
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v86: u64 = 12u64;
        let v87: Rc<str> = { let map = v7.borrow(); let mut areas = map.iter().collect::<Vec<_>>(); areas.sort_by(|left, right| right.1.iter().sum::<u64>().cmp(&left.1.iter().sum::<u64>()).then_with(|| left.0.cmp(right.0))); let shown = areas.iter().take(v86 as usize).map(|(area, counts)| format!("{}area={} staged={} modified={} untracked={} unmerged={}", &*v85, area, counts[0], counts[1], counts[2], counts[3])).collect::<Vec<_>>(); let hidden = areas.len().saturating_sub(shown.len()); let mut text = shown.join("
"); if hidden > 0 { text.push_str(&format!("
{}... {hidden} more areas (kit table query or git status for the rest)", &*v85)); } std::rc::Rc::<str>::from(text) };
        let v88: bool = &*v87 != "";
        if v88 {
            println!("{}", v87);
            ()
        };
        0i32
    } else {
        let v89: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit repo state rejected root="); } LIT.with(|lit| lit.clone()) }, v0));
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason=not-a-git-repository"); } LIT.with(|lit| lit.clone()) }));
        println!("{}", v90);
        2i32
    }
}
fn states_from_1(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v0 < v1;
        if v4 {
            let v5: Rc<str> = argument_2(v0);
            let v6: i32 = repo_state_of_5(v5, v2);
            let v7: i32 = v0.wrapping_add(1i32);
            let v8: bool = v6 > v3;
            let v9: i32 = if v8 {
                v6
            } else {
                v3
            };
            (v0, v3) = (v7, v9);
            continue;
        } else {
            return v3;
        }
    }
}
fn argument_count_12() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn repo_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: Rc<str> = argument_2(v0);
    let mut v2: bool = &*v1 == "state";
    let v5: bool = if v2 {
        let v3: i32 = argument_count_12();
        let v4: bool = v3 > 3i32;
        v4
    } else {
        false
    };
    if v5 {
        let v6: i32 = 3i32;
        let v7: i32 = argument_count_12();
        let v8: u64 = 2u64;
        let v9: i32 = 0i32;
        states_from_1(v6, v7, v8, v9)
    } else {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: repo usage: repo state <root> [<root>...]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v11);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        repo_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_repo_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
