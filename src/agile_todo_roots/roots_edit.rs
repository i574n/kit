#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Replaced,
    US0_TemporaryWriteFailed,
    US0_RenameFailed,
    US0_RenameFailedTemporaryKept,
    US0_ReadbackMismatch,
}
#[derive(Clone)]
enum US1 {
    US1_RootsEdited(Rc<str>),
    US1_RootsRefused(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
fn harvest_roots_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return v2.clone();
        } else {
            let v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9.clone()
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v11, &*v9, &*v12, &*v12].concat());
                v13.clone()
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v15, v14.clone());
            continue;
        }
    }
}
fn roots_printed_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("root "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            let v12: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v11.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let v13: i32 = v8.wrapping_add(1i32);
            let v14: i32 = v2.wrapping_add(1i32);
            (v0, v1, v2) = (v0.clone(), v13, v14);
            continue;
        }
    }
}
fn roots_receipt_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = harvest_roots_from_2(v1.clone(), v3, v4.clone());
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = roots_printed_3(v5.clone(), v6, v7);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile roots ok action="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v0));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" roots="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let v13: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v2));
    let v16: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v15.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v16
}
fn roots_written_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
    let v7: bool = std::fs::write(&*v6, &*v3).is_ok();
    let v19: US0 = if v7 {
        let v8: bool = std::fs::rename(&*v6, &*v0).is_ok();
        if v8 {
            let v9: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v3);
            if v9 {
                US0::US0_Replaced
            } else {
                US0::US0_ReadbackMismatch
            }
        } else {
            let v13: bool = std::fs::remove_file(&*v6).is_ok() || !std::path::Path::new(&*v6).exists();
            if v13 {
                US0::US0_RenameFailed
            } else {
                US0::US0_RenameFailedTemporaryKept
            }
        }
    } else {
        US0::US0_TemporaryWriteFailed
    };
    match &v19 {
        US0::US0_Replaced => {
            let v20: u64 = v2.len() as u64;
            let v21: u64 = v3.len() as u64;
            let v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
            let v23: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let v28: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
            let v32: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
            roots_receipt_1(v1.clone(), v3.clone(), v33.clone())
        }
        _ => {
            let v42: Rc<str> = match &v19 {
                US0::US0_ReadbackMismatch => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v38.clone()
                }
                US0::US0_RenameFailed => {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v36.clone()
                }
                US0::US0_RenameFailedTemporaryKept => {
                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v37.clone()
                }
                US0::US0_TemporaryWriteFailed => {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v35.clone()
                }
                _ => unreachable!(),
            };
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
            let v45: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v44 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v45
        }
    }
}
fn command_line_argument_count_7() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_6() -> i32 {
    let v0: i32 = command_line_argument_count_7();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn rows_end_from_10(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        let v7: i32 = if v4 {
            let v5: i32 = (v0.clone().len() as i32);
            v5
        } else {
            let v6: i32 = v3.wrapping_add(1i32);
            v6
        };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v7 as usize).unwrap_or(""));
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v8.contains(&*v9);
        if v10 {
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn root_added_9(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: bool = &*v2 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root needs a path"); } LIT.with(|lit| lit.clone()) };
        US1::US1_RootsRefused(v4.clone())
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v2.contains(&*v6);
        if v7 {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root cannot contain a double quote"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RootsRefused(v8.clone())
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v2, &*v11, &*v12, &*v12].concat());
            let v14: bool = v0.contains(&*v13);
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already a harvest root"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v15, &*v12, &*v12, &*v12].concat());
                US1::US1_RootsRefused(v16.clone())
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root ="); } LIT.with(|lit| lit.clone()) };
                let v19: bool = v0.contains(&*v18);
                let v34: Rc<str> = if v19 {
                    v0.clone()
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v21: bool = v0.starts_with(&*v20);
                    let v28: i32 = if v21 {
                        0i32
                    } else {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
                        let v23: i32 = v0.find(&*v22).map(|offset| offset as i32).unwrap_or(-1);
                        let v24: bool = v23 < 0i32;
                        if v24 {
                            let v25: i32 = (v0.clone().len() as i32);
                            v25
                        } else {
                            let v26: i32 = v23.wrapping_add(1i32);
                            v26
                        }
                    };
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root =\n    | HarvestRoot :: string -> harvest_root\n\n"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v28 as usize).unwrap_or(""));
                    let v31: i32 = (v0.clone().len() as i32);
                    let v32: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v31 as usize).unwrap_or(""));
                    let v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v29, &*v32, &*v12, &*v12].concat());
                    v33.clone()
                };
                let v35: i32 = v34.find(&*v18).map(|offset| offset as i32).unwrap_or(-1);
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
                let v37: i32 = v34.get(v35 as usize..).and_then(|rest| rest.find(&*v36)).map(|found| (found + v35 as usize) as i32).unwrap_or(-1);
                let v38: i32 = v37.wrapping_add(2i32);
                let v39: i32 = rows_end_from_10(v34.clone(), v38);
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
                let v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v41, &*v10, &*v2, &*v11].concat());
                let v43: Rc<str> = std::rc::Rc::<str>::from(v34.get(0i32 as usize..v39 as usize).unwrap_or(""));
                let v44: i32 = (v34.clone().len() as i32);
                let v45: Rc<str> = std::rc::Rc::<str>::from(v34.get(v39 as usize..v44 as usize).unwrap_or(""));
                let v46: Rc<str> = std::rc::Rc::<str>::from([&*v43, &*v42, &*v45, &*v12, &*v12].concat());
                US1::US1_RootsEdited(v46.clone())
            }
        }
    }
}
fn root_dropped_11(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v2, &*v6].concat());
    let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
    let v9: bool = v8 < 0i32;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a harvest root"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v10, &*v11, &*v11, &*v11].concat());
        US1::US1_RootsRefused(v12.clone())
    } else {
        let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        let v15: i32 = (v7.clone().len() as i32);
        let v16: i32 = v8.wrapping_add(v15);
        let v17: i32 = (v0.clone().len() as i32);
        let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v17 as usize).unwrap_or(""));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v18, &*v19, &*v19, &*v19].concat());
        US1::US1_RootsEdited(v20.clone())
    }
}
fn edits_applied_8(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> US1 {
    loop {
        let v4: bool = v3 <= v2;
        if v4 {
            return US1::US1_RootsEdited(v0.clone());
        } else {
            let v6: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v9: US1 = if v1 {
                root_added_9(v0.clone(), v6.clone())
            } else {
                root_dropped_11(v0.clone(), v6.clone())
            };
            match &v9 {
                US1::US1_RootsEdited(v10) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: i32 = v2.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v10.clone(), v1, v11, v3);
                    continue;
                }
                US1::US1_RootsRefused(v13) => {
                    let v13: Rc<str> = v13.clone();
                    return US1::US1_RootsRefused(v13.clone());
                }
            }
        }
    }
}
fn roots_edited_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    let v4: i32 = argument_count_6();
    let v5: bool = v4 <= 3i32;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        let v10: bool = &*v1 == "add";
        let v11: i32 = 3i32;
        let v12: i32 = argument_count_6();
        let v13: US1 = edits_applied_8(v2.clone(), v10, v11, v12);
        match &v13 {
            US1::US1_RootsEdited(v18) => {
                let v18: Rc<str> = v18.clone();
                roots_written_5(v0.clone(), v1.clone(), v2.clone(), v18.clone(), v3)
            }
            US1::US1_RootsRefused(v14) => {
                let v14: Rc<str> = v14.clone();
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
                let v17: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v16 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v17
            }
        }
    }
}
fn roots_entry_0() -> i32 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v1: i32 = 1i32;
    let v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v4, &*v4].concat());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let v9: US2 = if v6 {
        US2::US2_FileReadable(v5.clone())
    } else {
        US2::US2_FileMissing(v5.clone())
    };
    match &v9 {
        US2::US2_FileMissing(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US2::US2_FileReadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            let v17: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v16);
            let v18: i32 = 2i32;
            let v19: Rc<str> = usize::try_from(v18).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v20: bool = &*v19 == "";
            if v20 {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                let v22: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v0);
                let v23: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
                roots_receipt_1(v21.clone(), v17.clone(), v25.clone())
            } else {
                let v27: bool = &*v19 == "add";
                let v29: bool = if v27 {
                    true
                } else {
                    let v28: bool = &*v19 == "drop";
                    v28
                };
                if v29 {
                    roots_edited_4(v5.clone(), v19.clone(), v17.clone(), v0)
                } else {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown action "); } LIT.with(|lit| lit.clone()) };
                    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v19));
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected add or drop)"); } LIT.with(|lit| lit.clone()) };
                    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
                    let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                    v37
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        roots_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_roots_run() -> i32 {
    closure0()()
}
