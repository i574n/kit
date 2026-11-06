#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
}

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
fn task_summary_3(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>) -> Rc<str> {
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v2));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v3));
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v4));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v5));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("%  "); } LIT.with(|lit| lit.clone()) }));
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v6));
    v18
}
fn show_receipt_4(mut v0: Rc<str>, mut v1: &Rc<str>, mut v2: &Rc<str>, mut v3: &Rc<str>, mut v4: &Rc<str>, mut v5: &Rc<str>, mut v6: &Rc<str>, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: u64) -> Rc<str> {
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile show ok id="); } LIT.with(|lit| lit.clone()) }, v0));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" title_bytes="); } LIT.with(|lit| lit.clone()) }));
    let v12: i32 = (v1.clone().len() as i32);
    let v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v13));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
    let v16: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v17));
    v18
}
fn show_task_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: u64) -> i32 {
    let v12: Rc<str> = task_summary_3(v0.clone(), &v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), &v7, &v8);
    let v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v14: bool = &*v7 == "";
    let v16: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v15
    } else {
        v7.clone()
    };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps: "); } LIT.with(|lit| lit.clone()) }, v16));
    let v18: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v17.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v19: bool = &*v8 == "";
    let v21: Rc<str> = if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        v20
    } else {
        v8.clone()
    };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags: "); } LIT.with(|lit| lit.clone()) }, v21));
    let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("planning: "); } LIT.with(|lit| lit.clone()) }, v9));
    let v25: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v24.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v26: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v1.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v27: bool = &*v10 == "";
    let v29: i32 = if v27 {
        0i32
    } else {
        let v28: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v10.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
        v28
    };
    let v30: Rc<str> = show_receipt_4(v0, &v1, &v2, &v3, &v4, &v5, &v6, &v7, &v8, v11);
    let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v30.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v31
}
fn concat5_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn archived_text_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive/done-notes.spi"); } LIT.with(|lit| lit.clone()) }));
    let v2: bool = std::path::Path::new(&*v1).is_file();
    let v5: US1 = if v2 {
        US1::US1_FileReadable(v1)
    } else {
        US1::US1_FileMissing(v1)
    };
    match v5 {
        US1::US1_FileMissing(v6) => {
            let v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US1::US1_FileReadable(v8) => {
            let v8: Rc<str> = v8;
            let v9: Rc<str> = std::fs::read_to_string(&*v8).unwrap_or_default().into();
            let v10: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v9);
            v10
        }
    }
}
fn state_segments_7(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
    let v2: Rc<str> = agile_note_listing::kit_agile_state_segments(&*v1);
    v2
}
fn notes_joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: bool = &*v1 == "";
        if v3 {
            v0
        } else {
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
            v5
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
                let v25: US2 = match v3 {
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
                (v1, v3) = (v10, v25);
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
            let v6: i32 = field_end_11(v0.clone(), v1, v2, v5);
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
            return v7;
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
                            let v21: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
                            v21
                        }
                    }
                };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v24));
                (v1, v2, v4) = (v11, v11, v25);
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
    let v12: Rc<str> = spiral_json_quote(&*v0);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v11));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v12));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) }, v15));
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = spiral_json_quote(&*v1);
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v18));
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v19));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v22));
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = spiral_json_quote(&*v2);
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v25));
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v26));
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = spiral_json_quote(&*v3);
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v31));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v32));
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v35));
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = spiral_json_quote(&*v4);
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v38));
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v39));
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v42));
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v44));
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v5));
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v47));
    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v51: Rc<str> = spiral_json_quote(&*v6);
    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v50));
    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v51));
    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v54));
    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = spiral_json_quote(&*v7);
    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v57));
    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v58));
    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = spiral_json_quote(&*v8);
    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v63));
    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v64));
    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v67));
    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v9));
    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v69, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
    let v72: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v10);
    let v73: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v71));
    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v74, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) }));
    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v73));
    let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v76));
    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v77));
    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v43));
    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v56));
    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v78));
    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v81, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
    v82
}
fn agile_show_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US0) -> i32 {
    let v3: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_5(v0.clone(), v4, v5.clone(), v5.clone(), v5.clone());
    let v7: bool = std::path::Path::new(&*v6).is_file();
    let v10: US1 = if v7 {
        US1::US1_FileReadable(v6)
    } else {
        US1::US1_FileMissing(v6)
    };
    match v10 {
        US1::US1_FileMissing(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) }, v1));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v12));
            let v16: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v15 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v16
        }
        US1::US1_FileReadable(v17) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = std::fs::read_to_string(&*v17).unwrap_or_default().into();
            let v19: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v18);
            let v20: Rc<str> = archived_text_6(v0.clone());
            let v21: Rc<str> = state_segments_7(v0);
            let v22: Rc<str> = notes_joined_8(v19.clone(), v21);
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) }, v1.clone()));
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) }));
            let v25: i32 = v19.find(&*v24).map(|offset| offset as i32).unwrap_or(-1);
            let v26: bool = v25 < 0i32;
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) }, v1));
                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v27));
                let v31: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v30 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v31
            } else {
                let v32: i32 = v25.wrapping_add(6i32);
                match v2 {
                    US0::US0_ShowFiltered(v48, v49) => {
                        let v48: Rc<str> = v48;
                        let v49: Rc<str> = v49;
                        let v50: i32 = (v19.clone().len() as i32);
                        let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_9(v19.clone(), v32, v50);
                        let v60: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v19, &*v1);
                        let v61: Rc<str> = agile_note_listing::kit_agile_notes_filtered_text(&*v20, &*v1, &*v48, &*v49);
                        let v62: Rc<str> = agile_note_listing::kit_agile_notes_filtered_text(&*v22, &*v1, &*v48, &*v49);
                        let v63: Rc<str> = notes_or_archived_15(v61, v62);
                        show_task_2(v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v63, v3)
                    }
                    US0::US0_ShowJson => {
                        let v65: i32 = (v19.clone().len() as i32);
                        let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_json_record_at_16(v19.clone(), v32, v65);
                        let v75: Rc<str> = agile_planning_fields::kit_agile_planning_json(&*v19, &*v1);
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                        let v77: Rc<str> = json_notes_of_17(v20, v22, v1);
                        let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                        let v79: Rc<str> = task_json_18(v66, v67, v68, v69, v70, v71, v72, v73, v74, v78, v3);
                        let v80: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v79.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v80
                    }
                    US0::US0_ShowText => {
                        let v33: i32 = (v19.clone().len() as i32);
                        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = task_record_at_9(v19.clone(), v32, v33);
                        let v43: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v19, &*v1);
                        let v44: Rc<str> = agile_note_listing::kit_agile_notes_text(&*v20, &*v1);
                        let v45: Rc<str> = agile_note_listing::kit_agile_notes_text(&*v22, &*v1);
                        let v46: Rc<str> = notes_joined_8(v44, v45);
                        show_task_2(v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v46, v3)
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
        agile_show_1(v3, v5, v6)
    } else {
        let v8: i32 = argument_count_19();
        let v9: bool = v8 == 5i32;
        let v13: Rc<str> = if v9 {
            let v10: i32 = 4i32;
            let v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            v11
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v12
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
                US3::US3_Valid(v22)
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
                    let v32: US0 = US0::US0_ShowFiltered(v31, v13);
                    US3::US3_Valid(v32)
                } else {
                    US3::US3_Invalid
                }
            }
        };
        match v37 {
            US3::US3_Invalid => {
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("options are --json, --last <n> or --grep <word>"); } LIT.with(|lit| lit.clone()) };
                let v45: i32 = 2i32;
                let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile show "); } LIT.with(|lit| lit.clone()) }, v46));
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v44));
                let v50: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v49 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v50
            }
            US3::US3_Valid(v38) => {
                let v38: US0 = v38;
                let v39: i32 = 1i32;
                let v40: Rc<str> = usize::try_from(v39).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v41: i32 = 2i32;
                let v42: Rc<str> = usize::try_from(v41).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                agile_show_1(v40, v42, v38)
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
