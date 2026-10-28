#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US2 {
    US2_CreateOnly,
    US2_Overwrite,
}
#[derive(Clone)]
enum US1 {
    US1_AppendLine(Rc<str>, Rc<str>),
    US1_WriteFile(Rc<str>, Rc<str>, US2),
    US1_CopyTree(Rc<str>, Rc<str>, Rc<str>),
    US1_MovePath(Rc<str>, Rc<str>),
    US1_ReadAfter,
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_WriteParsed(US1),
    US0_WriteRejected(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Search,
    US4_ReadAround,
    US4_ListFiles,
    US4_Head,
    US4_Tail,
    US4_Wait,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Succeeded,
    US5_Failed,
}
fn now_ms_1() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn body_at_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn quoted_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn append_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let v4: bool = &*v3 == "";
    let v7: bool = if v4 {
        true
    } else {
        let v5: i32 = 2i32;
        quoted_at_7(v0.clone(), v1, v5)
    };
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected a file, then the quoted line"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v8.clone())
    } else {
        let v10: i32 = 3i32;
        let v11: bool = quoted_at_7(v0.clone(), v1, v10);
        let v12: bool = v11 == false;
        if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected the quoted line after the file"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v13.clone())
        } else {
            let v15: i32 = 3i32;
            let v16: Rc<str> = body_at_6(v0.clone(), v1, v15);
            let v17: US1 = US1::US1_AppendLine(v3.clone(), v16.clone());
            US0::US0_WriteParsed(v17.clone())
        }
    }
}
fn field_index_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_span_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; std::rc::Rc::<str>::from(fields.iter().take(until).skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")) };
    v4.clone()
}
fn copy_parsed_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_9(v0.clone(), v1, v4.clone());
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = field_index_9(v0.clone(), v1, v6.clone());
    let v8: bool = &*v3 == "";
    let v10: bool = if v8 {
        true
    } else {
        let v9: bool = &*v3 == "to";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected copy <glob> to <folder> [except <glob,...>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v11.clone())
    } else {
        let mut v13: bool = v5 < 0i32;
        if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected to <folder>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v14.clone())
        } else {
            let v16: i32 = v5.wrapping_add(1i32);
            let v17: Rc<str> = body_at_6(v0.clone(), v1, v16);
            let v18: bool = v7 < 0i32;
            let mut v23: Rc<str> = if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v19.clone()
            } else {
                let v20: i32 = v7.wrapping_add(1i32);
                let v21: i32 = -1i32;
                words_span_10(v0.clone(), v1, v20, v21)
            };
            let v24: bool = &*v17 == "";
            let v26: bool = if v24 {
                true
            } else {
                let v25: bool = &*v17 == "except";
                v25
            };
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to: expected a folder after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_WriteRejected(v27.clone())
            } else {
                let v29: bool = v7 > 0i32;
                let v31: bool = if v29 {
                    let v30: bool = &*v23 == "";
                    v30
                } else {
                    false
                };
                if v31 {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except: expected a glob after it"); } LIT.with(|lit| lit.clone()) };
                    US0::US0_WriteRejected(v32.clone())
                } else {
                    let v34: US1 = US1::US1_CopyTree(v3.clone(), v17.clone(), v23.clone());
                    US0::US0_WriteParsed(v34.clone())
                }
            }
        }
    }
}
fn move_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_9(v0.clone(), v1, v4.clone());
    let mut v6: bool = &*v3 == "";
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v3 == "to";
        v7
    };
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move: expected move <path> to <path>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v9.clone())
    } else {
        let v11: bool = v5 < 0i32;
        if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move: expected to <path>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v12.clone())
        } else {
            let v14: i32 = v5.wrapping_add(1i32);
            let v15: Rc<str> = body_at_6(v0.clone(), v1, v14);
            let v16: bool = &*v15 == "";
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to: expected a path after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_WriteRejected(v17.clone())
            } else {
                let v19: US1 = US1::US1_MovePath(v3.clone(), v15.clone());
                US0::US0_WriteParsed(v19.clone())
            }
        }
    }
}
fn file_write_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_6(v0.clone(), v1, v2);
    let v4: i32 = 3i32;
    let v5: bool = quoted_at_7(v0.clone(), v1, v4);
    let mut v6: bool = v5 == false;
    let v10: bool = if v6 {
        let v7: i32 = 3i32;
        let v8: Rc<str> = body_at_6(v0.clone(), v1, v7);
        let v9: bool = &*v8 == "overwrite";
        v9
    } else {
        false
    };
    let v11: i32 = if v10 {
        4i32
    } else {
        3i32
    };
    let v12: bool = &*v3 == "";
    let v15: bool = if v12 {
        true
    } else {
        let mut v13: i32 = 2i32;
        quoted_at_7(v0.clone(), v1, v13)
    };
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write: expected a file, then a <<< block"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v16.clone())
    } else {
        let v18: bool = quoted_at_7(v0.clone(), v1, v11);
        let v19: bool = v18 == false;
        if v19 {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write: expected the <<< block after the file"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v20.clone())
        } else {
            let mut v22: Rc<str> = body_at_6(v0.clone(), v1, v11);
            let v25: US2 = if v10 {
                US2::US2_Overwrite
            } else {
                US2::US2_CreateOnly
            };
            let v26: US1 = US1::US1_WriteFile(v3.clone(), v22.clone(), v25.clone());
            US0::US0_WriteParsed(v26.clone())
        }
    }
}
fn field_at_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn write_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let v3: Rc<str> = field_at_13(v0.clone(), v1, v2);
    let v4: i32 = 1i32;
    let v5: Rc<str> = field_at_13(v0.clone(), v1, v4);
    let mut v6: bool = &*v3 == "";
    if v6 {
        let v7: US1 = US1::US1_Blank;
        US0::US0_WriteParsed(v7.clone())
    } else {
        let v9: bool = &*v3 != "query";
        if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and write statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v10.clone())
        } else {
            let v12: bool = &*v5 == "append";
            if v12 {
                append_parsed_5(v0.clone(), v1)
            } else {
                let v14: bool = &*v5 == "copy";
                if v14 {
                    copy_parsed_8(v0.clone(), v1)
                } else {
                    let v16: bool = &*v5 == "move";
                    if v16 {
                        move_parsed_11(v0.clone(), v1)
                    } else {
                        let v18: bool = &*v5 == "write";
                        if v18 {
                            file_write_parsed_12(v0.clone(), v1)
                        } else {
                            let v20: bool = &*v5 == "search";
                            let v44: US3 = if v20 {
                                let v21: US4 = US4::US4_Search;
                                US3::US3_Some(v21.clone())
                            } else {
                                let mut v23: bool = &*v5 == "read";
                                if v23 {
                                    let v24: US4 = US4::US4_ReadAround;
                                    US3::US3_Some(v24.clone())
                                } else {
                                    let v26: bool = &*v5 == "files";
                                    if v26 {
                                        let v27: US4 = US4::US4_ListFiles;
                                        US3::US3_Some(v27.clone())
                                    } else {
                                        let v29: bool = &*v5 == "head";
                                        if v29 {
                                            let v30: US4 = US4::US4_Head;
                                            US3::US3_Some(v30.clone())
                                        } else {
                                            let v32: bool = &*v5 == "tail";
                                            if v32 {
                                                let v33: US4 = US4::US4_Tail;
                                                US3::US3_Some(v33.clone())
                                            } else {
                                                let v35: bool = &*v5 == "wait";
                                                if v35 {
                                                    let v36: US4 = US4::US4_Wait;
                                                    US3::US3_Some(v36.clone())
                                                } else {
                                                    US3::US3_None
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match &v44 {
                                US3::US3_None => {
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with append, copy or move yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
                                    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v48));
                                    US0::US0_WriteRejected(v49.clone())
                                }
                                US3::US3_Some(v45) => {
                                    let v45: US4 = v45.clone();
                                    let v46: US1 = US1::US1_ReadAfter;
                                    US0::US0_WriteParsed(v46.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn read_rejection_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_do_read::kit_do_read_rejection(&*v0);
    v1
}
fn record_alone_16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("ok{}{}", char::from(30u8), v0.get(v1 as usize).map(|fields| fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(&char::from(31u8).to_string())).unwrap_or_default()));
    v2.clone()
}
fn read_rejected_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: US1) -> Rc<str> {
    match &v2 {
        US1::US1_ReadAfter => {
            let v3: Rc<str> = record_alone_16(v0.clone(), v1);
            read_rejection_15(v3.clone())
        }
        _ => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5.clone()
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US0 = write_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_WriteParsed(v6) => {
                    let mut v6: US1 = v6.clone();
                    let v7: Rc<str> = read_rejected_14(v0.clone(), v1, v6.clone());
                    let v8: bool = &*v7 == "";
                    if v8 {
                        let v9: i32 = v1.wrapping_add(1i32);
                        (v0, v1) = (v0.clone(), v9);
                        continue;
                    } else {
                        return v7.clone();
                    }
                }
                US0::US0_WriteRejected(v5) => {
                    let v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v14.clone();
        }
    }
}
fn empty_write_tally_17() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn lines_joined_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0.clone()
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v4)), v1));
            v6.clone()
        }
    }
}
fn append_run_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64) = kit_do_file_steps::kit_do_append_step(&*v0, &*v1, &*v2);
    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v20: Rc<str> = lines_joined_21(v3.clone(), v16.clone());
        let v21: u64 = v10.wrapping_add(1u64);
        (v20, true, v5, v6, v7, v8, v9, v21, v11, v12, v13, v14, v15)
    } else {
        let mut v22: Rc<str> = lines_joined_21(v3, v16);
        let mut v23: u64 = v5.wrapping_add(1u64);
        let v24: u64 = v6.wrapping_add(v18);
        let v25: u64 = v9.wrapping_add(v19);
        let v26: u64 = v10.wrapping_add(1u64);
        (v22, v4, v23, v24, v7, v8, v25, v26, v11, v12, v13, v14, v15)
    };
    (v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39)
}
fn copy_run_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, bool, u64, u64, u64) = kit_do_file_steps::kit_do_copy_step(&*v0, &*v1, &*v2, &*v3);
    let mut v22: Rc<str> = lines_joined_21(v4.clone(), v17.clone());
    let mut v23: bool = v5 || v18;
    let v24: u64 = v8.wrapping_add(v19);
    let v25: u64 = v9.wrapping_add(v20);
    let v26: u64 = v10.wrapping_add(v21);
    let v27: u64 = v12.wrapping_add(1u64);
    (v22.clone(), v23, v6, v7, v24, v25, v26, v11, v27, v13, v14, v15, v16)
}
fn moved_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from({ let root = std::path::Path::new(&*v0); let from = root.join(&*v1); let to = root.join(&*v2); if !from.exists() { format!("move {}: no such file or directory", &*v1) } else if to.exists() { format!("move {}: {} already exists", &*v1, &*v2) } else { let made = to.parent().map(std::fs::create_dir_all).transpose(); match made.and_then(|_| std::fs::rename(&from, &to).or_else(|_| if from.is_file() { std::fs::copy(&from, &to).and_then(|_| std::fs::remove_file(&from)) } else { Err(std::io::Error::other("a folder cannot move across volumes")) })) { Ok(_) => String::new(), Err(error) => format!("move {} -> {}: {error}", &*v1, &*v2) } } }.as_str());
    v3.clone()
}
fn move_run_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: Rc<str> = moved_24(v0.clone(), v1.clone(), v2.clone());
    let v17: bool = &*v16 == "";
    let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move "); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v1));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v2));
        let mut v23: Rc<str> = lines_joined_21(v3.clone(), v22.clone());
        let v24: u64 = v12.wrapping_add(1u64);
        let v25: u64 = v13.wrapping_add(1u64);
        (v23.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v24, v25, v14, v15)
    } else {
        let v26: Rc<str> = lines_joined_21(v3.clone(), v16.clone());
        let v27: u64 = v13.wrapping_add(1u64);
        (v26.clone(), true, v5, v6, v7, v8, v9, v10, v11, v12, v27, v14, v15)
    };
    (v28.clone(), v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40)
}
fn method26(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_27(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method29(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn directory_get_parent_28(mut v0: Rc<str>) -> Option<Rc<str>> {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method29(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let v21: bool = true; v19 });
    let mut v23: Option<Rc<str>> = _optionm_map_;
    v23.clone()
}
fn text_bytes_30(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).len() as u64;
    v1
}
fn line_count_31(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let mut v6: Rc<str> = v2.clone().borrow()[v5 as usize].clone();
        let v7: bool = &*v6 == "";
        v7
    } else {
        false
    };
    let v10: i32 = if v8 {
        let v9: i32 = v3.wrapping_sub(1i32);
        v9
    } else {
        v3
    };
    let v11: u64 = (v10 as u64);
    v11
}
fn decimal_text_33(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn counted_32(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_33(v0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: bool = v0 == 1u64;
    let v7: Rc<str> = if v6 {
        v1.clone()
    } else {
        v2.clone()
    };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v7));
    v8.clone()
}
fn write_run_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v18: bool = v1.starts_with(&*v17);
    let v27: bool = if v18 {
        true
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v20: Option<usize> = v1.find(&*v19);
        let v21: Option<usize> = method26(v20.clone());
        let mut v22: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v23: Option<i32> = v21.map(|x| v22(x));
        let v25: i32 = v23.unwrap_or(-1i32);
        let v26: bool = v25 == 1i32;
        v26
    };
    let v30: Rc<str> = if v27 {
        v1.clone()
    } else {
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v17)), v1));
        v29.clone()
    };
    let v31: bool = file_exists_27(v30.clone());
    let (mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
        US2::US2_CreateOnly => {
            if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": already exists; write "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v1));
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" overwrite <<< ... >>> replaces it"); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                let v39: Rc<str> = lines_joined_21(v4.clone(), v38.clone());
                let v40: u64 = v16.wrapping_add(1u64);
                (v39.clone(), true, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v40)
            } else {
                let mut v41: Option<Rc<str>> = directory_get_parent_28(v30.clone());
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = v43.clone();
                let v45: Rc<str> = v41.unwrap_or(v44);
                let v46: bool = &*v45 != "";
                if v46 {
                    let v48: Result<(), std::io::Error> = std::fs::create_dir_all(&*v45);
                    ()
                };
                std::fs::write(&*v30, &*v2).unwrap();
                let v50: u64 = text_bytes_30(v2.clone());
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v1));
                let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
                let v55: u64 = line_count_31(v2.clone());
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<str> = counted_32(v55, v56.clone(), v57.clone());
                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v54, v58)), v53));
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
                let v63: Rc<str> = counted_32(v50, v61.clone(), v62.clone());
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v63));
                let v65: Rc<str> = lines_joined_21(v4.clone(), v64.clone());
                let v66: u64 = v15.wrapping_add(1u64);
                let v67: u64 = v10.wrapping_add(v50);
                let v68: u64 = v16.wrapping_add(1u64);
                (v65.clone(), v5, v6, v7, v8, v9, v67, v11, v12, v13, v14, v66, v68)
            }
        }
        _ => {
            let mut v82: Option<Rc<str>> = directory_get_parent_28(v30.clone());
            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v85: Rc<str> = v84.clone();
            let v86: Rc<str> = v82.unwrap_or(v85);
            let v87: bool = &*v86 != "";
            if v87 {
                let v89: Result<(), std::io::Error> = std::fs::create_dir_all(&*v86);
                ()
            };
            std::fs::write(&*v30, &*v2).unwrap();
            let v91: u64 = text_bytes_30(v2.clone());
            let v94: Rc<str> = if v31 {
                let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("overwrite "); } LIT.with(|lit| lit.clone()) };
                v92.clone()
            } else {
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) };
                v93.clone()
            };
            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v1));
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
            let v98: u64 = line_count_31(v2.clone());
            let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
            let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
            let v101: Rc<str> = counted_32(v98, v99.clone(), v100.clone());
            let v103: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v97, v101)), v96));
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
            let v106: Rc<str> = counted_32(v91, v104.clone(), v105.clone());
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v106));
            let v108: Rc<str> = lines_joined_21(v4.clone(), v107.clone());
            let v109: u64 = v15.wrapping_add(1u64);
            let v110: u64 = v10.wrapping_add(v91);
            let v111: u64 = v16.wrapping_add(1u64);
            (v108.clone(), v5, v6, v7, v8, v9, v110, v11, v12, v13, v14, v109, v111)
        }
    };
    (v112.clone(), v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124)
}
fn read_report_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_do_read::kit_do_read_report(&*v0, &*v1);
    v2
}
fn part_at_36(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn read_run_34(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: Rc<str> = record_alone_16(v1, v2);
    let v17: Rc<str> = read_report_35(v0, v16);
    let v18: i32 = 0i32;
    let v19: Rc<str> = part_at_36(v17.clone(), v18);
    let v20: Rc<str> = lines_joined_21(v3, v19);
    let v24: bool = if v4 {
        true
    } else {
        let v21: i32 = 1i32;
        let mut v22: Rc<str> = part_at_36(v17, v21);
        let mut v23: bool = &*v22 == "1";
        v23
    };
    (v20, v24, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
}
fn step_run_19(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
        US1::US1_AppendLine(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            append_run_20(v0.clone(), v17, v18, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
        }
        _ => {
            let (mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                US1::US1_CopyTree(v32, v33, v34) => {
                    let v32: Rc<str> = v32.clone();
                    let v33: Rc<str> = v33.clone();
                    let v34: Rc<str> = v34.clone();
                    copy_run_22(v0.clone(), v32, v33, v34, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                }
                _ => {
                    let (mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                        US1::US1_MovePath(v48, v49) => {
                            let v48: Rc<str> = v48.clone();
                            let v49: Rc<str> = v49.clone();
                            move_run_23(v0.clone(), v48, v49, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                        }
                        _ => {
                            let (mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                US1::US1_WriteFile(v63, v64, v65) => {
                                    let v63: Rc<str> = v63.clone();
                                    let v64: Rc<str> = v64.clone();
                                    let v65: US2 = v65.clone();
                                    write_run_25(v0.clone(), v63, v64, v65, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                                }
                                _ => {
                                    let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                        US1::US1_ReadAfter => {
                                            read_run_34(v0, v1, v2, v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                                        }
                                        _ => {
                                            let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                                US1::US1_Blank => {
                                                    (v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104)
                                        }
                                    };
                                    (v105, v106, v107, v108, v109, v110, v111, v112, v113, v114, v115, v116, v117)
                                }
                            };
                            (v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130)
                        }
                    };
                    (v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143)
                }
            };
            (v144, v145, v146, v147, v148, v149, v150, v151, v152, v153, v154, v155, v156)
        }
    };
    (v157, v158, v159, v160, v161, v162, v163, v164, v165, v166, v167, v168, v169)
}
fn writes_run_18(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: i32 = v1.len() as i32;
    let v17: bool = v2 < v16;
    let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: US0 = write_parsed_4(v1.clone(), v2);
        let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_WriteParsed(v19) => {
                let v19: US1 = v19.clone();
                let v20: i32 = v2.wrapping_add(1i32);
                let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_19(v0.clone(), v1.clone(), v2, v19, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                writes_run_18(v0.clone(), v1.clone(), v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
            }
            _ => {
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
                    US0::US0_WriteRejected(v47) => {
                        let v47: Rc<str> = v47.clone();
                        let v48: i32 = v2.wrapping_add(1i32);
                        writes_run_18(v0, v1, v48, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v62, v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
            }
        };
        (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
    } else {
        (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100)
}
fn written_counts_38(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: Rc<str> = decimal_text_33(v2);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  appended "); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = decimal_text_33(v3);
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: bool = v8 == 0u64;
    let v31: Rc<str> = if v20 {
        v19.clone()
    } else {
        let v21: bool = v7 == 0u64;
        if v21 {
            let mut v22: Rc<str> = decimal_text_33(v4);
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v22));
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" copied"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            v25.clone()
        } else {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  copied "); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v26));
            let v28: Rc<str> = decimal_text_33(v4);
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            v29.clone()
        }
    };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v34: Rc<str> = decimal_text_33(v6);
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
    if v20 {
        v35.clone()
    } else {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  skipped "); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
        let v38: Rc<str> = decimal_text_33(v5);
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
        v39.clone()
    }
}
fn receipt_counts_37(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: bool = v12 > 0u64;
    let v15: bool = if v13 {
        let v14: bool = v7 == 0u64;
        v14
    } else {
        false
    };
    let v17: bool = if v15 {
        let v16: bool = v8 == 0u64;
        v16
    } else {
        false
    };
    let v19: bool = if v17 {
        let v18: bool = v10 == 0u64;
        v18
    } else {
        false
    };
    if v19 {
        let v20: Rc<str> = decimal_text_33(v11);
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v20));
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written  bytes "); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
        let v25: Rc<str> = decimal_text_33(v6);
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
        v26.clone()
    } else {
        let v27: bool = v7 == 0u64;
        let v29: bool = if v27 {
            let v28: bool = v8 == 0u64;
            v28
        } else {
            false
        };
        if v29 {
            let v30: Rc<str> = decimal_text_33(v9);
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v30));
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" moved"); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
            v34.clone()
        } else {
            let v35: Rc<str> = written_counts_38(v0.clone(), v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12);
            let v36: bool = v10 == 0u64;
            if v36 {
                v35.clone()
            } else {
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  moved "); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v37));
                let v39: Rc<str> = decimal_text_33(v9);
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
                v40.clone()
            }
        }
    }
}
fn elapsed_text_40(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_42() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_41() -> Rc<str> {
    let v0: i64 = local_offset_minutes_42();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_39(mut v0: US5, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US5::US5_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_40(v2);
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_41();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn write_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_write_tally_17();
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = writes_run_18(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
        let v34: bool = &*v21 == "";
        if v34 {
            ()
        } else {
            println!("{}", v21);
            ()
        };
        if v22 {
            let v35: US5 = US5::US5_Failed;
            let v36: Rc<str> = receipt_counts_37(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_39(v35.clone(), v36.clone(), v2);
            1i32
        } else {
            let v37: US5 = US5::US5_Succeeded;
            let v38: Rc<str> = receipt_counts_37(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_39(v37.clone(), v38.clone(), v2);
            0i32
        }
    } else {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v5));
        println!("{}", v41);
        let v42: US5 = US5::US5_Failed;
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_39(v42.clone(), v43.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        write_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_write_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
