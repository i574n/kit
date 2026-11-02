#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ShowText,
    US0_ShowJson,
    US0_ShowFiltered(Rc<str>, Rc<str>),
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
    US3_Valid(US0),
    US3_Invalid,
}
fn task_summary_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v9)), v2)), v9)), v3));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v4)), v9)), v5));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%  "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, Rc::<str>::from(format!("{}{}", v16, v17)))), v6));
    v20
}
fn show_receipt_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: u64) -> Rc<str> {
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile show ok id="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" title_bytes="); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: i32 = (v1.clone().len() as i32);
    let v15: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let v20: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
    v21.clone()
}
fn show_task_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64) -> i32 {
    let v12: Rc<str> = task_summary_3(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone());
    let v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v14: bool = &*v7 == "";
    let v16: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v15.clone()
    } else {
        v7.clone()
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps: "); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
    let v19: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v18.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v20: bool = &*v8 == "";
    let v22: Rc<str> = if v20 {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    } else {
        v8.clone()
    };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags: "); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
    let v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning: "); } LIT.with(|lit| lit.clone()) };
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v9));
    let v28: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v27.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v29: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v1.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v30: bool = &*v10 == "";
    let v32: i32 = if v30 {
        0i32
    } else {
        let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v31
    };
    let v33: Rc<str> = show_receipt_4(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v11);
    let v34: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v33.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v34
}
fn concat5_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn archived_text_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive/done-notes.spi"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v3: bool = std::path::Path::new(&*v2).is_file();
    let v6: US1 = if v3 {
        US1::US1_FileReadable(v2.clone())
    } else {
        US1::US1_FileMissing(v2)
    };
    match &v6 {
        US1::US1_FileMissing(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US1::US1_FileReadable(v9) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = std::fs::read_to_string(&*v9).unwrap_or_default().into();
            let v11: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v10);
            v11
        }
    }
}
fn state_segments_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v3: Rc<str> = agile_note_listing::kit_agile_state_segments(&*v2);
    v3
}
fn notes_joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let v3: bool = &*v1 == "";
        if v3 {
            v0.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v4)), v1));
            v6
        }
    }
}
fn field_end_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US2) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US2::US2_ScanPlain => {
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
                let v25: US2 = match &v3 {
                    US2::US2_ScanEscaped => {
                        US2::US2_ScanQuoted
                    }
                    US2::US2_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US2::US2_ScanQuoted
                        } else {
                            US2::US2_ScanPlain
                        }
                    }
                    US2::US2_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US2::US2_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US2::US2_ScanPlain
                            } else {
                                US2::US2_ScanQuoted
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
fn skip_spaces_12(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
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
fn field_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US2 = US2::US2_ScanPlain;
            let v6: i32 = field_end_11(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_12(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn unescaped_from_14(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 <= v1;
        if v5 {
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
            return v7.clone();
        } else {
            let v8: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = v8 == 92i32;
            if v9 {
                let v10: i32 = v1.wrapping_add(1i32);
                let v11: i32 = v10.wrapping_add(1i32);
                let v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
                let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v12));
                let v14: i32 = v0.as_bytes().get(v10 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
                let v15: bool = v14 == 110i32;
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
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
                            v21.clone()
                        }
                    }
                };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v24));
                (v1, v2, v4) = (v11, v11, v25.clone());
                continue;
            } else {
                let v27: i32 = v1.wrapping_add(1i32);
                v1 = v27;
                continue;
            }
        }
    }
}
fn unquoted_13(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let v5: i32 = 0i32;
    let v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_14(v3, v4, v5, v6, v7)
}
fn task_record_at_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v3: i32 = 0i32;
    let v4: i32 = field_start_10(v0.clone(), v1, v2, v3);
    let v5: US2 = US2::US2_ScanPlain;
    let v6: i32 = field_end_11(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: Rc<str> = unquoted_13(v7);
    let v9: i32 = 1i32;
    let v10: i32 = field_start_10(v0.clone(), v1, v2, v9);
    let v11: US2 = US2::US2_ScanPlain;
    let v12: i32 = field_end_11(v0.clone(), v10, v2, v11);
    let v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v12 as usize).unwrap_or(""));
    let v14: Rc<str> = unquoted_13(v13);
    let v15: i32 = 2i32;
    let v16: i32 = field_start_10(v0.clone(), v1, v2, v15);
    let v17: US2 = US2::US2_ScanPlain;
    let v18: i32 = field_end_11(v0.clone(), v16, v2, v17);
    let v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v18 as usize).unwrap_or(""));
    let v20: i32 = 3i32;
    let v21: i32 = field_start_10(v0.clone(), v1, v2, v20);
    let v22: US2 = US2::US2_ScanPlain;
    let v23: i32 = field_end_11(v0.clone(), v21, v2, v22);
    let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(v21 as usize..v23 as usize).unwrap_or(""));
    let v25: i32 = 4i32;
    let v26: i32 = field_start_10(v0.clone(), v1, v2, v25);
    let v27: US2 = US2::US2_ScanPlain;
    let v28: i32 = field_end_11(v0.clone(), v26, v2, v27);
    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v26 as usize..v28 as usize).unwrap_or(""));
    let v30: i32 = 5i32;
    let v31: i32 = field_start_10(v0.clone(), v1, v2, v30);
    let v32: US2 = US2::US2_ScanPlain;
    let v33: i32 = field_end_11(v0.clone(), v31, v2, v32);
    let v34: Rc<str> = std::rc::Rc::<str>::from(v0.get(v31 as usize..v33 as usize).unwrap_or(""));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(v34.replace(&*v35, &*v36));
    let v38: i32 = 6i32;
    let v39: i32 = field_start_10(v0.clone(), v1, v2, v38);
    let v40: US2 = US2::US2_ScanPlain;
    let v41: i32 = field_end_11(v0.clone(), v39, v2, v40);
    let v42: Rc<str> = std::rc::Rc::<str>::from(v0.get(v39 as usize..v41 as usize).unwrap_or(""));
    let v43: i32 = 7i32;
    let v44: i32 = field_start_10(v0.clone(), v1, v2, v43);
    let v45: US2 = US2::US2_ScanPlain;
    let v46: i32 = field_end_11(v0.clone(), v44, v2, v45);
    let v47: Rc<str> = std::rc::Rc::<str>::from(v0.get(v44 as usize..v46 as usize).unwrap_or(""));
    let v48: Rc<str> = unquoted_13(v47);
    let v49: i32 = 8i32;
    let v50: i32 = field_start_10(v0.clone(), v1, v2, v49);
    let v51: US2 = US2::US2_ScanPlain;
    let v52: i32 = field_end_11(v0.clone(), v50, v2, v51);
    let v53: Rc<str> = std::rc::Rc::<str>::from(v0.get(v50 as usize..v52 as usize).unwrap_or(""));
    let v54: Rc<str> = unquoted_13(v53);
    (v8, v14, v19, v24, v29, v37, v42, v48, v54)
}
fn notes_or_archived_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        v1
    }
}
fn task_json_record_at_16(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) {
    let v3: i32 = 0i32;
    let v4: i32 = field_start_10(v0.clone(), v1, v2, v3);
    let v5: US2 = US2::US2_ScanPlain;
    let v6: i32 = field_end_11(v0.clone(), v4, v2, v5);
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_10(v0.clone(), v1, v2, v11);
    let v13: US2 = US2::US2_ScanPlain;
    let v14: i32 = field_end_11(v0.clone(), v12, v2, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_10(v0.clone(), v1, v2, v19);
    let v21: US2 = US2::US2_ScanPlain;
    let v22: i32 = field_end_11(v0.clone(), v20, v2, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let v25: i32 = field_start_10(v0.clone(), v1, v2, v24);
    let v26: US2 = US2::US2_ScanPlain;
    let v27: i32 = field_end_11(v0.clone(), v25, v2, v26);
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let v29: i32 = 4i32;
    let v30: i32 = field_start_10(v0.clone(), v1, v2, v29);
    let v31: US2 = US2::US2_ScanPlain;
    let v32: i32 = field_end_11(v0.clone(), v30, v2, v31);
    let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let v34: i32 = 5i32;
    let v35: i32 = field_start_10(v0.clone(), v1, v2, v34);
    let v36: US2 = US2::US2_ScanPlain;
    let v37: i32 = field_end_11(v0.clone(), v35, v2, v36);
    let v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v38.replace(&*v39, &*v40));
    let v42: i32 = 6i32;
    let v43: i32 = field_start_10(v0.clone(), v1, v2, v42);
    let v44: US2 = US2::US2_ScanPlain;
    let v45: i32 = field_end_11(v0.clone(), v43, v2, v44);
    let v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    let v47: i32 = 7i32;
    let v48: i32 = field_start_10(v0.clone(), v1, v2, v47);
    let v49: US2 = US2::US2_ScanPlain;
    let v50: i32 = field_end_11(v0.clone(), v48, v2, v49);
    let v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v48 as usize..v50 as usize).unwrap_or(""));
    let v52: i32 = (v51.clone().len() as i32);
    let v53: i32 = v52.wrapping_sub(1i32);
    let v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let v55: i32 = 8i32;
    let v56: i32 = field_start_10(v0.clone(), v1, v2, v55);
    let v57: US2 = US2::US2_ScanPlain;
    let v58: i32 = field_end_11(v0.clone(), v56, v2, v57);
    let v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let v60: i32 = (v59.clone().len() as i32);
    let v61: i32 = v60.wrapping_sub(1i32);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10, v18, v23, v28, v33, v41, v46, v54, v62)
}
fn json_notes_of_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = agile_note_listing::kit_agile_notes_json(&*v1, &*v2);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = agile_note_listing::kit_agile_notes_json(&*v4, &*v2);
    let v6: bool = &*v3 == &*v5;
    if v6 {
        let v7: Rc<str> = agile_note_listing::kit_agile_notes_json(&*v0, &*v2);
        v7
    } else {
        v3
    }
}
fn task_json_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: u64) -> Rc<str> {
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v0)), v12));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v16)), v14));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v1)), v12));
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v23)), v16)), v25)))), v21));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v2)), v12));
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v31)), v16)), v33)), v21));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v3)), v12));
    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v37, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v38)), v16)), v40)))), v21));
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v4)), v12));
    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v45, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v46)), v16)), v48))));
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v53)), v16)), v5)))), v21));
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v6)), v12));
    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v58, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v59)), v16)), v61)))), v21));
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v7)), v12));
    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v67)), v16)), v69)), v21));
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v8)), v12));
    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v73, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v74)), v16)), v76))));
    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v9)), v21));
    let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
    let v84: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v10);
    let v85: Rc<str> = std::rc::Rc::<str>::from((v84).to_string());
    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v80, Rc::<str>::from(format!("{}{}", v82, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v83)), v16)), v85))))));
    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v30, v52)), v66)), v90));
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v93, v94));
    v95.clone()
}
fn agile_show_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_5(v0.clone(), v4.clone(), v5.clone(), v5.clone(), v5.clone());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US1 = if v7 {
        US1::US1_FileReadable(v6.clone())
    } else {
        US1::US1_FileMissing(v6.clone())
    };
    match &v10 {
        US1::US1_FileMissing(v11) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v1));
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v15)), v12));
            let v18: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v17 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v18
        }
        US1::US1_FileReadable(v19) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = std::fs::read_to_string(&*v19).unwrap_or_default().into();
            let v21: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v20);
            let v22: Rc<str> = archived_text_6(v0.clone());
            let v23: Rc<str> = state_segments_7(v0.clone());
            let v24: Rc<str> = notes_joined_8(v21.clone(), v23.clone());
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v1));
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            let v29: i32 = v21.find(&*v28).map(|offset| offset as i32).unwrap_or(-1);
            let v30: bool = v29 < 0i32;
            if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v1));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v31));
                let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v37
            } else {
                let v38: i32 = v29.wrapping_add(6i32);
                match &v2 {
                    US0::US0_ShowFiltered(v54, v55) => {
                        let v54: Rc<str> = v54.clone();
                        let v55: Rc<str> = v55.clone();
                        let v56: i32 = (v21.clone().len() as i32);
                        let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_9(v21.clone(), v38, v56);
                        let v66: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v21, &*v1);
                        let v67: Rc<str> = agile_note_listing::kit_agile_notes_filtered_text(&*v22, &*v1, &*v54, &*v55);
                        let v68: Rc<str> = agile_note_listing::kit_agile_notes_filtered_text(&*v24, &*v1, &*v54, &*v55);
                        let v69: Rc<str> = notes_or_archived_15(v67.clone(), v68.clone());
                        show_task_2(v57.clone(), v58.clone(), v59.clone(), v60.clone(), v61.clone(), v62.clone(), v63.clone(), v64.clone(), v65.clone(), v66.clone(), v69.clone(), v3)
                    }
                    US0::US0_ShowJson => {
                        let v71: i32 = (v21.clone().len() as i32);
                        let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_json_record_at_16(v21.clone(), v38, v71);
                        let v81: Rc<str> = agile_planning_fields::kit_agile_planning_json(&*v21, &*v1);
                        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                        let v84: Rc<str> = json_notes_of_17(v22.clone(), v24.clone(), v1.clone());
                        let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                        let v86: Rc<str> = task_json_18(v72.clone(), v73.clone(), v74.clone(), v75.clone(), v76.clone(), v77.clone(), v78.clone(), v79.clone(), v80.clone(), v85.clone(), v3);
                        let v87: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v86.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v87
                    }
                    US0::US0_ShowText => {
                        let v39: i32 = (v21.clone().len() as i32);
                        let (mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_9(v21.clone(), v38, v39);
                        let v49: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v21, &*v1);
                        let v50: Rc<str> = agile_note_listing::kit_agile_notes_text(&*v22, &*v1);
                        let v51: Rc<str> = agile_note_listing::kit_agile_notes_text(&*v24, &*v1);
                        let v52: Rc<str> = notes_joined_8(v50.clone(), v51.clone());
                        show_task_2(v40.clone(), v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone(), v47.clone(), v48.clone(), v49.clone(), v52.clone(), v3)
                    }
                }
            }
        }
    }
}
fn command_line_argument_count_20() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_19() -> i32 {
    let v0: i32 = command_line_argument_count_20();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn agile_show_entry_0() -> i32 {
    let v0: i32 = argument_count_19();
    let mut v1: bool = v0 == 3i32;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = 2i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: US0 = US0::US0_ShowText;
        agile_show_1(v3.clone(), v5.clone(), v6.clone())
    } else {
        let v8: i32 = argument_count_19();
        let v9: bool = v8 == 5i32;
        let v13: Rc<str> = if v9 {
            let v10: i32 = 4i32;
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v11.clone()
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12.clone()
        };
        let v14: i32 = argument_count_19();
        let v15: bool = v14 > 5i32;
        let v37: US3 = if v15 {
            US3::US3_Invalid
        } else {
            let v17: i32 = 3i32;
            let v18: Rc<str> = usize::try_from(v17).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v19: bool = &*v18 == "--json";
            let v21: bool = if v19 {
                let v20: bool = &*v13 == "";
                v20
            } else {
                false
            };
            if v21 {
                let v22: US0 = US0::US0_ShowJson;
                US3::US3_Valid(v22.clone())
            } else {
                let v24: bool = &*v18 == "--last";
                let v26: bool = if v24 {
                    true
                } else {
                    let v25: bool = &*v18 == "--grep";
                    v25
                };
                let v28: bool = if v26 {
                    let v27: bool = &*v13 != "";
                    v27
                } else {
                    false
                };
                if v28 {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(v18.replace(&*v29, &*v30));
                    let v32: US0 = US0::US0_ShowFiltered(v31.clone(), v13.clone());
                    US3::US3_Valid(v32.clone())
                } else {
                    US3::US3_Invalid
                }
            }
        };
        match &v37 {
            US3::US3_Invalid => {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("options are --json, --last <n> or --grep <word>"); } LIT.with(|lit| lit.clone()) };
                let v45: i32 = 2i32;
                let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v46));
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v48, v49)), v44));
                let v52: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v51 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v52
            }
            US3::US3_Valid(v38) => {
                let v38: US0 = v38.clone();
                let v39: i32 = 1i32;
                let v40: Rc<str> = usize::try_from(v39).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v41: i32 = 2i32;
                let v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                agile_show_1(v40.clone(), v42.clone(), v38.clone())
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_show_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_show_run() -> i32 {
    closure0()()
}
