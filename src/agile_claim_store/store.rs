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
pub struct Mut0 { pub l0: i32, pub l1: Rc<UH1> }
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
pub struct Mut1 { pub l0: Rc<str> }
fn claims_loaded_0(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/claims.spi"); } LIT.with(|lit| lit.clone()) }));
    let mut v2: bool = std::path::Path::new(&*v1).is_file();
    let v5: US0 = if v2 {
        US0::US0_FileReadable(v1)
    } else {
        US0::US0_FileMissing(v1)
    };
    match v5 {
        US0::US0_FileMissing(v6) => {
            let v6: Rc<str> = v6;
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v7)
        }
        US0::US0_FileReadable(v8) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<str> = std::fs::read_to_string(&*v8).unwrap_or_default().into();
            (true, v9)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        claims_loaded_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn renamed_within_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn listed_6(mut v0: Rc<str>) -> bool {
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
fn module_registered_5(mut v0: Rc<str>) -> US1 {
    let mut v1: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/package.spiproj"); } LIT.with(|lit| lit.clone()) }));
    let mut v2: bool = std::path::Path::new(&*v1).is_file();
    let v5: US0 = if v2 {
        US0::US0_FileReadable(v1.clone())
    } else {
        US0::US0_FileMissing(v1.clone())
    };
    match v5 {
        US0::US0_FileMissing(v6) => {
            let v6: Rc<str> = v6;
            US1::US1_StateWritten
        }
        US0::US0_FileReadable(v8) => {
            let v8: Rc<str> = v8;
            let mut v9: Rc<str> = std::fs::read_to_string(&*v8).unwrap_or_default().into();
            let mut v10: bool = listed_6(v9.clone());
            if v10 {
                US1::US1_StateWritten
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n"); } LIT.with(|lit| lit.clone()) };
                let v13: bool = v9.contains(&*v12);
                let mut v14: bool = v13 == false;
                if v14 {
                    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/package.spiproj has no modules section to list claims in"); } LIT.with(|lit| lit.clone()) };
                    US1::US1_StateWriteFailed(v15)
                } else {
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modules:\n    claims\n"); } LIT.with(|lit| lit.clone()) };
                    let v18: Rc<str> = Rc::<str>::from(v9.replace(&*v12, &*v17));
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v1.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
                    let v20: bool = std::fs::write(&*v19, &*v18).is_ok();
                    let v33: US2 = if v20 {
                        let v21: i32 = 40i32;
                        let v22: bool = renamed_within_4(v1.clone(), v19.clone(), v21);
                        if v22 {
                            let v23: bool = std::fs::read_to_string(&*v1).is_ok_and(|value| value == *v18);
                            if v23 {
                                US2::US2_Replaced
                            } else {
                                US2::US2_ReadbackMismatch
                            }
                        } else {
                            let v27: bool = std::fs::remove_file(&*v19).is_ok() || !std::path::Path::new(&*v19).exists();
                            if v27 {
                                US2::US2_RenameFailed
                            } else {
                                US2::US2_RenameFailedTemporaryKept
                            }
                        }
                    } else {
                        US2::US2_TemporaryWriteFailed
                    };
                    match &v33 {
                        US2::US2_Replaced => {
                            US1::US1_StateWritten
                        }
                        _ => {
                            let v44: Rc<str> = match v33 {
                                US2::US2_ChangedSinceRead => {
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                    v39
                                }
                                US2::US2_ReadbackMismatch => {
                                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                    v38
                                }
                                US2::US2_RenameFailed => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                    v36
                                }
                                US2::US2_RenameFailedTemporaryKept => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                                US2::US2_TemporaryWriteFailed => {
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                    v35
                                }
                                _ => unreachable!(),
                            };
                            US1::US1_StateWriteFailed(v44)
                        }
                    }
                }
            }
        }
    }
}
fn claims_saved_3(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/claims.spi"); } LIT.with(|lit| lit.clone()) }));
    let v8: bool = if v1 {
        let v5: bool = std::fs::read_to_string(&*v4).is_ok_and(|value| value == *v2);
        v5
    } else {
        let v6: bool = std::path::Path::new(&*v4).is_file();
        let v7: bool = v6 == false;
        v7
    };
    let mut v9: bool = v8 == false;
    let v39: US1 = if v9 {
        US1::US1_StateChangedSinceRead
    } else {
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v4.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v12: bool = std::fs::write(&*v11, &*v3).is_ok();
        let v25: US2 = if v12 {
            let v13: i32 = 40i32;
            let mut v14: bool = renamed_within_4(v4.clone(), v11.clone(), v13);
            if v14 {
                let mut v15: bool = std::fs::read_to_string(&*v4).is_ok_and(|value| value == *v3);
                if v15 {
                    US2::US2_Replaced
                } else {
                    US2::US2_ReadbackMismatch
                }
            } else {
                let v19: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
                if v19 {
                    US2::US2_RenameFailed
                } else {
                    US2::US2_RenameFailedTemporaryKept
                }
            }
        } else {
            US2::US2_TemporaryWriteFailed
        };
        match &v25 {
            US2::US2_Replaced => {
                module_registered_5(v0)
            }
            _ => {
                let v36: Rc<str> = match v25 {
                    US2::US2_ChangedSinceRead => {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                        v31
                    }
                    US2::US2_ReadbackMismatch => {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v30
                    }
                    US2::US2_RenameFailed => {
                        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v28
                    }
                    US2::US2_RenameFailedTemporaryKept => {
                        let mut v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v29
                    }
                    US2::US2_TemporaryWriteFailed => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v27
                    }
                    _ => unreachable!(),
                };
                US1::US1_StateWriteFailed(v36)
            }
        }
    };
    match v39 {
        US1::US1_StateChangedSinceRead => {
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/claims.spi changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
            v41
        }
        US1::US1_StateWriteFailed(v42) => {
            let v42: Rc<str> = v42;
            v42
        }
        US1::US1_StateWritten => {
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v40
        }
    }
}
fn method8(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v1));
                (v0, v1) = (v6, v7);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn method10(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn method11(mut v0: i32, mut v1: &Rc<RefCell<Mut0>>) -> bool {
    let mut v2: i32 = v1.borrow().l0;
    let v3: bool = v2 < v0;
    v3
}
fn split_list_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v2 == 0i32;
    if v3 {
        let v4: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(UH1::UH1_Cons(v1, v4))
    } else {
        let v6: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v1.split(&*v0).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v7: i32 = (v6.clone().borrow().len() as i32);
        let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        let mut v9: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: 0i32, l1: v8 }));
        while method11(v7, &v9) {
            let v11: i32 = v9.borrow().l0;
            let v12: Rc<UH1> = v9.borrow().l1.clone();
            let v13: Rc<str> = v6.borrow()[v11 as usize].clone();
            let mut v14: i32 = v11.wrapping_add(1i32);
            let mut v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v13, v12));
            v9.borrow_mut().l0 = v14;
            v9.borrow_mut().l1 = v15;
            ()
        };
        let v16: Rc<UH1> = v9.borrow().l1.clone();
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method10(v16, v17)
    }
}
fn method14(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US4> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US4> = Rc::new(move |mut v0: (u64)| -> US4 {
        let mut v1: u64 = (v0);
        US4::US4_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn globs_from_16(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32) -> Rc<UH1> {
    let mut v2: i32 = (v0.clone().borrow().len() as i32);
    let v3: bool = v2 <= v1;
    if v3 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
        let v6: i32 = v1.wrapping_add(2i32);
        let v7: Rc<UH1> = globs_from_16(v0, v6);
        Rc::new(UH1::UH1_Cons(v5, v7))
    }
}
fn claim_of_row_13(mut v0: Rc<str>) -> US3 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 < 5i32;
    let v8: bool = if v4 {
        true
    } else {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) };
        let v6: bool = v0.starts_with(&*v5);
        let v7: bool = v6 == false;
        v7
    };
    if v8 {
        US3::US3_None
    } else {
        let mut v10: Rc<str> = v2.borrow()[4i32 as usize].clone();
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64"); } LIT.with(|lit| lit.clone()) };
        let v12: Option<usize> = v10.find(&*v11);
        let v13: Option<usize> = method14(v12);
        let mut v14: Rc<dyn Fn((usize)) -> i32> = closure2();
        let mut v15: Option<i32> = v13.map(|x| v14(x));
        let v17: i32 = v15.unwrap_or(-1i32);
        let v18: bool = v17 > 2i32;
        let v25: Rc<str> = if v18 {
            let v19: i32 = v17.wrapping_sub(2i32);
            let v20: usize = ((2i32) as usize);
            let v21: i32 = 2i32.wrapping_add(v19);
            let v22: usize = ((v21) as usize);
            let v23: Rc<str> = Rc::<str>::from(&(v10)[v20..v22]);
            v23
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        };
        let v26: Option<u64> = (v25).trim().parse::<u64>().ok();
        let v27: Option<u64> = method15(v26);
        let mut v28: Rc<dyn Fn((u64)) -> US4> = closure3();
        let mut v29: Option<US4> = v27.map(|x| v28(x));
        let v31: US4 = US4::US4_None;
        let v32: US4 = v31.clone();
        let v33: US4 = v29.unwrap_or(v32);
        match v33 {
            US4::US4_None => {
                US3::US3_None
            }
            US4::US4_Some(v34) => {
                let v34: u64 = v34;
                let v35: Rc<str> = v2.borrow()[1i32 as usize].clone();
                let v36: Rc<str> = v2.borrow()[3i32 as usize].clone();
                let v37: i32 = 5i32;
                let v38: Rc<UH1> = globs_from_16(&v2, v37);
                US3::US3_Some(v35, v36, v34, v38)
            }
        }
    }
}
fn method12(mut v0: Rc<UH1>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH1::UH1_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<UH1> = v3;
                let v4: US3 = claim_of_row_13(v2);
                let v11: Rc<UH0> = match v4 {
                    US3::US3_None => {
                        v1
                    }
                    US3::US3_Some(v5, v6, v7, v8) => {
                        let v5: Rc<str> = v5;
                        let v6: Rc<str> = v6;
                        let v7: u64 = v7;
                        let v8: Rc<UH1> = v8;
                        Rc::new(UH0::UH0_Cons(v5, v6, v7, v8, v1))
                    }
                };
                (v0, v1) = (v3, v11);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn claims_of_text_7(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(v0.replace(&*v2, &*v3));
    let v5: Rc<UH1> = split_list_9(v1, v4);
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH0> = method12(v5, v6);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method8(v7, v8)
}
fn method17(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<UH1> = method17(v3, v1);
            let v5: bool = &*v2 != "";
            if v5 {
                Rc::new(UH1::UH1_Cons(v2, v4))
            } else {
                v4
            }
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn change_word_19(mut v0: US5) -> Rc<str> {
    let v4: Rc<str> = match v0 {
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
fn change_of_18(mut v0: Rc<str>) -> US5 {
    let mut v1: US5 = US5::US5_ChangeUpsert;
    let mut v2: Rc<str> = change_word_19(v1);
    let v3: bool = &*v2 == &*v0;
    if v3 {
        US5::US5_ChangeUpsert
    } else {
        let v5: US5 = US5::US5_ChangeRemove;
        let v6: Rc<str> = change_word_19(v5);
        let v7: bool = &*v6 == &*v0;
        if v7 {
            US5::US5_ChangeRemove
        } else {
            let mut v9: US5 = US5::US5_ChangeNone;
            let mut v10: Rc<str> = change_word_19(v9);
            US5::US5_ChangeNone
        }
    }
}
fn method21(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: u64 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH0> = v7;
            let v8: Rc<UH0> = method21(v0.clone(), v7, v2);
            let mut v9: bool = &*v3 != &*v0;
            if v9 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn released_20(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<UH0> {
    let mut v2: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    method21(v1, v0, v2)
}
fn method23(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v0) {
        UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = v3;
            let v4: u64 = v4;
            let v5: Rc<UH1> = v5;
            let v6: Rc<UH0> = v6;
            let v7: Rc<UH0> = method23(v6, v1);
            Rc::new(UH0::UH0_Cons(v2, v3, v4, v5, v7))
        }
        UH0::UH0_Nil => {
            v1
        }
    }
}
fn decliner_25(mut v0: &Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH1>) -> Rc<str> {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = v1.starts_with(&*v4);
    if v5 {
        let v6: i32 = (v1.clone().len() as i32);
        let v7: i32 = v6.wrapping_sub(1i32);
        let v8: usize = ((1i32) as usize);
        let mut v9: i32 = 1i32.wrapping_add(v7);
        let mut v10: usize = ((v9) as usize);
        let v11: Rc<str> = Rc::<str>::from(&(v1)[v8..v10]);
        v11
    } else {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v12
    }
}
fn method24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: &Rc<UH1>, mut v4: Rc<UH0>, mut v5: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v4) {
        UH0::UH0_Cons(v6, v7, v8, v9, v10) => {
            let v6: Rc<str> = v6;
            let v7: Rc<str> = v7;
            let v8: u64 = v8;
            let mut v9: Rc<UH1> = v9;
            let mut v10: Rc<UH0> = v10;
            let v11: Rc<UH0> = method24(v0.clone(), v1.clone(), v2, v3, v10, v5);
            let v12: bool = &*v6 != &*v0;
            let v22: bool = if v12 {
                true
            } else {
                let v13: Rc<str> = decliner_25(&v0, v1.clone(), v2, v3);
                let mut v14: bool = &*v13 != "";
                if v14 {
                    let mut v15: bool = &*v7 != &*v1;
                    if v15 {
                        let v16: Rc<str> = decliner_25(&v0, v1, v2, v3);
                        let v17: bool = &*v7 != &*v16;
                        v17
                    } else {
                        false
                    }
                } else {
                    let v19: Rc<str> = decliner_25(&v6, v7.clone(), v8, &v9);
                    let v20: bool = &*v19 != "";
                    v20
                }
            };
            if v22 {
                Rc::new(UH0::UH0_Cons(v6, v7, v8, v9, v11))
            } else {
                v11
            }
        }
        UH0::UH0_Nil => {
            v5
        }
    }
}
fn upserted_22(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: Rc<UH1>) -> Rc<UH0> {
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v6: Rc<UH0> = method24(v1.clone(), v2.clone(), v3, &v4, v0, v5);
    let v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v2, v3, v4, v7));
    method23(v6, v8)
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    Rc::new(move |mut v2: Rc<UH1>| -> US6 {
        match Rc::unwrap_or_clone(v2) {
            UH1::UH1_Cons(v4, v5) => {
                let v4: Rc<str> = v4;
                let v5: Rc<UH1> = v5;
                let v6: bool = &*v4 == &*v0;
                if v6 {
                    US6::US6_Some(v1)
                } else {
                    let v8: i32 = v1.wrapping_add(1i32);
                    let mut v9: Rc<dyn Fn(Rc<UH1>) -> US6> = method27(v0.clone(), v8);
                    v9(v5)
                }
            }
            UH1::UH1_Nil => {
                US6::US6_None
            }
        }
    })
}
fn method27(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn(Rc<UH1>) -> US6> {
    closure4(v0, v1)
}
fn method26(mut v0: Rc<UH1>, mut v1: Rc<UH0>, mut v2: Rc<UH0>) -> Rc<UH0> {
    match Rc::unwrap_or_clone(v1) {
        UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
            let v3: Rc<str> = v3;
            let v4: Rc<str> = v4;
            let v5: u64 = v5;
            let v6: Rc<UH1> = v6;
            let v7: Rc<UH0> = v7;
            let v8: Rc<UH0> = method26(v0.clone(), v7, v2);
            let mut v9: i32 = 0i32;
            let mut v10: Rc<dyn Fn(Rc<UH1>) -> US6> = method27(v3.clone(), v9);
            let v11: US6 = v10(v0);
            let v12: bool = match v11 {
                US6::US6_None => {
                    true
                }
                _ => {
                    false
                }
            };
            let v13: bool = v12 != true;
            if v13 {
                Rc::new(UH0::UH0_Cons(v3, v4, v5, v6, v8))
            } else {
                v8
            }
        }
        UH0::UH0_Nil => {
            v2
        }
    }
}
fn quoted_31(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(v0.replace(&*v1, &*v2));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(v3.replace(&*v4, &*v5));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }, v6));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) }));
    v8
}
fn method33(mut v0: &Rc<RefCell<Mut1>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_32(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut1>> = Rc::new(RefCell::new(Mut1 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method33(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn paths_term_34(mut v0: Rc<UH1>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH1::UH1_Cons(v2, v3) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH1> = v3;
            let v4: Rc<str> = quoted_31(v2);
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("PathThen ("); } LIT.with(|lit| lit.clone()) }, v4));
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
            let v7: Rc<str> = paths_term_34(v3);
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
            let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
            v9
        }
        UH1::UH1_Nil => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("NoPaths"); } LIT.with(|lit| lit.clone()) };
            v1
        }
    }
}
fn claim_row_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<UH1>) -> Rc<str> {
    let v4: Rc<str> = decliner_25(&v0, v1.clone(), v2, &v3);
    let v5: bool = &*v4 == "";
    let v13: Rc<str> = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v6
    } else {
        let v7: Rc<str> = decliner_25(&v0, v1.clone(), v2, &v3);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
        let mut v10: Rc<str> = Rc::<str>::from(v7.replace(&*v8, &*v9));
        let v11: Rc<str> = Rc::<str>::from(v10.to_lowercase());
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("__declined_"); } LIT.with(|lit| lit.clone()) }, v11));
        v12
    };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(v0.replace(&*v14, &*v15));
    let v17: Rc<str> = Rc::<str>::from(v16.to_lowercase());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl claim_"); } LIT.with(|lit| lit.clone()) }, v17));
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v13));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Claim ("); } LIT.with(|lit| lit.clone()) }));
    let v21: Rc<str> = quoted_31(v0);
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v24: Rc<str> = quoted_31(v1);
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }));
    let v27: Rc<str> = format_real_32(v2);
    let mut v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("u64, "); } LIT.with(|lit| lit.clone()) }));
    let v30: Rc<str> = paths_term_34(v3);
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
    v32
}
fn method29(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3, v4, v5, v6) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: u64 = v4;
                let v5: Rc<UH1> = v5;
                let v6: Rc<UH0> = v6;
                let v7: Rc<str> = claim_row_30(v2, v3, v4, v5);
                let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v7));
                let mut v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                (v0, v1) = (v6, v9);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn claims_text_28(mut v0: Rc<UH0>) -> Rc<str> {
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
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
    method29(v0, v12)
}
fn claims_applied_2(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>) -> Rc<str> {
    let mut v9: Rc<UH0> = claims_of_text_7(v2.clone());
    let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<UH1> = split_list_9(v10, v8);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("~"); } LIT.with(|lit| lit.clone()) };
    let v13: bool = v5.starts_with(&*v12);
    let v19: Rc<UH1> = if v13 {
        { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(";"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<UH1> = split_list_9(v15, v7);
        let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
        method17(v16, v17)
    };
    let v20: US5 = change_of_18(v3.clone());
    let v25: Rc<UH0> = match v20 {
        US5::US5_ChangeNone => {
            v9
        }
        US5::US5_ChangeRemove => {
            released_20(v9, v4)
        }
        US5::US5_ChangeUpsert => {
            upserted_22(v9, v4, v5, v6, v19)
        }
    };
    let v26: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH0> = method26(v11, v25, v26);
    let mut v28: US5 = change_of_18(v3);
    let mut v29: bool = match v28 {
        US5::US5_ChangeNone => {
            true
        }
        _ => {
            false
        }
    };
    if v29 {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
    } else {
        let v31: Rc<str> = claims_text_28(v27);
        claims_saved_3(v0, v1, v2, v31)
    }
}
fn applied_claims_1(mut v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str> {
    let (mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>) = v0.borrow()[0i32 as usize].clone();
    claims_applied_2(v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn closure1() -> Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str>> = Rc::new(move |mut v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>| -> Rc<str> {
        applied_claims_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_claims_loaded(v0: &str) -> (bool, Rc<str>) {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_claims_applied(v0: Rc<RefCell<Vec<(Rc<str>, bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, Rc<str>, Rc<str>)>>>) -> Rc<str> {
    closure1()(v0)
}
pub type SpiralShape_accbf6de6842 = Mut0;
pub type SpiralShape_97df63ebabc3 = Mut1;
