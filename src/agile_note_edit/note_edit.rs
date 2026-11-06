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
    US0_ReplaceText,
    US0_AppendText,
}
#[derive(Clone)]
enum US1 {
    US1_NoGuard,
    US1_ExpectedAt(Rc<str>),
    US1_ExpectedPrefix(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_ScanPlain,
    US3_ScanQuoted,
    US3_ScanEscaped,
}
#[derive(Clone)]
enum US4 {
    US4_StateWritten,
    US4_StateChangedSinceRead,
    US4_StateWriteFailed(Rc<str>),
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
#[derive(Clone)]
enum US6 {
    US6_Some(i32),
    US6_None,
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US3) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US3::US3_ScanPlain => {
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
                let v25: US3 = match v3 {
                    US3::US3_ScanEscaped => {
                        US3::US3_ScanQuoted
                    }
                    US3::US3_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US3::US3_ScanQuoted
                        } else {
                            US3::US3_ScanPlain
                        }
                    }
                    US3::US3_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US3::US3_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US3::US3_ScanPlain
                            } else {
                                US3::US3_ScanQuoted
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
            let v5: US3 = US3::US3_ScanPlain;
            let v6: i32 = field_end_5(v0.clone(), v1, v2, v5);
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_2(v4, v6, v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
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
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = concat5_2(v4, v13, v24, v25.clone(), v25.clone());
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
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3, v4, v5, v6, v7)
}
fn note_at_3(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let v5: i32 = field_start_4(v0.clone(), v2, v3, v4);
    let v6: US3 = US3::US3_ScanPlain;
    let v7: i32 = field_end_5(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_7(v8);
    let v10: i32 = 3i32;
    let mut v11: i32 = field_start_4(v0.clone(), v2, v3, v10);
    let v12: US3 = US3::US3_ScanPlain;
    let v13: i32 = field_end_5(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_7(v14);
    let v16: i32 = 4i32;
    let v17: i32 = field_start_4(v0.clone(), v2, v3, v16);
    let v18: US3 = US3::US3_ScanPlain;
    let v19: i32 = field_end_5(v0.clone(), v17, v3, v18);
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let mut v21: Rc<str> = unquoted_7(v20);
    (v9, v15, v21)
}
fn note_row_9(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_2(v6, v10, v8, v11, v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let v30: Rc<str> = concat5_2(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_2(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let mut v40: Rc<str> = concat5_2(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_2(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_2(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_2(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v32, v47, v31, v53, v54)
}
fn renamed_within_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US4 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_2(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v8: bool = std::fs::write(&*v7, &*v2).is_ok();
        let mut v21: US5 = if v8 {
            let mut v9: i32 = 40i32;
            let v10: bool = renamed_within_11(v0.clone(), v7.clone(), v9);
            if v10 {
                let mut v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US5::US5_Replaced
                } else {
                    US5::US5_ReadbackMismatch
                }
            } else {
                let v15: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
                if v15 {
                    US5::US5_RenameFailed
                } else {
                    US5::US5_RenameFailedTemporaryKept
                }
            }
        } else {
            US5::US5_TemporaryWriteFailed
        };
        match &v21 {
            US5::US5_Replaced => {
                US4::US4_StateWritten
            }
            _ => {
                let v32: Rc<str> = match v21 {
                    US5::US5_ChangedSinceRead => {
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    US5::US5_ReadbackMismatch => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26
                    }
                    US5::US5_RenameFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    US5::US5_RenameFailedTemporaryKept => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US5::US5_TemporaryWriteFailed => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v23
                    }
                    _ => unreachable!(),
                };
                US4::US4_StateWriteFailed(v32)
            }
        }
    } else {
        US4::US4_StateChangedSinceRead
    }
}
fn note_edited_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US0, mut v6: US1) -> i32 {
    let v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_2(v0, v8, v9.clone(), v9.clone(), v9.clone());
    let mut v11: bool = std::path::Path::new(&*v10).is_file();
    let v14: US2 = if v11 {
        US2::US2_FileReadable(v10.clone())
    } else {
        US2::US2_FileMissing(v10.clone())
    };
    match v14 {
        US2::US2_FileMissing(v15) => {
            let v15: Rc<str> = v15;
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v1));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v16));
            let mut v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v20
        }
        US2::US2_FileReadable(v21) => {
            let mut v21: Rc<str> = v21;
            let v22: Rc<str> = std::fs::read_to_string(&*v21).unwrap_or_default().into();
            let v23: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v22);
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = Rc::<str>::from(v1.replace(&*v25, &*v26));
            let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
            let v29: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v30: Rc<str> = concat5_2(v24.clone(), v28, v26.clone(), v29, v9.clone());
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v30));
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
            let v33: i32 = v23.find(&*v32).map(|offset| offset as i32).unwrap_or(-1);
            let v34: bool = v33 < 0i32;
            if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v1));
                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v35));
                let v39: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v38 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v39
            } else {
                let mut v40: Rc<str> = Rc::<str>::from(v1.replace(&*v25, &*v26));
                let v41: Rc<str> = std::rc::Rc::<str>::from(v40.to_ascii_lowercase());
                let v42: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let v43: Rc<str> = concat5_2(v24, v41, v26, v42, v9.clone());
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) }, v43));
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) }));
                let v46: i32 = (v45.len() as i32);
                let v47: i32 = v33.wrapping_add(v46);
                let (mut v48, mut v49, mut v50): (Rc<str>, Rc<str>, Rc<str>) = note_at_3(v23.clone(), v47);
                let v68: Rc<str> = match v6 {
                    US1::US1_ExpectedAt(v51) => {
                        let v51: Rc<str> = v51;
                        let v52: bool = &*v48 == &*v51;
                        if v52 {
                            v9.clone()
                        } else {
                            let v53: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) }, v53));
                            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written at "); } LIT.with(|lit| lit.clone()) }));
                            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v48.clone()));
                            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", not "); } LIT.with(|lit| lit.clone()) }));
                            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v51));
                            v58
                        }
                    }
                    US1::US1_ExpectedPrefix(v60) => {
                        let v60: Rc<str> = v60;
                        let v61: bool = v50.starts_with(&*v60);
                        if v61 {
                            v9.clone()
                        } else {
                            let v62: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) }, v62));
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" does not start with the expected text"); } LIT.with(|lit| lit.clone()) }));
                            v64
                        }
                    }
                    US1::US1_NoGuard => {
                        v9.clone()
                    }
                };
                let v69: bool = &*v68 != "";
                if v69 {
                    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v1));
                    let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v68));
                    let v73: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v72 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v73
                } else {
                    let v74: bool = &*v4 == "";
                    let v76: bool = if v74 {
                        true
                    } else {
                        let v75: bool = &*v49 == &*v4;
                        v75
                    };
                    if v76 {
                        let v80: Rc<str> = match v5 {
                            US0::US0_AppendText => {
                                let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v50.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
                                let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v3));
                                v78
                            }
                            US0::US0_ReplaceText => {
                                v3
                            }
                        };
                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v82: i32 = v23.get(v33 as usize..).and_then(|rest| rest.find(&*v81)).map(|found| (found + v33 as usize) as i32).unwrap_or(-1);
                        let v83: bool = v82 < 0i32;
                        let v85: i32 = if v83 {
                            let v84: i32 = (v23.clone().len() as i32);
                            v84
                        } else {
                            v82
                        };
                        let v86: Rc<str> = note_row_9(v1.clone(), v2, v48.clone(), v49.clone(), v80.clone());
                        let v87: Rc<str> = std::rc::Rc::<str>::from(v23.get(0i32 as usize..v33 as usize).unwrap_or(""));
                        let v88: i32 = (v23.clone().len() as i32);
                        let v89: Rc<str> = std::rc::Rc::<str>::from(v23.get(v85 as usize..v88 as usize).unwrap_or(""));
                        let v90: Rc<str> = concat5_2(v87, v86, v89, v9.clone(), v9.clone());
                        let v91: US4 = state_written_10(v10, v22, v90.clone());
                        match &v91 {
                            US4::US4_StateWritten => {
                                let v92: u64 = v23.len() as u64;
                                let v93: u64 = v90.len() as u64;
                                let v94: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v7);
                                let v95: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile note-edit ok id="); } LIT.with(|lit| lit.clone()) }, v1));
                                let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) }));
                                let v97: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) }));
                                let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) }));
                                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let v104: Rc<str> = Rc::<str>::from(v49.replace(&*v102, &*v103));
                                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let v106: Rc<str> = Rc::<str>::from(v104.replace(&*v101, &*v105));
                                let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let v108: Rc<str> = Rc::<str>::from(v106.replace(&*v81, &*v107));
                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let v111: Rc<str> = Rc::<str>::from(v108.replace(&*v109, &*v110));
                                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let v114: Rc<str> = Rc::<str>::from(v111.replace(&*v112, &*v113));
                                let v115: Rc<str> = concat5_2(v101.clone(), v114, v101.clone(), v9.clone(), v9.clone());
                                let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v115));
                                let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) }));
                                let v118: i32 = (v50.clone().len() as i32);
                                let v119: Rc<str> = std::rc::Rc::<str>::from((v118).to_string());
                                let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v119));
                                let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v120));
                                let v122: Rc<str> = Rc::<str>::from(format!("{}{}", v121, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) }));
                                let v123: i32 = (v80.len() as i32);
                                let v124: Rc<str> = std::rc::Rc::<str>::from((v123).to_string());
                                let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v122, v124));
                                let v126: Rc<str> = Rc::<str>::from(format!("{}{}", v125, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replaced="); } LIT.with(|lit| lit.clone()) }));
                                let v127: Rc<str> = { let text = &*v50; if text.chars().count() <= 80i32 as usize { std::rc::Rc::<str>::from(text) } else { std::rc::Rc::<str>::from(text.chars().take((80i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
                                let v128: Rc<str> = Rc::<str>::from(v127.replace(&*v102, &*v103));
                                let v129: Rc<str> = Rc::<str>::from(v128.replace(&*v101, &*v105));
                                let v130: Rc<str> = Rc::<str>::from(v129.replace(&*v81, &*v107));
                                let v131: Rc<str> = Rc::<str>::from(v130.replace(&*v109, &*v110));
                                let v132: Rc<str> = Rc::<str>::from(v131.replace(&*v112, &*v113));
                                let v133: Rc<str> = concat5_2(v101.clone(), v132, v101.clone(), v9.clone(), v9.clone());
                                let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v133));
                                let v135: Rc<str> = std::rc::Rc::<str>::from((v92).to_string());
                                let v136: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v135));
                                let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v136, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
                                let v138: Rc<str> = std::rc::Rc::<str>::from((v93).to_string());
                                let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v138));
                                let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v139, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                                let v141: Rc<str> = std::rc::Rc::<str>::from((v94).to_string());
                                let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
                                let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v142));
                                let v144: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v143.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v144
                            }
                            _ => {
                                let v148: Rc<str> = match v91 {
                                    US4::US4_StateChangedSinceRead => {
                                        let v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                        v145
                                    }
                                    US4::US4_StateWriteFailed(v146) => {
                                        let v146: Rc<str> = v146;
                                        v146
                                    }
                                    _ => unreachable!(),
                                };
                                let v149: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v1));
                                let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                                let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v148));
                                let v152: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v151 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v152
                            }
                        }
                    } else {
                        let v154: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                        let v155: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) }, v154));
                        let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v155, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written by "); } LIT.with(|lit| lit.clone()) }));
                        let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v156, v49));
                        let v158: Rc<str> = Rc::<str>::from(format!("{}{}", v157, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; only its author edits it"); } LIT.with(|lit| lit.clone()) }));
                        let v159: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v1));
                        let v160: Rc<str> = Rc::<str>::from(format!("{}{}", v159, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                        let v161: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v158));
                        let v162: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v161 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v162
                    }
                }
            }
        }
    }
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method14(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US6> = Rc::new(move |mut v0: (i32)| -> US6 {
        let mut v1: i32 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_text_of_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method12(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method13(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: bool = &*v8 == "-";
    let mut v21: Rc<str> = if v9 {
        let v10: std::io::Stdin = std::io::stdin();
        let mut v11: Result<std::string::String, std::io::Error> = std::io::read_to_string(v10);
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: &str = &*v12;
        let v16: std::string::String = String::from(v14);
        let v18: std::string::String = v11.unwrap_or(v16);
        let mut v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
        v20
    } else {
        v0
    };
    let v22: i32 = (v21.clone().len() as i32);
    let v23: i32 = 0i32;
    let v24: i32 = method12(v21.clone(), v22, v23);
    let v25: i32 = v22.wrapping_sub(1i32);
    let v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
    let mut v27: i32 = (v26.clone().len() as i32);
    let v28: i32 = method13(v26.clone(), v27);
    let v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
    v29
}
fn options_of_16(mut v0: &Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: bool) -> (Rc<str>, US0, US1, bool) {
    let mut v9: bool = &*v6 != "";
    let v15: US1 = if v9 {
        US1::US1_ExpectedAt(v6)
    } else {
        let mut v11: bool = &*v7 != "";
        if v11 {
            US1::US1_ExpectedPrefix(v7)
        } else {
            US1::US1_NoGuard
        }
    };
    let v18: US0 = if v5 {
        US0::US0_AppendText
    } else {
        US0::US0_ReplaceText
    };
    (v4, v18, v15, v8)
}
fn note_edit_entry_0() -> i32 {
    let v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool)>>> = agile_note_edit_args::agile_note_edit_args_read(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>, bool) = v0.borrow()[0i32 as usize].clone();
    let v10: i32 = (v3.clone().len() as i32);
    let mut v11: i32 = 0i32;
    let v12: i32 = method12(v3.clone(), v10, v11);
    let v13: i32 = v10.wrapping_sub(1i32);
    let v14: Rc<str> = string_slice(&v3.clone(), v12 as i64, v13 as i64);
    let v15: i32 = (v14.clone().len() as i32);
    let v16: i32 = method13(v14.clone(), v15);
    let v17: Rc<str> = string_slice(&v14, 0i32 as i64, v16 as i64);
    let v18: Option<i32> = (v17).trim().parse::<i32>().ok();
    let v19: Option<i32> = method14(v18);
    let mut v20: Rc<dyn Fn((i32)) -> US6> = closure1();
    let mut v21: Option<US6> = v19.map(|x| v20(x));
    let v23: US6 = US6::US6_None;
    let v24: US6 = v23.clone();
    let v25: US6 = v21.unwrap_or(v24);
    let v28: i32 = match v25 {
        US6::US6_None => {
            -1i32
        }
        US6::US6_Some(v26) => {
            let v26: i32 = v26;
            v26
        }
    };
    let v29: Rc<str> = note_text_of_15(v4.clone());
    let v30: bool = v28 < 1i32;
    if v30 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v2));
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v31));
        let v35: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v34 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v35
    } else {
        let v36: bool = &*v29 == "";
        if v36 {
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v2));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
            let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v37));
            let v41: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v40 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v41
        } else {
            let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, US0, US1, bool) = options_of_16(&v1, &v2, &v3, &v4, v5, v6, v7, v8, v9);
            if v45 {
                note_edited_1(v1, v2, v28, v29, v42, v43, v44)
            } else {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) }, v2));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v47));
                let v51: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v50 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v51
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_note_edit_run() -> i32 {
    closure0()()
}
