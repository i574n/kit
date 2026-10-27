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
    US1_StateWritten,
    US1_StateChangedSinceRead,
    US1_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
}
#[derive(Clone)]
enum US3 {
    US3_AuthorDefault,
    US3_AuthorNamed(Rc<str>),
    US3_AuthorUnnamedAgent,
    US3_AuthorMalformed,
}
#[derive(Clone)]
enum US4 {
    US4_GroomedText(Rc<str>),
    US4_GroomedRefused(Rc<str>),
}
fn declared_from_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1.clone(), v9);
                continue;
            }
        }
    }
}
fn note_schema_ensured_3(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note ="); } LIT.with(|lit| lit.clone()) };
    let v2: i32 = v0.find(&*v1).map(|offset| offset as i32).unwrap_or(-1);
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
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union task_note =\n    | Note :: string * u32 * string * string * string -> task_note"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v0, &*v13, &*v13].concat());
            v14.clone()
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
            let v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v21, &*v22, &*v22, &*v22].concat());
            let v24: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v19 as usize).unwrap_or(""));
            let v25: i32 = (v0.clone().len() as i32);
            let v26: Rc<str> = std::rc::Rc::<str>::from(v0.get(v19 as usize..v25 as usize).unwrap_or(""));
            let v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v23, &*v26, &*v22, &*v22].concat());
            v27.clone()
        }
    } else {
        v0.clone()
    }
}
fn declared_count_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            let v7: i32 = v3.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v7);
            continue;
        }
    }
}
fn civil_of_days_6(mut v0: u64) -> (u64, u64, u64) {
    let v1: u64 = v0.wrapping_add(719468u64);
    let v2: u64 = v1.wrapping_div(146097u64);
    let v3: u64 = v2.wrapping_mul(146097u64);
    let v4: u64 = v1.wrapping_sub(v3);
    let v5: u64 = v4.wrapping_div(1460u64);
    let v6: u64 = v4.wrapping_sub(v5);
    let v7: u64 = v4.wrapping_div(36524u64);
    let v8: u64 = v6.wrapping_add(v7);
    let mut v9: u64 = v4.wrapping_div(146096u64);
    let v10: u64 = v8.wrapping_sub(v9);
    let v11: u64 = v10.wrapping_div(365u64);
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
fn utc_stamp_5(mut v0: u64) -> Rc<str> {
    let v1: u64 = v0.wrapping_div(1000u64);
    let v2: u64 = v1.wrapping_div(86400u64);
    let (mut v3, mut v4, mut v5): (u64, u64, u64) = civil_of_days_6(v2);
    let v6: u64 = v1.wrapping_rem(86400u64);
    let v7: u64 = v6.wrapping_div(60u64);
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v10: bool = v4 < 10u64;
    let v16: Rc<str> = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v12, &*v13, &*v13, &*v13].concat());
        v14.clone()
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
        v15.clone()
    };
    let v17: bool = v5 < 10u64;
    let v23: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
        v21.clone()
    } else {
        let v22: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
        v22.clone()
    };
    let v24: Rc<str> = std::rc::Rc::<str>::from([&*v8, &*v9, &*v16, &*v9, &*v23].concat());
    let v25: u64 = v7.wrapping_div(60u64);
    let v26: bool = v25 < 10u64;
    let v32: Rc<str> = if v26 {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = std::rc::Rc::<str>::from([&*v27, &*v28, &*v29, &*v29, &*v29].concat());
        v30.clone()
    } else {
        let v31: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
        v31.clone()
    };
    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let v34: u64 = v7.wrapping_rem(60u64);
    let v35: bool = v34 < 10u64;
    let v41: Rc<str> = if v35 {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v37, &*v38, &*v38, &*v38].concat());
        v39.clone()
    } else {
        let v40: Rc<str> = std::rc::Rc::<str>::from((v34).to_string());
        v40.clone()
    };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Z"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v41, &*v42, &*v43].concat());
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("T"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v45, &*v44, &*v43, &*v43].concat());
    v46.clone()
}
fn note_row_7(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note_of_"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v7, &v8));
    let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
    let v11: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v10, &*v8, &*v11, &*v12].concat());
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note ("); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from(v0.replace(&*v16, &v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = std::rc::Rc::<str>::from(v18.replace(&*v15, &v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = std::rc::Rc::<str>::from(v20.replace(&*v21, &v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = std::rc::Rc::<str>::from(v23.replace(&*v24, &v25));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = std::rc::Rc::<str>::from(v26.replace(&*v27, &v28));
    let v30: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v12, &*v12].concat());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = std::rc::Rc::<str>::from([&*v5, &*v13, &*v14, &*v30, &*v31].concat());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u32, "); } LIT.with(|lit| lit.clone()) };
    let v35: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
    let v36: Rc<str> = std::rc::Rc::<str>::from(v35.replace(&*v15, &v19));
    let v37: Rc<str> = std::rc::Rc::<str>::from(v36.replace(&*v21, &v22));
    let v38: Rc<str> = std::rc::Rc::<str>::from(v37.replace(&*v24, &v25));
    let v39: Rc<str> = std::rc::Rc::<str>::from(v38.replace(&*v27, &v28));
    let v40: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v39, &*v15, &*v12, &*v12].concat());
    let v41: Rc<str> = std::rc::Rc::<str>::from(v3.replace(&*v16, &v17));
    let v42: Rc<str> = std::rc::Rc::<str>::from(v41.replace(&*v15, &v19));
    let v43: Rc<str> = std::rc::Rc::<str>::from(v42.replace(&*v21, &v22));
    let v44: Rc<str> = std::rc::Rc::<str>::from(v43.replace(&*v24, &v25));
    let v45: Rc<str> = std::rc::Rc::<str>::from(v44.replace(&*v27, &v28));
    let v46: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v45, &*v15, &*v12, &*v12].concat());
    let v47: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v40, &*v31, &*v46].concat());
    let v48: Rc<str> = std::rc::Rc::<str>::from(v4.replace(&*v16, &v17));
    let v49: Rc<str> = std::rc::Rc::<str>::from(v48.replace(&*v15, &v19));
    let v50: Rc<str> = std::rc::Rc::<str>::from(v49.replace(&*v21, &v22));
    let v51: Rc<str> = std::rc::Rc::<str>::from(v50.replace(&*v24, &v25));
    let v52: Rc<str> = std::rc::Rc::<str>::from(v51.replace(&*v27, &v28));
    let v53: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v52, &*v15, &*v12, &*v12].concat());
    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v55: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v47, &*v31, &*v53, &*v54].concat());
    v55.clone()
}
fn last_declared_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: i32 = declared_from_2(v0.clone(), v1.clone(), v2);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v3;
        } else {
            let v6: i32 = v4.wrapping_add(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v6, v4);
            continue;
        }
    }
}
fn notes_insertion_9(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v1, &*v4, &*v5].concat());
    let v7: i32 = 0i32;
    let v8: i32 = declared_from_2(v0.clone(), v6.clone(), v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Planning ("); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v3, &*v1, &*v4, &*v5].concat());
    let v11: i32 = 0i32;
    let v12: i32 = declared_from_2(v0.clone(), v10.clone(), v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v3, &*v1, &*v4, &*v5].concat());
    let v15: i32 = 0i32;
    let v16: i32 = -1i32;
    let v17: i32 = last_declared_10(v0.clone(), v14.clone(), v15, v16);
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
        let v25: i32 = (v0.clone().len() as i32);
        v25
    } else {
        v23
    }
}
fn notes_appended_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = notes_insertion_9(v0.clone(), v1.clone());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v5, &*v5].concat());
    let v7: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v3 as usize).unwrap_or(""));
    let v8: i32 = (v0.clone().len() as i32);
    let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v8 as usize).unwrap_or(""));
    let v10: Rc<str> = std::rc::Rc::<str>::from([&*v7, &*v6, &*v9, &*v5, &*v5].concat());
    v10.clone()
}
fn state_written_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US1 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v7, &*v8, &*v9, &*v4].concat());
        let v11: bool = std::fs::write(&*v10, &*v2).is_ok();
        let v23: US2 = if v11 {
            let v12: bool = std::fs::rename(&*v10, &*v0).is_ok();
            if v12 {
                let v13: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v13 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v17: bool = std::fs::remove_file(&*v10).is_ok() || !std::path::Path::new(&*v10).exists();
                if v17 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v23 {
            US2::US2_Replaced => {
                US1::US1_StateWritten
            }
            _ => {
                let v32: Rc<str> = match &v23 {
                    US2::US2_ReadbackMismatch => {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US2::US2_RenameFailed => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v26.clone()
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v25.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v32.clone())
            }
        }
    } else {
        US1::US1_StateChangedSinceRead
    }
}
fn note_added_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v6, &*v6].concat());
    let v8: bool = std::path::Path::new(&*v7).is_file();
    let v11: US0 = if v8 {
        US0::US0_FileReadable(v7.clone())
    } else {
        US0::US0_FileMissing(v7.clone())
    };
    match &v11 {
        US0::US0_FileMissing(v12) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v1));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v16)), v13));
            let v19: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v18 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v19
        }
        US0::US0_FileReadable(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: Rc<str> = std::fs::read_to_string(&*v20).unwrap_or_default().into();
            let v22: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v21);
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task ("); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v1, &*v25, &*v6].concat());
            let v27: i32 = 0i32;
            let v28: i32 = declared_from_2(v22.clone(), v26.clone(), v27);
            let v29: bool = v28 < 0i32;
            let v30: bool = v29 == false;
            if v30 {
                let v31: Rc<str> = note_schema_ensured_3(v22.clone());
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note ("); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v24, &*v1, &*v25, &*v6].concat());
                let v34: i32 = 0i32;
                let v35: i32 = 0i32;
                let v36: i32 = declared_count_4(v31.clone(), v33.clone(), v34, v35);
                let v37: i32 = v36.wrapping_add(1i32);
                let v38: Rc<str> = utc_stamp_5(v4);
                let v39: Rc<str> = note_row_7(v1.clone(), v37, v38.clone(), v2.clone(), v3.clone());
                let v40: Rc<str> = notes_appended_8(v31.clone(), v1.clone(), v39.clone());
                let v41: US1 = state_written_11(v7.clone(), v21.clone(), v40.clone());
                match &v41 {
                    US1::US1_StateWritten => {
                        let v42: u64 = v22.len() as u64;
                        let v43: u64 = v40.len() as u64;
                        let v44: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
                        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile note ok id="); } LIT.with(|lit| lit.clone()) };
                        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v1));
                        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" note="); } LIT.with(|lit| lit.clone()) };
                        let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                        let v49: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
                        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at="); } LIT.with(|lit| lit.clone()) };
                        let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
                        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" author="); } LIT.with(|lit| lit.clone()) };
                        let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v53));
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                        let v57: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v55, &v56));
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                        let v59: Rc<str> = std::rc::Rc::<str>::from(v57.replace(&*v24, &v58));
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                        let v62: Rc<str> = std::rc::Rc::<str>::from(v59.replace(&*v60, &v61));
                        let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                        let v65: Rc<str> = std::rc::Rc::<str>::from(v62.replace(&*v63, &v64));
                        let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                        let v68: Rc<str> = std::rc::Rc::<str>::from(v65.replace(&*v66, &v67));
                        let v69: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v68, &*v24, &*v6, &*v6].concat());
                        let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v69));
                        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" text_bytes="); } LIT.with(|lit| lit.clone()) };
                        let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
                        let v73: i32 = (v3.clone().len() as i32);
                        let v74: Rc<str> = std::rc::Rc::<str>::from((v73).to_string());
                        let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v52, Rc::<str>::from(format!("{}{}", v72, v74))));
                        let v77: Rc<str> = std::rc::Rc::<str>::from((v42).to_string());
                        let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                        let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v77));
                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                        let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
                        let v82: Rc<str> = std::rc::Rc::<str>::from((v43).to_string());
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                        let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                        let v86: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
                        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v76, Rc::<str>::from(format!("{}{}", v85, v86))));
                        let v89: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v88.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                        v89
                    }
                    _ => {
                        let v93: Rc<str> = match &v41 {
                            US1::US1_StateChangedSinceRead => {
                                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                v90.clone()
                            }
                            US1::US1_StateWriteFailed(v91) => {
                                let v91: Rc<str> = v91.clone();
                                v91.clone()
                            }
                            _ => unreachable!(),
                        };
                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                        let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v1));
                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v98: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v95, v96)), v93));
                        let v99: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v98 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v99
                    }
                }
            } else {
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no task with this id"); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v1));
                let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v103, v104)), v101));
                let v107: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v106 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v107
            }
        }
    }
}
fn command_line_argument_count_13() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_12() -> i32 {
    let v0: i32 = command_line_argument_count_13();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
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
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
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
fn note_entry_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: i32 = argument_count_12();
    let v3: bool = v2 == 4i32;
    let (mut v149, mut v150): (Rc<str>, US3) = if v3 {
        let v4: i32 = 3i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: i32 = (v5.clone().len() as i32);
        let v7: i32 = 0i32;
        let v8: i32 = method14(v5.clone(), v6, v7);
        let mut v9: i32 = v6.wrapping_sub(1i32);
        let v10: Rc<str> = string_slice(&v5.clone(), v8 as i64, v9 as i64);
        let v11: i32 = (v10.clone().len() as i32);
        let v12: i32 = method15(v10.clone(), v11);
        let v13: Rc<str> = string_slice(&v10.clone(), 0i32 as i64, v12 as i64);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = std::env::var(&*v14).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
        let v16: i32 = (v15.clone().len() as i32);
        let v17: i32 = 0i32;
        let v18: i32 = method14(v15.clone(), v16, v17);
        let v19: i32 = v16.wrapping_sub(1i32);
        let v20: Rc<str> = string_slice(&v15.clone(), v18 as i64, v19 as i64);
        let v21: i32 = (v20.clone().len() as i32);
        let v22: i32 = method15(v20.clone(), v21);
        let v23: Rc<str> = string_slice(&v20.clone(), 0i32 as i64, v22 as i64);
        let v24: bool = &*v23 == "";
        let v25: bool = v24 == false;
        let v64: US3 = if v25 {
            let v26: Rc<str> = std::env::var(&*v14).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
            let v27: i32 = (v26.clone().len() as i32);
            let v28: i32 = 0i32;
            let v29: i32 = method14(v26.clone(), v27, v28);
            let v30: i32 = v27.wrapping_sub(1i32);
            let v31: Rc<str> = string_slice(&v26.clone(), v29 as i64, v30 as i64);
            let v32: i32 = (v31.clone().len() as i32);
            let v33: i32 = method15(v31.clone(), v32);
            let v34: Rc<str> = string_slice(&v31.clone(), 0i32 as i64, v33 as i64);
            US3::US3_AuthorNamed(v34.clone())
        } else {
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AI_AGENT"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = std::env::var(&*v36).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
            let v38: i32 = (v37.clone().len() as i32);
            let v39: i32 = 0i32;
            let v40: i32 = method14(v37.clone(), v38, v39);
            let v41: i32 = v38.wrapping_sub(1i32);
            let v42: Rc<str> = string_slice(&v37.clone(), v40 as i64, v41 as i64);
            let v43: i32 = (v42.clone().len() as i32);
            let v44: i32 = method15(v42.clone(), v43);
            let v45: Rc<str> = string_slice(&v42.clone(), 0i32 as i64, v44 as i64);
            let v46: bool = &*v45 == "";
            let v47: bool = v46 == false;
            let v60: bool = if v47 {
                true
            } else {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CLAUDECODE"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = std::env::var(&*v48).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let v50: i32 = (v49.clone().len() as i32);
                let v51: i32 = 0i32;
                let v52: i32 = method14(v49.clone(), v50, v51);
                let v53: i32 = v50.wrapping_sub(1i32);
                let v54: Rc<str> = string_slice(&v49.clone(), v52 as i64, v53 as i64);
                let v55: i32 = (v54.clone().len() as i32);
                let v56: i32 = method15(v54.clone(), v55);
                let v57: Rc<str> = string_slice(&v54.clone(), 0i32 as i64, v56 as i64);
                let v58: bool = &*v57 == "";
                let v59: bool = v58 == false;
                v59
            };
            if v60 {
                US3::US3_AuthorUnnamedAgent
            } else {
                US3::US3_AuthorDefault
            }
        };
        (v13.clone(), v64.clone())
    } else {
        let v65: bool = v2 == 6i32;
        let v69: bool = if v65 {
            let v66: i32 = 4i32;
            let v67: Rc<str> = usize::try_from(v66).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v68: bool = &*v67 == "--as";
            v68
        } else {
            false
        };
        let (mut v147, mut v148): (Rc<str>, US3) = if v69 {
            let v70: i32 = 3i32;
            let v71: Rc<str> = usize::try_from(v70).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v72: i32 = (v71.clone().len() as i32);
            let v73: i32 = 0i32;
            let v74: i32 = method14(v71.clone(), v72, v73);
            let v75: i32 = v72.wrapping_sub(1i32);
            let v76: Rc<str> = string_slice(&v71.clone(), v74 as i64, v75 as i64);
            let v77: i32 = (v76.clone().len() as i32);
            let v78: i32 = method15(v76.clone(), v77);
            let v79: Rc<str> = string_slice(&v76.clone(), 0i32 as i64, v78 as i64);
            let v80: i32 = 5i32;
            let v81: Rc<str> = usize::try_from(v80).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v82: i32 = (v81.clone().len() as i32);
            let v83: i32 = 0i32;
            let v84: i32 = method14(v81.clone(), v82, v83);
            let v85: i32 = v82.wrapping_sub(1i32);
            let v86: Rc<str> = string_slice(&v81.clone(), v84 as i64, v85 as i64);
            let v87: i32 = (v86.clone().len() as i32);
            let v88: i32 = method15(v86.clone(), v87);
            let v89: Rc<str> = string_slice(&v86.clone(), 0i32 as i64, v88 as i64);
            let v90: bool = &*v89 == "";
            let v99: US3 = if v90 {
                US3::US3_AuthorMalformed
            } else {
                let v92: i32 = 0i32;
                let v93: i32 = method14(v81.clone(), v82, v92);
                let v94: Rc<str> = string_slice(&v81.clone(), v93 as i64, v85 as i64);
                let v95: i32 = (v94.clone().len() as i32);
                let v96: i32 = method15(v94.clone(), v95);
                let v97: Rc<str> = string_slice(&v94.clone(), 0i32 as i64, v96 as i64);
                US3::US3_AuthorNamed(v97.clone())
            };
            (v79.clone(), v99.clone())
        } else {
            let v103: bool = if v65 {
                let v100: i32 = 3i32;
                let v101: Rc<str> = usize::try_from(v100).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v102: bool = &*v101 == "--as";
                v102
            } else {
                false
            };
            let (mut v145, mut v146): (Rc<str>, US3) = if v103 {
                let v104: i32 = 5i32;
                let v105: Rc<str> = usize::try_from(v104).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v106: i32 = (v105.clone().len() as i32);
                let v107: i32 = 0i32;
                let v108: i32 = method14(v105.clone(), v106, v107);
                let v109: i32 = v106.wrapping_sub(1i32);
                let v110: Rc<str> = string_slice(&v105.clone(), v108 as i64, v109 as i64);
                let v111: i32 = (v110.clone().len() as i32);
                let v112: i32 = method15(v110.clone(), v111);
                let v113: Rc<str> = string_slice(&v110.clone(), 0i32 as i64, v112 as i64);
                let v114: i32 = 4i32;
                let v115: Rc<str> = usize::try_from(v114).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v116: i32 = (v115.clone().len() as i32);
                let v117: i32 = 0i32;
                let v118: i32 = method14(v115.clone(), v116, v117);
                let v119: i32 = v116.wrapping_sub(1i32);
                let v120: Rc<str> = string_slice(&v115.clone(), v118 as i64, v119 as i64);
                let v121: i32 = (v120.clone().len() as i32);
                let v122: i32 = method15(v120.clone(), v121);
                let v123: Rc<str> = string_slice(&v120.clone(), 0i32 as i64, v122 as i64);
                let v124: bool = &*v123 == "";
                let v133: US3 = if v124 {
                    US3::US3_AuthorMalformed
                } else {
                    let v126: i32 = 0i32;
                    let v127: i32 = method14(v115.clone(), v116, v126);
                    let v128: Rc<str> = string_slice(&v115.clone(), v127 as i64, v119 as i64);
                    let v129: i32 = (v128.clone().len() as i32);
                    let v130: i32 = method15(v128.clone(), v129);
                    let v131: Rc<str> = string_slice(&v128.clone(), 0i32 as i64, v130 as i64);
                    US3::US3_AuthorNamed(v131.clone())
                };
                (v113.clone(), v133.clone())
            } else {
                let v134: i32 = 3i32;
                let v135: Rc<str> = usize::try_from(v134).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v136: i32 = (v135.clone().len() as i32);
                let v137: i32 = 0i32;
                let v138: i32 = method14(v135.clone(), v136, v137);
                let v139: i32 = v136.wrapping_sub(1i32);
                let v140: Rc<str> = string_slice(&v135.clone(), v138 as i64, v139 as i64);
                let v141: i32 = (v140.clone().len() as i32);
                let v142: i32 = method15(v140.clone(), v141);
                let v143: Rc<str> = string_slice(&v140.clone(), 0i32 as i64, v142 as i64);
                let v144: US3 = US3::US3_AuthorMalformed;
                (v143.clone(), v144.clone())
            };
            (v145.clone(), v146.clone())
        };
        (v147.clone(), v148.clone())
    };
    let v151: bool = &*v149 == "";
    if v151 {
        let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the note text is empty"); } LIT.with(|lit| lit.clone()) };
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v153, v1));
        let v155: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v157: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v154, v155)), v152));
        let v158: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v157 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v158
    } else {
        match &v150 {
            US3::US3_AuthorDefault => {
                let v159: i32 = 1i32;
                let v160: Rc<str> = usize::try_from(v159).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("user"); } LIT.with(|lit| lit.clone()) };
                note_added_1(v160.clone(), v1.clone(), v161.clone(), v149.clone())
            }
            US3::US3_AuthorMalformed => {
                let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile note <root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                let v175: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v1));
                let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v179: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v176, v177)), v174));
                let v180: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v179 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v180
            }
            US3::US3_AuthorNamed(v163) => {
                let v163: Rc<str> = v163.clone();
                let v164: i32 = 1i32;
                let v165: Rc<str> = usize::try_from(v164).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                note_added_1(v165.clone(), v1.clone(), v163.clone(), v149.clone())
            }
            US3::US3_AuthorUnnamedAgent => {
                let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("an agent session must name the note's author: agile note <root> <id> <text> --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile note "); } LIT.with(|lit| lit.clone()) };
                let v169: Rc<str> = Rc::<str>::from(format!("{}{}", v168, v1));
                let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v172: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v169, v170)), v167));
                let v173: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v172 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v173
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        note_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn planned_17(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US4 {
    match &v0 {
        US4::US4_GroomedRefused(v4) => {
            let v4: Rc<str> = v4.clone();
            v0.clone()
        }
        US4::US4_GroomedText(v5) => {
            let v5: Rc<str> = v5.clone();
            let v6: bool = &*v3 == "";
            if v6 {
                v0
            } else {
                let (mut v7, mut v8): (i32, Rc<str>) = agile_planning_values::kit_agile_planning_value_token(&*v2, &*v3);
                let mut v9: bool = v7 == 0i32;
                let v12: US4 = if v9 {
                    US4::US4_GroomedText(v8.clone())
                } else {
                    US4::US4_GroomedRefused(v8)
                };
                match &v12 {
                    US4::US4_GroomedRefused(v13) => {
                        let v13: Rc<str> = v13.clone();
                        US4::US4_GroomedRefused(v13)
                    }
                    US4::US4_GroomedText(v15) => {
                        let v15: Rc<str> = v15.clone();
                        let (mut v16, mut v17): (i32, Rc<str>) = agile_planning_rewrite::kit_agile_planning_rewrite(&*v5, &*v1, &*v2, &*v15);
                        let v18: bool = v16 == 0i32;
                        if v18 {
                            US4::US4_GroomedText(v17.clone())
                        } else {
                            US4::US4_GroomedRefused(v17)
                        }
                    }
                }
            }
        }
    }
}
fn groomed_text_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>) -> (i32, Rc<str>) {
    let v7: US4 = US4::US4_GroomedText(v0.clone());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
    let mut v9: US4 = planned_17(v7.clone(), v1.clone(), v8.clone(), v2.clone());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
    let v11: US4 = planned_17(v9.clone(), v1.clone(), v10.clone(), v3.clone());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
    let v13: US4 = planned_17(v11.clone(), v1.clone(), v12.clone(), v4.clone());
    match &v13 {
        US4::US4_GroomedRefused(v14) => {
            let v14: Rc<str> = v14.clone();
            (1i32, v14.clone())
        }
        US4::US4_GroomedText(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: bool = &*v5 == "";
            if v16 {
                (0i32, v15.clone())
            } else {
                let v17: Rc<str> = note_schema_ensured_3(v15.clone());
                let v18: i32 = 1i32;
                let v19: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                let v20: Rc<str> = utc_stamp_5(v19);
                let v21: Rc<str> = note_row_7(v1.clone(), v18, v20.clone(), v6.clone(), v5.clone());
                let v22: Rc<str> = notes_appended_8(v17.clone(), v1.clone(), v21.clone());
                (0i32, v22.clone())
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (i32, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>| -> (i32, Rc<str>) {
        groomed_text_16(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> Rc<str>> = Rc::new(move |mut v0: u64| -> Rc<str> {
        utc_stamp_5(v0)
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
