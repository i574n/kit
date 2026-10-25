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
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_ScanPlain,
    US2_ScanQuoted,
    US2_ScanEscaped,
}
#[derive(Clone)]
enum US3 {
    US3_StateWritten,
    US3_StateChangedSinceRead,
    US3_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Replaced,
    US4_TemporaryWriteFailed,
    US4_RenameFailed,
    US4_RenameFailedTemporaryKept,
    US4_ReadbackMismatch,
}
#[derive(Clone)]
enum US5 {
    US5_Some(i32),
    US5_None,
}
fn field_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v9: bool = match &v3 {
                US2::US2_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let mut v7: bool = v5 == 41i32;
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
                let mut v10: i32 = v1.wrapping_add(1i32);
                let mut v25: US2 = match &v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let mut v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let mut v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let mut v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US2 = US2::US2_ScanPlain;
            let mut v6: i32 = field_end_4(v0.clone(), v1, v2, v5.clone());
            let mut v7: i32 = v6.wrapping_add(1i32);
            let mut v8: i32 = skip_spaces_5(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let mut v10: bool = v9 == 92i32;
            if v10 {
                let mut v11: i32 = v1.wrapping_add(1i32);
                let mut v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let mut v24: Rc<str> = if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let mut v17: bool = v14 == 114i32;
                    if v17 {
                        let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let mut v19: bool = v14 == 116i32;
                        if v19 {
                            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let mut v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let mut v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let mut v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_7(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_2(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1 + 6i32;
    let mut v3: i32 = (v0.clone().len() as i32);
    let mut v4: i32 = 2i32;
    let mut v5: i32 = field_start_3(v0.clone(), v2, v3, v4);
    let mut v6: US2 = US2::US2_ScanPlain;
    let mut v7: i32 = field_end_4(v0.clone(), v5, v3, v6.clone());
    let mut v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_6(v8.clone());
    let mut v10: i32 = v1 + 6i32;
    let mut v11: i32 = 3i32;
    let mut v12: i32 = field_start_3(v0.clone(), v10, v3, v11);
    let mut v13: US2 = US2::US2_ScanPlain;
    let mut v14: i32 = field_end_4(v0.clone(), v12, v3, v13.clone());
    let mut v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let mut v16: Rc<str> = unquoted_6(v15.clone());
    let mut v17: i32 = v1 + 6i32;
    let mut v18: i32 = 4i32;
    let mut v19: i32 = field_start_3(v0.clone(), v17, v3, v18);
    let mut v20: US2 = US2::US2_ScanPlain;
    let mut v21: i32 = field_end_4(v0.clone(), v19, v3, v20.clone());
    let mut v22: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v21 as usize).unwrap_or(""));
    let mut v23: Rc<str> = unquoted_6(v22.clone());
    (v9.clone(), v16.clone(), v23.clone())
}
fn note_row_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let mut v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let mut v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let mut v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let mut v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let mut v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let mut v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let mut v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let mut v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let mut v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let mut v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let mut v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let mut v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let mut v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let mut v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let mut v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let mut v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let mut v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let mut v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let mut v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let mut v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let mut v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let mut v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let mut v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let mut v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let mut v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let mut v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let mut v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn state_written_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US3 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("EOIE_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let mut v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let mut v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let mut v23: US4 = if v11 {
            let mut v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let mut v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US4::US4_Replaced
                } else {
                    US4::US4_ReadbackMismatch
                }
            } else {
                let mut v17: bool = std::fs::remove_file(&*v10).is_ok() || !std::path::Path::new(&*v10).exists();
                if v17 {
                    US4::US4_RenameFailed
                } else {
                    US4::US4_RenameFailedTemporaryKept
                }
            }
        } else {
            US4::US4_TemporaryWriteFailed
        };
        match &v23 {
            US4::US4_Replaced => {
                US3::US3_StateWritten
            }
            _ => {
                let mut v32: Rc<str> = match &v23 {
                    US4::US4_ReadbackMismatch => {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US4::US4_RenameFailed => {
                        let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US4::US4_RenameFailedTemporaryKept => {
                        let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    US4::US4_TemporaryWriteFailed => {
                        let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    _ => unreachable!(),
                };
                US3::US3_StateWriteFailed(v32.clone())
            }
        }
    } else {
        US3::US3_StateChangedSinceRead
    }
}
fn note_edited_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US0) -> i32 {
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v8, &*v8].concat());
    let mut v10: bool = std::path::Path::new(&*v9).is_file();
    let mut v13: US1 = if v10 {
        US1::US1_FileReadable(v9.clone())
    } else {
        US1::US1_FileMissing(v9.clone())
    };
    match &v13 {
        US1::US1_FileMissing(v14) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v1));
            let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v15));
            let mut v21: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v20 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v21
        }
        US1::US1_FileReadable(v22) => {
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = std::fs::read_to_string(&*v22).unwrap_or_default().into();
            let mut v24: Rc<str> = eoie_agile_records::eoie_agile_records_as_lines(&*v23);
            let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let mut v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v28: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v26, &v27));
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(v28.to_ascii_lowercase());
            let mut v30: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let mut v31: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v29, &*v27, &*v30, &*v8].concat());
            let mut v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let mut v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v31));
            let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let mut v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
            let mut v36: i32 = v24.find(&*v35).map(|offset| offset as i32).unwrap_or(-1);
            let mut v37: bool = v36 < 0i32;
            if v37 {
                let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let mut v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let mut v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v1));
                let mut v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v38));
                let mut v44: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v43 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v44
            } else {
                let mut v45: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v26, &v27));
                let mut v46: Rc<str> = std::rc::Rc::<str>::from(v45.to_ascii_lowercase());
                let mut v47: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let mut v48: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v46, &*v27, &*v47, &*v8].concat());
                let mut v49: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v48));
                let mut v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v34));
                let mut v51: i32 = (v50.clone().len() as i32);
                let mut v52: i32 = v36.wrapping_add(v51);
                let (mut v53, mut v54, mut v55): (Rc<str>, Rc<str>, Rc<str>) = note_at_2(v24.clone(), v52);
                let mut v56: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let mut v58: bool = if v56 {
                    true
                } else {
                    let mut v57: bool = v54.clone() == v4.clone();
                    v57
                };
                if v58 {
                    let mut v63: Rc<str> = match &v5 {
                        US0::US0_AppendText => {
                            let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let mut v60: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v59));
                            let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v3));
                            v61.clone()
                        }
                        US0::US0_ReplaceText => {
                            v3.clone()
                        }
                    };
                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let mut v65: i32 = v24.get(v36 as usize..).and_then(|rest| rest.find(&*v64)).map(|found| (found + v36 as usize) as i32).unwrap_or(-1);
                    let mut v66: bool = v65 < 0i32;
                    let mut v68: i32 = if v66 {
                        let mut v67: i32 = (v24.clone().len() as i32);
                        v67
                    } else {
                        v65
                    };
                    let mut v69: Rc<str> = note_row_8(v1.clone(), v2, v53.clone(), v54.clone(), v63.clone());
                    let mut v70: Rc<str> = std::rc::Rc::<str>::from(v24.get(0i32 as usize..v36 as usize).unwrap_or(""));
                    let mut v71: i32 = (v24.clone().len() as i32);
                    let mut v72: Rc<str> = std::rc::Rc::<str>::from(v24.get(v68 as usize..v71 as usize).unwrap_or(""));
                    let mut v73: Rc<str> = std::rc::Rc::<str>::from([&*v70, &*v69, &*v72, &*v8, &*v8].concat());
                    let mut v74: US3 = state_written_9(v9.clone(), v23.clone(), v73.clone());
                    match &v74 {
                        US3::US3_StateWritten => {
                            let mut v75: u64 = v24.len() as u64;
                            let mut v76: u64 = v73.len() as u64;
                            let mut v77: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
                            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie agile note-edit ok id="); } LIT.with(|lit| lit.clone()) };
                            let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v1));
                            let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                            let mut v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                            let mut v82: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let mut v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                            let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                            let mut v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                            let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                            let mut v87: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v86));
                            let mut v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                            let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                            let mut v91: Rc<str> = std::rc::Rc::<str>::from(v54.replace(&*v89, &v90));
                            let mut v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                            let mut v93: Rc<str> = std::rc::Rc::<str>::from(v91.replace(&*v88, &v92));
                            let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                            let mut v95: Rc<str> = std::rc::Rc::<str>::from(v93.replace(&*v64, &v94));
                            let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                            let mut v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                            let mut v98: Rc<str> = std::rc::Rc::<str>::from(v95.replace(&*v96, &v97));
                            let mut v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                            let mut v101: Rc<str> = std::rc::Rc::<str>::from(v98.replace(&*v99, &v100));
                            let mut v102: Rc<str> = std::rc::Rc::<str>::from([&*v88, &*v101, &*v88, &*v8, &*v8].concat());
                            let mut v103: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v102));
                            let mut v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                            let mut v105: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v104));
                            let mut v106: i32 = (v55.clone().len() as i32);
                            let mut v107: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                            let mut v108: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v107));
                            let mut v109: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v108));
                            let mut v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
                            let mut v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
                            let mut v112: i32 = (v63.clone().len() as i32);
                            let mut v113: Rc<str> = std::rc::Rc::<str>::from((v112).to_string());
                            let mut v114: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v113));
                            let mut v115: Rc<str> = std::rc::Rc::<str>::from((v75).to_string());
                            let mut v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                            let mut v117: Rc<str> = Rc::<str>::from(format!("{}{}", v116, v115));
                            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                            let mut v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
                            let mut v120: Rc<str> = std::rc::Rc::<str>::from((v76).to_string());
                            let mut v121: Rc<str> = Rc::<str>::from(format!("{}{}", v119, v120));
                            let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                            let mut v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
                            let mut v124: Rc<str> = std::rc::Rc::<str>::from((v77).to_string());
                            let mut v125: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v124));
                            let mut v126: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v125));
                            let mut v127: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v126.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                            v127
                        }
                        _ => {
                            let mut v131: Rc<str> = match &v74 {
                                US3::US3_StateChangedSinceRead => {
                                    let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v128.clone()
                                }
                                US3::US3_StateWriteFailed(v129) => {
                                    let mut v129: Rc<str> = v129.clone();
                                    v129.clone()
                                }
                                _ => unreachable!(),
                            };
                            let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                            let mut v133: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v1));
                            let mut v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                            let mut v135: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v134));
                            let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v131));
                            let mut v137: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v136 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                            v137
                        }
                    }
                } else {
                    let mut v139: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                    let mut v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) };
                    let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v139));
                    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written by "); } LIT.with(|lit| lit.clone()) };
                    let mut v143: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v142));
                    let mut v144: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v54));
                    let mut v145: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; only its author edits it"); } LIT.with(|lit| lit.clone()) };
                    let mut v146: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v145));
                    let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                    let mut v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v1));
                    let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let mut v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                    let mut v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v146));
                    let mut v152: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v151 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v152
                }
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method12(mut v0: Option<i32>) -> Option<i32> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((i32)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US5> = Rc::new(move |mut v0: (i32)| -> US5 {
        let mut v1: i32 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn options_from_13(mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: bool) -> (Rc<str>, US0, bool) {
    let mut v4: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v5: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v45, mut v46, mut v47): (Rc<str>, US0, bool) = if v5 {
        (v1.clone(), v2.clone(), v3)
    } else {
        let mut v6: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--append"); } LIT.with(|lit| lit.clone()) };
        let (mut v42, mut v43, mut v44): (Rc<str>, US0, bool) = if v6 {
            let mut v7: i32 = v0.wrapping_add(1i32);
            let mut v8: US0 = US0::US0_AppendText;
            let (mut v9, mut v10, mut v11): (Rc<str>, US0, bool) = options_from_13(v7, v1.clone(), v8.clone(), v3);
            (v9.clone(), v10.clone(), v11)
        } else {
            let mut v12: bool = v4.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as"); } LIT.with(|lit| lit.clone()) };
            let mut v24: bool = if v12 {
                let mut v13: i32 = v0.wrapping_add(1i32);
                let mut v14: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v15: i32 = (v14.clone().len() as i32);
                let mut v16: i32 = 0i32;
                let mut v17: i32 = method10(v14.clone(), v15, v16);
                let mut v18: i32 = v15.wrapping_sub(1i32);
                let mut v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
                let mut v20: i32 = (v19.clone().len() as i32);
                let mut v21: i32 = method11(v19.clone(), v20);
                let mut v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
                let mut v23: bool = v22.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v23
            } else {
                false
            };
            let (mut v39, mut v40, mut v41): (Rc<str>, US0, bool) = if v24 {
                let mut v25: i32 = v0.wrapping_add(2i32);
                let mut v26: i32 = v0.wrapping_add(1i32);
                let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let mut v28: i32 = (v27.clone().len() as i32);
                let mut v29: i32 = 0i32;
                let mut v30: i32 = method10(v27.clone(), v28, v29);
                let mut v31: i32 = v28.wrapping_sub(1i32);
                let mut v32: Rc<str> = string_slice(&v27.clone(), v30 as i64, v31 as i64);
                let mut v33: i32 = (v32.clone().len() as i32);
                let mut v34: i32 = method11(v32.clone(), v33);
                let mut v35: Rc<str> = string_slice(&v32.clone(), 0i32 as i64, v34 as i64);
                let (mut v36, mut v37, mut v38): (Rc<str>, US0, bool) = options_from_13(v25, v35.clone(), v2.clone(), v3);
                (v36.clone(), v37.clone(), v38)
            } else {
                (v1.clone(), v2.clone(), false)
            };
            (v39.clone(), v40.clone(), v41)
        };
        (v42.clone(), v43.clone(), v44)
    };
    (v45.clone(), v46.clone(), v47)
}
fn note_edit_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 3i32;
    let mut v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: i32 = (v3.clone().len() as i32);
    let mut v5: i32 = 0i32;
    let mut v6: i32 = method10(v3.clone(), v4, v5);
    let mut v7: i32 = v4.wrapping_sub(1i32);
    let mut v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let mut v10: i32 = method11(v8.clone(), v9);
    let mut v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let mut v12: Option<i32> = (v11).trim().parse::<i32>().ok();
    let mut v13: Option<i32> = method12(v12.clone());
    let mut v14: Rc<dyn Fn((i32)) -> US5> = closure1();
    let mut v15: Option<US5> = v13.map(|x| v14(x));
    let mut v17: US5 = US5::US5_None;
    let mut v18: US5 = v17.clone();
    let mut v19: US5 = v15.unwrap_or(v18);
    let mut v22: i32 = match &v19 {
        US5::US5_None => {
            -1i32
        }
        US5::US5_Some(v20) => {
            let mut v20: i32 = *v20;
            v20
        }
    };
    let mut v23: i32 = 4i32;
    let mut v24: Rc<str> = usize::try_from(v23).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v25: i32 = (v24.clone().len() as i32);
    let mut v26: i32 = 0i32;
    let mut v27: i32 = method10(v24.clone(), v25, v26);
    let mut v28: i32 = v25.wrapping_sub(1i32);
    let mut v29: Rc<str> = string_slice(&v24.clone(), v27 as i64, v28 as i64);
    let mut v30: i32 = (v29.clone().len() as i32);
    let mut v31: i32 = method11(v29.clone(), v30);
    let mut v32: Rc<str> = string_slice(&v29.clone(), 0i32 as i64, v31 as i64);
    let mut v33: bool = v22 < 1i32;
    if v33 {
        let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let mut v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
        let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v1));
        let mut v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
        let mut v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v34));
        let mut v40: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v39 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v40
    } else {
        let mut v41: bool = v32.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v41 {
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v1));
            let mut v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let mut v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
            let mut v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v42));
            let mut v48: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v47 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v48
        } else {
            let mut v49: i32 = 5i32;
            let mut v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v51: US0 = US0::US0_ReplaceText;
            let mut v52: bool = true;
            let (mut v53, mut v54, mut v55): (Rc<str>, US0, bool) = options_from_13(v49, v50.clone(), v51.clone(), v52);
            if v55 {
                let mut v56: i32 = 1i32;
                let mut v57: Rc<str> = usize::try_from(v56).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_edited_1(v57.clone(), v1.clone(), v22, v32.clone(), v53.clone(), v54.clone())
            } else {
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>] [--append]"); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let mut v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v1));
                let mut v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
                let mut v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v59));
                let mut v65: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("eoie error: ".to_string() + &*v64 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v65
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_agile_note_edit_run() -> i32 {
    closure0()()
}
