#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_ClaudeSession(u64),
    US1_KitJob(Rc<str>),
    US1_CellsVm(u64),
    US1_JobLeftover(Rc<str>),
    US1_Orphan,
    US1_NotATool,
}
#[derive(Clone)]
enum US2 {
    US2_Talking,
    US2_Buffered(f64),
    US2_QuietInLiveBlock,
    US2_WaitingByDesign(Rc<str>),
    US2_CellsBusy(f64),
    US2_Silent(u64),
}
fn option_value_2(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v1.wrapping_add(1i32);
        let mut v3: i32 = std::env::args().skip(0i32 as usize).count() as i32;
        let mut v4: bool = v2 >= v3;
        if v4 {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v5.clone();
        } else {
            let v6: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
            let v7: bool = &*v6 == &*v0;
            if v7 {
                let v8: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v2 as usize).unwrap_or_default());
                return v8.clone();
            } else {
                v1 = v2;
                continue;
            }
        }
    }
}
fn method4(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_3(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method4(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US0> = closure1();
    let mut v4: Option<US0> = v2.map(|x| v3(x));
    let v6: US0 = US0::US0_None;
    let v7: US0 = v6.clone();
    let v8: US0 = v4.unwrap_or(v7);
    match &v8 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn process_rows_sampled_5(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn job_left_by_11(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = kit_machine_sample::kit_machine_job_left(v0, v1);
    v2
}
fn left_or_orphan_10(mut v0: u64, mut v1: u64) -> US1 {
    let mut v2: Rc<str> = job_left_by_11(v0, v1);
    let mut v3: bool = &*v2 == "";
    if v3 {
        US1::US1_Orphan
    } else {
        US1::US1_JobLeftover(v2)
    }
}
fn cells_vm_at_12(mut v0: Rc<str>) -> bool {
    let mut v1: bool = kit_machine_sample::kit_machine_cells_vm(&*v0);
    v1
}
fn owner_walk_9(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US1 {
    loop {
        let mut v3: bool = v1 < 0i32;
        let mut v5: bool = if v3 {
            true
        } else {
            let mut v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US1::US1_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = &*v7 == "claude";
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US1::US1_ClaudeSession(v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US1::US1_KitJob(v9.clone());
                } else {
                    let v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            cells_vm_at_12(v19.clone())
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US1::US1_CellsVm(v22);
                        } else {
                            let v24: u64 = v0[v1 as usize].1;
                            let v25: u64 = v0[v1 as usize].5;
                            return left_or_orphan_10(v24, v25);
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
fn owner_of_8(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> US1 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet|rustc|cargo|erl|beam.smp|pwsh|powershell|node|bun|gcc|cc1|cc1plus|ld|zig|lean|java|swiftc|swift-frontend|python|kotlinc|gleam|wasmtime|link|clang|mix|escript|make|git"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = v0[v1 as usize].2.clone();
    let mut v4: bool = v2.split("|").any(|item| item == &*v3);
    if v4 {
        let mut v5: i32 = 0i32;
        owner_walk_9(v0.clone(), v1, v5)
    } else {
        US1::US1_NotATool
    }
}
fn busy_at_7(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32) -> (u64, Rc<str>, f64, u64, u64, US1) {
    let mut v2: u64 = v0[v1 as usize].0;
    let mut v3: Rc<str> = v0[v1 as usize].2.clone();
    let mut v4: f64 = v0[v1 as usize].4;
    let mut v5: u64 = v0[v1 as usize].5;
    let v6: u64 = v0[v1 as usize].6;
    let v7: US1 = owner_of_8(v0, v1);
    (v2, v3, v4, v5, v6, v7)
}
fn owner_text_13(mut v0: US1) -> Rc<str> {
    match &v0 {
        US1::US1_CellsVm(v8) => {
            let v8: u64 = *v8;
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            v11
        }
        US1::US1_ClaudeSession(v1) => {
            let mut v1: u64 = *v1;
            let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            v4
        }
        US1::US1_JobLeftover(v12) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-by:"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
            v14
        }
        US1::US1_KitJob(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
            v7
        }
        US1::US1_NotATool => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US1::US1_Orphan => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
            v15
        }
    }
}
fn concat5_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn busy_lines_6(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: Rc<str>, mut v3: f64) -> (Rc<str>, f64) {
    loop {
        let mut v4: i32 = v0.len() as i32;
        let mut v5: bool = v1 >= v4;
        if v5 {
            return (v2.clone(), v3);
        } else {
            let v6: f64 = v0[v1 as usize].4;
            let v7: bool = v6 < 0.1f64;
            if v7 {
                let v8: i32 = v1.wrapping_add(1i32);
                v1 = v8;
                continue;
            } else {
                let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (u64, Rc<str>, f64, u64, u64, US1) = busy_at_7(v0.clone(), v1);
                let v17: Rc<str> = std::rc::Rc::<str>::from((v11).to_string());
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v6));
                let v20: Rc<str> = std::rc::Rc::<str>::from((v14).to_string());
                let v21: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
                let v22: Rc<str> = owner_text_13(v16.clone());
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v24: Rc<str> = concat5_14(v21.clone(), v18.clone(), v22.clone(), v23.clone(), v23.clone());
                let v25: Rc<str> = concat5_14(v19.clone(), v18.clone(), v20.clone(), v18.clone(), v24.clone());
                let v26: Rc<str> = concat5_14(v17.clone(), v18.clone(), v12.clone(), v18.clone(), v25.clone());
                let v27: i32 = v1.wrapping_add(1i32);
                let v28: bool = &*v2 == "";
                let v31: Rc<str> = if v28 {
                    v26.clone()
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    concat5_14(v2.clone(), v29.clone(), v26.clone(), v23.clone(), v23.clone())
                };
                let v32: f64 = v3 + v6;
                (v1, v2, v3) = (v27, v31.clone(), v32);
                continue;
            }
        }
    }
}
fn rogue_16(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: Rc<str>, mut v6: f64, mut v7: u64, mut v8: u64, mut v9: US1) -> bool {
    let v10: f64 = (v8 as f64);
    let v11: f64 = (v7 as f64);
    let v12: bool = v7 >= 180u64;
    let v15: bool = if v12 {
        let v13: f64 = 0.8f64 * v11;
        let v14: bool = v10 >= v13;
        v14
    } else {
        false
    };
    let v16: bool = match &v9 {
        US1::US1_NotATool => {
            false
        }
        _ => {
            true
        }
    };
    let v18: bool = if v16 {
        let v17: bool = v6 >= v2;
        v17
    } else {
        false
    };
    if v18 {
        let v19: bool = match &v9 {
            US1::US1_Orphan => {
                true
            }
            _ => {
                false
            }
        };
        let v21: bool = if v19 {
            let v20: bool = v7 >= v1;
            v20
        } else {
            false
        };
        let v24: bool = if v21 {
            true
        } else {
            match &v9 {
                US1::US1_JobLeftover(v22) => {
                    let v22: Rc<str> = v22.clone();
                    true
                }
                _ => {
                    false
                }
            }
        };
        let v25: bool = v24 || v15;
        v25
    } else {
        false
    }
}
fn rogue_lines_15(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let v7: i32 = v4.len() as i32;
        let v8: bool = v5 >= v7;
        if v8 {
            return v6.clone();
        } else {
            let v9: f64 = v4[v5 as usize].4;
            let v10: bool = v9 < v2;
            if v10 {
                let v11: i32 = v5.wrapping_add(1i32);
                v5 = v11;
                continue;
            } else {
                let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (u64, Rc<str>, f64, u64, u64, US1) = busy_at_7(v4.clone(), v5);
                let v19: bool = rogue_16(v0, v1, v2, v3, v13, v14.clone(), v15, v16, v17, v18.clone());
                if v19 {
                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
                    let v21: Rc<str> = std::rc::Rc::<str>::from((v13).to_string());
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v15));
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores, age "); } LIT.with(|lit| lit.clone()) };
                    let v25: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, owner "); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = owner_text_13(v18.clone());
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<str> = concat5_14(v26.clone(), v27.clone(), v28.clone(), v28.clone(), v28.clone());
                    let v30: Rc<str> = concat5_14(v22.clone(), v23.clone(), v24.clone(), v25.clone(), v29.clone());
                    let v31: Rc<str> = concat5_14(v20.clone(), v21.clone(), v22.clone(), v14.clone(), v30.clone());
                    let v32: i32 = v5.wrapping_add(1i32);
                    let v33: bool = &*v6 == "";
                    let v36: Rc<str> = if v33 {
                        v31.clone()
                    } else {
                        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        concat5_14(v6.clone(), v34.clone(), v31.clone(), v28.clone(), v28.clone())
                    };
                    (v5, v6) = (v32, v36.clone());
                    continue;
                } else {
                    let v38: i32 = v5.wrapping_add(1i32);
                    v5 = v38;
                    continue;
                }
            }
        }
    }
}
fn hog_lines_17(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
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
                let v19: US1 = owner_of_8(v4.clone(), v5);
                let v20: Rc<str> = owner_text_13(v19.clone());
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v22: Rc<str> = concat5_14(v14.clone(), v17.clone(), v18.clone(), v20.clone(), v21.clone());
                let v23: Rc<str> = concat5_14(v11.clone(), v13.clone(), v14.clone(), v15.clone(), v22.clone());
                let v24: i32 = v5.wrapping_add(1i32);
                let v25: bool = &*v6 == "";
                let v28: Rc<str> = if v25 {
                    v23.clone()
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    concat5_14(v6.clone(), v26.clone(), v23.clone(), v21.clone(), v21.clone())
                };
                (v5, v6) = (v24, v28.clone());
                continue;
            } else {
                let v30: i32 = v5.wrapping_add(1i32);
                v5 = v30;
                continue;
            }
        }
    }
}
fn stdout_line_18(mut v0: Rc<str>) -> () {
    println!("{}", v0);
    ()
}
fn notebook_lines_19(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: i32 = v0.len() as i32;
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let mut v5: Rc<str> = v0[v1 as usize].3.clone();
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
                concat5_14(v10.clone(), v12.clone(), v13.clone(), v7.clone(), v14.clone())
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
                    concat5_14(v2.clone(), v20.clone(), v16.clone(), v21.clone(), v21.clone())
                }
            };
            (v1, v2) = (v17, v24.clone());
            continue;
        }
    }
}
fn windows_root_21() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_WINDOWS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("windows"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("windows"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-windows")).to_string_lossy().as_ref());
    v0.clone()
}
fn live_block_held_22(mut v0: Rc<str>) -> bool {
    let mut v1: bool = (&*v0).lines().any(|row| { let mut fields = row.split(char::from(9u8)); fields.nth(1) == Some("live-gui") && fields.next() == Some("held") });
    v1
}
fn jobs_root_23() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn cells_cores_24(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: f64) -> f64 {
    loop {
        let mut v3: i32 = v0.len() as i32;
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let mut v5: f64 = v0[v1 as usize].4;
            let v6: bool = v5 < 0.1f64;
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                v1 = v7;
                continue;
            } else {
                let v9: US1 = owner_of_8(v0.clone(), v1);
                let v10: Rc<str> = owner_text_13(v9.clone());
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
                let v12: bool = v10.starts_with(&*v11);
                let v13: i32 = v1.wrapping_add(1i32);
                let v16: f64 = if v12 {
                    let v14: f64 = v0[v1 as usize].4;
                    let v15: f64 = v2 + v14;
                    v15
                } else {
                    v2
                };
                (v1, v2) = (v13, v16);
                continue;
            }
        }
    }
}
fn joined_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn job_directory_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_23();
    joined_27(v1, v0)
}
fn job_file_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_28(v0);
    joined_27(v2, v1)
}
fn file_bytes_29(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = std::fs::metadata(&*v0).map(|metadata| metadata.len()).unwrap_or(0);
    v1
}
fn owner_cores_30(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: i32, mut v3: f64) -> f64 {
    loop {
        let mut v4: i32 = v0.len() as i32;
        let mut v5: bool = v2 >= v4;
        if v5 {
            return v3;
        } else {
            let v6: f64 = v0[v2 as usize].4;
            let v7: bool = v6 >= 0.1f64;
            let v11: bool = if v7 {
                let v8: US1 = owner_of_8(v0.clone(), v2);
                let v9: Rc<str> = owner_text_13(v8.clone());
                let v10: bool = &*v9 == &*v1;
                v10
            } else {
                false
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                let v13: f64 = v0[v2 as usize].4;
                let v14: f64 = v3 + v13;
                (v2, v3) = (v12, v14);
                continue;
            } else {
                let v16: i32 = v2.wrapping_add(1i32);
                v2 = v16;
                continue;
            }
        }
    }
}
fn last_lines_31(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = { let text = std::fs::read_to_string(&*v0).unwrap_or_default(); let lines = text.lines().collect::<Vec<_>>(); let from = lines.len().saturating_sub(v1 as usize); std::rc::Rc::<str>::from(lines[from..].join("
")) };
    v2
}
fn quiet_verdict_of_32(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: u64, mut v5: f64, mut v6: bool, mut v7: Rc<str>, mut v8: f64) -> US2 {
    let v9: bool = v4 < v0;
    if v9 {
        US2::US2_Talking
    } else {
        let v11: bool = v5 >= 0.1f64;
        if v11 {
            US2::US2_Buffered(v5)
        } else {
            if v6 {
                US2::US2_QuietInLiveBlock
            } else {
                let v14: Rc<str> = std::rc::Rc::<str>::from((&*v7).to_lowercase());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waiting for"); } LIT.with(|lit| lit.clone()) };
                let v16: bool = v14.contains(&*v15);
                if v16 {
                    US2::US2_WaitingByDesign(v7)
                } else {
                    let v18: bool = v8 >= 0.5f64;
                    if v18 {
                        US2::US2_CellsBusy(v8)
                    } else {
                        US2::US2_Silent(v4)
                    }
                }
            }
        }
    }
}
fn text_or_empty_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn verdict_text_34(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_Buffered(v2) => {
            let mut v2: f64 = *v2;
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet output, tree busy "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v2));
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: buffered)"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v3, v4, v5, v6.clone(), v6)
        }
        US2::US2_CellsBusy(v14) => {
            let v14: f64 = *v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet, tree idle, detached Cells VMs use "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v14));
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores: likely this job's compiler work)"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v15, v16, v17, v18.clone(), v18)
        }
        US2::US2_QuietInLiveBlock => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (quiet during a live block: waiting for a window is expected)"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_Silent(v20) => {
            let v20: u64 = *v20;
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <- SILENT "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree idle: check the job's own progress files"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v21, v22, v23, v24.clone(), v24)
        }
        US2::US2_Talking => {
            let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_WaitingByDesign(v9) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  (waiting by design, last line: "); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v10, v9, v11, v12.clone(), v12)
        }
    }
}
fn jobs_watched_25(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v5: Rc<str>, mut v6: i32, mut v7: Rc<str>, mut v8: bool, mut v9: f64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>) -> (Rc<str>, Rc<str>) {
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
            let v19: Rc<str> = job_file_26(v15.clone(), v18.clone());
            let v20: u64 = file_bytes_29(v19.clone());
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = job_file_26(v15.clone(), v21.clone());
            let v23: u64 = file_bytes_29(v22.clone());
            let v24: u64 = v20.wrapping_add(v23);
            let v25: u64 = (&*v7).lines().find_map(|line| { let mut fields = line.split(char::from(9u8)); if fields.next() == Some(&*v15) && fields.next().and_then(|value| value.parse::<u64>().ok()) == Some(v24) { fields.next().and_then(|value| value.parse::<u64>().ok()) } else { None } }).unwrap_or(v10);
            let v26: u64 = v10.wrapping_sub(v25);
            let v27: u64 = v26.wrapping_div(60000u64);
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = concat5_14(v28.clone(), v15.clone(), v29.clone(), v29.clone(), v29.clone());
            let v31: i32 = 0i32;
            let v32: f64 = 0.0f64;
            let v33: f64 = owner_cores_30(v4.clone(), v30.clone(), v31, v32);
            let v34: Rc<str> = job_file_26(v15.clone(), v18.clone());
            let v35: u64 = 1u64;
            let v36: Rc<str> = last_lines_31(v34.clone(), v35);
            let v37: US2 = quiet_verdict_of_32(v0, v1, v2, v3, v27, v33, v8, v36.clone(), v9);
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = job_file_26(v15.clone(), v38.clone());
            let v40: Rc<str> = text_or_empty_33(v39.clone());
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output "); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes, unchanged "); } LIT.with(|lit| lit.clone()) };
            let v45: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, tree "); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v33));
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores"); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = verdict_text_34(v37.clone());
            let v50: Rc<str> = concat5_14(v48.clone(), v49.clone(), v29.clone(), v29.clone(), v29.clone());
            let v51: Rc<str> = concat5_14(v44.clone(), v45.clone(), v46.clone(), v47.clone(), v50.clone());
            let v52: Rc<str> = concat5_14(v41.clone(), v40.clone(), v42.clone(), v43.clone(), v51.clone());
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = std::rc::Rc::<str>::from((v24).to_string());
            let v55: Rc<str> = std::rc::Rc::<str>::from((v25).to_string());
            let v56: Rc<str> = concat5_14(v15.clone(), v53.clone(), v54.clone(), v53.clone(), v55.clone());
            let v57: i32 = v6.wrapping_add(1i32);
            let v58: bool = &*v11 == "";
            let v61: Rc<str> = if v58 {
                v52.clone()
            } else {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                concat5_14(v11.clone(), v59.clone(), v52.clone(), v29.clone(), v29.clone())
            };
            let v62: bool = &*v12 == "";
            let v65: Rc<str> = if v62 {
                v56.clone()
            } else {
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                concat5_14(v12.clone(), v63.clone(), v56.clone(), v29.clone(), v29.clone())
            };
            let (mut v66, mut v67): (Rc<str>, Rc<str>) = jobs_watched_25(v0, v1, v2, v3, v4.clone(), v5.clone(), v57, v7.clone(), v8, v9, v10, v61.clone(), v65.clone());
            (v66.clone(), v67.clone())
        } else {
            let v68: i32 = v6.wrapping_add(1i32);
            let (mut v69, mut v70): (Rc<str>, Rc<str>) = jobs_watched_25(v0, v1, v2, v3, v4.clone(), v5.clone(), v68, v7.clone(), v8, v9, v10, v11.clone(), v12.clone());
            (v69.clone(), v70.clone())
        };
        (v71.clone(), v72.clone())
    };
    (v73.clone(), v74.clone())
}
fn gate_summary_35() -> Rc<str> {
    let v0: Rc<str> = kit_machine_window_watch::kit_machine_gate_summary();
    v0
}
fn windows_lines_36(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_machine_window_watch::kit_machine_windows_lines(&*v0);
    v1
}
fn jobs_and_windows_20(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>) -> Rc<str> {
    let mut v5: Rc<str> = windows_root_21();
    let v6: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_dir(&*v5).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().extension().map(|extension| extension == "window").unwrap_or(false)).filter_map(|entry| std::fs::read_to_string(entry.path()).ok()).map(|text| text.lines().next().unwrap_or("").to_owned()).collect::<Vec<_>>().join("
")).unwrap_or_default().as_str());
    let v7: bool = live_block_held_22(v6.clone());
    let v8: Rc<str> = jobs_root_23();
    let v9: u64 = 60u64;
    let v10: Rc<str> = { let mut names = std::fs::read_dir(&*v8).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v9 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let v11: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("LOCALAPPDATA").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir).join("kit").join("machine-watch").join("quiet.tsv").to_string_lossy().as_ref());
    let v12: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v11).unwrap_or_default().as_str());
    let v13: i32 = 0i32;
    let v14: i32 = 0i32;
    let v15: f64 = 0.0f64;
    let v16: f64 = cells_cores_24(v4.clone(), v14, v15);
    let v17: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v19, mut v20): (Rc<str>, Rc<str>) = jobs_watched_25(v0, v1, v2, v3, v4.clone(), v10.clone(), v13, v12.clone(), v7, v16, v17, v18.clone(), v18.clone());
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
    let v28: Rc<str> = concat5_14(v22.clone(), v25.clone(), v27.clone(), v18.clone(), v18.clone());
    let v29: bool = &*v19 == "";
    let v32: Rc<str> = if v29 {
        v28.clone()
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        concat5_14(v28.clone(), v30.clone(), v19.clone(), v18.clone(), v18.clone())
    };
    let v33: Rc<str> = gate_summary_35();
    let v34: bool = &*v33 == "";
    let v38: Rc<str> = if v34 {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate: no gate.log"); } LIT.with(|lit| lit.clone()) };
        v35.clone()
    } else {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== stop gate (gate.log, newest 4000 lines)\n"); } LIT.with(|lit| lit.clone()) };
        concat5_14(v36.clone(), v33.clone(), v18.clone(), v18.clone(), v18.clone())
    };
    let v39: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    let v40: Rc<str> = kit_machine_crowding::kit_machine_crowding(&*v6, v39);
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n== windows\n"); } LIT.with(|lit| lit.clone()) };
    let v43: Rc<str> = windows_lines_36(v6.clone());
    let v44: Rc<str> = concat5_14(v32.clone(), v41.clone(), v38.clone(), v42.clone(), v43.clone());
    let v45: bool = &*v40 == "";
    if v45 {
        v44.clone()
    } else {
        concat5_14(v44.clone(), v41.clone(), v40.clone(), v18.clone(), v18.clone())
    }
}
fn watch_run_1() -> i32 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--sample-seconds"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = 2i32;
    let mut v2: Rc<str> = option_value_2(v0.clone(), v1);
    let mut v3: u64 = decimal_u64_3(v2.clone());
    let mut v4: bool = v3 == 0u64;
    let mut v5: u64 = if v4 {
        10u64
    } else {
        v3
    };
    let v6: u64 = v5.wrapping_mul(1000u64);
    let v7: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_5(v6);
    let v8: i32 = 0i32;
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v10: f64 = 0.0f64;
    let (mut v11, mut v12): (Rc<str>, f64) = busy_lines_6(v7.clone(), v8, v9.clone(), v10);
    let v13: u64 = 60u64;
    let v14: u64 = 30u64;
    let v15: f64 = 0.8f64;
    let v16: u64 = 8192u64;
    let v17: i32 = 0i32;
    let v18: Rc<str> = rogue_lines_15(v13, v14, v15, v16, v7.clone(), v17, v9.clone());
    let v19: u64 = 60u64;
    let v20: u64 = 30u64;
    let v21: f64 = 0.8f64;
    let v22: u64 = 8192u64;
    let v23: i32 = 0i32;
    let v24: Rc<str> = hog_lines_17(v19, v20, v21, v22, v7.clone(), v23, v9.clone());
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== machine: "); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v12));
    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores busy over "); } LIT.with(|lit| lit.clone()) };
    let v28: Rc<str> = std::rc::Rc::<str>::from((v5).to_string());
    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" s, "); } LIT.with(|lit| lit.clone()) };
    let v30: u64 = { let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_memory(sysinfo::MemoryRefreshKind::nothing().with_ram())); system.refresh_memory(); system.available_memory() / 1048576 };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB free"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = concat5_14(v29.clone(), v31.clone(), v32.clone(), v9.clone(), v9.clone());
    let v34: Rc<str> = concat5_14(v25.clone(), v26.clone(), v27.clone(), v28.clone(), v33.clone());
    stdout_line_18(v34.clone());
    let v35: Rc<str> = { let mut rows = (&*v11).lines().collect::<Vec<_>>(); rows.sort_by(|left, right| { let cores = |row: &str| row.split(char::from(9u8)).nth(2).and_then(|value| value.parse::<f64>().ok()).unwrap_or(0.0); cores(right).total_cmp(&cores(left)) }); std::rc::Rc::<str>::from(rows.into_iter().take(12).map(|row| format!("  {}", row.replace(char::from(9u8), "  "))).collect::<Vec<_>>().join("
")) };
    stdout_line_18(v35.clone());
    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== rogue candidates: "); } LIT.with(|lit| lit.clone()) };
    let v37: u64 = (&*v18).lines().filter(|line| !line.is_empty()).count() as u64;
    let v38: Rc<str> = std::rc::Rc::<str>::from((v37).to_string());
    let v39: Rc<str> = concat5_14(v36.clone(), v38.clone(), v9.clone(), v9.clone(), v9.clone());
    stdout_line_18(v39.clone());
    let v40: bool = &*v18 == "";
    if v40 {
        ()
    } else {
        stdout_line_18(v18.clone())
    };
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== memory hogs (>= "); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = std::rc::Rc::<str>::from((8192u64).to_string());
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB): "); } LIT.with(|lit| lit.clone()) };
    let v44: u64 = (&*v24).lines().filter(|line| !line.is_empty()).count() as u64;
    let v45: Rc<str> = std::rc::Rc::<str>::from((v44).to_string());
    let v46: Rc<str> = concat5_14(v41.clone(), v42.clone(), v43.clone(), v45.clone(), v9.clone());
    stdout_line_18(v46.clone());
    let v47: bool = &*v24 == "";
    if v47 {
        ()
    } else {
        stdout_line_18(v24.clone())
    };
    let v48: i32 = 0i32;
    let v49: Rc<str> = notebook_lines_19(v7.clone(), v48, v9.clone());
    let v50: bool = &*v49 == "";
    if v50 {
        ()
    } else {
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("== notebook batches:\n"); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = concat5_14(v51.clone(), v49.clone(), v9.clone(), v9.clone(), v9.clone());
        stdout_line_18(v52.clone())
    };
    let v53: u64 = 60u64;
    let v54: u64 = 30u64;
    let v55: f64 = 0.8f64;
    let v56: u64 = 8192u64;
    let v57: Rc<str> = jobs_and_windows_20(v53, v54, v55, v56, v7.clone());
    stdout_line_18(v57.clone());
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
    let v70: Rc<str> = concat5_14(v65.clone(), v67.clone(), v68.clone(), v69.clone(), v9.clone());
    let v71: Rc<str> = concat5_14(v58.clone(), v61.clone(), v62.clone(), v64.clone(), v70.clone());
    stdout_line_18(v71.clone());
    0i32
}
fn health_run_37() -> i32 {
    let v0: i32 = kit_machine_health::kit_machine_health_run();
    v0
}
fn machine_run_0() -> i32 {
    let v0: i32 = 2i32;
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
    let mut v2: bool = &*v1 == "watch";
    if v2 {
        watch_run_1()
    } else {
        let mut v4: i32 = 2i32;
        let mut v5: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v4 as usize).unwrap_or_default());
        let v6: bool = &*v5 == "health";
        if v6 {
            health_run_37()
        } else {
            eprintln!("kit error: machine: usage: machine watch [--sample-seconds n] | machine health [--all] [--every s [--times n]]");
            2i32
        }
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
