#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_MigrationDryRun,
    US0_MigrationApply,
}
#[derive(Clone)]
enum US1 {
    US1_Replaced,
    US1_TemporaryWriteFailed,
    US1_RenameFailed,
    US1_RenameFailedTemporaryKept,
    US1_ReadbackMismatch,
}
#[derive(Clone)]
enum US2 {
    US2_FileReadable(Rc<str>),
    US2_FileMissing(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_ScanPlain,
    US3_ScanQuoted,
    US3_ScanEscaped,
}
#[derive(Clone)]
enum US4 {
    US4_PlainTag,
    US4_PlanningTag(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_StepText(Rc<str>),
    US5_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Valid(US0),
    US6_Invalid,
}
fn migration_receipt_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64) -> i32 {
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile migrate-tags "); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
    let v9: Rc<str> = std::rc::Rc::<str>::from((v2).to_string());
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tasks="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tags_moved="); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected="); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
    let v18: Rc<str> = std::rc::Rc::<str>::from((v4).to_string());
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v8, Rc::<str>::from(format!("{}{}", v17, v18))));
    let v21: u64 = v5.len() as u64;
    let v22: u64 = v1.len() as u64;
    let v23: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v6);
    let v24: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v24));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
    let v29: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let v33: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v20, Rc::<str>::from(format!("{}{}", v32, v33))));
    let v36: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v35.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v36
}
fn migration_written_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>, mut v6: u64) -> i32 {
    let v7: bool = v2 == 0i32;
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok unchanged"); } LIT.with(|lit| lit.clone()) };
        migration_receipt_3(v8.clone(), v1.clone(), v2, v3, v4, v5.clone(), v6)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v10));
        let v12: bool = std::fs::write(&*v11, &*v1).is_ok();
        let v24: US1 = if v12 {
            let v13: bool = std::fs::rename(&*v11, &*v0).is_ok();
            if v13 {
                let v14: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v1);
                if v14 {
                    US1::US1_Replaced
                } else {
                    US1::US1_ReadbackMismatch
                }
            } else {
                let v18: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
                if v18 {
                    US1::US1_RenameFailed
                } else {
                    US1::US1_RenameFailedTemporaryKept
                }
            }
        } else {
            US1::US1_TemporaryWriteFailed
        };
        match &v24 {
            US1::US1_Replaced => {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                migration_receipt_3(v25.clone(), v1.clone(), v2, v3, v4, v5.clone(), v6)
            }
            _ => {
                let v34: Rc<str> = match &v24 {
                    US1::US1_ReadbackMismatch => {
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                        v30.clone()
                    }
                    US1::US1_RenameFailed => {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                        v28.clone()
                    }
                    US1::US1_RenameFailedTemporaryKept => {
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                        v29.clone()
                    }
                    US1::US1_TemporaryWriteFailed => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                        v27.clone()
                    }
                    _ => unreachable!(),
                };
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
                let v37: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v36 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v37
            }
        }
    }
}
fn field_end_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: US3) -> i32 {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v9: bool = match &v3 {
                US3::US3_ScanPlain => {
                    let v6: bool = v5 == 44i32;
                    if v6 {
                        true
                    } else {
                        let v7: bool = v5 == 41i32;
                        v7
                    }
                }
                _ => {
                    false
                }
            };
            if v9 {
                return v1;
            } else {
                let v10: i32 = v1.wrapping_add(1i32);
                let v25: US3 = match &v3 {
                    US3::US3_ScanEscaped => {
                        US3::US3_ScanQuoted
                    }
                    US3::US3_ScanPlain => {
                        let v19: bool = v5 == 34i32;
                        if v19 {
                            US3::US3_ScanQuoted
                        } else {
                            US3::US3_ScanPlain
                        }
                    }
                    US3::US3_ScanQuoted => {
                        let v12: bool = v5 == 92i32;
                        if v12 {
                            US3::US3_ScanEscaped
                        } else {
                            let v14: bool = v5 == 34i32;
                            if v14 {
                                US3::US3_ScanPlain
                            } else {
                                US3::US3_ScanQuoted
                            }
                        }
                    }
                };
                (v0, v1, v2, v3) = (v0.clone(), v10, v2, v25.clone());
                continue;
            }
        }
    }
}
fn skip_spaces_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.as_bytes().get(v1 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
        let v3: bool = v2 == 32i32;
        if v3 {
            let v4: i32 = v1.wrapping_add(1i32);
            (v0, v1) = (v0.clone(), v4);
            continue;
        } else {
            return v1;
        }
    }
}
fn top_level_field_count_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: US3 = US3::US3_ScanPlain;
        let v5: i32 = field_end_6(v0.clone(), v1, v2, v4.clone());
        let v6: bool = v2 <= v5;
        if v6 {
            return -1i32;
        } else {
            let v7: i32 = v0.as_bytes().get(v5 as usize).map(|byte| i32::from(*byte)).unwrap_or(-1);
            let v8: bool = v7 == 41i32;
            if v8 {
                let v9: i32 = v3.wrapping_add(1i32);
                return v9;
            } else {
                let v10: i32 = v5.wrapping_add(1i32);
                let v11: i32 = skip_spaces_7(v0.clone(), v10);
                let v12: i32 = v3.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v12);
                continue;
            }
        }
    }
}
fn first_malformed_task_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
        let v4: i32 = v0.get(v1 as usize..).and_then(|rest| rest.find(&*v3)).map(|found| (found + v1 as usize) as i32).unwrap_or(-1);
        let v5: bool = v4 < 0i32;
        if v5 {
            return -1i32;
        } else {
            let v6: i32 = v4.wrapping_add(6i32);
            let v7: i32 = v0.get(..v4 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
            let v8: Rc<str> = std::rc::Rc::<str>::from(v0.get(v7 as usize..v4 as usize).unwrap_or(""));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v8.starts_with(&*v9);
            if v10 {
                let v11: i32 = 0i32;
                let v12: i32 = top_level_field_count_5(v0.clone(), v6, v2, v11);
                let v13: bool = v12 == 11i32;
                if v13 {
                    (v0, v1, v2) = (v0.clone(), v6, v2);
                    continue;
                } else {
                    return v4;
                }
            } else {
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        }
    }
}
fn fresh_migration_8(mut v0: Rc<str>) -> (Rc<str>, i32, i32, i32) {
    (v0, 0i32, 0i32, 0i32)
}
fn field_start_10(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v3 == 0i32;
        if v4 {
            return v1;
        } else {
            let v5: US3 = US3::US3_ScanPlain;
            let v6: i32 = field_end_6(v0.clone(), v1, v2, v5.clone());
            let v7: i32 = v6.wrapping_add(1i32);
            let v8: i32 = skip_spaces_7(v0.clone(), v7);
            let v9: i32 = v3.wrapping_sub(1i32);
            (v0, v1, v2, v3) = (v0.clone(), v8, v2, v9);
            continue;
        }
    }
}
fn fresh_tag_pass_11(mut v0: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), v1.clone(), 0i32, v1.clone(), 0i32)
}
fn tag_kind_of_14(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("est:"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    if v2 {
        let v3: i32 = (v1.clone().len() as i32);
        let v4: i32 = (v0.clone().len() as i32);
        let v5: Rc<str> = std::rc::Rc::<str>::from(v0.get(v3 as usize..v4 as usize).unwrap_or(""));
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("estimate"); } LIT.with(|lit| lit.clone()) };
        US4::US4_PlanningTag(v6.clone(), v5.clone())
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load:"); } LIT.with(|lit| lit.clone()) };
        let v9: bool = v0.starts_with(&*v8);
        if v9 {
            let v10: i32 = (v8.clone().len() as i32);
            let v11: i32 = (v0.clone().len() as i32);
            let v12: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v11 as usize).unwrap_or(""));
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("load"); } LIT.with(|lit| lit.clone()) };
            US4::US4_PlanningTag(v13.clone(), v12.clone())
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agent:"); } LIT.with(|lit| lit.clone()) };
            let v16: bool = v0.starts_with(&*v15);
            if v16 {
                let v17: i32 = (v15.clone().len() as i32);
                let v18: i32 = (v0.clone().len() as i32);
                let v19: Rc<str> = std::rc::Rc::<str>::from(v0.get(v17 as usize..v18 as usize).unwrap_or(""));
                let v20: Rc<str> = agile_planning_values::kit_agile_planning_legacy_agent_value(&*v19);
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("assignee"); } LIT.with(|lit| lit.clone()) };
                US4::US4_PlanningTag(v21.clone(), v20.clone())
            } else {
                US4::US4_PlainTag
            }
        }
    }
}
fn value_token_15(mut v0: Rc<str>, mut v1: Rc<str>) -> US5 {
    let (mut v2, mut v3): (i32, Rc<str>) = agile_planning_values::kit_agile_planning_value_token(&*v0, &*v1);
    let v4: bool = v2 == 0i32;
    if v4 {
        US5::US5_StepText(v3.clone())
    } else {
        US5::US5_StepRejected(v3)
    }
}
fn rewritten_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US5 {
    let (mut v4, mut v5): (i32, Rc<str>) = agile_planning_rewrite::kit_agile_planning_rewrite(&*v0, &*v1, &*v2, &*v3);
    let v6: bool = v4 == 0i32;
    if v6 {
        US5::US5_StepText(v5.clone())
    } else {
        US5::US5_StepRejected(v5)
    }
}
fn tag_moved_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let v9: US5 = rewritten_17(v0.clone(), v5.clone(), v7.clone(), v8.clone());
    let (mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v9 {
        US5::US5_StepText(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v2.wrapping_add(1i32);
            (v10.clone(), v1.clone(), v11, v3.clone(), v4)
        }
        _ => {
            let (mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v9 {
                US5::US5_StepRejected(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: bool = &*v1 == "";
                    let v17: Rc<str> = if v13 {
                        v6.clone()
                    } else {
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v14)), v6));
                        v16.clone()
                    };
                    let v18: bool = &*v3 == "";
                    let v22: Rc<str> = if v18 {
                        v6.clone()
                    } else {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v19)), v6));
                        v21.clone()
                    };
                    let v23: i32 = v4.wrapping_add(1i32);
                    (v0.clone(), v17.clone(), v2, v22.clone(), v23)
                }
                _ => unreachable!(),
            };
            (v24.clone(), v25.clone(), v26, v27.clone(), v28)
        }
    };
    (v29.clone(), v30.clone(), v31, v32.clone(), v33)
}
fn tag_applied_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let v7: US4 = tag_kind_of_14(v6.clone());
    let (mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v7 {
        US4::US4_PlainTag => {
            let v8: bool = &*v1 == "";
            let v12: Rc<str> = if v8 {
                v6.clone()
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v9)), v6));
                v11.clone()
            };
            (v0.clone(), v12.clone(), v2, v3.clone(), v4)
        }
        _ => {
            let (mut v44, mut v45, mut v46, mut v47, mut v48): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v7 {
                US4::US4_PlanningTag(v13, v14) => {
                    let v13: Rc<str> = v13.clone();
                    let v14: Rc<str> = v14.clone();
                    let v15: US5 = value_token_15(v13.clone(), v14.clone());
                    match &v15 {
                        US5::US5_StepText(v16) => {
                            let v16: Rc<str> = v16.clone();
                            tag_moved_16(v0.clone(), v1.clone(), v2, v3.clone(), v4, v5.clone(), v6.clone(), v13.clone(), v16.clone())
                        }
                        _ => {
                            let (mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = match &v15 {
                                US5::US5_StepRejected(v22) => {
                                    let v22: Rc<str> = v22.clone();
                                    let v23: bool = &*v1 == "";
                                    let v27: Rc<str> = if v23 {
                                        v6.clone()
                                    } else {
                                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v24)), v6));
                                        v26.clone()
                                    };
                                    let v28: bool = &*v3 == "";
                                    let v32: Rc<str> = if v28 {
                                        v6.clone()
                                    } else {
                                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                                        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v29)), v6));
                                        v31.clone()
                                    };
                                    let v33: i32 = v4.wrapping_add(1i32);
                                    (v0.clone(), v27.clone(), v2, v32.clone(), v33)
                                }
                                _ => unreachable!(),
                            };
                            (v34.clone(), v35.clone(), v36, v37.clone(), v38)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v44.clone(), v45.clone(), v46, v47.clone(), v48)
        }
    };
    (v49.clone(), v50.clone(), v51, v52.clone(), v53)
}
fn tags_applied_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: Rc<str>, mut v4: i32, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32) -> (Rc<str>, Rc<str>, i32, Rc<str>, i32) {
    let v8: i32 = (v6.clone().len() as i32);
    let v9: bool = v8 <= v7;
    let (mut v33, mut v34, mut v35, mut v36, mut v37): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = if v9 {
        (v0.clone(), v1.clone(), v2, v3.clone(), v4)
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
        let v11: i32 = v6.get(v7 as usize..).and_then(|rest| rest.find(&*v10)).map(|found| (found + v7 as usize) as i32).unwrap_or(-1);
        let v12: bool = v11 < 0i32;
        let v13: i32 = if v12 {
            v8
        } else {
            v11
        };
        let v14: Rc<str> = std::rc::Rc::<str>::from(v6.get(v7 as usize..v13 as usize).unwrap_or(""));
        let v15: i32 = (v14.clone().len() as i32);
        let v16: bool = v15 == 0i32;
        let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = if v16 {
            (v0.clone(), v1.clone(), v2, v3.clone(), v4)
        } else {
            let (mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = tag_applied_13(v0.clone(), v1.clone(), v2, v3.clone(), v4, v5.clone(), v14.clone());
            (v17.clone(), v18.clone(), v19, v20.clone(), v21)
        };
        let v27: i32 = v13.wrapping_add(1i32);
        let (mut v28, mut v29, mut v30, mut v31, mut v32): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = tags_applied_12(v22.clone(), v23.clone(), v24, v25.clone(), v26, v5.clone(), v6.clone(), v27);
        (v28.clone(), v29.clone(), v30, v31.clone(), v32)
    };
    (v33.clone(), v34.clone(), v35, v36.clone(), v37)
}
fn tags_replaced_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: i32 = (v0.clone().len() as i32);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
    let v8: i32 = v0.find(&*v7).map(|offset| offset as i32).unwrap_or(-1);
    let v9: i32 = v8.wrapping_add(6i32);
    let v10: i32 = 8i32;
    let v11: i32 = field_start_10(v0.clone(), v9, v3, v10);
    let v12: US3 = US3::US3_ScanPlain;
    let v13: i32 = field_end_6(v0.clone(), v11, v3, v12.clone());
    let v14: Rc<str> = std::rc::Rc::<str>::from(v0.get(0i32 as usize..v11 as usize).unwrap_or(""));
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\\\"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v16, &v17));
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
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v29, &*v15, &*v30, &*v30].concat());
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v31));
    let v33: Rc<str> = std::rc::Rc::<str>::from(v0.get(v13 as usize..v3 as usize).unwrap_or(""));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
    v34.clone()
}
fn task_report_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i32, mut v4: Rc<str>, mut v5: i32) -> i32 {
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" moved="); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v6));
    let v8: Rc<str> = std::rc::Rc::<str>::from((v3).to_string());
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v1, &*v0);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tags="); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v14)), v2));
    let v17: bool = v5 == 0i32;
    let v21: Rc<str> = if v17 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v18.clone()
    } else {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected="); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v4));
        v20.clone()
    };
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v21));
    let v23: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v22.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
    v23
}
fn task_migrated_18(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32, i32, i32) {
    let v10: Rc<str> = tags_replaced_19(v5, v4.clone(), v6.clone());
    let v11: i32 = task_report_20(v4, v10.clone(), v6, v7, v8, v9);
    let v12: i32 = v1.wrapping_add(1i32);
    let v13: i32 = v2.wrapping_add(v7);
    let v14: i32 = v3.wrapping_add(v9);
    (v10, v12, v13, v14)
}
fn task_with_rejections_21(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: i32, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, i32, i32, i32) {
    let v10: i32 = task_report_20(v4, v5, v6, v7, v8, v9);
    let v11: i32 = v3.wrapping_add(v9);
    (v0, v1, v2, v11)
}
fn migrated_from_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32) -> (Rc<str>, i32, i32, i32) {
    let v5: i32 = (v0.clone().len() as i32);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Task (\""); } LIT.with(|lit| lit.clone()) };
    let v7: i32 = v0.get(v4 as usize..).and_then(|rest| rest.find(&*v6)).map(|found| (found + v4 as usize) as i32).unwrap_or(-1);
    let v8: bool = v7 < 0i32;
    let (mut v88, mut v89, mut v90, mut v91): (Rc<str>, i32, i32, i32) = if v8 {
        (v0.clone(), v1, v2, v3)
    } else {
        let v9: i32 = v7.wrapping_add(6i32);
        let v10: i32 = v0.get(..v7 as usize).and_then(|head| head.rfind(char::from(10u8))).map(|newline| newline as i32 + 1).unwrap_or(0);
        let v11: Rc<str> = std::rc::Rc::<str>::from(v0.get(v10 as usize..v7 as usize).unwrap_or(""));
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("inl "); } LIT.with(|lit| lit.clone()) };
        let v13: bool = v11.starts_with(&*v12);
        let (mut v84, mut v85, mut v86, mut v87): (Rc<str>, i32, i32, i32) = if v13 {
            let v14: i32 = 0i32;
            let v15: i32 = field_start_10(v0.clone(), v9, v5, v14);
            let v16: US3 = US3::US3_ScanPlain;
            let v17: i32 = field_end_6(v0.clone(), v15, v5, v16.clone());
            let v18: Rc<str> = std::rc::Rc::<str>::from(v0.get(v15 as usize..v17 as usize).unwrap_or(""));
            let v19: i32 = (v18.clone().len() as i32);
            let v20: i32 = v19.wrapping_sub(1i32);
            let v21: Rc<str> = std::rc::Rc::<str>::from(v18.get(1i32 as usize..v20 as usize).unwrap_or(""));
            let (mut v22, mut v23, mut v24, mut v25, mut v26): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = fresh_tag_pass_11(v0.clone());
            let v27: i32 = 8i32;
            let v28: i32 = field_start_10(v0.clone(), v9, v5, v27);
            let v29: US3 = US3::US3_ScanPlain;
            let v30: i32 = field_end_6(v0.clone(), v28, v5, v29.clone());
            let v31: Rc<str> = std::rc::Rc::<str>::from(v0.get(v28 as usize..v30 as usize).unwrap_or(""));
            let v32: i32 = (v31.clone().len() as i32);
            let v33: i32 = v32.wrapping_sub(1i32);
            let v34: Rc<str> = std::rc::Rc::<str>::from(v31.get(1i32 as usize..v33 as usize).unwrap_or(""));
            let v35: i32 = 0i32;
            let (mut v36, mut v37, mut v38, mut v39, mut v40): (Rc<str>, Rc<str>, i32, Rc<str>, i32) = tags_applied_12(v22.clone(), v23.clone(), v24, v25.clone(), v26, v21.clone(), v34.clone(), v35);
            let v41: bool = 1i32 <= v38;
            let (mut v76, mut v77, mut v78, mut v79): (Rc<str>, i32, i32, i32) = if v41 {
                let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, i32, i32, i32) = task_migrated_18(v0.clone(), v1, v2, v3, v21.clone(), v36.clone(), v37.clone(), v38, v39.clone(), v40);
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v21));
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\","); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
                let v49: i32 = v42.find(&*v48).map(|offset| offset as i32).unwrap_or(-1);
                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v51: i32 = v42.get(v49 as usize..).and_then(|rest| rest.find(&*v50)).map(|found| (found + v49 as usize) as i32).unwrap_or(-1);
                let v52: bool = v51 < 0i32;
                let v54: i32 = if v52 {
                    let v53: i32 = (v42.clone().len() as i32);
                    v53
                } else {
                    v51
                };
                let (mut v55, mut v56, mut v57, mut v58): (Rc<str>, i32, i32, i32) = migrated_from_9(v42.clone(), v43, v44, v45, v54);
                (v55.clone(), v56, v57, v58)
            } else {
                let v59: bool = 1i32 <= v40;
                let (mut v72, mut v73, mut v74, mut v75): (Rc<str>, i32, i32, i32) = if v59 {
                    let (mut v60, mut v61, mut v62, mut v63): (Rc<str>, i32, i32, i32) = task_with_rejections_21(v0.clone(), v1, v2, v3, v21.clone(), v36.clone(), v37.clone(), v38, v39.clone(), v40);
                    let (mut v64, mut v65, mut v66, mut v67): (Rc<str>, i32, i32, i32) = migrated_from_9(v60.clone(), v61, v62, v63, v9);
                    (v64.clone(), v65, v66, v67)
                } else {
                    let (mut v68, mut v69, mut v70, mut v71): (Rc<str>, i32, i32, i32) = migrated_from_9(v0.clone(), v1, v2, v3, v9);
                    (v68.clone(), v69, v70, v71)
                };
                (v72.clone(), v73, v74, v75)
            };
            (v76.clone(), v77, v78, v79)
        } else {
            let (mut v80, mut v81, mut v82, mut v83): (Rc<str>, i32, i32, i32) = migrated_from_9(v0.clone(), v1, v2, v3, v9);
            (v80.clone(), v81, v82, v83)
        };
        (v84.clone(), v85, v86, v87)
    };
    (v88.clone(), v89, v90, v91)
}
fn tags_migration_1(mut v0: Rc<str>, mut v1: US0) -> i32 {
    let v2: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v3, &*v4, &*v4, &*v4].concat());
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
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            let v14: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v13 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
            v14
        }
        US2::US2_FileReadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
            let v17: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v16);
            let v18: i32 = 0i32;
            let v19: i32 = (v17.clone().len() as i32);
            let v20: i32 = first_malformed_task_4(v17.clone(), v18, v19);
            let v21: bool = v20 < 0i32;
            if v21 {
                let (mut v22, mut v23, mut v24, mut v25): (Rc<str>, i32, i32, i32) = fresh_migration_8(v17.clone());
                let v26: i32 = 0i32;
                let (mut v27, mut v28, mut v29, mut v30): (Rc<str>, i32, i32, i32) = migrated_from_9(v22.clone(), v23, v24, v25, v26);
                match &v1 {
                    US0::US0_MigrationApply => {
                        migration_written_2(v5.clone(), v27.clone(), v28, v29, v30, v17.clone(), v2)
                    }
                    US0::US0_MigrationDryRun => {
                        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
                        migration_receipt_3(v31.clone(), v27.clone(), v28, v29, v30, v17.clone(), v2)
                    }
                }
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("malformed Task fields"); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: "); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: malformed Task fields"); } LIT.with(|lit| lit.clone()) };
                let v39: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v38 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v39
            }
        }
    }
}
fn command_line_argument_count_23() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_22() -> i32 {
    let v0: i32 = command_line_argument_count_23();
    let v1: bool = v0 > 0i32;
    if v1 {
        let v2: i32 = v0.wrapping_sub(1i32);
        v2
    } else {
        0i32
    }
}
fn tags_migration_entry_0() -> i32 {
    let v0: i32 = argument_count_22();
    let v1: bool = v0 == 3i32;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: bool = &*v3 == "apply";
        let v12: US6 = if v4 {
            let v5: US0 = US0::US0_MigrationApply;
            US6::US6_Valid(v5.clone())
        } else {
            let v7: bool = &*v3 == "preview";
            if v7 {
                let v8: US0 = US0::US0_MigrationDryRun;
                US6::US6_Valid(v8.clone())
            } else {
                US6::US6_Invalid
            }
        };
        match &v12 {
            US6::US6_Invalid => {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
                let v20: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v19 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
                v20
            }
            US6::US6_Valid(v13) => {
                let v13: US0 = v13.clone();
                let v14: i32 = 2i32;
                let v15: Rc<str> = usize::try_from(v14).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
                tags_migration_1(v15.clone(), v13.clone())
            }
        }
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile migrate-tags rejected: migrate-tags expects preview <root> or apply <root>"); } LIT.with(|lit| lit.clone()) };
        let v26: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v25 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
        v26
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        tags_migration_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_migrate_tags_run() -> i32 {
    closure0()()
}
