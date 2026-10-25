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
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3.clone()
}
fn argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let mut v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let mut v9: US0 = if v5 {
        let mut v6: Rc<str> = command_line_argument_4(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let mut v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn git_output_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::process::Command::new("git").arg("-C").arg(&*v0).args(v1.split(char::from(10u8))).output().ok().filter(|output| output.status.success()).map(|output| String::from_utf8_lossy(&output.stdout).into_owned()).unwrap_or_default());
    v2.clone()
}
fn path_of_8(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let mut v14: bool = v0.starts_with(&*v13);
                    if v14 {
                        US1::US1_HeaderLine
                    } else {
                        US1::US1_OtherLine
                    }
                }
            }
        }
    };
    match &v21 {
        US1::US1_HeaderLine => {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        }
        US1::US1_OtherLine => {
            let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v31.clone()
        }
        US1::US1_RenamedChange => {
            let mut v24: u64 = 9u64;
            let mut v25: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v24 as usize) + 1, char::from(32u8)).nth(v24 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v25.clone()
        }
        US1::US1_TrackedChange => {
            let mut v22: u64 = 8u64;
            let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v22 as usize) + 1, char::from(32u8)).nth(v22 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v23.clone()
        }
        US1::US1_UnmergedEntry => {
            let mut v28: u64 = 10u64;
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v28 as usize) + 1, char::from(32u8)).nth(v28 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v29.clone()
        }
        US1::US1_UntrackedEntry => {
            let mut v26: u64 = 1u64;
            let mut v27: Rc<str> = std::rc::Rc::<str>::from(v0.splitn((v26 as usize) + 1, char::from(32u8)).nth(v26 as usize).unwrap_or("").split(char::from(9u8)).next().unwrap_or(""));
            v27.clone()
        }
    }
}
fn counts_of_9(mut v0: Rc<str>) -> (u64, u64, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1 "); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.starts_with(&*v1);
    let mut v21: US1 = if v2 {
        US1::US1_TrackedChange
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2 "); } LIT.with(|lit| lit.clone()) };
        let mut v5: bool = v0.starts_with(&*v4);
        if v5 {
            US1::US1_RenamedChange
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("? "); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v0.starts_with(&*v7);
            if v8 {
                US1::US1_UntrackedEntry
            } else {
                let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u "); } LIT.with(|lit| lit.clone()) };
                let mut v11: bool = v0.starts_with(&*v10);
                if v11 {
                    US1::US1_UnmergedEntry
                } else {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# "); } LIT.with(|lit| lit.clone()) };
                    let mut v14: bool = v0.starts_with(&*v13);
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
            let mut v22: u64 = 1u64;
            let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v22 as usize).unwrap_or(""));
            let mut v24: bool = v23.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let mut v25: u64 = if v24 {
                1u64
            } else {
                0u64
            };
            let mut v26: bool = v23.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
            let mut v27: u64 = if v26 {
                1u64
            } else {
                0u64
            };
            (v25, v27, 0u64, 0u64)
        }
        _ => {
            let (mut v50, mut v51, mut v52, mut v53): (u64, u64, u64, u64) = match &v21 {
                US1::US1_RenamedChange => {
                    let mut v28: u64 = 1u64;
                    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(32u8)).nth(v28 as usize).unwrap_or(""));
                    let mut v30: bool = v29.chars().next().map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let mut v31: u64 = if v30 {
                        1u64
                    } else {
                        0u64
                    };
                    let mut v32: bool = v29.chars().nth(1).map(|c| c == char::from(46u8)).unwrap_or(true) == false;
                    let mut v33: u64 = if v32 {
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
                                            let (mut v34, mut v35, mut v36, mut v37): (u64, u64, u64, u64) = match &v21 {
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
        let mut v5: bool = v2 < v3;
        if v5 {
            let mut v6: i32 = (v2 as i32);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
            let mut v9: bool = v6 >= 0i32;
            let mut v19: bool = if v9 {
                let mut v10: i32 = (v8.clone().borrow().len() as i32);
                let mut v11: bool = v10 > 0i32;
                let mut v15: bool = if v11 {
                    let mut v12: i32 = v10.wrapping_sub(1i32);
                    let mut v13: Rc<str> = v8.clone().borrow()[v12 as usize].clone();
                    let mut v14: bool = v13.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v14
                } else {
                    false
                };
                let mut v17: i32 = if v15 {
                    let mut v16: i32 = v10.wrapping_sub(1i32);
                    v16
                } else {
                    v10
                };
                let mut v18: bool = v6 < v17;
                v18
            } else {
                false
            };
            let mut v32: Rc<str> = if v19 {
                let mut v20: Rc<str> = v8.clone().borrow()[v6 as usize].clone();
                let mut v21: i32 = (v20.clone().len() as i32);
                let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                let mut v23: bool = v20.ends_with(&*v22);
                let mut v24: bool = v23 == false;
                if v24 {
                    v20.clone()
                } else {
                    let mut v25: bool = v21 == 1i32;
                    if v25 {
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    } else {
                        let mut v27: i32 = v21.wrapping_sub(2i32);
                        let mut v28: Rc<str> = string_slice(&v20.clone(), 0i32 as i64, v27 as i64);
                        v28.clone()
                    }
                }
            } else {
                let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v31.clone()
            };
            let mut v33: Rc<str> = path_of_8(v32.clone());
            let mut v34: bool = v33.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v34 {
                let mut v35: Rc<str> = { let parts = v33.split(char::from(47u8)).collect::<Vec<_>>(); let keep = (v4 as usize).min(parts.len().saturating_sub(1)).max(1).min(parts.len()); std::rc::Rc::<str>::from(parts[..keep].join("/")) };
                let (mut v36, mut v37, mut v38, mut v39): (u64, u64, u64, u64) = counts_of_9(v32.clone());
                { let mut map = v0.borrow_mut(); let entry = map.entry(v35.to_string()).or_insert([0; 4]); entry[0] += v36; entry[1] += v37; entry[2] += v38; entry[3] += v39; };
                ()
            };
            let mut v40: u64 = v2.wrapping_add(1u64);
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v40, v3, v4);
            continue;
        }
        return ();
    }
}
fn repo_state_of_5(mut v0: Rc<str>, mut v1: u64) -> i32 {
    let mut v2: bool = std::path::Path::new(&*v0).join(".git").exists();
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain=v2\n--branch\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = git_output_6(v0.clone(), v3.clone());
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n-1\n--format=%h %cI %s"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = git_output_6(v0.clone(), v5.clone());
        let mut v7: std::rc::Rc<std::cell::RefCell<std::collections::BTreeMap<String, [u64; 4]>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeMap::new()));
        let mut v8: u64 = 0u64;
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v4.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v11: i32 = (v10.clone().borrow().len() as i32);
        let mut v12: bool = v11 > 0i32;
        let mut v16: bool = if v12 {
            let mut v13: i32 = v11.wrapping_sub(1i32);
            let mut v14: Rc<str> = v10.clone().borrow()[v13 as usize].clone();
            let mut v15: bool = v14.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15
        } else {
            false
        };
        let mut v18: i32 = if v16 {
            let mut v17: i32 = v11.wrapping_sub(1i32);
            v17
        } else {
            v11
        };
        let mut v19: u64 = (v18 as u64);
        tally_status_7(v7.clone(), v4.clone(), v8, v19, v1);
        let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.head "); } LIT.with(|lit| lit.clone()) };
        let mut v21: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v20)).unwrap_or("").trim());
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("# branch.ab "); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = std::rc::Rc::<str>::from(v4.lines().find_map(|line| line.strip_prefix(&*v22)).unwrap_or("").trim());
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie repo state ok root="); } LIT.with(|lit| lit.clone()) };
        let mut v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v0));
        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" branch="); } LIT.with(|lit| lit.clone()) };
        let mut v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v21));
        let mut v29: bool = v23.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v31: Rc<str> = if v29 {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("none"); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        } else {
            v23.clone()
        };
        let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ab="); } LIT.with(|lit| lit.clone()) };
        let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v31));
        let mut v34: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v33));
        let mut v35: u64 = 0u64;
        let mut v36: u64 = v7.borrow().values().map(|counts| counts[v35 as usize]).sum::<u64>();
        let mut v37: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
        let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" staged="); } LIT.with(|lit| lit.clone()) };
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
        let mut v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" modified="); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
        let mut v42: u64 = 1u64;
        let mut v43: u64 = v7.borrow().values().map(|counts| counts[v42 as usize]).sum::<u64>();
        let mut v44: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
        let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v44));
        let mut v46: u64 = 2u64;
        let mut v47: u64 = v7.borrow().values().map(|counts| counts[v46 as usize]).sum::<u64>();
        let mut v48: Rc<str> = std::rc::Rc::<str>::from((v47).to_string());
        let mut v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" untracked="); } LIT.with(|lit| lit.clone()) };
        let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v48));
        let mut v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unmerged="); } LIT.with(|lit| lit.clone()) };
        let mut v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
        let mut v53: u64 = 3u64;
        let mut v54: u64 = v7.borrow().values().map(|counts| counts[v53 as usize]).sum::<u64>();
        let mut v55: Rc<str> = std::rc::Rc::<str>::from((v54).to_string());
        let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v55));
        let mut v57: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v56));
        let mut v58: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v57));
        let mut v59: u64 = v7.borrow().len() as u64;
        let mut v60: Rc<str> = std::rc::Rc::<str>::from((v59).to_string());
        let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" areas="); } LIT.with(|lit| lit.clone()) };
        let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v60));
        let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v62));
        let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" last="); } LIT.with(|lit| lit.clone()) };
        let mut v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
        let mut v66: u64 = 0u64;
        let mut v67: i32 = (v66 as i32);
        let mut v68: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v9).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let mut v69: bool = v67 >= 0i32;
        let mut v79: bool = if v69 {
            let mut v70: i32 = (v68.clone().borrow().len() as i32);
            let mut v71: bool = v70 > 0i32;
            let mut v75: bool = if v71 {
                let mut v72: i32 = v70.wrapping_sub(1i32);
                let mut v73: Rc<str> = v68.clone().borrow()[v72 as usize].clone();
                let mut v74: bool = v73.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v74
            } else {
                false
            };
            let mut v77: i32 = if v75 {
                let mut v76: i32 = v70.wrapping_sub(1i32);
                v76
            } else {
                v70
            };
            let mut v78: bool = v67 < v77;
            v78
        } else {
            false
        };
        let mut v92: Rc<str> = if v79 {
            let mut v80: Rc<str> = v68.clone().borrow()[v67 as usize].clone();
            let mut v81: i32 = (v80.clone().len() as i32);
            let mut v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let mut v83: bool = v80.ends_with(&*v82);
            let mut v84: bool = v83 == false;
            if v84 {
                v80.clone()
            } else {
                let mut v85: bool = v81 == 1i32;
                if v85 {
                    let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v86.clone()
                } else {
                    let mut v87: i32 = v81.wrapping_sub(2i32);
                    let mut v88: Rc<str> = string_slice(&v80.clone(), 0i32 as i64, v87 as i64);
                    v88.clone()
                }
            }
        } else {
            let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v91.clone()
        };
        let mut v93: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v92));
        println!("{}", &*v93);
        let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let mut v95: u64 = 12u64;
        let mut v96: Rc<str> = { let map = v7.borrow(); let mut areas = map.iter().collect::<Vec<_>>(); areas.sort_by(|left, right| right.1.iter().sum::<u64>().cmp(&left.1.iter().sum::<u64>()).then_with(|| left.0.cmp(right.0))); let shown = areas.iter().take(v95 as usize).map(|(area, counts)| format!("{}area={} staged={} modified={} untracked={} unmerged={}", &*v94, area, counts[0], counts[1], counts[2], counts[3])).collect::<Vec<_>>(); let hidden = areas.len().saturating_sub(shown.len()); let mut text = shown.join("
"); if hidden > 0 { text.push_str(&format!("
{}... {hidden} more areas (eoie table query or git status for the rest)", &*v94)); } std::rc::Rc::<str>::from(text) };
        let mut v97: bool = v96.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v97 {
            println!("{}", &*v96);
            ()
        };
        0i32
    } else {
        let mut v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie repo state rejected root="); } LIT.with(|lit| lit.clone()) };
        let mut v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v0));
        let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason=not-a-git-repository"); } LIT.with(|lit| lit.clone()) };
        let mut v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
        println!("{}", &*v101);
        2i32
    }
}
fn states_from_1(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v0 < v1;
        if v4 {
            let mut v5: Rc<str> = argument_2(v0);
            let mut v6: i32 = repo_state_of_5(v5.clone(), v2);
            let mut v7: i32 = v0.wrapping_add(1i32);
            let mut v8: bool = v6 > v3;
            let mut v9: i32 = if v8 {
                v6
            } else {
                v3
            };
            (v0, v1, v2, v3) = (v7, v1, v2, v9);
            continue;
        } else {
            return v3;
        }
    }
}
fn argument_count_10() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn repo_run_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = argument_2(v0);
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = if v2 {
        let mut v3: i32 = argument_count_10();
        let mut v4: bool = v3 > 3i32;
        v4
    } else {
        false
    };
    if v5 {
        let mut v6: i32 = 3i32;
        let mut v7: i32 = argument_count_10();
        let mut v8: u64 = 2u64;
        let mut v9: i32 = 0i32;
        states_from_1(v6, v7, v8, v9)
    } else {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: repo usage: repo state <root> [<root>...]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v11);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        repo_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_repo_run() -> i32 {
    closure0()()
}
