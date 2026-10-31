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
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Epic"); } LIT.with(|lit| lit.clone()) };
        let v29: bool = v27 == v28 ;
        let mut v33: US1 = if v29 {
            let v30: US2 = US2::US2_AgileEpic;
            US1::US1_Valid(v30.clone())
        } else {
            US1::US1_Invalid
        };
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Feature"); } LIT.with(|lit| lit.clone()) };
        let v35: bool = v27 == v34 ;
        let v38: US1 = if v35 {
            let v36: US2 = US2::US2_AgileFeature;
            US1::US1_Valid(v36.clone())
        } else {
            v33.clone()
        };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Story"); } LIT.with(|lit| lit.clone()) };
        let v40: bool = v27 == v39 ;
        let v43: US1 = if v40 {
            let v41: US2 = US2::US2_AgileStory;
            US1::US1_Valid(v41.clone())
        } else {
            v38.clone()
        };
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("TaskKind"); } LIT.with(|lit| lit.clone()) };
        let v45: bool = v27 == v44 ;
        let v48: US1 = if v45 {
            let v46: US2 = US2::US2_AgileTaskKind;
            US1::US1_Valid(v46.clone())
        } else {
            v43.clone()
        };
        let v49: i32 = 4i32;
        let v50: Rc<str> = usize::try_from(v49).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P0"); } LIT.with(|lit| lit.clone()) };
        let v52: bool = v50 == v51 ;
        let v56: US3 = if v52 {
            let v53: US4 = US4::US4_AgileP0;
            US3::US3_Valid(v53.clone())
        } else {
            US3::US3_Invalid
        };
        let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P1"); } LIT.with(|lit| lit.clone()) };
        let v58: bool = v50 == v57 ;
        let v61: US3 = if v58 {
            let v59: US4 = US4::US4_AgileP1;
            US3::US3_Valid(v59.clone())
        } else {
            v56.clone()
        };
        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P2"); } LIT.with(|lit| lit.clone()) };
        let v63: bool = v50 == v62 ;
        let v66: US3 = if v63 {
            let mut v64: US4 = US4::US4_AgileP2;
            US3::US3_Valid(v64.clone())
        } else {
            v61.clone()
        };
        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P3"); } LIT.with(|lit| lit.clone()) };
        let v68: bool = v50 == v67 ;
        let v71: US3 = if v68 {
            let v69: US4 = US4::US4_AgileP3;
            US3::US3_Valid(v69.clone())
        } else {
            v66.clone()
        };
        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P4"); } LIT.with(|lit| lit.clone()) };
        let v73: bool = v50 == v72 ;
        let v76: US3 = if v73 {
            let v74: US4 = US4::US4_AgileP4;
            US3::US3_Valid(v74.clone())
        } else {
            v71.clone()
        };
        let v77: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("P5"); } LIT.with(|lit| lit.clone()) };
        let v78: bool = v50 == v77 ;
        let v81: US3 = if v78 {
            let v79: US4 = US4::US4_AgileP5;
            US3::US3_Valid(v79.clone())
        } else {
            v76.clone()
        };
        let v82: i32 = 5i32;
        let v83: Rc<str> = usize::try_from(v82).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D0"); } LIT.with(|lit| lit.clone()) };
        let mut v85: bool = v83 == v84 ;
        let v89: US5 = if v85 {
            let v86: US6 = US6::US6_AgileD0;
            US5::US5_Valid(v86.clone())
        } else {
            US5::US5_Invalid
        };
        let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D1"); } LIT.with(|lit| lit.clone()) };
        let v91: bool = v83 == v90 ;
        let v94: US5 = if v91 {
            let v92: US6 = US6::US6_AgileD1;
            US5::US5_Valid(v92.clone())
        } else {
            v89.clone()
        };
        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D2"); } LIT.with(|lit| lit.clone()) };
        let v96: bool = v83 == v95 ;
        let v99: US5 = if v96 {
            let v97: US6 = US6::US6_AgileD2;
            US5::US5_Valid(v97.clone())
        } else {
            v94.clone()
        };
        let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D3"); } LIT.with(|lit| lit.clone()) };
        let v101: bool = v83 == v100 ;
        let v104: US5 = if v101 {
            let v102: US6 = US6::US6_AgileD3;
            US5::US5_Valid(v102.clone())
        } else {
            v99.clone()
        };
        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D4"); } LIT.with(|lit| lit.clone()) };
        let v106: bool = v83 == v105 ;
        let v109: US5 = if v106 {
            let v107: US6 = US6::US6_AgileD4;
            US5::US5_Valid(v107.clone())
        } else {
            v104.clone()
        };
        let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D5"); } LIT.with(|lit| lit.clone()) };
        let v111: bool = v83 == v110 ;
        let v114: US5 = if v111 {
            let v112: US6 = US6::US6_AgileD5;
            US5::US5_Valid(v112.clone())
        } else {
            v109.clone()
        };
        let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D6"); } LIT.with(|lit| lit.clone()) };
        let v116: bool = v83 == v115 ;
        let v119: US5 = if v116 {
            let v117: US6 = US6::US6_AgileD6;
            US5::US5_Valid(v117.clone())
        } else {
            v114.clone()
        };
        let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D7"); } LIT.with(|lit| lit.clone()) };
        let v121: bool = v83 == v120 ;
        let v124: US5 = if v121 {
            let v122: US6 = US6::US6_AgileD7;
            US5::US5_Valid(v122.clone())
        } else {
            v119.clone()
        };
        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("D8"); } LIT.with(|lit| lit.clone()) };
        let v126: bool = v83 == v125 ;
        let v129: US5 = if v126 {
            let v127: US6 = US6::US6_AgileD8;
            US5::US5_Valid(v127.clone())
        } else {
            v124.clone()
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
            US0::US0_Valid(v130) => {
                let v130: Rc<str> = v130.clone();
                match &v48 {
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
                    US1::US1_Valid(v131) => {
                        let v131: US2 = v131.clone();
                        match &v81 {
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
                            US3::US3_Valid(v132) => {
                                let v132: US4 = v132.clone();
                                match &v129 {
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
                                    US5::US5_Valid(v133) => {
                                        let v133: US6 = v133.clone();
                                        let v134: i32 = 8i32;
                                        let v135: i32 = std::env::args().skip(2i32 as usize).count() as i32;
                                        let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        let v137: US7 = extras_from_2(v134, v135, v136.clone(), v136.clone(), v136.clone(), v136.clone(), v136.clone(), v136.clone());
                                        let v138: US7 = extras_checked_4(v137.clone());
                                        match &v138 {
                                            US7::US7_ExtrasRead(v139, v140, v141, v142, v143, v144) => {
                                                let v139: Rc<str> = v139.clone();
                                                let v140: Rc<str> = v140.clone();
                                                let v141: Rc<str> = v141.clone();
                                                let v142: Rc<str> = v142.clone();
                                                let v143: Rc<str> = v143.clone();
                                                let v144: Rc<str> = v144.clone();
                                                let v145: i32 = 1i32;
                                                let v146: Rc<str> = usize::try_from(v145).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v147: i32 = 6i32;
                                                let v148: Rc<str> = usize::try_from(v147).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v149: i32 = 7i32;
                                                let v150: Rc<str> = usize::try_from(v149).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                                                let v151: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                                                let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
                                                let v153: Rc<str> = std::rc::Rc::<str>::from([&*v146, &*v152, &*v136, &*v136, &*v136].concat());
                                                let v154: bool = std::path::Path::new(&*v153).is_file();
                                                let v157: US10 = if v154 {
                                                    US10::US10_FileReadable(v153.clone())
                                                } else {
                                                    US10::US10_FileMissing(v153.clone())
                                                };
                                                match &v157 {
                                                    US10::US10_FileMissing(v158) => {
                                                        let v158: Rc<str> = v158.clone();
                                                        let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
                                                        let v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                        let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                        let v162: Rc<str> = std::rc::Rc::<str>::from([&*v160, &*v130, &*v161, &*v159, &*v136].concat());
                                                        let v163: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v162 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                        v163
                                                    }
                                                    US10::US10_FileReadable(v164) => {
                                                        let v164: Rc<str> = v164.clone();
                                                        let v165: Rc<str> = std::fs::read_to_string(&*v164).unwrap_or_default().into();
                                                        let mut v166: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v165);
                                                        let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
                                                        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                                                        let v169: Rc<str> = std::rc::Rc::<str>::from([&*v167, &*v130, &*v168, &*v136, &*v136].concat());
                                                        let v170: bool = v166.contains(&*v169);
                                                        if v170 {
                                                            let v171: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a task with this id already exists"); } LIT.with(|lit| lit.clone()) };
                                                            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                            let v173: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                            let v174: Rc<str> = std::rc::Rc::<str>::from([&*v172, &*v130, &*v173, &*v171, &*v136].concat());
                                                            let v175: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v174 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                            v175
                                                        } else {
                                                            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v177: Rc<str> = std::rc::Rc::<str>::from(v166.trim_end_matches(&*v176));
                                                            let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
                                                            let v179: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                                                            let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v181: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v179, &v180));
                                                            let v182: Rc<str> = std::rc::Rc::<str>::from(v181.to_ascii_lowercase());
                                                            let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" () = Task ("); } LIT.with(|lit| lit.clone()) };
                                                            let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                            let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
                                                            let v187: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v185, &v186));
                                                            let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\""); } LIT.with(|lit| lit.clone()) };
                                                            let v189: Rc<str> = std::rc::Rc::<str>::from(v187.replace(&*v184, &v188));
                                                            let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\n"); } LIT.with(|lit| lit.clone()) };
                                                            let v191: Rc<str> = std::rc::Rc::<str>::from(v189.replace(&*v176, &v190));
                                                            let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v193: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\r"); } LIT.with(|lit| lit.clone()) };
                                                            let v194: Rc<str> = std::rc::Rc::<str>::from(v191.replace(&*v192, &v193));
                                                            let v195: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\t"); } LIT.with(|lit| lit.clone()) };
                                                            let v197: Rc<str> = std::rc::Rc::<str>::from(v194.replace(&*v195, &v196));
                                                            let v198: Rc<str> = std::rc::Rc::<str>::from([&*v184, &*v197, &*v184, &*v136, &*v136].concat());
                                                            let v199: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                                                            let v200: Rc<str> = std::rc::Rc::<str>::from([&*v178, &*v182, &*v183, &*v198, &*v199].concat());
                                                            let v201: Rc<str> = std::rc::Rc::<str>::from(v148.replace(&*v185, &v186));
                                                            let v202: Rc<str> = std::rc::Rc::<str>::from(v201.replace(&*v184, &v188));
                                                            let v203: Rc<str> = std::rc::Rc::<str>::from(v202.replace(&*v176, &v190));
                                                            let v204: Rc<str> = std::rc::Rc::<str>::from(v203.replace(&*v192, &v193));
                                                            let v205: Rc<str> = std::rc::Rc::<str>::from(v204.replace(&*v195, &v196));
                                                            let v206: Rc<str> = std::rc::Rc::<str>::from([&*v184, &*v205, &*v184, &*v136, &*v136].concat());
                                                            let v207: Rc<str> = match &v131 {
                                                                US2::US2_AgileEpic => {
                                                                    v28.clone()
                                                                }
                                                                US2::US2_AgileFeature => {
                                                                    v34.clone()
                                                                }
                                                                US2::US2_AgileStory => {
                                                                    v39.clone()
                                                                }
                                                                US2::US2_AgileTaskKind => {
                                                                    v44.clone()
                                                                }
                                                            };
                                                            let v208: Rc<str> = match &v132 {
                                                                US4::US4_AgileP0 => {
                                                                    v51.clone()
                                                                }
                                                                US4::US4_AgileP1 => {
                                                                    v57.clone()
                                                                }
                                                                US4::US4_AgileP2 => {
                                                                    v62.clone()
                                                                }
                                                                US4::US4_AgileP3 => {
                                                                    v67.clone()
                                                                }
                                                                US4::US4_AgileP4 => {
                                                                    v72.clone()
                                                                }
                                                                US4::US4_AgileP5 => {
                                                                    v77.clone()
                                                                }
                                                            };
                                                            let v209: Rc<str> = std::rc::Rc::<str>::from([&*v206, &*v199, &*v207, &*v199, &*v208].concat());
                                                            let v210: Rc<str> = match &v133 {
                                                                US6::US6_AgileD0 => {
                                                                    v84.clone()
                                                                }
                                                                US6::US6_AgileD1 => {
                                                                    v90.clone()
                                                                }
                                                                US6::US6_AgileD2 => {
                                                                    v95.clone()
                                                                }
                                                                US6::US6_AgileD3 => {
                                                                    v100.clone()
                                                                }
                                                                US6::US6_AgileD4 => {
                                                                    v105.clone()
                                                                }
                                                                US6::US6_AgileD5 => {
                                                                    v110.clone()
                                                                }
                                                                US6::US6_AgileD6 => {
                                                                    v115.clone()
                                                                }
                                                                US6::US6_AgileD7 => {
                                                                    v120.clone()
                                                                }
                                                                US6::US6_AgileD8 => {
                                                                    v125.clone()
                                                                }
                                                            };
                                                            let v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", 0u32, Planned, "); } LIT.with(|lit| lit.clone()) };
                                                            let v212: Rc<str> = std::rc::Rc::<str>::from(v142.replace(&*v185, &v186));
                                                            let v213: Rc<str> = std::rc::Rc::<str>::from(v212.replace(&*v184, &v188));
                                                            let v214: Rc<str> = std::rc::Rc::<str>::from(v213.replace(&*v176, &v190));
                                                            let v215: Rc<str> = std::rc::Rc::<str>::from(v214.replace(&*v192, &v193));
                                                            let v216: Rc<str> = std::rc::Rc::<str>::from(v215.replace(&*v195, &v196));
                                                            let v217: Rc<str> = std::rc::Rc::<str>::from([&*v184, &*v216, &*v184, &*v136, &*v136].concat());
                                                            let v218: Rc<str> = std::rc::Rc::<str>::from(v150.replace(&*v185, &v186));
                                                            let v219: Rc<str> = std::rc::Rc::<str>::from(v218.replace(&*v184, &v188));
                                                            let v220: Rc<str> = std::rc::Rc::<str>::from(v219.replace(&*v176, &v190));
                                                            let v221: Rc<str> = std::rc::Rc::<str>::from(v220.replace(&*v192, &v193));
                                                            let v222: Rc<str> = std::rc::Rc::<str>::from(v221.replace(&*v195, &v196));
                                                            let v223: Rc<str> = std::rc::Rc::<str>::from([&*v184, &*v222, &*v184, &*v136, &*v136].concat());
                                                            let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", \"current\", \"current\")"); } LIT.with(|lit| lit.clone()) };
                                                            let v225: Rc<str> = std::rc::Rc::<str>::from([&*v199, &*v223, &*v224, &*v136, &*v136].concat());
                                                            let v226: Rc<str> = std::rc::Rc::<str>::from([&*v199, &*v210, &*v211, &*v217, &*v225].concat());
                                                            let v227: Rc<str> = std::rc::Rc::<str>::from([&*v200, &*v209, &*v226, &*v136, &*v136].concat());
                                                            let v228: Rc<str> = std::rc::Rc::<str>::from([&*v177, &*v176, &*v227, &*v176, &*v136].concat());
                                                            let v229: US11 = extras_applied_5(v228.clone(), v130.clone(), v139.clone(), v140.clone(), v141.clone(), v142.clone(), v143.clone(), v144.clone());
                                                            match &v229 {
                                                                US11::US11_AddedRefused(v230) => {
                                                                    let v230: Rc<str> = v230.clone();
                                                                    let v231: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile add "); } LIT.with(|lit| lit.clone()) };
                                                                    let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
                                                                    let v233: Rc<str> = std::rc::Rc::<str>::from([&*v231, &*v130, &*v232, &*v230, &*v136].concat());
                                                                    let v234: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v233 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                                                                    v234
                                                                }
                                                                US11::US11_AddedText(v235) => {
                                                                    let v235: Rc<str> = v235.clone();
                                                                    let v236: US12 = replace_records_unchanged_6(v153.clone(), v166.clone(), v235.clone());
                                                                    match &v236 {
                                                                        US12::US12_Replaced => {
                                                                            let mut v237: Rc<str> = std::rc::Rc::<str>::from(v130.replace(&*v179, &v180));
                                                                            let v238: Rc<str> = std::rc::Rc::<str>::from(v237.to_ascii_lowercase());
                                                                            let v239: u64 = v166.len() as u64;
                                                                            let v240: u64 = v235.len() as u64;
                                                                            let v241: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v151);
                                                                            let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile add ok id="); } LIT.with(|lit| lit.clone()) };
                                                                            let v243: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" binding="); } LIT.with(|lit| lit.clone()) };
                                                                            let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" status=Planned progress=0"); } LIT.with(|lit| lit.clone()) };
                                                                            let v245: bool = &*v142 == "";
                                                                            let v248: Rc<str> = if v245 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deps="); } LIT.with(|lit| lit.clone()) };
                                                                                let v247: Rc<str> = std::rc::Rc::<str>::from([&*v246, &*v142, &*v136, &*v136, &*v136].concat());
                                                                                v247.clone()
                                                                            };
                                                                            let v249: bool = &*v139 == "";
                                                                            let v251: Rc<str> = if v249 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v250: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" estimate"); } LIT.with(|lit| lit.clone()) };
                                                                                v250.clone()
                                                                            };
                                                                            let v252: bool = &*v140 == "";
                                                                            let v254: Rc<str> = if v252 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" load"); } LIT.with(|lit| lit.clone()) };
                                                                                v253.clone()
                                                                            };
                                                                            let v255: bool = &*v141 == "";
                                                                            let v257: Rc<str> = if v255 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" assignee"); } LIT.with(|lit| lit.clone()) };
                                                                                v256.clone()
                                                                            };
                                                                            let v258: Rc<str> = std::rc::Rc::<str>::from([&*v251, &*v254, &*v257, &*v136, &*v136].concat());
                                                                            let v259: bool = &*v258 == "";
                                                                            let v266: Rc<str> = if v259 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v260: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" planned="); } LIT.with(|lit| lit.clone()) };
                                                                                let v261: Rc<str> = std::rc::Rc::<str>::from(v258.trim());
                                                                                let v262: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                                                let v263: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                                                                let v264: Rc<str> = std::rc::Rc::<str>::from(v261.replace(&*v262, &v263));
                                                                                let v265: Rc<str> = std::rc::Rc::<str>::from([&*v260, &*v264, &*v136, &*v136, &*v136].concat());
                                                                                v265.clone()
                                                                            };
                                                                            let v267: bool = &*v143 == "";
                                                                            let v269: Rc<str> = if v267 {
                                                                                v136.clone()
                                                                            } else {
                                                                                let v268: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" notes=1"); } LIT.with(|lit| lit.clone()) };
                                                                                v268.clone()
                                                                            };
                                                                            let v270: Rc<str> = std::rc::Rc::<str>::from([&*v248, &*v266, &*v269, &*v136, &*v136].concat());
                                                                            let v271: Rc<str> = std::rc::Rc::<str>::from([&*v244, &*v270, &*v136, &*v136, &*v136].concat());
                                                                            let v272: Rc<str> = std::rc::Rc::<str>::from([&*v242, &*v130, &*v243, &*v238, &*v271].concat());
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
                                                                            let v284: Rc<str> = std::rc::Rc::<str>::from([&*v272, &*v283, &*v136, &*v136, &*v136].concat());
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
                                                                            let v298: Rc<str> = std::rc::Rc::<str>::from([&*v296, &*v130, &*v297, &*v295, &*v136].concat());
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
                                                let v309: Rc<str> = std::rc::Rc::<str>::from([&*v307, &*v130, &*v308, &*v306, &*v136].concat());
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
