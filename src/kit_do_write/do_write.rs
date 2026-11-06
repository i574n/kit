#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_verbs::{write_verb, option_write_verb};
use kit_do_model::{query_verb};
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
enum US6 {
    US6_Some(query_verb),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Succeeded,
    US7_Failed,
}
fn now_ms_1() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn body_at_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3
}
fn quoted_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn append_parsed_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_7(v0.clone(), v1, v2);
    let v4: bool = &*v3 == "";
    let v7: bool = if v4 {
        true
    } else {
        let v5: i32 = 2i32;
        quoted_at_8(v0.clone(), v1, v5)
    };
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected a file, then the quoted line"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v8)
    } else {
        let v10: i32 = 3i32;
        let v11: bool = quoted_at_8(v0.clone(), v1, v10);
        let v12: bool = v11 == false;
        if v12 {
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected the quoted line after the file"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v13)
        } else {
            let v15: i32 = 3i32;
            let v16: Rc<str> = body_at_7(v0, v1, v15);
            let v17: US1 = US1::US1_AppendLine(v3, v16);
            US0::US0_WriteParsed(v17)
        }
    }
}
fn field_index_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_span_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; std::rc::Rc::<str>::from(fields.iter().take(until).skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")) };
    v4
}
fn copy_parsed_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_7(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_10(v0.clone(), v1, v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = field_index_10(v0.clone(), v1, v6);
    let v8: bool = &*v3 == "";
    let v10: bool = if v8 {
        true
    } else {
        let v9: bool = &*v3 == "to";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected copy <glob> to <folder> [except <glob,...>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v11)
    } else {
        let mut v13: bool = v5 < 0i32;
        if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: expected to <folder>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v14)
        } else {
            let v16: i32 = v5.wrapping_add(1i32);
            let v17: Rc<str> = body_at_7(v0.clone(), v1, v16);
            let v18: bool = v7 < 0i32;
            let mut v23: Rc<str> = if v18 {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v19
            } else {
                let v20: i32 = v7.wrapping_add(1i32);
                let v21: i32 = -1i32;
                words_span_11(v0, v1, v20, v21)
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
                US0::US0_WriteRejected(v27)
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
                    US0::US0_WriteRejected(v32)
                } else {
                    let v34: US1 = US1::US1_CopyTree(v3, v17, v23);
                    US0::US0_WriteParsed(v34)
                }
            }
        }
    }
}
fn move_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_7(v0.clone(), v1, v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_10(v0.clone(), v1, v4);
    let mut v6: bool = &*v3 == "";
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v3 == "to";
        v7
    };
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move: expected move <path> to <path>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v9)
    } else {
        let v11: bool = v5 < 0i32;
        if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move: expected to <path>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v12)
        } else {
            let v14: i32 = v5.wrapping_add(1i32);
            let v15: Rc<str> = body_at_7(v0, v1, v14);
            let v16: bool = &*v15 == "";
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("to: expected a path after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_WriteRejected(v17)
            } else {
                let v19: US1 = US1::US1_MovePath(v3, v15);
                US0::US0_WriteParsed(v19)
            }
        }
    }
}
fn file_write_parsed_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 2i32;
    let v3: Rc<str> = body_at_7(v0.clone(), v1, v2);
    let v4: i32 = 3i32;
    let v5: bool = quoted_at_8(v0.clone(), v1, v4);
    let mut v6: bool = v5 == false;
    let v10: bool = if v6 {
        let v7: i32 = 3i32;
        let v8: Rc<str> = body_at_7(v0.clone(), v1, v7);
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
        quoted_at_8(v0.clone(), v1, v13)
    };
    if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write: expected a file, then a <<< block"); } LIT.with(|lit| lit.clone()) };
        US0::US0_WriteRejected(v16)
    } else {
        let v18: bool = quoted_at_8(v0.clone(), v1, v11);
        let v19: bool = v18 == false;
        if v19 {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write: expected the <<< block after the file"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v20)
        } else {
            let mut v22: Rc<str> = body_at_7(v0, v1, v11);
            let v25: US2 = if v10 {
                US2::US2_Overwrite
            } else {
                US2::US2_CreateOnly
            };
            let v26: US1 = US1::US1_WriteFile(v3, v22, v25);
            US0::US0_WriteParsed(v26)
        }
    }
}
fn file_step_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: write_verb) -> US0 {
    match v2 {
        write_verb::write_verb_FileAppend => {
            append_parsed_6(v0, v1)
        }
        write_verb::write_verb_FileCopy => {
            copy_parsed_9(v0, v1)
        }
        write_verb::write_verb_FileMove => {
            move_parsed_12(v0, v1)
        }
        write_verb::write_verb_FileWrite => {
            file_write_parsed_13(v0, v1)
        }
    }
}
fn field_at_14(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn write_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let v3: Rc<str> = field_at_14(v0.clone(), v1, v2);
    let v4: i32 = 1i32;
    let v5: Rc<str> = field_at_14(v0.clone(), v1, v4);
    let mut v6: bool = &*v3 == "";
    if v6 {
        let v7: US1 = US1::US1_Blank;
        US0::US0_WriteParsed(v7)
    } else {
        let v9: bool = &*v3 != "query";
        if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and write statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
            US0::US0_WriteRejected(v10)
        } else {
            let v12: option_write_verb = kit_do_verbs::kit_do_write_named(&v5);
            match v12 {
                option_write_verb::option_write_verb_None => {
                    let v15: query_verb = query_verb::query_verb_Search;
                    let v16: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v15);
                    let v17: bool = &*v5 == &*v16;
                    let v51: US6 = if v17 {
                        let v18: query_verb = query_verb::query_verb_Search;
                        US6::US6_Some(v18)
                    } else {
                        let v20: query_verb = query_verb::query_verb_Read;
                        let v21: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v20);
                        let mut v22: bool = &*v5 == &*v21;
                        if v22 {
                            let mut v23: query_verb = query_verb::query_verb_Read;
                            US6::US6_Some(v23)
                        } else {
                            let v25: query_verb = query_verb::query_verb_Files;
                            let v26: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v25);
                            let v27: bool = &*v5 == &*v26;
                            if v27 {
                                let v28: query_verb = query_verb::query_verb_Files;
                                US6::US6_Some(v28)
                            } else {
                                let v30: query_verb = query_verb::query_verb_Head;
                                let v31: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v30);
                                let v32: bool = &*v5 == &*v31;
                                if v32 {
                                    let v33: query_verb = query_verb::query_verb_Head;
                                    US6::US6_Some(v33)
                                } else {
                                    let v35: query_verb = query_verb::query_verb_Tail;
                                    let v36: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v35);
                                    let v37: bool = &*v5 == &*v36;
                                    if v37 {
                                        let mut v38: query_verb = query_verb::query_verb_Tail;
                                        US6::US6_Some(v38)
                                    } else {
                                        let v40: query_verb = query_verb::query_verb_Wait;
                                        let v41: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v40);
                                        let v42: bool = &*v5 == &*v41;
                                        if v42 {
                                            let v43: query_verb = query_verb::query_verb_Wait;
                                            US6::US6_Some(v43)
                                        } else {
                                            US6::US6_None
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match v51 {
                        US6::US6_None => {
                            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with append, copy or move yet; run it as its own call"); } LIT.with(|lit| lit.clone()) }));
                            US0::US0_WriteRejected(v55)
                        }
                        US6::US6_Some(v52) => {
                            let v52: query_verb = v52;
                            let v53: US1 = US1::US1_ReadAfter;
                            US0::US0_WriteParsed(v53)
                        }
                    }
                }
                option_write_verb::option_write_verb_Some(v13) => {
                    let mut v13: write_verb = v13;
                    file_step_parsed_5(v0, v1, v13)
                }
            }
        }
    }
}
fn read_rejection_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_do_read::kit_do_read_rejection(&*v0);
    v1
}
fn record_alone_17(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(format!("ok{}{}", char::from(30u8), v0.get(v1 as usize).map(|fields| fields.iter().map(|field| field.to_string()).collect::<Vec<_>>().join(&char::from(31u8).to_string())).unwrap_or_default()));
    v2
}
fn read_rejected_15(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: US1) -> Rc<str> {
    match v2 {
        US1::US1_ReadAfter => {
            let v3: Rc<str> = record_alone_17(v0, v1);
            read_rejection_16(v3)
        }
        _ => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v5
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US0 = write_parsed_4(v0.clone(), v1);
            match v4 {
                US0::US0_WriteParsed(v6) => {
                    let mut v6: US1 = v6;
                    let v7: Rc<str> = read_rejected_15(v0.clone(), v1, v6);
                    let v8: bool = &*v7 == "";
                    if v8 {
                        let v9: i32 = v1.wrapping_add(1i32);
                        v1 = v9;
                        continue;
                    } else {
                        return v7;
                    }
                }
                US0::US0_WriteRejected(v5) => {
                    let v5: Rc<str> = v5;
                    return v5;
                }
            }
        } else {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v14;
        }
    }
}
fn empty_write_tally_18() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn lines_joined_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1
        } else {
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
            v5
        }
    }
}
fn append_run_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64) = kit_do_file_steps::kit_do_append_step(&*v0, &*v1, &*v2);
    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v20: Rc<str> = lines_joined_22(v3, v16);
        let v21: u64 = v10.wrapping_add(1u64);
        (v20, true, v5, v6, v7, v8, v9, v21, v11, v12, v13, v14, v15)
    } else {
        let mut v22: Rc<str> = lines_joined_22(v3, v16);
        let mut v23: u64 = v5.wrapping_add(1u64);
        let v24: u64 = v6.wrapping_add(v18);
        let v25: u64 = v9.wrapping_add(v19);
        let v26: u64 = v10.wrapping_add(1u64);
        (v22, v4, v23, v24, v7, v8, v25, v26, v11, v12, v13, v14, v15)
    };
    (v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39)
}
fn copy_run_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, bool, u64, u64, u64) = kit_do_file_steps::kit_do_copy_step(&*v0, &*v1, &*v2, &*v3);
    let mut v22: Rc<str> = lines_joined_22(v4, v17);
    let mut v23: bool = v5 || v18;
    let v24: u64 = v8.wrapping_add(v19);
    let v25: u64 = v9.wrapping_add(v20);
    let v26: u64 = v10.wrapping_add(v21);
    let v27: u64 = v12.wrapping_add(1u64);
    (v22, v23, v6, v7, v24, v25, v26, v11, v27, v13, v14, v15, v16)
}
fn moved_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from({ let root = std::path::Path::new(&*v0); let from = root.join(&*v1); let to = root.join(&*v2); if !from.exists() { format!("move {}: no such file or directory", &*v1) } else if to.exists() { format!("move {}: {} already exists", &*v1, &*v2) } else { let made = to.parent().map(std::fs::create_dir_all).transpose(); match made.and_then(|_| std::fs::rename(&from, &to).or_else(|_| if from.is_file() { std::fs::copy(&from, &to).and_then(|_| std::fs::remove_file(&from)) } else { Err(std::io::Error::other("a folder cannot move across volumes")) })) { Ok(_) => String::new(), Err(error) => format!("move {} -> {}: {error}", &*v1, &*v2) } } }.as_str());
    v3
}
fn move_run_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: Rc<str> = moved_25(v0, v1.clone(), v2.clone());
    let v17: bool = &*v16 == "";
    let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move "); } LIT.with(|lit| lit.clone()) }, v1));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) }));
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v2));
        let v21: Rc<str> = lines_joined_22(v3, v20);
        let mut v22: u64 = v12.wrapping_add(1u64);
        let mut v23: u64 = v13.wrapping_add(1u64);
        (v21, v4, v5, v6, v7, v8, v9, v10, v11, v22, v23, v14, v15)
    } else {
        let v24: Rc<str> = lines_joined_22(v3, v16);
        let v25: u64 = v13.wrapping_add(1u64);
        (v24, true, v5, v6, v7, v8, v9, v10, v11, v12, v25, v14, v15)
    };
    (v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38)
}
fn method27(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_28(mut v0: Rc<str>) -> bool {
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
fn method30(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn directory_get_parent_29(mut v0: Rc<str>) -> Option<Rc<str>> {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method30(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let v21: bool = true; v19 });
    let mut v23: Option<Rc<str>> = _optionm_map_;
    v23
}
fn text_bytes_31(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = (v0).len() as u64;
    v1
}
fn line_count_32(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let mut v6: Rc<str> = v2.borrow()[v5 as usize].clone();
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
fn decimal_text_34(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn counted_33(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_34(v0);
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v5: bool = v0 == 1u64;
    let mut v6: Rc<str> = if v5 {
        v1
    } else {
        v2
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
    v7
}
fn write_run_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US2, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v18: bool = v1.starts_with(&*v17);
    let v27: bool = if v18 {
        true
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
        let v20: Option<usize> = v1.find(&*v19);
        let v21: Option<usize> = method27(v20);
        let mut v22: Rc<dyn Fn((usize)) -> i32> = closure1();
        let mut v23: Option<i32> = v21.map(|x| v22(x));
        let v25: i32 = v23.unwrap_or(-1i32);
        let v26: bool = v25 == 1i32;
        v26
    };
    let v30: Rc<str> = if v27 {
        v1.clone()
    } else {
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v1.clone()));
        v29
    };
    let v31: bool = file_exists_28(v30.clone());
    let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v3 {
        US2::US2_CreateOnly => {
            if v31 {
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": already exists; write "); } LIT.with(|lit| lit.clone()) }));
                let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v1));
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" overwrite <<< ... >>> replaces it"); } LIT.with(|lit| lit.clone()) }));
                let v36: Rc<str> = lines_joined_22(v4, v35);
                let v37: u64 = v16.wrapping_add(1u64);
                (v36, true, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v37)
            } else {
                let mut v38: Option<Rc<str>> = directory_get_parent_29(v30.clone());
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = v40.clone();
                let v42: Rc<str> = v38.unwrap_or(v41);
                let v43: bool = &*v42 != "";
                if v43 {
                    let v45: Result<(), std::io::Error> = std::fs::create_dir_all(&*v42);
                    ()
                };
                std::fs::write(&*v30, &*v2).unwrap();
                let v47: u64 = text_bytes_31(v2.clone());
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) }, v1));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v50: u64 = line_count_32(v2);
                let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
                let v53: Rc<str> = counted_33(v50, v51, v52);
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v53));
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v54, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<str> = counted_33(v47, v56, v57);
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v58));
                let v60: Rc<str> = lines_joined_22(v4, v59);
                let v61: u64 = v15.wrapping_add(1u64);
                let v62: u64 = v10.wrapping_add(v47);
                let v63: u64 = v16.wrapping_add(1u64);
                (v60, v5, v6, v7, v8, v9, v62, v11, v12, v13, v14, v61, v63)
            }
        }
        _ => {
            let mut v77: Option<Rc<str>> = directory_get_parent_29(v30.clone());
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v80: Rc<str> = v79.clone();
            let v81: Rc<str> = v77.unwrap_or(v80);
            let v82: bool = &*v81 != "";
            if v82 {
                let v84: Result<(), std::io::Error> = std::fs::create_dir_all(&*v81);
                ()
            };
            std::fs::write(&*v30, &*v2).unwrap();
            let v86: u64 = text_bytes_31(v2.clone());
            let v89: Rc<str> = if v31 {
                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("overwrite "); } LIT.with(|lit| lit.clone()) };
                v87
            } else {
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write "); } LIT.with(|lit| lit.clone()) };
                v88
            };
            let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v1));
            let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v92: u64 = line_count_32(v2);
            let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
            let v95: Rc<str> = counted_33(v92, v93, v94);
            let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v95));
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v96, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
            let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
            let v100: Rc<str> = counted_33(v86, v98, v99);
            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v100));
            let v102: Rc<str> = lines_joined_22(v4, v101);
            let v103: u64 = v15.wrapping_add(1u64);
            let v104: u64 = v10.wrapping_add(v86);
            let v105: u64 = v16.wrapping_add(1u64);
            (v102, v5, v6, v7, v8, v9, v104, v11, v12, v13, v14, v103, v105)
        }
    };
    (v106, v107, v108, v109, v110, v111, v112, v113, v114, v115, v116, v117, v118)
}
fn read_report_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_do_read::kit_do_read_report(&*v0, &*v1);
    v2
}
fn part_at_37(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn read_run_35(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: Rc<str> = record_alone_17(v1, v2);
    let v17: Rc<str> = read_report_36(v0, v16);
    let v18: i32 = 0i32;
    let v19: Rc<str> = part_at_37(v17.clone(), v18);
    let v20: Rc<str> = lines_joined_22(v3, v19);
    let v24: bool = if v4 {
        true
    } else {
        let v21: i32 = 1i32;
        let mut v22: Rc<str> = part_at_37(v17, v21);
        let mut v23: bool = &*v22 == "1";
        v23
    };
    (v20, v24, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
}
fn step_run_20(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: US1, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
        US1::US1_AppendLine(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            append_run_21(v0, v17, v18, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
        }
        _ => {
            let (mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                US1::US1_CopyTree(v32, v33, v34) => {
                    let v32: Rc<str> = v32.clone();
                    let v33: Rc<str> = v33.clone();
                    let v34: Rc<str> = v34.clone();
                    copy_run_23(v0, v32, v33, v34, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                }
                _ => {
                    let (mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                        US1::US1_MovePath(v48, v49) => {
                            let v48: Rc<str> = v48.clone();
                            let v49: Rc<str> = v49.clone();
                            move_run_24(v0, v48, v49, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                        }
                        _ => {
                            let (mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                US1::US1_WriteFile(v63, v64, v65) => {
                                    let v63: Rc<str> = v63.clone();
                                    let v64: Rc<str> = v64.clone();
                                    let v65: US2 = v65.clone();
                                    write_run_26(v0, v63, v64, v65, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                                }
                                _ => {
                                    let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v3 {
                                        US1::US1_ReadAfter => {
                                            read_run_35(v0, v1, v2, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
                                        }
                                        _ => {
                                            let (mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v3 {
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
fn writes_run_19(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: i32 = v1.len() as i32;
    let v17: bool = v2 < v16;
    let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: US0 = write_parsed_4(v1.clone(), v2);
        let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_WriteParsed(v19) => {
                let v19: US1 = v19.clone();
                let v20: i32 = v2.wrapping_add(1i32);
                let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_20(v0.clone(), v1.clone(), v2, v19, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                writes_run_19(v0, v1, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
            }
            _ => {
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v18 {
                    US0::US0_WriteRejected(v47) => {
                        let v47: Rc<str> = v47;
                        let v48: i32 = v2.wrapping_add(1i32);
                        writes_run_19(v0, v1, v48, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
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
fn written_counts_39(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let mut v13: Rc<str> = decimal_text_34(v2);
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  appended "); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = decimal_text_34(v3);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: bool = v8 == 0u64;
    let v27: Rc<str> = if v18 {
        v17
    } else {
        let v19: bool = v7 == 0u64;
        if v19 {
            let v20: Rc<str> = decimal_text_34(v4);
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v20));
            let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" copied"); } LIT.with(|lit| lit.clone()) }));
            v22
        } else {
            let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  copied "); } LIT.with(|lit| lit.clone()) }));
            let v24: Rc<str> = decimal_text_34(v4);
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            v25
        }
    };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) }));
    let v29: Rc<str> = decimal_text_34(v6);
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    if v18 {
        v30
    } else {
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  skipped "); } LIT.with(|lit| lit.clone()) }));
        let v32: Rc<str> = decimal_text_34(v5);
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
        v33
    }
}
fn receipt_counts_38(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
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
        let v20: Rc<str> = decimal_text_34(v11);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v20));
        let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written  bytes "); } LIT.with(|lit| lit.clone()) }));
        let mut v23: Rc<str> = decimal_text_34(v6);
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
        v24
    } else {
        let v25: bool = v7 == 0u64;
        let v27: bool = if v25 {
            let v26: bool = v8 == 0u64;
            v26
        } else {
            false
        };
        if v27 {
            let v28: Rc<str> = decimal_text_34(v9);
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v28));
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" moved"); } LIT.with(|lit| lit.clone()) }));
            v30
        } else {
            let v31: Rc<str> = written_counts_39(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12);
            let v32: bool = v10 == 0u64;
            if v32 {
                v31
            } else {
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  moved "); } LIT.with(|lit| lit.clone()) }));
                let v34: Rc<str> = decimal_text_34(v9);
                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
                v35
            }
        }
    }
}
fn elapsed_text_41(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_43() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_42() -> Rc<str> {
    let v0: i64 = local_offset_minutes_43();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_40(mut v0: US7, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match v0 {
        US7::US7_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US7::US7_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_41(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let mut v13: Rc<str> = clock_text_42();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn write_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let mut v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_write_tally_18();
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = writes_run_19(v0, v3, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
        let v34: bool = &*v21 == "";
        if v34 {
            ()
        } else {
            println!("{}", v21);
            ()
        };
        if v22 {
            let v35: US7 = US7::US7_Failed;
            let v36: Rc<str> = receipt_counts_38(&v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_40(v35, v36, v2);
            1i32
        } else {
            let v37: US7 = US7::US7_Succeeded;
            let mut v38: Rc<str> = receipt_counts_38(&v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_40(v37, v38, v2);
            0i32
        }
    } else {
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) }, v5));
        println!("{}", v40);
        let v41: US7 = US7::US7_Failed;
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_40(v41, v42, v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        write_program_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_write_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
