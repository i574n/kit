#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US0 {
    US0_CountMatches,
    US0_SumCaptures,
    US0_PathField,
    US0_CountByCapture,
    US0_LastCapture,
    US0_ValuesPerMatch,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoMeasures,
    UH0_MeasureThen(US0, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_RequestTaken(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<UH0>),
    US1_RequestRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_KindKnown(US0),
    US2_KindUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_SpecTaken(Rc<UH0>),
    US3_SpecRejected(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_PerFile,
    US4_PerMatch(Rc<str>, Rc<str>),
    US4_MixedShapes,
}
fn stderr_line_2(mut v0: Rc<str>) -> () {
    eprintln!("{}", v0);
    ()
}
fn failed_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: table logs "); } LIT.with(|lit| lit.clone()) }, v0));
    stderr_line_2(v1);
    2i32
}
fn stdout_text_4(mut v0: Rc<str>) -> () {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.ends_with(&*v1);
    let v8: Rc<str> = if v2 {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: i32 = v3.wrapping_sub(1i32);
        let v5: usize = ((0i32) as usize);
        let v6: usize = ((v4) as usize);
        let v7: Rc<str> = Rc::<str>::from(&(v0)[v5..v6]);
        v7
    } else {
        v0
    };
    println!("{}", v8);
    ()
}
fn whole_spi_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_table_text::kit_table_spi_of_tsv(&*v0);
    v1
}
fn text_written_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::fs::write(&*v0, (v1).as_bytes()).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default().as_str());
    v2
}
fn stdout_line_7(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn table_written_3(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: bool = &*v0 == "";
    let v7: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" out="); } LIT.with(|lit| lit.clone()) }, v0.clone()));
        v6
    };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v7));
    if v4 {
        stdout_text_4(v2);
        if v1 {
            ()
        } else {
            stderr_line_2(v8)
        };
        0i32
    } else {
        let v9: Rc<str> = Rc::<str>::from(v0.to_lowercase());
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spi"); } LIT.with(|lit| lit.clone()) };
        let v11: bool = v9.ends_with(&*v10);
        let v13: Rc<str> = if v11 {
            whole_spi_5(v2)
        } else {
            v2
        };
        let v14: Rc<str> = text_written_6(v0, v13);
        let v15: bool = &*v14 != "";
        if v15 {
            failed_1(v14)
        } else {
            if v1 {
                ()
            } else {
                stdout_line_7(v8)
            };
            0i32
        }
    }
}
fn files_listed_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ fn wildcard(pattern: &[u8], name: &[u8]) -> bool { match (pattern.first(), name.first()) { (None, None) => true, (Some(&42u8), _) => wildcard(&pattern[1..], name) || (!name.is_empty() && wildcard(pattern, &name[1..])), (Some(&63u8), Some(_)) => wildcard(&pattern[1..], &name[1..]), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b) => wildcard(&pattern[1..], &name[1..]), _ => false } } fn walk(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) { if path.is_dir() { let mut entries = std::fs::read_dir(path).into_iter().flatten().flatten().map(|entry| entry.path()).collect::<Vec<_>>(); entries.sort(); for entry in entries { walk(&entry, found); } } else if path.is_file() { found.push(path.to_path_buf()); } } let glob = &*v1; let mut lines = Vec::new(); for raw in (v0).split(char::from(10u8)).filter(|item| !item.is_empty()) { let path = std::path::Path::new(raw); if !path.exists() { lines.push(format!("{}{raw}", char::from(1u8))); continue; } let mut found = Vec::new(); walk(path, &mut found); for file in found { let name = file.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(); if glob.is_empty() || wildcard(glob.as_bytes(), name.as_bytes()) { lines.push(file.to_string_lossy().replace(char::from(92u8), "/")); } } } lines.join("
") }.as_str());
    v2
}
fn first_of_11(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).next().unwrap_or(""));
    v1
}
fn missing_marker_12(mut v0: Rc<str>) -> bool {
    let v1: bool = (v0).starts_with(char::from(1u8));
    v1
}
fn rest_of_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
    v1
}
fn file_text_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8_lossy(&std::fs::read(&*v0).unwrap_or_default()).as_ref());
    v1
}
fn text_since_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        let v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|marker| marker.find(&*v0)).map(|found| (&(v0)[found.end()..]).split_once(char::from(10u8)).map(|(_, after)| after).unwrap_or("")).unwrap_or(""));
        v3
    }
}
fn match_end_from_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let v3: i32 = regex::Regex::new(&*v1).ok().and_then(|p| p.find_at(&*v0, v2 as usize)).map(|m| m.end().max(v2 as usize + 1) as i32).unwrap_or(-1);
    v3
}
fn capture_from_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|p| p.captures_at(&*v0, v2 as usize)).and_then(|f| f.get(1).or(f.get(0))).map(|g| g.as_str()).unwrap_or(""));
    v3
}
fn matched_rows_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> (Rc<str>, i32) {
    loop {
        let v6: i32 = match_end_from_17(v1.clone(), v2.clone(), v3);
        let v7: bool = v6 < 0i32;
        let v10: bool = if v7 {
            true
        } else {
            let v8: i32 = (v1.clone().len() as i32);
            let v9: bool = v6 > v8;
            v9
        };
        if v10 {
            return (v4, v5);
        } else {
            let v11: Rc<str> = capture_from_18(v1.clone(), v2.clone(), v3);
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v0.clone()));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v14));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v19: i32 = v5.wrapping_add(1i32);
            (v3, v4, v5) = (v6, v18, v19);
            continue;
        }
    }
}
fn values_rows_10(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: i32, mut v6: Rc<str>) -> (Rc<str>, i32, i32, Rc<str>) {
    loop {
        let v7: bool = &*v2 == "";
        if v7 {
            return (v3, v4, v5, v6);
        } else {
            let v8: Rc<str> = first_of_11(v2.clone());
            let v9: bool = missing_marker_12(v8.clone());
            if v9 {
                let v10: Rc<str> = rest_of_13(v2);
                let v11: bool = &*v6 == "";
                let v14: Rc<str> = if v11 {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v12
                } else {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    v13
                };
                let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v14));
                let v16: Rc<str> = std::rc::Rc::<str>::from(&(v8)[1..]);
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                (v2, v6) = (v10, v17);
                continue;
            } else {
                let v22: Rc<str> = file_text_14(v8.clone());
                let v23: Rc<str> = text_since_15(v22, v1.clone());
                let v24: i32 = 0i32;
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v26: i32 = 0i32;
                let (mut v27, mut v28): (Rc<str>, i32) = matched_rows_16(v8, v23, v0.clone(), v24, v25, v26);
                let v29: Rc<str> = rest_of_13(v2);
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v27));
                let v31: i32 = v4.wrapping_add(1i32);
                let v32: i32 = v5.wrapping_add(v28);
                (v2, v3, v4, v5) = (v29, v30, v31, v32);
                continue;
            }
        }
    }
}
fn method21(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_20(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method21(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn count_text_19(mut v0: i32) -> Rc<str> {
    format_real_20(v0)
}
fn values_run_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: Rc<str>) -> i32 {
    let v7: Rc<str> = files_listed_9(v0, v1);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v9: i32 = 0i32;
    let v10: i32 = 0i32;
    let (mut v11, mut v12, mut v13, mut v14): (Rc<str>, i32, i32, Rc<str>) = values_rows_10(v6, v2, v7, v8.clone(), v9, v10, v8);
    let v15: bool = &*v14 != "";
    if v15 {
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) }, v14));
        failed_1(v16)
    } else {
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file\t"); } LIT.with(|lit| lit.clone()) }, v5));
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v11));
        let v21: Rc<str> = count_text_19(v12);
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table logs ok files="); } LIT.with(|lit| lit.clone()) }, v21));
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rows="); } LIT.with(|lit| lit.clone()) }));
        let v24: Rc<str> = count_text_19(v13);
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns=1"); } LIT.with(|lit| lit.clone()) }));
        table_written_3(v3, v4, v20, v26)
    }
}
fn kind_of_24(mut v0: Rc<str>) -> US2 {
    let v1: bool = &*v0 == "--count";
    if v1 {
        let v2: US0 = US0::US0_CountMatches;
        US2::US2_KindKnown(v2)
    } else {
        let v4: bool = &*v0 == "--sum";
        if v4 {
            let v5: US0 = US0::US0_SumCaptures;
            US2::US2_KindKnown(v5)
        } else {
            let v7: bool = &*v0 == "--field";
            if v7 {
                let v8: US0 = US0::US0_PathField;
                US2::US2_KindKnown(v8)
            } else {
                let v10: bool = &*v0 == "--count-by";
                if v10 {
                    let v11: US0 = US0::US0_CountByCapture;
                    US2::US2_KindKnown(v11)
                } else {
                    let v13: bool = &*v0 == "--last";
                    if v13 {
                        let v14: US0 = US0::US0_LastCapture;
                        US2::US2_KindKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "--values";
                        if v16 {
                            let v17: US0 = US0::US0_ValuesPerMatch;
                            US2::US2_KindKnown(v17)
                        } else {
                            US2::US2_KindUnknown
                        }
                    }
                }
            }
        }
    }
}
fn name_ok_26(mut v0: Rc<str>) -> bool {
    let v1: bool = !(v0).is_empty() && (v0).chars().all(|c| c.is_ascii_alphanumeric() || c == char::from(95u8) || c == char::from(45u8));
    v1
}
fn measures_appended_27(mut v0: Rc<UH0>, mut v1: US0, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v7, v8, v9, v10, v11) => {
            let v7: US0 = v7;
            let v8: Rc<str> = v8;
            let v9: Rc<str> = v9;
            let v10: Rc<str> = v10;
            let v11: Rc<UH0> = v11;
            let v12: Rc<UH0> = measures_appended_27(v11, v1, v2, v3);
            Rc::new(UH0::UH0_MeasureThen(v7, v8, v9, v10, v12))
        }
        UH0::UH0_NoMeasures => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) };
            Rc::new(UH0::UH0_MeasureThen(v1, v2, v3, v4, v5))
        }
    }
}
fn spec_applied_25(mut v0: Rc<UH0>, mut v1: US0, mut v2: Rc<str>) -> US3 {
    let v3: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(61u8)).map(|(name, _)| name).unwrap_or(""));
    let v4: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(61u8)).map(|(_, pattern)| pattern).unwrap_or(""));
    let v5: bool = name_ok_26(v3.clone());
    let v6: bool = v5 == false;
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v4 == "";
        v7
    };
    if v8 {
        let v20: Rc<str> = match v1 {
            US0::US0_CountByCapture => {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count-by"); } LIT.with(|lit| lit.clone()) };
                v12
            }
            US0::US0_CountMatches => {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--count"); } LIT.with(|lit| lit.clone()) };
                v9
            }
            US0::US0_LastCapture => {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--last"); } LIT.with(|lit| lit.clone()) };
                v13
            }
            US0::US0_PathField => {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--field"); } LIT.with(|lit| lit.clone()) };
                v11
            }
            US0::US0_SumCaptures => {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sum"); } LIT.with(|lit| lit.clone()) };
                v10
            }
            US0::US0_ValuesPerMatch => {
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values"); } LIT.with(|lit| lit.clone()) };
                v14
            }
        };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects name=regex with a letters/digits/_/- name, received '"); } LIT.with(|lit| lit.clone()) }));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v2));
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) }));
        US3::US3_SpecRejected(v23)
    } else {
        let v25: bool = &*v3 == "file";
        if v25 {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the column name 'file' is taken by the file path"); } LIT.with(|lit| lit.clone()) };
            US3::US3_SpecRejected(v26)
        } else {
            let v28: Rc<UH0> = measures_appended_27(v0, v1, v3, v4);
            US3::US3_SpecTaken(v28)
        }
    }
}
fn measures_applied_23(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: &Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: bool, mut v8: Rc<UH0>) -> US1 {
    loop {
        let v9: i32 = (v0.clone().borrow().len() as i32);
        let v10: bool = v2 >= v9;
        if v10 {
            return US1::US1_RequestTaken(v3, v4, v5, v6, v7, v8);
        } else {
            let v12: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v13: US2 = kind_of_24(v12.clone());
            match v13 {
                US2::US2_KindKnown(v16) => {
                    let v16: US0 = v16;
                    let v17: Rc<str> = v1.borrow()[v2 as usize].clone();
                    let v18: US3 = spec_applied_25(v8, v16, v17);
                    match v18 {
                        US3::US3_SpecRejected(v19) => {
                            let v19: Rc<str> = v19;
                            return US1::US1_RequestRejected(v19);
                        }
                        US3::US3_SpecTaken(v21) => {
                            let v21: Rc<UH0> = v21;
                            let v22: i32 = v2.wrapping_add(1i32);
                            (v2, v8) = (v22, v21);
                            continue;
                        }
                    }
                }
                US2::US2_KindUnknown => {
                    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v12));
                    return US1::US1_RequestRejected(v14);
                }
            }
        }
    }
}
fn request_of_22(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: &Rc<RefCell<Vec<Rc<str>>>>, mut v7: &Rc<RefCell<Vec<Rc<str>>>>) -> US1 {
    let v8: bool = &*v5 != "";
    if v8 {
        US1::US1_RequestRejected(v5)
    } else {
        let v10: i32 = 0i32;
        let v11: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) };
        measures_applied_23(v6, v7, v10, v0, v1, v3, v2, v4, v11)
    }
}
fn measure_count_28(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let v2: US0 = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v7: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v6, v7);
                continue;
            }
            UH0::UH0_NoMeasures => {
                return v1;
            }
        }
    }
}
fn pattern_error_30(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v0).err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1
}
fn pattern_problem_29(mut v0: Rc<UH0>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let v2: US0 = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<str> = pattern_error_30(v4);
                let v8: bool = &*v7 != "";
                if v8 {
                    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid pattern for "); } LIT.with(|lit| lit.clone()) }, v3));
                    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) }));
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v7));
                    return v11;
                } else {
                    v0 = v6;
                    continue;
                }
            }
            UH0::UH0_NoMeasures => {
                let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v1;
            }
        }
    }
}
fn values_measure_32(mut v0: Rc<UH0>) -> (Rc<str>, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
                let v2: US0 = v2;
                let v3: Rc<str> = v3;
                let v4: Rc<str> = v4;
                let v5: Rc<str> = v5;
                let v6: Rc<UH0> = v6;
                match v2 {
                    US0::US0_ValuesPerMatch => {
                        return (v3, v4);
                    }
                    _ => {
                        v0 = v6;
                        continue;
                    }
                }
            }
            UH0::UH0_NoMeasures => {
                let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return (v1.clone(), v1.clone());
            }
        }
    }
}
fn shape_of_31(mut v0: Rc<UH0>) -> US4 {
    let (mut v1, mut v2): (Rc<str>, Rc<str>) = values_measure_32(v0.clone());
    let v3: bool = &*v1 == "";
    if v3 {
        US4::US4_PerFile
    } else {
        let v5: i32 = 0i32;
        let v6: i32 = measure_count_28(v0, v5);
        let v7: bool = v6 == 1i32;
        if v7 {
            US4::US4_PerMatch(v1, v2)
        } else {
            US4::US4_MixedShapes
        }
    }
}
fn value_measures_34(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH0>) -> Rc<UH0> {
    let v4: bool = &*v2 == "";
    if v4 {
        v3
    } else {
        let v5: Rc<str> = std::rc::Rc::<str>::from((v2).split(char::from(10u8)).next().unwrap_or(""));
        let v6: Rc<str> = std::rc::Rc::<str>::from((v2).split_once(char::from(10u8)).map(|(_, rest)| rest).unwrap_or(""));
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) }));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5.clone()));
        let v9: Rc<UH0> = value_measures_34(v0, v1.clone(), v6, v3);
        let v10: US0 = US0::US0_CountByCapture;
        Rc::new(UH0::UH0_MeasureThen(v10, v8, v1, v5, v9))
    }
}
fn capture_values_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).map(|pattern| { let mut values = std::collections::BTreeSet::new(); for file in (v0).split(char::from(10u8)).filter(|line| !line.is_empty() && !line.starts_with(char::from(1u8))) { let text = String::from_utf8_lossy(&std::fs::read(file).unwrap_or_default()).into_owned(); for found in pattern.captures_iter(&text) { if let Some(group) = found.get(1) { values.insert(group.as_str().replace(char::from(9u8), " ").replace(char::from(10u8), " ")); } } } values.into_iter().collect::<Vec<_>>().join("
") }).unwrap_or_default().as_str());
    v2
}
fn expanded_33(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v3, v4, v5, v6, v7) => {
            let v3: US0 = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<UH0> = v7;
            match &v3 {
                US0::US0_CountByCapture => {
                    let v8: Rc<str> = capture_values_35(v1.clone(), v5.clone());
                    let v9: Rc<UH0> = expanded_33(v7, v1);
                    value_measures_34(v4, v5, v8, v9)
                }
                _ => {
                    let v11: Rc<UH0> = expanded_33(v7, v1);
                    Rc::new(UH0::UH0_MeasureThen(v3, v4, v5, v6, v11))
                }
            }
        }
        UH0::UH0_NoMeasures => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoMeasures); } CASE.with(|case| case.clone()) }
        }
    }
}
fn value_count_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: i32 = regex::Regex::new(&*v1).map(|pattern| pattern.captures_iter(&*v0).filter(|found| found.get(1).map(|group| group.as_str() == &*v2).unwrap_or(false)).count() as i32).unwrap_or(0);
    v3
}
fn match_count_39(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: i32 = regex::Regex::new(&*v1).map(|pattern| pattern.find_iter(&*v0).count() as i32).unwrap_or(0);
    v2
}
fn capture_at_40(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).ok().and_then(|pattern| { let mut found = pattern.captures_iter(&*v0); if v2 { found.last() } else { found.next() } }.and_then(|found| found.get(1).or_else(|| found.get(0)).map(|group| group.as_str().replace(char::from(9u8), " ")))).unwrap_or_default().as_str());
    v3
}
fn capture_sum_41(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v1).map(|pattern| { let sum: f64 = pattern.captures_iter(&*v0).filter_map(|found| found.get(1).and_then(|group| group.as_str().trim().parse::<f64>().ok())).sum(); let text = format!("{:.6}", sum + 0.0); text.trim_end_matches(char::from(48u8)).trim_end_matches(char::from(46u8)).to_owned() }).unwrap_or_default().as_str());
    v2
}
fn row_cells_37(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v4, v5, v6, v7, v8) => {
            let v4: US0 = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: Rc<UH0> = v8;
            let v24: Rc<str> = match v4 {
                US0::US0_CountByCapture => {
                    let v11: i32 = value_count_38(v2.clone(), v6, v7);
                    count_text_19(v11)
                }
                US0::US0_CountMatches => {
                    let v9: i32 = match_count_39(v2.clone(), v6);
                    count_text_19(v9)
                }
                US0::US0_LastCapture => {
                    let v16: bool = true;
                    capture_at_40(v2.clone(), v6, v16)
                }
                US0::US0_PathField => {
                    let v14: bool = false;
                    capture_at_40(v1.clone(), v6, v14)
                }
                US0::US0_SumCaptures => {
                    capture_sum_41(v2.clone(), v6)
                }
                US0::US0_ValuesPerMatch => {
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v18
                }
            };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }, v24));
            let v26: Rc<str> = row_cells_37(v8, v1, v2);
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            v27
        }
        UH0::UH0_NoMeasures => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3
        }
    }
}
fn rows_of_36(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>) -> (Rc<str>, i32, Rc<str>) {
    loop {
        let v6: bool = &*v2 == "";
        if v6 {
            return (v3, v4, v5);
        } else {
            let v7: Rc<str> = first_of_11(v2.clone());
            let v8: bool = missing_marker_12(v7.clone());
            if v8 {
                let v9: Rc<str> = rest_of_13(v2);
                let v10: bool = &*v5 == "";
                let v13: Rc<str> = if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11
                } else {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    v12
                };
                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v13));
                let v15: Rc<str> = std::rc::Rc::<str>::from(&(v7)[1..]);
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                (v2, v5) = (v9, v16);
                continue;
            } else {
                let v20: Rc<str> = rest_of_13(v2);
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v7.clone()));
                let v22: Rc<str> = file_text_14(v7.clone());
                let v23: Rc<str> = text_since_15(v22, v1.clone());
                let v24: Rc<str> = row_cells_37(v0.clone(), v7, v23);
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v24));
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                let v27: i32 = v4.wrapping_add(1i32);
                (v2, v3, v4) = (v20, v26, v27);
                continue;
            }
        }
    }
}
fn measure_header_42(mut v0: Rc<UH0>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_MeasureThen(v2, v3, v4, v5, v6) => {
            let v2: US0 = v2;
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: Rc<str> = v5;
            let v6: Rc<UH0> = v6;
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }, v3));
            let v8: Rc<str> = measure_header_42(v6);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
            v9
        }
        UH0::UH0_NoMeasures => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn logs_run_0() -> i32 {
    let mut v0: Rc<RefCell<Vec<(Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = kit_table_logs_args::kit_table_logs_args_read(2i32);
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, bool, Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, Rc<RefCell<Vec<Rc<str>>>>) = v0.borrow()[0i32 as usize].clone();
    let v9: US1 = request_of_22(v1, v2, v3, v4, v5, v6, &v7, &v8);
    match v9 {
        US1::US1_RequestRejected(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match; to pair a per-file id with a value, use --field for the id and --last for the value) [--out file.tsv|file.spi] [--quiet]"); } LIT.with(|lit| lit.clone()) }));
            failed_1(v12)
        }
        US1::US1_RequestTaken(v14, v15, v16, v17, v18, v19) => {
            let v14: Rc<str> = v14;
            let v15: Rc<str> = v15;
            let v16: Rc<str> = v16;
            let v17: Rc<str> = v17;
            let v18: bool = v18;
            let v19: Rc<UH0> = v19;
            let v20: bool = &*v14 == "";
            if v20 {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one file or directory; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match; to pair a per-file id with a value, use --field for the id and --last for the value) [--out file.tsv|file.spi] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                failed_1(v21)
            } else {
                let v23: i32 = 0i32;
                let v24: i32 = measure_count_28(v19.clone(), v23);
                let v25: bool = v24 == 0i32;
                if v25 {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("needs at least one --count, --count-by, --sum, --last or --field; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match; to pair a per-file id with a value, use --field for the id and --last for the value) [--out file.tsv|file.spi] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                    failed_1(v26)
                } else {
                    let v28: Rc<str> = pattern_problem_29(v19.clone());
                    let v29: bool = &*v28 != "";
                    if v29 {
                        let v30: Rc<str> = pattern_problem_29(v19);
                        failed_1(v30)
                    } else {
                        let v32: bool = &*v17 != "";
                        let v35: bool = if v32 {
                            let v33: Rc<str> = pattern_error_30(v17.clone());
                            let v34: bool = &*v33 != "";
                            v34
                        } else {
                            false
                        };
                        if v35 {
                            let v36: Rc<str> = pattern_error_30(v17);
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid --since pattern: "); } LIT.with(|lit| lit.clone()) }, v36));
                            failed_1(v37)
                        } else {
                            let v39: US4 = shape_of_31(v19.clone());
                            match v39 {
                                US4::US4_MixedShapes => {
                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--values makes one row per match and cannot share the table with other measures; usage: table logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match; to pair a per-file id with a value, use --field for the id and --last for the value) [--out file.tsv|file.spi] [--quiet]"); } LIT.with(|lit| lit.clone()) };
                                    failed_1(v40)
                                }
                                US4::US4_PerFile => {
                                    let v45: Rc<str> = files_listed_9(v14, v15);
                                    let v46: Rc<UH0> = expanded_33(v19, v45.clone());
                                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v48: i32 = 0i32;
                                    let (mut v49, mut v50, mut v51): (Rc<str>, i32, Rc<str>) = rows_of_36(v46.clone(), v17, v45, v47.clone(), v48, v47.clone());
                                    let v52: bool = &*v51 != "";
                                    if v52 {
                                        let v53: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no such file or directory: "); } LIT.with(|lit| lit.clone()) }, v51));
                                        failed_1(v53)
                                    } else {
                                        let v55: Rc<str> = measure_header_42(v46.clone());
                                        let v56: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file"); } LIT.with(|lit| lit.clone()) }, v55));
                                        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                                        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v49));
                                        let v59: Rc<str> = count_text_19(v50);
                                        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit table logs ok files="); } LIT.with(|lit| lit.clone()) }, v59));
                                        let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" columns="); } LIT.with(|lit| lit.clone()) }));
                                        let v62: i32 = 0i32;
                                        let v63: i32 = measure_count_28(v46, v62);
                                        let v64: Rc<str> = count_text_19(v63);
                                        let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v64));
                                        table_written_3(v16, v18, v58, v65)
                                    }
                                }
                                US4::US4_PerMatch(v42, v43) => {
                                    let v42: Rc<str> = v42;
                                    let v43: Rc<str> = v43;
                                    values_run_8(v14, v15, v17, v16, v18, v42, v43)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        logs_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_table_logs_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
