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
    US0_ChangedSinceRead,
}
#[derive(Clone)]
enum US1 {
    US1_RootsEdited(Rc<str>),
    US1_RootsRefused(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH0> }
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
fn concat5_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn harvest_roots_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let mut v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v5: bool = v4 < 0i32;
        if v5 {
            return v2.clone();
        } else {
            let v6: i32 = v4.wrapping_add(19i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
            let v8: i32 = v0.get(v6 as usize..).and_then(|rest| rest.find(&*v7)).map(|found| (found + v6 as usize) as i32).unwrap_or(-1);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v6 as usize..v8 as usize).unwrap_or(""));
            let v10: bool = &*v2 == "";
            let v14: Rc<str> = if v10 {
                v9.clone()
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_3(v2.clone(), v11.clone(), v9.clone(), v12.clone(), v12.clone())
            };
            let v15: i32 = v8.wrapping_add(1i32);
            (v1, v2) = (v15, v14.clone());
            continue;
        }
    }
}
fn roots_printed_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v3 <= v1;
        if v4 {
            return v2;
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v5)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
            let v7: bool = v6 < 0i32;
            let v8: i32 = if v7 {
                v3
            } else {
                v6
            };
            let mut v9: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v8 as usize).unwrap_or(""));
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("root "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            let v12: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v11.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            let v13: i32 = v8.wrapping_add(1i32);
            let v14: i32 = v2.wrapping_add(1i32);
            (v1, v2) = (v13, v14);
            continue;
        }
    }
}
fn roots_receipt_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: i32 = 0i32;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = harvest_roots_from_2(v1.clone(), v3, v4.clone());
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = roots_printed_4(v5.clone(), v6, v7);
    let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile roots ok action="); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v0));
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" roots="); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
    let v13: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v12, v13)), v2));
    let v16: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v15.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v16
}
fn renamed_within_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let mut v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let mut v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn replace_records_unchanged_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: bool = kit_agile_records::kit_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
        let v6: bool = std::fs::write(&*v5, &*v2).is_ok();
        if v6 {
            let v7: i32 = 40i32;
            let v8: bool = renamed_within_8(v0.clone(), v5.clone(), v7);
            if v8 {
                let mut v9: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v9 {
                    US0::US0_Replaced
                } else {
                    US0::US0_ReadbackMismatch
                }
            } else {
                let v13: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
                if v13 {
                    US0::US0_RenameFailed
                } else {
                    US0::US0_RenameFailedTemporaryKept
                }
            }
        } else {
            US0::US0_TemporaryWriteFailed
        }
    } else {
        US0::US0_ChangedSinceRead
    }
}
fn roots_written_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: US0 = replace_records_unchanged_7(v0.clone(), v2.clone(), v3.clone());
    match &v5 {
        US0::US0_Replaced => {
            let v6: u64 = v2.len() as u64;
            let v7: u64 = v3.len() as u64;
            let v8: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v4);
            let mut v9: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            let v14: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            let v18: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
            roots_receipt_1(v1.clone(), v3.clone(), v19.clone())
        }
        _ => {
            let v30: Rc<str> = match &v5 {
                US0::US0_ChangedSinceRead => {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v25.clone()
                }
                US0::US0_ReadbackMismatch => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v24.clone()
                }
                US0::US0_RenameFailed => {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v22.clone()
                }
                US0::US0_RenameFailedTemporaryKept => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v23.clone()
                }
                US0::US0_TemporaryWriteFailed => {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v21.clone()
                }
                _ => unreachable!(),
            };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v30));
            let v33: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v32 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v33
        }
    }
}
fn command_line_argument_count_10() -> i32 {
    let v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_9() -> i32 {
    let v0: i32 = command_line_argument_count_10();
    let mut v1: bool = v0 > 0i32;
    if v1 {
        let mut v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn method16(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure1() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn root_key_15(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":/"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method16(v2.clone());
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure1();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let v7: i32 = v5.unwrap_or(-1i32);
    let v8: bool = v7 == 1i32;
    if v8 {
        let mut v9: Rc<str> = Rc::<str>::from(v0.to_lowercase());
        v9.clone()
    } else {
        v0.clone()
    }
}
fn method14(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<str>) -> Rc<str> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH0> = v4.clone();
                let mut v5: Rc<str> = root_key_15(v3.clone());
                let v6: bool = &*v2 != "";
                let v8: bool = if v6 {
                    true
                } else {
                    let v7: bool = &*v3 == "";
                    v7
                };
                let v19: Rc<str> = if v8 {
                    v2.clone()
                } else {
                    let mut v9: bool = &*v5 == &*v0;
                    let v13: bool = if v9 {
                        true
                    } else {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v10));
                        let v12: bool = v0.starts_with(&*v11);
                        v12
                    };
                    let v17: bool = if v13 {
                        true
                    } else {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v14));
                        let v16: bool = v5.starts_with(&*v15);
                        v16
                    };
                    if v17 {
                        v3.clone()
                    } else {
                        v2.clone()
                    }
                };
                (v1, v2) = (v4.clone(), v19.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method19(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_17(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH0::UH0_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method19(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH0> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let v16: Rc<UH0> = v9.borrow().l1.clone();
        let v17: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
        method18(v16.clone(), v17.clone())
    }
}
fn overlapping_root_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = root_key_15(v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: i32 = 0i32;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = harvest_roots_from_2(v0, v4, v5.clone());
    let v7: Rc<UH0> = split_list_17(v3, v6);
    method14(v2, v7, v5)
}
fn overlap_reason_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = root_key_15(v0.clone());
    let v3: Rc<str> = root_key_15(v1.clone());
    let mut v4: bool = &*v2 == &*v3;
    if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is already a harvest root"); } LIT.with(|lit| lit.clone()) };
        let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
        v6
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" overlaps harvest root "); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v7)), v1));
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (one root per tree, so every file is scanned once)"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
        v11
    }
}
fn rows_end_from_21(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v3: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v2)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let v7: i32 = if v4 {
            let mut v5: i32 = (v0.clone().len() as i32);
            v5
        } else {
            let v6: i32 = v3.wrapping_add(1i32);
            v6
        };
        let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize..v7 as usize).unwrap_or(""));
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
        let v10: bool = v8.contains(&*v9);
        if v10 {
            v1 = v7;
            continue;
        } else {
            return v1;
        }
    }
}
fn root_added_12(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: bool = &*v2 == "";
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root needs a path"); } LIT.with(|lit| lit.clone()) };
        US1::US1_RootsRefused(v4.clone())
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v7: bool = v2.contains(&*v6);
        if v7 {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a harvest root cannot contain a double quote"); } LIT.with(|lit| lit.clone()) };
            US1::US1_RootsRefused(v8.clone())
        } else {
            let v10: Rc<str> = overlapping_root_13(v0.clone(), v2.clone());
            let v11: bool = &*v10 != "";
            if v11 {
                let v12: Rc<str> = overlapping_root_13(v0.clone(), v2.clone());
                let v13: Rc<str> = overlap_reason_20(v2.clone(), v12.clone());
                US1::US1_RootsRefused(v13.clone())
            } else {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root ="); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v0.contains(&*v15);
                let v32: Rc<str> = if v16 {
                    v0.clone()
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                    let v18: bool = v0.starts_with(&*v17);
                    let v25: i32 = if v18 {
                        0i32
                    } else {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) };
                        let v20: i32 = v0.find(&*v19).map(|offset| offset as i32).unwrap_or(-1);
                        let v21: bool = v20 < 0i32;
                        if v21 {
                            let v22: i32 = (v0.clone().len() as i32);
                            v22
                        } else {
                            let v23: i32 = v20.wrapping_add(1i32);
                            v23
                        }
                    };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union harvest_root =\n    | HarvestRoot :: string -> harvest_root\n\n"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v25 as usize).unwrap_or(""));
                    let v28: i32 = (v0.clone().len() as i32);
                    let v29: Rc<str> = std::rc::Rc::<str>::from(v0.get(v25 as usize..v28 as usize).unwrap_or(""));
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    concat5_3(v27.clone(), v26.clone(), v29.clone(), v30.clone(), v30.clone())
                };
                let v33: i32 = v32.find(&*v15).map(|offset| offset as i32).unwrap_or(-1);
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n\n"); } LIT.with(|lit| lit.clone()) };
                let v35: i32 = v32.get(v33 as usize..).and_then(|rest| rest.find(&*v34)).map(|found| (found + v33 as usize) as i32).unwrap_or(-1);
                let v36: i32 = v35.wrapping_add(2i32);
                let v37: i32 = rows_end_from_21(v32.clone(), v36);
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = concat5_3(v38.clone(), v39.clone(), v40.clone(), v2.clone(), v41.clone());
                let v43: Rc<str> = std::rc::Rc::<str>::from(v32.get(0i32 as usize..v37 as usize).unwrap_or(""));
                let v44: i32 = (v32.clone().len() as i32);
                let v45: Rc<str> = std::rc::Rc::<str>::from(v32.get(v37 as usize..v44 as usize).unwrap_or(""));
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = concat5_3(v43.clone(), v42.clone(), v45.clone(), v46.clone(), v46.clone());
                US1::US1_RootsEdited(v47.clone())
            }
        }
    }
}
fn root_dropped_22(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from({ let forward = (v1).trim().replace(char::from(92u8), "/"); let trimmed = forward.trim_end_matches(char::from(47u8)); if trimmed.is_empty() { forward } else { trimmed.to_owned() } });
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(format!("harvest_root_{}", (v2).to_ascii_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { char::from(95u8) }).collect::<String>().trim_matches(char::from(95u8))));
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = HarvestRoot \""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\"\n"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_3(v3.clone(), v4.clone(), v5.clone(), v2.clone(), v6.clone());
    let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
    let mut v9: bool = v8 < 0i32;
    if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is not a harvest root"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = concat5_3(v2.clone(), v10.clone(), v11.clone(), v11.clone(), v11.clone());
        US1::US1_RootsRefused(v12.clone())
    } else {
        let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v8 as usize).unwrap_or(""));
        let v15: i32 = (v7.clone().len() as i32);
        let v16: i32 = v8.wrapping_add(v15);
        let v17: i32 = (v0.clone().len() as i32);
        let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v16 as usize..v17 as usize).unwrap_or(""));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = concat5_3(v14.clone(), v18.clone(), v19.clone(), v19.clone(), v19.clone());
        US1::US1_RootsEdited(v20.clone())
    }
}
fn edits_applied_11(mut v0: Rc<str>, mut v1: bool, mut v2: i32, mut v3: i32) -> US1 {
    loop {
        let mut v4: bool = v3 <= v2;
        if v4 {
            return US1::US1_RootsEdited(v0.clone());
        } else {
            let v6: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let mut v9: US1 = if v1 {
                root_added_12(v0.clone(), v6.clone())
            } else {
                root_dropped_22(v0.clone(), v6.clone())
            };
            match &v9 {
                US1::US1_RootsEdited(v10) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: i32 = v2.wrapping_add(1i32);
                    (v0, v2) = (v10.clone(), v11);
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
fn roots_edited_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> i32 {
    let mut v4: i32 = argument_count_9();
    let mut v5: bool = v4 <= 3i32;
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile roots rejected: add and drop need at least one path"); } LIT.with(|lit| lit.clone()) };
        let mut v9: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v8 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v9
    } else {
        let v10: bool = &*v1 == "add";
        let v11: i32 = 3i32;
        let v12: i32 = argument_count_9();
        let v13: US1 = edits_applied_11(v2.clone(), v10, v11, v12);
        match &v13 {
            US1::US1_RootsEdited(v18) => {
                let v18: Rc<str> = v18.clone();
                roots_written_6(v0.clone(), v1.clone(), v2.clone(), v18.clone(), v3)
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
    let mut v1: i32 = 1i32;
    let mut v2: Rc<str> = usize::try_from(v1).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_3(v2.clone(), v3.clone(), v4.clone(), v4.clone(), v4.clone());
    let v6: bool = std::path::Path::new(&*v5).is_file();
    let mut v9: US2 = if v6 {
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
                    roots_edited_5(v5.clone(), v19.clone(), v17.clone(), v0)
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
