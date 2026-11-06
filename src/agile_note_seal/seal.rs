#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_SealApply,
    US0_SealDryRun,
}
#[derive(Clone)]
enum US1 {
    US1_FileReadable(Rc<str>),
    US1_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_Replaced,
    US2_TemporaryWriteFailed,
    US2_RenameFailed,
    US2_RenameFailedTemporaryKept,
    US2_ReadbackMismatch,
    US2_ChangedSinceRead,
}
#[derive(Clone)]
enum US3 {
    US3_StateWritten,
    US3_StateChangedSinceRead,
    US3_StateWriteFailed(Rc<str>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(i32),
    US4_None,
}
fn seal_failure_2(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile seal rejected: "); } LIT.with(|lit| lit.clone()) }, v0));
    let mut v2: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v1 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v2
}
fn seal_receipt_3(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<RefCell<Vec<Rc<str>>>>, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: u64, mut v8: u64, mut v9: u64) -> i32 {
    let v13: Rc<str> = match v0 {
        US0::US0_SealApply => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        US0::US0_SealDryRun => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry-run"); } LIT.with(|lit| lit.clone()) };
            v11
        }
    };
    let v14: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" sealed="); } LIT.with(|lit| lit.clone()) }, v14));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" new_in_archive="); } LIT.with(|lit| lit.clone()) }));
    let v17: Rc<str> = std::rc::Rc::<str>::from((v6).to_string());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile seal "); } LIT.with(|lit| lit.clone()) }, v13));
    let mut v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v18));
    let v21: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v9);
    let v22: Rc<str> = std::rc::Rc::<str>::from((v7).to_string());
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v22));
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
    let v25: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
    let v28: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v29));
    let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v30.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v31
}
fn concat5_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn done_ids_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = agile_task_census::kit_agile_done_ids(&*v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v1));
    v2
}
fn member_8(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: bool = &*v1 != "";
    if v2 {
        let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v1));
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let mut v5: bool = v0.contains(&*v4);
        v5
    } else {
        false
    }
}
fn method10(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn note_owner_9(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Note (\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Option<usize> = v0.find(&*v1);
    let v3: Option<usize> = method10(v2);
    let mut v4: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v5: Option<i32> = v3.map(|x| v4(x));
    let mut v7: i32 = v5.unwrap_or(-1i32);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl note_of_"); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v0.starts_with(&*v8);
    let v10: bool = v9 == false;
    let v12: bool = if v10 {
        true
    } else {
        let v11: bool = v7 < 0i32;
        v11
    };
    if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: i32 = v7.wrapping_add(13i32);
        let v15: i32 = (v0.clone().len() as i32);
        let v16: Rc<str> = std::rc::Rc::<str>::from(v0.get(v14 as usize..v15 as usize).unwrap_or(""));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
        let v18: Option<usize> = v16.find(&*v17);
        let v19: Option<usize> = method10(v18);
        let mut v20: Option<i32> = v19.map(|x| v4(x));
        let v22: i32 = v20.unwrap_or(-1i32);
        let v23: bool = v22 < 0i32;
        if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        } else {
            let v25: Rc<str> = std::rc::Rc::<str>::from(v16.get(0i32 as usize..v22 as usize).unwrap_or(""));
            v25
        }
    }
}
fn sealed_line_7(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = note_owner_9(v1);
    member_8(v0, v2)
}
fn closure1(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = sealed_line_7(v0.clone(), v1);
        let v3: bool = v2 == false;
        v3
    })
}
fn method12(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<Vec<Rc<str>>>, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: bool = v2 < 0i32;
        if v4 {
            return v3;
        } else {
            let mut v5: usize = ((v2) as usize);
            let mut v6: Rc<str> = (v1)[v5].clone();
            let mut v7: i32 = v2.wrapping_sub(1i32);
            let v8: bool = v0(v6.clone());
            let v10: Rc<UH0> = if v8 {
                Rc::new(UH0::UH0_Cons(v6, v3))
            } else {
                v3
            };
            (v2, v3) = (v7, v10);
            continue;
        }
    }
}
fn method13(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let mut v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn filtered_11(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<RefCell<Vec<Rc<str>>>> {
    let mut v2: Rc<Vec<Rc<str>>> = Rc::new((v1).borrow().clone());
    let v3: usize = (v2).len();
    let mut v4: i32 = ((v3) as i32);
    let mut v5: i32 = v4.wrapping_sub(1i32);
    let mut v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let mut v7: Rc<UH0> = method12(v0, v2, v5, v6);
    let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v9: i32 = 0i32;
    let v10: i32 = method13(v8.clone(), v7, v9);
    let v11: Rc<Vec<Rc<str>>> = Rc::new(v8.borrow().clone());
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v11).as_ref().clone()));
    v12
}
fn closure3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        sealed_line_7(v0.clone(), v1)
    })
}
fn joined_lines_14(mut v0: Rc<RefCell<Vec<Rc<str>>>>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from((*v0).borrow().iter().map(|x| &**x).collect::<Vec<&str>>().join(&*v1).as_str());
    v2
}
fn sealing_of_6(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, i32, i32, i32) {
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.lines().map(Rc::<str>::from).collect::<Vec<Rc<str>>>()));
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = closure1(v0.clone());
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = filtered_11(v3, v2.clone());
    let mut v5: Rc<dyn Fn(Rc<str>) -> bool> = closure3(v0);
    let mut v6: Rc<RefCell<Vec<Rc<str>>>> = filtered_11(v5, v2.clone());
    let mut v7: Rc<str> = joined_lines_14(v4.clone());
    let v8: i32 = (v4.borrow().len() as i32);
    let v9: i32 = (v6.clone().borrow().len() as i32);
    let v10: i32 = (v2.borrow().len() as i32);
    (v7, v6, v8, v9, v10)
}
fn archive_path_16(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive/done-notes.spi"); } LIT.with(|lit| lit.clone()) }));
    v1
}
fn method17(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn binding_of_19(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = (v0.clone().len() as i32);
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.get(4i32 as usize..v1 as usize).unwrap_or(""));
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v4: Option<usize> = v2.find(&*v3);
    let mut v5: Option<usize> = method10(v4);
    let mut v6: Rc<dyn Fn((usize)) -> i32> = closure2();
    let mut v7: Option<i32> = v5.map(|x| v6(x));
    let v9: i32 = v7.unwrap_or(-1i32);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
    let v11: bool = v0.starts_with(&*v10);
    let v12: bool = v11 == false;
    let v14: bool = if v12 {
        true
    } else {
        let v13: bool = v9 < 0i32;
        v13
    };
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v15
    } else {
        let v16: Rc<str> = std::rc::Rc::<str>::from(v2.get(0i32 as usize..v9 as usize).unwrap_or(""));
        v16
    }
}
fn archived_binding_18(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let mut v2: Rc<str> = binding_of_19(v1);
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\ninl "); } LIT.with(|lit| lit.clone()) }, v2));
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () ="); } LIT.with(|lit| lit.clone()) }));
    let mut v5: bool = v0.contains(&*v4);
    v5
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = archived_binding_18(v0.clone(), v1);
        let v3: bool = v2 == false;
        v3
    })
}
fn renamed_within_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn state_written_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US3 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".paused"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_4(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    std::env::var("KIT_AGILE_TEST_PAUSE_MS").ok().and_then(|value| value.parse::<u64>().ok()).map(|ms| { let _ = std::fs::write(&*v5, b""); std::thread::sleep(std::time::Duration::from_millis(ms)); let _ = std::fs::remove_file(&*v5); }).unwrap_or(());
    let mut v6: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
    if v6 {
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v8: bool = std::fs::write(&*v7, &*v2).is_ok();
        let v21: US2 = if v8 {
            let v9: i32 = 40i32;
            let v10: bool = renamed_within_20(v0.clone(), v7.clone(), v9);
            if v10 {
                let v11: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v11 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v15: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
                if v15 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v21 {
            US2::US2_Replaced => {
                US3::US3_StateWritten
            }
            _ => {
                let v32: Rc<str> = match v21 {
                    US2::US2_ChangedSinceRead => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    US2::US2_ReadbackMismatch => {
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v26
                    }
                    US2::US2_RenameFailed => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v24
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v25
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v23
                    }
                    _ => unreachable!(),
                };
                US3::US3_StateWriteFailed(v32)
            }
        }
    } else {
        US3::US3_StateChangedSinceRead
    }
}
fn sealed_written_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<RefCell<Vec<Rc<str>>>>, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> Rc<str> {
    let v9: Rc<str> = archive_path_16(v0.clone());
    let v10: bool = std::path::Path::new(&*v9).is_file();
    let v23: Rc<str> = if v10 {
        let v12: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v9);
        let v14: Vec<u8> = v12.unwrap();
        let v15: Vec<u8> = method17(v14);
        let v17: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v15);
        let v19: std::string::String = v17.unwrap();
        let v21: Rc<str> = Rc::<str>::from(String::as_str(&v19));
        v21
    } else {
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v22
    };
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }, v23.clone()));
    let v25: Rc<dyn Fn(Rc<str>) -> bool> = closure4(v24);
    let v26: Rc<RefCell<Vec<Rc<str>>>> = filtered_11(v25, v4);
    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/archive"); } LIT.with(|lit| lit.clone()) }));
    let v29: Result<(), std::io::Error> = std::fs::create_dir_all(&*v27);
    let v30: bool = &*v23 == "";
    let v33: bool = if v30 {
        true
    } else {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v32: bool = v23.ends_with(&*v31);
        v32
    };
    let v35: Rc<str> = if v33 {
        v23.clone()
    } else {
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v23.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        v34
    };
    let v36: i32 = (v26.clone().borrow().len() as i32);
    let v37: bool = v36 == 0i32;
    let v41: Rc<str> = if v37 {
        v23
    } else {
        let v38: Rc<str> = joined_lines_14(v26);
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v38));
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        v40
    };
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v9.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
    let v43: bool = std::fs::write(&*v42, &*v41).is_ok();
    let v56: US2 = if v43 {
        let v44: i32 = 40i32;
        let v45: bool = renamed_within_20(v9.clone(), v42.clone(), v44);
        if v45 {
            let mut v46: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v41);
            if v46 {
                US2::US2_Replaced
            } else {
                US2::US2_ReadbackMismatch
            }
        } else {
            let v50: bool = std::fs::remove_file(&*v42).is_ok() || !std::path::Path::new(&*v42).exists();
            if v50 {
                US2::US2_RenameFailed
            } else {
                US2::US2_RenameFailedTemporaryKept
            }
        }
    } else {
        US2::US2_TemporaryWriteFailed
    };
    match &v56 {
        US2::US2_Replaced => {
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v8));
            let v58: US3 = state_written_21(v1, v2, v57);
            match &v58 {
                US3::US3_StateWritten => {
                    let v59: Rc<str> = std::rc::Rc::<str>::from((v36).to_string());
                    v59
                }
                _ => {
                    let v63: Rc<str> = match v58 {
                        US3::US3_StateChangedSinceRead => {
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                            v60
                        }
                        US3::US3_StateWriteFailed(v61) => {
                            let v61: Rc<str> = v61;
                            v61
                        }
                        _ => unreachable!(),
                    };
                    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) }, v63));
                    v64
                }
            }
        }
        _ => {
            let v75: Rc<str> = match v56 {
                US2::US2_ChangedSinceRead => {
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v70
                }
                US2::US2_ReadbackMismatch => {
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v69
                }
                US2::US2_RenameFailed => {
                    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v67
                }
                US2::US2_RenameFailedTemporaryKept => {
                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v68
                }
                US2::US2_TemporaryWriteFailed => {
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v66
                }
                _ => unreachable!(),
            };
            let v76: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!archive "); } LIT.with(|lit| lit.clone()) }, v75));
            v76
        }
    }
}
fn method22(mut v0: Option<i32>) -> Option<i32> {
    v0
}
fn closure5() -> Rc<dyn Fn((i32)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((i32)) -> US4> = Rc::new(move |mut v0: (i32)| -> US4 {
        let v1: i32 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn seal_run_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let mut v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = concat5_4(v0.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    let mut v6: bool = std::path::Path::new(&*v5).is_file();
    let v9: US1 = if v6 {
        US1::US1_FileReadable(v5.clone())
    } else {
        US1::US1_FileMissing(v5.clone())
    };
    match v9 {
        US1::US1_FileMissing(v10) => {
            let v10: Rc<str> = v10;
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            seal_failure_2(v11)
        }
        US1::US1_FileReadable(v13) => {
            let v13: Rc<str> = v13;
            let v14: Rc<str> = std::fs::read_to_string(&*v13).unwrap_or_default().into();
            let v15: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v14);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v17: bool = v15.contains(&*v16);
            if v17 {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi has CR line endings; sealing keeps LF only"); } LIT.with(|lit| lit.clone()) };
                seal_failure_2(v18)
            } else {
                let mut v20: Rc<str> = done_ids_5(v15.clone());
                let (mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, Rc<RefCell<Vec<Rc<str>>>>, i32, i32, i32) = sealing_of_6(v20, v15.clone());
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v27: bool = v15.ends_with(&*v26);
                let v28: Rc<str> = if v27 {
                    v26
                } else {
                    v4
                };
                let v29: i32 = v23.wrapping_add(v24);
                let v30: bool = v29 != v25;
                if v30 {
                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the split is not a partition of the state lines; nothing written"); } LIT.with(|lit| lit.clone()) };
                    seal_failure_2(v31)
                } else {
                    match &v1 {
                        US0::US0_SealApply => {
                            let v38: Rc<str> = sealed_written_15(v0, v5, v14, v21.clone(), v22.clone(), v23, v24, v25, v28.clone());
                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("!"); } LIT.with(|lit| lit.clone()) };
                            let v40: bool = v38.starts_with(&*v39);
                            if v40 {
                                let v41: i32 = (v38.clone().len() as i32);
                                let v42: Rc<str> = std::rc::Rc::<str>::from(v38.get(1i32 as usize..v41 as usize).unwrap_or(""));
                                seal_failure_2(v42)
                            } else {
                                let v44: Option<i32> = (v38).trim().parse::<i32>().ok();
                                let v45: Option<i32> = method22(v44);
                                let mut v46: Rc<dyn Fn((i32)) -> US4> = closure5();
                                let mut v47: Option<US4> = v45.map(|x| v46(x));
                                let v49: US4 = US4::US4_None;
                                let v50: US4 = v49.clone();
                                let v51: US4 = v47.unwrap_or(v50);
                                let v54: i32 = match v51 {
                                    US4::US4_None => {
                                        0i32
                                    }
                                    US4::US4_Some(v52) => {
                                        let v52: i32 = v52;
                                        v52
                                    }
                                };
                                let v55: u64 = v15.len() as u64;
                                let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v28));
                                let v57: u64 = v56.len() as u64;
                                seal_receipt_3(v1, v21, v22, v23, v24, v25, v54, v55, v57, v2)
                            }
                        }
                        US0::US0_SealDryRun => {
                            let v33: i32 = 0i32;
                            let v34: u64 = v15.len() as u64;
                            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v21.clone(), v28));
                            let v36: u64 = v35.len() as u64;
                            seal_receipt_3(v1, v21, v22, v23, v24, v25, v33, v34, v36, v2)
                        }
                    }
                }
            }
        }
    }
}
fn seal_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let mut v4: bool = &*v3 == "--dry-run";
    if v4 {
        let mut v5: US0 = US0::US0_SealDryRun;
        seal_run_1(v1, v5)
    } else {
        let mut v7: bool = &*v3 == "";
        if v7 {
            let v8: US0 = US0::US0_SealApply;
            seal_run_1(v1, v8)
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: agile seal <root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
            seal_failure_2(v10)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        seal_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_seal_run() -> i32 {
    closure0()()
}
