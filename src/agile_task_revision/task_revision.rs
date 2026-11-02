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
    US1_ScanPlain,
    US1_ScanQuoted,
    US1_ScanEscaped,
}
#[derive(Clone)]
enum US2 {
    US2_TokenRendered(Rc<str>),
    US2_TokenRejected,
}
#[derive(Clone)]
enum US3 {
    US3_Replaced,
    US3_TemporaryWriteFailed,
    US3_RenameFailed,
    US3_RenameFailedTemporaryKept,
    US3_ReadbackMismatch,
    US3_ChangedSinceRead,
}
fn agile_edit_failure_1(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v0));
    let v7: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v6 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v7
}
fn argument_owned_2(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    v1.clone()
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US1) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US1::US1_ScanPlain => {
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
                let v25: US1 = match &v3 {
                    US1::US1_ScanEscaped => {
                        US1::US1_ScanQuoted
                    }
                    US1::US1_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US1::US1_ScanQuoted
                        } else {
                            US1::US1_ScanPlain
                        }
                    }
                    US1::US1_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US1::US1_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US1::US1_ScanPlain
                            } else {
                                US1::US1_ScanQuoted
                            }
                        }
                    }
                };
                (v1, v3) = (v10, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US1 = US1::US1_ScanPlain;
            let v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn renamed_within_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US3 {
    let v3: bool = kit_agile_records::kit_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
        let v6: bool = std::fs::write(&*v5, &*v2).is_ok();
        if v6 {
            let v7: i32 = 40i32;
            let v8: bool = renamed_within_8(v0.clone(), v5.clone(), v7);
            if v8 {
                let v9: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v9 {
                    US3::US3_Replaced
                } else {
                    US3::US3_ReadbackMismatch
                }
            } else {
                let v13: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
                if v13 {
                    US3::US3_RenameFailed
                } else {
                    US3::US3_RenameFailedTemporaryKept
                }
            }
        } else {
            US3::US3_TemporaryWriteFailed
        }
    } else {
        US3::US3_ChangedSinceRead
    }
}
fn agile_edit_entry_0() -> i32 {
    let v0: i32 = 3i32;
    let mut v1: Rc<str> = argument_owned_2(v0);
    let mut v2: i32 = agile_field_rendering::kit_agile_edit_field_position(&*v1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let v4: Rc<str> = agile_field_rendering::kit_agile_edit_field_rule(&*v1);
        let v5: i32 = 2i32;
        let v6: Rc<str> = argument_owned_2(v5);
        agile_edit_failure_1(v4.clone(), v6.clone())
    } else {
        let v8: i32 = 1i32;
        let v9: Rc<str> = argument_owned_2(v8);
        let v10: i32 = 2i32;
        let v11: Rc<str> = argument_owned_2(v10);
        let v12: i32 = 4i32;
        let v13: Rc<str> = argument_owned_2(v12);
        let v14: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = concat5_3(v9.clone(), v15.clone(), v16.clone(), v16.clone(), v16.clone());
        let v18: bool = std::path::Path::new(&*v17).is_file();
        let v21: US0 = if v18 {
            US0::US0_FileReadable(v17.clone())
        } else {
            US0::US0_FileMissing(v17.clone())
        };
        match &v21 {
            US0::US0_FileMissing(v22) => {
                let v22: Rc<str> = v22.clone();
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                agile_edit_failure_1(v23.clone(), v11.clone())
            }
            US0::US0_FileReadable(v25) => {
                let v25: Rc<str> = v25.clone();
                let v26: Rc<str> = std::fs::read_to_string(&*v25).unwrap_or_default().into();
                let v27: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v26);
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v11));
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
                let v32: i32 = v27.find(&*v31).map(|offset| offset as i32).unwrap_or(-1);
                let v33: bool = v32 < 0i32;
                if v33 {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                    agile_edit_failure_1(v34.clone(), v11.clone())
                } else {
                    let v36: i32 = (v27.clone().len() as i32);
                    let v37: i32 = v32.wrapping_add(6i32);
                    let v38: i32 = field_start_4(v27.clone(), v37, v36, v2);
                    let v39: US1 = US1::US1_ScanPlain;
                    let v40: i32 = field_end_5(v27.clone(), v38, v36, v39.clone());
                    let v41: Rc<str> = std::rc::Rc::<str>::from(v27.get(v38 as usize..v40 as usize).unwrap_or(""));
                    let v42: Rc<str> = agile_field_rendering::kit_agile_edit_field_token(&*v1, &*v41, &*v13);
                    let v43: bool = &*v42 == "";
                    let v46: US2 = if v43 {
                        US2::US2_TokenRejected
                    } else {
                        US2::US2_TokenRendered(v42.clone())
                    };
                    match &v46 {
                        US2::US2_TokenRejected => {
                            let v47: Rc<str> = agile_field_rendering::kit_agile_edit_field_rule(&*v1);
                            agile_edit_failure_1(v47.clone(), v11.clone())
                        }
                        US2::US2_TokenRendered(v49) => {
                            let v49: Rc<str> = v49.clone();
                            let v50: Rc<str> = std::rc::Rc::<str>::from(v27.get(0i32 as usize..v38 as usize).unwrap_or(""));
                            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
                            let v52: Rc<str> = std::rc::Rc::<str>::from(v27.get(v40 as usize..v36 as usize).unwrap_or(""));
                            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                            let v54: US3 = replace_records_unchanged_7(v17.clone(), v27.clone(), v53.clone());
                            match &v54 {
                                US3::US3_Replaced => {
                                    let v55: u64 = v27.len() as u64;
                                    let v56: u64 = v53.len() as u64;
                                    let v57: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v14);
                                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile edit ok id="); } LIT.with(|lit| lit.clone()) };
                                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v11));
                                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
                                    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v59, v60)), v1));
                                    let v63: Rc<str> = std::rc::Rc::<str>::from((v55).to_string());
                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v63));
                                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
                                    let v68: Rc<str> = std::rc::Rc::<str>::from((v56).to_string());
                                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
                                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                    let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
                                    let v72: Rc<str> = std::rc::Rc::<str>::from((v57).to_string());
                                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v62, Rc::<str>::from(format!("{}{}", v71, v72))));
                                    let v75: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v74.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                    v75
                                }
                                _ => {
                                    let v85: Rc<str> = match &v54 {
                                        US3::US3_ChangedSinceRead => {
                                            let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                            v80.clone()
                                        }
                                        US3::US3_ReadbackMismatch => {
                                            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                            v79.clone()
                                        }
                                        US3::US3_RenameFailed => {
                                            let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                            v77.clone()
                                        }
                                        US3::US3_RenameFailedTemporaryKept => {
                                            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                            v78.clone()
                                        }
                                        US3::US3_TemporaryWriteFailed => {
                                            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                            v76.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    agile_edit_failure_1(v85.clone(), v11.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_edit_run() -> i32 {
    closure0()()
}
