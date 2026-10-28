#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_ClaudeSession(u64),
    US0_KitJob(Rc<str>),
    US0_CellsVm(u64),
    US0_Orphan,
    US0_NotATool,
}
#[derive(Clone)]
enum US1 {
    US1_Talking,
    US1_Buffered(f64),
    US1_QuietInLiveBlock,
    US1_WaitingByDesign(Rc<str>),
    US1_CellsBusy(f64),
    US1_Silent(u64),
}
fn option_value_2(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v1.wrapping_add(1i32);
        let v3: i32 = std::env::args().count() as i32;
        let v4: bool = v2 >= v3;
        if v4 {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
            let v7: bool = &*v6 == &*v0;
            if v7 {
                let v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
                return v8.clone();
            } else {
                (v0, v1) = (v0.clone(), v2);
                continue;
            }
        }
    }
}
fn process_rows_sampled_3(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn owner_walk_7(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US0 {
    loop {
        let v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US0::US0_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = &*v7 == "claude";
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US0::US0_ClaudeSession(v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US0::US0_KitJob(v9.clone());
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
                            return US0::US0_CellsVm(v22);
                        } else {
                            return US0::US0_Orphan;
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
fn owner_of_6(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet|rustc|cargo|erl|beam.smp|pwsh|powershell|node|bun|gcc|cc1|cc1plus|ld|zig|lean|java|swiftc|swift-frontend|python|kotlinc|gleam|wasmtime|link|clang|mix|escript|make|git"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = v0[v1 as usize].2.clone();
    let v4: bool = v2.split("|").any(|item| item == &*v3);
    if v4 {
        let v5: i32 = 0i32;
        owner_walk_7(v0.clone(), v1, v5)
    } else {
        US0::US0_NotATool
    }
}
fn busy_at_5(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> (u64, Rc<str>, f64, u64, u64, US0) {
    let v2: u64 = v0[v1 as usize].0;
    let v3: Rc<str> = v0[v1 as usize].2.clone();
    let v4: f64 = v0[v1 as usize].4;
    let v5: u64 = v0[v1 as usize].5;
    let v6: u64 = v0[v1 as usize].6;
    let v7: US0 = owner_of_6(v0, v1);
    (v2, v3, v4, v5, v6, v7)
}
fn busy_lines_4(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: Rc<str>, mut v3: f64) -> (Rc<str>, f64) {
    loop {
        let v4: i32 = v0.len() as i32;
        let v5: bool = v1 >= v4;
        if v5 {
            return (v2.clone(), v3);
        } else {
            let v6: f64 = v0[v1 as usize].4;
            let v7: bool = v6 < 0.1f64;
            if v7 {
                let v8: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v8, v2.clone(), v3);
                continue;
            } else {
                let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (u64, Rc<str>, f64, u64, u64, US0) = busy_at_5(v0.clone(), v1);
                let v17: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v6));
                let v20: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let v21: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let v42: Rc<str> = match &v16 {
                    US0::US0_CellsVm(v31) => {
                        let v31: u64 = *v31;
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<str> = std::rc::Rc::<str>::from((v31).to_string());
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v35: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v33, &*v34, &*v34, &*v34].concat());
                        v35.clone()
                    }
                    US0::US0_ClaudeSession(v22) => {
                        let v22: u64 = *v22;
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = std::rc::Rc::<str>::from([&*v23, &*v24, &*v25, &*v25, &*v25].concat());
                        v26.clone()
                    }
                    US0::US0_KitJob(v27) => {
                        let v27: Rc<str> = v27.clone();
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v27, &*v29, &*v29, &*v29].concat());
                        v30.clone()
                    }
                    US0::US0_NotATool => {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                    US0::US0_Orphan => {
                        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v36.clone()
                    }
                };
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v18, &*v42, &*v43, &*v43].concat());
                let v45: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v18, &*v20, &*v18, &*v44].concat());
                let v46: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v12, &*v18, &*v45].concat());
                let v47: i32 = v1.wrapping_add(1i32);
                let v48: bool = &*v2 == "";
                let v51: Rc<str> = if v48 {
                    v46.clone()
                } else {
                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v50: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v49, &*v46, &*v43, &*v43].concat());
                    v50.clone()
                };
                let v52: f64 = v3 + v6;
                (v0, v1, v2, v3) = (v0.clone(), v47, v51.clone(), v52);
                continue;
            }
        }
    }
}
fn rogue_9(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: f64, mut v7: u64, mut v8: u64, mut v9: US0) -> bool {
    let v10: bool = v7 >= 180u64;
    let v12: bool = if v10 {
        let v11: bool = (v8 as f64) >= 0.8 * (v7 as f64);
        v11
    } else {
        false
    };
    let v13: bool = match &v9 {
        US0::US0_NotATool => {
            false
        }
        _ => {
            true
        }
    };
    let v15: bool = if v13 {
        let v14: bool = v6 >= v2;
        v14
    } else {
        false
    };
    if v15 {
        let v16: bool = match &v9 {
            US0::US0_Orphan => {
                true
            }
            _ => {
                false
            }
        };
        let v18: bool = if v16 {
            let v17: bool = v7 >= v1;
            v17
        } else {
            false
        };
        let v19: bool = v18 || v12;
        v19
    } else {
        false
    }
}
fn rogue_lines_8(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let v7: i32 = v4.len() as i32;
        let v8: bool = v5 >= v7;
        if v8 {
            return v6.clone();
        } else {
            let (mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (u64, Rc<str>, f64, u64, u64, US0) = busy_at_5(v4.clone(), v5);
            let v15: bool = rogue_9(v0, v1, v2, v3, v9, v10.clone(), v11, v12, v13, v14.clone());
            if v15 {
                let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let v17: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v11));
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores, age "); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, owner "); } LIT.with(|lit| lit.clone()) };
                let v43: Rc<str> = match &v14 {
                    US0::US0_CellsVm(v32) => {
                        let v32: u64 = *v32;
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let v34: Rc<str> = std::rc::Rc::<str>::from((v32).to_string());
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v36: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v35, &*v35].concat());
                        v36.clone()
                    }
                    US0::US0_ClaudeSession(v23) => {
                        let v23: u64 = *v23;
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let v25: Rc<str> = std::rc::Rc::<str>::from((v23).to_string());
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v26, &*v26].concat());
                        v27.clone()
                    }
                    US0::US0_KitJob(v28) => {
                        let v28: Rc<str> = v28.clone();
                        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v28, &*v30, &*v30, &*v30].concat());
                        v31.clone()
                    }
                    US0::US0_NotATool => {
                        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v38.clone()
                    }
                    US0::US0_Orphan => {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v37.clone()
                    }
                };
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v43, &*v44, &*v44, &*v44].concat());
                let v46: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v21, &*v45].concat());
                let v47: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v17, &*v18, &*v10, &*v46].concat());
                let v48: i32 = v5.wrapping_add(1i32);
                let v49: bool = &*v6 == "";
                let v52: Rc<str> = if v49 {
                    v47.clone()
                } else {
                    let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v51: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v50, &*v47, &*v44, &*v44].concat());
                    v51.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v48, v52.clone());
                continue;
            } else {
                let v54: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v54, v6.clone());
                continue;
            }
        }
    }
}
fn hog_lines_10(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let v7: i32 = v4.len() as i32;
        let v8: bool = v5 >= v7;
        if v8 {
            return v6.clone();
        } else {
            let v9: u64 = v4[v5 as usize].7;
            let v10: bool = v9 >= v3;
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let v12: u64 = v4[v5 as usize].0;
                let v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = v4[v5 as usize].2.clone();
                let v16: u64 = v4[v5 as usize].7;
                let v17: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB, owner "); } LIT.with(|lit| lit.clone()) };
                let v19: US0 = owner_of_6(v4.clone(), v5);
                let v40: Rc<str> = match &v19 {
                    US0::US0_CellsVm(v29) => {
                        let v29: u64 = *v29;
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let v31: Rc<str> = std::rc::Rc::<str>::from((v29).to_string());
                        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v33: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v32, &*v32].concat());
                        v33.clone()
                    }
                    US0::US0_ClaudeSession(v20) => {
                        let v20: u64 = *v20;
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let v22: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v23, &*v23, &*v23].concat());
                        v24.clone()
                    }
                    US0::US0_KitJob(v25) => {
                        let v25: Rc<str> = v25.clone();
                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v25, &*v27, &*v27, &*v27].concat());
                        v28.clone()
                    }
                    US0::US0_NotATool => {
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v35.clone()
                    }
                    US0::US0_Orphan => {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v34.clone()
                    }
                };
                let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v42: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v17, &*v18, &*v40, &*v41].concat());
                let v43: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v13, &*v14, &*v15, &*v42].concat());
                let v44: i32 = v5.wrapping_add(1i32);
                let v45: bool = &*v6 == "";
                let v48: Rc<str> = if v45 {
                    v43.clone()
                } else {
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v46, &*v43, &*v41, &*v41].concat());
                    v47.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v44, v48.clone());
                continue;
            } else {
                let v50: i32 = v5.wrapping_add(1i32);
                (v0, v1, v2, v3, v4, v5, v6) = (v0, v1, v2, v3, v4.clone(), v50, v6.clone());
                continue;
            }
        }
    }
}
fn notebook_lines_11(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = v0.len() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0[v1 as usize].3.clone();
            let v6: u64 = v0[v1 as usize].5;
            let v7: Rc<str> = kit_notebook_progress::kit_notebook_batch_line(&*v5, v6);
            let v8: bool = &*v7 == "";
            let v16: Rc<str> = if v8 {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v9.clone()
            } else {
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                let v11: u64 = v0[v1 as usize].0;
                let v12: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v15: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v12, &*v13, &*v7, &*v14].concat());
                v15.clone()
            };
            let v17: i32 = v1.wrapping_add(1i32);
            let v18: bool = &*v16 == "";
            let v24: Rc<str> = if v18 {
                v2.clone()
            } else {
                let v19: bool = &*v2 == "";
                if v19 {
                    v16.clone()
                } else {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v22: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v20, &*v16, &*v21, &*v21].concat());
                    v22.clone()
                }
            };
            (v0, v1, v2) = (v0.clone(), v17, v24.clone());
            continue;
        }
    }
}
fn windows_root_13() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn live_block_held_14(mut v0: Rc<str>) -> bool {
    let v1: bool = (&*v0).lines().any(|row| { let mut fields = row.split(char::from(9u8)); fields.nth(1) == Some("live-gui") && fields.next() == Some("held") });
    v1
}
fn jobs_root_15() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn cells_cores_16(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: f64) -> f64 {
    loop {
        let v3: i32 = v0.len() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: f64 = v0[v1 as usize].4;
            let v6: bool = v5 >= 0.1f64;
            let v31: bool = if v6 {
                let v7: US0 = owner_of_6(v0.clone(), v1);
                let v28: Rc<str> = match &v7 {
                    US0::US0_CellsVm(v17) => {
                        let v17: u64 = *v17;
                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let v19: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v21: Rc<str> = std::rc::Rc::<str>::from([&*v18, &*v19, &*v20, &*v20, &*v20].concat());
                        v21.clone()
                    }
                    US0::US0_ClaudeSession(v8) => {
                        let v8: u64 = *v8;
                        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let v10: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
                        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v12: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v11, &*v11].concat());
                        v12.clone()
                    }
                    US0::US0_KitJob(v13) => {
                        let v13: Rc<str> = v13.clone();
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v13, &*v15, &*v15, &*v15].concat());
                        v16.clone()
                    }
                    US0::US0_NotATool => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                    US0::US0_Orphan => {
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v22.clone()
                    }
                };
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                let v30: bool = v28.starts_with(&*v29);
                v30
            } else {
                false
            };
            let v32: i32 = v1.wrapping_add(1i32);
            let v35: f64 = if v31 {
                let v33: f64 = v0[v1 as usize].4;
                let v34: f64 = v2 + v33;
                v34
            } else {
                v2
            };
            (v0, v1, v2) = (v0.clone(), v32, v35);
            continue;
        }
    }
}
fn joined_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn job_directory_20(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_15();
    joined_19(v1, v0)
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_20(v0);
    joined_19(v2, v1)
}
fn file_bytes_21(mut v0: Rc<str>) -> u64 {
    let v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn owner_cores_22(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: i32, mut v3: f64) -> f64 {
    loop {
        let v4: i32 = v0.len() as i32;
        let v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let v6: f64 = v0[v2 as usize].4;
            let v7: bool = v6 >= 0.1f64;
            let v31: bool = if v7 {
                let v8: US0 = owner_of_6(v0.clone(), v2);
                let v29: Rc<str> = match &v8 {
                    US0::US0_CellsVm(v18) => {
                        let v18: u64 = *v18;
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                        let v20: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
                        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v22: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v20, &*v21, &*v21, &*v21].concat());
                        v22.clone()
                    }
                    US0::US0_ClaudeSession(v9) => {
                        let v9: u64 = *v9;
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
                        let v11: Rc<str> = std::rc::Rc::<str>::from((v9).to_string());
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v11, &*v12, &*v12, &*v12].concat());
                        v13.clone()
                    }
                    US0::US0_KitJob(v14) => {
                        let v14: Rc<str> = v14.clone();
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v14, &*v16, &*v16, &*v16].concat());
                        v17.clone()
                    }
                    US0::US0_NotATool => {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                        v24.clone()
                    }
                    US0::US0_Orphan => {
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
                        v23.clone()
                    }
                };
                let v30: bool = &*v29 == &*v1;
                v30
            } else {
                false
            };
            if v31 {
                let v32: i32 = v2.wrapping_add(1i32);
                let v33: f64 = v0[v2 as usize].4;
                let v34: f64 = v3 + v33;
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v32, v34);
                continue;
            } else {
                let v36: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v36, v3);
                continue;
            }
        }
    }
}
fn last_lines_23(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2
}
fn quiet_verdict_of_24(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: f64, mut v6: bool, mut v7: Rc<str>, mut v8: f64) -> US1 {
    let v9: bool = v4 < v0;
    if v9 {
        US1::US1_Talking
    } else {
        let v11: bool = v5 >= 0.1f64;
        if v11 {
            US1::US1_Buffered(v5)
        } else {
            if v6 {
                US1::US1_QuietInLiveBlock
            } else {
                let v14: Rc<str> = std::rc::Rc::<str>::from((&*v7).to_lowercase());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting for"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v14.contains(&*v15);
                if v16 {
                    US1::US1_WaitingByDesign(v7.clone())
                } else {
                    let v18: bool = v8 >= 0.5f64;
                    if v18 {
                        US1::US1_CellsBusy(v8)
                    } else {
                        US1::US1_Silent(v4)
                    }
                }
            }
        }
    }
}
fn text_or_empty_25(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn verdict_text_26(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_Buffered(v2) => {
            let v2: f64 = *v2;
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet output, tree busy "); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v2));
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: buffered)"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v5, &*v6, &*v6].concat());
            v7.clone()
        }
        US1::US1_CellsBusy(v14) => {
            let v14: f64 = *v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet, tree idle, detached Cells VMs use "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v14));
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: likely this job's compiler work)"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v18, &*v18].concat());
            v19.clone()
        }
        US1::US1_QuietInLiveBlock => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet during a live block: waiting for a window is expected)"); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        }
        US1::US1_Silent(v20) => {
            let v20: u64 = *v20;
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- SILENT "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree idle: check the job's own progress files"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = std::rc::Rc::<str>::from([&*v21, &*v22, &*v23, &*v24, &*v24].concat());
            v25.clone()
        }
        US1::US1_Talking => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1.clone()
        }
        US1::US1_WaitingByDesign(v9) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (waiting by design, last line: "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = std::rc::Rc::<str>::from([&*v10, &*v9, &*v11, &*v12, &*v12].concat());
            v13.clone()
        }
    }
}
fn jobs_watched_17(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool, mut v9: f64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>) -> (Rc<str>, Rc<str>) {
    let v13: i32 = (&*v5).lines().filter(|line| !line.is_empty()).count() as i32;
    let v14: bool = v6 >= v13;
    let (mut v73, mut v74): (Rc<str>, Rc<str>) = if v14 {
        (v11.clone(), v12.clone())
    } else {
        let v15: Rc<str> = std::rc::Rc::<str>::from((&*v5).lines().filter(|line| !line.is_empty()).nth(v6 as usize).unwrap_or(""));
        let v16: Rc<str> = kit_job_control::kit_job_state_word(&*v15);
        let v17: bool = &*v16 == "running";
        let (mut v71, mut v72): (Rc<str>, Rc<str>) = if v17 {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stdout.log"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = job_file_18(v15.clone(), v18.clone());
            let v20: u64 = file_bytes_21(v19.clone());
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = job_file_18(v15.clone(), v21.clone());
            let v23: u64 = file_bytes_21(v22.clone());
            let v24: u64 = v20.wrapping_add(v23);
            let v25: u64 = (&*v7).lines().find_map(|line| { let mut fields = line.split(char::from(9u8)); if fields.next() == Some(&*v15) && fields.next().and_then(|value| value.parse::<u64>().ok()) == Some(v24) { fields.next().and_then(|value| value.parse::<u64>().ok()) } else { None } }).unwrap_or(v10);
            let v26: u64 = v10.wrapping_sub(v25);
            let v27: u64 = v26.wrapping_div(60000u64);
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v15, &*v29, &*v29, &*v29].concat());
            let v31: i32 = 0i32;
            let v32: f64 = 0.0f64;
            let v33: f64 = owner_cores_22(v4.clone(), v30.clone(), v31, v32);
            let v34: Rc<str> = job_file_18(v15.clone(), v18.clone());
            let v35: u64 = 1u64;
            let v36: Rc<str> = last_lines_23(v34.clone(), v35);
            let v37: US1 = quiet_verdict_of_24(v0, v1, v2, v3, v27, v33, v8, v36.clone(), v9);
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = job_file_18(v15.clone(), v38.clone());
            let v40: Rc<str> = text_or_empty_25(v39.clone());
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output "); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes, unchanged "); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree "); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v33));
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores"); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = verdict_text_26(v37.clone());
            let v50: Rc<str> = std::rc::Rc::<str>::from([&*v48, &*v49, &*v29, &*v29, &*v29].concat());
            let v51: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v45, &*v46, &*v47, &*v50].concat());
            let v52: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v40, &*v42, &*v43, &*v51].concat());
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let v55: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
            let v56: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v53, &*v54, &*v53, &*v55].concat());
            let v57: i32 = v6.wrapping_add(1i32);
            let v58: bool = &*v11 == "";
            let v61: Rc<str> = if v58 {
                v52.clone()
            } else {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v60: Rc<str> = std::rc::Rc::<str>::from([&*v11, &*v59, &*v52, &*v29, &*v29].concat());
                v60.clone()
            };
            let v62: bool = &*v12 == "";
            let v65: Rc<str> = if v62 {
                v56.clone()
            } else {
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v63, &*v56, &*v29, &*v29].concat());
                v64.clone()
            };
            let (mut v66, mut v67): (Rc<str>, Rc<str>) = jobs_watched_17(v0, v1, v2, v3, v4.clone(), v5.clone(), v57, v7.clone(), v8, v9, v10, v61.clone(), v65.clone());
            (v66.clone(), v67.clone())
        } else {
            let v68: i32 = v6.wrapping_add(1i32);
            let (mut v69, mut v70): (Rc<str>, Rc<str>) = jobs_watched_17(v0, v1, v2, v3, v4.clone(), v5.clone(), v68, v7.clone(), v8, v9, v10, v11.clone(), v12.clone());
            (v69.clone(), v70.clone())
        };
        (v71.clone(), v72.clone())
    };
    (v73.clone(), v74.clone())
}
fn gate_summary_27() -> Rc<str> {
    let v0: Rc<str> = kit_machine_window_watch::kit_machine_gate_summary();
    v0
}
fn windows_lines_28(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_machine_window_watch::kit_machine_windows_lines(&*v0);
    v1
}
fn jobs_and_windows_12(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>) -> Rc<str> {
    let v5: Rc<str> = windows_root_13();
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v5).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().extension().map(|extension| extension == "window").unwrap_or(false)).filter_map(|entry| std::fs::read_to_string(entry.path()).ok()).map(|text| text.lines().next().unwrap_or("").to_owned()).collect::<Vec<_>>().join("
")).unwrap_or_default().as_str());
    let v7: bool = live_block_held_14(v6.clone());
    let v8: Rc<str> = jobs_root_15();
    let v9: u64 = 60u64;
    let v10: Rc<str> = { let mut names = std::fs::read_dir(&*v8).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v9 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("kit").join("machine-watch").join("quiet.tsv").to_string_lossy().as_ref());
    let v12: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
    let v13: i32 = 0i32;
    let v14: i32 = 0i32;
    let v15: f64 = 0.0f64;
    let v16: f64 = cells_cores_16(v4.clone(), v14, v15);
    let v17: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v19, mut v20): (Rc<str>, Rc<str>) = jobs_watched_17(v0, v1, v2, v3, v4.clone(), v10.clone(), v13, v12.clone(), v7, v16, v17, v18.clone(), v18.clone());
    let v21: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("kit").join("machine-watch").join("quiet.tsv").to_string_lossy().as_ref());
    { let path = std::path::PathBuf::from(&*v21); if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); } let _ = std::fs::write(&path, (&*v20).as_bytes()); };
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== kit jobs running: "); } LIT.with(|lit| lit.clone()) };
    let v23: i32 = (&*v20).lines().filter(|line| !line.is_empty()).count() as i32;
    let v24: u64 = v23 as u64;
    let v25: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
    let v27: Rc<str> = if v7 {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (a live block holds the machine)"); } LIT.with(|lit| lit.clone()) };
        v26.clone()
    } else {
        v18.clone()
    };
    let v28: Rc<str> = std::rc::Rc::<str>::from([&*v22, &*v25, &*v27, &*v18, &*v18].concat());
    let v29: bool = &*v19 == "";
    let v32: Rc<str> = if v29 {
        v28.clone()
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v30, &*v19, &*v18, &*v18].concat());
        v31.clone()
    };
    let v33: Rc<str> = gate_summary_27();
    let v34: bool = &*v33 == "";
    let v38: Rc<str> = if v34 {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate: no gate.log"); } LIT.with(|lit| lit.clone()) };
        v35.clone()
    } else {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate (gate.log, newest 4000 lines)\n"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v33, &*v18, &*v18, &*v18].concat());
        v37.clone()
    };
    let v39: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let v40: Rc<str> = kit_machine_crowding::kit_machine_crowding(&*v6, v39);
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n== windows\n"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = windows_lines_28(v6.clone());
    let v44: Rc<str> = std::rc::Rc::<str>::from([&*v32, &*v41, &*v38, &*v42, &*v43].concat());
    let v45: bool = &*v40 == "";
    if v45 {
        v44.clone()
    } else {
        let v46: Rc<str> = std::rc::Rc::<str>::from([&*v44, &*v41, &*v40, &*v18, &*v18].concat());
        v46.clone()
    }
}
fn watch_run_1() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sample-seconds"); } LIT.with(|lit| lit.clone()) };
    let v1: i32 = 2i32;
    let v2: Rc<str> = option_value_2(v0.clone(), v1);
    let v3: u64 = v2.trim().parse::<u64>().unwrap_or(0);
    let v4: bool = v3 == 0u64;
    let v5: u64 = if v4 {
        10u64
    } else {
        v3
    };
    let v6: u64 = v5.wrapping_mul(1000u64);
    let v7: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_3(v6);
    let v8: i32 = 0i32;
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: f64 = 0.0f64;
    let (mut v11, mut v12): (Rc<str>, f64) = busy_lines_4(v7.clone(), v8, v9.clone(), v10);
    let v13: u64 = 60u64;
    let v14: u64 = 30u64;
    let v15: f64 = 0.8f64;
    let v16: u64 = 8192u64;
    let v17: i32 = 0i32;
    let v18: Rc<str> = rogue_lines_8(v13, v14, v15, v16, v7.clone(), v17, v9.clone());
    let v19: u64 = 60u64;
    let v20: u64 = 30u64;
    let v21: f64 = 0.8f64;
    let v22: u64 = 8192u64;
    let v23: i32 = 0i32;
    let v24: Rc<str> = hog_lines_10(v19, v20, v21, v22, v7.clone(), v23, v9.clone());
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine: "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v12));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores busy over "); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" s, "); } LIT.with(|lit| lit.clone()) };
    let v30: u64 = { let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::nothing().with_ram())); system.refresh_memory(); system.available_memory() / 1048576 };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB free"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = std::rc::Rc::<str>::from([&*v29, &*v31, &*v32, &*v9, &*v9].concat());
    let v34: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v26, &*v27, &*v28, &*v33].concat());
    println!("{}", &*v34);
    let v35: Rc<str> = { let mut rows = (&*v11).lines().collect::<Vec<_>>(); rows.sort_by(|left, right| { let cores = |row: &str| row.split(char::from(9u8)).nth(2).and_then(|value| value.parse::<f64>().ok()).unwrap_or(0.0); cores(right).total_cmp(&cores(left)) }); std::rc::Rc::<str>::from(rows.into_iter().take(12).map(|row| format!("  {}", row.replace(char::from(9u8), "  "))).collect::<Vec<_>>().join("
")) };
    println!("{}", &*v35);
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== rogue candidates: "); } LIT.with(|lit| lit.clone()) };
    let v37: u64 = (&*v18).lines().filter(|line| !line.is_empty()).count() as u64;
    let v38: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
    let v39: Rc<str> = std::rc::Rc::<str>::from([&*v36, &*v38, &*v9, &*v9, &*v9].concat());
    println!("{}", &*v39);
    let v40: bool = &*v18 == "";
    if v40 {
        ()
    } else {
        println!("{}", &*v18);
        ()
    };
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== memory hogs (>= "); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = std::rc::Rc::<str>::from((8192u64).to_string());
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB): "); } LIT.with(|lit| lit.clone()) };
    let v44: u64 = (&*v24).lines().filter(|line| !line.is_empty()).count() as u64;
    let v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
    let v46: Rc<str> = std::rc::Rc::<str>::from([&*v41, &*v42, &*v43, &*v45, &*v9].concat());
    println!("{}", &*v46);
    let v47: bool = &*v24 == "";
    if v47 {
        ()
    } else {
        println!("{}", &*v24);
        ()
    };
    let v48: i32 = 0i32;
    let v49: Rc<str> = notebook_lines_11(v7.clone(), v48, v9.clone());
    let v50: bool = &*v49 == "";
    if v50 {
        ()
    } else {
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== notebook batches:\n"); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = std::rc::Rc::<str>::from([&*v51, &*v49, &*v9, &*v9, &*v9].concat());
        println!("{}", &*v52);
        ()
    };
    let v53: u64 = 60u64;
    let v54: u64 = 30u64;
    let v55: f64 = 0.8f64;
    let v56: u64 = 8192u64;
    let v57: Rc<str> = jobs_and_windows_12(v53, v54, v55, v56, v7.clone());
    println!("{}", &*v57);
    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit machine watch ok processes="); } LIT.with(|lit| lit.clone()) };
    let v59: i32 = v7.len() as i32;
    let v60: u64 = v59 as u64;
    let v61: Rc<str> = std::rc::Rc::<str>::from((v60).to_string());
    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" rogues="); } LIT.with(|lit| lit.clone()) };
    let v63: u64 = (&*v18).lines().filter(|line| !line.is_empty()).count() as u64;
    let v64: Rc<str> = std::rc::Rc::<str>::from((v63).to_string());
    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hogs="); } LIT.with(|lit| lit.clone()) };
    let v66: u64 = (&*v24).lines().filter(|line| !line.is_empty()).count() as u64;
    let v67: Rc<str> = std::rc::Rc::<str>::from((v66).to_string());
    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unix="); } LIT.with(|lit| lit.clone()) };
    let v69: Rc<str> = std::rc::Rc::<str>::from(format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0)));
    let v70: Rc<str> = std::rc::Rc::<str>::from([&*v65, &*v67, &*v68, &*v69, &*v9].concat());
    let v71: Rc<str> = std::rc::Rc::<str>::from([&*v58, &*v61, &*v62, &*v64, &*v70].concat());
    println!("{}", &*v71);
    0i32
}
fn machine_run_0() -> i32 {
    let v0: i32 = 2i32;
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let v2: bool = &*v1 == "watch";
    if v2 {
        watch_run_1()
    } else {
        eprintln!("kit error: machine: usage: machine watch [--sample-seconds n]");
        2i32
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        machine_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_machine_watch_run() -> i32 {
    closure0()()
}
