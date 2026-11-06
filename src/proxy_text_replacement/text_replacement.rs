#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ReplacementReceipt(Rc<str>),
    US0_ReplacementRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_CheckReplacement,
    US2_ApplyReplacement,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_ExactlyOneMatch,
    US4_ExactlyMatches(u64),
    US4_EveryMatch,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US5 {
    US5_Replaced,
    US5_TemporaryWriteFailed,
    US5_RenameFailed,
    US5_RenameFailedTemporaryKept,
    US5_ReadbackMismatch,
    US5_ChangedSinceRead,
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn proxy_argument_count_2() -> i32 {
    let v0: i32 = command_line_argument_count_3();
    let v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn regular_file_4(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn renamed_within_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn text_replace_outcome_1() -> US0 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
    let mut v2: bool = &*v1 == "check";
    let v10: US1 = if v2 {
        let v3: US2 = US2::US2_CheckReplacement;
        US1::US1_Valid(v3)
    } else {
        let v5: bool = &*v1 == "apply";
        if v5 {
            let mut v6: US2 = US2::US2_ApplyReplacement;
            US1::US1_Valid(v6)
        } else {
            US1::US1_Invalid
        }
    };
    let v11: i32 = proxy_argument_count_2();
    let v12: bool = 6i32 == v11;
    let v37: US3 = if v12 {
        let v13: US4 = US4::US4_ExactlyOneMatch;
        US3::US3_Valid(v13)
    } else {
        let v15: bool = 7i32 == v11;
        if v15 {
            let v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
            let v17: bool = &*v16 == "--all";
            if v17 {
                let v18: US4 = US4::US4_EveryMatch;
                US3::US3_Valid(v18)
            } else {
                US3::US3_Invalid
            }
        } else {
            let v22: bool = 8i32 == v11;
            if v22 {
                let v23: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 7i32 as usize).unwrap_or_default());
                let v24: u64 = v23.parse::<u64>().unwrap_or(0);
                let v25: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
                let v26: bool = &*v25 == "--expect";
                if v26 {
                    let v27: bool = 0u64 < v24;
                    if v27 {
                        let v28: US4 = US4::US4_ExactlyMatches(v24);
                        US3::US3_Valid(v28)
                    } else {
                        US3::US3_Invalid
                    }
                } else {
                    US3::US3_Invalid
                }
            } else {
                US3::US3_Invalid
            }
        }
    };
    match &v10 {
        US1::US1_Invalid => {
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace mode must be check or apply"); } LIT.with(|lit| lit.clone()) };
            US0::US0_ReplacementRejected(v38)
        }
        _ => {
            match &v37 {
                US3::US3_Invalid => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expects <check|apply> <root> <relative> <old> <new> [--expect N | --all]"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReplacementRejected(v40)
                }
                _ => {
                    match v10 {
                        US1::US1_Valid(v42) => {
                            let v42: US2 = v42;
                            match v37 {
                                US3::US3_Valid(v43) => {
                                    let v43: US4 = v43;
                                    let v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 4i32 as usize).unwrap_or_default());
                                    let v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 5i32 as usize).unwrap_or_default());
                                    let v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
                                    let v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
                                    let v48: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v46), &*v47).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let v49: bool = &*v48 == "";
                                    if v49 {
                                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target escapes the root"); } LIT.with(|lit| lit.clone()) };
                                        US0::US0_ReplacementRejected(v50)
                                    } else {
                                        let v52: bool = &*v44 == "";
                                        if v52 {
                                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old text is empty"); } LIT.with(|lit| lit.clone()) };
                                            US0::US0_ReplacementRejected(v53)
                                        } else {
                                            let v55: bool = &*v44 == &*v45;
                                            if v55 {
                                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old and new text are identical"); } LIT.with(|lit| lit.clone()) };
                                                US0::US0_ReplacementRejected(v56)
                                            } else {
                                                let v58: bool = regular_file_4(v48.clone());
                                                if v58 {
                                                    let mut v59: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v48).unwrap_or_default());
                                                    let v60: u64 = v59.match_indices(&*v44).count() as u64;
                                                    let v67: bool = match &v43 {
                                                        US4::US4_EveryMatch => {
                                                            let v64: bool = 0u64 < v60;
                                                            v64
                                                        }
                                                        US4::US4_ExactlyMatches(v62) => {
                                                            let v62: u64 = *v62;
                                                            let v63: bool = v60 == v62;
                                                            v63
                                                        }
                                                        US4::US4_ExactlyOneMatch => {
                                                            let v61: bool = v60 == 1u64;
                                                            v61
                                                        }
                                                    };
                                                    if v67 {
                                                        let v70: Rc<str> = match &v43 {
                                                            US4::US4_EveryMatch => {
                                                                let v68: Rc<str> = Rc::<str>::from(v59.replace(&*v44, &*v45));
                                                                v68
                                                            }
                                                            _ => {
                                                                let v69: Rc<str> = std::rc::Rc::<str>::from(v59.replacen(&*v44, &*v45, v60 as usize));
                                                                v69
                                                            }
                                                        };
                                                        match v42 {
                                                            US2::US2_ApplyReplacement => {
                                                                let v97: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v48).unwrap_or_default());
                                                                let v98: bool = &*v97 != &*v59;
                                                                if v98 {
                                                                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace: "); } LIT.with(|lit| lit.clone()) }, v48));
                                                                    let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" changed since it was read; nothing written, run it again"); } LIT.with(|lit| lit.clone()) }));
                                                                    US0::US0_ReplacementRejected(v100)
                                                                } else {
                                                                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v48.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
                                                                    let v103: bool = std::fs::write(&*v102, &*v70).is_ok();
                                                                    let v116: US5 = if v103 {
                                                                        let v104: i32 = 40i32;
                                                                        let v105: bool = renamed_within_5(v48.clone(), v102.clone(), v104);
                                                                        if v105 {
                                                                            let v106: bool = std::fs::read_to_string(&*v48).is_ok_and(|value| value == *v70);
                                                                            if v106 {
                                                                                US5::US5_Replaced
                                                                            } else {
                                                                                US5::US5_ReadbackMismatch
                                                                            }
                                                                        } else {
                                                                            let v110: bool = std::fs::remove_file(&*v102).is_ok() || !std::path::Path::new(&*v102).exists();
                                                                            if v110 {
                                                                                US5::US5_RenameFailed
                                                                            } else {
                                                                                US5::US5_RenameFailedTemporaryKept
                                                                            }
                                                                        }
                                                                    } else {
                                                                        US5::US5_TemporaryWriteFailed
                                                                    };
                                                                    match &v116 {
                                                                        US5::US5_Replaced => {
                                                                            let v117: u64 = v59.len() as u64;
                                                                            let v118: u64 = v70.len() as u64;
                                                                            let v119: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode=apply path="); } LIT.with(|lit| lit.clone()) }, v48));
                                                                            let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v121: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                            let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v121, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v129: Rc<str> = match v43 {
                                                                                US4::US4_EveryMatch => {
                                                                                    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                                    v126
                                                                                }
                                                                                US4::US4_ExactlyMatches(v124) => {
                                                                                    let v124: u64 = v124;
                                                                                    let v125: Rc<str> = std::rc::Rc::<str>::from((v124).to_string());
                                                                                    v125
                                                                                }
                                                                                US4::US4_ExactlyOneMatch => {
                                                                                    let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                                    v123
                                                                                }
                                                                            };
                                                                            let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v129));
                                                                            let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v130));
                                                                            let v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                            let v133: Rc<str> = std::rc::Rc::<str>::from((v117).to_string());
                                                                            let v134: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v133));
                                                                            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v136: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                                                                            let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
                                                                            let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v137, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v139: Rc<str> = std::rc::Rc::<str>::from((v132).to_string());
                                                                            let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                                                                            let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v140));
                                                                            US0::US0_ReplacementReceipt(v141)
                                                                        }
                                                                        _ => {
                                                                            let v152: Rc<str> = match v116 {
                                                                                US5::US5_ChangedSinceRead => {
                                                                                    let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                    v147
                                                                                }
                                                                                US5::US5_ReadbackMismatch => {
                                                                                    let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v146
                                                                                }
                                                                                US5::US5_RenameFailed => {
                                                                                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v144
                                                                                }
                                                                                US5::US5_RenameFailedTemporaryKept => {
                                                                                    let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v145
                                                                                }
                                                                                US5::US5_TemporaryWriteFailed => {
                                                                                    let v143: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v143
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            US0::US0_ReplacementRejected(v152)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            US2::US2_CheckReplacement => {
                                                                let v71: u64 = v59.len() as u64;
                                                                let v72: u64 = v70.len() as u64;
                                                                let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode=check path="); } LIT.with(|lit| lit.clone()) }, v48));
                                                                let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v73, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) }));
                                                                let v75: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) }));
                                                                let v83: Rc<str> = match v43 {
                                                                    US4::US4_EveryMatch => {
                                                                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                        v80
                                                                    }
                                                                    US4::US4_ExactlyMatches(v78) => {
                                                                        let v78: u64 = v78;
                                                                        let v79: Rc<str> = std::rc::Rc::<str>::from((v78).to_string());
                                                                        v79
                                                                    }
                                                                    US4::US4_ExactlyOneMatch => {
                                                                        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                        v77
                                                                    }
                                                                };
                                                                let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v83));
                                                                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v84));
                                                                let v86: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                let v87: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                                                                let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v87));
                                                                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v88, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
                                                                let v90: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                                                                let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v90));
                                                                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                                                                let v93: Rc<str> = std::rc::Rc::<str>::from((v86).to_string());
                                                                let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                                                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v94));
                                                                US0::US0_ReplacementReceipt(v95)
                                                            }
                                                        }
                                                    } else {
                                                        let v164: Rc<str> = match v43 {
                                                            US4::US4_EveryMatch => {
                                                                let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                v161
                                                            }
                                                            US4::US4_ExactlyMatches(v159) => {
                                                                let v159: u64 = v159;
                                                                let v160: Rc<str> = std::rc::Rc::<str>::from((v159).to_string());
                                                                v160
                                                            }
                                                            US4::US4_ExactlyOneMatch => {
                                                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                v158
                                                            }
                                                        };
                                                        let v165: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expected "); } LIT.with(|lit| lit.clone()) }, v164));
                                                        let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" match(es), observed matches="); } LIT.with(|lit| lit.clone()) }));
                                                        let v167: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                        let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v167));
                                                        let v169: Rc<str> = Rc::<str>::from(format!("{}{}", v168, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (use --expect N or --all for repeated text)"); } LIT.with(|lit| lit.clone()) }));
                                                        US0::US0_ReplacementRejected(v169)
                                                    }
                                                } else {
                                                    let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target is not a regular file"); } LIT.with(|lit| lit.clone()) };
                                                    US0::US0_ReplacementRejected(v172)
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
fn text_replace_report_0() -> (i32, Rc<str>) {
    let v0: US0 = text_replace_outcome_1();
    match v0 {
        US0::US0_ReplacementReceipt(v1) => {
            let v1: Rc<str> = v1;
            (0i32, v1)
        }
        US0::US0_ReplacementRejected(v2) => {
            let mut v2: Rc<str> = v2;
            (2i32, v2)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (i32, Rc<str>)> = Rc::new(move || -> (i32, Rc<str>) {
        text_replace_report_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_text_replace_outcome() -> (i32, Rc<str>) {
    closure0()()
}
