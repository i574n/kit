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
    US1_StepText(Rc<str>),
    US1_StepRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_PlanningUnchanged,
    US2_PlanningRewritten(Rc<str>),
    US2_PlanningRefused(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_FileReadable(Rc<str>),
    US3_FileMissing(Rc<str>),
}
fn planning_edit_failure_2(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile edit "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rejected: "); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v0));
    let v7: i32 = std::io::Write::write_all(&mut std::io::stderr(), ("kit error: ".to_string() + &*v6 + "
").as_bytes()).map(|_| 2i32).unwrap_or(2i32);
    v7
}
fn renamed_within_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> bool {
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
fn replace_records_unchanged_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US0 {
    let v3: bool = kit_agile_records::kit_agile_records_unchanged(&*v0, &*v1);
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".pending"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
        let v6: bool = std::fs::write(&*v5, &*v2).is_ok();
        if v6 {
            let v7: i32 = 40i32;
            let v8: bool = renamed_within_6(v0.clone(), v5.clone(), v7);
            if v8 {
                let v9: bool = std::fs::read_to_string(&*v0).is_ok_and(|value| value == *v2);
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
fn planning_written_4(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let v6: US0 = replace_records_unchanged_5(v0.clone(), v3.clone(), v4.clone());
    match &v6 {
        US0::US0_Replaced => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile edit ok id="); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v8, v9)), v2));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            let v14: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v4, &*v1);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: u64 = v3.len() as u64;
            let v17: u64 = v4.len() as u64;
            let v18: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
            let v19: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
            let v24: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let v28: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v15, Rc::<str>::from(format!("{}{}", v27, v28))));
            let v31: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v30.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v31
        }
        _ => {
            let v41: Rc<str> = match &v6 {
                US0::US0_ChangedSinceRead => {
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the file changed since it was read; nothing written, run the call again"); } LIT.with(|lit| lit.clone()) };
                    v36.clone()
                }
                US0::US0_ReadbackMismatch => {
                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("readback did not match the written file"); } LIT.with(|lit| lit.clone()) };
                    v35.clone()
                }
                US0::US0_RenameFailed => {
                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place"); } LIT.with(|lit| lit.clone()) };
                    v33.clone()
                }
                US0::US0_RenameFailedTemporaryKept => {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not move the replacement file into place or remove its .pending copy"); } LIT.with(|lit| lit.clone()) };
                    v34.clone()
                }
                US0::US0_TemporaryWriteFailed => {
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("could not write the replacement file"); } LIT.with(|lit| lit.clone()) };
                    v32.clone()
                }
                _ => unreachable!(),
            };
            planning_edit_failure_2(v41.clone(), v1.clone())
        }
    }
}
fn rewritten_7(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> US1 {
    let (mut v4, mut v5): (i32, Rc<str>) = agile_planning_rewrite::kit_agile_planning_rewrite(&*v0, &*v1, &*v2, &*v3);
    let v6: bool = v4 == 0i32;
    if v6 {
        US1::US1_StepText(v5.clone())
    } else {
        US1::US1_StepRejected(v5)
    }
}
fn planning_revised_3(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64) -> i32 {
    let v6: US1 = rewritten_7(v4.clone(), v1.clone(), v2.clone(), v3.clone());
    let v15: US2 = match &v6 {
        US1::US1_StepRejected(v7) => {
            let v7: Rc<str> = v7.clone();
            US2::US2_PlanningRefused(v7.clone())
        }
        US1::US1_StepText(v9) => {
            let v9: Rc<str> = v9.clone();
            let v10: bool = &*v9 == &*v4;
            if v10 {
                US2::US2_PlanningUnchanged
            } else {
                US2::US2_PlanningRewritten(v9.clone())
            }
        }
    };
    match &v15 {
        US2::US2_PlanningRefused(v16) => {
            let v16: Rc<str> = v16.clone();
            planning_edit_failure_2(v16.clone(), v1.clone())
        }
        US2::US2_PlanningRewritten(v43) => {
            let v43: Rc<str> = v43.clone();
            planning_written_4(v0.clone(), v1.clone(), v2.clone(), v4.clone(), v43.clone(), v5)
        }
        US2::US2_PlanningUnchanged => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile edit ok id="); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v1));
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" field="); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v2));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unchanged "); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
            let v25: u64 = v4.len() as u64;
            let v26: u64 = v4.len() as u64;
            let v27: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0).saturating_sub(v5);
            let v28: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_before="); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v28));
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes_after="); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let v33: Rc<str> = std::rc::Rc::<str>::from((v26).to_string());
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" elapsed_ms="); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let v37: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
            let v39: Rc<str> = agile_planning_fields::kit_agile_planning_summary(&*v4, &*v1);
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v24, v39)), v38));
            let v42: i32 = std::io::Write::write_all(&mut std::io::stdout(), (v41.to_string() + "
").as_bytes()).map(|_| 0i32).unwrap_or(2i32);
            v42
        }
    }
}
fn value_token_8(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let (mut v2, mut v3): (i32, Rc<str>) = agile_planning_values::kit_agile_planning_value_token(&*v0, &*v1);
    let v4: bool = v2 == 0i32;
    if v4 {
        US1::US1_StepText(v3.clone())
    } else {
        US1::US1_StepRejected(v3)
    }
}
fn planning_edit_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> i32 {
    let v4: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/state/agile.spi"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v5, &*v6, &*v6, &*v6].concat());
    let v8: bool = std::path::Path::new(&*v7).is_file();
    let v11: US3 = if v8 {
        US3::US3_FileReadable(v7.clone())
    } else {
        US3::US3_FileMissing(v7.clone())
    };
    match &v11 {
        US3::US3_FileMissing(v12) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state/agile.spi is missing"); } LIT.with(|lit| lit.clone()) };
            planning_edit_failure_2(v13.clone(), v1.clone())
        }
        US3::US3_FileReadable(v15) => {
            let v15: Rc<str> = v15.clone();
            let v16: US1 = value_token_8(v2.clone(), v3.clone());
            match &v16 {
                US1::US1_StepRejected(v17) => {
                    let v17: Rc<str> = v17.clone();
                    planning_edit_failure_2(v17.clone(), v1.clone())
                }
                US1::US1_StepText(v19) => {
                    let v19: Rc<str> = v19.clone();
                    let v20: Rc<str> = std::fs::read_to_string(&*v15).unwrap_or_default().into();
                    let v21: Rc<str> = kit_agile_records::kit_agile_records_as_lines(&*v20);
                    planning_revised_3(v7.clone(), v1.clone(), v2.clone(), v19.clone(), v21.clone(), v4)
                }
            }
        }
    }
}
fn planning_edit_entry_0() -> i32 {
    let v0: i32 = 1i32;
    let v1: Rc<str> = usize::try_from(v0).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v2: i32 = 2i32;
    let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v4: i32 = 3i32;
    let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    let v6: i32 = 4i32;
    let v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
    planning_edit_1(v1.clone(), v3.clone(), v5.clone(), v7.clone())
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        planning_edit_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_planning_edit_run() -> i32 {
    closure0()()
}
