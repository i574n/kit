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
    US2_ChangedSinceRead,
}
#[derive(Clone)]
enum US4 {
    US4_ViewList,
    US4_ViewNext,
}
#[derive(Clone)]
enum US3 {
    US3_VerbRead(US4),
    US3_VerbRefused,
}
#[derive(Clone)]
enum US5 {
    US5_ScanPlain,
    US5_ScanQuoted,
    US5_ScanEscaped,
}
#[derive(Clone)]
enum US7 {
    US7_ViewsCommand,
    US7_ViewCommand,
    US7_ViewAddCommand,
}
#[derive(Clone)]
enum US6 {
    US6_CommandRead(US7),
    US6_CommandUnknown,
}
fn views_loaded_3(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2.clone()));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2.clone()));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2.clone()));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US0 = if v7 {
        US0::US0_FileReadable(v6)
    } else {
        US0::US0_FileMissing(v6)
    };
    let (mut v16, mut v17): (bool, Rc<str>) = match &v10 {
        US0::US0_FileMissing(v11) => {
            let v11: Rc<str> = v11.clone();
            (false, v2)
        }
        _ => {
            let (mut v14, mut v15): (bool, Rc<str>) = match v10 {
                US0::US0_FileReadable(v12) => {
                    let v12: Rc<str> = v12;
                    let v13: Rc<str> = std::fs::read_to_string(&*v12).unwrap_or_default().into();
                    (true, v13)
                }
                _ => unreachable!(),
            };
            (v14, v15)
        }
    };
    (v16, v17)
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn declared_from_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
                v2 = v9;
                continue;
            }
        }
    }
}
fn renamed_within_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn views_written_6(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    let v7: bool = if v1 {
        let v4: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
        v4
    } else {
        let v5: bool = std::path::Path::new(&*v0).is_file();
        let v6: bool = v5 == false;
        v6
    };
    if v7 {
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v9: bool = std::fs::write(&*v8, &*v3).is_ok();
        let v22: US2 = if v9 {
            let v10: i32 = 40i32;
            let v11: bool = renamed_within_7(v0.clone(), v8.clone(), v10);
            if v11 {
                let v12: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v3);
                if v12 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v16: bool = std::fs::remove_file(&*v8).is_ok() || !std::path::Path::new(&*v8).exists();
                if v16 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v22 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v33: Rc<str> = match v22 {
                    US2::US2_ChangedSinceRead => {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v28
                    }
                    US2::US2_ReadbackMismatch => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    US2::US2_RenameFailed => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v26
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v33)
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn views_module_registered_8(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2.clone()));
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2.clone()));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2.clone()));
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US0 = if v7 {
        US0::US0_FileReadable(v6.clone())
    } else {
        US0::US0_FileMissing(v6.clone())
    };
    match v10 {
        US0::US0_FileMissing(v11) => {
            let v11: Rc<str> = v11;
            US1::US1_StateWritten
        }
        US0::US0_FileReadable(v13) => {
            let v13: Rc<str> = v13;
            let v14: Rc<str> = std::fs::read_to_string(&*v13).unwrap_or_default().into();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    views\n"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16.clone()));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v2.clone()));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v2.clone()));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v2.clone()));
            let v21: bool = v14.contains(&*v20);
            let v24: bool = if v21 {
                true
            } else {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    views"); } LIT.with(|lit| lit.clone()) };
                let v23: bool = v14.ends_with(&*v22);
                v23
            };
            if v24 {
                US1::US1_StateWritten
            } else {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
                let v27: bool = v14.contains(&*v26);
                if v27 {
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26.clone(), v16));
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2.clone()));
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v2.clone()));
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v2));
                    let v32: Rc<str> = Rc::<str>::from(v14.replace(&*v26, &*v31));
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v6.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
                    let v34: bool = std::fs::write(&*v33, &*v32).is_ok();
                    let v47: US2 = if v34 {
                        let v35: i32 = 40i32;
                        let v36: bool = renamed_within_7(v6.clone(), v33.clone(), v35);
                        if v36 {
                            let v37: bool = std::fs::read_to_string(&*v6).is_ok_and(|value| value == *v32);
                            if v37 {
                                US2::US2_Replaced
                            } else {
                                US2::US2_ReadbackMismatch
                            }
                        } else {
                            let v41: bool = std::fs::remove_file(&*v33).is_ok() || !std::path::Path::new(&*v33).exists();
                            if v41 {
                                US2::US2_RenameFailed
                            } else {
                                US2::US2_RenameFailedTemporaryKept
                            }
                        }
                    } else {
                        US2::US2_TemporaryWriteFailed
                    };
                    match &v47 {
                        US2::US2_Replaced => {
                            US1::US1_StateWritten
                        }
                        _ => {
                            let v58: Rc<str> = match v47 {
                                US2::US2_ChangedSinceRead => {
                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v53
                                }
                                US2::US2_ReadbackMismatch => {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                    v52
                                }
                                US2::US2_RenameFailed => {
                                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                    v50
                                }
                                US2::US2_RenameFailedTemporaryKept => {
                                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                    v51
                                }
                                US2::US2_TemporaryWriteFailed => {
                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                    v49
                                }
                                _ => unreachable!(),
                            };
                            US1::US1_StateWriteFailed(v58)
                        }
                    }
                } else {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list views in"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_StateWriteFailed(v61)
                }
            }
        }
    }
}
fn view_stored_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let (mut v4, mut v5): (bool, Rc<str>) = views_loaded_3(v0.clone());
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(v10.replace(&*v7, &*v11));
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(v12.replace(&*v13, &*v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = concat5_4(v7.clone(), v21, v7.clone(), v22.clone(), v22.clone());
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v23));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v22.clone()));
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v22.clone()));
    let v29: i32 = 0i32;
    let v30: i32 = declared_from_5(v5.clone(), v28, v29);
    let v31: bool = v30 < 0i32;
    let v32: bool = v31 == false;
    let v130: Rc<str> = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(v1.replace(&*v35, &*v36));
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v37));
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v22.clone()));
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v22.clone()));
        let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v22.clone()));
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
        let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v7, &*v11));
        let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v13, &*v14));
        let v46: Rc<str> = Rc::<str>::from(v45.replace(&*v16, &*v17));
        let v47: Rc<str> = Rc::<str>::from(v46.replace(&*v19, &*v20));
        let v48: Rc<str> = concat5_4(v7.clone(), v47, v7.clone(), v22.clone(), v22.clone());
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v41));
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v42));
        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v48));
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v49.clone()));
        let v54: Rc<str> = Rc::<str>::from(v2.replace(&*v8, &*v9));
        let v55: Rc<str> = Rc::<str>::from(v54.replace(&*v7, &*v11));
        let v56: Rc<str> = Rc::<str>::from(v55.replace(&*v13, &*v14));
        let v57: Rc<str> = Rc::<str>::from(v56.replace(&*v16, &*v17));
        let v58: Rc<str> = Rc::<str>::from(v57.replace(&*v19, &*v20));
        let v59: Rc<str> = concat5_4(v7.clone(), v58, v7.clone(), v22.clone(), v22.clone());
        let v60: Rc<str> = Rc::<str>::from(v3.replace(&*v8, &*v9));
        let v61: Rc<str> = Rc::<str>::from(v60.replace(&*v7, &*v11));
        let v62: Rc<str> = Rc::<str>::from(v61.replace(&*v13, &*v14));
        let v63: Rc<str> = Rc::<str>::from(v62.replace(&*v16, &*v17));
        let v64: Rc<str> = Rc::<str>::from(v63.replace(&*v19, &*v20));
        let v65: Rc<str> = concat5_4(v7.clone(), v64, v7.clone(), v22.clone(), v22.clone());
        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v59));
        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v49));
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v65));
        let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v66));
        let v71: i32 = v5.get(..v30 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let v72: i32 = v5.get(v30 as usize..).and_then(|rest| rest.find(&*v13)).map(|found| (found + v30 as usize) as i32).unwrap_or(-1);
        let v73: bool = v72 < 0i32;
        let v75: i32 = if v73 {
            let v74: i32 = (v5.clone().len() as i32);
            v74
        } else {
            v72
        };
        let v76: Rc<str> = std::rc::Rc::<str>::from(v5.get(0i32 as usize..v71 as usize).unwrap_or(""));
        let v77: i32 = (v5.clone().len() as i32);
        let v78: Rc<str> = std::rc::Rc::<str>::from(v5.get(v75 as usize..v77 as usize).unwrap_or(""));
        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v70));
        let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v78));
        let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v22.clone()));
        let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v22.clone()));
        v82
    } else {
        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view_"); } LIT.with(|lit| lit.clone()) };
        let v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let v87: Rc<str> = Rc::<str>::from(v1.replace(&*v85, &*v86));
        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v87));
        let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v22.clone()));
        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v22.clone()));
        let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v22.clone()));
        let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = View ("); } LIT.with(|lit| lit.clone()) };
        let v93: Rc<str> = Rc::<str>::from(v1.replace(&*v8, &*v9));
        let v94: Rc<str> = Rc::<str>::from(v93.replace(&*v7, &*v11));
        let v95: Rc<str> = Rc::<str>::from(v94.replace(&*v13, &*v14));
        let v96: Rc<str> = Rc::<str>::from(v95.replace(&*v16, &*v17));
        let v97: Rc<str> = Rc::<str>::from(v96.replace(&*v19, &*v20));
        let v98: Rc<str> = concat5_4(v7.clone(), v97, v7.clone(), v22.clone(), v22.clone());
        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v91));
        let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v92));
        let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v98));
        let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v99.clone()));
        let v104: Rc<str> = Rc::<str>::from(v2.replace(&*v8, &*v9));
        let v105: Rc<str> = Rc::<str>::from(v104.replace(&*v7, &*v11));
        let v106: Rc<str> = Rc::<str>::from(v105.replace(&*v13, &*v14));
        let v107: Rc<str> = Rc::<str>::from(v106.replace(&*v16, &*v17));
        let v108: Rc<str> = Rc::<str>::from(v107.replace(&*v19, &*v20));
        let v109: Rc<str> = concat5_4(v7.clone(), v108, v7.clone(), v22.clone(), v22.clone());
        let v110: Rc<str> = Rc::<str>::from(v3.replace(&*v8, &*v9));
        let v111: Rc<str> = Rc::<str>::from(v110.replace(&*v7, &*v11));
        let v112: Rc<str> = Rc::<str>::from(v111.replace(&*v13, &*v14));
        let v113: Rc<str> = Rc::<str>::from(v112.replace(&*v16, &*v17));
        let v114: Rc<str> = Rc::<str>::from(v113.replace(&*v19, &*v20));
        let v115: Rc<str> = concat5_4(v7.clone(), v114, v7.clone(), v22.clone(), v22.clone());
        let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v109));
        let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v99));
        let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v115));
        let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v116));
        let v121: bool = &*v5 == "";
        let v123: Rc<str> = if v121 {
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell\nunion agile_view =\n    | View :: string * string * string -> agile_view\n"); } LIT.with(|lit| lit.clone()) };
            v122
        } else {
            v5.clone()
        };
        let v124: bool = v123.ends_with(&*v13);
        let v125: Rc<str> = if v124 {
            v22.clone()
        } else {
            v13.clone()
        };
        let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v125));
        let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v120));
        let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v13));
        let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v22.clone()));
        v129
    };
    let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), v131));
    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v22.clone()));
    let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v22.clone()));
    let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v22.clone()));
    let v136: US1 = views_written_6(v135, v4, v5, v130);
    let v138: US1 = match &v136 {
        US1::US1_StateWritten => {
            views_module_registered_8(v0)
        }
        _ => {
            v136
        }
    };
    match &v138 {
        US1::US1_StateWritten => {
            let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" verb="); } LIT.with(|lit| lit.clone()) };
            let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" flags='"); } LIT.with(|lit| lit.clone()) };
            let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
            let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v2));
            let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v140));
            let v144: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v3));
            let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v141));
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile view-add ok name="); } LIT.with(|lit| lit.clone()) };
            let v149: Rc<str> = if v32 {
                let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=yes"); } LIT.with(|lit| lit.clone()) };
                v147
            } else {
                let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced=no"); } LIT.with(|lit| lit.clone()) };
                v148
            };
            let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v146, v1));
            let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v145));
            let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v149));
            let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v22));
            let v154: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v153.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v154
        }
        _ => {
            let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add "); } LIT.with(|lit| lit.clone()) };
            let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v160: Rc<str> = match v138 {
                US1::US1_StateChangedSinceRead => {
                    let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v157
                }
                US1::US1_StateWriteFailed(v158) => {
                    let v158: Rc<str> = v158;
                    v158
                }
                _ => unreachable!(),
            };
            let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v1));
            let v162: Rc<str> = Rc::<str>::from(format!("{}{}", v161, v156));
            let v163: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v160));
            let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v163, v22));
            let v165: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v164 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v165
        }
    }
}
fn view_add_run_1() -> i32 {
    let v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: bool = !(v3).is_empty() && (v3).bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == 45u8);
    if v4 {
        let v5: i32 = 3i32;
        let v6: Rc<str> = usize::try_from(v5).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v7: bool = "list" == &*v6;
        let v15: US3 = if v7 {
            let v8: US4 = US4::US4_ViewList;
            US3::US3_VerbRead(v8)
        } else {
            let v10: bool = "next" == &*v6;
            if v10 {
                let v11: US4 = US4::US4_ViewNext;
                US3::US3_VerbRead(v11)
            } else {
                US3::US3_VerbRefused
            }
        };
        match v15 {
            US3::US3_VerbRead(v24) => {
                let v24: US4 = v24;
                let v25: bool = std::env::args().skip(6i32 as usize).all(|argument| !argument.is_empty() && !argument.contains(char::is_whitespace));
                if v25 {
                    let v29: Rc<str> = match v24 {
                        US4::US4_ViewList => {
                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                            v26
                        }
                        US4::US4_ViewNext => {
                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                            v27
                        }
                    };
                    let v30: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(6i32 as usize).collect::<Vec<_>>().join(" ").as_str());
                    view_stored_2(v1, v3, v29, v30)
                } else {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: each stored flag must be one word without spaces"); } LIT.with(|lit| lit.clone()) };
                    let v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v33
                }
            }
            US3::US3_VerbRefused => {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are "); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list|next"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view verbs are list|next"); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18.clone()));
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v18.clone()));
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v18));
                let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v23
            }
        }
    } else {
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view-add: view names are lowercase letters, digits and hyphens"); } LIT.with(|lit| lit.clone()) };
        let v38: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v37 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v38
    }
}
fn field_end_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US5) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US5::US5_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US5 = match v3 {
                    US5::US5_ScanEscaped => {
                        US5::US5_ScanQuoted
                    }
                    US5::US5_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US5::US5_ScanQuoted
                        } else {
                            US5::US5_ScanPlain
                        }
                    }
                    US5::US5_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US5::US5_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US5::US5_ScanPlain
                            } else {
                                US5::US5_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25);
                continue;
            }
        }
    }
}
fn skip_spaces_13(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            v1 = v4;
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US5 = US5::US5_ScanPlain;
            let v6: i32 = field_end_12(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_13(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_15(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_4(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_4(v4, v13, v24, v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26);
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                v1 = v28;
                continue;
            }
        }
    }
}
fn unquoted_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_15(v3, v4, v5, v6, v7)
}
fn view_at_10(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 0i32;
    let v5: i32 = field_start_11(v0.clone(), v2, v3, v4);
    let v6: US5 = US5::US5_ScanPlain;
    let v7: i32 = field_end_12(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let v9: Rc<str> = unquoted_14(v8);
    let v10: i32 = 1i32;
    let v11: i32 = field_start_11(v0.clone(), v2, v3, v10);
    let v12: US5 = US5::US5_ScanPlain;
    let v13: i32 = field_end_12(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_14(v14);
    let v16: i32 = 2i32;
    let v17: i32 = field_start_11(v0.clone(), v2, v3, v16);
    let v18: US5 = US5::US5_ScanPlain;
    let v19: i32 = field_end_12(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_14(v20);
    (v9, v15, v21)
}
fn view_run_9() -> i32 {
    let v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: bool = &*v3 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile view <root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
        let v6: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v5 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v6
    } else {
        let (mut v7, mut v8): (bool, Rc<str>) = views_loaded_3(v1.clone());
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(v3.replace(&*v11, &*v12));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(v13.replace(&*v10, &*v14));
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(v15.replace(&*v16, &*v17));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(v18.replace(&*v19, &*v20));
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(v21.replace(&*v22, &*v23));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = concat5_4(v10.clone(), v24, v10.clone(), v25.clone(), v25.clone());
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v26));
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v25.clone()));
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v25.clone()));
        let v32: i32 = 0i32;
        let v33: i32 = declared_from_5(v8.clone(), v31, v32);
        let v34: bool = v33 < 0i32;
        if v34 {
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile view: no view named "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" in "); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v37));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v25.clone()));
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v25.clone()));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v25));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; agile views <root> lists them"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v3));
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v36));
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v41));
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v42));
            let v47: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v46 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v47
        } else {
            let (mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, Rc<str>) = view_at_10(v8, v33);
            let v51: i32 = { let extra = std::env::args().skip(5i32 as usize).collect::<Vec<_>>(); match std::env::current_exe().and_then(|exe| std::process::Command::new(exe).arg("agile").arg(&*v49).arg(&*v1).args((v50).split_whitespace()).args(&extra).status()) { Ok(status) => status.code().unwrap_or(1), Err(error) => { eprintln!("kit error: agile view could not start: {}", error); 2 } } };
            v51
        }
    }
}
fn views_listed_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View ("); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("View (\""); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5.clone()));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5.clone()));
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v5));
        let v10: i32 = declared_from_5(v0.clone(), v9, v1);
        let v11: bool = v10 < 0i32;
        if v11 {
            return v2;
        } else {
            let (mut v12, mut v13, mut v14): (Rc<str>, Rc<str>, Rc<str>) = view_at_10(v0.clone(), v10);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v15.clone()));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v13));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v14));
            let v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let v21: i32 = v10.wrapping_add(1i32);
            let v22: i32 = v2.wrapping_add(1i32);
            (v1, v2) = (v21, v22);
            continue;
        }
    }
}
fn views_run_16() -> i32 {
    let v0: i32 = 1i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let (mut v2, mut v3): (bool, Rc<str>) = views_loaded_3(v1.clone());
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = views_listed_17(v3, v4, v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile views ok views="); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = std::rc::Rc::<str>::from((v6).to_string().as_str());
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" path="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/views.spi"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v10));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11.clone()));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v11.clone()));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v11.clone()));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v9));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v11));
    let v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v20
}
fn views_entry_0() -> i32 {
    let v0: i32 = 0i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: bool = "views" == &*v1;
    let v14: US6 = if v2 {
        let v3: US7 = US7::US7_ViewsCommand;
        US6::US6_CommandRead(v3)
    } else {
        let v5: bool = "view" == &*v1;
        if v5 {
            let v6: US7 = US7::US7_ViewCommand;
            US6::US6_CommandRead(v6)
        } else {
            let v8: bool = "view-add" == &*v1;
            if v8 {
                let v9: US7 = US7::US7_ViewAddCommand;
                US6::US6_CommandRead(v9)
            } else {
                US6::US6_CommandUnknown
            }
        }
    };
    match v14 {
        US6::US6_CommandRead(v15) => {
            let v15: US7 = v15;
            match v15 {
                US7::US7_ViewAddCommand => {
                    view_add_run_1()
                }
                US7::US7_ViewCommand => {
                    view_run_9()
                }
                US7::US7_ViewsCommand => {
                    views_run_16()
                }
            }
        }
        US6::US6_CommandUnknown => {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile views: expected views, view or view-add"); } LIT.with(|lit| lit.clone()) };
            let v23: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v22 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v23
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        views_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_views_run() -> i32 {
    closure0()()
}
