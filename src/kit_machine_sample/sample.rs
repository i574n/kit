#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Some(u64),
    US0_None,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<str>, u64, bool, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(Rc<str>),
    US1_None,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(u64, Rc<UH2>),
}
fn live_parent_2(mut v0: std::rc::Rc<std::cell::RefCell<sysinfo::System>>, mut v1: u32, mut v2: u64) -> u32 {
    let v3: u32 = v0.borrow().process(sysinfo::Pid::from_u32(v1)).and_then(sysinfo::Process::parent).map_or(0, sysinfo::Pid::as_u32);
    let mut v4: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v1)).map_or(0, sysinfo::Process::start_time);
    let v5: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v3)).map_or(0, sysinfo::Process::start_time);
    let mut v6: bool = v3 <= 4u32;
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = v4 == 0u64;
        v7
    };
    let v10: bool = if v8 {
        true
    } else {
        let v9: bool = v5 == 0u64;
        v9
    };
    let v12: bool = if v10 {
        true
    } else {
        let mut v11: bool = v5 <= v4;
        v11
    };
    let v14: bool = if v12 {
        true
    } else {
        let v13: bool = v5 > v2;
        v13
    };
    if v14 {
        v3
    } else {
        0u32
    }
}
fn process_private_bytes_3(mut v0: u64) -> u64 {
    let mut v1: u64 = kit_os_launch::kit_process_private_bytes(v0);
    v1
}
fn rows_from_1(mut v0: std::rc::Rc<std::cell::RefCell<sysinfo::System>>, mut v1: std::rc::Rc<Vec<u32>>, mut v2: std::rc::Rc<std::cell::RefCell<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>>, mut v3: std::rc::Rc<std::collections::HashMap<u32, u64>>, mut v4: i32, mut v5: i32, mut v6: f64) -> () {
    loop {
        let v7: bool = v4 < v5;
        if v7 {
            let v8: usize = ((v4) as usize);
            let v9: u32 = v1[v8];
            let v10: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::accumulated_cpu_time);
            let mut v11: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
            let v12: u64 = (v9 as u64);
            let v13: u32 = live_parent_2(v0.clone(), v9, v11);
            let v14: u64 = (v13 as u64);
            let v15: Rc<str> = std::rc::Rc::<str>::from(v0.borrow().process(sysinfo::Pid::from_u32(v9)).map(|process| process.name().to_string_lossy().trim_end_matches(".exe").to_owned()).unwrap_or_default());
            let v16: Rc<str> = std::rc::Rc::<str>::from(v0.borrow().process(sysinfo::Pid::from_u32(v9)).map(|process| process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" ")).unwrap_or_default());
            let v17: u64 = v3.get(&v9).copied().unwrap_or(0);
            let v18: bool = v10 > v17;
            let v20: bool = if v18 {
                let v19: bool = v6 > 0.0f64;
                v19
            } else {
                false
            };
            let v25: f64 = if v20 {
                let mut v21: u64 = v10.wrapping_sub(v17);
                let mut v22: f64 = (v21 as f64);
                let mut v23: f64 = v6 * 1000.0f64;
                let mut v24: f64 = v22 / v23;
                v24
            } else {
                0.0f64
            };
            let v26: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::run_time);
            let v27: u64 = v26.wrapping_div(60u64);
            let v28: u64 = v10.wrapping_div(60000u64);
            let mut v29: u64 = v0.borrow().process(sysinfo::Pid::from_u32(v9)).map_or(0, sysinfo::Process::memory);
            let v30: u64 = v29.wrapping_div(1048576u64);
            let v31: u64 = process_private_bytes_3(v12);
            let v32: u64 = v31.wrapping_div(1048576u64);
            v2.borrow_mut().push((v12, v14, v15, v16, v25, v27, v28, v30, v32));
            let v33: i32 = v4.wrapping_add(1i32);
            v4 = v33;
            continue;
        }
        return ();
    }
}
fn rows_sampled_0(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: sysinfo::ProcessRefreshKind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always);
    let mut v2: std::rc::Rc<std::cell::RefCell<sysinfo::System>> = std::rc::Rc::new(std::cell::RefCell::new(sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(v1.clone()))));
    let v3: std::rc::Rc<std::collections::HashMap<u32, u64>> = std::rc::Rc::new(v2.borrow().processes().iter().map(|(pid, process)| (pid.as_u32(), process.accumulated_cpu_time())).collect());
    let mut v4: std::time::Instant = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(v0));
    let v5: usize = v2.borrow_mut().refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, v1);
    let mut v6: f64 = (v4).elapsed().as_secs_f64();
    let v7: std::rc::Rc<Vec<u32>> = std::rc::Rc::new(v2.borrow().processes().keys().copied().map(sysinfo::Pid::as_u32).collect::<Vec<_>>());
    let v8: std::rc::Rc<std::cell::RefCell<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let v9: i32 = 0i32;
    let v10: usize = v7.len();
    let mut v11: i32 = ((v10) as i32);
    rows_from_1(v2.clone(), v7.clone(), v8.clone(), v3.clone(), v9, v11, v6);
    let v12: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = std::rc::Rc::new(v8.borrow().clone());
    v12.clone()
}
fn closure0() -> Rc<dyn Fn(u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>> = Rc::new(move |mut v0: u64| -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
        rows_sampled_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn cells_vm_command_4(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((&*v0).to_lowercase());
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.notebook"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = v1.contains(&*v2);
    let mut v6: bool = if v3 {
        true
    } else {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral-cells"); } LIT.with(|lit| lit.clone()) };
        let v5: bool = v1.contains(&*v4);
        v5
    };
    let v9: bool = if v6 {
        true
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-build"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = v1.contains(&*v7);
        v8
    };
    if v9 {
        true
    } else {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiral.cells.compilerdaemon"); } LIT.with(|lit| lit.clone()) };
        let mut v11: bool = v1.contains(&*v10);
        v11
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        cells_vm_command_4(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_10() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_9(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_10();
    joined_8(v1, v0)
}
fn job_file_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_9(v0);
    joined_8(v2, v1)
}
fn text_or_empty_11(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1
}
fn method12(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method13(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method14(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure3() -> Rc<dyn Fn((u64)) -> US0> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US0> = Rc::new(move |mut v0: (u64)| -> US0 {
        let mut v1: u64 = (v0);
        US0::US0_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_left_from_6(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: u64, mut v3: u64) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v6.clone();
        } else {
            let v7: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = job_file_7(v7.clone(), v8.clone());
            let v10: Rc<str> = text_or_empty_11(v9.clone());
            let mut v11: i32 = (v10.clone().len() as i32);
            let v12: i32 = 0i32;
            let v13: i32 = method12(v10.clone(), v11, v12);
            let v14: i32 = v11.wrapping_sub(1i32);
            let v15: Rc<str> = string_slice(&v10.clone(), v13 as i64, v14 as i64);
            let v16: i32 = (v15.clone().len() as i32);
            let v17: i32 = method13(v15.clone(), v16);
            let v18: Rc<str> = string_slice(&v15.clone(), 0i32 as i64, v17 as i64);
            let v19: Option<u64> = (v18).trim().parse::<u64>().ok();
            let v20: Option<u64> = method14(v19.clone());
            let mut v21: Rc<dyn Fn((u64)) -> US0> = closure3();
            let mut v22: Option<US0> = v20.map(|x| v21(x));
            let mut v24: US0 = US0::US0_None;
            let v25: US0 = v24.clone();
            let v26: US0 = v22.unwrap_or(v25);
            let mut v29: u64 = match &v26 {
                US0::US0_None => {
                    0u64
                }
                US0::US0_Some(v27) => {
                    let v27: u64 = *v27;
                    v27
                }
            };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("supervisor"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = job_file_7(v7.clone(), v30.clone());
            let v32: Rc<str> = text_or_empty_11(v31.clone());
            let v33: i32 = (v32.clone().len() as i32);
            let v34: i32 = 0i32;
            let v35: i32 = method12(v32.clone(), v33, v34);
            let v36: i32 = v33.wrapping_sub(1i32);
            let mut v37: Rc<str> = string_slice(&v32.clone(), v35 as i64, v36 as i64);
            let v38: i32 = (v37.clone().len() as i32);
            let v39: i32 = method13(v37.clone(), v38);
            let mut v40: Rc<str> = string_slice(&v37.clone(), 0i32 as i64, v39 as i64);
            let mut v41: Option<u64> = (v40).trim().parse::<u64>().ok();
            let v42: Option<u64> = method14(v41.clone());
            let mut v43: Option<US0> = v42.map(|x| v21(x));
            let v45: US0 = US0::US0_None;
            let v46: US0 = v45.clone();
            let v47: US0 = v43.unwrap_or(v46);
            let v50: u64 = match &v47 {
                US0::US0_None => {
                    0u64
                }
                US0::US0_Some(v48) => {
                    let v48: u64 = *v48;
                    v48
                }
            };
            let v51: bool = &*v7 != "";
            let v55: bool = if v51 {
                let v52: bool = v29 == v2;
                if v52 {
                    true
                } else {
                    let v53: bool = v50 == v2;
                    v53
                }
            } else {
                false
            };
            if v55 {
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = job_file_7(v7.clone(), v56.clone());
                let v58: Rc<str> = text_or_empty_11(v57.clone());
                let v59: i32 = (v58.clone().len() as i32);
                let v60: i32 = 0i32;
                let v61: i32 = method12(v58.clone(), v59, v60);
                let v62: i32 = v59.wrapping_sub(1i32);
                let v63: Rc<str> = string_slice(&v58.clone(), v61 as i64, v62 as i64);
                let v64: i32 = (v63.clone().len() as i32);
                let v65: i32 = method13(v63.clone(), v64);
                let v66: Rc<str> = string_slice(&v63.clone(), 0i32 as i64, v65 as i64);
                let mut v67: bool = &*v66 != "";
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = job_file_7(v7.clone(), v68.clone());
                let v70: Rc<str> = text_or_empty_11(v69.clone());
                let v71: i32 = (v70.clone().len() as i32);
                let v72: i32 = 0i32;
                let v73: i32 = method12(v70.clone(), v71, v72);
                let v74: i32 = v71.wrapping_sub(1i32);
                let v75: Rc<str> = string_slice(&v70.clone(), v73 as i64, v74 as i64);
                let v76: i32 = (v75.clone().len() as i32);
                let v77: i32 = method13(v75.clone(), v76);
                let v78: Rc<str> = string_slice(&v75.clone(), 0i32 as i64, v77 as i64);
                let v79: Option<u64> = (v78).trim().parse::<u64>().ok();
                let v80: Option<u64> = method14(v79.clone());
                let mut v81: Option<US0> = v80.map(|x| v21(x));
                let v83: US0 = US0::US0_None;
                let v84: US0 = v83.clone();
                let v85: US0 = v81.unwrap_or(v84);
                let v88: u64 = match &v85 {
                    US0::US0_None => {
                        0u64
                    }
                    US0::US0_Some(v86) => {
                        let v86: u64 = *v86;
                        v86
                    }
                };
                let v90: bool = if v67 {
                    let v89: bool = v2 > 4u64;
                    v89
                } else {
                    false
                };
                let v94: bool = if v90 {
                    let v91: bool = v2 == v29;
                    if v91 {
                        true
                    } else {
                        let v92: bool = v2 == v50;
                        v92
                    }
                } else {
                    false
                };
                let v97: bool = if v94 {
                    let v95: u64 = v88.wrapping_div(1000u64);
                    let v96: bool = v3 >= v95;
                    v96
                } else {
                    false
                };
                if v97 {
                    return v7.clone();
                } else {
                    let v98: i32 = v1.wrapping_add(1i32);
                    v1 = v98;
                    continue;
                }
            } else {
                let v101: i32 = v1.wrapping_add(1i32);
                v1 = v101;
                continue;
            }
        }
    }
}
fn job_left_5(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: bool = v0 <= 4u64;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3.clone()
    } else {
        let mut v4: Rc<str> = jobs_root_10();
        let v5: u64 = 100u64;
        let mut v6: Rc<str> = { let mut names = std::fs::read_dir(&*v4).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v5 as usize); std::rc::Rc::<str>::from(names.join("
")) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v6.split(&*v7).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v9: i32 = 0i32;
        let v10: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
        let mut v11: u64 = v1.wrapping_mul(60u64);
        let v12: u64 = v10.wrapping_sub(v11);
        job_left_from_6(v8.clone(), v9, v0, v12)
    }
}
fn closure2() -> Rc<dyn Fn(u64, u64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(u64, u64) -> Rc<str>> = Rc::new(move |mut v0: u64, mut v1: u64| -> Rc<str> {
        job_left_5(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn job_seen_of_17(mut v0: Rc<str>, mut v1: u64) -> (Rc<str>, Rc<str>, u64, bool) {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = job_file_7(v0.clone(), v2.clone());
    let mut v4: Rc<str> = text_or_empty_11(v3.clone());
    let v5: i32 = (v4.clone().len() as i32);
    let mut v6: i32 = 0i32;
    let v7: i32 = method12(v4.clone(), v5, v6);
    let v8: i32 = v5.wrapping_sub(1i32);
    let v9: Rc<str> = string_slice(&v4.clone(), v7 as i64, v8 as i64);
    let v10: i32 = (v9.clone().len() as i32);
    let mut v11: i32 = method13(v9.clone(), v10);
    let v12: Rc<str> = string_slice(&v9.clone(), 0i32 as i64, v11 as i64);
    let v13: i32 = (v12.clone().len() as i32);
    let v14: i32 = 0i32;
    let v15: i32 = method12(v12.clone(), v13, v14);
    let v16: i32 = v13.wrapping_sub(1i32);
    let v17: Rc<str> = string_slice(&v12.clone(), v15 as i64, v16 as i64);
    let v18: i32 = (v17.clone().len() as i32);
    let v19: i32 = method13(v17.clone(), v18);
    let v20: Rc<str> = string_slice(&v17.clone(), 0i32 as i64, v19 as i64);
    let mut v21: Option<u64> = (v20).trim().parse::<u64>().ok();
    let mut v22: Option<u64> = method14(v21.clone());
    let mut v23: Rc<dyn Fn((u64)) -> US0> = closure3();
    let mut v24: Option<US0> = v22.map(|x| v23(x));
    let v26: US0 = US0::US0_None;
    let v27: US0 = v26.clone();
    let v28: US0 = v24.unwrap_or(v27);
    let v31: u64 = match &v28 {
        US0::US0_None => {
            0u64
        }
        US0::US0_Some(v29) => {
            let mut v29: u64 = *v29;
            v29
        }
    };
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = job_file_7(v0.clone(), v32.clone());
    let v34: Rc<str> = text_or_empty_11(v33.clone());
    let v35: i32 = (v34.clone().len() as i32);
    let v36: i32 = 0i32;
    let mut v37: i32 = method12(v34.clone(), v35, v36);
    let v38: i32 = v35.wrapping_sub(1i32);
    let v39: Rc<str> = string_slice(&v34.clone(), v37 as i64, v38 as i64);
    let mut v40: i32 = (v39.clone().len() as i32);
    let mut v41: i32 = method13(v39.clone(), v40);
    let v42: Rc<str> = string_slice(&v39.clone(), 0i32 as i64, v41 as i64);
    let mut v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v42.split(&*v43).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v45: Rc<RefCell<Vec<Rc<str>>>> = v44 ;
    let v46: i32 = (v45.clone().borrow().len() as i32);
    let v47: bool = 1i32 < v46;
    let v51: US1 = if v47 {
        let v48: Rc<str> = v45.borrow()[1i32 as usize].clone();
        US1::US1_Some(v48.clone())
    } else {
        US1::US1_None
    };
    let v55: Rc<str> = match &v51 {
        US1::US1_None => {
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v53.clone()
        }
        US1::US1_Some(v52) => {
            let v52: Rc<str> = v52.clone();
            v52.clone()
        }
    };
    let v56: bool = &*v42 == "";
    let v75: u64 = if v56 {
        v1
    } else {
        let v57: i32 = (v55.clone().len() as i32);
        let v58: i32 = 0i32;
        let v59: i32 = method12(v55.clone(), v57, v58);
        let v60: i32 = v57.wrapping_sub(1i32);
        let v61: Rc<str> = string_slice(&v55.clone(), v59 as i64, v60 as i64);
        let v62: i32 = (v61.clone().len() as i32);
        let v63: i32 = method13(v61.clone(), v62);
        let v64: Rc<str> = string_slice(&v61.clone(), 0i32 as i64, v63 as i64);
        let v65: Option<u64> = (v64).trim().parse::<u64>().ok();
        let v66: Option<u64> = method14(v65.clone());
        let mut v67: Option<US0> = v66.map(|x| v23(x));
        let v69: US0 = US0::US0_None;
        let v70: US0 = v69.clone();
        let v71: US0 = v67.unwrap_or(v70);
        match &v71 {
            US0::US0_None => {
                0u64
            }
            US0::US0_Some(v72) => {
                let v72: u64 = *v72;
                v72
            }
        }
    };
    let v76: bool = v75 > v31;
    let v78: u64 = if v76 {
        let v77: u64 = v75.wrapping_sub(v31);
        v77
    } else {
        0u64
    };
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
    let v80: Rc<str> = job_file_7(v0.clone(), v79.clone());
    let mut v81: Rc<str> = text_or_empty_11(v80.clone());
    let v82: i32 = (v81.clone().len() as i32);
    let v83: i32 = 0i32;
    let v84: i32 = method12(v81.clone(), v82, v83);
    let v85: i32 = v82.wrapping_sub(1i32);
    let v86: Rc<str> = string_slice(&v81.clone(), v84 as i64, v85 as i64);
    let v87: i32 = (v86.clone().len() as i32);
    let v88: i32 = method13(v86.clone(), v87);
    let v89: Rc<str> = string_slice(&v86.clone(), 0i32 as i64, v88 as i64);
    let v90: bool = &*v42 != "";
    (v0.clone(), v89.clone(), v78, v90)
}
fn jobs_seen_16(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: u64, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (v0.clone().borrow().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            return v3.clone();
        } else {
            let mut v6: Rc<str> = v0.borrow()[v1 as usize].clone();
            let (mut v7, mut v8, mut v9, mut v10): (Rc<str>, Rc<str>, u64, bool) = job_seen_of_17(v6.clone(), v2);
            let mut v11: i32 = v1.wrapping_add(1i32);
            let v12: bool = &*v8 == "";
            let v14: Rc<UH0> = if v12 {
                v3.clone()
            } else {
                Rc::new(UH0::UH0_Cons(v7.clone(), v8.clone(), v9, v10, v3.clone()))
            };
            (v1, v3) = (v11, v14.clone());
            continue;
        }
    }
}
fn method20(mut v0: Rc<str>, mut v1: Rc<UH0>, mut v2: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: u64 = *v5;
                let mut v6: bool = *v6;
                let v7: Rc<UH0> = v7.clone();
                let v9: bool = if v6 {
                    let v8: bool = &*v4 == &*v0;
                    v8
                } else {
                    false
                };
                let mut v11: Rc<UH2> = if v9 {
                    Rc::new(UH2::UH2_Cons(v5, v2.clone()))
                } else {
                    v2.clone()
                };
                (v1, v2) = (v7.clone(), v11.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method22(mut v0: Rc<UH2>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = *v2;
                let v3: Rc<UH2> = v3.clone();
                let mut v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH2::UH2_Nil => {
                return v1;
            }
        }
    }
}
fn nth_of_24(mut v0: Rc<UH2>, mut v1: i32) -> u64 {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = *v2;
                let v3: Rc<UH2> = v3.clone();
                let mut v4: bool = v1 <= 0i32;
                if v4 {
                    return v2;
                } else {
                    let v5: i32 = v1.wrapping_sub(1i32);
                    (v0, v1) = (v3.clone(), v5);
                    continue;
                }
            }
            UH2::UH2_Nil => {
                return 0u64;
            }
        }
    }
}
fn sorted_in_26(mut v0: u64, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v4, v5) => {
            let mut v4: u64 = *v4;
            let v5: Rc<UH2> = v5.clone();
            let mut v6: bool = v0 <= v4;
            if v6 {
                Rc::new(UH2::UH2_Cons(v0, v1))
            } else {
                let v8: Rc<UH2> = sorted_in_26(v0, v5);
                Rc::new(UH2::UH2_Cons(v4, v8))
            }
        }
        UH2::UH2_Nil => {
            let mut v2: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v0, v2))
        }
    }
}
fn method25(mut v0: Rc<UH2>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3) => {
                let mut v2: u64 = *v2;
                let v3: Rc<UH2> = v3.clone();
                let mut v4: Rc<UH2> = sorted_in_26(v2, v1.clone());
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn usual_ms_23(mut v0: Rc<UH2>) -> u64 {
    let mut v1: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v2: Rc<UH2> = method25(v0.clone(), v1);
    let v3: i32 = 0i32;
    let mut v4: i32 = method22(v0, v3);
    let v5: i32 = v4.wrapping_div(2i32);
    nth_of_24(v2, v5)
}
fn stalled_by_21(mut v0: u64, mut v1: Rc<UH2>) -> bool {
    let mut v2: i32 = 0i32;
    let v3: i32 = method22(v1.clone(), v2);
    let mut v4: bool = v3 >= 3i32;
    if v4 {
        let v5: u64 = usual_ms_23(v1);
        let mut v6: u64 = 2u64.wrapping_mul(v5);
        let v7: bool = v0 > v6;
        v7
    } else {
        false
    }
}
fn stalled_step_19(mut v0: Rc<UH0>, mut v1: Rc<UH1>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: bool) -> Rc<UH1> {
    let mut v6: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let v7: Rc<UH2> = method20(v3, v0, v6);
    let v10: bool = if v5 {
        true
    } else {
        let v8: bool = stalled_by_21(v4, v7.clone());
        let v9: bool = v8 == false;
        v9
    };
    if v10 {
        v1.clone()
    } else {
        let mut v11: i32 = 0i32;
        let v12: i32 = method22(v7.clone(), v11);
        let v13: Rc<str> = std::rc::Rc::<str>::from((v12).to_string());
        let v14: u64 = usual_ms_23(v7);
        let v15: u64 = v14.wrapping_div(60000u64);
        let v16: Rc<str> = std::rc::Rc::<str>::from((v15).to_string());
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  job "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v2));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" running "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
        let mut v21: u64 = v4.wrapping_div(60000u64);
        let mut v22: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
        let mut v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min, usual "); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v24)), v16));
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min over "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v27)), v13));
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" runs  stalled"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
        Rc::new(UH1::UH1_Cons(v31, v1))
    }
}
fn method18(mut v0: Rc<UH0>, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6, v7) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<str> = v4.clone();
                let v5: u64 = *v5;
                let mut v6: bool = *v6;
                let v7: Rc<UH0> = v7.clone();
                let v8: Rc<UH1> = stalled_step_19(v0.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6);
                (v1, v2) = (v7.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method27(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH1>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH1::UH1_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let mut v4: Rc<UH1> = v4.clone();
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4.clone(), v5);
                continue;
            }
            UH1::UH1_Nil => {
                return v2;
            }
        }
    }
}
fn stalled_jobs_15() -> Rc<str> {
    let mut v0: Rc<str> = jobs_root_10();
    let mut v1: u64 = 300u64;
    let mut v2: Rc<str> = { let mut names = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with("job-")).collect::<Vec<_>>()).unwrap_or_default(); names.sort(); names.reverse(); names.truncate(v1 as usize); std::rc::Rc::<str>::from(names.join("
")) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v2.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = 0i32;
    let mut v6: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
    let v7: u64 = v6.wrapping_mul(1000u64);
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = jobs_seen_16(v4.clone(), v5, v7, v8.clone());
    let v10: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v11: Rc<UH1> = method18(v9.clone(), v9.clone(), v10.clone());
    let v12: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    let v13: i32 = 0i32;
    let v14: i32 = method27(v12.clone(), v11.clone(), v13);
    let v15: Rc<Vec<Rc<str>>> = Rc::new(v12.borrow().clone());
    let v16: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v15).as_ref().clone()));
    let v17: Rc<str> = Rc::<str>::from((*v16.borrow()).join(&*v3));
    v17.clone()
}
fn closure4() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        stalled_jobs_15()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn file_exists_30(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn history_appended_28(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = get_environment_variable_29(v1.clone());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\kit\\machine-watch"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v3));
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\health.tsv"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let v8: Result<(), std::io::Error> = std::fs::create_dir_all(&*v4);
    let v9: bool = file_exists_30(v6.clone());
    let v26: Rc<str> = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v10.clone()
    } else {
        let mut v11: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unix"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v12);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tools"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v13);
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v14);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unowned"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v15);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v16);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leftover"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v17);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hogs"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v18);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stalled"); } LIT.with(|lit| lit.clone()) };
        v11.borrow_mut().push(v19);
        let v20: Rc<Vec<Rc<str>>> = Rc::new(v11.borrow().clone());
        let mut v21: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v20).as_ref().clone()));
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = Rc::<str>::from((*v21.borrow()).join(&*v22));
        let mut v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
        v25.clone()
    };
    let v27: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs());
    let v28: Rc<str> = std::rc::Rc::<str>::from((v27).to_string());
    let mut v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v29.borrow_mut().push(v28);
    v29.borrow_mut().push(v0);
    let v30: Rc<Vec<Rc<str>>> = Rc::new(v29.borrow().clone());
    let v31: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v30).as_ref().clone()));
    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v33: Rc<str> = Rc::<str>::from((*v31.borrow()).join(&*v32));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v33));
    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let mut v37: std::io::Result<std::fs::File> = std::fs::OpenOptions::new().create(true).append(true).open(&*v6);
    let v38: bool = v37.is_ok();
    let v39: bool = v38 == false;
    if v39 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the file could not be opened"); } LIT.with(|lit| lit.clone()) }))
    };
    let mut v40: std::fs::File = v37.unwrap();
    let mut v41: std::io::Result<()> = std::io::Write::write_all(&mut v40, (v36).as_bytes());
    let v42: bool = v41.is_ok();
    let mut v43: bool = v42 == false;
    if v43 {
        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file_system.append_all_text: the write failed"); } LIT.with(|lit| lit.clone()) }))
    };
    v6.clone()
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        history_appended_28(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_machine_sampled(v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    closure0()(v0)
}
pub fn kit_machine_cells_vm(v0: &str) -> bool {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_machine_job_left(v0: u64, v1: u64) -> Rc<str> {
    closure2()(v0, v1)
}
pub fn kit_machine_stalled_jobs() -> Rc<str> {
    closure4()()
}
pub fn kit_machine_history_appended(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
