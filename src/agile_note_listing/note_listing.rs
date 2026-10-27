#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
#[derive(Clone)]
enum US1 {
    US1_AllNotes,
    US1_NewestNotes(i32),
    US1_NotesMatching(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(i32),
    US2_None,
}
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v6.starts_with(&*v7);
            if v8 {
                return v3;
            } else {
                let v9: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn field_end_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US0) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US0::US0_ScanPlain => {
                    let mut v6: bool = v5 == 44i32;
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
                let v25: US0 = match &v3 {
                    US0::US0_ScanEscaped => {
                        US0::US0_ScanQuoted
                    }
                    US0::US0_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US0::US0_ScanQuoted
                        } else {
                            US0::US0_ScanPlain
                        }
                    }
                    US0::US0_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US0::US0_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US0::US0_ScanPlain
                            } else {
                                US0::US0_ScanQuoted
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
fn skip_spaces_6(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn field_start_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v6, &*v7, &*v7, &*v7].concat());
            return v8.clone();
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
fn unquoted_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    let v3: Rc<str> = std::rc::Rc::<str>::from(v0.get(1i32 as usize..v2 as usize).unwrap_or(""));
    let v4: i32 = 0i32;
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3.clone(), v4, v5, v6, v7.clone())
}
fn note_at_3(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let mut v5: i32 = field_start_4(v0.clone(), v2, v3, v4);
    let mut v6: US0 = US0::US0_ScanPlain;
    let v7: i32 = field_end_5(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let v9: Rc<str> = unquoted_7(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_4(v0.clone(), v2, v3, v10);
    let v12: US0 = US0::US0_ScanPlain;
    let v13: i32 = field_end_5(v0.clone(), v11, v3, v12);
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(v11 as usize..v13 as usize).unwrap_or(""));
    let v15: Rc<str> = unquoted_7(v14);
    let v16: i32 = 4i32;
    let v17: i32 = field_start_4(v0.clone(), v2, v3, v16);
    let v18: US0 = US0::US0_ScanPlain;
    let v19: i32 = field_end_5(v0.clone(), v17, v3, v18);
    let v20: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v19 as usize).unwrap_or(""));
    let v21: Rc<str> = unquoted_7(v20);
    (v9, v15, v21)
}
fn local_offset_minutes_10() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn local_stamp_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: i64 = local_offset_minutes_10();
    let v2: Rc<str> = { let at = &*v0; let offset = v1; let parsed = (|| -> Option<i64> { let hours: i64 = at.get(11..13)?.parse().ok()?; let minutes: i64 = at.get(14..16)?.parse().ok()?; Some(hours * 60 + minutes) })(); std::rc::Rc::<str>::from(match parsed { Some(total) => { let local = (total + offset).rem_euclid(1440); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; format!(" ({:02}:{:02} {}{:02}:{:02})", local / 60, local % 60, sign, offset.abs() / 60, offset.abs() % 60) } None => String::new() }) };
    v2.clone()
}
fn note_lines_from_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let (mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, Rc<str>) = note_at_3(v0.clone(), v4);
            let v11: Rc<str> = local_stamp_9(v8.clone());
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v10, &*v13, &*v13, &*v13].concat());
            let v15: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v11, &*v12, &*v9, &*v14].concat());
            let v16: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v7, &*v15, &*v13, &*v13].concat());
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v16.clone());
            continue;
        }
    }
}
fn declared_count_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v7);
            continue;
        }
    }
}
fn notes_text_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let v9: i32 = declared_count_11(v0.clone(), v6.clone(), v7, v8);
    let v10: bool = v9 == 0i32;
    if v10 {
        v5.clone()
    } else {
        let v11: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
        let v12: i32 = 0i32;
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v14, &*v15, &*v5, &*v5].concat());
        note_lines_from_1(v0.clone(), v11.clone(), v12, v16.clone())
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_text_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_json_at_14(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"at\":"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = v1.wrapping_add(6i32);
    let v4: i32 = (v0.clone().len() as i32);
    let mut v5: i32 = 2i32;
    let mut v6: i32 = field_start_4(v0.clone(), v3, v4, v5);
    let v7: US0 = US0::US0_ScanPlain;
    let v8: i32 = field_end_5(v0.clone(), v6, v4, v7.clone());
    let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"author\":"); } LIT.with(|lit| lit.clone()) };
    let v11: i32 = 3i32;
    let v12: i32 = field_start_4(v0.clone(), v3, v4, v11);
    let v13: US0 = US0::US0_ScanPlain;
    let v14: i32 = field_end_5(v0.clone(), v12, v4, v13.clone());
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v9, &*v10, &*v15, &*v16].concat());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":"); } LIT.with(|lit| lit.clone()) };
    let v19: i32 = 4i32;
    let v20: i32 = field_start_4(v0.clone(), v3, v4, v19);
    let v21: US0 = US0::US0_ScanPlain;
    let v22: i32 = field_end_5(v0.clone(), v20, v4, v21.clone());
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v23, &*v24, &*v25].concat());
    v26.clone()
}
fn note_json_from_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v3.clone();
        } else {
            let v7: i32 = v5.wrapping_add(1i32);
            let v8: Rc<str> = note_json_at_14(v0.clone(), v5);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v8, &*v9, &*v9].concat());
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            (v0, v1, v2, v3, v4) = (v0.clone(), v1.clone(), v7, v10.clone(), v11.clone());
            continue;
        }
    }
}
fn notes_json_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"notes\":["); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v1, &*v5, &*v6].concat());
    let v8: i32 = 0i32;
    let v9: Rc<str> = note_json_from_13(v0.clone(), v7.clone(), v8, v6.clone(), v6.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v9, &*v10, &*v6, &*v6].concat());
    v11.clone()
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_json_12(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure3() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_filter_of_15(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v2: bool = &*v0 == "last";
    if v2 {
        let v3: Option<i32> = (v1).trim().parse::<i32>().ok();
        let v4: Option<i32> = method16(v3.clone());
        let mut v5: Rc<dyn Fn((i32)) -> US2> = closure3();
        let mut v6: Option<US2> = v4.map(|x| v5(x));
        let v8: US2 = US2::US2_None;
        let v9: US2 = v8.clone();
        let v10: US2 = v6.unwrap_or(v9);
        match &v10 {
            US2::US2_None => {
                US1::US1_AllNotes
            }
            US2::US2_Some(v11) => {
                let v11: i32 = *v11;
                US1::US1_NewestNotes(v11)
            }
        }
    } else {
        let v16: bool = &*v0 == "grep";
        if v16 {
            let v17: Rc<str> = Rc::<str>::from(v1.to_lowercase());
            US1::US1_NotesMatching(v17.clone())
        } else {
            US1::US1_AllNotes
        }
    }
}
fn kept_lines_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: US1, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32) {
    loop {
        let v8: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let v9: bool = v8 < 0i32;
        if v9 {
            return (v6.clone(), v7);
        } else {
            let (mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>) = note_at_3(v0.clone(), v8);
            let v13: i32 = v4.wrapping_add(1i32);
            let v25: bool = match &v3 {
                US1::US1_AllNotes => {
                    true
                }
                US1::US1_NewestNotes(v14) => {
                    let v14: i32 = *v14;
                    let v15: i32 = v5.wrapping_sub(v14);
                    let v16: bool = v13 > v15;
                    v16
                }
                US1::US1_NotesMatching(v17) => {
                    let v17: Rc<str> = v17.clone();
                    let v18: Rc<str> = Rc::<str>::from(v12.to_lowercase());
                    let v19: bool = v18.contains(&*v17);
                    if v19 {
                        true
                    } else {
                        let v20: Rc<str> = Rc::<str>::from(v11.to_lowercase());
                        let v21: bool = v20.contains(&*v17);
                        v21
                    }
                }
            };
            if v25 {
                let v26: i32 = v8.wrapping_add(6i32);
                let v27: i32 = (v0.clone().len() as i32);
                let v28: i32 = 1i32;
                let v29: i32 = field_start_4(v0.clone(), v26, v27, v28);
                let v30: US0 = US0::US0_ScanPlain;
                let v31: i32 = field_end_5(v0.clone(), v29, v27, v30.clone());
                let v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v29 as usize..v31 as usize).unwrap_or(""));
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = Rc::<str>::from(v32.replace(&*v33, &*v34));
                let v36: i32 = v8.wrapping_add(1i32);
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n#"); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v37)), v35));
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                let v42: Rc<str> = local_stamp_9(v10.clone());
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v12, &*v34, &*v34, &*v34].concat());
                let v45: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v42, &*v43, &*v11, &*v44].concat());
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v45));
                let v47: i32 = v7.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v36, v3.clone(), v13, v5, v46.clone(), v47);
                continue;
            } else {
                let v50: i32 = v8.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v0.clone(), v1.clone(), v50, v3.clone(), v13, v5, v6.clone(), v7);
                continue;
            }
        }
    }
}
fn notes_filtered_text_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US1) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v1, &*v5, &*v6].concat());
    let v8: i32 = 0i32;
    let v9: i32 = 0i32;
    let v10: i32 = declared_count_11(v0.clone(), v7.clone(), v8, v9);
    let v11: bool = v10 == 0i32;
    if v11 {
        v6.clone()
    } else {
        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v1, &*v5, &*v6].concat());
        let v13: i32 = 0i32;
        let v14: i32 = 0i32;
        let v15: i32 = 0i32;
        let (mut v16, mut v17): (Rc<str>, i32) = kept_lines_18(v0.clone(), v12.clone(), v13, v2.clone(), v14, v10, v6.clone(), v15);
        let v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v25)), v16));
        v27.clone()
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        let v4: US1 = note_filter_of_15(v2.clone(), v3.clone());
        notes_filtered_text_17(v0.clone(), v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_notes_text(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_notes_json(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_notes_filtered_text(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
