#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    US2_ChangedSinceRead,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, u64, Rc<UH1>, Rc<UH0>),
}
struct Mut0 { l0: i32, l1: Rc<UH1> }
#[derive(Clone)]
enum US3 {
    US3_Some(Rc<str>, Rc<str>, u64, Rc<UH1>),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Some(u64),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_ChangeUpsert,
    US5_ChangeRemove,
    US5_ChangeNone,
}
#[derive(Clone)]
enum US6 {
    US6_Some(i32),
    US6_None,
}
struct Mut1 { l0: Rc<str> }
fn claims_loaded_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/claims.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v3: bool = std::path::Path::new(&*v2).is_file();
    let v6: US0 = if v3 {
        US0::US0_FileReadable(v2.clone())
    } else {
        US0::US0_FileMissing(v2)
    };
    match &v6 {
        US0::US0_FileMissing(v7) => {
            let v7: Rc<str> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v8)
        }
        US0::US0_FileReadable(v9) => {
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = std::fs::read_to_string(&*v9).unwrap_or_default().into();
            (true, v10)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        claims_loaded_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn renamed_within_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
    loop {
        let v3: bool = std::fs::rename(&*v1, &*v0).is_ok();
        if v3 {
            return true;
        } else {
            let v4: bool = v2 <= 1i32;
            if v4 {
                return false;
            } else {
                std::thread::sleep(std::time::Duration::from_millis(50u64));
                let v5: i32 = v2.wrapping_sub(1i32);
                v2 = v5;
                continue;
            }
        }
    }
}
fn listed_5(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    claims\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.contains(&*v1);
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n    claims"); } LIT.with(|lit| lit.clone()) };
        let v4: bool = v0.ends_with(&*v3);
        v4
    }
}
fn module_registered_4(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v3: bool = std::path::Path::new(&*v2).is_file();
    let v6: US0 = if v3 {
        US0::US0_FileReadable(v2.clone())
    } else {
        US0::US0_FileMissing(v2.clone())
    };
    match &v6 {
        US0::US0_FileMissing(v7) => {
            let v7: Rc<str> = v7.clone();
            US1::US1_StateWritten
        }
        US0::US0_FileReadable(v9) => {
            let mut v9: Rc<str> = v9.clone();
            let mut v10: Rc<str> = std::fs::read_to_string(&*v9).unwrap_or_default().into();
            let v11: bool = listed_5(v10.clone());
            if v11 {
                US1::US1_StateWritten
            } else {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
                let v14: bool = v10.contains(&*v13);
                let v15: bool = v14 == false;
                if v15 {
                    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list claims in"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_StateWriteFailed(v16.clone())
                } else {
                    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
                    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    claims\n"); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = Rc::<str>::from(v10.replace(&*v13, &*v19));
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v21));
                    let v23: bool = std::fs::write(&*v22, &*v20).is_ok();
                    let v36: US2 = if v23 {
                        let v24: i32 = 40i32;
                        let v25: bool = renamed_within_3(v18.clone(), v22.clone(), v24);
                        if v25 {
                            let v26: bool = std::fs::read_to_string(&*v18).is_ok_and(|value| value == *v20);
                            if v26 {
                                US2::US2_Replaced
                            } else {
                                US2::US2_ReadbackMismatch
                            }
                        } else {
                            let mut v30: bool = std::fs::remove_file(&*v22).is_ok() || !std::path::Path::new(&*v22).exists();
                            if v30 {
                                US2::US2_RenameFailed
                            } else {
                                US2::US2_RenameFailedTemporaryKept
                            }
                        }
                    } else {
                        US2::US2_TemporaryWriteFailed
                    };
                    match &v36 {
                        US2::US2_Replaced => {
                            US1::US1_StateWritten
                        }
                        _ => {
                            let v47: Rc<str> = match &v36 {
                                US2::US2_ChangedSinceRead => {
                                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v42.clone()
                                }
                                US2::US2_ReadbackMismatch => {
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                    v41.clone()
                                }
                                US2::US2_RenameFailed => {
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                    v39.clone()
                                }
                                US2::US2_RenameFailedTemporaryKept => {
                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                    v40.clone()
                                }
                                US2::US2_TemporaryWriteFailed => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                    v38.clone()
                                }
                                _ => unreachable!(),
                            };
                            US1::US1_StateWriteFailed(v47.clone())
                        }
                    }
                }
            }
        }
    }
}
fn claims_saved_2(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/claims.spi"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let mut v9: bool = if v1 {
        let v6: bool = std::fs::read_to_string(&*v5).is_ok_and(|value| value == *v2);
        v6
    } else {
        let v7: bool = std::path::Path::new(&*v5).is_file();
        let v8: bool = v7 == false;
        v8
    };
    let mut v10: bool = v9 == false;
    let v41: US1 = if v10 {
        US1::US1_StateChangedSinceRead
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v12));
        let v14: bool = std::fs::write(&*v13, &*v3).is_ok();
        let v27: US2 = if v14 {
            let v15: i32 = 40i32;
            let mut v16: bool = renamed_within_3(v5.clone(), v13.clone(), v15);
            if v16 {
                let mut v17: bool = std::fs::read_to_string(&*v5).is_ok_and(|value| value == *v3);
                if v17 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v21: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
                if v21 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v27 {
            US2::US2_Replaced => {
                module_registered_4(v0.clone())
            }
            _ => {
                let v38: Rc<str> = match &v27 {
                    US2::US2_ChangedSinceRead => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v33.clone()
                    }
                    US2::US2_ReadbackMismatch => {
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v32.clone()
                    }
                    US2::US2_RenameFailed => {
                        let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v30.clone()
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let mut v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v31.clone()
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v29.clone()
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v38.clone())
            }
        }
    };
    match &v41 {
        US1::US1_StateChangedSinceRead => {
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/claims.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
            v43.clone()
        }
        US1::US1_StateWriteFailed(v44) => {
            let v44: Rc<str> = v44.clone();
            v44.clone()
        }
        US1::US1_StateWritten => {
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v42.clone()
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: u64 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v3.clone(), v4, v5.clone(), v1.clone()));
                (v0, v1) = (v6.clone(), v7.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method9(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method10(mut v0: i32, mut v1: Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1.clone(), v4.clone()))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8.clone() }));
        while method10(v7, v9.clone()) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let v14: i32 = v11.wrapping_add(1i32);
            let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13.clone(), v12.clone()));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15.clone();
            ()
        };
        let mut v16: Rc<UH1> = v9.borrow().l1.clone();
        let mut v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method9(v16.clone(), v17.clone())
    }
}
fn method13(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn every_other_15(mut v0: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<UH1> = v2.clone();
            match &*v2 {
                UH1::UH1_Cons(v3, v4) => {
                    let v3: Rc<str> = v3.clone();
                    let v4: Rc<UH1> = v4.clone();
                    let v5: Rc<UH1> = every_other_15(v4);
                    Rc::new(UH1::UH1_Cons(v1, v5))
                }
                _ => {
                    v0.clone()
                }
            }
        }
        _ => {
            v0
        }
    }
}
fn claim_of_row_12(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<UH1> = split_list_8(v1.clone(), v0.clone());
    match &*v2 {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<UH1> = v4.clone();
            match &*v4 {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5.clone();
                    let v6: Rc<UH1> = v6.clone();
                    match &*v6 {
                        UH1::UH1_Cons(v7, v8) => {
                            let v7: Rc<str> = v7.clone();
                            let v8: Rc<UH1> = v8.clone();
                            match &*v8 {
                                UH1::UH1_Cons(v9, v10) => {
                                    let mut v9: Rc<str> = v9.clone();
                                    let mut v10: Rc<UH1> = v10.clone();
                                    match &*v10 {
                                        UH1::UH1_Cons(v11, v12) => {
                                            let v11: Rc<str> = v11.clone();
                                            let v12: Rc<UH1> = v12.clone();
                                            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
                                            let v14: Option<usize> = v11.find(&*v13);
                                            let v15: Option<usize> = method13(v14.clone());
                                            let mut v16: Rc<dyn Fn((usize)) -> i32> = closure2();
                                            let mut v17: Option<i32> = v15.map(|x| v16(x));
                                            let v19: i32 = v17.unwrap_or(-1i32);
                                            let v20: bool = v19 > 2i32;
                                            let v27: Rc<str> = if v20 {
                                                let v21: i32 = v19.wrapping_sub(2i32);
                                                let v22: usize = ((2i32) as usize);
                                                let v23: i32 = 2i32.wrapping_add(v21);
                                                let v24: usize = ((v23) as usize);
                                                let v25: Rc<str> = Rc::<str>::from(&(v11)[v22..v24]);
                                                v25.clone()
                                            } else {
                                                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                v26.clone()
                                            };
                                            let v28: Option<u64> = (v27).trim().parse::<u64>().ok();
                                            let v29: Option<u64> = method14(v28.clone());
                                            let mut v30: Rc<dyn Fn((u64)) -> US4> = closure3();
                                            let mut v31: Option<US4> = v29.map(|x| v30(x));
                                            let v33: US4 = US4::US4_None;
                                            let v34: US4 = v33.clone();
                                            let v35: US4 = v31.unwrap_or(v34);
                                            match &v35 {
                                                US4::US4_None => {
                                                    US3::US3_None
                                                }
                                                US4::US4_Some(v36) => {
                                                    let v36: u64 = *v36;
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
                                                    let v38: bool = v3.starts_with(&*v37);
                                                    if v38 {
                                                        let v39: Rc<UH1> = every_other_15(v12.clone());
                                                        US3::US3_Some(v5.clone(), v9.clone(), v36, v39.clone())
                                                    } else {
                                                        US3::US3_None
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            US3::US3_None
                                        }
                                    }
                                }
                                _ => {
                                    US3::US3_None
                                }
                            }
                        }
                        _ => {
                            US3::US3_None
                        }
                    }
                }
                _ => {
                    US3::US3_None
                }
            }
        }
        _ => {
            US3::US3_None
        }
    }
}
fn method11(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: US3 = claim_of_row_12(v2.clone());
                let v11: Rc<UH0> = match &v4 {
                    US3::US3_None => {
                        v1.clone()
                    }
                    US3::US3_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5.clone();
                        let v6: Rc<str> = v6.clone();
                        let v7: u64 = *v7;
                        let v8: Rc<UH1> = v8.clone();
                        Rc::new(UH0::UH0_Cons(v5.clone(), v6.clone(), v7, v8.clone(), v1.clone()))
                    }
                };
                (v0, v1) = (v3.clone(), v11.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn claims_of_text_6(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_8(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method11(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method7(v7, v8)
}
fn method16(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH1> = method16(v3, v1.clone());
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4.clone()))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn change_word_18(mut v0: US5) -> Rc<str> {
    let v4: Rc<str> = match &v0 {
        US5::US5_ChangeNone => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("None"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US5::US5_ChangeRemove => {
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Remove"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US5::US5_ChangeUpsert => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Upsert"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    };
    let v5: Rc<str> = Rc::<str>::from(v4.to_lowercase());
    v5
}
fn change_of_17(mut v0: Rc<str>) -> US5 {
    let mut v1: US5 = US5::US5_ChangeUpsert;
    let mut v2: Rc<str> = change_word_18(v1);
    let v3: bool = &*v2 == &*v0;
    if v3 {
        US5::US5_ChangeUpsert
    } else {
        let v5: US5 = US5::US5_ChangeRemove;
        let v6: Rc<str> = change_word_18(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US5::US5_ChangeRemove
        } else {
            let mut v9: US5 = US5::US5_ChangeNone;
            let mut v10: Rc<str> = change_word_18(v9);
            US5::US5_ChangeNone
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: u64 = *v5;
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = method20(v0.clone(), v7, v2.clone());
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8.clone()))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn released_19(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method20(v1, v0, v2)
}
fn method22(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &*v0 {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: u64 = *v4;
            let v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH0> = v6.clone();
            let v7: Rc<UH0> = method22(v6, v1.clone());
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn method23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    match &*v4 {
        UH0::UH0_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: u64 = *v8;
            let mut v9: Rc<UH1> = v9.clone();
            let mut v10: Rc<UH0> = v10.clone();
            let v11: Rc<UH0> = method23(v0.clone(), v1, v2, v3, v10, v5.clone());
            let v12: bool = &*v6 != &*v0;
            if v12 {
                Rc::new(UH0::UH0_Cons(v6, v7, v8, v9, v11.clone()))
            } else {
                v11
            }
        }
        UH0::UH0_Nil => {
            v5
        }
    }
}
fn upserted_21(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH1>) -> Rc<UH0> {
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method23(v1.clone(), v2.clone(), v3, v4.clone(), v0, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v2, v3, v4, v7));
    method22(v6, v8)
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    Rc::new(move |mut v2: Rc<UH1>| -> US6 {
        match &*v2 {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4.clone();
                let v5: Rc<UH1> = v5.clone();
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US6::US6_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US6> = method25(v0.clone(), v8);
                    v9(v5.clone())
                }
            }
            UH1::UH1_Nil => {
                US6::US6_None
            }
        }
    })
}
fn method25(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    closure4(v0, v1)
}
fn method24(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match &*v1 {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: u64 = *v5;
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH0> = v7.clone();
            let v8: Rc<UH0> = method24(v0.clone(), v7, v2.clone());
            let mut v9: i32 = 0i32;
            let mut v10: Rc<dyn Fn(Rc<UH1>) -> US6> = method25(v3.clone(), v9);
            let v11: US6 = v10(v0);
            let v12: bool = match &v11 {
                US6::US6_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v13: bool = v12 != true;
            if v13 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8.clone()))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn quoted_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v6)), v4));
    v8
}
fn method31(mut v0: Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_30(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method31(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn paths_term_32(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<str> = quoted_29(v2);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PathThen ("); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = paths_term_32(v3);
            let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
            v12
        }
        UH1::UH1_Nil => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NoPaths"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn claim_row_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v0.replace(&*v4, &*v5));
    let v7: Rc<str> = Rc::<str>::from(v6.to_lowercase());
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v7));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Claim ("); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = quoted_29(v0);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let mut v16: Rc<str> = quoted_29(v1);
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v16)), v14));
    let v19: Rc<str> = format_real_30(v2);
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = paths_term_32(v3);
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    v26
}
fn method27(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: u64 = *v4;
                let v5: Rc<UH1> = v5.clone();
                let v6: Rc<UH0> = v6.clone();
                let v7: Rc<str> = claim_row_28(v2.clone(), v3.clone(), v4, v5.clone());
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v7));
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let mut v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                (v0, v1) = (v6.clone(), v10.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn claims_text_26(mut v0: Rc<UH0>) -> Rc<str> {
    let mut v1: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("///- --no-respell"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v2);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union rec claim_paths ="); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v3);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | NoPaths :: claim_paths"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v4);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | PathThen :: string * claim_paths -> claim_paths"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v5);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("union agile_claim ="); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v6);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("    | Claim :: string * string * u64 * claim_paths -> agile_claim"); } LIT.with(|lit| lit.clone()) };
    v1.borrow_mut().push(v7);
    let v8: Rc<Vec<Rc<str>>> = Rc::new(v1.borrow().clone());
    let mut v9: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v8).as_ref().clone()));
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from((*v9.borrow()).join(&*v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v10));
    method27(v0, v12)
}
fn claims_applied_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: Rc<UH0> = claims_of_text_6(v2.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<UH1> = split_list_8(v10, v8);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<UH1> = split_list_8(v12, v7);
    let v14: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v15: Rc<UH1> = method16(v13, v14);
    let mut v16: US5 = change_of_17(v3.clone());
    let v21: Rc<UH0> = match &v16 {
        US5::US5_ChangeNone => {
            v9.clone()
        }
        US5::US5_ChangeRemove => {
            released_19(v9.clone(), v4.clone())
        }
        US5::US5_ChangeUpsert => {
            upserted_21(v9, v4, v5, v6, v15)
        }
    };
    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v23: Rc<UH0> = method24(v11, v21, v22);
    let v24: US5 = change_of_17(v3);
    let v25: bool = match &v24 {
        US5::US5_ChangeNone => {
            true
        }
        _ => {
            false
        }
    };
    if v25 {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v26
    } else {
        let v27: Rc<str> = claims_text_26(v23);
        claims_saved_2(v0, v1, v2, v27)
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>| -> Rc<str> {
        claims_applied_1(v0.clone(), v1, v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6, v7.clone(), v8.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claims_loaded(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_claims_applied(v0: &str, v1: bool, v2: &str, v3: &str, v4: &str, v5: &str, v6: u64, v7: &str, v8: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3), Rc::<str>::from(v4), Rc::<str>::from(v5), v6, Rc::<str>::from(v7), Rc::<str>::from(v8))
}
