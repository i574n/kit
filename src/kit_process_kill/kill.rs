#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US2 {
    US2_SwitchTree,
    US2_SwitchMine,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedTheirs,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_AnyProcess,
    US5_MineOnly(std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, u64, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Allowed(u64),
    US6_OwnProcess,
    US6_OwnAncestor,
    US6_NotAProcessId,
    US6_NotYours(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Some(u64),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_YourJob(Rc<str>),
    US8_TheirJob(Rc<str>, Rc<str>),
    US8_NoOwningJob,
}
#[derive(Clone)]
enum US9 {
    US9_ClaudeSession(u64),
    US9_KitJob(Rc<str>),
    US9_CellsVm(u64),
    US9_JobLeftover(Rc<str>),
    US9_Orphan,
    US9_NotATool,
}
#[derive(Clone)]
enum US10 {
    US10_EveryProcess,
    US10_SessionTree(u64),
    US10_NoSession,
}
#[derive(Clone)]
enum US11 {
    US11_NoSuchProcess,
    US11_StoppedOf(u64, u64),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US12 {
    US12_Succeeded,
    US12_Failed,
}
fn now_ms_0() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_4() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_3(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_4();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_5(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_3(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn kill_words_2() -> Rc<UH0> {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_3(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            v3
        }
        UH0::UH0_Nil => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn method7(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
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
fn switch_applied_8(mut v0: US2, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<UH0>) -> (bool, bool, Rc<str>, Rc<UH0>) {
    let (mut v9, mut v10, mut v11, mut v12): (bool, bool, Rc<str>, Rc<UH0>) = match &v0 {
        US2::US2_SwitchTree => {
            (true, v2, v3.clone(), v4.clone())
        }
        _ => {
            let (mut v5, mut v6, mut v7, mut v8): (bool, bool, Rc<str>, Rc<UH0>) = match &v0 {
                US2::US2_SwitchMine => {
                    (v1, true, v3, v4)
                }
                _ => unreachable!(),
            };
            (v5, v6, v7, v8)
        }
    };
    (v9, v10, v11, v12)
}
fn kill_flags_read_6(mut v0: Rc<UH0>, mut v1: bool, mut v2: bool, mut v3: Rc<str>, mut v4: Rc<UH0>) -> (bool, bool, Rc<str>, Rc<UH0>) {
    let (mut v76, mut v77, mut v78, mut v79): (bool, bool, Rc<str>, Rc<UH0>) = match &*v0 {
        UH0::UH0_Nil => {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v6: Rc<UH0> = method7(v4.clone(), v5);
            (v1, v2, v3.clone(), v6)
        }
        _ => {
            let (mut v72, mut v73, mut v74, mut v75): (bool, bool, Rc<str>, Rc<UH0>) = match &*v0 {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<UH0> = v8.clone();
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tree"); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
                    let v12: bool = &*v7 == &*v11;
                    let v22: US1 = if v12 {
                        let v13: US2 = US2::US2_SwitchTree;
                        US1::US1_Some(v13)
                    } else {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mine"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mine"); } LIT.with(|lit| lit.clone()) };
                        let v17: bool = &*v7 == &*v16;
                        if v17 {
                            let v18: US2 = US2::US2_SwitchMine;
                            US1::US1_Some(v18)
                        } else {
                            US1::US1_None
                        }
                    };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("theirs"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--theirs"); } LIT.with(|lit| lit.clone()) };
                    let v25: bool = &*v7 == &*v24;
                    let v29: US3 = if v25 {
                        let v26: US4 = US4::US4_ValuedTheirs;
                        US3::US3_Some(v26)
                    } else {
                        US3::US3_None
                    };
                    match &v22 {
                        US1::US1_Some(v30) => {
                            let v30: US2 = v30.clone();
                            let (mut v31, mut v32, mut v33, mut v34): (bool, bool, Rc<str>, Rc<UH0>) = switch_applied_8(v30, v1, v2, v3.clone(), v4.clone());
                            kill_flags_read_6(v8.clone(), v31, v32, v33, v34)
                        }
                        _ => {
                            let (mut v64, mut v65, mut v66, mut v67): (bool, bool, Rc<str>, Rc<UH0>) = match &v29 {
                                US3::US3_Some(v39) => {
                                    let v39: US4 = v39.clone();
                                    match &*v8 {
                                        UH0::UH0_Cons(v40, v41) => {
                                            let v40: Rc<str> = v40.clone();
                                            let v41: Rc<UH0> = v41.clone();
                                            kill_flags_read_6(v41, v1, v2, v40, v4.clone())
                                        }
                                        _ => {
                                            let (mut v51, mut v52, mut v53, mut v54): (bool, bool, Rc<str>, Rc<UH0>) = match &*v8 {
                                                UH0::UH0_Nil => {
                                                    let v46: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                                                    kill_flags_read_6(v46, v1, v2, v3.clone(), v4.clone())
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v51, v52, v53, v54)
                                        }
                                    }
                                }
                                _ => {
                                    let v59: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v4));
                                    let (mut v60, mut v61, mut v62, mut v63): (bool, bool, Rc<str>, Rc<UH0>) = kill_flags_read_6(v8, v1, v2, v3, v59);
                                    (v60, v61, v62, v63)
                                }
                            };
                            (v64, v65, v66, v67)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v72, v73, v74, v75)
        }
    };
    (v76, v77, v78, v79)
}
fn process_rows_sampled_9(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn get_environment_variable_10(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn ancestor_pids_11() -> Rc<str> {
    let mut v0: Rc<str> = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let mut chain: Vec<u32> = Vec::new(); let mut current = sysinfo::Pid::from_u32(std::process::id()); while let Some(parent) = system.process(current).and_then(|process| process.parent()) { if chain.contains(&parent.as_u32()) { break; } chain.push(parent.as_u32()); current = parent; } std::rc::Rc::<str>::from(chain.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(",")) };
    v0.clone()
}
fn method15(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US7> = Rc::new(move |mut v0: (u64)| -> US7 {
        let v1: u64 = (v0);
        US7::US7_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_14(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method15(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US7> = closure1();
    let mut v4: Option<US7> = v2.map(|x| v3(x));
    let v6: US7 = US7::US7_None;
    let v7: US7 = v6.clone();
    let v8: US7 = v4.unwrap_or(v7);
    match &v8 {
        US7::US7_None => {
            0u64
        }
        US7::US7_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn stop_refusal_of_13(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> US6 {
    let mut v3: bool = !v0.is_empty() && v0.bytes().all(|byte| byte.is_ascii_digit());
    if v3 {
        let mut v4: u64 = decimal_of_14(v0.clone());
        let v5: bool = v4 == v1;
        if v5 {
            US6::US6_OwnProcess
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v0)), v7));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v2)), v7));
            let v12: bool = v11.contains(&*v9);
            if v12 {
                US6::US6_OwnAncestor
            } else {
                US6::US6_Allowed(v4)
            }
        }
    } else {
        US6::US6_NotAProcessId
    }
}
fn mine_verdict_17(mut v0: bool, mut v1: US8, mut v2: Rc<str>) -> US0 {
    match &v1 {
        US8::US8_NoOwningJob => {
            if v0 {
                US0::US0_None
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no kit job started it and it is outside this agent session"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Some(v25)
            }
        }
        US8::US8_TheirJob(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: bool = &*v2 == &*v5;
            if v7 {
                US0::US0_None
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("it belongs to job "); } LIT.with(|lit| lit.clone()) };
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v5));
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" by "); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                let v13: bool = &*v6 == "";
                let v15: Rc<str> = if v13 {
                    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no author"); } LIT.with(|lit| lit.clone()) };
                    v14
                } else {
                    v6
                };
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v15));
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; pass --theirs "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v17)), v5));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" to stop it anyway"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                US0::US0_Some(v21)
            }
        }
        US8::US8_YourJob(v3) => {
            let mut v3: Rc<str> = v3.clone();
            US0::US0_None
        }
    }
}
fn job_left_by_20(mut v0: u64, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = kit_machine_sample::kit_machine_job_left(v0, v1);
    v2
}
fn left_or_orphan_19(mut v0: u64, mut v1: u64) -> US9 {
    let v2: Rc<str> = job_left_by_20(v0, v1);
    let mut v3: bool = &*v2 == "";
    if v3 {
        US9::US9_Orphan
    } else {
        US9::US9_JobLeftover(v2)
    }
}
fn cells_vm_at_21(mut v0: Rc<str>) -> bool {
    let v1: bool = kit_machine_sample::kit_machine_cells_vm(&*v0);
    v1
}
fn owner_walk_18(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US9 {
    loop {
        let mut v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US9::US9_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = &*v7 == "claude";
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US9::US9_ClaudeSession(v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US9::US9_KitJob(v9.clone());
                } else {
                    let v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            cells_vm_at_21(v19.clone())
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US9::US9_CellsVm(v22);
                        } else {
                            let v24: u64 = v0[v1 as usize].1;
                            let v25: u64 = v0[v1 as usize].5;
                            return left_or_orphan_19(v24, v25);
                        }
                    } else {
                        let v28: i32 = v2.wrapping_add(1i32);
                        (v1, v2) = (v16, v28);
                        continue;
                    }
                }
            }
        }
    }
}
fn joined_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_25() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_25();
    joined_23(v1, v0)
}
fn job_file_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_24(v0);
    joined_23(v2, v1)
}
fn text_or_empty_26(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn session_root_27(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
    loop {
        let mut v3: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
        let mut v4: bool = v3 < 0i32;
        let v6: bool = if v4 {
            true
        } else {
            let v5: bool = v2 > 32i32;
            v5
        };
        if v6 {
            return 0u64;
        } else {
            let v7: Rc<str> = v0[v3 as usize].2.clone();
            let v8: bool = &*v7 == "claude";
            let v10: bool = if v8 {
                true
            } else {
                let v9: bool = &*v7 == "codex";
                v9
            };
            if v10 {
                return v1;
            } else {
                let v11: u64 = v0[v3 as usize].1;
                let v12: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v11, v12);
                continue;
            }
        }
    }
}
fn descends_from_28(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v1 == v2;
        if v4 {
            return true;
        } else {
            let v5: bool = v1 == 0u64;
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v3 > 64i32;
                v6
            };
            if v7 {
                return false;
            } else {
                let v8: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
                let v9: bool = v8 < 0i32;
                if v9 {
                    return false;
                } else {
                    let v10: u64 = v0[v8 as usize].1;
                    let v11: i32 = v3.wrapping_add(1i32);
                    (v1, v3) = (v10, v11);
                    continue;
                }
            }
        }
    }
}
fn job_owner_of_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US8 {
    let mut v3: bool = &*v0 == "";
    if v3 {
        US8::US8_NoOwningJob
    } else {
        let v5: bool = &*v2 != "";
        let v7: bool = if v5 {
            let v6: bool = &*v1 == &*v2;
            v6
        } else {
            false
        };
        if v7 {
            US8::US8_YourJob(v0.clone())
        } else {
            US8::US8_TheirJob(v0, v1)
        }
    }
}
fn guard_refusal_16(mut v0: US5, mut v1: u64) -> US0 {
    match &v0 {
        US5::US5_AnyProcess => {
            US0::US0_None
        }
        US5::US5_MineOnly(v3, v4, v5, v6) => {
            let mut v3: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = v3.clone();
            let mut v4: u64 = *v4;
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: i32 = v3.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
            let v8: bool = v7 < 0i32;
            let v15: Rc<str> = if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let v10: i32 = 0i32;
                let v11: US9 = owner_walk_18(v3.clone(), v7, v10);
                match &v11 {
                    US9::US9_KitJob(v12) => {
                        let v12: Rc<str> = v12.clone();
                        v12.clone()
                    }
                    _ => {
                        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v13.clone()
                    }
                }
            };
            let v16: bool = &*v15 == "";
            let v21: Rc<str> = if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v17.clone()
            } else {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = job_file_22(v15.clone(), v18.clone());
                text_or_empty_26(v19.clone())
            };
            let v22: i32 = 0i32;
            let v23: u64 = session_root_27(v3.clone(), v4, v22);
            let v24: bool = v23 == 0u64;
            let v27: US10 = if v24 {
                US10::US10_NoSession
            } else {
                US10::US10_SessionTree(v23)
            };
            let v33: bool = match &v27 {
                US10::US10_EveryProcess => {
                    true
                }
                US10::US10_NoSession => {
                    false
                }
                US10::US10_SessionTree(v28) => {
                    let v28: u64 = *v28;
                    let v29: i32 = 0i32;
                    descends_from_28(v3.clone(), v1, v28, v29)
                }
            };
            let v34: US8 = job_owner_of_29(v15.clone(), v21.clone(), v5.clone());
            mine_verdict_17(v33, v34.clone(), v6.clone())
        }
    }
}
fn process_stopped_31(mut v0: u64, mut v1: bool) -> (u64, u64) {
    let v2: (u64, u64) = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let root = sysinfo::Pid::from_u32(v0 as u32); match system.process(root) { None => (0u64, 0u64), Some(_) => { let mut order = vec![root]; if v1 { let mut index = 0; while index < order.len() { let parent = order[index]; for (pid, process) in system.processes() { if process.parent() == Some(parent) && !order.contains(pid) { order.push(*pid); } } index += 1; } } for pid in order.iter().rev() { #[cfg(windows)] { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn TerminateProcess(process: *mut std::ffi::c_void, code: u32) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(1, 0, pid.as_u32()); if !handle.is_null() { TerminateProcess(handle, 1); CloseHandle(handle); } } } #[cfg(not(windows))] { if let Some(process) = system.process(*pid) { process.kill(); } } } std::thread::sleep(std::time::Duration::from_millis(200)); let after = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let survivors = order.iter().filter(|pid| { #[cfg(windows)] { let _ = &after; unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid.as_u32()); if handle.is_null() { false } else { let running = WaitForSingleObject(handle, 2000) == 258; CloseHandle(handle); running } } } #[cfg(not(windows))] { after.process(**pid).is_some() } }).count(); ((order.len() - survivors) as u64, order.len() as u64) } } };
    v2.clone()
}
fn method33(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_32(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method33(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn stop_once_30(mut v0: Rc<str>, mut v1: u64, mut v2: bool) -> bool {
    let mut v3: (u64, u64) = process_stopped_31(v1, v2);
    let mut v4: u64 = v3.clone().0;
    let v5: u64 = v3.1;
    let v6: bool = v5 == 0u64;
    let v9: US11 = if v6 {
        US11::US11_NoSuchProcess
    } else {
        US11::US11_StoppedOf(v4, v5)
    };
    let v45: Rc<str> = match &v9 {
        US11::US11_NoSuchProcess => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  not found"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            v13
        }
        US11::US11_StoppedOf(v14, v15) => {
            let v14: u64 = *v14;
            let v15: u64 = *v15;
            let v16: bool = v14 != v15;
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v0));
                let v19: Rc<str> = format_real_32(v14);
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                let v24: Rc<str> = format_real_32(v15);
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v18, Rc::<str>::from(format!("{}{}", v25, v26))));
                v28
            } else {
                let v29: bool = v15 == 1u64;
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v0));
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    v33
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v0));
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree: "); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                    let v38: Rc<str> = format_real_32(v15);
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes)"); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    v41
                }
            }
        }
    };
    println!("{}", v45);
    match &v9 {
        US11::US11_NoSuchProcess => {
            false
        }
        US11::US11_StoppedOf(v46, v47) => {
            let v46: u64 = *v46;
            let v47: u64 = *v47;
            let v48: bool = v46 == v47;
            v48
        }
    }
}
fn stops_from_12(mut v0: Rc<UH0>, mut v1: bool, mut v2: US5, mut v3: u64, mut v4: Rc<str>, mut v5: u64, mut v6: u64) -> (u64, u64) {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v7, v8) => {
                let v7: Rc<str> = v7.clone();
                let v8: Rc<UH0> = v8.clone();
                let v9: US6 = stop_refusal_of_13(v7.clone(), v3, v4.clone());
                match &v9 {
                    US6::US6_Allowed(v10) => {
                        let v10: u64 = *v10;
                        let v11: US0 = guard_refusal_16(v2.clone(), v10);
                        match &v11 {
                            US0::US0_None => {
                                let v21: bool = stop_once_30(v7.clone(), v10, v1);
                                if v21 {
                                    let v22: u64 = v5.wrapping_add(1u64);
                                    (v0, v5) = (v8.clone(), v22);
                                    continue;
                                } else {
                                    let v25: u64 = v6.wrapping_add(1u64);
                                    (v0, v6) = (v8.clone(), v25);
                                    continue;
                                }
                            }
                            US0::US0_Some(v12) => {
                                let v12: Rc<str> = v12.clone();
                                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v7));
                                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: "); } LIT.with(|lit| lit.clone()) };
                                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v14, v15)), v12));
                                println!("{}", v17);
                                let v18: u64 = v6.wrapping_add(1u64);
                                (v0, v6) = (v8.clone(), v18);
                                continue;
                            }
                        }
                    }
                    _ => {
                        let v55: Rc<str> = match &v9 {
                            US6::US6_NotAProcessId => {
                                let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v7));
                                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: not a process id"); } LIT.with(|lit| lit.clone()) };
                                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v44));
                                v45.clone()
                            }
                            US6::US6_NotYours(v46) => {
                                let v46: Rc<str> = v46.clone();
                                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v7));
                                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: "); } LIT.with(|lit| lit.clone()) };
                                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v48, v49)), v46));
                                v51.clone()
                            }
                            US6::US6_OwnAncestor => {
                                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v7));
                                let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is a parent of this kit (the calling shell)"); } LIT.with(|lit| lit.clone()) };
                                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                                v41.clone()
                            }
                            US6::US6_OwnProcess => {
                                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v7));
                                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is this kit process"); } LIT.with(|lit| lit.clone()) };
                                let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                                v37.clone()
                            }
                            _ => unreachable!(),
                        };
                        println!("{}", v55);
                        let v56: u64 = v6.wrapping_add(1u64);
                        (v0, v6) = (v8.clone(), v56);
                        continue;
                    }
                }
            }
            UH0::UH0_Nil => {
                return (v5, v6);
            }
        }
    }
}
fn decimal_text_34(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_36(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_38() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_37() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_38();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_35(mut v0: US12, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US12::US12_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US12::US12_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_36(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_37();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn stop_run_1(mut v0: u64) -> i32 {
    let v1: Rc<UH0> = kill_words_2();
    let v2: bool = false;
    let mut v3: bool = false;
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v6, mut v7, mut v8, mut v9): (bool, bool, Rc<str>, Rc<UH0>) = kill_flags_read_6(v1.clone(), v2, v3, v4, v5);
    let v10: bool = match &*v1 {
        UH0::UH0_Nil => {
            true
        }
        _ => {
            false
        }
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", v13);
        2i32
    } else {
        let v14: u64 = std::process::id() as u64;
        let v21: US5 = if v7 {
            let v15: u64 = 100u64;
            let v16: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_9(v15);
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = get_environment_variable_10(v17);
            US5::US5_MineOnly(v16, v14, v18, v8)
        } else {
            US5::US5_AnyProcess
        };
        let v22: Rc<str> = ancestor_pids_11();
        let v23: u64 = 0u64;
        let v24: u64 = 0u64;
        let (mut v25, mut v26): (u64, u64) = stops_from_12(v9, v6, v21, v14, v22, v23, v24);
        let v27: Rc<str> = decimal_text_34(v25);
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped "); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed "); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
        let v32: Rc<str> = decimal_text_34(v26);
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
        let v34: bool = v26 == 0u64;
        if v34 {
            let v35: US12 = US12::US12_Succeeded;
            receipt_line_35(v35, v33.clone(), v0);
            0i32
        } else {
            let v36: US12 = US12::US12_Failed;
            receipt_line_35(v36, v33, v0);
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: u64 = now_ms_0();
        stop_run_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_kill_run() -> i32 {
    closure0()()
}
