#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_verbs::{write_verb, option_write_verb};
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
    US0_Ok,
    US0_Error(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
fn file_text_1(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default());
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
    let mut v2: Rc<str> = { let seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs() as i64).unwrap_or(0) + v1 * 60; let (days, day_seconds) = (seconds.div_euclid(86400), seconds.rem_euclid(86400)); let shifted = days + 719468; let era = shifted.div_euclid(146097); let of_era = shifted - era * 146097; let year_of_era = (of_era - of_era / 1460 + of_era / 36524 - of_era / 146096) / 365; let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100); let month_index = (5 * of_year + 2) / 153; let day = of_year - (153 * month_index + 2) / 5 + 1; let month = if month_index < 10 { month_index + 3 } else { month_index - 9 }; let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 }; let formatted = |format: &str| format.replace("yyyy", &format!("{year:04}")).replace("MM", &format!("{month:02}")).replace("dd", &format!("{day:02}")).replace("HH", &format!("{:02}", day_seconds / 3600)).replace("mm", &format!("{:02}", day_seconds / 60 % 60)).replace("ss", &format!("{:02}", day_seconds % 60)); let pattern = regex::Regex::new("[{]clock(?::([^}]*))?[}]").unwrap(); std::rc::Rc::<str>::from(pattern.replace_all(&*v0, |captures: &regex::Captures| formatted(captures.get(1).map(|format| format.as_str()).unwrap_or("HH:mm:ss"))).into_owned()) };
    v2
}
fn ended_with_newline_6(mut v0: Rc<str>) -> bool {
    let v1: bool = (v0).is_empty() || (v0).ends_with(char::from(10u8));
    v1
}
fn method8(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let mut v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method8(v13);
    let mut v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn directory_get_parent_9(mut v0: Rc<str>) -> Option<Rc<str>> {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: Option<std::path::PathBuf> = v6.parent().map(std::path::PathBuf::from);
    let v10: bool = true; let _optionm_map_ = v8.map(|x| { //;
    let mut v12: std::path::PathBuf = x;
    let mut v13: std::path::PathBuf = method8(v12.clone());
    let v15: std::path::Display = v13.display();
    let v17: std::string::String = format!("{}", v15);
    let v19: Rc<str> = Rc::<str>::from(String::as_str(&v17));
    let v21: bool = true; v19 });
    let mut v23: Option<Rc<str>> = _optionm_map_;
    v23
}
fn text_bytes_10(mut v0: Rc<str>) -> u64 {
    let v1: u64 = (v0).len() as u64;
    v1
}
fn line_count_11(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let mut v6: Rc<str> = v2.borrow()[v5 as usize].clone();
        let mut v7: bool = &*v6 == "";
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
fn decimal_text_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn counted_12(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_13(v0);
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v5: bool = v0 == 1u64;
    let mut v6: Rc<str> = if v5 {
        v1
    } else {
        v2
    };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
    v7
}
fn append_step_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let v3: Rc<str> = file_text_1(v0.clone(), v1.clone());
    let v4: bool = uses_crlf_2(v3.clone());
    let mut v7: Rc<str> = if v4 {
        crlf_3()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v6
    };
    let v8: i64 = local_offset_minutes_4();
    let v9: Rc<str> = clock_filled_5(v2, v8);
    let v10: bool = ended_with_newline_6(v3);
    let mut v12: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        v7.clone()
    };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v9.clone()));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v7));
    let v15: Rc<str> = method7(v0.clone(), v1.clone());
    let mut v16: Option<Rc<str>> = directory_get_parent_9(v15.clone());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = v18.clone();
    let v20: Rc<str> = v16.unwrap_or(v19);
    let v21: bool = &*v20 != "";
    let v26: bool = if v21 {
        let mut v23: Result<(), std::io::Error> = std::fs::create_dir_all(&*v20);
        let v25: bool = v23.is_err();
        v25
    } else {
        false
    };
    let v50: Rc<str> = if v26 {
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": the folder "); } LIT.with(|lit| lit.clone()) }));
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v20));
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" could not be created"); } LIT.with(|lit| lit.clone()) }));
        v30
    } else {
        let mut v31: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v15);
        let v32: bool = v31.is_ok();
        let v33: bool = v32 == false;
        let v43: US0 = if v33 {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file could not be opened"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Error(v34)
        } else {
            let mut v36: std::fs::File = v31.unwrap();
            let mut v37: std::io::Result<()> = std::io::Write::write_all(&mut v36, (v14).as_bytes());
            let v38: bool = v37.is_ok();
            if v38 {
                US0::US0_Ok
            } else {
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the write failed"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Error(v40)
            }
        };
        match v43 {
            US0::US0_Error(v44) => {
                let v44: Rc<str> = v44;
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v44));
                v47
            }
            US0::US0_Ok => {
                v18
            }
        }
    };
    let v51: bool = &*v50 == "";
    if v51 {
        let v52: Rc<str> = file_text_1(v0, v1.clone());
        let v53: u64 = text_bytes_10(v14);
        let v54: u64 = line_count_11(v9);
        let v55: u64 = line_count_11(v52.clone());
        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
        let v58: Rc<str> = counted_12(v55, v56.clone(), v57.clone());
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  now "); } LIT.with(|lit| lit.clone()) }, v58));
        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v59, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
        let v61: u64 = text_bytes_10(v52);
        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
        let v64: Rc<str> = counted_12(v61, v62.clone(), v63.clone());
        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v64));
        let v66: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) }, v1));
        let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v66, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
        let v68: Rc<str> = counted_12(v54, v56, v57);
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
        let v70: Rc<str> = counted_12(v53, v62, v63);
        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v70));
        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v71, v65));
        let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v72));
        (v73, false, v54, v53)
    } else {
        (v50, true, 0u64, 0u64)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        append_step_0(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn now_ms_15() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_16(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_at_18(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn body_at_21(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3
}
fn quoted_at_22(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn directory_exists_24(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn append_preview_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, u64) {
    let v3: Rc<str> = file_text_1(v0.clone(), v1.clone());
    let v4: bool = uses_crlf_2(v3.clone());
    let mut v7: Rc<str> = if v4 {
        crlf_3()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        v6
    };
    let v8: i64 = local_offset_minutes_4();
    let v9: Rc<str> = clock_filled_5(v2, v8);
    let v10: bool = ended_with_newline_6(v3);
    let mut v12: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11
    } else {
        v7.clone()
    };
    let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v9.clone()));
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v7));
    let v15: Rc<str> = method7(v0, v1.clone());
    let mut v16: Option<Rc<str>> = directory_get_parent_9(v15);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = v18.clone();
    let v20: Rc<str> = v16.unwrap_or(v19);
    let v21: bool = &*v20 != "";
    let v24: bool = if v21 {
        let v22: bool = directory_exists_24(v20.clone());
        let mut v23: bool = v22 == false;
        v23
    } else {
        false
    };
    let v26: Rc<str> = if v24 {
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  creates "); } LIT.with(|lit| lit.clone()) }, v20));
        v25
    } else {
        v18
    };
    let v27: u64 = text_bytes_10(v14);
    let v28: u64 = line_count_11(v9);
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("line"); } LIT.with(|lit| lit.clone()) };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
    let mut v31: Rc<str> = counted_12(v28, v29, v30);
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = counted_12(v27, v33, v34);
    let mut v36: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v35));
    let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append "); } LIT.with(|lit| lit.clone()) }, v1));
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v36));
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v26));
    (v40, v27)
}
fn append_previewed_20(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32) -> (Rc<str>, u64, u64, bool) {
    let v3: i32 = 2i32;
    let v4: Rc<str> = body_at_21(v1.clone(), v2, v3);
    let v5: bool = &*v4 == "";
    let v8: bool = if v5 {
        true
    } else {
        let mut v6: i32 = 2i32;
        quoted_at_22(v1.clone(), v2, v6)
    };
    let mut v12: bool = if v8 {
        true
    } else {
        let v9: i32 = 3i32;
        let v10: bool = quoted_at_22(v1.clone(), v2, v9);
        let v11: bool = v10 == false;
        v11
    };
    if v12 {
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("append: expected a file, then the quoted line"); } LIT.with(|lit| lit.clone()) };
        (v13, 0u64, 0u64, true)
    } else {
        let mut v14: i32 = 3i32;
        let v15: Rc<str> = body_at_21(v1, v2, v14);
        let (mut v16, mut v17): (Rc<str>, u64) = append_preview_23(v0, v4, v15);
        (v16, 1u64, v17, false)
    }
}
fn step_previewed_19(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: write_verb) -> (Rc<str>, u64, u64, bool) {
    match &v3 {
        write_verb::write_verb_FileAppend => {
            append_previewed_20(v0, v1, v2)
        }
        _ => {
            let v11: Rc<str> = match v3 {
                write_verb::write_verb_FileCopy => {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy"); } LIT.with(|lit| lit.clone()) };
                    v8
                }
                write_verb::write_verb_FileMove => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("move"); } LIT.with(|lit| lit.clone()) };
                    v9
                }
                write_verb::write_verb_FileWrite => {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("write"); } LIT.with(|lit| lit.clone()) };
                    v10
                }
                _ => unreachable!(),
            };
            let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry run previews append only; "); } LIT.with(|lit| lit.clone()) }, v11));
            let mut v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" runs only without --dry-run"); } LIT.with(|lit| lit.clone()) }));
            (v13, 0u64, 0u64, true)
        }
    }
}
fn joined_25(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "";
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
fn previews_from_17(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: bool) -> (Rc<str>, u64, u64, bool) {
    loop {
        let mut v7: i32 = v1.len() as i32;
        let v8: bool = v2 < v7;
        if v8 {
            let v9: i32 = v2.wrapping_add(1i32);
            let v10: i32 = 0i32;
            let v11: Rc<str> = field_at_18(v1.clone(), v2, v10);
            let mut v12: bool = &*v11 == "";
            if v12 {
                v2 = v9;
                continue;
            } else {
                let v17: i32 = 1i32;
                let v18: Rc<str> = field_at_18(v1.clone(), v2, v17);
                let v19: option_write_verb = kit_do_verbs::kit_do_write_named(&v18);
                match v19 {
                    option_write_verb::option_write_verb_None => {
                        v2 = v9;
                        continue;
                    }
                    option_write_verb::option_write_verb_Some(v24) => {
                        let v24: write_verb = v24;
                        let (mut v25, mut v26, mut v27, mut v28): (Rc<str>, u64, u64, bool) = step_previewed_19(v0.clone(), v1.clone(), v2, v24);
                        let v29: bool = v6 || v28;
                        let v30: Rc<str> = joined_25(v3, v25);
                        let mut v31: u64 = v4.wrapping_add(v26);
                        let v32: u64 = v5.wrapping_add(v27);
                        (v2, v3, v4, v5, v6) = (v9, v30, v31, v32, v29);
                        continue;
                    }
                }
            }
        } else {
            return (v3, v4, v5, v6);
        }
    }
}
fn elapsed_text_27(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn clock_text_28() -> Rc<str> {
    let v0: i64 = local_offset_minutes_4();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_26(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match v0 {
        US3::US3_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US3::US3_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_27(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let mut v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let mut v13: Rc<str> = clock_text_28();
    let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn writes_preview_14(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_15();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_16(v1);
    let v4: i32 = 1i32;
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: u64 = 0u64;
    let mut v7: u64 = 0u64;
    let v8: bool = false;
    let (mut v9, mut v10, mut v11, mut v12): (Rc<str>, u64, u64, bool) = previews_from_17(v0, v3, v4, v5, v6, v7, v8);
    let mut v13: bool = &*v9 == "";
    if v13 {
        ()
    } else {
        println!("{}", v9);
        ()
    };
    let mut v14: Rc<str> = decimal_text_13(v10);
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("appends "); } LIT.with(|lit| lit.clone()) }, v14));
    let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) }));
    let v17: Rc<str> = decimal_text_13(v11);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  dry run, nothing written"); } LIT.with(|lit| lit.clone()) }));
    if v12 {
        let v20: US3 = US3::US3_Failed;
        receipt_line_26(v20, v19, v2);
        2i32
    } else {
        let v21: US3 = US3::US3_Succeeded;
        receipt_line_26(v21, v19, v2);
        0i32
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        writes_preview_14(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn file_exists_30(mut v0: Rc<str>) -> bool {
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
fn glob_base_32(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).replace(char::from(92u8), "/").split(char::from(47u8)).take_while(|part| !part.contains(char::from(42u8)) && !part.contains(char::from(63u8))).collect::<Vec<_>>().join("/"));
    v1
}
fn method33(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure3() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn copy_base_31(mut v0: Rc<str>, mut v1: bool) -> Rc<str> {
    let mut v2: Rc<str> = glob_base_32(v0.clone());
    let v3: bool = v1 == false;
    let v8: bool = if v3 {
        true
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
        let mut v7: bool = &*v2 != &*v6;
        v7
    };
    if v8 {
        v2
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v10: Option<usize> = v2.rfind(&*v9);
        let v11: Option<usize> = method33(v10);
        let mut v12: Rc<dyn Fn((usize)) -> i32> = closure3();
        let mut v13: Option<i32> = v11.map(|x| v12(x));
        let v15: i32 = v13.unwrap_or(-1i32);
        let mut v16: bool = v15 < 0i32;
        if v16 {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v17
        } else {
            let v18: usize = ((0i32) as usize);
            let v19: usize = ((v15) as usize);
            let v20: Rc<str> = Rc::<str>::from(&(v2)[v18..v19]);
            v20
        }
    }
}
fn matching_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = kit_do_read::kit_do_read_matching(&*v0, &*v1);
    v2
}
fn line_at_36(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: i32 = (v1 as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: bool = v2 >= 0i32;
    let v15: bool = if v5 {
        let mut v6: i32 = (v4.clone().borrow().len() as i32);
        let mut v7: bool = v6 > 0i32;
        let v11: bool = if v7 {
            let v8: i32 = v6.wrapping_sub(1i32);
            let v9: Rc<str> = v4.borrow()[v8 as usize].clone();
            let v10: bool = &*v9 == "";
            v10
        } else {
            false
        };
        let mut v13: i32 = if v11 {
            let mut v12: i32 = v6.wrapping_sub(1i32);
            v12
        } else {
            v6
        };
        let mut v14: bool = v2 < v13;
        v14
    } else {
        false
    };
    if v15 {
        let mut v16: Rc<str> = v4.borrow()[v2 as usize].clone();
        let v17: i32 = (v16.clone().len() as i32);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
        let v19: bool = v16.ends_with(&*v18);
        let v20: bool = v19 == false;
        if v20 {
            v16
        } else {
            let v21: bool = v17 == 1i32;
            if v21 {
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v22
            } else {
                let mut v23: i32 = v17.wrapping_sub(2i32);
                let v24: Rc<str> = string_slice(&v16, 0i32 as i64, v23 as i64);
                v24
            }
        }
    } else {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v27
    }
}
fn excluded_by_37(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = (v0).lines().any(|pattern| { let name = (v1).rsplit(char::from(47u8)).next().unwrap_or(""); let mut translated = String::from("^"); for c in pattern.chars() { if c == char::from(42u8) { translated.push_str("[^/]*") } else if c == char::from(63u8) { translated.push_str("[^/]") } else { translated.push_str(&regex::escape(&c.to_string())) } } let translated = format!("{}$", translated.replace("[^/]*[^/]*", ".*")); regex::Regex::new(&translated).map(|regex| regex.is_match(name) || regex.is_match(&*v1)).unwrap_or(false) });
    v2
}
fn relative_under_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(if (v0).is_empty() { &*v1 } else { (v1).strip_prefix(&*v0).map(|rest| rest.trim_start_matches(char::from(47u8))).unwrap_or(&*v1) });
    v2
}
fn copied_bytes_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i64 {
    let v3: i64 = { let root = std::path::Path::new(&*v0); let target = root.join(&*v2); target.parent().map(std::fs::create_dir_all).unwrap_or(Ok(())).and_then(|_| std::fs::copy(root.join(&*v1), &target)).map(|bytes| bytes as i64).unwrap_or(-1) };
    v3
}
fn copies_run_35(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>, mut v7: bool, mut v8: u64, mut v9: u64, mut v10: u64) -> (Rc<str>, bool, u64, u64, u64) {
    let v11: u64 = line_count_11(v4.clone());
    let mut v12: bool = v5 < v11;
    let (mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<str>, bool, u64, u64, u64) = if v12 {
        let mut v13: Rc<str> = line_at_36(v4.clone(), v5);
        let mut v14: bool = excluded_by_37(v3.clone(), v13.clone());
        let (mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64) = if v14 {
            let v15: u64 = v9.wrapping_add(1u64);
            (v6, v7, v8, v15, v10)
        } else {
            let mut v16: Rc<str> = Rc::<str>::from(format!("{}{}", v2.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
            let v17: Rc<str> = relative_under_38(v1.clone(), v13.clone());
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            let v19: i64 = copied_bytes_39(v0.clone(), v13.clone(), v18);
            let v20: bool = v19 < 0i64;
            let (mut v27, mut v28, mut v29, mut v30, mut v31): (Rc<str>, bool, u64, u64, u64) = if v20 {
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) }, v13));
                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed"); } LIT.with(|lit| lit.clone()) }));
                let mut v23: Rc<str> = joined_25(v6, v22);
                (v23, true, v8, v9, v10)
            } else {
                let v24: u64 = v8.wrapping_add(1u64);
                let v25: u64 = (v19 as u64);
                let v26: u64 = v10.wrapping_add(v25);
                (v6, v7, v24, v9, v26)
            };
            (v27, v28, v29, v30, v31)
        };
        let mut v37: u64 = v5.wrapping_add(1u64);
        let (mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, bool, u64, u64, u64) = copies_run_35(v0, v1, v2, v3, v4, v37, v32, v33, v34, v35, v36);
        (v38, v39, v40, v41, v42)
    } else {
        (v6, v7, v8, v9, v10)
    };
    (v43, v44, v45, v46, v47)
}
fn copy_step_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, bool, u64, u64, u64) {
    let v4: Rc<str> = method7(v0.clone(), v1.clone());
    let v5: bool = file_exists_30(v4);
    let mut v6: Rc<str> = copy_base_31(v1.clone(), v5);
    let mut v7: Rc<str> = matching_34(v0.clone(), v1.clone());
    let v8: u64 = 0u64;
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: bool = false;
    let v11: u64 = 0u64;
    let mut v12: u64 = 0u64;
    let mut v13: u64 = 0u64;
    let (mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, bool, u64, u64, u64) = copies_run_35(v0, v6.clone(), v2.clone(), v3.clone(), v7.clone(), v8, v9.clone(), v10, v11, v12, v13);
    let v19: bool = &*v3 == "";
    let v28: Rc<str> = if v19 {
        v9
    } else {
        let v20: Rc<str> = decimal_text_13(v17);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v20));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" skipped ("); } LIT.with(|lit| lit.clone()) }));
        let mut v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(v3.replace(&*v23, &*v24));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v25));
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
        v27
    };
    let v29: bool = &*v6 == "";
    let mut v31: Rc<str> = if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        v6
    };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy "); } LIT.with(|lit| lit.clone()) }, v31));
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) }));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v2));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) };
    let mut v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
    let mut v37: Rc<str> = counted_12(v16, v35, v36);
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v37));
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("byte"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bytes"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = counted_12(v18, v40, v41);
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v42));
    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v28));
    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v44));
    let v47: bool = if v15 {
        true
    } else {
        let v46: bool = &*v7 == "";
        v46
    };
    let v48: bool = &*v7 == "";
    let v50: Rc<str> = if v48 {
        let v49: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("copy: no file matched "); } LIT.with(|lit| lit.clone()) }, v1));
        v49
    } else {
        v45
    };
    let v51: Rc<str> = joined_25(v14, v50);
    (v51, v47, v16, v17, v18)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (Rc<str>, bool, u64, u64, u64) {
        copy_step_29(v0, v1, v2, v3)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_append_step(v0: &str, v1: &str, v2: &str) -> (Rc<str>, bool, u64, u64) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_do_writes_preview(v0: &str, v1: &str) -> i32 {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_do_copy_step(v0: &str, v1: &str, v2: &str, v3: &str) -> (Rc<str>, bool, u64, u64, u64) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
