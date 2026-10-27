#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_AnyProcess,
    US1_MineOnly(std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, u64, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Allowed(u64),
    US2_OwnProcess,
    US2_OwnAncestor,
    US2_NotAProcessId,
    US2_NotYours(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_YourJob(Rc<str>),
    US4_TheirJob(Rc<str>, Rc<str>),
    US4_NoOwningJob,
}
#[derive(Clone)]
enum US5 {
    US5_ClaudeSession(u64),
    US5_KitJob(Rc<str>),
    US5_CellsVm(u64),
    US5_Orphan,
    US5_NotATool,
}
#[derive(Clone)]
enum US6 {
    US6_EveryProcess,
    US6_SessionTree(u64),
    US6_NoSession,
}
#[derive(Clone)]
enum US7 {
    US7_NoSuchProcess,
    US7_StoppedOf(u64, u64),
}
#[derive(Clone)]
enum US8 {
    US8_Succeeded,
    US8_Failed,
}
fn now_ms_0() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_3() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_2() -> i32 {
    let v0: i32 = command_line_argument_count_3();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_6(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_3();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_6(v1);
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
        }
    }
}
fn flag_present_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: Rc<str> = argument_5(v1);
            let v5: bool = &*v4 == &*v0;
            if v5 {
                return true;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        } else {
            return false;
        }
    }
}
fn process_rows_sampled_7(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn value_after_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    loop {
        let mut v3: i32 = v1.wrapping_add(1i32);
        let mut v4: bool = v3 < v2;
        if v4 {
            let v5: Rc<str> = argument_5(v1);
            let v6: bool = &*v5 == &*v0;
            if v6 {
                return argument_5(v3);
            } else {
                (v0, v1, v2) = (v0.clone(), v3, v2);
                continue;
            }
        } else {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v10.clone();
        }
    }
}
fn ancestor_pids_10() -> Rc<str> {
    let v0: Rc<str> = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let mut chain: Vec<u32> = Vec::new(); let mut current = sysinfo::Pid::from_u32(std::process::id()); while let Some(parent) = system.process(current).and_then(|process| process.parent()) { if chain.contains(&parent.as_u32()) { break; } chain.push(parent.as_u32()); current = parent; } std::rc::Rc::<str>::from(chain.iter().map(|pid| pid.to_string()).collect::<Vec<_>>().join(",")) };
    v0.clone()
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_13(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method14(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn stop_refusal_of_12(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> US2 {
    let mut v3: bool = !v0.is_empty() && v0.bytes().all(|byte| byte.is_ascii_digit());
    if v3 {
        let mut v4: u64 = decimal_of_13(v0.clone());
        let v5: bool = v4 == v1;
        if v5 {
            US2::US2_OwnProcess
        } else {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v0)), v7));
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v2)), v7));
            let v12: bool = v11.contains(&*v9);
            if v12 {
                US2::US2_OwnAncestor
            } else {
                US2::US2_Allowed(v4)
            }
        }
    } else {
        US2::US2_NotAProcessId
    }
}
fn mine_verdict_16(mut v0: bool, mut v1: US4, mut v2: Rc<str>) -> US0 {
    if v0 {
        US0::US0_None
    } else {
        match &v1 {
            US4::US4_NoOwningJob => {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no kit job started it and it is outside this agent session"); } LIT.with(|lit| lit.clone()) };
                US0::US0_Some(v25.clone())
            }
            US4::US4_TheirJob(v6, v7) => {
                let v6: Rc<str> = v6.clone();
                let v7: Rc<str> = v7.clone();
                let v8: bool = &*v2 == &*v6;
                if v8 {
                    US0::US0_None
                } else {
                    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("it belongs to job "); } LIT.with(|lit| lit.clone()) };
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v6));
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" by "); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    let v14: bool = &*v7 == "";
                    let v16: Rc<str> = if v14 {
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no author"); } LIT.with(|lit| lit.clone()) };
                        v15.clone()
                    } else {
                        v7.clone()
                    };
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v16));
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; pass --theirs "); } LIT.with(|lit| lit.clone()) };
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v18)), v6));
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" to stop it anyway"); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                    US0::US0_Some(v22.clone())
                }
            }
            US4::US4_YourJob(v4) => {
                let mut v4: Rc<str> = v4.clone();
                US0::US0_None
            }
        }
    }
}
fn owner_walk_17(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US5 {
    loop {
        let mut v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US5::US5_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = &*v7 == "claude";
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US5::US5_ClaudeSession(v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US5::US5_KitJob(v9.clone());
                } else {
                    let v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            let v20: bool = { let lower = (v19).to_lowercase(); lower.contains("spiral.notebook") || lower.contains("spiral-cells") || lower.contains("cells-build") };
                            v20
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US5::US5_CellsVm(v22);
                        } else {
                            return US5::US5_Orphan;
                        }
                    } else {
                        let v26: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v0.clone(), v16, v26);
                        continue;
                    }
                }
            }
        }
    }
}
fn joined_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_21() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_21();
    joined_19(v1, v0)
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_20(v0);
    joined_19(v2, v1)
}
fn text_or_empty_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn session_root_23(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
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
                (v0, v1, v2) = (v0.clone(), v11, v12);
                continue;
            }
        }
    }
}
fn descends_from_24(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
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
                    (v0, v1, v2, v3) = (v0.clone(), v10, v2, v11);
                    continue;
                }
            }
        }
    }
}
fn job_owner_of_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> US4 {
    let mut v3: bool = &*v0 == "";
    if v3 {
        US4::US4_NoOwningJob
    } else {
        let v5: bool = &*v2 != "";
        let v7: bool = if v5 {
            let v6: bool = &*v1 == &*v2;
            v6
        } else {
            false
        };
        if v7 {
            US4::US4_YourJob(v0.clone())
        } else {
            US4::US4_TheirJob(v0, v1)
        }
    }
}
fn guard_refusal_15(mut v0: US1, mut v1: u64) -> US0 {
    match &v0 {
        US1::US1_AnyProcess => {
            US0::US0_None
        }
        US1::US1_MineOnly(v3, v4, v5, v6) => {
            let mut v3: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = v3.clone();
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
                let v11: US5 = owner_walk_17(v3.clone(), v7, v10);
                match &v11 {
                    US5::US5_KitJob(v12) => {
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
                let v19: Rc<str> = job_file_18(v15.clone(), v18.clone());
                text_or_empty_22(v19.clone())
            };
            let v22: i32 = 0i32;
            let v23: u64 = session_root_23(v3.clone(), v4, v22);
            let v24: bool = v23 == 0u64;
            let v27: US6 = if v24 {
                US6::US6_NoSession
            } else {
                US6::US6_SessionTree(v23)
            };
            let v33: bool = match &v27 {
                US6::US6_EveryProcess => {
                    true
                }
                US6::US6_NoSession => {
                    false
                }
                US6::US6_SessionTree(v28) => {
                    let v28: u64 = *v28;
                    let v29: i32 = 0i32;
                    descends_from_24(v3.clone(), v1, v28, v29)
                }
            };
            let v34: US4 = job_owner_of_25(v15.clone(), v21.clone(), v5.clone());
            mine_verdict_16(v33, v34.clone(), v6.clone())
        }
    }
}
fn process_stopped_27(mut v0: u64, mut v1: bool) -> (u64, u64) {
    let v2: (u64, u64) = { let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let root = sysinfo::Pid::from_u32(v0 as u32); match system.process(root) { None => (0u64, 0u64), Some(_) => { let mut order = vec![root]; if v1 { let mut index = 0; while index < order.len() { let parent = order[index]; for (pid, process) in system.processes() { if process.parent() == Some(parent) && !order.contains(pid) { order.push(*pid); } } index += 1; } } for pid in order.iter().rev() { #[cfg(windows)] { unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn TerminateProcess(process: *mut std::ffi::c_void, code: u32) -> i32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(1, 0, pid.as_u32()); if !handle.is_null() { TerminateProcess(handle, 1); CloseHandle(handle); } } } #[cfg(not(windows))] { if let Some(process) = system.process(*pid) { process.kill(); } } } std::thread::sleep(std::time::Duration::from_millis(200)); let after = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(sysinfo::ProcessRefreshKind::nothing())); let survivors = order.iter().filter(|pid| { #[cfg(windows)] { let _ = &after; unsafe extern "system" { fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut std::ffi::c_void; fn WaitForSingleObject(handle: *mut std::ffi::c_void, ms: u32) -> u32; fn CloseHandle(handle: *mut std::ffi::c_void) -> i32; } unsafe { let handle = OpenProcess(0x0010_0000, 0, pid.as_u32()); if handle.is_null() { false } else { let running = WaitForSingleObject(handle, 2000) == 258; CloseHandle(handle); running } } } #[cfg(not(windows))] { after.process(**pid).is_some() } }).count(); ((order.len() - survivors) as u64, order.len() as u64) } } };
    v2.clone()
}
fn stop_once_26(mut v0: Rc<str>, mut v1: u64, mut v2: bool) -> bool {
    let mut v3: (u64, u64) = process_stopped_27(v1, v2);
    let mut v4: u64 = v3.clone().0;
    let v5: u64 = v3.1;
    let v6: bool = v5 == 0u64;
    let v9: US7 = if v6 {
        US7::US7_NoSuchProcess
    } else {
        US7::US7_StoppedOf(v4, v5)
    };
    let v45: Rc<str> = match &v9 {
        US7::US7_NoSuchProcess => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v0));
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  not found"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
            v13.clone()
        }
        US7::US7_StoppedOf(v14, v15) => {
            let v14: u64 = *v14;
            let v15: u64 = *v15;
            let v16: bool = v14 != v15;
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v0));
                let v19: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                let v24: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes"); } LIT.with(|lit| lit.clone()) };
                let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v18, Rc::<str>::from(format!("{}{}", v25, v26))));
                v28.clone()
            } else {
                let v29: bool = v15 == 1u64;
                if v29 {
                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v0));
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    v33.clone()
                } else {
                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v0));
                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree: "); } LIT.with(|lit| lit.clone()) };
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                    let v38: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes)"); } LIT.with(|lit| lit.clone()) };
                    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
                    v41.clone()
                }
            }
        }
    };
    println!("{}", &*v45);
    match &v9 {
        US7::US7_NoSuchProcess => {
            false
        }
        US7::US7_StoppedOf(v46, v47) => {
            let v46: u64 = *v46;
            let v47: u64 = *v47;
            let v48: bool = v46 == v47;
            v48
        }
    }
}
fn stops_from_11(mut v0: i32, mut v1: i32, mut v2: bool, mut v3: US1, mut v4: u64, mut v5: Rc<str>, mut v6: u64, mut v7: u64) -> (u64, u64) {
    loop {
        let v8: bool = v0 < v1;
        if v8 {
            let v9: Rc<str> = argument_5(v0);
            let v10: bool = &*v9 == "--tree";
            let v12: bool = if v10 {
                true
            } else {
                let v11: bool = &*v9 == "--mine";
                v11
            };
            if v12 {
                let v13: i32 = v0.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6, v7) = (v13, v1, v2, v3.clone(), v4, v5.clone(), v6, v7);
                continue;
            } else {
                let v16: bool = &*v9 == "--theirs";
                if v16 {
                    let v17: i32 = v0.wrapping_add(2i32);
                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v17, v1, v2, v3.clone(), v4, v5.clone(), v6, v7);
                    continue;
                } else {
                    let v20: US2 = stop_refusal_of_12(v9.clone(), v4, v5.clone());
                    match &v20 {
                        US2::US2_Allowed(v21) => {
                            let v21: u64 = *v21;
                            let v22: US0 = guard_refusal_15(v3.clone(), v21);
                            match &v22 {
                                US0::US0_None => {
                                    let v33: bool = stop_once_26(v9.clone(), v21, v2);
                                    if v33 {
                                        let v34: i32 = v0.wrapping_add(1i32);
                                        let v35: u64 = v6.wrapping_add(1u64);
                                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v34, v1, v2, v3.clone(), v4, v5.clone(), v35, v7);
                                        continue;
                                    } else {
                                        let v38: i32 = v0.wrapping_add(1i32);
                                        let v39: u64 = v7.wrapping_add(1u64);
                                        (v0, v1, v2, v3, v4, v5, v6, v7) = (v38, v1, v2, v3.clone(), v4, v5.clone(), v6, v39);
                                        continue;
                                    }
                                }
                                US0::US0_Some(v23) => {
                                    let v23: Rc<str> = v23.clone();
                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v9));
                                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: "); } LIT.with(|lit| lit.clone()) };
                                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v25, v26)), v23));
                                    println!("{}", &*v28);
                                    let v29: i32 = v0.wrapping_add(1i32);
                                    let v30: u64 = v7.wrapping_add(1u64);
                                    (v0, v1, v2, v3, v4, v5, v6, v7) = (v29, v1, v2, v3.clone(), v4, v5.clone(), v6, v30);
                                    continue;
                                }
                            }
                        }
                        _ => {
                            let v69: Rc<str> = match &v20 {
                                US2::US2_NotAProcessId => {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v9));
                                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: not a process id"); } LIT.with(|lit| lit.clone()) };
                                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v58));
                                    v59.clone()
                                }
                                US2::US2_NotYours(v60) => {
                                    let v60: Rc<str> = v60.clone();
                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v9));
                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: "); } LIT.with(|lit| lit.clone()) };
                                    let v65: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v62, v63)), v60));
                                    v65.clone()
                                }
                                US2::US2_OwnAncestor => {
                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                    let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v9));
                                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is a parent of this kit (the calling shell)"); } LIT.with(|lit| lit.clone()) };
                                    let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                                    v55.clone()
                                }
                                US2::US2_OwnProcess => {
                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stop "); } LIT.with(|lit| lit.clone()) };
                                    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v9));
                                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: that is this kit process"); } LIT.with(|lit| lit.clone()) };
                                    let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
                                    v51.clone()
                                }
                                _ => unreachable!(),
                            };
                            println!("{}", &*v69);
                            let v70: i32 = v0.wrapping_add(1i32);
                            let v71: u64 = v7.wrapping_add(1u64);
                            (v0, v1, v2, v3, v4, v5, v6, v7) = (v70, v1, v2, v3.clone(), v4, v5.clone(), v6, v71);
                            continue;
                        }
                    }
                }
            }
        } else {
            return (v6, v7);
        }
    }
}
fn decimal_text_28(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_30(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_32() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_31() -> Rc<str> {
    let v0: i64 = local_offset_minutes_32();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_29(mut v0: US8, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US8::US8_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US8::US8_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_30(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_31();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn stop_run_1(mut v0: u64) -> i32 {
    let v1: i32 = argument_count_2();
    let v2: bool = v1 < 3i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
        println!("{}", &*v5);
        2i32
    } else {
        let v6: u64 = std::process::id() as u64;
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mine"); } LIT.with(|lit| lit.clone()) };
        let v8: i32 = 2i32;
        let v9: bool = flag_present_4(v7.clone(), v8, v1);
        let v19: US1 = if v9 {
            let v10: u64 = 100u64;
            let v11: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_7(v10);
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = get_environment_variable_8(v12.clone());
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--theirs"); } LIT.with(|lit| lit.clone()) };
            let v15: i32 = 2i32;
            let v16: Rc<str> = value_after_9(v14.clone(), v15, v1);
            US1::US1_MineOnly(v11.clone(), v6, v13.clone(), v16.clone())
        } else {
            US1::US1_AnyProcess
        };
        let v20: i32 = 2i32;
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
        let v22: i32 = 2i32;
        let v23: bool = flag_present_4(v21.clone(), v22, v1);
        let v24: Rc<str> = ancestor_pids_10();
        let v25: u64 = 0u64;
        let v26: u64 = 0u64;
        let (mut v27, mut v28): (u64, u64) = stops_from_11(v20, v1, v23, v19.clone(), v6, v24.clone(), v25, v26);
        let v29: Rc<str> = decimal_text_28(v27);
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped "); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v29));
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed "); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
        let v34: Rc<str> = decimal_text_28(v28);
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
        let v36: bool = v28 == 0u64;
        if v36 {
            let v37: US8 = US8::US8_Succeeded;
            receipt_line_29(v37.clone(), v35.clone(), v0);
            0i32
        } else {
            let v38: US8 = US8::US8_Failed;
            receipt_line_29(v38.clone(), v35.clone(), v0);
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let v0: u64 = now_ms_0();
        stop_run_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_kill_run() -> i32 {
    closure0()()
}
