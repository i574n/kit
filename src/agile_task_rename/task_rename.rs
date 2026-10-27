#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
}
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn binding_declared_3(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ("); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v1)), v4));
    let v9: bool = v0.starts_with(&*v8);
    if v9 {
        true
    } else {
        let v10: i32 = v0.find(&*v5).map(|offset| offset as i32).unwrap_or(-1);
        let v11: bool = v10 < 0i32;
        let v12: bool = v11 == false;
        v12
    }
}
fn state_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let v23: US2 = if v11 {
            let v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v17: bool = std::fs::remove_file(&*v10).is_ok() || !std::path::Path::new(&*v10).exists();
                if v17 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v23 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v32: Rc<str> = match &v23 {
                    US2::US2_ReadbackMismatch => {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US2::US2_RenameFailed => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v32.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn renamed_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v5, &*v5, &*v5].concat());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    match &v10 {
        US0::US0_FileMissing(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v15)), v12));
            let v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US0::US0_FileReadable(v19) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let v21: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v20);
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v1, &*v24, &*v5].concat());
            let v26: i32 = 0i32;
            let v27: i32 = declared_from_2(v21.clone(), v25.clone(), v26);
            let v28: bool = v27 < 0i32;
            let v29: bool = v28 == false;
            let v30: bool = v29 == false;
            if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v31));
                let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v37
            } else {
                let v38: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v23, &*v2, &*v24, &*v5].concat());
                let v39: i32 = 0i32;
                let v40: i32 = declared_from_2(v21.clone(), v38.clone(), v39);
                let v41: bool = v40 < 0i32;
                let v42: bool = v41 == false;
                if v42 {
                    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id "); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v2));
                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already taken"); } LIT.with(|lit| lit.clone()) };
                    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v1));
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v48, v49)), v46));
                    let v52: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v51 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v52
                } else {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                    let mut v55: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v53, &v54));
                    let v56: Rc<str> = std::rc::Rc::<str>::from(v55.to_ascii_lowercase());
                    let v57: bool = binding_declared_3(v21.clone(), v56.clone());
                    if v57 {
                        let mut v58: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v53, &v54));
                        let v59: Rc<str> = std::rc::Rc::<str>::from(v58.to_ascii_lowercase());
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                        let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v1));
                        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v65, v66)), v63));
                        let v69: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v68 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v69
                    } else {
                        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning_of_"); } LIT.with(|lit| lit.clone()) };
                        let mut v71: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v53, &v54));
                        let v72: Rc<str> = std::rc::Rc::<str>::from(v71.to_ascii_lowercase());
                        let v73: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v72, &*v5, &*v5, &*v5].concat());
                        let v74: bool = binding_declared_3(v21.clone(), v73.clone());
                        if v74 {
                            let mut v75: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v53, &v54));
                            let v76: Rc<str> = std::rc::Rc::<str>::from(v75.to_ascii_lowercase());
                            let v77: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v76, &*v5, &*v5, &*v5].concat());
                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the name inl "); } LIT.with(|lit| lit.clone()) };
                            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v77));
                            let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already declared"); } LIT.with(|lit| lit.clone()) };
                            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                            let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                            let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v1));
                            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v84)), v81));
                            let v87: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v86 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v87
                        } else {
                            let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93): (Rc<str>, i32, i32, Rc<str>, i32, Rc<str>) = agile_rename_rewrite::kit_agile_renamed_text(&*v21, &*v1, &*v2);
                            let v94: US1 = state_written_4(v6.clone(), v20.clone(), v88.clone());
                            match &v94 {
                                US1::US1_StateWritten => {
                                    let v95: u64 = v21.len() as u64;
                                    let v96: u64 = v88.len() as u64;
                                    let v97: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v3);
                                    let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile rename ok old="); } LIT.with(|lit| lit.clone()) };
                                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v1));
                                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" new="); } LIT.with(|lit| lit.clone()) };
                                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v99, v100)), v2));
                                    let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" items="); } LIT.with(|lit| lit.clone()) };
                                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                                    let v105: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                                    let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_rewritten="); } LIT.with(|lit| lit.clone()) };
                                    let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
                                    let v108: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
                                    let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
                                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps_in="); } LIT.with(|lit| lit.clone()) };
                                    let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
                                    let v112: bool = &*v91 == "";
                                    let v113: Rc<str> = if v112 {
                                        v53.clone()
                                    } else {
                                        v91.clone()
                                    };
                                    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v113));
                                    let v115: Rc<str> = std::rc::Rc::<str>::from((v92).to_string());
                                    let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_mentioning_old="); } LIT.with(|lit| lit.clone()) };
                                    let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v115));
                                    let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" titles_in="); } LIT.with(|lit| lit.clone()) };
                                    let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
                                    let v120: bool = &*v93 == "";
                                    let v121: Rc<str> = if v120 {
                                        v53.clone()
                                    } else {
                                        v93.clone()
                                    };
                                    let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v121));
                                    let v124: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v104, v114)), v122));
                                    let v125: Rc<str> = std::rc::Rc::<str>::from((v95).to_string());
                                    let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                    let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v125));
                                    let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                    let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v128));
                                    let v130: Rc<str> = std::rc::Rc::<str>::from((v96).to_string());
                                    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", v129, v130));
                                    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
                                    let v134: Rc<str> = std::rc::Rc::<str>::from((v97).to_string());
                                    let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v124, Rc::<str>::from(format!("{}{}", v133, v134))));
                                    let v137: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v136.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v137
                                }
                                _ => {
                                    let v141: Rc<str> = match &v94 {
                                        US1::US1_StateChangedSinceRead => {
                                            let v138: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v138.clone()
                                        }
                                        US1::US1_StateWriteFailed(v139) => {
                                            let v139: Rc<str> = v139.clone();
                                            v139.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                                    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v1));
                                    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                    let v146: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v143, v144)), v141));
                                    let v147: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v146 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                    v147
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_6() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_5() -> i32 {
    let v0: i32 = command_line_argument_count_6();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn id_bytes_valid_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v2 <= v1;
        if v3 {
            return true;
        } else {
            let v4: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v5: bool = 65i32 <= v4;
            let v11: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                let v7: bool = 48i32 <= v4;
                if v7 {
                    let v8: bool = v4 <= 57i32;
                    v8
                } else {
                    let v9: bool = v4 == 45i32;
                    v9
                }
            };
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v12, v2);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn rename_entry_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: i32 = 3i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = argument_count_5();
    let v5: bool = v4 == 4i32;
    let v6: bool = v5 == false;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile rename <root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v1));
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v7));
        let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let v14: i32 = (v3.clone().len() as i32);
        let v15: bool = v14 <= 0i32;
        let v18: bool = if v15 {
            false
        } else {
            let v16: i32 = 0i32;
            id_bytes_valid_7(v3.clone(), v16, v14)
        };
        let v19: bool = v18 == false;
        if v19 {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task id is upper-case letters, digits and -"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v1));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v23)), v20));
            let v26: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v25 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v26
        } else {
            let v27: i32 = v3.as_bytes().get(0i32 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v28: bool = 48i32 <= v27;
            let v30: bool = if v28 {
                let v29: bool = v27 <= 57i32;
                v29
            } else {
                false
            };
            if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v31));
                let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v37
            } else {
                let v38: bool = &*v1 == &*v3;
                if v38 {
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the new id equals the old one"); } LIT.with(|lit| lit.clone()) };
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile rename "); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v41, v42)), v39));
                    let v45: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v44 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v45
                } else {
                    let v46: i32 = 1i32;
                    let v47: Rc<str> = usize::try_from(v46).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    renamed_1(v47.clone(), v1.clone(), v3.clone())
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        rename_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_rename_run() -> i32 {
    closure0()()
}
