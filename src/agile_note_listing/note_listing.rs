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
    US0_ScanPlain,
    US0_ScanQuoted,
    US0_ScanEscaped,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
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
                let mut v9: i32 = v3.wrapping_add(1i32);
                v2 = v9;
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
            let mut v9: bool = match &v3 {
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
            let mut v5: US0 = US0::US0_ScanPlain;
            let mut v6: i32 = field_end_5(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_6(v0.clone(), v7);
            let mut v9: i32 = v3.wrapping_sub(1i32);
            (v1, v3) = (v8, v9);
            continue;
        }
    }
}
fn concat5_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn unescaped_from_8(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: bool = v3 <= v1;
        if v5 {
            let mut v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v3 as usize).unwrap_or(""));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return concat5_9(v4.clone(), v6.clone(), v7.clone(), v7.clone(), v7.clone());
        } else {
            let mut v9: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v10: bool = v9 == 92i32;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                let v12: i32 = v11.wrapping_add(1i32);
                let mut v13: Rc<str> = std::rc::Rc::<str>::from(v0.get(v2 as usize..v1 as usize).unwrap_or(""));
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
                let v26: Rc<str> = concat5_9(v4.clone(), v13.clone(), v24.clone(), v25.clone(), v25.clone());
                (v1, v2, v4) = (v12, v12, v26.clone());
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
    let mut v5: i32 = 0i32;
    let mut v6: i32 = (v3.clone().len() as i32);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    unescaped_from_8(v3, v4, v5, v6, v7)
}
fn note_at_3(mut v0: Rc<str>, mut v1: i32) -> (Rc<str>, Rc<str>, Rc<str>) {
    let mut v2: i32 = v1.wrapping_add(6i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = 2i32;
    let mut v5: i32 = field_start_4(v0.clone(), v2, v3, v4);
    let mut v6: US0 = US0::US0_ScanPlain;
    let v7: i32 = field_end_5(v0.clone(), v5, v3, v6);
    let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v7 as usize).unwrap_or(""));
    let mut v9: Rc<str> = unquoted_7(v8);
    let v10: i32 = 3i32;
    let v11: i32 = field_start_4(v0.clone(), v2, v3, v10);
    let v12: US0 = US0::US0_ScanPlain;
    let mut v13: i32 = field_end_5(v0.clone(), v11, v3, v12);
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
fn local_offset_minutes_11() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn local_stamp_10(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i64 = local_offset_minutes_11();
    let mut v2: Rc<str> = { let at = &*v0; let offset = v1; let parsed = (|| -> Option<i64> { let hours: i64 = at.get(11..13)?.parse().ok()?; let minutes: i64 = at.get(14..16)?.parse().ok()?; Some(hours * 60 + minutes) })(); std::rc::Rc::<str>::from(match parsed { Some(total) => { let local = (total + offset).rem_euclid(1440); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; format!(" ({:02}:{:02} {}{:02}:{:02})", local / 60, local % 60, sign, offset.abs() / 60, offset.abs() % 60) } None => String::new() }) };
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
            let v11: Rc<str> = local_stamp_10(v8.clone());
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = concat5_9(v12.clone(), v10.clone(), v13.clone(), v13.clone(), v13.clone());
            let v15: Rc<str> = concat5_9(v8.clone(), v11.clone(), v12.clone(), v9.clone(), v14.clone());
            let v16: Rc<str> = concat5_9(v3.clone(), v7.clone(), v15.clone(), v13.clone(), v13.clone());
            (v2, v3) = (v6, v16.clone());
            continue;
        }
    }
}
fn declared_count_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let mut v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v6, v7);
            continue;
        }
    }
}
fn notes_text_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = concat5_9(v2.clone(), v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = 0i32;
    let mut v9: i32 = declared_count_12(v0.clone(), v6, v7, v8);
    let v10: bool = v9 == 0i32;
    if v10 {
        v5.clone()
    } else {
        let v11: Rc<str> = concat5_9(v2, v3, v1, v4, v5.clone());
        let v12: i32 = 0i32;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_9(v13, v14, v15, v5.clone(), v5);
        note_lines_from_1(v0, v11, v12, v16)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_text_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_json_at_15(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"at\":"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = v1.wrapping_add(6i32);
    let v4: i32 = (v0.clone().len() as i32);
    let mut v5: i32 = 2i32;
    let mut v6: i32 = field_start_4(v0.clone(), v3, v4, v5);
    let v7: US0 = US0::US0_ScanPlain;
    let v8: i32 = field_end_5(v0.clone(), v6, v4, v7);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"author\":"); } LIT.with(|lit| lit.clone()) };
    let v11: i32 = 3i32;
    let v12: i32 = field_start_4(v0.clone(), v3, v4, v11);
    let mut v13: US0 = US0::US0_ScanPlain;
    let v14: i32 = field_end_5(v0.clone(), v12, v4, v13);
    let v15: Rc<str> = std::rc::Rc::<str>::from(v0.get(v12 as usize..v14 as usize).unwrap_or(""));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = concat5_9(v2, v9, v10, v15, v16);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"text\":"); } LIT.with(|lit| lit.clone()) };
    let v19: i32 = 4i32;
    let v20: i32 = field_start_4(v0.clone(), v3, v4, v19);
    let v21: US0 = US0::US0_ScanPlain;
    let v22: i32 = field_end_5(v0.clone(), v20, v4, v21);
    let v23: Rc<str> = std::rc::Rc::<str>::from(v0.get(v20 as usize..v22 as usize).unwrap_or(""));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_9(v17, v18, v23, v24, v25)
}
fn note_json_from_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let mut v5: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v6: bool = v5 < 0i32;
        if v6 {
            return v3.clone();
        } else {
            let v7: i32 = v5.wrapping_add(1i32);
            let v8: Rc<str> = note_json_at_15(v0.clone(), v5);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = concat5_9(v3.clone(), v4.clone(), v8.clone(), v9.clone(), v9.clone());
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            (v2, v3, v4) = (v7, v10.clone(), v11.clone());
            continue;
        }
    }
}
fn notes_json_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"notes\":["); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_9(v3, v4, v1, v5, v6.clone());
    let v8: i32 = 0i32;
    let mut v9: Rc<str> = note_json_from_14(v0, v7, v8, v6.clone(), v6.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]"); } LIT.with(|lit| lit.clone()) };
    concat5_9(v2, v9, v10, v6.clone(), v6)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        notes_json_13(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_18(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn method19(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn state_file_text_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v2.borrow_mut().push(v0);
    v2.borrow_mut().push(v1);
    let v3: Rc<Vec<Rc<str>>> = Rc::new(v2.borrow().clone());
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v3).as_ref().clone()));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from((*v4.borrow()).join(&*v5));
    let v7: bool = file_exists_18(v6.clone());
    if v7 {
        let mut v9: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v6);
        let v11: Vec<u8> = v9.unwrap();
        let v12: Vec<u8> = method19(v11);
        let v14: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v12);
        let v16: std::string::String = v14.unwrap();
        let v18: Rc<str> = Rc::<str>::from(String::as_str(&v16));
        v18
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v19
    }
}
fn method21(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method22(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method22(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let mut v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method21(v16.clone(), v17.clone())
    }
}
fn method24(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
fn method25(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
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
fn segment_modules_23(mut v0: Rc<UH0>, mut v1: bool, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let v7: i32 = (v5.clone().len() as i32);
                let v8: i32 = 0i32;
                let mut v9: i32 = method24(v5.clone(), v7, v8);
                let v10: i32 = v7.wrapping_sub(1i32);
                let v11: Rc<str> = string_slice(&v5.clone(), v9 as i64, v10 as i64);
                let v12: i32 = (v11.clone().len() as i32);
                let mut v13: i32 = method25(v11.clone(), v12);
                let v14: Rc<str> = string_slice(&v11.clone(), 0i32 as i64, v13 as i64);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v14.ends_with(&*v15);
                if v16 {
                    let v17: bool = &*v14 == "modules:";
                    (v0, v1) = (v6.clone(), v17);
                    continue;
                } else {
                    let v21: bool = if v1 {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes_"); } LIT.with(|lit| lit.clone()) };
                        let v20: bool = v14.starts_with(&*v19);
                        v20
                    } else {
                        false
                    };
                    if v21 {
                        let v22: bool = true;
                        let v23: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14.clone(), v2.clone()));
                        (v0, v1, v2) = (v6.clone(), v22, v23.clone());
                        continue;
                    } else {
                        v0 = v6.clone();
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                let v3: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                return method21(v2.clone(), v3.clone());
            }
        }
    }
}
fn method26(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH0> = v4.clone();
            let mut v5: Rc<UH0> = method26(v0.clone(), v4, v2.clone());
            let mut v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            v6.borrow_mut().push(v3);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
            v6.borrow_mut().push(v7);
            let v8: Rc<Vec<Rc<str>>> = Rc::new(v6.borrow().clone());
            let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
            let v12: Rc<str> = state_file_text_17(v0, v11);
            let mut v13: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            v13.borrow_mut().push(v14);
            v13.borrow_mut().push(v12);
            let v15: Rc<Vec<Rc<str>>> = Rc::new(v13.borrow().clone());
            let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
            let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v10));
            Rc::new(UH0::UH0_Cons(v17, v5))
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn state_segments_text_16(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = state_file_text_17(v0.clone(), v2);
    let v4: Rc<UH0> = split_list_20(v1, v3);
    let mut v5: bool = false;
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = segment_modules_23(v4, v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v9: Rc<UH0> = method26(v0, v7, v8);
    let v10: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v11: i32 = 0i32;
    let v12: i32 = method27(v10.clone(), v9, v11);
    let mut v13: Rc<Vec<Rc<str>>> = Rc::new(v10.borrow().clone());
    let v14: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v13).as_ref().clone()));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from((*v14.borrow()).join(&*v15));
    v16
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        state_segments_text_16(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure4() -> Rc<dyn Fn((i32)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US2> = Rc::new(move |mut v0: (i32)| -> US2 {
        let mut v1: i32 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_filter_of_28(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: bool = &*v0 == "last";
    if v2 {
        let v3: Option<i32> = (v1).trim().parse::<i32>().ok();
        let v4: Option<i32> = method29(v3.clone());
        let mut v5: Rc<dyn Fn((i32)) -> US2> = closure4();
        let mut v6: Option<US2> = v4.map(|x| v5(x));
        let v8: US2 = US2::US2_None;
        let mut v9: US2 = v8.clone();
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
fn kept_lines_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: US1, mut v4: i32, mut v5: i32, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, i32) {
    loop {
        let v8: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let mut v9: bool = v8 < 0i32;
        if v9 {
            return (v6.clone(), v7);
        } else {
            let (mut v10, mut v11, mut v12): (Rc<str>, Rc<str>, Rc<str>) = note_at_3(v0.clone(), v8);
            let mut v13: i32 = v4.wrapping_add(1i32);
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
                let v42: Rc<str> = local_stamp_10(v10.clone());
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = concat5_9(v43.clone(), v12.clone(), v34.clone(), v34.clone(), v34.clone());
                let v45: Rc<str> = concat5_9(v10.clone(), v42.clone(), v43.clone(), v11.clone(), v44.clone());
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v45));
                let v47: i32 = v7.wrapping_add(1i32);
                (v2, v4, v6, v7) = (v36, v13, v46.clone(), v47);
                continue;
            } else {
                let v50: i32 = v8.wrapping_add(1i32);
                (v2, v4) = (v50, v13);
                continue;
            }
        }
    }
}
fn notes_filtered_text_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: US1) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_9(v3.clone(), v4.clone(), v1.clone(), v5.clone(), v6.clone());
    let v8: i32 = 0i32;
    let mut v9: i32 = 0i32;
    let v10: i32 = declared_count_12(v0.clone(), v7, v8, v9);
    let v11: bool = v10 == 0i32;
    if v11 {
        v6.clone()
    } else {
        let v12: Rc<str> = concat5_9(v3, v4, v1, v5, v6.clone());
        let mut v13: i32 = 0i32;
        let v14: i32 = 0i32;
        let v15: i32 = 0i32;
        let (mut v16, mut v17): (Rc<str>, i32) = kept_lines_31(v0, v12, v13, v2, v14, v10, v6, v15);
        let v18: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes ("); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: Rc<str> = std::rc::Rc::<str>::from((v10).to_string());
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v25)), v16));
        v27
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        let v4: US1 = note_filter_of_28(v2.clone(), v3.clone());
        notes_filtered_text_30(v0.clone(), v1.clone(), v4.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_notes_text(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_notes_json(v0: &str, v1: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_state_segments(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
pub fn kit_agile_notes_filtered_text(v0: &str, v1: &str, v2: &str, v3: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
