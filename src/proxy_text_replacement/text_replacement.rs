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
fn regular_file_4(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_file();
    v8
}
fn text_replace_outcome_1() -> US0 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 1i32 as usize).unwrap_or_default());
    let v2: bool = &*v1 == "check";
    let v10: US1 = if v2 {
        let v3: US2 = US2::US2_CheckReplacement;
        US1::US1_Valid(v3.clone())
    } else {
        let v5: bool = &*v1 == "apply";
        if v5 {
            let mut v6: US2 = US2::US2_ApplyReplacement;
            US1::US1_Valid(v6.clone())
        } else {
            US1::US1_Invalid
        }
    };
    let v11: i32 = proxy_argument_count_2();
    let v12: bool = 6i32 == v11;
    let v37: US3 = if v12 {
        let v13: US4 = US4::US4_ExactlyOneMatch;
        US3::US3_Valid(v13.clone())
    } else {
        let v15: bool = 7i32 == v11;
        if v15 {
            let v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 6i32 as usize).unwrap_or_default());
            let v17: bool = &*v16 == "--all";
            if v17 {
                let v18: US4 = US4::US4_EveryMatch;
                US3::US3_Valid(v18.clone())
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
                        US3::US3_Valid(v28.clone())
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
            US0::US0_ReplacementRejected(v38.clone())
        }
        _ => {
            match &v37 {
                US3::US3_Invalid => {
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expects <check|apply> <root> <relative> <old> <new> [--expect N | --all]"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_ReplacementRejected(v40.clone())
                }
                _ => {
                    match &v10 {
                        US1::US1_Valid(v42) => {
                            let v42: US2 = v42.clone();
                            match &v37 {
                                US3::US3_Valid(v43) => {
                                    let v43: US4 = v43.clone();
                                    let v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 4i32 as usize).unwrap_or_default());
                                    let v45: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 5i32 as usize).unwrap_or_default());
                                    let v46: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 2i32 as usize).unwrap_or_default());
                                    let v47: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(2 + 3i32 as usize).unwrap_or_default());
                                    let v48: Rc<str> = kit_rust_std_fs::rooted_path(std::path::Path::new(&*v46), &*v47).map(|path| std::rc::Rc::<str>::from(path.to_string_lossy().as_ref())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
                                    let v49: bool = &*v48 == "";
                                    if v49 {
                                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target escapes the root"); } LIT.with(|lit| lit.clone()) };
                                        US0::US0_ReplacementRejected(v50.clone())
                                    } else {
                                        let v52: bool = &*v44 == "";
                                        if v52 {
                                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old text is empty"); } LIT.with(|lit| lit.clone()) };
                                            US0::US0_ReplacementRejected(v53.clone())
                                        } else {
                                            let v55: bool = &*v44 == &*v45;
                                            if v55 {
                                                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace old and new text are identical"); } LIT.with(|lit| lit.clone()) };
                                                US0::US0_ReplacementRejected(v56.clone())
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
                                                                v68.clone()
                                                            }
                                                            _ => {
                                                                let v69: Rc<str> = std::rc::Rc::<str>::from(v59.replacen(&*v44, &*v45, v60 as usize));
                                                                v69.clone()
                                                            }
                                                        };
                                                        match &v42 {
                                                            US2::US2_ApplyReplacement => {
                                                                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                                                                let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v107));
                                                                let v109: bool = std::fs::write(&*v108, &*v70).is_ok();
                                                                let v121: US5 = if v109 {
                                                                    let v110: bool = std::fs::rename(&*v108, &*v48).is_ok();
                                                                    if v110 {
                                                                        let v111: bool = std::fs::read_to_string(&*v48).is_ok_and(|value| value == *v70);
                                                                        if v111 {
                                                                            US5::US5_Replaced
                                                                        } else {
                                                                            US5::US5_ReadbackMismatch
                                                                        }
                                                                    } else {
                                                                        let v115: bool = std::fs::remove_file(&*v108).is_ok() || !std::path::Path::new(&*v108).exists();
                                                                        if v115 {
                                                                            US5::US5_RenameFailed
                                                                        } else {
                                                                            US5::US5_RenameFailedTemporaryKept
                                                                        }
                                                                    }
                                                                } else {
                                                                    US5::US5_TemporaryWriteFailed
                                                                };
                                                                match &v121 {
                                                                    US5::US5_Replaced => {
                                                                        let v122: u64 = v59.len() as u64;
                                                                        let v123: u64 = v70.len() as u64;
                                                                        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
                                                                        let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode=apply"); } LIT.with(|lit| lit.clone()) };
                                                                        let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v126, v127)), v48));
                                                                        let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                        let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                                                                        let v132: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                        let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                        let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
                                                                        let v141: Rc<str> = match &v43 {
                                                                            US4::US4_EveryMatch => {
                                                                                let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                                v138.clone()
                                                                            }
                                                                            US4::US4_ExactlyMatches(v136) => {
                                                                                let v136: u64 = *v136;
                                                                                let v137: Rc<str> = std::rc::Rc::<str>::from((v136).to_string());
                                                                                v137.clone()
                                                                            }
                                                                            US4::US4_ExactlyOneMatch => {
                                                                                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                                v135.clone()
                                                                            }
                                                                        };
                                                                        let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v131, Rc::<str>::from(format!("{}{}", v134, v141))));
                                                                        let v144: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                        let v145: Rc<str> = std::rc::Rc::<str>::from((v122).to_string());
                                                                        let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                        let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v146, v145));
                                                                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                        let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v148));
                                                                        let v150: Rc<str> = std::rc::Rc::<str>::from((v123).to_string());
                                                                        let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v150));
                                                                        let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                        let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v152));
                                                                        let v154: Rc<str> = std::rc::Rc::<str>::from((v144).to_string());
                                                                        let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v143, Rc::<str>::from(format!("{}{}", v153, v154))));
                                                                        US0::US0_ReplacementReceipt(v156.clone())
                                                                    }
                                                                    _ => {
                                                                        let v165: Rc<str> = match &v121 {
                                                                            US5::US5_ReadbackMismatch => {
                                                                                let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                v161.clone()
                                                                            }
                                                                            US5::US5_RenameFailed => {
                                                                                let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                v159.clone()
                                                                            }
                                                                            US5::US5_RenameFailedTemporaryKept => {
                                                                                let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                v160.clone()
                                                                            }
                                                                            US5::US5_TemporaryWriteFailed => {
                                                                                let v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                v158.clone()
                                                                            }
                                                                            _ => unreachable!(),
                                                                        };
                                                                        US0::US0_ReplacementRejected(v165.clone())
                                                                    }
                                                                }
                                                            }
                                                            US2::US2_CheckReplacement => {
                                                                let v71: u64 = v59.len() as u64;
                                                                let v72: u64 = v70.len() as u64;
                                                                let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode="); } LIT.with(|lit| lit.clone()) };
                                                                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit proxy text-replace ok mode=check"); } LIT.with(|lit| lit.clone()) };
                                                                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
                                                                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v75, v76)), v48));
                                                                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matches="); } LIT.with(|lit| lit.clone()) };
                                                                let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v79));
                                                                let v81: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expected="); } LIT.with(|lit| lit.clone()) };
                                                                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                                                                let v90: Rc<str> = match &v43 {
                                                                    US4::US4_EveryMatch => {
                                                                        let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                        v87.clone()
                                                                    }
                                                                    US4::US4_ExactlyMatches(v85) => {
                                                                        let v85: u64 = *v85;
                                                                        let v86: Rc<str> = std::rc::Rc::<str>::from((v85).to_string());
                                                                        v86.clone()
                                                                    }
                                                                    US4::US4_ExactlyOneMatch => {
                                                                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                        v84.clone()
                                                                    }
                                                                };
                                                                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v80, Rc::<str>::from(format!("{}{}", v83, v90))));
                                                                let v93: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                                                                let v94: Rc<str> = std::rc::Rc::<str>::from((v71).to_string());
                                                                let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v94));
                                                                let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                                                                let v99: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
                                                                let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
                                                                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v101));
                                                                let v103: Rc<str> = std::rc::Rc::<str>::from((v93).to_string());
                                                                let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v92, Rc::<str>::from(format!("{}{}", v102, v103))));
                                                                US0::US0_ReplacementReceipt(v105.clone())
                                                            }
                                                        }
                                                    } else {
                                                        let v176: Rc<str> = match &v43 {
                                                            US4::US4_EveryMatch => {
                                                                let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
                                                                v173.clone()
                                                            }
                                                            US4::US4_ExactlyMatches(v171) => {
                                                                let v171: u64 = *v171;
                                                                let v172: Rc<str> = std::rc::Rc::<str>::from((v171).to_string());
                                                                v172.clone()
                                                            }
                                                            US4::US4_ExactlyOneMatch => {
                                                                let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                                                                v170.clone()
                                                            }
                                                        };
                                                        let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace expected "); } LIT.with(|lit| lit.clone()) };
                                                        let v178: Rc<str> = Rc::<str>::from(format!("{}{}", v177, v176));
                                                        let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" match(es), observed matches="); } LIT.with(|lit| lit.clone()) };
                                                        let v180: Rc<str> = Rc::<str>::from(format!("{}{}", v178, v179));
                                                        let v181: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
                                                        let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v181));
                                                        let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (use --expect N or --all for repeated text)"); } LIT.with(|lit| lit.clone()) };
                                                        let v184: Rc<str> = Rc::<str>::from(format!("{}{}", v182, v183));
                                                        US0::US0_ReplacementRejected(v184.clone())
                                                    }
                                                } else {
                                                    let v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("text-replace target is not a regular file"); } LIT.with(|lit| lit.clone()) };
                                                    US0::US0_ReplacementRejected(v187.clone())
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
    match &v0 {
        US0::US0_ReplacementReceipt(v1) => {
            let v1: Rc<str> = v1.clone();
            (0i32, v1)
        }
        US0::US0_ReplacementRejected(v2) => {
            let v2: Rc<str> = v2.clone();
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
