#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ReadFiles,
    US2_CountLines,
    US2_ReadUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_WholeFile,
    US4_FromTo(u64, u64),
    US4_OneLine(u64),
    US4_LastLines(u64),
}
fn decimal_text_2(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_4(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_6() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let v0: i64 = local_offset_minutes_6();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_3(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_4(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_5();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn lines_receipt_1(mut v0: [u64; 4], mut v1: u64) -> i32 {
    let v2: u64 = 0u64;
    let mut v3: u64 = v0[v2 as usize];
    let mut v4: Rc<str> = decimal_text_2(v3);
    let v5: u64 = 1u64;
    let v6: u64 = v0[v5 as usize];
    let v7: Rc<str> = decimal_text_2(v6);
    let v8: u64 = 2u64;
    let v9: u64 = v0[v8 as usize];
    let v10: Rc<str> = decimal_text_2(v9);
    let v11: u64 = 3u64;
    let v12: u64 = v0[v11 as usize];
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v4));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v7));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v14, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v17)), v10))));
    let v21: bool = v12 == 0u64;
    if v21 {
        let v22: US0 = US0::US0_Succeeded;
        receipt_line_3(v22.clone(), v20.clone(), v1);
        0i32
    } else {
        let v23: US0 = US0::US0_Failed;
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v24));
        let v26: Rc<str> = decimal_text_2(v12);
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        receipt_line_3(v23.clone(), v27.clone(), v1);
        2i32
    }
}
fn read_receipt_7(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let v6: Rc<str> = decimal_text_2(v0);
    let v7: Rc<str> = decimal_text_2(v2);
    if v4 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"files\":["); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v3));
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"lines\":"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v10)), v7));
        let v13: Rc<str> = decimal_text_2(v1);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v13));
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v12, Rc::<str>::from(format!("{}{}", v15, v16))));
        println!("{}", &*v18);
        let v19: bool = v1 == 0u64;
        if v19 {
            0i32
        } else {
            2i32
        }
    } else {
        let v21: bool = v1 == 0u64;
        if v21 {
            let v22: US0 = US0::US0_Succeeded;
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v6));
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v25)), v7));
            receipt_line_3(v22.clone(), v27.clone(), v5);
            0i32
        } else {
            let v28: US0 = US0::US0_Failed;
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v6));
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let v33: Rc<str> = decimal_text_2(v1);
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, Rc::<str>::from(format!("{}{}", v35, v7))));
            receipt_line_3(v28.clone(), v37.clone(), v5);
            2i32
        }
    }
}
fn now_ms_8() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_10() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_11(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_9(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_10();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US1 = if v5 {
        let v6: Rc<str> = command_line_argument_11(v1);
        US1::US1_Some(v6.clone())
    } else {
        US1::US1_None
    };
    match &v9 {
        US1::US1_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US1::US1_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn argument_count_12() -> i32 {
    let v0: i32 = command_line_argument_count_10();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn patterns_from_13(mut v0: i32, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 < v1;
        if v3 {
            let mut v4: i32 = v0.wrapping_add(1i32);
            let v5: bool = &*v2 == "";
            let v11: Rc<str> = if v5 {
                argument_9(v0)
            } else {
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
                let v9: Rc<str> = argument_9(v0);
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                v10.clone()
            };
            (v0, v1, v2) = (v4, v1, v11.clone());
            continue;
        } else {
            return v2.clone();
        }
    }
}
fn expanded_from_14(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).nth(v1 as usize).unwrap_or(""));
        let mut v4: i32 = (v0).split(char::from(10u8)).count() as i32;
        let v5: bool = v4 <= v1;
        if v5 {
            return v2.clone();
        } else {
            let v6: bool = &*v3 == "";
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v7, v2.clone());
                continue;
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**"); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v3.contains(&*v9);
                let v13: Rc<str> = if v10 {
                    let v11: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v3; let at = text.find("**").unwrap_or(0); let base = text[..at].trim_end_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let mask = text[at + 2..].trim_start_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let root = if base.is_empty() { std::path::PathBuf::from(".") } else { std::path::PathBuf::from(base) }; let mut found = Vec::new(); let mut pending = vec![root.clone()]; while let Some(folder) = pending.pop() { for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let name = entry.file_name().to_string_lossy().into_owned(); if path.is_dir() { if name != "target" && name != ".git" && name != "node_modules" { pending.push(path); } } else if name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len() { let relative = path.strip_prefix(&root).map(|rest| rest.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or(name); found.push(if base.is_empty() { relative } else { format!("{base}/{relative}") }); } } } found.sort(); found.join(&char::from(10u8).to_string()) }.as_str());
                    v11.clone()
                } else {
                    let v12: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v3; if !text.contains(char::from(42u8)) { text.to_owned() } else { let split = text.rfind(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (folder_text, separator, mask) = match split { Some(at) => (&text[..at], &text[at..at + 1], &text[at + 1..]), None => (".", "", text) }; let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let mut found = std::fs::read_dir(folder_text).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len()).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found.into_iter().map(|name| if separator.is_empty() { name } else { format!("{folder_text}{separator}{name}") }).collect::<Vec<_>>().join(&char::from(10u8).to_string()) } }.as_str());
                    v12.clone()
                };
                let v14: i32 = v1.wrapping_add(1i32);
                let v15: bool = &*v13 == "";
                let v21: Rc<str> = if v15 {
                    v2.clone()
                } else {
                    let v16: bool = &*v2 == "";
                    if v16 {
                        v13.clone()
                    } else {
                        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v17)), v13));
                        v19.clone()
                    }
                };
                (v0, v1, v2) = (v0.clone(), v14, v21.clone());
                continue;
            }
        }
    }
}
fn lines_counted_15(mut v0: Rc<str>) -> [u64; 4] {
    let v1: [u64; 4] = (|| -> [u64; 4] { let mut rows = Vec::new(); for shown in (&*v0).lines().filter(|line| !line.is_empty()) { rows.push((shown.to_owned(), std::fs::read(shown).map(|bytes| (bytes.len() as u64, String::from_utf8_lossy(&bytes).lines().count() as u64)).map_err(|error| error.to_string()))); } let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.as_ref().map(|counts| counts.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); println!("{:<path_width$}  {:<bytes_width$}  {}", "path", "bytes", "lines"); let mut tally = [0u64; 4]; for (shown, counts) in &rows { tally[0] += 1; match counts { Ok((bytes, lines)) => { tally[1] += bytes; tally[2] += lines; println!("{:<path_width$}  {:<bytes_width$}  {}", shown, bytes, lines); } Err(error) => { tally[3] += 1; println!("{:<path_width$}  {:<bytes_width$}  -  (unreadable: {error})", shown, "-"); } } } tally })();
    v1.clone()
}
fn method18(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_17(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method18(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn max_lines_from_16(mut v0: i32, mut v1: i32) -> u64 {
    loop {
        let v2: i32 = v0.wrapping_add(1i32);
        let mut v3: bool = v2 < v1;
        if v3 {
            let mut v4: Rc<str> = argument_9(v0);
            let v5: bool = &*v4 == "--max-lines";
            if v5 {
                let v6: Rc<str> = argument_9(v2);
                return decimal_u64_17(v6.clone());
            } else {
                (v0, v1) = (v2, v1);
                continue;
            }
        } else {
            return 400u64;
        }
    }
}
fn read_start_19() -> (u64, u64, u64, Rc<str>) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, v0.clone())
}
fn decimal_or_zero_22(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method18(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn read_spec_of_21(mut v0: Rc<str>) -> (Rc<str>, US4) {
    let v1: i64 = { let bytes = v0.as_bytes(); bytes.iter().rposition(|byte| *byte == 58u8).filter(|index| *index > 1 || !(bytes.len() > 1 && bytes[1] == 58u8)).map(|index| index as i64).unwrap_or(-1) };
    let v2: bool = v1 < 0i64;
    let (mut v45, mut v46): (Rc<str>, US4) = if v2 {
        let mut v3: US4 = US4::US4_WholeFile;
        (v0.clone(), v3.clone())
    } else {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[(v1 as usize) + 1..]);
        let v5: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
        let v12: bool = if v5 {
            true
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v4.contains(&*v6);
            if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(v4.replace(&*v6, &*v8));
                let v10: bool = !v9.is_empty() && v9.bytes().all(|byte| byte.is_ascii_digit());
                v10
            } else {
                false
            }
        };
        let (mut v43, mut v44): (Rc<str>, US4) = if v12 {
            let v13: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[..v1 as usize]);
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v15: bool = v4.starts_with(&*v14);
            let v41: US4 = if v15 {
                let v16: i64 = 0i64;
                let v17: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[(v16 as usize) + 1..]);
                let v18: u64 = decimal_or_zero_22(v17.clone());
                let v19: bool = v18 == 0u64;
                if v19 {
                    US4::US4_WholeFile
                } else {
                    US4::US4_LastLines(v18)
                }
            } else {
                let v23: bool = v4.contains(&*v14);
                if v23 {
                    let v24: i64 = v4.find(char::from(45u8)).map(|index| index as i64).unwrap_or(-1);
                    let v25: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[..v24 as usize]);
                    let v26: u64 = decimal_or_zero_22(v25.clone());
                    let v27: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[(v24 as usize) + 1..]);
                    let v28: u64 = decimal_or_zero_22(v27.clone());
                    let v29: bool = v26 == 0u64;
                    let v31: bool = if v29 {
                        true
                    } else {
                        let v30: bool = v28 < v26;
                        v30
                    };
                    if v31 {
                        US4::US4_WholeFile
                    } else {
                        US4::US4_FromTo(v26, v28)
                    }
                } else {
                    let v35: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
                    if v35 {
                        let v36: u64 = decimal_or_zero_22(v4.clone());
                        US4::US4_OneLine(v36)
                    } else {
                        US4::US4_WholeFile
                    }
                }
            };
            (v13.clone(), v41.clone())
        } else {
            let v42: US4 = US4::US4_WholeFile;
            (v0.clone(), v42.clone())
        };
        (v43.clone(), v44.clone())
    };
    (v45.clone(), v46.clone())
}
fn range_bounds_26(mut v0: US4, mut v1: u64) -> (u64, u64) {
    match &v0 {
        US4::US4_FromTo(v2, v3) => {
            let v2: u64 = *v2;
            let mut v3: u64 = *v3;
            (v2, v3)
        }
        US4::US4_LastLines(v5) => {
            let v5: u64 = *v5;
            let v6: bool = v5 >= v1;
            let v9: u64 = if v6 {
                1u64
            } else {
                let v7: u64 = v1.wrapping_sub(v5);
                let v8: u64 = v7.wrapping_add(1u64);
                v8
            };
            (v9, v1)
        }
        US4::US4_OneLine(v4) => {
            let mut v4: u64 = *v4;
            (v4, v4)
        }
        US4::US4_WholeFile => {
            (1u64, v1)
        }
    }
}
fn slice_json_27(mut v0: std::rc::Rc<Result<Vec<String>, String>>, mut v1: u64, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1 <= v2;
        if v4 {
            let v5: Rc<str> = std::rc::Rc::<str>::from((*v0).as_ref().ok().and_then(|lines| lines.get((v1 - 1) as usize)).map(|line| line.as_str()).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v5).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v7: u64 = v1.wrapping_add(1u64);
            let v8: bool = &*v3 == "";
            let v12: Rc<str> = if v8 {
                v6.clone()
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v9)), v6));
                v11.clone()
            };
            (v0, v1, v2, v3) = (v0.clone(), v7, v2, v12.clone());
            continue;
        } else {
            return v3.clone();
        }
    }
}
fn slice_printed_28(mut v0: std::rc::Rc<Result<Vec<String>, String>>, mut v1: u64, mut v2: u64, mut v3: u64) -> () {
    loop {
        let mut v4: bool = v1 <= v2;
        if v4 {
            let v5: Rc<str> = std::rc::Rc::<str>::from((*v0).as_ref().ok().and_then(|lines| lines.get((v1 - 1) as usize)).map(|line| line.as_str()).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from(format!("{:>width$}  {}", v1, &*v5, width = v3 as usize).as_str());
            println!("{}", &*v6);
            let v7: u64 = v1.wrapping_add(1u64);
            (v0, v1, v2, v3) = (v0.clone(), v7, v2, v3);
            continue;
        }
        return ();
    }
}
fn file_slice_25(mut v0: Rc<str>, mut v1: US4, mut v2: u64, mut v3: bool) -> (i64, Rc<str>) {
    let mut v4: std::rc::Rc<Result<Vec<String>, String>> = std::rc::Rc::new(std::fs::read(&*v0).map(|bytes| String::from_utf8_lossy(&bytes).lines().map(str::to_owned).collect::<Vec<_>>()).map_err(|error| error.to_string()));
    let v5: bool = (v4).is_err();
    let (mut v93, mut v94): (i64, Rc<str>) = if v5 {
        let v6: Rc<str> = std::rc::Rc::<str>::from((*v4).as_ref().err().map(|error| error.as_str()).unwrap_or(""));
        let (mut v22, mut v23): (i64, Rc<str>) = if v3 {
            let v7: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v0).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"error\":"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v6).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            (-1i64, v15.clone())
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (unreadable: "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v16)), v6));
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
            println!("{}", &*v20);
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (-1i64, v21.clone())
        };
        (v22, v23.clone())
    } else {
        let v24: u64 = (*v4).as_ref().map(|lines| lines.len() as u64).unwrap_or(0);
        let (mut v25, mut v26): (u64, u64) = range_bounds_26(v1.clone(), v24);
        let v27: u64 = (v25).max(1u64);
        let v28: u64 = (v26).min(v24);
        let v29: u64 = (v27).saturating_add(v2);
        let v30: u64 = (v29).saturating_sub(1u64);
        let v31: u64 = (v28).min(v30);
        let v32: bool = v27 <= v31;
        let v35: u64 = if v32 {
            let v33: u64 = v31.wrapping_sub(v27);
            let v34: u64 = v33.wrapping_add(1u64);
            v34
        } else {
            0u64
        };
        let v36: i64 = v35 as i64;
        let (mut v91, mut v92): (i64, Rc<str>) = if v3 {
            let v37: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v0).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"from\":"); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
            let v42: Rc<str> = std::rc::Rc::<str>::from((v27).to_string().as_str());
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
            let v44: u64 = (v27).saturating_sub(1u64);
            let v45: u64 = (v31).max(v44);
            let v46: Rc<str> = std::rc::Rc::<str>::from((v45).to_string().as_str());
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"to\":"); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v46));
            let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"total\":"); } LIT.with(|lit| lit.clone()) };
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            let v51: Rc<str> = std::rc::Rc::<str>::from((v24).to_string().as_str());
            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v43, Rc::<str>::from(format!("{}{}", v50, v51))));
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"more\":"); } LIT.with(|lit| lit.clone()) };
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
            let v56: u64 = (v28).saturating_sub(v31);
            let v57: Rc<str> = std::rc::Rc::<str>::from((v56).to_string().as_str());
            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v57));
            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"lines\":["); } LIT.with(|lit| lit.clone()) };
            let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v62: Rc<str> = slice_json_27(v4.clone(), v27, v31, v61.clone());
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) };
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v60, Rc::<str>::from(format!("{}{}", v62, v63))));
            (v36, v65.clone())
        } else {
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (lines "); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v66));
            let v68: Rc<str> = std::rc::Rc::<str>::from((v27).to_string().as_str());
            let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
            let v72: u64 = (v27).saturating_sub(1u64);
            let v73: u64 = (v31).max(v72);
            let v74: Rc<str> = std::rc::Rc::<str>::from((v73).to_string().as_str());
            let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
            let v77: Rc<str> = std::rc::Rc::<str>::from((v24).to_string().as_str());
            let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v71, Rc::<str>::from(format!("{}{}", v78, v79))));
            println!("{}", &*v81);
            let v82: u64 = (v31).to_string().len() as u64;
            slice_printed_28(v4.clone(), v27, v31, v82);
            let v83: bool = v31 < v28;
            if v83 {
                let v84: u64 = v28.wrapping_sub(v31);
                let v85: Rc<str> = std::rc::Rc::<str>::from((v84).to_string().as_str());
                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("... "); } LIT.with(|lit| lit.clone()) };
                let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v85));
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" more lines (raise --max-lines or pass a range)"); } LIT.with(|lit| lit.clone()) };
                let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v87, v88));
                println!("{}", &*v89);
                ()
            };
            let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v36, v90.clone())
        };
        (v91, v92.clone())
    };
    (v93, v94.clone())
}
fn slice_tallied_24(mut v0: Rc<str>, mut v1: US4, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let (mut v8, mut v9): (i64, Rc<str>) = file_slice_25(v0.clone(), v1.clone(), v2, v3);
    let v10: u64 = v4.wrapping_add(1u64);
    let v11: bool = &*v7 == "";
    let v15: Rc<str> = if v11 {
        v9.clone()
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v12)), v9));
        v14.clone()
    };
    let v16: bool = v8 < 0i64;
    let (mut v20, mut v21, mut v22, mut v23): (u64, u64, u64, Rc<str>) = if v16 {
        let v17: u64 = v5.wrapping_add(1u64);
        (v10, v17, v6, v15.clone())
    } else {
        let v18: u64 = v8 as u64;
        let v19: u64 = v6.wrapping_add(v18);
        (v10, v5, v19, v15.clone())
    };
    (v20, v21, v22, v23.clone())
}
fn globbed_from_29(mut v0: Rc<str>, mut v1: i32, mut v2: US4, mut v3: u64, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let v9: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).nth(v1 as usize).unwrap_or(""));
    let v10: bool = &*v9 == "";
    let (mut v20, mut v21, mut v22, mut v23): (u64, u64, u64, Rc<str>) = if v10 {
        (v5, v6, v7, v8.clone())
    } else {
        let v11: i32 = v1.wrapping_add(1i32);
        let (mut v12, mut v13, mut v14, mut v15): (u64, u64, u64, Rc<str>) = slice_tallied_24(v9, v2.clone(), v3, v4, v5, v6, v7, v8);
        let (mut v16, mut v17, mut v18, mut v19): (u64, u64, u64, Rc<str>) = globbed_from_29(v0, v11, v2, v3, v4, v12, v13, v14, v15);
        (v16, v17, v18, v19)
    };
    (v20, v21, v22, v23)
}
fn spec_tallied_23(mut v0: Rc<str>, mut v1: US4, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**"); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v0.contains(&*v8);
    let v12: Rc<str> = if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v0; let at = text.find("**").unwrap_or(0); let base = text[..at].trim_end_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let mask = text[at + 2..].trim_start_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let root = if base.is_empty() { std::path::PathBuf::from(".") } else { std::path::PathBuf::from(base) }; let mut found = Vec::new(); let mut pending = vec![root.clone()]; while let Some(folder) = pending.pop() { for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let name = entry.file_name().to_string_lossy().into_owned(); if path.is_dir() { if name != "target" && name != ".git" && name != "node_modules" { pending.push(path); } } else if name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len() { let relative = path.strip_prefix(&root).map(|rest| rest.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or(name); found.push(if base.is_empty() { relative } else { format!("{base}/{relative}") }); } } } found.sort(); found.join(&char::from(10u8).to_string()) }.as_str());
        v10.clone()
    } else {
        let v11: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v0; if !text.contains(char::from(42u8)) { text.to_owned() } else { let split = text.rfind(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (folder_text, separator, mask) = match split { Some(at) => (&text[..at], &text[at..at + 1], &text[at + 1..]), None => (".", "", text) }; let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let mut found = std::fs::read_dir(folder_text).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len()).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found.into_iter().map(|name| if separator.is_empty() { name } else { format!("{folder_text}{separator}{name}") }).collect::<Vec<_>>().join(&char::from(10u8).to_string()) } }.as_str());
        v11.clone()
    };
    let v13: bool = &*v12 == "";
    let (mut v23, mut v24, mut v25, mut v26): (u64, u64, u64, Rc<str>) = if v13 {
        let (mut v14, mut v15, mut v16, mut v17): (u64, u64, u64, Rc<str>) = slice_tallied_24(v0.clone(), v1.clone(), v2, v3, v4, v5, v6, v7.clone());
        (v14, v15, v16, v17.clone())
    } else {
        let v18: i32 = 0i32;
        let (mut v19, mut v20, mut v21, mut v22): (u64, u64, u64, Rc<str>) = globbed_from_29(v12.clone(), v18, v1.clone(), v2, v3, v4, v5, v6, v7.clone());
        (v19, v20, v21, v22.clone())
    };
    (v23, v24, v25, v26.clone())
}
fn read_from_20(mut v0: i32, mut v1: i32, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let v8: bool = v0 < v1;
    let (mut v41, mut v42, mut v43, mut v44): (u64, u64, u64, Rc<str>) = if v8 {
        let v9: Rc<str> = argument_9(v0);
        let v10: bool = &*v9 == "--max-lines";
        let (mut v37, mut v38, mut v39, mut v40): (u64, u64, u64, Rc<str>) = if v10 {
            let v11: i32 = v0.wrapping_add(2i32);
            let (mut v12, mut v13, mut v14, mut v15): (u64, u64, u64, Rc<str>) = read_from_20(v11, v1, v2, v3, v4, v5, v6, v7.clone());
            (v12, v13, v14, v15)
        } else {
            let v16: bool = &*v9 == "--json";
            let (mut v33, mut v34, mut v35, mut v36): (u64, u64, u64, Rc<str>) = if v16 {
                let v17: i32 = v0.wrapping_add(1i32);
                let (mut v18, mut v19, mut v20, mut v21): (u64, u64, u64, Rc<str>) = read_from_20(v17, v1, v2, v3, v4, v5, v6, v7.clone());
                (v18, v19, v20, v21)
            } else {
                let v22: i32 = v0.wrapping_add(1i32);
                let (mut v23, mut v24): (Rc<str>, US4) = read_spec_of_21(v9);
                let (mut v25, mut v26, mut v27, mut v28): (u64, u64, u64, Rc<str>) = spec_tallied_23(v23, v24, v2, v3, v4, v5, v6, v7.clone());
                let (mut v29, mut v30, mut v31, mut v32): (u64, u64, u64, Rc<str>) = read_from_20(v22, v1, v2, v3, v25, v26, v27, v28);
                (v29, v30, v31, v32)
            };
            (v33, v34, v35, v36)
        };
        (v37, v38, v39, v40)
    } else {
        (v4, v5, v6, v7)
    };
    (v41, v42, v43, v44)
}
fn read_run_0() -> i32 {
    let v0: u64 = now_ms_8();
    let v1: i32 = 1i32;
    let v2: Rc<str> = argument_9(v1);
    let mut v3: bool = &*v2 == "read";
    let v9: US2 = if v3 {
        US2::US2_ReadFiles
    } else {
        let v5: bool = &*v2 == "lines";
        if v5 {
            US2::US2_CountLines
        } else {
            US2::US2_ReadUnknown
        }
    };
    match &v9 {
        US2::US2_CountLines => {
            let v32: i32 = argument_count_12();
            let v33: bool = v32 < 3i32;
            if v33 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", &*v36);
                2i32
            } else {
                let v37: i32 = 2i32;
                let v38: i32 = argument_count_12();
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v40: Rc<str> = patterns_from_13(v37, v38, v39.clone());
                let v41: i32 = 0i32;
                let v42: Rc<str> = expanded_from_14(v40.clone(), v41, v39.clone());
                let v43: [u64; 4] = lines_counted_15(v42.clone());
                lines_receipt_1(v43.clone(), v0)
            }
        }
        US2::US2_ReadFiles => {
            let v10: i32 = argument_count_12();
            let v11: bool = v10 < 3i32;
            if v11 {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                eprintln!("{}", &*v14);
                2i32
            } else {
                let v15: i32 = 2i32;
                let v16: i32 = argument_count_12();
                let v17: i32 = 2i32;
                let v18: i32 = argument_count_12();
                let v19: u64 = max_lines_from_16(v17, v18);
                let v20: bool = std::env::args().skip(2).any(|argument| argument == "--json");
                let (mut v21, mut v22, mut v23, mut v24): (u64, u64, u64, Rc<str>) = read_start_19();
                let (mut v25, mut v26, mut v27, mut v28): (u64, u64, u64, Rc<str>) = read_from_20(v15, v16, v19, v20, v21, v22, v23, v24.clone());
                let v29: bool = std::env::args().skip(2).any(|argument| argument == "--json");
                read_receipt_7(v25, v26, v27, v28.clone(), v29, v0)
            }
        }
        US2::US2_ReadUnknown => {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            eprintln!("{}", &*v48);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        read_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_read_run() -> i32 {
    closure0()()
}
