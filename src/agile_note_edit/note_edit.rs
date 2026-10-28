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
}
#[derive(Clone)]
enum US6 {
    US6_Some(i32),
    US6_None,
}
fn field_end_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US3) -> i32 {
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
                let v25: US3 = match &v3 {
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
                            let mut v14: bool = v5 == 34i32;
                            if v14 {
                                US3::US3_ScanPlain
                            } else {
                                US3::US3_ScanQuoted
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
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US3 = US3::US3_ScanPlain;
            let v6: i32 = field_end_4(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_5(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let mut v14: i32 = v0.as_bytes().get(v11 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let mut v15: bool = v14 == 110i32;
                let v24: Rc<str> = if v15 {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    v16.clone()
                } else {
                    let v17: bool = v14 == 114i32;
                    if v17 {
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        v18.clone()
                    } else {
                        let v19: bool = v14 == 116i32;
                        if v19 {
                            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                            v20.clone()
                        } else {
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v12 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v13, &*v24, &*v25, &*v25].concat());
                (v0, v1, v2, v3, v4) = (v0.clone(), v12, v12, v3, v26.clone());
                continue;
            } else {
                let v28: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3, v4) = (v0.clone(), v28, v2, v3, v4.clone());
                continue;
            }
        }
    }
}
fn unquoted_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_7(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_2(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let v5: i32 = field_start_3(v0.clone(), v2, v3, v4);
    let v6: US3 = US3::US3_ScanPlain;
    let v7: i32 = field_end_4(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_6(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_3(v0.clone(), v2, v3, v10);
    let v12: US3 = US3::US3_ScanPlain;
    let v13: i32 = field_end_4(v0.clone(), v11, v3, v12);
    let mut v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let mut v15: Rc<str> = unquoted_6(v14);
    let v16: i32 = 4i32;
    let v17: i32 = field_start_3(v0.clone(), v2, v3, v16);
    let v18: US3 = US3::US3_ScanPlain;
    let v19: i32 = field_end_4(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_6(v20);
    (v9, v15, v21)
}
fn note_row_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let mut v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let mut v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn state_written_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US4 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v7));
        let mut v9: bool = std::fs::write(&*v8, &*v2).is_ok();
        let v21: US5 = if v9 {
            let v10: bool = std::fs::rename(&*v8, &*v0).is_ok();
            if v10 {
                let v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US5::US5_Replaced
                } else {
                    US5::US5_ReadbackMismatch
                }
            } else {
                let mut v15: bool = std::fs::remove_file(&*v8).is_ok() || !std::path::Path::new(&*v8).exists();
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
                let v30: Rc<str> = match &v21 {
                    US5::US5_ReadbackMismatch => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US5::US5_RenameFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US5::US5_RenameFailedTemporaryKept => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    US5::US5_TemporaryWriteFailed => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    _ => unreachable!(),
                };
                US4::US4_StateWriteFailed(v30.clone())
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
    let v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v8, &*v9, &*v9, &*v9].concat());
    let v11: bool = std::path::Path::new(&*v10).is_file();
    let mut v14: US2 = if v11 {
        US2::US2_FileReadable(v10.clone())
    } else {
        US2::US2_FileMissing(v10.clone())
    };
    match &v14 {
        US2::US2_FileMissing(v15) => {
            let mut v15: Rc<str> = v15.clone();
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v1));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v16));
            let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v22
        }
        US2::US2_FileReadable(v23) => {
            let v23: Rc<str> = v23.clone();
            let v24: Rc<str> = std::fs::read_to_string(&*v23).unwrap_or_default().into();
            let v25: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v24);
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
            let mut v29: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v27, &v28));
            let v30: Rc<str> = std::rc::Rc::<str>::from(v29.to_ascii_lowercase());
            let v31: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
            let v32: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v30, &*v28, &*v31, &*v9].concat());
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let v37: i32 = v25.find(&*v36).map(|offset| offset as i32).unwrap_or(-1);
            let v38: bool = v37 < 0i32;
            if v38 {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no note with this number"); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v41, v42)), v39));
                let v45: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v44 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v45
            } else {
                let mut v46: Rc<str> = std::rc::Rc::<str>::from(v1.replace(&*v27, &v28));
                let v47: Rc<str> = std::rc::Rc::<str>::from(v46.to_ascii_lowercase());
                let v48: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                let v49: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v47, &*v28, &*v48, &*v9].concat());
                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v49)), v35));
                let v52: i32 = (v51.clone().len() as i32);
                let v53: i32 = v37.wrapping_add(v52);
                let (mut v54, mut v55, mut v56): (Rc<str>, Rc<str>, Rc<str>) = note_at_2(v25.clone(), v53);
                let v79: Rc<str> = match &v6 {
                    US1::US1_ExpectedAt(v57) => {
                        let v57: Rc<str> = v57.clone();
                        let v58: bool = &*v54 == &*v57;
                        if v58 {
                            v9.clone()
                        } else {
                            let v59: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) };
                            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v59));
                            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written at "); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v61, v62)), v54));
                            let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", not "); } LIT.with(|lit| lit.clone()) };
                            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v64, v65)), v57));
                            v67.clone()
                        }
                    }
                    US1::US1_ExpectedPrefix(v69) => {
                        let v69: Rc<str> = v69.clone();
                        let v70: bool = v56.starts_with(&*v69);
                        if v70 {
                            v9.clone()
                        } else {
                            let v71: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) };
                            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v71));
                            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" does not start with the expected text"); } LIT.with(|lit| lit.clone()) };
                            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                            v75.clone()
                        }
                    }
                    US1::US1_NoGuard => {
                        v9.clone()
                    }
                };
                let v80: bool = &*v79 != "";
                if v80 {
                    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v1));
                    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v82, v83)), v79));
                    let v86: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v85 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v86
                } else {
                    let v87: bool = &*v4 == "";
                    let v89: bool = if v87 {
                        true
                    } else {
                        let v88: bool = &*v55 == &*v4;
                        v88
                    };
                    if v89 {
                        let v94: Rc<str> = match &v5 {
                            US0::US0_AppendText => {
                                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v56, v90)), v3));
                                v92.clone()
                            }
                            US0::US0_ReplaceText => {
                                v3.clone()
                            }
                        };
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v96: i32 = v25.get(v37 as usize..).and_then(|rest| rest.find(&*v95)).map(|found| (found + v37 as usize) as i32).unwrap_or(-1);
                        let v97: bool = v96 < 0i32;
                        let v99: i32 = if v97 {
                            let v98: i32 = (v25.clone().len() as i32);
                            v98
                        } else {
                            v96
                        };
                        let v100: Rc<str> = note_row_8(v1.clone(), v2, v54.clone(), v55.clone(), v94.clone());
                        let v101: Rc<str> = std::rc::Rc::<str>::from(v25.get(0i32 as usize..v37 as usize).unwrap_or(""));
                        let v102: i32 = (v25.clone().len() as i32);
                        let v103: Rc<str> = std::rc::Rc::<str>::from(v25.get(v99 as usize..v102 as usize).unwrap_or(""));
                        let v104: Rc<str> = std::rc::Rc::<str>::from([&*v101, &*v100, &*v103, &*v9, &*v9].concat());
                        let v105: US4 = state_written_9(v10.clone(), v24.clone(), v104.clone());
                        match &v105 {
                            US4::US4_StateWritten => {
                                let v106: u64 = v25.len() as u64;
                                let v107: u64 = v104.len() as u64;
                                let v108: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v7);
                                let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile note-edit ok id="); } LIT.with(|lit| lit.clone()) };
                                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v1));
                                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                                let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v111));
                                let v113: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                                let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
                                let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                                let v116: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v115));
                                let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                                let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v117));
                                let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                let v122: Rc<str> = std::rc::Rc::<str>::from(v55.replace(&*v120, &v121));
                                let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                let v124: Rc<str> = std::rc::Rc::<str>::from(v122.replace(&*v119, &v123));
                                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                let v126: Rc<str> = std::rc::Rc::<str>::from(v124.replace(&*v95, &v125));
                                let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                let v129: Rc<str> = std::rc::Rc::<str>::from(v126.replace(&*v127, &v128));
                                let v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                let v132: Rc<str> = std::rc::Rc::<str>::from(v129.replace(&*v130, &v131));
                                let v133: Rc<str> = std::rc::Rc::<str>::from([&*v119, &*v132, &*v119, &*v9, &*v9].concat());
                                let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v118, v133));
                                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                                let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v135));
                                let v137: i32 = (v56.clone().len() as i32);
                                let v138: Rc<str> = std::rc::Rc::<str>::from((v137).to_string());
                                let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v116, Rc::<str>::from(format!("{}{}", v136, v138))));
                                let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("->"); } LIT.with(|lit| lit.clone()) };
                                let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v141));
                                let v143: i32 = (v94.clone().len() as i32);
                                let v144: Rc<str> = std::rc::Rc::<str>::from((v143).to_string());
                                let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v144));
                                let v146: Rc<str> = std::rc::Rc::<str>::from((v106).to_string());
                                let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v146));
                                let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                                let v151: Rc<str> = std::rc::Rc::<str>::from((v107).to_string());
                                let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v151));
                                let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
                                let v155: Rc<str> = std::rc::Rc::<str>::from((v108).to_string());
                                let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v145, Rc::<str>::from(format!("{}{}", v154, v155))));
                                let v158: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v157.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                v158
                            }
                            _ => {
                                let v162: Rc<str> = match &v105 {
                                    US4::US4_StateChangedSinceRead => {
                                        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                        v159.clone()
                                    }
                                    US4::US4_StateWriteFailed(v160) => {
                                        let v160: Rc<str> = v160.clone();
                                        v160.clone()
                                    }
                                    _ => unreachable!(),
                                };
                                let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                                let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v163, v1));
                                let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v167: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v164, v165)), v162));
                                let v168: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v167 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v168
                            }
                        }
                    } else {
                        let v170: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
                        let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note "); } LIT.with(|lit| lit.clone()) };
                        let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v170));
                        let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" was written by "); } LIT.with(|lit| lit.clone()) };
                        let v175: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v172, v173)), v55));
                        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; only its author edits it"); } LIT.with(|lit| lit.clone()) };
                        let v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
                        let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                        let v179: Rc<str> = Rc::<str>::from(format!("{}{}", v178, v1));
                        let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v182: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v179, v180)), v177));
                        let v183: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v182 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v183
                    }
                }
            }
        }
    }
}
fn method10(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
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
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method12(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure1() -> Rc<dyn Fn((i32)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US6> = Rc::new(move |mut v0: (i32)| -> US6 {
        let v1: i32 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn options_from_13(mut v0: i32, mut v1: Rc<str>, mut v2: US0, mut v3: US1, mut v4: bool) -> (Rc<str>, US0, US1, bool) {
    let v5: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v6: bool = &*v5 == "";
    let (mut v100, mut v101, mut v102, mut v103): (Rc<str>, US0, US1, bool) = if v6 {
        (v1.clone(), v2.clone(), v3.clone(), v4)
    } else {
        let v7: bool = &*v5 == "--append";
        let (mut v96, mut v97, mut v98, mut v99): (Rc<str>, US0, US1, bool) = if v7 {
            let v8: i32 = v0.wrapping_add(1i32);
            let mut v9: US0 = US0::US0_AppendText;
            let (mut v10, mut v11, mut v12, mut v13): (Rc<str>, US0, US1, bool) = options_from_13(v8, v1.clone(), v9.clone(), v3.clone(), v4);
            (v10.clone(), v11.clone(), v12.clone(), v13)
        } else {
            let mut v14: bool = &*v5 == "--as";
            let v26: bool = if v14 {
                let mut v15: i32 = v0.wrapping_add(1i32);
                let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v17: i32 = (v16.clone().len() as i32);
                let v18: i32 = 0i32;
                let v19: i32 = method10(v16.clone(), v17, v18);
                let v20: i32 = v17.wrapping_sub(1i32);
                let v21: Rc<str> = string_slice(&v16.clone(), v19 as i64, v20 as i64);
                let v22: i32 = (v21.clone().len() as i32);
                let v23: i32 = method11(v21.clone(), v22);
                let v24: Rc<str> = string_slice(&v21.clone(), 0i32 as i64, v23 as i64);
                let v25: bool = &*v24 != "";
                v25
            } else {
                false
            };
            let (mut v92, mut v93, mut v94, mut v95): (Rc<str>, US0, US1, bool) = if v26 {
                let v27: i32 = v0.wrapping_add(2i32);
                let v28: i32 = v0.wrapping_add(1i32);
                let mut v29: Rc<str> = usize::try_from(v28).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v30: i32 = (v29.clone().len() as i32);
                let v31: i32 = 0i32;
                let v32: i32 = method10(v29.clone(), v30, v31);
                let v33: i32 = v30.wrapping_sub(1i32);
                let v34: Rc<str> = string_slice(&v29.clone(), v32 as i64, v33 as i64);
                let v35: i32 = (v34.clone().len() as i32);
                let v36: i32 = method11(v34.clone(), v35);
                let v37: Rc<str> = string_slice(&v34.clone(), 0i32 as i64, v36 as i64);
                let (mut v38, mut v39, mut v40, mut v41): (Rc<str>, US0, US1, bool) = options_from_13(v27, v37.clone(), v2.clone(), v3.clone(), v4);
                (v38.clone(), v39.clone(), v40.clone(), v41)
            } else {
                let v42: bool = &*v5 == "--expect-at";
                let v54: bool = if v42 {
                    let v43: i32 = v0.wrapping_add(1i32);
                    let v44: Rc<str> = usize::try_from(v43).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v45: i32 = (v44.clone().len() as i32);
                    let mut v46: i32 = 0i32;
                    let v47: i32 = method10(v44.clone(), v45, v46);
                    let v48: i32 = v45.wrapping_sub(1i32);
                    let v49: Rc<str> = string_slice(&v44.clone(), v47 as i64, v48 as i64);
                    let v50: i32 = (v49.clone().len() as i32);
                    let v51: i32 = method11(v49.clone(), v50);
                    let v52: Rc<str> = string_slice(&v49.clone(), 0i32 as i64, v51 as i64);
                    let v53: bool = &*v52 != "";
                    v53
                } else {
                    false
                };
                let (mut v88, mut v89, mut v90, mut v91): (Rc<str>, US0, US1, bool) = if v54 {
                    let v55: i32 = v0.wrapping_add(2i32);
                    let v56: i32 = v0.wrapping_add(1i32);
                    let v57: Rc<str> = usize::try_from(v56).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                    let v58: i32 = (v57.clone().len() as i32);
                    let v59: i32 = 0i32;
                    let v60: i32 = method10(v57.clone(), v58, v59);
                    let v61: i32 = v58.wrapping_sub(1i32);
                    let v62: Rc<str> = string_slice(&v57.clone(), v60 as i64, v61 as i64);
                    let v63: i32 = (v62.clone().len() as i32);
                    let v64: i32 = method11(v62.clone(), v63);
                    let v65: Rc<str> = string_slice(&v62.clone(), 0i32 as i64, v64 as i64);
                    let v66: US1 = US1::US1_ExpectedAt(v65.clone());
                    let (mut v67, mut v68, mut v69, mut v70): (Rc<str>, US0, US1, bool) = options_from_13(v55, v1.clone(), v2.clone(), v66.clone(), v4);
                    (v67.clone(), v68.clone(), v69.clone(), v70)
                } else {
                    let v71: bool = &*v5 == "--expect-prefix";
                    let v75: bool = if v71 {
                        let v72: i32 = v0.wrapping_add(1i32);
                        let v73: Rc<str> = usize::try_from(v72).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v74: bool = &*v73 != "";
                        v74
                    } else {
                        false
                    };
                    let (mut v84, mut v85, mut v86, mut v87): (Rc<str>, US0, US1, bool) = if v75 {
                        let v76: i32 = v0.wrapping_add(2i32);
                        let v77: i32 = v0.wrapping_add(1i32);
                        let v78: Rc<str> = usize::try_from(v77).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v79: US1 = US1::US1_ExpectedPrefix(v78.clone());
                        let (mut v80, mut v81, mut v82, mut v83): (Rc<str>, US0, US1, bool) = options_from_13(v76, v1.clone(), v2.clone(), v79.clone(), v4);
                        (v80.clone(), v81.clone(), v82.clone(), v83)
                    } else {
                        (v1.clone(), v2.clone(), v3.clone(), false)
                    };
                    (v84.clone(), v85.clone(), v86.clone(), v87)
                };
                (v88.clone(), v89.clone(), v90.clone(), v91)
            };
            (v92.clone(), v93.clone(), v94.clone(), v95)
        };
        (v96.clone(), v97.clone(), v98.clone(), v99)
    };
    (v100.clone(), v101.clone(), v102.clone(), v103)
}
fn note_edit_entry_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: i32 = 3i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let v6: i32 = method10(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let mut v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method11(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let v12: Option<i32> = (v11).trim().parse::<i32>().ok();
    let v13: Option<i32> = method12(v12.clone());
    let mut v14: Rc<dyn Fn((i32)) -> US6> = closure1();
    let mut v15: Option<US6> = v13.map(|x| v14(x));
    let v17: US6 = US6::US6_None;
    let v18: US6 = v17.clone();
    let v19: US6 = v15.unwrap_or(v18);
    let v22: i32 = match &v19 {
        US6::US6_None => {
            -1i32
        }
        US6::US6_Some(v20) => {
            let v20: i32 = *v20;
            v20
        }
    };
    let v23: i32 = 4i32;
    let v24: Rc<str> = usize::try_from(v23).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v25: i32 = (v24.clone().len() as i32);
    let v26: i32 = 0i32;
    let v27: i32 = method10(v24.clone(), v25, v26);
    let v28: i32 = v25.wrapping_sub(1i32);
    let mut v29: Rc<str> = string_slice(&v24.clone(), v27 as i64, v28 as i64);
    let v30: i32 = (v29.clone().len() as i32);
    let v31: i32 = method11(v29.clone(), v30);
    let v32: Rc<str> = string_slice(&v29.clone(), 0i32 as i64, v31 as i64);
    let v33: bool = v22 < 1i32;
    if v33 {
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note number must be a positive integer"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v1));
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v34));
        let v40: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v39 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v40
    } else {
        let v41: bool = &*v32 == "";
        if v41 {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v1));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v44, v45)), v42));
            let v48: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v47 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v48
        } else {
            let v49: i32 = 5i32;
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v51: US0 = US0::US0_ReplaceText;
            let v52: US1 = US1::US1_NoGuard;
            let v53: bool = true;
            let (mut v54, mut v55, mut v56, mut v57): (Rc<str>, US0, US1, bool) = options_from_13(v49, v50.clone(), v51.clone(), v52.clone(), v53);
            if v57 {
                let v58: i32 = 1i32;
                let v59: Rc<str> = usize::try_from(v58).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_edited_1(v59.clone(), v1.clone(), v22, v32.clone(), v54.clone(), v55.clone(), v56.clone())
            } else {
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note-edit <root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note-edit "); } LIT.with(|lit| lit.clone()) };
                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v1));
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v63, v64)), v61));
                let v67: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v66 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v67
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
