#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Valid(Rc<str>),
    US0_Invalid,
}
#[derive(Clone)]
enum US2 {
    US2_AgileEpic,
    US2_AgileFeature,
    US2_AgileStory,
    US2_AgileTaskKind,
}
#[derive(Clone)]
enum US1 {
    US1_Valid(US2),
    US1_Invalid,
}
#[derive(Clone)]
enum US4 {
    US4_AgileP0,
    US4_AgileP1,
    US4_AgileP2,
    US4_AgileP3,
    US4_AgileP4,
    US4_AgileP5,
}
#[derive(Clone)]
enum US3 {
    US3_Valid(US4),
    US3_Invalid,
}
#[derive(Clone)]
enum US6 {
    US6_AgileD0,
    US6_AgileD1,
    US6_AgileD2,
    US6_AgileD3,
    US6_AgileD4,
    US6_AgileD5,
    US6_AgileD6,
    US6_AgileD7,
    US6_AgileD8,
}
#[derive(Clone)]
enum US5 {
    US5_Valid(US6),
    US5_Invalid,
}
#[derive(Clone)]
enum US7 {
    US7_ExtrasRead(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>),
    US7_ExtrasRefused(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_FlagEstimate,
    US9_FlagLoad,
    US9_FlagAssignee,
    US9_FlagDeps,
    US9_FlagNote,
    US9_FlagAs,
}
#[derive(Clone)]
enum US8 {
    US8_FlagKnown(US9),
    US8_FlagUnknown,
}
#[derive(Clone)]
enum US10 {
    US10_FileReadable(Rc<str>),
    US10_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_AddedText(Rc<str>),
    US11_AddedRefused(Rc<str>),
}
#[derive(Clone)]
enum US12 {
    US12_Replaced,
    US12_TemporaryWriteFailed,
    US12_RenameFailed,
    US12_RenameFailedTemporaryKept,
    US12_ReadbackMismatch,
    US12_ChangedSinceRead,
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn task_id_bytes_allowed_2(mut v0: i32, mut v1: i32) -> bool {
    loop {
        let mut v2: bool = v0 == v1;
        if v2 {
            return true;
        } else {
            let v3: i32 = 2i32;
            let v4: i32 = usize::try_from(v3).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v0).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
            let v5: bool = 65i32 <= v4;
            let v7: bool = if v5 {
                let v6: bool = v4 <= 90i32;
                v6
            } else {
                false
            };
            let v13: bool = if v7 {
                true
            } else {
                let v8: bool = 48i32 <= v4;
                let v10: bool = if v8 {
                    let mut v9: bool = v4 <= 57i32;
                    v9
                } else {
                    false
                };
                if v10 {
                    true
                } else {
                    let v11: bool = v4 == 45i32;
                    v11
                }
            };
            if v13 {
                let v14: i32 = v0.wrapping_add(1i32);
                v0 = v14;
                continue;
            } else {
                return false;
            }
        }
    }
}
fn flag_of_word_4(mut v0: Rc<str>) -> US8 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = concat5_1(v1.clone(), v3, v4.clone(), v4.clone(), v4.clone());
    let v6: bool = &*v0 == &*v5;
    if v6 {
        let v7: US9 = US9::US9_FlagEstimate;
        US8::US8_FlagKnown(v7)
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
        let v11: Rc<str> = concat5_1(v1.clone(), v10, v4.clone(), v4.clone(), v4.clone());
        let v12: bool = &*v0 == &*v11;
        if v12 {
            let v13: US9 = US9::US9_FlagLoad;
            US8::US8_FlagKnown(v13)
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
            let v17: Rc<str> = concat5_1(v1.clone(), v16, v4.clone(), v4.clone(), v4.clone());
            let v18: bool = &*v0 == &*v17;
            if v18 {
                let v19: US9 = US9::US9_FlagAssignee;
                US8::US8_FlagKnown(v19)
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from(v21.to_ascii_lowercase());
                let v23: Rc<str> = concat5_1(v1.clone(), v22, v4.clone(), v4.clone(), v4.clone());
                let v24: bool = &*v0 == &*v23;
                if v24 {
                    let v25: US9 = US9::US9_FlagDeps;
                    US8::US8_FlagKnown(v25)
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
                    let v29: Rc<str> = concat5_1(v1.clone(), v28, v4.clone(), v4.clone(), v4.clone());
                    let v30: bool = &*v0 == &*v29;
                    if v30 {
                        let v31: US9 = US9::US9_FlagNote;
                        US8::US8_FlagKnown(v31)
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = std::rc::Rc::<str>::from(v33.to_ascii_lowercase());
                        let v35: Rc<str> = concat5_1(v1, v34, v4.clone(), v4.clone(), v4);
                        let v36: bool = &*v0 == &*v35;
                        if v36 {
                            let v37: US9 = US9::US9_FlagAs;
                            US8::US8_FlagKnown(v37)
                        } else {
                            US8::US8_FlagUnknown
                        }
                    }
                }
            }
        }
    }
}
fn extras_from_3(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US7 {
    loop {
        let v8: bool = v1 <= v0;
        if v8 {
            return US7::US7_ExtrasRead(v2, v3, v4, v5, v6, v7);
        } else {
            let v10: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v11: US8 = flag_of_word_4(v10.clone());
            match v11 {
                US8::US8_FlagKnown(v12) => {
                    let v12: US9 = v12;
                    let v13: i32 = v0.wrapping_add(1i32);
                    let v14: bool = v1 <= v13;
                    if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs a value"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = concat5_1(v10, v15, v16.clone(), v16.clone(), v16.clone());
                        return US7::US7_ExtrasRefused(v17);
                    } else {
                        let v19: i32 = v0.wrapping_add(2i32);
                        let v20: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                            US9::US9_FlagEstimate => {
                                (v20, v3, v4, v5, v6, v7)
                            }
                            _ => {
                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                    US9::US9_FlagLoad => {
                                        (v2, v20, v4, v5, v6, v7)
                                    }
                                    _ => {
                                        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                            US9::US9_FlagAssignee => {
                                                (v2, v3, v20, v5, v6, v7)
                                            }
                                            _ => {
                                                let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                    US9::US9_FlagDeps => {
                                                        (v2, v3, v4, v20, v6, v7)
                                                    }
                                                    _ => {
                                                        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                            US9::US9_FlagNote => {
                                                                let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                (v2, v3, v4, v5, v21, v7)
                                                            }
                                                            _ => {
                                                                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match v12 {
                                                                    US9::US9_FlagAs => {
                                                                        let v22: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                        (v2, v3, v4, v5, v6, v22)
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v23, v24, v25, v26, v27, v28)
                                                            }
                                                        };
                                                        (v29, v30, v31, v32, v33, v34)
                                                    }
                                                };
                                                (v35, v36, v37, v38, v39, v40)
                                            }
                                        };
                                        (v41, v42, v43, v44, v45, v46)
                                    }
                                };
                                (v47, v48, v49, v50, v51, v52)
                            }
                        };
                        (v0, v2, v3, v4, v5, v6, v7) = (v19, v53, v54, v55, v56, v57, v58);
                        continue;
                    }
                }
                US8::US8_FlagUnknown => {
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let mut v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<str> = std::rc::Rc::<str>::from(v64.to_ascii_lowercase());
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v67: Rc<str> = concat5_1(v63.clone(), v65, v66.clone(), v66.clone(), v66.clone());
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v67));
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v68));
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = std::rc::Rc::<str>::from(v70.to_ascii_lowercase());
                    let v72: Rc<str> = concat5_1(v63.clone(), v71, v66.clone(), v66.clone(), v66.clone());
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v72));
                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v73));
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = std::rc::Rc::<str>::from(v75.to_ascii_lowercase());
                    let v77: Rc<str> = concat5_1(v63.clone(), v76, v66.clone(), v66.clone(), v66.clone());
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v77));
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v78));
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                    let v81: Rc<str> = std::rc::Rc::<str>::from(v80.to_ascii_lowercase());
                    let v82: Rc<str> = concat5_1(v63.clone(), v81, v66.clone(), v66.clone(), v66.clone());
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v82));
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v83));
                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v86: Rc<str> = std::rc::Rc::<str>::from(v85.to_ascii_lowercase());
                    let v87: Rc<str> = concat5_1(v63.clone(), v86, v66.clone(), v66.clone(), v66.clone());
                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v87));
                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v88));
                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v91: Rc<str> = std::rc::Rc::<str>::from(v90.to_ascii_lowercase());
                    let v92: Rc<str> = concat5_1(v63, v91, v66.clone(), v66.clone(), v66.clone());
                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v92));
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v89, v93));
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = concat5_1(v61, v10, v62, v94, v95);
                    return US7::US7_ExtrasRefused(v96);
                }
            }
        }
    }
}
fn extras_checked_5(mut v0: US7) -> US7 {
    match &v0 {
        US7::US7_ExtrasRead(v2, v3, v4, v5, v6, v7) => {
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: bool = &*v7 == "";
            let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = if v8 {
                let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = std::env::var(&*v9).map_or_else(|_| std::rc::Rc::<str>::default(), std::rc::Rc::<str>::from);
                let v11: Rc<str> = std::rc::Rc::<str>::from(v10.trim());
                (v2, v3, v4, v5, v6, v11)
            } else {
                (v2, v3, v4, v5, v6, v7)
            };
            let v18: bool = &*v16 == "";
            if v18 {
                if v8 {
                    US7::US7_ExtrasRead(v12, v13, v14, v15, v16, v17)
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as names the author of --note; give --note too"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v20)
                }
            } else {
                let v23: bool = &*v17 == "";
                if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a note needs its author: --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v24)
                } else {
                    US7::US7_ExtrasRead(v12, v13, v14, v15, v16, v17)
                }
            }
        }
        US7::US7_ExtrasRefused(v1) => {
            let v1: Rc<str> = v1.clone();
            v0
        }
    }
}
fn extras_applied_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: &Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US11 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::kit_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let v10: bool = v8 == 0i32;
    if v10 {
        US11::US11_AddedText(v9)
    } else {
        US11::US11_AddedRefused(v9)
    }
}
fn renamed_within_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US12 {
    let v3: bool = kit_agile_records::kit_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0.clone(), { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) }));
        let v5: bool = std::fs::write(&*v4, &*v2).is_ok();
        if v5 {
            let v6: i32 = 40i32;
            let v7: bool = renamed_within_8(v0.clone(), v4.clone(), v6);
            if v7 {
                let v8: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v8 {
                    US12::US12_Replaced
                } else {
                    US12::US12_ReadbackMismatch
                }
            } else {
                let v12: bool = std::fs::remove_file(&*v4).is_ok() || !std::path::Path::new(&*v4).exists();
                if v12 {
                    US12::US12_RenameFailed
                } else {
                    US12::US12_RenameFailedTemporaryKept
                }
            }
        } else {
            US12::US12_TemporaryWriteFailed
        }
    } else {
        US12::US12_ChangedSinceRead
    }
}
fn agile_add_entry_0() -> i32 {
    let mut v0: i32 = 2i32;
    let v1: i32 = 0i32;
    let mut v2: i32 = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).and_then(|value| usize::try_from(v1).ok().and_then(|offset| value.as_bytes().get(offset).copied())).map_or(-1, i32::from);
    let v3: bool = 48i32 <= v2;
    let v5: bool = if v3 {
        let v4: bool = v2 <= 57i32;
        v4
    } else {
        false
    };
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must start with a letter, so its inl name is a valid identifier"); } LIT.with(|lit| lit.clone()) };
        let v7: i32 = 2i32;
        let v8: Rc<str> = usize::try_from(v7).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = concat5_1(v9, v8, v10, v6, v11);
        let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let v14: i32 = 2i32;
        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let v16: bool = 0 < v15;
        let v25: US0 = if v16 {
            let v17: i32 = 0i32;
            let v18: bool = task_id_bytes_allowed_2(v17, v15);
            if v18 {
                let v19: i32 = 2i32;
                let v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                US0::US0_Valid(v20)
            } else {
                US0::US0_Invalid
            }
        } else {
            US0::US0_Invalid
        };
        let v26: i32 = 3i32;
        let mut v27: Rc<str> = usize::try_from(v26).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v28: bool = &*v27 == "Epic";
        let v32: US1 = if v28 {
            let v29: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v29)
        } else {
            US1::US1_Invalid
        };
        let mut v33: bool = &*v27 == "Feature";
        let v36: US1 = if v33 {
            let v34: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v34)
        } else {
            v32
        };
        let v37: bool = &*v27 == "Story";
        let v40: US1 = if v37 {
            let v38: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v38)
        } else {
            v36
        };
        let v41: bool = &*v27 == "TaskKind";
        let v44: US1 = if v41 {
            let v42: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v42)
        } else {
            v40
        };
        let v45: i32 = 4i32;
        let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v47: bool = &*v46 == "P0";
        let v51: US3 = if v47 {
            let v48: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v48)
        } else {
            US3::US3_Invalid
        };
        let v52: bool = &*v46 == "P1";
        let v55: US3 = if v52 {
            let v53: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v53)
        } else {
            v51
        };
        let v56: bool = &*v46 == "P2";
        let v59: US3 = if v56 {
            let v57: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v57)
        } else {
            v55
        };
        let v60: bool = &*v46 == "P3";
        let v63: US3 = if v60 {
            let v61: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v61)
        } else {
            v59
        };
        let mut v64: bool = &*v46 == "P4";
        let v67: US3 = if v64 {
            let v65: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v65)
        } else {
            v63
        };
        let v68: bool = &*v46 == "P5";
        let v71: US3 = if v68 {
            let v69: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v69)
        } else {
            v67
        };
        let v72: i32 = 5i32;
        let v73: Rc<str> = usize::try_from(v72).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v74: bool = &*v73 == "D0";
        let v78: US5 = if v74 {
            let mut v75: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v75)
        } else {
            US5::US5_Invalid
        };
        let v79: bool = &*v73 == "D1";
        let v82: US5 = if v79 {
            let mut v80: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v80)
        } else {
            v78
        };
        let v83: bool = &*v73 == "D2";
        let v86: US5 = if v83 {
            let v84: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v84)
        } else {
            v82
        };
        let v87: bool = &*v73 == "D3";
        let mut v90: US5 = if v87 {
            let v88: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v88)
        } else {
            v86
        };
        let v91: bool = &*v73 == "D4";
        let v94: US5 = if v91 {
            let v92: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v92)
        } else {
            v90
        };
        let v95: bool = &*v73 == "D5";
        let v98: US5 = if v95 {
            let v96: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v96)
        } else {
            v94
        };
        let v99: bool = &*v73 == "D6";
        let v102: US5 = if v99 {
            let v100: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v100)
        } else {
            v98
        };
        let v103: bool = &*v73 == "D7";
        let v106: US5 = if v103 {
            let v104: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v104)
        } else {
            v102
        };
        let v107: bool = &*v73 == "D8";
        let v110: US5 = if v107 {
            let v108: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v108)
        } else {
            v106
        };
        match v25 {
            US0::US0_Invalid => {
                let v340: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let v341: i32 = 2i32;
                let v342: Rc<str> = usize::try_from(v341).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v343: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v344: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v345: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v346: Rc<str> = concat5_1(v343, v342, v344, v340, v345);
                let v347: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v346 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v347
            }
            US0::US0_Valid(v111) => {
                let v111: Rc<str> = v111;
                match v44 {
                    US1::US1_Invalid => {
                        let v330: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let v331: i32 = 2i32;
                        let v332: Rc<str> = usize::try_from(v331).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let v334: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v335: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v336: Rc<str> = concat5_1(v333, v332, v334, v330, v335);
                        let v337: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v336 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v337
                    }
                    US1::US1_Valid(v112) => {
                        let v112: US2 = v112;
                        match v71 {
                            US3::US3_Invalid => {
                                let v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let v321: i32 = 2i32;
                                let v322: Rc<str> = usize::try_from(v321).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let v324: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v326: Rc<str> = concat5_1(v323, v322, v324, v320, v325);
                                let v327: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v326 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v327
                            }
                            US3::US3_Valid(v113) => {
                                let v113: US4 = v113;
                                match v110 {
                                    US5::US5_Invalid => {
                                        let v310: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let v311: i32 = 2i32;
                                        let v312: Rc<str> = usize::try_from(v311).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let v314: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let v315: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v316: Rc<str> = concat5_1(v313, v312, v314, v310, v315);
                                        let v317: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v316 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v317
                                    }
                                    US5::US5_Valid(v114) => {
                                        let v114: US6 = v114;
                                        let v115: i32 = 8i32;
                                        let v116: i32 = std::env::args().skip(2i32 as usize).count() as i32;
                                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v118: US7 = extras_from_3(v115, v116, v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone());
                                        let v119: US7 = extras_checked_5(v118);
                                        match v119 {
                                            US7::US7_ExtrasRead(v120, v121, v122, v123, v124, v125) => {
                                                let v120: Rc<str> = v120;
                                                let v121: Rc<str> = v121;
                                                let v122: Rc<str> = v122;
                                                let v123: Rc<str> = v123;
                                                let v124: Rc<str> = v124;
                                                let v125: Rc<str> = v125;
                                                let v126: i32 = 1i32;
                                                let v127: Rc<str> = usize::try_from(v126).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v128: i32 = 6i32;
                                                let v129: Rc<str> = usize::try_from(v128).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v130: i32 = 7i32;
                                                let v131: Rc<str> = usize::try_from(v130).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let v134: Rc<str> = concat5_1(v127, v133, v117.clone(), v117.clone(), v117.clone());
                                                let v135: bool = std::path::Path::new(&*v134).is_file();
                                                let v138: US10 = if v135 {
                                                    US10::US10_FileReadable(v134.clone())
                                                } else {
                                                    US10::US10_FileMissing(v134.clone())
                                                };
                                                match v138 {
                                                    US10::US10_FileMissing(v139) => {
                                                        let v139: Rc<str> = v139;
                                                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let v143: Rc<str> = concat5_1(v141, v111, v142, v140, v117);
                                                        let v144: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v143 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v144
                                                    }
                                                    US10::US10_FileReadable(v145) => {
                                                        let v145: Rc<str> = v145;
                                                        let v146: Rc<str> = std::fs::read_to_string(&*v145).unwrap_or_default().into();
                                                        let mut v147: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v146);
                                                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                        let v150: Rc<str> = concat5_1(v148, v111.clone(), v149, v117.clone(), v117.clone());
                                                        let v151: bool = v147.contains(&*v150);
                                                        if v151 {
                                                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let v155: Rc<str> = concat5_1(v153, v111, v154, v152, v117);
                                                            let v156: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v155 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v156
                                                        } else {
                                                            let v157: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v158: Rc<str> = std::rc::Rc::<str>::from(v147.trim_end_matches(&*v157));
                                                            let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                            let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                            let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v162: Rc<str> = Rc::<str>::from(v111.replace(&*v160, &*v161));
                                                            let v163: Rc<str> = std::rc::Rc::<str>::from(v162.to_ascii_lowercase());
                                                            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                            let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v168: Rc<str> = Rc::<str>::from(v111.replace(&*v166, &*v167));
                                                            let v169: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                            let v170: Rc<str> = Rc::<str>::from(v168.replace(&*v165, &*v169));
                                                            let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v172: Rc<str> = Rc::<str>::from(v170.replace(&*v157, &*v171));
                                                            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v175: Rc<str> = Rc::<str>::from(v172.replace(&*v173, &*v174));
                                                            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v177: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v178: Rc<str> = Rc::<str>::from(v175.replace(&*v176, &*v177));
                                                            let v179: Rc<str> = concat5_1(v165.clone(), v178, v165.clone(), v117.clone(), v117.clone());
                                                            let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                            let v181: Rc<str> = concat5_1(v159, v163, v164, v179, v180.clone());
                                                            let v182: Rc<str> = Rc::<str>::from(v129.replace(&*v166, &*v167));
                                                            let v183: Rc<str> = Rc::<str>::from(v182.replace(&*v165, &*v169));
                                                            let v184: Rc<str> = Rc::<str>::from(v183.replace(&*v157, &*v171));
                                                            let v185: Rc<str> = Rc::<str>::from(v184.replace(&*v173, &*v174));
                                                            let v186: Rc<str> = Rc::<str>::from(v185.replace(&*v176, &*v177));
                                                            let v187: Rc<str> = concat5_1(v165.clone(), v186, v165.clone(), v117.clone(), v117.clone());
                                                            let v192: Rc<str> = match v112 {
                                                                US2::US2_AgileEpic => {
                                                                    let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                                    v188
                                                                }
                                                                US2::US2_AgileFeature => {
                                                                    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                                    v189
                                                                }
                                                                US2::US2_AgileStory => {
                                                                    let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                                    v190
                                                                }
                                                                US2::US2_AgileTaskKind => {
                                                                    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                                    v191
                                                                }
                                                            };
                                                            let v199: Rc<str> = match v113 {
                                                                US4::US4_AgileP0 => {
                                                                    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                                    v193
                                                                }
                                                                US4::US4_AgileP1 => {
                                                                    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                                    v194
                                                                }
                                                                US4::US4_AgileP2 => {
                                                                    let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                                    v195
                                                                }
                                                                US4::US4_AgileP3 => {
                                                                    let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                                    v196
                                                                }
                                                                US4::US4_AgileP4 => {
                                                                    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                                    v197
                                                                }
                                                                US4::US4_AgileP5 => {
                                                                    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                                    v198
                                                                }
                                                            };
                                                            let v200: Rc<str> = concat5_1(v187, v180.clone(), v192, v180.clone(), v199);
                                                            let v210: Rc<str> = match v114 {
                                                                US6::US6_AgileD0 => {
                                                                    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                                    v201
                                                                }
                                                                US6::US6_AgileD1 => {
                                                                    let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                                    v202
                                                                }
                                                                US6::US6_AgileD2 => {
                                                                    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                                    v203
                                                                }
                                                                US6::US6_AgileD3 => {
                                                                    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                                    v204
                                                                }
                                                                US6::US6_AgileD4 => {
                                                                    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                                    v205
                                                                }
                                                                US6::US6_AgileD5 => {
                                                                    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                                    v206
                                                                }
                                                                US6::US6_AgileD6 => {
                                                                    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                                    v207
                                                                }
                                                                US6::US6_AgileD7 => {
                                                                    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                                    v208
                                                                }
                                                                US6::US6_AgileD8 => {
                                                                    let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                                    v209
                                                                }
                                                            };
                                                            let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, "); } LIT.with(|lit| lit.clone()) };
                                                            let v212: Rc<str> = Rc::<str>::from(v123.replace(&*v166, &*v167));
                                                            let v213: Rc<str> = Rc::<str>::from(v212.replace(&*v165, &*v169));
                                                            let v214: Rc<str> = Rc::<str>::from(v213.replace(&*v157, &*v171));
                                                            let v215: Rc<str> = Rc::<str>::from(v214.replace(&*v173, &*v174));
                                                            let v216: Rc<str> = Rc::<str>::from(v215.replace(&*v176, &*v177));
                                                            let v217: Rc<str> = concat5_1(v165.clone(), v216, v165.clone(), v117.clone(), v117.clone());
                                                            let v218: Rc<str> = Rc::<str>::from(v131.replace(&*v166, &*v167));
                                                            let v219: Rc<str> = Rc::<str>::from(v218.replace(&*v165, &*v169));
                                                            let v220: Rc<str> = Rc::<str>::from(v219.replace(&*v157, &*v171));
                                                            let v221: Rc<str> = Rc::<str>::from(v220.replace(&*v173, &*v174));
                                                            let v222: Rc<str> = Rc::<str>::from(v221.replace(&*v176, &*v177));
                                                            let v223: Rc<str> = concat5_1(v165.clone(), v222, v165.clone(), v117.clone(), v117.clone());
                                                            let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                            let v225: Rc<str> = concat5_1(v180.clone(), v223, v224, v117.clone(), v117.clone());
                                                            let v226: Rc<str> = concat5_1(v180, v210, v211, v217, v225);
                                                            let v227: Rc<str> = concat5_1(v181, v200, v226, v117.clone(), v117.clone());
                                                            let v228: Rc<str> = concat5_1(v158, v157.clone(), v227, v157.clone(), v117.clone());
                                                            let v229: US11 = extras_applied_6(v228, v111.clone(), v120.clone(), v121.clone(), v122.clone(), &v123, v124.clone(), v125);
                                                            match v229 {
                                                                US11::US11_AddedRefused(v230) => {
                                                                    let v230: Rc<str> = v230;
                                                                    let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                    let v233: Rc<str> = concat5_1(v231, v111, v232, v230, v117);
                                                                    let v234: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v233 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v234
                                                                }
                                                                US11::US11_AddedText(v235) => {
                                                                    let v235: Rc<str> = v235;
                                                                    let v236: US12 = replace_records_unchanged_7(v134, v147.clone(), v235.clone());
                                                                    match &v236 {
                                                                        US12::US12_Replaced => {
                                                                            let mut v237: Rc<str> = Rc::<str>::from(v111.replace(&*v160, &*v161));
                                                                            let v238: Rc<str> = std::rc::Rc::<str>::from(v237.to_ascii_lowercase());
                                                                            let v239: u64 = v147.len() as u64;
                                                                            let v240: u64 = v235.len() as u64;
                                                                            let v241: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v132);
                                                                            let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                                            let v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                                            let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                                            let v245: bool = &*v123 == "";
                                                                            let v248: Rc<str> = if v245 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps="); } LIT.with(|lit| lit.clone()) };
                                                                                concat5_1(v246, v123, v117.clone(), v117.clone(), v117.clone())
                                                                            };
                                                                            let v249: bool = &*v120 == "";
                                                                            let v251: Rc<str> = if v249 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
                                                                                v250
                                                                            };
                                                                            let v252: bool = &*v121 == "";
                                                                            let v254: Rc<str> = if v252 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
                                                                                v253
                                                                            };
                                                                            let v255: bool = &*v122 == "";
                                                                            let v257: Rc<str> = if v255 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
                                                                                v256
                                                                            };
                                                                            let v258: Rc<str> = concat5_1(v251, v254, v257, v117.clone(), v117.clone());
                                                                            let v259: bool = &*v258 == "";
                                                                            let v266: Rc<str> = if v259 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" planned="); } LIT.with(|lit| lit.clone()) };
                                                                                let v261: Rc<str> = std::rc::Rc::<str>::from(v258.trim());
                                                                                let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                                                let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                                let v264: Rc<str> = Rc::<str>::from(v261.replace(&*v262, &*v263));
                                                                                concat5_1(v260, v264, v117.clone(), v117.clone(), v117.clone())
                                                                            };
                                                                            let v267: bool = &*v124 == "";
                                                                            let v269: Rc<str> = if v267 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
                                                                                v268
                                                                            };
                                                                            let v270: Rc<str> = concat5_1(v248, v266, v269, v117.clone(), v117.clone());
                                                                            let v271: Rc<str> = concat5_1(v244, v270, v117.clone(), v117.clone(), v117.clone());
                                                                            let v272: Rc<str> = concat5_1(v242, v111, v243, v238, v271);
                                                                            let v273: Rc<str> = std::rc::Rc::<str>::from((v239).to_string());
                                                                            let v274: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) }, v273));
                                                                            let v275: Rc<str> = Rc::<str>::from(format!("{}{}", v274, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v276: Rc<str> = std::rc::Rc::<str>::from((v240).to_string());
                                                                            let v277: Rc<str> = Rc::<str>::from(format!("{}{}", v275, v276));
                                                                            let v278: Rc<str> = Rc::<str>::from(format!("{}{}", v277, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) }));
                                                                            let v279: Rc<str> = std::rc::Rc::<str>::from((v241).to_string());
                                                                            let v280: Rc<str> = Rc::<str>::from(format!("{}{}", v278, v279));
                                                                            let v281: Rc<str> = concat5_1(v272, v280, v117.clone(), v117.clone(), v117.clone());
                                                                            let v282: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v281.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                                            v282
                                                                        }
                                                                        _ => {
                                                                            let v292: Rc<str> = match v236 {
                                                                                US12::US12_ChangedSinceRead => {
                                                                                    let v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                    v287
                                                                                }
                                                                                US12::US12_ReadbackMismatch => {
                                                                                    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v286
                                                                                }
                                                                                US12::US12_RenameFailed => {
                                                                                    let v284: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v284
                                                                                }
                                                                                US12::US12_RenameFailedTemporaryKept => {
                                                                                    let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v285
                                                                                }
                                                                                US12::US12_TemporaryWriteFailed => {
                                                                                    let v283: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v283
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            let v293: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                            let v294: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                            let v295: Rc<str> = concat5_1(v293, v111, v294, v292, v117);
                                                                            let v296: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v295 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                            v296
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            US7::US7_ExtrasRefused(v303) => {
                                                let v303: Rc<str> = v303;
                                                let v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let v305: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let v306: Rc<str> = concat5_1(v304, v111, v305, v303, v117);
                                                let v307: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v306 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v307
                                            }
                                        }
                                    }
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
        agile_add_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_add_run() -> i32 {
    closure0()()
}
