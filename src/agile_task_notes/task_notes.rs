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
    US0_FileReadable(Rc<str>),
    US0_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US1 {
    US1_AuthorDefault,
    US1_AuthorNamed(Rc<str>),
    US1_AuthorUnnamedAgent,
    US1_AuthorMalformed,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_GroomedText(Rc<str>),
    US3_GroomedRefused(Rc<str>),
}
fn concat5_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn declared_from_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = v0.get(v2 as usize..).and_then(|rest| rest.find(&*v1)).map(|found| (found + v2 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        if v4 {
            return v3;
        } else {
            let v5: i32 = v0.get(..v3 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v6: Rc<str> = std::rc::Rc::<str>::from(v0.get(v5 as usize..v3 as usize).unwrap_or(""));
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
fn note_schema_ensured_4(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let mut v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
    let v3: bool = v2 < 0i32;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> task_planning"); } LIT.with(|lit| lit.clone()) };
        let v5: i32 = v0.find(&*v4).map(|offset| offset as i32).unwrap_or(-1);
        let v6: bool = v5 < 0i32;
        let mut v9: i32 = if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-> agile_item"); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
            v8
        } else {
            v5
        };
        let v10: bool = v9 < 0i32;
        if v10 {
            let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_2(v11, v12, v0, v13.clone(), v13.clone())
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v16: i32 = v0.get(v9 as usize..).and_then(|rest| rest.find(&*v15)).map(|found| (found + v9 as usize) as i32).unwrap_or(-1);
            let v17: bool = v16 < 0i32;
            let v19: i32 = if v17 {
                let v18: i32 = (v0.clone().len() as i32);
                v18
            } else {
                v16
            };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = concat5_2(v20, v21, v22.clone(), v22.clone(), v22.clone());
            let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v25: i32 = (v0.clone().len() as i32);
            let v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            concat5_2(v24, v23, v26, v22.clone(), v22.clone())
        }
    } else {
        v0
    }
}
fn declared_count_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_3(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v2, v3) = (v6, v7);
            continue;
        }
    }
}
fn civil_of_days_7(mut v0: u64) -> (u64, u64, u64) {
    let mut v1: u64 = v0.wrapping_add(719468u64);
    let mut v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let mut v11: u64 = v10.wrapping_div(365u64);
    let v12: u64 = 365u64.wrapping_mul(v11);
    let v13: u64 = v11.wrapping_div(4u64);
    let v14: u64 = v12.wrapping_add(v13);
    let v15: u64 = v11.wrapping_div(100u64);
    let v16: u64 = v14.wrapping_sub(v15);
    let v17: u64 = v4.wrapping_sub(v16);
    let v18: u64 = 5u64.wrapping_mul(v17);
    let v19: u64 = v18.wrapping_add(2u64);
    let v20: u64 = v19.wrapping_div(153u64);
    let v21: u64 = 153u64.wrapping_mul(v20);
    let v22: u64 = v21.wrapping_add(2u64);
    let v23: u64 = v22.wrapping_div(5u64);
    let v24: u64 = v17.wrapping_sub(v23);
    let v25: u64 = v24.wrapping_add(1u64);
    let v26: bool = v20 < 10u64;
    let v29: u64 = if v26 {
        let v27: u64 = v20.wrapping_add(3u64);
        v27
    } else {
        let v28: u64 = v20.wrapping_sub(9u64);
        v28
    };
    let v30: u64 = v2.wrapping_mul(400u64);
    let v31: u64 = v11.wrapping_add(v30);
    let v32: bool = v29 < 3u64;
    let v33: u64 = if v32 {
        1u64
    } else {
        0u64
    };
    let v34: u64 = v31.wrapping_add(v33);
    (v34, v29, v25)
}
fn utc_stamp_6(mut v0: u64) -> Rc<str> {
    let mut v1: u64 = v0.wrapping_div(1000u64);
    let mut v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_7(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v11, v12, v13.clone(), v13.clone(), v13)
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v18, v19, v20.clone(), v20.clone(), v20)
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22
    };
    let v24: Rc<str> = concat5_2(v8, v9.clone(), v16, v9, v23);
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v27, v28, v29.clone(), v29.clone(), v29)
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_2(v36, v37, v38.clone(), v38.clone(), v38)
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = concat5_2(v32, v33, v41, v42, v43.clone());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v24, v45, v44, v43.clone(), v43)
}
fn note_row_8(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(v0.replace(&*v7, &*v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let mut v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = concat5_2(v6, v10, v8, v11, v12.clone());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(v0.replace(&*v16, &*v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(v18.replace(&*v15, &*v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(v20.replace(&*v21, &*v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(v23.replace(&*v24, &*v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(v26.replace(&*v27, &*v28));
    let v30: Rc<str> = concat5_2(v15.clone(), v29, v15.clone(), v12.clone(), v12.clone());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = concat5_2(v5, v13, v14, v30, v31.clone());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = Rc::<str>::from(v2.replace(&*v16, &*v17));
    let v36: Rc<str> = Rc::<str>::from(v35.replace(&*v15, &*v19));
    let v37: Rc<str> = Rc::<str>::from(v36.replace(&*v21, &*v22));
    let v38: Rc<str> = Rc::<str>::from(v37.replace(&*v24, &*v25));
    let v39: Rc<str> = Rc::<str>::from(v38.replace(&*v27, &*v28));
    let v40: Rc<str> = concat5_2(v15.clone(), v39, v15.clone(), v12.clone(), v12.clone());
    let v41: Rc<str> = Rc::<str>::from(v3.replace(&*v16, &*v17));
    let v42: Rc<str> = Rc::<str>::from(v41.replace(&*v15, &*v19));
    let v43: Rc<str> = Rc::<str>::from(v42.replace(&*v21, &*v22));
    let v44: Rc<str> = Rc::<str>::from(v43.replace(&*v24, &*v25));
    let v45: Rc<str> = Rc::<str>::from(v44.replace(&*v27, &*v28));
    let v46: Rc<str> = concat5_2(v15.clone(), v45, v15.clone(), v12.clone(), v12.clone());
    let v47: Rc<str> = concat5_2(v33, v34, v40, v31.clone(), v46);
    let v48: Rc<str> = Rc::<str>::from(v4.replace(&*v16, &*v17));
    let v49: Rc<str> = Rc::<str>::from(v48.replace(&*v15, &*v19));
    let v50: Rc<str> = Rc::<str>::from(v49.replace(&*v21, &*v22));
    let v51: Rc<str> = Rc::<str>::from(v50.replace(&*v24, &*v25));
    let v52: Rc<str> = Rc::<str>::from(v51.replace(&*v27, &*v28));
    let v53: Rc<str> = concat5_2(v15.clone(), v52, v15, v12.clone(), v12);
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    concat5_2(v32, v47, v31, v53, v54)
}
fn note_added_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_2(v0.clone(), v5, v6.clone(), v6.clone(), v6.clone());
    let v8: bool = std::path::Path::new(&*v7).is_file();
    let mut v11: US0 = if v8 {
        US0::US0_FileReadable(v7.clone())
    } else {
        US0::US0_FileMissing(v7.clone())
    };
    match v11 {
        US0::US0_FileMissing(v12) => {
            let v12: Rc<str> = v12;
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v13));
            let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v17
        }
        US0::US0_FileReadable(v18) => {
            let v18: Rc<str> = v18;
            let v19: Rc<str> = std::fs::read_to_string(&*v18).unwrap_or_default().into();
            let v20: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v19);
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = concat5_2(v21, v22.clone(), v1.clone(), v23.clone(), v6.clone());
            let v25: i32 = 0i32;
            let v26: i32 = declared_from_3(v20.clone(), v24, v25);
            let v27: bool = v26 < 0i32;
            let v28: bool = v27 == false;
            if v28 {
                let v29: Rc<str> = note_schema_ensured_4(v20.clone());
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state"); } LIT.with(|lit| lit.clone()) }));
                let v31: Rc<str> = agile_note_listing::kit_agile_state_segments(&*v30);
                let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v31));
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = concat5_2(v34, v22.clone(), v1.clone(), v23, v6.clone());
                let v36: i32 = 0i32;
                let v37: i32 = 0i32;
                let v38: i32 = declared_count_5(v33, v35, v36, v37);
                let v39: i32 = v38.wrapping_add(1i32);
                let v40: Rc<str> = utc_stamp_6(v4);
                let v41: Rc<str> = note_row_8(v1.clone(), v39, v40.clone(), v2.clone(), v3.clone());
                let (mut v42, mut v43): (Rc<str>, u64) = agile_note_segments::kit_agile_note_persisted(&*v30, &*v7, &*v19, &*v29, &*v1, &*v41, &*v40);
                let v44: bool = &*v42 == "";
                if v44 {
                    let v45: u64 = v20.len() as u64;
                    let v46: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile note ok id="); } LIT.with(|lit| lit.clone()) }, v1));
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) }));
                    let v49: Rc<str> = std::rc::Rc::<str>::from((v39).to_string());
                    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v50, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) }));
                    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) }));
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                    let v55: Rc<str> = Rc::<str>::from(v2.replace(&*v53, &*v54));
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<str> = Rc::<str>::from(v55.replace(&*v22, &*v56));
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                    let v60: Rc<str> = Rc::<str>::from(v57.replace(&*v58, &*v59));
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = Rc::<str>::from(v60.replace(&*v61, &*v62));
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                    let v66: Rc<str> = Rc::<str>::from(v63.replace(&*v64, &*v65));
                    let v67: Rc<str> = concat5_2(v22.clone(), v66, v22.clone(), v6.clone(), v6.clone());
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v67));
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v68, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) }));
                    let v70: i32 = (v3.len() as i32);
                    let v71: Rc<str> = std::rc::Rc::<str>::from((v70).to_string());
                    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v71));
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v72));
                    let v74: Rc<str> = std::rc::Rc::<str>::from((v45).to_string());
                    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v74));
                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
                    let v77: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                    let v80: Rc<str> = std::rc::Rc::<str>::from((v46).to_string());
                    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                    let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v81));
                    let v83: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v82.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                    v83
                } else {
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
                    let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                    let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v42));
                    let v87: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v86 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v87
                }
            } else {
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v90: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
                let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v89));
                let v93: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v92 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v93
            }
        }
    }
}
fn command_line_argument_count_11() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_12(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_10(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_11();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_12(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let mut v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_10(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method16(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn note_text_of_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = (v0.clone().len() as i32);
    let mut v2: i32 = 0i32;
    let v3: i32 = method15(v0.clone(), v1, v2);
    let v4: i32 = v1.wrapping_sub(1i32);
    let v5: Rc<str> = string_slice(&v0.clone(), v3 as i64, v4 as i64);
    let v6: i32 = (v5.clone().len() as i32);
    let v7: i32 = method16(v5.clone(), v6);
    let v8: Rc<str> = string_slice(&v5, 0i32 as i64, v7 as i64);
    let mut v9: bool = &*v8 == "-";
    let v21: Rc<str> = if v9 {
        let v10: std::io::Stdin = std::io::stdin();
        let mut v11: Result<std::string::String, std::io::Error> = std::io::read_to_string(v10);
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: &str = &*v12;
        let v16: std::string::String = String::from(v14);
        let v18: std::string::String = v11.unwrap_or(v16);
        let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
        v20
    } else {
        v0
    };
    let v22: i32 = (v21.clone().len() as i32);
    let v23: i32 = 0i32;
    let v24: i32 = method15(v21.clone(), v22, v23);
    let v25: i32 = v22.wrapping_sub(1i32);
    let v26: Rc<str> = string_slice(&v21, v24 as i64, v25 as i64);
    let v27: i32 = (v26.clone().len() as i32);
    let v28: i32 = method16(v26.clone(), v27);
    let v29: Rc<str> = string_slice(&v26, 0i32 as i64, v28 as i64);
    v29
}
fn note_request_from_13(mut v0: &Rc<UH0>) -> (Rc<str>, US1) {
    let (mut v143, mut v144): (Rc<str>, US1) = match &**v0 {
        UH0::UH0_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH0> = v2.clone();
            match Rc::unwrap_or_clone(v2) {
                UH0::UH0_Cons(v3, v4) => {
                    let v3: Rc<str> = v3;
                    let v4: Rc<UH0> = v4;
                    match Rc::unwrap_or_clone(v4) {
                        UH0::UH0_Cons(v5, v6) => {
                            let v5: Rc<str> = v5;
                            let v6: Rc<UH0> = v6;
                            match Rc::unwrap_or_clone(v6) {
                                UH0::UH0_Cons(v7, v8) => {
                                    let v7: Rc<str> = v7;
                                    let v8: Rc<UH0> = v8;
                                    match Rc::unwrap_or_clone(v8) {
                                        UH0::UH0_Cons(v9, v10) => {
                                            let mut v9: Rc<str> = v9;
                                            let v10: Rc<UH0> = v10;
                                            match &*v10 {
                                                UH0::UH0_Nil => {
                                                    let mut v11: Rc<str> = note_text_of_14(v9);
                                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                                                    let v13: Rc<str> = std::env::var(&*v12).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                    let v14: i32 = (v13.clone().len() as i32);
                                                    let v15: i32 = 0i32;
                                                    let v16: i32 = method15(v13.clone(), v14, v15);
                                                    let v17: i32 = v14.wrapping_sub(1i32);
                                                    let v18: Rc<str> = string_slice(&v13, v16 as i64, v17 as i64);
                                                    let v19: i32 = (v18.clone().len() as i32);
                                                    let v20: i32 = method16(v18.clone(), v19);
                                                    let v21: Rc<str> = string_slice(&v18, 0i32 as i64, v20 as i64);
                                                    let v22: bool = &*v21 == "";
                                                    let v23: bool = v22 == false;
                                                    let v62: US1 = if v23 {
                                                        let v24: Rc<str> = std::env::var(&*v12).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                        let v25: i32 = (v24.clone().len() as i32);
                                                        let v26: i32 = 0i32;
                                                        let v27: i32 = method15(v24.clone(), v25, v26);
                                                        let v28: i32 = v25.wrapping_sub(1i32);
                                                        let v29: Rc<str> = string_slice(&v24, v27 as i64, v28 as i64);
                                                        let v30: i32 = (v29.clone().len() as i32);
                                                        let v31: i32 = method16(v29.clone(), v30);
                                                        let v32: Rc<str> = string_slice(&v29, 0i32 as i64, v31 as i64);
                                                        US1::US1_AuthorNamed(v32)
                                                    } else {
                                                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AI_AGENT"); } LIT.with(|lit| lit.clone()) };
                                                        let v35: Rc<str> = std::env::var(&*v34).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                        let v36: i32 = (v35.clone().len() as i32);
                                                        let v37: i32 = 0i32;
                                                        let v38: i32 = method15(v35.clone(), v36, v37);
                                                        let v39: i32 = v36.wrapping_sub(1i32);
                                                        let v40: Rc<str> = string_slice(&v35, v38 as i64, v39 as i64);
                                                        let v41: i32 = (v40.clone().len() as i32);
                                                        let v42: i32 = method16(v40.clone(), v41);
                                                        let v43: Rc<str> = string_slice(&v40, 0i32 as i64, v42 as i64);
                                                        let v44: bool = &*v43 == "";
                                                        let v45: bool = v44 == false;
                                                        let v58: bool = if v45 {
                                                            true
                                                        } else {
                                                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CLAUDECODE"); } LIT.with(|lit| lit.clone()) };
                                                            let v47: Rc<str> = std::env::var(&*v46).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                                                            let v48: i32 = (v47.clone().len() as i32);
                                                            let v49: i32 = 0i32;
                                                            let v50: i32 = method15(v47.clone(), v48, v49);
                                                            let v51: i32 = v48.wrapping_sub(1i32);
                                                            let v52: Rc<str> = string_slice(&v47, v50 as i64, v51 as i64);
                                                            let v53: i32 = (v52.clone().len() as i32);
                                                            let v54: i32 = method16(v52.clone(), v53);
                                                            let v55: Rc<str> = string_slice(&v52, 0i32 as i64, v54 as i64);
                                                            let v56: bool = &*v55 == "";
                                                            let v57: bool = v56 == false;
                                                            v57
                                                        };
                                                        if v58 {
                                                            US1::US1_AuthorUnnamedAgent
                                                        } else {
                                                            US1::US1_AuthorDefault
                                                        }
                                                    };
                                                    (v11, v62)
                                                }
                                                _ => {
                                                    let (mut v121, mut v122): (Rc<str>, US1) = match Rc::unwrap_or_clone(v10) {
                                                        UH0::UH0_Cons(v63, v64) => {
                                                            let v63: Rc<str> = v63;
                                                            let v64: Rc<UH0> = v64;
                                                            match Rc::unwrap_or_clone(v64) {
                                                                UH0::UH0_Cons(v65, v66) => {
                                                                    let v65: Rc<str> = v65;
                                                                    let v66: Rc<UH0> = v66;
                                                                    match Rc::unwrap_or_clone(v66) {
                                                                        UH0::UH0_Nil => {
                                                                            let v67: bool = &*v63 == "--as";
                                                                            if v67 {
                                                                                let v68: Rc<str> = note_text_of_14(v9);
                                                                                let v69: i32 = (v65.clone().len() as i32);
                                                                                let v70: i32 = 0i32;
                                                                                let v71: i32 = method15(v65.clone(), v69, v70);
                                                                                let v72: i32 = v69.wrapping_sub(1i32);
                                                                                let v73: Rc<str> = string_slice(&v65.clone(), v71 as i64, v72 as i64);
                                                                                let v74: i32 = (v73.clone().len() as i32);
                                                                                let v75: i32 = method16(v73.clone(), v74);
                                                                                let v76: Rc<str> = string_slice(&v73, 0i32 as i64, v75 as i64);
                                                                                let v77: bool = &*v76 == "";
                                                                                let v86: US1 = if v77 {
                                                                                    US1::US1_AuthorMalformed
                                                                                } else {
                                                                                    let v79: i32 = 0i32;
                                                                                    let v80: i32 = method15(v65.clone(), v69, v79);
                                                                                    let v81: Rc<str> = string_slice(&v65, v80 as i64, v72 as i64);
                                                                                    let v82: i32 = (v81.clone().len() as i32);
                                                                                    let v83: i32 = method16(v81.clone(), v82);
                                                                                    let v84: Rc<str> = string_slice(&v81, 0i32 as i64, v83 as i64);
                                                                                    US1::US1_AuthorNamed(v84)
                                                                                };
                                                                                (v68, v86)
                                                                            } else {
                                                                                let v87: bool = &*v9 == "--as";
                                                                                let (mut v109, mut v110): (Rc<str>, US1) = if v87 {
                                                                                    let v88: Rc<str> = note_text_of_14(v65);
                                                                                    let v89: i32 = (v63.clone().len() as i32);
                                                                                    let v90: i32 = 0i32;
                                                                                    let v91: i32 = method15(v63.clone(), v89, v90);
                                                                                    let v92: i32 = v89.wrapping_sub(1i32);
                                                                                    let v93: Rc<str> = string_slice(&v63.clone(), v91 as i64, v92 as i64);
                                                                                    let v94: i32 = (v93.clone().len() as i32);
                                                                                    let v95: i32 = method16(v93.clone(), v94);
                                                                                    let v96: Rc<str> = string_slice(&v93, 0i32 as i64, v95 as i64);
                                                                                    let v97: bool = &*v96 == "";
                                                                                    let v106: US1 = if v97 {
                                                                                        US1::US1_AuthorMalformed
                                                                                    } else {
                                                                                        let v99: i32 = 0i32;
                                                                                        let v100: i32 = method15(v63.clone(), v89, v99);
                                                                                        let v101: Rc<str> = string_slice(&v63, v100 as i64, v92 as i64);
                                                                                        let v102: i32 = (v101.clone().len() as i32);
                                                                                        let v103: i32 = method16(v101.clone(), v102);
                                                                                        let v104: Rc<str> = string_slice(&v101, 0i32 as i64, v103 as i64);
                                                                                        US1::US1_AuthorNamed(v104)
                                                                                    };
                                                                                    (v88, v106)
                                                                                } else {
                                                                                    let v107: Rc<str> = note_text_of_14(v9);
                                                                                    let v108: US1 = US1::US1_AuthorMalformed;
                                                                                    (v107, v108)
                                                                                };
                                                                                (v109, v110)
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            let v113: Rc<str> = note_text_of_14(v9);
                                                                            let v114: US1 = US1::US1_AuthorMalformed;
                                                                            (v113, v114)
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    let v117: Rc<str> = note_text_of_14(v9);
                                                                    let v118: US1 = US1::US1_AuthorMalformed;
                                                                    (v117, v118)
                                                                }
                                                            }
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v121, v122)
                                                }
                                            }
                                        }
                                        _ => {
                                            let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            let v126: US1 = US1::US1_AuthorMalformed;
                                            (v125, v126)
                                        }
                                    }
                                }
                                _ => {
                                    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v130: US1 = US1::US1_AuthorMalformed;
                                    (v129, v130)
                                }
                            }
                        }
                        _ => {
                            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v134: US1 = US1::US1_AuthorMalformed;
                            (v133, v134)
                        }
                    }
                }
                _ => {
                    let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v138: US1 = US1::US1_AuthorMalformed;
                    (v137, v138)
                }
            }
        }
        _ => {
            let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v142: US1 = US1::US1_AuthorMalformed;
            (v141, v142)
        }
    };
    (v143, v144)
}
fn note_request_of_9() -> (Rc<str>, US1) {
    let v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_10(v0);
    let (mut v2, mut v3): (Rc<str>, US1) = note_request_from_13(&v1);
    (v2, v3)
}
fn note_entry_0() -> i32 {
    let v0: i32 = 2i32;
    let mut v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let (mut v2, mut v3): (Rc<str>, US1) = note_request_of_9();
    let v4: bool = &*v2 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
        let mut v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        match v3 {
            US1::US1_AuthorDefault => {
                let v10: i32 = 1i32;
                let mut v11: Rc<str> = usize::try_from(v10).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
                note_added_1(v11, v1, v12, v2)
            }
            US1::US1_AuthorMalformed => {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note <root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v23));
                let v27: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v26 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v27
            }
            US1::US1_AuthorNamed(v14) => {
                let v14: Rc<str> = v14;
                let v15: i32 = 1i32;
                let v16: Rc<str> = usize::try_from(v15).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_added_1(v16, v1, v14, v2)
            }
            US1::US1_AuthorUnnamedAgent => {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("an agent session must name the note's author: agile note <root> <id> <text> --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) }, v1));
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) }));
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v18));
                let v22: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v21 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v22
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planned_18(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US3 {
    match &v0 {
        US3::US3_GroomedRefused(v4) => {
            let v4: Rc<str> = v4.clone();
            v0
        }
        US3::US3_GroomedText(v5) => {
            let v5: Rc<str> = v5.clone();
            let v6: bool = &*v3 == "";
            if v6 {
                v0
            } else {
                let (mut v7, mut v8): (i32, Rc<str>) = agile_planning_values::kit_agile_planning_value_token(&*v2, &*v3);
                let mut v9: bool = v7 == 0i32;
                let v12: US3 = if v9 {
                    US3::US3_GroomedText(v8)
                } else {
                    US3::US3_GroomedRefused(v8)
                };
                match v12 {
                    US3::US3_GroomedRefused(v13) => {
                        let v13: Rc<str> = v13;
                        US3::US3_GroomedRefused(v13)
                    }
                    US3::US3_GroomedText(v15) => {
                        let v15: Rc<str> = v15;
                        let (mut v16, mut v17): (i32, Rc<str>) = agile_planning_rewrite::kit_agile_planning_rewrite(&*v5, &*v1, &*v2, &*v15);
                        let v18: bool = v16 == 0i32;
                        if v18 {
                            US3::US3_GroomedText(v17)
                        } else {
                            US3::US3_GroomedRefused(v17)
                        }
                    }
                }
            }
        }
    }
}
fn last_declared_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_3(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            (v2, v3) = (v6, v4);
            continue;
        }
    }
}
fn notes_insertion_20(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v2, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let v7: i32 = 0i32;
    let v8: i32 = declared_from_3(v0.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = concat5_2(v9, v3.clone(), v1.clone(), v4.clone(), v5.clone());
    let mut v11: i32 = 0i32;
    let v12: i32 = declared_from_3(v0.clone(), v10, v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = concat5_2(v13, v3, v1, v4, v5);
    let v15: i32 = 0i32;
    let v16: i32 = -1i32;
    let v17: i32 = last_declared_21(v0.clone(), v14, v15, v16);
    let v18: bool = v12 <= v17;
    let v19: i32 = if v18 {
        v17
    } else {
        v12
    };
    let v20: bool = v8 <= v19;
    let v21: i32 = if v20 {
        v19
    } else {
        v8
    };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = v0.get(v21 as usize..).and_then(|rest| rest.find(&*v22)).map(|found| (found + v21 as usize) as i32).unwrap_or(-1);
    let v24: bool = v23 < 0i32;
    if v24 {
        let v25: i32 = (v0.len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = notes_insertion_20(v0.clone(), v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_2(v4, v2, v5.clone(), v5.clone(), v5.clone());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    concat5_2(v7, v6, v9, v5.clone(), v5)
}
fn groomed_text_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (i32, Rc<str>) {
    let v7: US3 = US3::US3_GroomedText(v0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US3 = planned_18(v7, v1.clone(), v8, v2);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let mut v11: US3 = planned_18(v9, v1.clone(), v10, v3);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let v13: US3 = planned_18(v11, v1.clone(), v12, v4);
    match v13 {
        US3::US3_GroomedRefused(v14) => {
            let v14: Rc<str> = v14;
            (1i32, v14)
        }
        US3::US3_GroomedText(v15) => {
            let v15: Rc<str> = v15;
            let v16: bool = &*v5 == "";
            if v16 {
                (0i32, v15)
            } else {
                let v17: Rc<str> = note_schema_ensured_4(v15);
                let v18: i32 = 1i32;
                let v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                let v20: Rc<str> = utc_stamp_6(v19);
                let v21: Rc<str> = note_row_8(v1.clone(), v18, v20, v6, v5);
                let v22: Rc<str> = notes_appended_19(v17, v1, v21);
                (0i32, v22)
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (i32, Rc<str>) {
        groomed_text_17(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> Rc<str>> = Rc::new(move |mut v0: u64| -> Rc<str> {
        utc_stamp_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_note_run() -> i32 {
    closure0()()
}
pub fn kit_agile_groomed(v0: &str, v1: &str, v2: &str, v3: &str, v4: &str, v5: &str, v6: &str) -> (i32, Rc<str>) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), Rc::<str>::from(v6))
}
pub fn kit_utc_stamp(v0: u64) -> Rc<str> {
    closure2()(v0)
}
