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
    US0_AllStatuses,
    US0_OnlyStatus(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_TableFormat,
    US1_JsonFormat,
}
#[derive(Clone)]
enum US2 {
    US2_SortPriority,
    US2_SortDifficulty,
    US2_SortProgress,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoRows,
    UH0_RowThen(i32, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_ScanPlain,
    US4_ScanQuoted,
    US4_ScanEscaped,
}
#[derive(Clone)]
enum US5 {
    US5_FileReadable(Rc<str>),
    US5_FileMissing(Rc<str>),
}
fn listing_header_3(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_JsonFormat => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"tasks\":["); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        }
        US1::US1_TableFormat => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ID"); } LIT.with(|lit| lit.clone()) };
            let v2: Rc<str> = { let text = v1; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" P  D     %  "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("STATUS"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { let text = v5; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIND"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { let text = v8; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v9));
            let v11: Rc<str> = agile_planning_fields::kit_agile_planning_header();
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TITLE"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v10, Rc::<str>::from(format!("{}{}", v11, v12))));
            v14.clone()
        }
    }
}
fn json_line_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: Rc<str>, mut v11: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v12: bool) -> Rc<str> {
    let v15: Rc<str> = if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        v14.clone()
    };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("id"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v0)), v19));
    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"id"); } LIT.with(|lit| lit.clone()) };
    let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\":"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v17, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v22, v23)), v21))));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("title"); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v1)), v19));
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v28, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v29)), v23)), v31))));
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind"); } LIT.with(|lit| lit.clone()) };
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v2)), v19));
    let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v27, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v36)), v23)), v38)))), v27));
    let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v3)), v19));
    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v43, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v44)), v23)), v46)))), v27));
    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v4)), v19));
    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v52)), v23)), v54)), v27));
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v58, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v59)), v23)), v5)))), v27));
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
    let v67: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v6)), v19));
    let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v64, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v65)), v23)), v67))));
    let v72: u32 = v11.get(&*v0).map(|entry| entry.0).unwrap_or(0);
    let v73: Rc<str> = std::rc::Rc::<str>::from((v72).to_string());
    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("deps"); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v7)), v19));
    let v80: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v74)), v23)), v76)), v27));
    let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tags"); } LIT.with(|lit| lit.clone()) };
    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v8)), v19));
    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v80, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v81)), v23)), v83)))), v27));
    let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v89)), v23)), v73)), v27));
    let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("last_note"); } LIT.with(|lit| lit.clone()) };
    let v95: Rc<str> = std::rc::Rc::<str>::from(v11.get(&*v0).map(|entry| entry.1.as_str()).unwrap_or(""));
    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v95)), v19));
    let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v88, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v93, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v94)), v23)), v97)))), v27))));
    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v51)), v71)), v27));
    let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v103, v10));
    let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v106, Rc::<str>::from(format!("{}{}", v107, v108))));
    v110.clone()
}
fn table_line_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32, mut v10: Rc<str>) -> Rc<str> {
    let v11: Rc<str> = { let text = v0; let filler = (26i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v12)), v3)), v12)), v4));
    let v17: Rc<str> = { let text = v5; let filler = (4i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(" ".repeat(filler) + &*text) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = { let text = v6; let filler = (8i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let mut v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let mut v23: Rc<str> = { let text = v2; let filler = (9i32 as usize).saturating_sub(text.chars().count()); std::rc::Rc::<str>::from(text.to_string() + &" ".repeat(filler)) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, Rc::<str>::from(format!("{}{}", v22, v23)))), v10));
    let v27: Rc<str> = { let text = v1; if text.chars().count() <= 96i32 as usize { text } else { std::rc::Rc::<str>::from(text.chars().take((96i32 as usize).saturating_sub(3)).collect::<String>() + "...") } };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    v28.clone()
}
fn method9(mut v0: Option<Rc<str>>) -> Option<Rc<str>> {
    v0
}
fn closure1() -> Rc<dyn Fn((Rc<str>)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((Rc<str>)) -> US3> = Rc::new(move |mut v0: (Rc<str>)| -> US3 {
        let v1: Rc<str> = (v0);
        US3::US3_Some(v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn row_line_6(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v6: Rc<str>, mut v7: US2, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: i32, mut v19: bool) -> Rc<str> {
    let v20: Option<Rc<str>> = v5.get(&v9).cloned();
    let v21: Option<Rc<str>> = method9(v20.clone());
    let mut v22: Rc<dyn Fn((Rc<str>)) -> US3> = closure1();
    let mut v23: Option<US3> = v21.map(|x| v22(x));
    let mut v25: US3 = US3::US3_None;
    let v26: US3 = v25.clone();
    let v27: US3 = v23.unwrap_or(v26);
    let v31: Rc<str> = match &v27 {
        US3::US3_None => {
            let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v29.clone()
        }
        US3::US3_Some(v28) => {
            let v28: Rc<str> = v28.clone();
            v28.clone()
        }
    };
    match &v3 {
        US1::US1_JsonFormat => {
            let v34: Rc<str> = agile_planning_fields::kit_agile_planning_json_near(&*v31, &*v9, 0i32);
            json_line_7(v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18, v34.clone(), v4.clone(), v19)
        }
        US1::US1_TableFormat => {
            let v32: Rc<str> = agile_planning_fields::kit_agile_planning_columns_near(&*v31, &*v9, 0i32);
            table_line_8(v9.clone(), v10.clone(), v11.clone(), v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18, v32.clone())
        }
    }
}
fn rows_printed_5(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v7: Rc<str>, mut v8: US2, mut v9: Rc<str>, mut v10: Rc<UH0>, mut v11: i32) -> i32 {
    loop {
        match &*v10 {
            UH0::UH0_NoRows => {
                return v11;
            }
            UH0::UH0_RowThen(v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
                let v12: i32 = *v12;
                let mut v13: Rc<str> = v13.clone();
                let v14: Rc<str> = v14.clone();
                let v15: Rc<str> = v15.clone();
                let v16: Rc<str> = v16.clone();
                let v17: Rc<str> = v17.clone();
                let v18: Rc<str> = v18.clone();
                let v19: Rc<str> = v19.clone();
                let v20: Rc<str> = v20.clone();
                let v21: Rc<str> = v21.clone();
                let mut v22: i32 = *v22;
                let mut v23: Rc<UH0> = v23.clone();
                let v24: bool = v11 == 0i32;
                let mut v25: Rc<str> = row_line_6(v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22, v24);
                let v26: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v25.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                let v27: i32 = v11.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v23.clone(), v27);
                continue;
            }
        }
    }
}
fn field_end_13(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US4) -> i32 {
    loop {
        let mut v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US4::US4_ScanPlain => {
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
                let mut v25: US4 = match &v3 {
                    US4::US4_ScanEscaped => {
                        US4::US4_ScanQuoted
                    }
                    US4::US4_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US4::US4_ScanQuoted
                        } else {
                            US4::US4_ScanPlain
                        }
                    }
                    US4::US4_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US4::US4_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US4::US4_ScanPlain
                            } else {
                                US4::US4_ScanQuoted
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
fn skip_spaces_14(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let mut v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn field_start_12(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US4 = US4::US4_ScanPlain;
            let v6: i32 = field_end_13(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_14(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn task_row_at_11(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32) {
    let v3: i32 = 0i32;
    let mut v4: i32 = field_start_12(v0.clone(), v1, v2, v3);
    let mut v5: US4 = US4::US4_ScanPlain;
    let v6: i32 = field_end_13(v0.clone(), v4, v2, v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(v4 as usize..v6 as usize).unwrap_or(""));
    let v8: i32 = (v7.clone().len() as i32);
    let v9: i32 = v8.wrapping_sub(1i32);
    let v10: Rc<str> = std::rc::Rc::<str>::from(v7.get(1i32 as usize..v9 as usize).unwrap_or(""));
    let v11: i32 = 1i32;
    let v12: i32 = field_start_12(v0.clone(), v1, v2, v11);
    let mut v13: US4 = US4::US4_ScanPlain;
    let v14: i32 = field_end_13(v0.clone(), v12, v2, v13.clone());
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: i32 = (v15.clone().len() as i32);
    let v17: i32 = v16.wrapping_sub(1i32);
    let v18: Rc<str> = std::rc::Rc::<str>::from(v15.get(1i32 as usize..v17 as usize).unwrap_or(""));
    let v19: i32 = 2i32;
    let v20: i32 = field_start_12(v0.clone(), v1, v2, v19);
    let v21: US4 = US4::US4_ScanPlain;
    let mut v22: i32 = field_end_13(v0.clone(), v20, v2, v21.clone());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: i32 = 3i32;
    let mut v25: i32 = field_start_12(v0.clone(), v1, v2, v24);
    let v26: US4 = US4::US4_ScanPlain;
    let v27: i32 = field_end_13(v0.clone(), v25, v2, v26.clone());
    let v28: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v27 as usize).unwrap_or(""));
    let mut v29: i32 = 4i32;
    let mut v30: i32 = field_start_12(v0.clone(), v1, v2, v29);
    let v31: US4 = US4::US4_ScanPlain;
    let v32: i32 = field_end_13(v0.clone(), v30, v2, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v30 as usize..v32 as usize).unwrap_or(""));
    let v34: i32 = 5i32;
    let v35: i32 = field_start_12(v0.clone(), v1, v2, v34);
    let v36: US4 = US4::US4_ScanPlain;
    let v37: i32 = field_end_13(v0.clone(), v35, v2, v36.clone());
    let v38: Rc<str> = std::rc::Rc::<str>::from(v0.get(v35 as usize..v37 as usize).unwrap_or(""));
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(v38.replace(&*v39, &*v40));
    let v42: i32 = 6i32;
    let mut v43: i32 = field_start_12(v0.clone(), v1, v2, v42);
    let mut v44: US4 = US4::US4_ScanPlain;
    let v45: i32 = field_end_13(v0.clone(), v43, v2, v44.clone());
    let v46: Rc<str> = std::rc::Rc::<str>::from(v0.get(v43 as usize..v45 as usize).unwrap_or(""));
    let v47: i32 = 7i32;
    let v48: i32 = field_start_12(v0.clone(), v1, v2, v47);
    let v49: US4 = US4::US4_ScanPlain;
    let v50: i32 = field_end_13(v0.clone(), v48, v2, v49.clone());
    let v51: Rc<str> = std::rc::Rc::<str>::from(v0.get(v48 as usize..v50 as usize).unwrap_or(""));
    let v52: i32 = (v51.clone().len() as i32);
    let v53: i32 = v52.wrapping_sub(1i32);
    let v54: Rc<str> = std::rc::Rc::<str>::from(v51.get(1i32 as usize..v53 as usize).unwrap_or(""));
    let v55: i32 = 8i32;
    let v56: i32 = field_start_12(v0.clone(), v1, v2, v55);
    let v57: US4 = US4::US4_ScanPlain;
    let v58: i32 = field_end_13(v0.clone(), v56, v2, v57.clone());
    let v59: Rc<str> = std::rc::Rc::<str>::from(v0.get(v56 as usize..v58 as usize).unwrap_or(""));
    let v60: i32 = (v59.clone().len() as i32);
    let v61: i32 = v60.wrapping_sub(1i32);
    let v62: Rc<str> = std::rc::Rc::<str>::from(v59.get(1i32 as usize..v61 as usize).unwrap_or(""));
    (v10.clone(), v18.clone(), v23.clone(), v28.clone(), v33.clone(), v41.clone(), v46.clone(), v54.clone(), v62.clone(), v1)
}
fn status_selected_16(mut v0: US0, mut v1: Rc<str>) -> bool {
    match &v0 {
        US0::US0_AllStatuses => {
            true
        }
        US0::US0_OnlyStatus(v2) => {
            let v2: Rc<str> = v2.clone();
            let v3: bool = &*v2 == &*v1;
            v3
        }
    }
}
fn row_selected_15(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v6: Rc<str>, mut v7: US2, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>, mut v15: Rc<str>, mut v16: Rc<str>, mut v17: Rc<str>, mut v18: i32) -> bool {
    let v19: bool = status_selected_16(v0.clone(), v15.clone());
    let v24: bool = if v19 {
        let v20: bool = (v16).split(char::from(44u8)).map(|dep| dep.trim()).filter(|dep| (v8).lines().any(|line| line == *dep)).next().is_none();
        let v21: bool = &*v6 == "";
        if v21 {
            true
        } else {
            let mut v22: bool = agile_task_query::kit_query_passes(&*v6, &*v9, &*v10, &*v17, &*v15, v20);
            v22
        }
    } else {
        false
    };
    if v24 {
        let mut v25: bool = &*v1 == "all";
        if v25 {
            let v26: bool = &*v2 == "all";
            if v26 {
                true
            } else {
                let v27: Option<Rc<str>> = v5.get(&v9).cloned();
                let v28: Option<Rc<str>> = method9(v27.clone());
                let mut v29: Rc<dyn Fn((Rc<str>)) -> US3> = closure1();
                let mut v30: Option<US3> = v28.map(|x| v29(x));
                let v32: US3 = US3::US3_None;
                let v33: US3 = v32.clone();
                let v34: US3 = v30.unwrap_or(v33);
                let v38: Rc<str> = match &v34 {
                    US3::US3_None => {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v36.clone()
                    }
                    US3::US3_Some(v35) => {
                        let v35: Rc<str> = v35.clone();
                        v35.clone()
                    }
                };
                let v39: bool = agile_planning_filters::kit_agile_planning_selected_near(&*v38, &*v9, 0i32, &*v1, &*v2);
                v39
            }
        } else {
            let v41: Option<Rc<str>> = v5.get(&v9).cloned();
            let v42: Option<Rc<str>> = method9(v41.clone());
            let mut v43: Rc<dyn Fn((Rc<str>)) -> US3> = closure1();
            let mut v44: Option<US3> = v42.map(|x| v43(x));
            let v46: US3 = US3::US3_None;
            let v47: US3 = v46.clone();
            let v48: US3 = v44.unwrap_or(v47);
            let v52: Rc<str> = match &v48 {
                US3::US3_None => {
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v50.clone()
                }
                US3::US3_Some(v49) => {
                    let v49: Rc<str> = v49.clone();
                    v49.clone()
                }
            };
            let v53: bool = agile_planning_filters::kit_agile_planning_selected_near(&*v52, &*v9, 0i32, &*v1, &*v2);
            v53
        }
    } else {
        false
    }
}
fn row_inserted_17(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32, mut v11: Rc<UH0>) -> Rc<UH0> {
    match &*v11 {
        UH0::UH0_NoRows => {
            let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoRows); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_RowThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v12.clone()))
        }
        UH0::UH0_RowThen(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) => {
            let v14: i32 = *v14;
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = v16.clone();
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = v20.clone();
            let v21: Rc<str> = v21.clone();
            let mut v22: Rc<str> = v22.clone();
            let mut v23: Rc<str> = v23.clone();
            let v24: i32 = *v24;
            let mut v25: Rc<UH0> = v25.clone();
            let v26: bool = v0 < v14;
            if v26 {
                Rc::new(UH0::UH0_RowThen(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v11.clone()))
            } else {
                let v28: Rc<UH0> = row_inserted_17(v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10, v25.clone());
                Rc::new(UH0::UH0_RowThen(v14, v15.clone(), v16.clone(), v17.clone(), v18.clone(), v19.clone(), v20.clone(), v21.clone(), v22.clone(), v23.clone(), v24, v28.clone()))
            }
        }
    }
}
fn rows_from_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: US1, mut v7: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v8: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v9: Rc<str>, mut v10: US2, mut v11: Rc<str>, mut v12: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v14: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v13)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v15: bool = v14 < 0i32;
        if v15 {
            return v12.clone();
        } else {
            let v16: i32 = v14.wrapping_add(6i32);
            let v17: i32 = v0.get(..v14 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v14 as usize).unwrap_or(""));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v20: bool = v18.starts_with(&*v19);
            if v20 {
                let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, i32) = task_row_at_11(v0.clone(), v16, v2);
                let v31: bool = row_selected_15(v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30);
                if v31 {
                    let v32: i32 = (v24).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
                    let v42: i32 = match &v10 {
                        US2::US2_SortDifficulty => {
                            let v33: i32 = (v25).get(1..).and_then(|digits| digits.parse::<i32>().ok()).unwrap_or(9);
                            let v34: i32 = v33.wrapping_mul(10i32);
                            let v35: i32 = v34.wrapping_add(v32);
                            v35
                        }
                        US2::US2_SortPriority => {
                            v32
                        }
                        US2::US2_SortProgress => {
                            let v36: i32 = (v26).trim().parse::<i32>().unwrap_or(0).clamp(0, 100);
                            let v37: i32 = 100i32.wrapping_sub(v36);
                            let v38: i32 = v37.wrapping_mul(10i32);
                            let v39: i32 = v38.wrapping_add(v32);
                            v39
                        }
                    };
                    let mut v43: Rc<UH0> = row_inserted_17(v42, v21.clone(), v22.clone(), v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone(), v29.clone(), v30, v12.clone());
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v16, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v43.clone());
                    continue;
                } else {
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v16, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone());
                    continue;
                }
            } else {
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = (v0.clone(), v16, v2, v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v11.clone(), v12.clone());
                continue;
            }
        }
    }
}
fn print_rows_4(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v8: Rc<str>, mut v9: US2, mut v10: Rc<str>) -> i32 {
    let v11: i32 = 0i32;
    let v12: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoRows); } CASE.with(|case| case.clone()) };
    let mut v13: Rc<UH0> = rows_from_10(v0.clone(), v11, v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v12.clone());
    let v14: i32 = 0i32;
    rows_printed_5(v0.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v13.clone(), v14)
}
fn query_text_19(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US1, mut v4: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v6: Rc<str>, mut v7: US2, mut v8: Rc<str>) -> Rc<str> {
    let v16: Rc<str> = match &v7 {
        US2::US2_SortPriority => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        _ => {
            let mut v13: Rc<str> = match &v7 {
                US2::US2_SortDifficulty => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty"); } LIT.with(|lit| lit.clone()) };
                    v10.clone()
                }
                US2::US2_SortProgress => {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("progress"); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                }
                _ => unreachable!(),
            };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sort="); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
            v15.clone()
        }
    };
    let v17: bool = &*v6 == "";
    let v20: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v18.clone()
    } else {
        let v19: Rc<str> = agile_task_query::kit_query_shown(&*v6);
        v19.clone()
    };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
    v21.clone()
}
fn method20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
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
fn method21(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
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
fn filter_text_22(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_AllStatuses => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US0::US0_OnlyStatus(v2) => {
            let v2: Rc<str> = v2.clone();
            v2.clone()
        }
    }
}
fn json_footer_18(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v8: Rc<str>, mut v9: US2, mut v10: Rc<str>, mut v11: i32, mut v12: Rc<str>) -> Rc<str> {
    let mut v13: Rc<str> = query_text_19(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone());
    let v14: bool = &*v13 == "";
    let mut v29: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v15.clone()
    } else {
        let v16: i32 = (v13.clone().len() as i32);
        let v17: i32 = 0i32;
        let v18: i32 = method20(v13.clone(), v16, v17);
        let v19: i32 = v16.wrapping_sub(1i32);
        let v20: Rc<str> = string_slice(&v13.clone(), v18 as i64, v19 as i64);
        let v21: i32 = (v20.clone().len() as i32);
        let mut v22: i32 = method21(v20.clone(), v21);
        let mut v23: Rc<str> = string_slice(&v20.clone(), 0i32 as i64, v22 as i64);
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v23)), v24));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"query\":"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v26));
        v28.clone()
    };
    let mut v30: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"shown\":"); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v30));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"filter\":"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
    let v35: Rc<str> = filter_text_22(v2.clone());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v34, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v35)), v36))));
    let v40: bool = &*v3 == "all";
    let v57: Rc<str> = if v40 {
        let v41: bool = &*v4 == "all";
        if v41 {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v42.clone()
        } else {
            let mut v44: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v4)), v36));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
            v46.clone()
        }
    } else {
        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v3)), v36));
        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"load\":"); } LIT.with(|lit| lit.clone()) };
        let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v49));
        let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"assignee\":"); } LIT.with(|lit| lit.clone()) };
        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v53, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v4)), v36))));
        v56.clone()
    };
    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v39, Rc::<str>::from(format!("{}{}", v57, v29))));
    let v60: Rc<str> = agile_task_census::kit_agile_status_counts_json(&v0);
    let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v59, v60));
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"elapsed_ms\":"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v61, v62)), v12));
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
    v66.clone()
}
fn table_footer_23(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v8: Rc<str>, mut v9: US2, mut v10: Rc<str>, mut v11: i32, mut v12: Rc<str>) -> Rc<str> {
    let mut v13: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile list ok shown="); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" filter="); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = filter_text_22(v2.clone());
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: bool = &*v3 == "all";
    let v31: Rc<str> = if v20 {
        let v21: bool = &*v4 == "all";
        if v21 {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v22.clone()
        } else {
            let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v4));
            v24.clone()
        }
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load="); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v3));
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee="); } LIT.with(|lit| lit.clone()) };
        let mut v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v27, v28)), v4));
        v30.clone()
    };
    let v32: Rc<str> = query_text_19(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone());
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v31, v32))));
    let v35: Rc<str> = agile_task_census::kit_agile_status_counts(&v0);
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v12));
    v39.clone()
}
fn print_listing_2(mut v0: Rc<str>, mut v1: i32, mut v2: US0, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: US1, mut v6: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v7: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v8: Rc<str>, mut v9: US2, mut v10: Rc<str>, mut v11: u64) -> i32 {
    let v12: Rc<str> = listing_header_3(v5.clone());
    let mut v13: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v12.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    let v14: i32 = print_rows_4(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone());
    let v15: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v11);
    let v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
    match &v5 {
        US1::US1_JsonFormat => {
            let v19: Rc<str> = json_footer_18(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v14, v16.clone());
            let v20: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v19.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v20
        }
        US1::US1_TableFormat => {
            let v17: Rc<str> = table_footer_23(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone(), v8.clone(), v9.clone(), v10.clone(), v14, v16.clone());
            let v18: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v17.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v18
        }
    }
}
fn note_index_of_24(mut v0: Rc<str>) -> std::rc::Rc<std::collections::HashMap<String, (u32, String)>> {
    let v1: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(v0.lines().filter(|line| line.starts_with("inl note_of_")).filter_map(|line| { let body = line.split_once("= Note (")?.1; let mut fields = body.splitn(4, ", "); let id = fields.next()?.trim_matches(char::from(34u8)).to_owned(); let _seq = fields.next()?; let at = fields.next()?.trim_matches(char::from(34u8)).to_owned(); Some((id, at)) }).fold(std::collections::HashMap::new(), |mut index, (id, at)| { let entry = index.entry(id).or_insert((0u32, String::new())); entry.0 += 1; entry.1 = at; index }));
    v1.clone()
}
fn method28(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b'"';
            if v5 {
                let v6: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v6);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn planning_id_27(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("= Planning ("); } LIT.with(|lit| lit.clone()) };
    let v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method28(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 < 0i32;
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9.clone()
    } else {
        let v10: i32 = v7.wrapping_add(12i32);
        let v11: i32 = (v0.clone().len() as i32);
        let v12: i32 = v11.wrapping_sub(v10);
        let mut v13: usize = ((v10) as usize);
        let v14: i32 = v10.wrapping_add(v12);
        let v15: usize = ((v14) as usize);
        let v16: Rc<str> = Rc::<str>::from(&(v0)[v13..v15]);
        let v17: i32 = (v16.clone().len() as i32);
        let v18: i32 = 0i32;
        let v19: i32 = method29(v16.clone(), v17, v18);
        let v20: i32 = v17.wrapping_sub(1i32);
        let v21: Rc<str> = string_slice(&v16.clone(), v19 as i64, v20 as i64);
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let mut v23: Option<usize> = v21.find(&*v22);
        let v24: Option<usize> = method28(v23.clone());
        let mut v25: Option<i32> = v24.map(|x| v4(x));
        let v27: i32 = v25.unwrap_or(-1i32);
        let v28: bool = v27 < 0i32;
        if v28 {
            v21.clone()
        } else {
            let mut v29: usize = ((0i32) as usize);
            let mut v30: usize = ((v27) as usize);
            let v31: Rc<str> = Rc::<str>::from(&(v21)[v29..v30]);
            v31.clone()
        }
    }
}
fn planning_entries_26(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v1 as usize].clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl planning_of_"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v5.starts_with(&*v6);
            let v10: Rc<str> = if v7 {
                planning_id_27(v5.clone())
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            };
            let v11: i32 = v1.wrapping_add(1i32);
            let v12: bool = &*v10 == "";
            let v19: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = if v12 {
                v2.clone()
            } else {
                let mut v13: std::collections::BTreeMap<Rc<str>, Rc<str>> = std::rc::Rc::unwrap_or_clone(v2);
                let v15: bool = true; // 1;
                let mut v13 = v13;
                let v17: Option<Rc<str>> = v13.insert(v10, v5);
                let v18: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = std::rc::Rc::new(v13);
                v18.clone()
            };
            (v0, v1, v2) = (v0.clone(), v11, v19.clone());
            continue;
        }
    }
}
fn planning_index_of_25(mut v0: Rc<str>) -> std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> {
    let v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v2: i32 = 0i32;
    let mut v4: std::collections::BTreeMap<Rc<str>, Rc<str>> = std::collections::BTreeMap::new();
    let mut v5: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = std::rc::Rc::new(v4);
    planning_entries_26(v1, v2, v5)
}
fn agile_list_1(mut v0: Rc<str>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: US1, mut v5: std::rc::Rc<std::collections::HashMap<String, (u32, String)>>, mut v6: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>>, mut v7: Rc<str>, mut v8: US2, mut v9: Rc<str>) -> i32 {
    let v10: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v13: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v11, &*v12, &*v12, &*v12].concat());
    let v14: bool = std::path::Path::new(&*v13).is_file();
    let v17: US5 = if v14 {
        US5::US5_FileReadable(v13.clone())
    } else {
        US5::US5_FileMissing(v13.clone())
    };
    match &v17 {
        US5::US5_FileMissing(v18) => {
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let mut v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v22
        }
        US5::US5_FileReadable(v23) => {
            let mut v23: Rc<str> = v23.clone();
            let v24: Rc<str> = std::fs::read_to_string(&*v23).unwrap_or_default().into();
            let mut v25: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v24);
            let v26: i32 = (v25.clone().len() as i32);
            let v27: i32 = agile_task_census::kit_agile_first_malformed_task(&*v25);
            let v28: bool = v27 < 0i32;
            if v28 {
                let mut v29: bool = &*v7 == "";
                let v31: bool = if v29 {
                    false
                } else {
                    let mut v30: bool = agile_task_query::kit_query_reads_deps(&*v7);
                    v30
                };
                let v33: Rc<str> = if v31 {
                    let v32: Rc<str> = agile_task_census::kit_agile_unsettled_ids(&v25);
                    v32.clone()
                } else {
                    v12.clone()
                };
                let v34: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = note_index_of_24(v25.clone());
                let v35: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = planning_index_of_25(v25.clone());
                print_listing_2(v25.clone(), v26, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v34.clone(), v35.clone(), v7.clone(), v8.clone(), v33.clone(), v10)
            } else {
                let v37: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields at byte "); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                let v42: Rc<str> = std::rc::Rc::<str>::from((11i32).to_string());
                let mut v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
                let mut v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" fields)"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v45));
                let v48: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v47 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v48
            }
        }
    }
}
fn agile_list_entry_0() -> i32 {
    let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7): (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<str>) = kit_agile_list_args::kit_agile_list_request();
    if v0 {
        let v8: bool = &*v2 == "all";
        let v11: US0 = if v8 {
            US0::US0_AllStatuses
        } else {
            US0::US0_OnlyStatus(v2.clone())
        };
        let v14: US1 = if v5 {
            US1::US1_JsonFormat
        } else {
            US1::US1_TableFormat
        };
        let v15: std::rc::Rc<std::collections::HashMap<String, (u32, String)>> = std::rc::Rc::new(std::collections::HashMap::new());
        let v17: std::collections::BTreeMap<Rc<str>, Rc<str>> = std::collections::BTreeMap::new();
        let v18: std::rc::Rc<std::collections::BTreeMap<Rc<str>, Rc<str>>> = std::rc::Rc::new(v17);
        let v19: bool = &*v7 == "difficulty";
        let mut v25: US2 = if v19 {
            US2::US2_SortDifficulty
        } else {
            let v21: bool = &*v7 == "progress";
            if v21 {
                US2::US2_SortProgress
            } else {
                US2::US2_SortPriority
            }
        };
        let v26: i32 = 1i32;
        let v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        agile_list_1(v27.clone(), v11.clone(), v3.clone(), v4.clone(), v14.clone(), v15.clone(), v18.clone(), v6.clone(), v25.clone(), v28.clone())
    } else {
        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile list rejected: "); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v1));
        let v32: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v31 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        agile_list_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_list_run() -> i32 {
    closure0()()
}
