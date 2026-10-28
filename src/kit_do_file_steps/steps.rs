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
fn file_text_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default());
    v2
}
fn uses_crlf_2(mut v0: Rc<str>) -> bool {
    let v1: bool = (v0).contains(&format!("{}{}", char::from(13u8), char::from(10u8)));
    v1
}
fn crlf_3() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(format!("{}{}", char::from(13u8), char::from(10u8)));
    v0
}
fn local_offset_minutes_4() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_filled_5(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let v2: Rc<str> = { let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64).unwrap_or(0) + v1 * 60; let (days, day_seconds) = (seconds.div_euclid(86400), seconds.rem_euclid(86400)); let shifted = days + 719468; let era = shifted.div_euclid(146097); let of_era = shifted - era * 146097; let year_of_era = (of_era - of_era / 1460 + of_era / 36524 - of_era / 146096) / 365; let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100); let month_index = (5 * of_year + 2) / 153; let day = of_year - (153 * month_index + 2) / 5 + 1; let month = if month_index < 10 { month_index + 3 } else { month_index - 9 }; let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 }; let formatted = |format: &str| format.replace("yyyy", &format!("{year:04}")).replace("MM", &format!("{month:02}")).replace("dd", &format!("{day:02}")).replace("HH", &format!("{:02}", day_seconds / 3600)).replace("mm", &format!("{:02}", day_seconds / 60 % 60)).replace("ss", &format!("{:02}", day_seconds % 60)); let pattern = regex::Regex::new("[{]clock(?::([^}]*))?[}]").unwrap(); std::rc::Rc::<str>::from(pattern.replace_all(&*v0, |captures: &regex::Captures| formatted(captures.get(1).map(|format| format.as_str()).unwrap_or("HH:mm:ss"))).into_owned()) };
    v2.clone()
}
fn ended_with_newline_6(mut v0: Rc<str>) -> bool {
    let v1: bool = (v0).is_empty() || (v0).ends_with(char::from(10u8));
    v1
}
fn appended_to_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { use std::io::Write; std::rc::Rc::<str>::from(std::fs::OpenOptions::new().create(true).append(true).open(std::path::Path::new(&*v0).join(&*v1)).and_then(|mut file| file.write_all((v2).as_bytes())).err().map(|error| format!("append {}: {error}", &*v1)).unwrap_or_default()) };
    v3.clone()
}
fn text_bytes_8(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).len() as u64;
    v1
}
fn line_count_9(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let v6: Rc<str> = v2.clone().borrow()[v5 as usize].clone();
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
fn decimal_text_11(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn counted_10(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_11(v0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: bool = v0 == 1u64;
    let v7: Rc<str> = if v6 {
        v1.clone()
    } else {
        v2.clone()
    };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v7));
    v8.clone()
}
fn append_step_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let v3: Rc<str> = file_text_1(v0.clone(), v1.clone());
    let v4: bool = uses_crlf_2(v3.clone());
    let v7: Rc<str> = if v4 {
        crlf_3()
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v6.clone()
    };
    let v8: i64 = local_offset_minutes_4();
    let v9: Rc<str> = clock_filled_5(v2.clone(), v8);
    let v10: bool = ended_with_newline_6(v3.clone());
    let v12: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        v7.clone()
    };
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v9)), v7));
    let v15: Rc<str> = appended_to_7(v0.clone(), v1.clone(), v14.clone());
    let v16: bool = &*v15 == "";
    if v16 {
        let v17: Rc<str> = file_text_1(v0.clone(), v1.clone());
        let v18: u64 = text_bytes_8(v14.clone());
        let v19: u64 = line_count_9(v9.clone());
        let v20: u64 = line_count_9(v17.clone());
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = counted_10(v20, v21.clone(), v22.clone());
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  now "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        let v28: u64 = text_bytes_8(v17.clone());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = counted_10(v28, v29.clone(), v30.clone());
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v31));
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v1));
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
        let v37: Rc<str> = counted_10(v19, v21.clone(), v22.clone());
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
        let v39: Rc<str> = counted_10(v18, v29.clone(), v30.clone());
        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v39)), v32))));
        (v42.clone(), false, v19, v18)
    } else {
        (v15.clone(), true, 0u64, 0u64)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        append_step_0(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn glob_base_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).replace(char::from(92u8), "/").split(char::from(47u8)).take_while(|part| !part.contains(char::from(42u8)) && !part.contains(char::from(63u8))).collect::<Vec<_>>().join("/"));
    v1.clone()
}
fn matching_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_do_read::kit_do_read_matching(&*v0, &*v1);
    v2
}
fn line_at_16(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: i32 = (v1 as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: bool = v2 >= 0i32;
    let v15: bool = if v5 {
        let v6: i32 = (v4.clone().borrow().len() as i32);
        let v7: bool = v6 > 0i32;
        let v11: bool = if v7 {
            let v8: i32 = v6.wrapping_sub(1i32);
            let v9: Rc<str> = v4.clone().borrow()[v8 as usize].clone();
            let v10: bool = &*v9 == "";
            v10
        } else {
            false
        };
        let v13: i32 = if v11 {
            let v12: i32 = v6.wrapping_sub(1i32);
            v12
        } else {
            v6
        };
        let v14: bool = v2 < v13;
        v14
    } else {
        false
    };
    if v15 {
        let v16: Rc<str> = v4.clone().borrow()[v2 as usize].clone();
        let v17: i32 = (v16.clone().len() as i32);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v16.ends_with(&*v18);
        let v20: bool = v19 == false;
        if v20 {
            v16.clone()
        } else {
            let v21: bool = v17 == 1i32;
            if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22.clone()
            } else {
                let v23: i32 = v17.wrapping_sub(2i32);
                let v24: Rc<str> = string_slice(&v16.clone(), 0i32 as i64, v23 as i64);
                v24.clone()
            }
        }
    } else {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v27.clone()
    }
}
fn excluded_by_17(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v0).lines().any(|pattern| { let name = (v1).rsplit(char::from(47u8)).next().unwrap_or(""); let mut translated = String::from("^"); for c in pattern.chars() { if c == char::from(42u8) { translated.push_str("[^/]*") } else if c == char::from(63u8) { translated.push_str("[^/]") } else { translated.push_str(&regex::escape(&c.to_string())) } } let translated = format!("{}$", translated.replace("[^/]*[^/]*", ".*")); regex::Regex::new(&translated).map(|regex| regex.is_match(name) || regex.is_match(&*v1)).unwrap_or(false) });
    v2
}
fn relative_under_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(if (v0).is_empty() { &*v1 } else { (v1).strip_prefix(&*v0).map(|rest| rest.trim_start_matches(char::from(47u8))).unwrap_or(&*v1) });
    v2.clone()
}
fn copied_bytes_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i64 {
    let v3: i64 = { let root = std::path::Path::new(&*v0); let target = root.join(&*v2); target.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::copy(root.join(&*v1), &target)).map(|bytes| bytes as i64).unwrap_or(-1) };
    v3
}
fn joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0.clone()
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v4)), v1));
            v6.clone()
        }
    }
}
fn copies_run_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: bool, mut v8: u64, mut v9: u64, mut v10: u64) -> (Rc<str>, bool, u64, u64, u64) {
    let v11: u64 = line_count_9(v4.clone());
    let v12: bool = v5 < v11;
    let (mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, bool, u64, u64, u64) = if v12 {
        let v13: Rc<str> = line_at_16(v4.clone(), v5);
        let v14: bool = excluded_by_17(v3.clone(), v13.clone());
        let (mut v35, mut v36, mut v37, mut v38, mut v39): (Rc<str>, bool, u64, u64, u64) = if v14 {
            let v15: u64 = v9.wrapping_add(1u64);
            (v6.clone(), v7, v8, v15, v10)
        } else {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v16));
            let v18: Rc<str> = relative_under_18(v1.clone(), v13.clone());
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            let v20: i64 = copied_bytes_19(v0.clone(), v13.clone(), v19.clone());
            let v21: bool = v20 < 0i64;
            let (mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, bool, u64, u64, u64) = if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v13));
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed"); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let v26: Rc<str> = joined_20(v6.clone(), v25.clone());
                (v26.clone(), true, v8, v9, v10)
            } else {
                let v27: u64 = v8.wrapping_add(1u64);
                let v28: u64 = (v20 as u64);
                let v29: u64 = v10.wrapping_add(v28);
                (v6.clone(), v7, v27, v9, v29)
            };
            (v30.clone(), v31, v32, v33, v34)
        };
        let v40: u64 = v5.wrapping_add(1u64);
        let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, bool, u64, u64, u64) = copies_run_15(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v40, v35.clone(), v36, v37, v38, v39);
        (v41.clone(), v42, v43, v44, v45)
    } else {
        (v6.clone(), v7, v8, v9, v10)
    };
    (v46.clone(), v47, v48, v49, v50)
}
fn copy_step_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, bool, u64, u64, u64) {
    let v4: Rc<str> = glob_base_13(v1.clone());
    let v5: Rc<str> = matching_14(v0.clone(), v1.clone());
    let v6: u64 = 0u64;
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: bool = false;
    let v9: u64 = 0u64;
    let v10: u64 = 0u64;
    let v11: u64 = 0u64;
    let (mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64) = copies_run_15(v0.clone(), v4.clone(), v2.clone(), v3.clone(), v5.clone(), v6, v7.clone(), v8, v9, v10, v11);
    let v17: bool = &*v3 == "";
    let v29: Rc<str> = if v17 {
        v7.clone()
    } else {
        let v18: Rc<str> = decimal_text_11(v15);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped ("); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(v3.replace(&*v23, &*v24));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v25));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        v28.clone()
    };
    let v30: bool = &*v4 == "";
    let v32: Rc<str> = if v30 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        v31.clone()
    } else {
        v4.clone()
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v34, v35)), v2));
    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let v40: Rc<str> = counted_10(v14, v38.clone(), v39.clone());
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v41, v40)), v41));
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = counted_10(v16, v44.clone(), v45.clone());
    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v37, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v43, v46)), v29))));
    let v51: bool = if v13 {
        true
    } else {
        let v50: bool = &*v5 == "";
        v50
    };
    let v52: bool = &*v5 == "";
    let v55: Rc<str> = if v52 {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: no file matched "); } LIT.with(|lit| lit.clone()) };
        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v1));
        v54.clone()
    } else {
        v49.clone()
    };
    let v56: Rc<str> = joined_20(v12.clone(), v55.clone());
    (v56.clone(), v51, v14, v15, v16)
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (Rc<str>, bool, u64, u64, u64) {
        copy_step_12(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_append_step(v0: &str, v1: &str, v2: &str) -> (Rc<str>, bool, u64, u64) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_do_copy_step(v0: &str, v1: &str, v2: &str, v3: &str) -> (Rc<str>, bool, u64, u64, u64) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
