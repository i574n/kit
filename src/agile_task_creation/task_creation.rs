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
fn task_id_bytes_allowed_1(mut v0: i32, mut v1: i32) -> bool {
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
                (v0, v1) = (v14, v1);
                continue;
            } else {
                return false;
            }
        }
    }
}
fn flag_of_word_3(mut v0: Rc<str>) -> US8 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Estimate"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = std::rc::Rc::<str>::from(v2.to_ascii_lowercase());
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v3, &*v4, &*v4, &*v4].concat());
    let v6: bool = &*v0 == &*v5;
    if v6 {
        let v7: US9 = US9::US9_FlagEstimate;
        US8::US8_FlagKnown(v7)
    } else {
        let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = std::rc::Rc::<str>::from(v9.to_ascii_lowercase());
        let v11: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v10, &*v4, &*v4, &*v4].concat());
        let v12: bool = &*v0 == &*v11;
        if v12 {
            let v13: US9 = US9::US9_FlagLoad;
            US8::US8_FlagKnown(v13)
        } else {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = std::rc::Rc::<str>::from(v15.to_ascii_lowercase());
            let v17: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v16, &*v4, &*v4, &*v4].concat());
            let v18: bool = &*v0 == &*v17;
            if v18 {
                let v19: US9 = US9::US9_FlagAssignee;
                US8::US8_FlagKnown(v19)
            } else {
                let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = std::rc::Rc::<str>::from(v21.to_ascii_lowercase());
                let v23: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v22, &*v4, &*v4, &*v4].concat());
                let v24: bool = &*v0 == &*v23;
                if v24 {
                    let v25: US9 = US9::US9_FlagDeps;
                    US8::US8_FlagKnown(v25)
                } else {
                    let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = std::rc::Rc::<str>::from(v27.to_ascii_lowercase());
                    let v29: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v28, &*v4, &*v4, &*v4].concat());
                    let v30: bool = &*v0 == &*v29;
                    if v30 {
                        let v31: US9 = US9::US9_FlagNote;
                        US8::US8_FlagKnown(v31)
                    } else {
                        let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = std::rc::Rc::<str>::from(v33.to_ascii_lowercase());
                        let v35: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v34, &*v4, &*v4, &*v4].concat());
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
fn extras_from_2(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US7 {
    loop {
        let v8: bool = v1 <= v0;
        if v8 {
            return US7::US7_ExtrasRead(v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone());
        } else {
            let v10: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
            let v11: US8 = flag_of_word_3(v10.clone());
            match &v11 {
                US8::US8_FlagKnown(v12) => {
                    let v12: US9 = v12.clone();
                    let v13: i32 = v0.wrapping_add(1i32);
                    let v14: bool = v1 <= v13;
                    if v14 {
                        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" needs a value"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v15, &*v16, &*v16, &*v16].concat());
                        return US7::US7_ExtrasRefused(v17.clone());
                    } else {
                        let v19: i32 = v0.wrapping_add(2i32);
                        let v20: Rc<str> = usize::try_from(v13).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                            US9::US9_FlagEstimate => {
                                (v20.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
                            }
                            _ => {
                                let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                    US9::US9_FlagLoad => {
                                        (v2.clone(), v20.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
                                    }
                                    _ => {
                                        let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                            US9::US9_FlagAssignee => {
                                                (v2.clone(), v3.clone(), v20.clone(), v5.clone(), v6.clone(), v7.clone())
                                            }
                                            _ => {
                                                let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                    US9::US9_FlagDeps => {
                                                        (v2.clone(), v3.clone(), v4.clone(), v20.clone(), v6.clone(), v7.clone())
                                                    }
                                                    _ => {
                                                        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                            US9::US9_FlagNote => {
                                                                let mut v21: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v21.clone(), v7.clone())
                                                            }
                                                            _ => {
                                                                let (mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>) = match &v12 {
                                                                    US9::US9_FlagAs => {
                                                                        let v22: Rc<str> = std::rc::Rc::<str>::from(v20.trim());
                                                                        (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v22.clone())
                                                                    }
                                                                    _ => unreachable!(),
                                                                };
                                                                (v23.clone(), v24.clone(), v25.clone(), v26.clone(), v27.clone(), v28.clone())
                                                            }
                                                        };
                                                        (v29.clone(), v30.clone(), v31.clone(), v32.clone(), v33.clone(), v34.clone())
                                                    }
                                                };
                                                (v35.clone(), v36.clone(), v37.clone(), v38.clone(), v39.clone(), v40.clone())
                                            }
                                        };
                                        (v41.clone(), v42.clone(), v43.clone(), v44.clone(), v45.clone(), v46.clone())
                                    }
                                };
                                (v47.clone(), v48.clone(), v49.clone(), v50.clone(), v51.clone(), v52.clone())
                            }
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v19, v1, v53.clone(), v54.clone(), v55.clone(), v56.clone(), v57.clone(), v58.clone());
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
                    let v67: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v65, &*v66, &*v66, &*v66].concat());
                    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v67.clone()));
                    let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, v68.clone()));
                    let mut v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Load"); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = std::rc::Rc::<str>::from(v70.to_ascii_lowercase());
                    let v72: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v71, &*v66, &*v66, &*v66].concat());
                    let v73: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v72.clone()));
                    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v69.clone(), v73.clone()));
                    let mut v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Assignee"); } LIT.with(|lit| lit.clone()) };
                    let v76: Rc<str> = std::rc::Rc::<str>::from(v75.to_ascii_lowercase());
                    let v77: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v76, &*v66, &*v66, &*v66].concat());
                    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v77.clone()));
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v74.clone(), v78.clone()));
                    let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Deps"); } LIT.with(|lit| lit.clone()) };
                    let v81: Rc<str> = std::rc::Rc::<str>::from(v80.to_ascii_lowercase());
                    let v82: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v81, &*v66, &*v66, &*v66].concat());
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v82.clone()));
                    let v84: Rc<str> = Rc::<str>::from(format!("{}{}", v79.clone(), v83.clone()));
                    let mut v85: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Note"); } LIT.with(|lit| lit.clone()) };
                    let v86: Rc<str> = std::rc::Rc::<str>::from(v85.to_ascii_lowercase());
                    let v87: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v86, &*v66, &*v66, &*v66].concat());
                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v87.clone()));
                    let v89: Rc<str> = Rc::<str>::from(format!("{}{}", v84.clone(), v88.clone()));
                    let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("As"); } LIT.with(|lit| lit.clone()) };
                    let v91: Rc<str> = std::rc::Rc::<str>::from(v90.to_ascii_lowercase());
                    let v92: Rc<str> = std::rc::Rc::<str>::from([&*v63, &*v91, &*v66, &*v66, &*v66].concat());
                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) }, v92.clone()));
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v89.clone(), v93.clone()));
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = std::rc::Rc::<str>::from([&*v61, &*v10, &*v62, &*v94, &*v95].concat());
                    return US7::US7_ExtrasRefused(v96.clone());
                }
            }
        }
    }
}
fn extras_checked_4(mut v0: US7) -> US7 {
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
                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v11.clone())
            } else {
                (v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone())
            };
            let v18: bool = &*v16 == "";
            if v18 {
                if v8 {
                    US7::US7_ExtrasRead(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone())
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--as names the author of --note; give --note too"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v20.clone())
                }
            } else {
                let v23: bool = &*v17 == "";
                if v23 {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a note needs its author: --as <author> (or set KIT_AUTHOR)"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_ExtrasRefused(v24.clone())
                } else {
                    US7::US7_ExtrasRead(v12.clone(), v13.clone(), v14.clone(), v15.clone(), v16.clone(), v17.clone())
                }
            }
        }
        US7::US7_ExtrasRefused(v1) => {
            let v1: Rc<str> = v1.clone();
            v0.clone()
        }
    }
}
fn extras_applied_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> US11 {
    let (mut v8, mut v9): (i32, Rc<str>) = agile_task_notes::kit_agile_groomed(&*v0, &*v1, &*v2, &*v3, &*v4, &*v6, &*v7);
    let v10: bool = v8 == 0i32;
    if v10 {
        US11::US11_AddedText(v9.clone())
    } else {
        US11::US11_AddedRefused(v9)
    }
}
fn renamed_within_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
                (v0, v1, v2) = (v0.clone(), v1.clone(), v5);
                continue;
            }
        }
    }
}
fn replace_records_unchanged_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US12 {
    let v3: bool = kit_agile_records::kit_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
        let v6: bool = std::fs::write(&*v5, &*v2).is_ok();
        if v6 {
            let v7: i32 = 40i32;
            let v8: bool = renamed_within_7(v0.clone(), v5.clone(), v7);
            if v8 {
                let mut v9: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
                if v9 {
                    US12::US12_Replaced
                } else {
                    US12::US12_ReadbackMismatch
                }
            } else {
                let v13: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
                if v13 {
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
    let v0: i32 = 2i32;
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
        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v8, &*v10, &*v6, &*v11].concat());
        let v13: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v12 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v13
    } else {
        let v14: i32 = 2i32;
        let mut v15: i32 = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or(-1, |value| i32::try_from(value.len()).unwrap_or(i32::MAX));
        let v16: bool = 0 < v15;
        let v25: US0 = if v16 {
            let v17: i32 = 0i32;
            let v18: bool = task_id_bytes_allowed_1(v17, v15);
            if v18 {
                let v19: i32 = 2i32;
                let v20: Rc<str> = usize::try_from(v19).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                US0::US0_Valid(v20.clone())
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
            US1::US1_Valid(v29.clone())
        } else {
            US1::US1_Invalid
        };
        let mut v33: bool = &*v27 == "Feature";
        let v36: US1 = if v33 {
            let v34: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v34.clone())
        } else {
            v32.clone()
        };
        let v37: bool = &*v27 == "Story";
        let v40: US1 = if v37 {
            let v38: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v38.clone())
        } else {
            v36.clone()
        };
        let v41: bool = &*v27 == "TaskKind";
        let v44: US1 = if v41 {
            let v42: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v42.clone())
        } else {
            v40.clone()
        };
        let v45: i32 = 4i32;
        let v46: Rc<str> = usize::try_from(v45).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v47: bool = &*v46 == "P0";
        let v51: US3 = if v47 {
            let v48: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v48.clone())
        } else {
            US3::US3_Invalid
        };
        let v52: bool = &*v46 == "P1";
        let v55: US3 = if v52 {
            let v53: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v53.clone())
        } else {
            v51.clone()
        };
        let v56: bool = &*v46 == "P2";
        let v59: US3 = if v56 {
            let v57: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v57.clone())
        } else {
            v55.clone()
        };
        let v60: bool = &*v46 == "P3";
        let v63: US3 = if v60 {
            let v61: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v61.clone())
        } else {
            v59.clone()
        };
        let mut v64: bool = &*v46 == "P4";
        let v67: US3 = if v64 {
            let v65: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v65.clone())
        } else {
            v63.clone()
        };
        let v68: bool = &*v46 == "P5";
        let v71: US3 = if v68 {
            let v69: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v69.clone())
        } else {
            v67.clone()
        };
        let v72: i32 = 5i32;
        let v73: Rc<str> = usize::try_from(v72).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v74: bool = &*v73 == "D0";
        let v78: US5 = if v74 {
            let mut v75: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v75.clone())
        } else {
            US5::US5_Invalid
        };
        let v79: bool = &*v73 == "D1";
        let v82: US5 = if v79 {
            let mut v80: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v80.clone())
        } else {
            v78.clone()
        };
        let v83: bool = &*v73 == "D2";
        let v86: US5 = if v83 {
            let v84: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v84.clone())
        } else {
            v82.clone()
        };
        let v87: bool = &*v73 == "D3";
        let mut v90: US5 = if v87 {
            let v88: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v88.clone())
        } else {
            v86.clone()
        };
        let v91: bool = &*v73 == "D4";
        let v94: US5 = if v91 {
            let v92: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v92.clone())
        } else {
            v90.clone()
        };
        let v95: bool = &*v73 == "D5";
        let v98: US5 = if v95 {
            let v96: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v96.clone())
        } else {
            v94.clone()
        };
        let v99: bool = &*v73 == "D6";
        let v102: US5 = if v99 {
            let v100: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v100.clone())
        } else {
            v98.clone()
        };
        let v103: bool = &*v73 == "D7";
        let v106: US5 = if v103 {
            let v104: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v104.clone())
        } else {
            v102.clone()
        };
        let v107: bool = &*v73 == "D8";
        let v110: US5 = if v107 {
            let v108: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v108.clone())
        } else {
            v106.clone()
        };
        match &v25 {
            US0::US0_Invalid => {
                let v343: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the id must be uppercase letters, digits and dashes"); } LIT.with(|lit| lit.clone()) };
                let v344: i32 = 2i32;
                let v345: Rc<str> = usize::try_from(v344).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                let v346: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                let v347: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                let v348: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v349: Rc<str> = std::rc::Rc::<str>::from([&*v346, &*v345, &*v347, &*v343, &*v348].concat());
                let v350: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v349 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v350
            }
            US0::US0_Valid(v111) => {
                let v111: Rc<str> = v111.clone();
                match &v44 {
                    US1::US1_Invalid => {
                        let v333: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kind must be Epic, Feature, Story or TaskKind"); } LIT.with(|lit| lit.clone()) };
                        let v334: i32 = 2i32;
                        let v335: Rc<str> = usize::try_from(v334).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                        let v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                        let v337: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                        let v338: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v339: Rc<str> = std::rc::Rc::<str>::from([&*v336, &*v335, &*v337, &*v333, &*v338].concat());
                        let v340: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v339 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                        v340
                    }
                    US1::US1_Valid(v112) => {
                        let v112: US2 = v112.clone();
                        match &v71 {
                            US3::US3_Invalid => {
                                let v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("priority must be P0..P5"); } LIT.with(|lit| lit.clone()) };
                                let v324: i32 = 2i32;
                                let v325: Rc<str> = usize::try_from(v324).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                let v326: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                let v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                let v328: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v329: Rc<str> = std::rc::Rc::<str>::from([&*v326, &*v325, &*v327, &*v323, &*v328].concat());
                                let v330: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v329 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                v330
                            }
                            US3::US3_Valid(v113) => {
                                let v113: US4 = v113.clone();
                                match &v110 {
                                    US5::US5_Invalid => {
                                        let v313: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("difficulty must be D0..D8"); } LIT.with(|lit| lit.clone()) };
                                        let v314: i32 = 2i32;
                                        let v315: Rc<str> = usize::try_from(v314).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                        let v316: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                        let v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                        let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v319: Rc<str> = std::rc::Rc::<str>::from([&*v316, &*v315, &*v317, &*v313, &*v318].concat());
                                        let v320: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v319 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                        v320
                                    }
                                    US5::US5_Valid(v114) => {
                                        let v114: US6 = v114.clone();
                                        let v115: i32 = 8i32;
                                        let v116: i32 = std::env::args().skip(2i32 as usize).count() as i32;
                                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v118: US7 = extras_from_2(v115, v116, v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone(), v117.clone());
                                        let v119: US7 = extras_checked_4(v118.clone());
                                        match &v119 {
                                            US7::US7_ExtrasRead(v120, v121, v122, v123, v124, v125) => {
                                                let v120: Rc<str> = v120.clone();
                                                let v121: Rc<str> = v121.clone();
                                                let v122: Rc<str> = v122.clone();
                                                let v123: Rc<str> = v123.clone();
                                                let v124: Rc<str> = v124.clone();
                                                let v125: Rc<str> = v125.clone();
                                                let v126: i32 = 1i32;
                                                let v127: Rc<str> = usize::try_from(v126).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v128: i32 = 6i32;
                                                let v129: Rc<str> = usize::try_from(v128).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v130: i32 = 7i32;
                                                let v131: Rc<str> = usize::try_from(v130).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v132: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let v134: Rc<str> = std::rc::Rc::<str>::from([&*v127, &*v133, &*v117, &*v117, &*v117].concat());
                                                let v135: bool = std::path::Path::new(&*v134).is_file();
                                                let v138: US10 = if v135 {
                                                    US10::US10_FileReadable(v134.clone())
                                                } else {
                                                    US10::US10_FileMissing(v134.clone())
                                                };
                                                match &v138 {
                                                    US10::US10_FileMissing(v139) => {
                                                        let v139: Rc<str> = v139.clone();
                                                        let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                        let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let v143: Rc<str> = std::rc::Rc::<str>::from([&*v141, &*v111, &*v142, &*v140, &*v117].concat());
                                                        let v144: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v143 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v144
                                                    }
                                                    US10::US10_FileReadable(v145) => {
                                                        let v145: Rc<str> = v145.clone();
                                                        let v146: Rc<str> = std::fs::read_to_string(&*v145).unwrap_or_default().into();
                                                        let mut v147: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v146);
                                                        let v148: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                        let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                        let v150: Rc<str> = std::rc::Rc::<str>::from([&*v148, &*v111, &*v149, &*v117, &*v117].concat());
                                                        let v151: bool = v147.contains(&*v150);
                                                        if v151 {
                                                            let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                            let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let v155: Rc<str> = std::rc::Rc::<str>::from([&*v153, &*v111, &*v154, &*v152, &*v117].concat());
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
                                                            let v179: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v178, &*v165, &*v117, &*v117].concat());
                                                            let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                            let v181: Rc<str> = std::rc::Rc::<str>::from([&*v159, &*v163, &*v164, &*v179, &*v180].concat());
                                                            let v182: Rc<str> = Rc::<str>::from(v129.replace(&*v166, &*v167));
                                                            let v183: Rc<str> = Rc::<str>::from(v182.replace(&*v165, &*v169));
                                                            let v184: Rc<str> = Rc::<str>::from(v183.replace(&*v157, &*v171));
                                                            let v185: Rc<str> = Rc::<str>::from(v184.replace(&*v173, &*v174));
                                                            let v186: Rc<str> = Rc::<str>::from(v185.replace(&*v176, &*v177));
                                                            let v187: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v186, &*v165, &*v117, &*v117].concat());
                                                            let v192: Rc<str> = match &v112 {
                                                                US2::US2_AgileEpic => {
                                                                    let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
                                                                    v188.clone()
                                                                }
                                                                US2::US2_AgileFeature => {
                                                                    let v189: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
                                                                    v189.clone()
                                                                }
                                                                US2::US2_AgileStory => {
                                                                    let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
                                                                    v190.clone()
                                                                }
                                                                US2::US2_AgileTaskKind => {
                                                                    let v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
                                                                    v191.clone()
                                                                }
                                                            };
                                                            let v199: Rc<str> = match &v113 {
                                                                US4::US4_AgileP0 => {
                                                                    let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
                                                                    v193.clone()
                                                                }
                                                                US4::US4_AgileP1 => {
                                                                    let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
                                                                    v194.clone()
                                                                }
                                                                US4::US4_AgileP2 => {
                                                                    let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
                                                                    v195.clone()
                                                                }
                                                                US4::US4_AgileP3 => {
                                                                    let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
                                                                    v196.clone()
                                                                }
                                                                US4::US4_AgileP4 => {
                                                                    let v197: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
                                                                    v197.clone()
                                                                }
                                                                US4::US4_AgileP5 => {
                                                                    let v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
                                                                    v198.clone()
                                                                }
                                                            };
                                                            let v200: Rc<str> = std::rc::Rc::<str>::from([&*v187, &*v180, &*v192, &*v180, &*v199].concat());
                                                            let v210: Rc<str> = match &v114 {
                                                                US6::US6_AgileD0 => {
                                                                    let v201: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
                                                                    v201.clone()
                                                                }
                                                                US6::US6_AgileD1 => {
                                                                    let v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
                                                                    v202.clone()
                                                                }
                                                                US6::US6_AgileD2 => {
                                                                    let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
                                                                    v203.clone()
                                                                }
                                                                US6::US6_AgileD3 => {
                                                                    let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
                                                                    v204.clone()
                                                                }
                                                                US6::US6_AgileD4 => {
                                                                    let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
                                                                    v205.clone()
                                                                }
                                                                US6::US6_AgileD5 => {
                                                                    let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
                                                                    v206.clone()
                                                                }
                                                                US6::US6_AgileD6 => {
                                                                    let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
                                                                    v207.clone()
                                                                }
                                                                US6::US6_AgileD7 => {
                                                                    let v208: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
                                                                    v208.clone()
                                                                }
                                                                US6::US6_AgileD8 => {
                                                                    let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
                                                                    v209.clone()
                                                                }
                                                            };
                                                            let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, "); } LIT.with(|lit| lit.clone()) };
                                                            let v212: Rc<str> = Rc::<str>::from(v123.replace(&*v166, &*v167));
                                                            let v213: Rc<str> = Rc::<str>::from(v212.replace(&*v165, &*v169));
                                                            let v214: Rc<str> = Rc::<str>::from(v213.replace(&*v157, &*v171));
                                                            let v215: Rc<str> = Rc::<str>::from(v214.replace(&*v173, &*v174));
                                                            let v216: Rc<str> = Rc::<str>::from(v215.replace(&*v176, &*v177));
                                                            let v217: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v216, &*v165, &*v117, &*v117].concat());
                                                            let v218: Rc<str> = Rc::<str>::from(v131.replace(&*v166, &*v167));
                                                            let v219: Rc<str> = Rc::<str>::from(v218.replace(&*v165, &*v169));
                                                            let v220: Rc<str> = Rc::<str>::from(v219.replace(&*v157, &*v171));
                                                            let v221: Rc<str> = Rc::<str>::from(v220.replace(&*v173, &*v174));
                                                            let v222: Rc<str> = Rc::<str>::from(v221.replace(&*v176, &*v177));
                                                            let v223: Rc<str> = std::rc::Rc::<str>::from([&*v165, &*v222, &*v165, &*v117, &*v117].concat());
                                                            let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                            let v225: Rc<str> = std::rc::Rc::<str>::from([&*v180, &*v223, &*v224, &*v117, &*v117].concat());
                                                            let v226: Rc<str> = std::rc::Rc::<str>::from([&*v180, &*v210, &*v211, &*v217, &*v225].concat());
                                                            let v227: Rc<str> = std::rc::Rc::<str>::from([&*v181, &*v200, &*v226, &*v117, &*v117].concat());
                                                            let v228: Rc<str> = std::rc::Rc::<str>::from([&*v158, &*v157, &*v227, &*v157, &*v117].concat());
                                                            let v229: US11 = extras_applied_5(v228.clone(), v111.clone(), v120.clone(), v121.clone(), v122.clone(), v123.clone(), v124.clone(), v125.clone());
                                                            match &v229 {
                                                                US11::US11_AddedRefused(v230) => {
                                                                    let v230: Rc<str> = v230.clone();
                                                                    let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                    let v233: Rc<str> = std::rc::Rc::<str>::from([&*v231, &*v111, &*v232, &*v230, &*v117].concat());
                                                                    let v234: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v233 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v234
                                                                }
                                                                US11::US11_AddedText(v235) => {
                                                                    let v235: Rc<str> = v235.clone();
                                                                    let v236: US12 = replace_records_unchanged_6(v134.clone(), v147.clone(), v235.clone());
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
                                                                                let v247: Rc<str> = std::rc::Rc::<str>::from([&*v246, &*v123, &*v117, &*v117, &*v117].concat());
                                                                                v247.clone()
                                                                            };
                                                                            let v249: bool = &*v120 == "";
                                                                            let v251: Rc<str> = if v249 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
                                                                                v250.clone()
                                                                            };
                                                                            let v252: bool = &*v121 == "";
                                                                            let v254: Rc<str> = if v252 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
                                                                                v253.clone()
                                                                            };
                                                                            let v255: bool = &*v122 == "";
                                                                            let v257: Rc<str> = if v255 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
                                                                                v256.clone()
                                                                            };
                                                                            let v258: Rc<str> = std::rc::Rc::<str>::from([&*v251, &*v254, &*v257, &*v117, &*v117].concat());
                                                                            let v259: bool = &*v258 == "";
                                                                            let v266: Rc<str> = if v259 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" planned="); } LIT.with(|lit| lit.clone()) };
                                                                                let v261: Rc<str> = std::rc::Rc::<str>::from(v258.trim());
                                                                                let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                                                let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                                let v264: Rc<str> = Rc::<str>::from(v261.replace(&*v262, &*v263));
                                                                                let v265: Rc<str> = std::rc::Rc::<str>::from([&*v260, &*v264, &*v117, &*v117, &*v117].concat());
                                                                                v265.clone()
                                                                            };
                                                                            let v267: bool = &*v124 == "";
                                                                            let v269: Rc<str> = if v267 {
                                                                                v117.clone()
                                                                            } else {
                                                                                let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
                                                                                v268.clone()
                                                                            };
                                                                            let v270: Rc<str> = std::rc::Rc::<str>::from([&*v248, &*v266, &*v269, &*v117, &*v117].concat());
                                                                            let v271: Rc<str> = std::rc::Rc::<str>::from([&*v244, &*v270, &*v117, &*v117, &*v117].concat());
                                                                            let v272: Rc<str> = std::rc::Rc::<str>::from([&*v242, &*v111, &*v243, &*v238, &*v271].concat());
                                                                            let v273: Rc<str> = std::rc::Rc::<str>::from((v239).to_string());
                                                                            let v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
                                                                            let v275: Rc<str> = Rc::<str>::from(format!("{}{}", v274, v273));
                                                                            let v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
                                                                            let v277: Rc<str> = Rc::<str>::from(format!("{}{}", v275, v276));
                                                                            let v278: Rc<str> = std::rc::Rc::<str>::from((v240).to_string());
                                                                            let v279: Rc<str> = Rc::<str>::from(format!("{}{}", v277, v278));
                                                                            let v280: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
                                                                            let v281: Rc<str> = Rc::<str>::from(format!("{}{}", v279, v280));
                                                                            let v282: Rc<str> = std::rc::Rc::<str>::from((v241).to_string());
                                                                            let v283: Rc<str> = Rc::<str>::from(format!("{}{}", v281, v282));
                                                                            let v284: Rc<str> = std::rc::Rc::<str>::from([&*v272, &*v283, &*v117, &*v117, &*v117].concat());
                                                                            let v285: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v284.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
                                                                            v285
                                                                        }
                                                                        _ => {
                                                                            let v295: Rc<str> = match &v236 {
                                                                                US12::US12_ChangedSinceRead => {
                                                                                    let v290: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                                                                                    v290.clone()
                                                                                }
                                                                                US12::US12_ReadbackMismatch => {
                                                                                    let v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v289.clone()
                                                                                }
                                                                                US12::US12_RenameFailed => {
                                                                                    let v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                                                                                    v287.clone()
                                                                                }
                                                                                US12::US12_RenameFailedTemporaryKept => {
                                                                                    let v288: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v288.clone()
                                                                                }
                                                                                US12::US12_TemporaryWriteFailed => {
                                                                                    let v286: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                                                                                    v286.clone()
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                            let v297: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                            let v298: Rc<str> = std::rc::Rc::<str>::from([&*v296, &*v111, &*v297, &*v295, &*v117].concat());
                                                                            let v299: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v298 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                            v299
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            US7::US7_ExtrasRefused(v306) => {
                                                let v306: Rc<str> = v306.clone();
                                                let v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                let v308: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                let v309: Rc<str> = std::rc::Rc::<str>::from([&*v307, &*v111, &*v308, &*v306, &*v117].concat());
                                                let v310: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v309 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                v310
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
