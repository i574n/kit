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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_JobHost,
    US1_JobSupervisor(Rc<str>),
    US1_JobWaiter(Rc<str>),
    US1_WindowHolder,
    US1_OtherCall,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(u64, bool, US1, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_IdleOrphanHost(u64),
    US2_SettledButSupervised(u64, Rc<str>),
    US2_WaitOnSettled(u64, Rc<str>),
    US2_WaitOnMissing(u64, Rc<str>),
    US2_ChainToMissing(Rc<str>, Rc<str>),
    US2_HostEnvironmentDiffers(Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(US2, Rc<UH1>),
}
#[derive(Clone)]
enum US3 {
    US3_JobLive,
    US3_JobSettled,
    US3_JobMissing,
}
#[derive(Clone)]
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<str>, Rc<str>, US3, Rc<UH2>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US4 {
    US4_Some(US2),
    US4_None,
}
struct Mut0 { l0: Rc<str> }
fn env_drift_0() -> (bool, Rc<str>) {
    let (mut v0, mut v1): (bool, Rc<str>) = kit_jobs_doctor_env::kit_job_doctor_env_drift();
    (v0, v1)
}
fn closure1() -> Rc<dyn Fn() -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>)> = Rc::new(move || -> (bool, Rc<str>) {
        env_drift_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn method6(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: bool = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v3, v4.clone(), v1.clone()));
                (v0, v1) = (v5.clone(), v6.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method7(mut v0: Option<usize>) -> Option<usize> {
    v0
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let v1: usize = (v0);
        let v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
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
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method9(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn facts_from_5(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v4: bool = v2 <= v1;
        if v4 {
            let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            return method6(v3.clone(), v5.clone());
        } else {
            let mut v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
            let v9: bool = v7.starts_with(&*v8);
            let v10: bool = v9 == false;
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v3.clone());
                continue;
            } else {
                let v13: u64 = v0[v1 as usize].0;
                let v14: u64 = v0[v1 as usize].1;
                let v15: i32 = v0.iter().position(|row| row.0 == v14).map(|at| at as i32).unwrap_or(-1);
                let v16: bool = v15 >= 0i32;
                let v17: Rc<str> = v0[v1 as usize].3.clone();
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job host"); } LIT.with(|lit| lit.clone()) };
                let v19: bool = v17.contains(&*v18);
                let v98: US1 = if v19 {
                    US1::US1_JobHost
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job supervise "); } LIT.with(|lit| lit.clone()) };
                    let v22: bool = v17.contains(&*v21);
                    if v22 {
                        let v23: Option<usize> = v17.find(&*v21);
                        let v24: Option<usize> = method7(v23.clone());
                        let mut v25: Rc<dyn Fn((usize)) -> i32> = closure2();
                        let mut v26: Option<i32> = v24.map(|x| v25(x));
                        let v28: i32 = v26.unwrap_or(-1i32);
                        let v29: bool = v28 < 0i32;
                        let v54: Rc<str> = if v29 {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v30.clone()
                        } else {
                            let v31: i32 = (v21.clone().len() as i32);
                            let v32: i32 = v28.wrapping_add(v31);
                            let v33: i32 = (v17.clone().len() as i32);
                            let v34: i32 = v33.wrapping_sub(v28);
                            let v35: i32 = v34.wrapping_sub(v31);
                            let v36: usize = ((v32) as usize);
                            let v37: i32 = v32.wrapping_add(v35);
                            let v38: usize = ((v37) as usize);
                            let v39: Rc<str> = Rc::<str>::from(&(v17)[v36..v38]);
                            let v40: i32 = (v39.clone().len() as i32);
                            let v41: i32 = 0i32;
                            let v42: i32 = method8(v39.clone(), v40, v41);
                            let v43: i32 = v40.wrapping_sub(1i32);
                            let v44: Rc<str> = string_slice(&v39.clone(), v42 as i64, v43 as i64);
                            let v45: i32 = (v44.clone().len() as i32);
                            let v46: i32 = method9(v44.clone(), v45);
                            let v47: Rc<str> = string_slice(&v44.clone(), 0i32 as i64, v46 as i64);
                            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                            let v49: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v47.split(&*v48).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                            let v50: Rc<str> = v49.clone().borrow()[0i32 as usize].clone();
                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v53: Rc<str> = Rc::<str>::from(v50.replace(&*v51, &*v52));
                            v53.clone()
                        };
                        US1::US1_JobSupervisor(v54.clone())
                    } else {
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job wait "); } LIT.with(|lit| lit.clone()) };
                        let v57: bool = v17.contains(&*v56);
                        if v57 {
                            let v58: Option<usize> = v17.find(&*v56);
                            let v59: Option<usize> = method7(v58.clone());
                            let mut v60: Rc<dyn Fn((usize)) -> i32> = closure2();
                            let mut v61: Option<i32> = v59.map(|x| v60(x));
                            let v63: i32 = v61.unwrap_or(-1i32);
                            let v64: bool = v63 < 0i32;
                            let v89: Rc<str> = if v64 {
                                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v65.clone()
                            } else {
                                let v66: i32 = (v56.clone().len() as i32);
                                let v67: i32 = v63.wrapping_add(v66);
                                let v68: i32 = (v17.clone().len() as i32);
                                let v69: i32 = v68.wrapping_sub(v63);
                                let v70: i32 = v69.wrapping_sub(v66);
                                let v71: usize = ((v67) as usize);
                                let v72: i32 = v67.wrapping_add(v70);
                                let v73: usize = ((v72) as usize);
                                let v74: Rc<str> = Rc::<str>::from(&(v17)[v71..v73]);
                                let v75: i32 = (v74.clone().len() as i32);
                                let v76: i32 = 0i32;
                                let v77: i32 = method8(v74.clone(), v75, v76);
                                let v78: i32 = v75.wrapping_sub(1i32);
                                let v79: Rc<str> = string_slice(&v74.clone(), v77 as i64, v78 as i64);
                                let v80: i32 = (v79.clone().len() as i32);
                                let v81: i32 = method9(v79.clone(), v80);
                                let v82: Rc<str> = string_slice(&v79.clone(), 0i32 as i64, v81 as i64);
                                let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                let v84: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v82.split(&*v83).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                                let v85: Rc<str> = v84.clone().borrow()[0i32 as usize].clone();
                                let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v88: Rc<str> = Rc::<str>::from(v85.replace(&*v86, &*v87));
                                v88.clone()
                            };
                            US1::US1_JobWaiter(v89.clone())
                        } else {
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" window run "); } LIT.with(|lit| lit.clone()) };
                            let v92: bool = v17.contains(&*v91);
                            if v92 {
                                US1::US1_WindowHolder
                            } else {
                                US1::US1_OtherCall
                            }
                        }
                    }
                };
                let v99: i32 = v1.wrapping_add(1i32);
                let v100: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v16, v98.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v99, v2, v100.clone());
                continue;
            }
        }
    }
}
fn process_rows_sampled_10(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn process_facts_4() -> Rc<UH0> {
    let mut v0: u64 = 50u64;
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_10(v0);
    let v2: i32 = 0i32;
    let v3: i32 = v1.len() as i32;
    let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    facts_from_5(v1.clone(), v2, v3, v4.clone())
}
fn method11(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: bool = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let v8: i32 = match &v4 {
                    US1::US1_JobSupervisor(v6) => {
                        let mut v6: Rc<str> = v6.clone();
                        let mut v7: i32 = v1.wrapping_add(1i32);
                        v7
                    }
                    _ => {
                        v1
                    }
                };
                (v0, v1) = (v5.clone(), v8);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn joined_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_16() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_14(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_16();
    joined_15(v1, v0)
}
fn directory_exists_17(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_14(v0);
    joined_15(v2, v1)
}
fn file_exists_19(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn standing_of_13(mut v0: Rc<str>) -> US3 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.starts_with(&*v1);
    let v5: bool = if v2 {
        let v3: Rc<str> = job_directory_14(v0.clone());
        directory_exists_17(v3.clone())
    } else {
        false
    };
    let mut v6: bool = v5 == false;
    if v6 {
        US3::US3_JobMissing
    } else {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = job_file_18(v0.clone(), v8.clone());
        let v10: bool = file_exists_19(v9.clone());
        if v10 {
            US3::US3_JobSettled
        } else {
            US3::US3_JobLive
        }
    }
}
fn method12(mut v0: bool, mut v1: Rc<UH0>, mut v2: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v3, v4, v5, v6) => {
                let v3: u64 = *v3;
                let v4: bool = *v4;
                let v5: US1 = v5.clone();
                let mut v6: Rc<UH0> = v6.clone();
                let v28: Rc<UH1> = match &v5 {
                    US1::US1_JobHost => {
                        let mut v7: bool = v4 || v0;
                        if v7 {
                            v2.clone()
                        } else {
                            let v8: US2 = US2::US2_IdleOrphanHost(v3);
                            Rc::new(UH1::UH1_Cons(v8.clone(), v2.clone()))
                        }
                    }
                    US1::US1_JobSupervisor(v11) => {
                        let v11: Rc<str> = v11.clone();
                        let v12: US3 = standing_of_13(v11.clone());
                        let v13: bool = match &v12 {
                            US3::US3_JobSettled => {
                                true
                            }
                            _ => {
                                false
                            }
                        };
                        if v13 {
                            let v14: US2 = US2::US2_SettledButSupervised(v3, v11.clone());
                            Rc::new(UH1::UH1_Cons(v14.clone(), v2.clone()))
                        } else {
                            v2.clone()
                        }
                    }
                    US1::US1_JobWaiter(v17) => {
                        let v17: Rc<str> = v17.clone();
                        let v18: US3 = standing_of_13(v17.clone());
                        match &v18 {
                            US3::US3_JobLive => {
                                v2.clone()
                            }
                            US3::US3_JobMissing => {
                                let v21: US2 = US2::US2_WaitOnMissing(v3, v17.clone());
                                Rc::new(UH1::UH1_Cons(v21.clone(), v2.clone()))
                            }
                            US3::US3_JobSettled => {
                                let v19: US2 = US2::US2_WaitOnSettled(v3, v17.clone());
                                Rc::new(UH1::UH1_Cons(v19.clone(), v2.clone()))
                            }
                        }
                    }
                    _ => {
                        v2.clone()
                    }
                };
                (v0, v1, v2) = (v0, v6.clone(), v28.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v2.clone();
            }
        }
    }
}
fn method20(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US2 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method23(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method22(mut v0: Rc<UH3>, mut v1: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH3> = v3.clone();
                let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v5: Rc<str> = job_file_18(v2.clone(), v4.clone());
                let mut v6: bool = file_exists_19(v5.clone());
                let v11: bool = if v6 {
                    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after-done"); } LIT.with(|lit| lit.clone()) };
                    let v8: Rc<str> = job_file_18(v2.clone(), v7.clone());
                    let v9: bool = file_exists_19(v8.clone());
                    let v10: bool = v9 == false;
                    v10
                } else {
                    false
                };
                let v16: bool = if v11 {
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
                    let v13: Rc<str> = job_file_18(v2.clone(), v12.clone());
                    let v14: bool = file_exists_19(v13.clone());
                    let v15: bool = v14 == false;
                    v15
                } else {
                    false
                };
                let v41: Rc<UH2> = if v16 {
                    let v18: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v5);
                    let v20: Vec<u8> = v18.unwrap();
                    let v21: Vec<u8> = method23(v20.clone());
                    let v23: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v21);
                    let mut v25: std::string::String = v23.unwrap();
                    let v27: Rc<str> = Rc::<str>::from(String::as_str(&v25));
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                    let v29: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v27.split(&*v28).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
                    let v30: Rc<str> = v29.clone().borrow()[0i32 as usize].clone();
                    let v31: i32 = (v30.clone().len() as i32);
                    let v32: i32 = 0i32;
                    let v33: i32 = method8(v30.clone(), v31, v32);
                    let v34: i32 = v31.wrapping_sub(1i32);
                    let v35: Rc<str> = string_slice(&v30.clone(), v33 as i64, v34 as i64);
                    let v36: i32 = (v35.clone().len() as i32);
                    let v37: i32 = method9(v35.clone(), v36);
                    let v38: Rc<str> = string_slice(&v35.clone(), 0i32 as i64, v37 as i64);
                    let v39: US3 = standing_of_13(v38.clone());
                    Rc::new(UH2::UH2_Cons(v2.clone(), v38.clone(), v39.clone(), v1.clone()))
                } else {
                    v1.clone()
                };
                (v0, v1) = (v3.clone(), v41.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method25(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match &*v0 {
        UH3::UH3_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH3> = v3.clone();
            let v4: Rc<UH3> = method25(v3.clone(), v1.clone());
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
            let mut v6: bool = v2.starts_with(&*v5);
            if v6 {
                Rc::new(UH3::UH3_Cons(v2.clone(), v4.clone()))
            } else {
                v4.clone()
            }
        }
        UH3::UH3_Nil => {
            v1.clone()
        }
    }
}
fn directory_exists_26(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn job_names_24() -> Rc<UH3> {
    let mut v0: Rc<str> = jobs_root_16();
    let v1: bool = directory_exists_26(v0.clone());
    let v2: bool = v1 == false;
    if v2 {
        { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) }
    } else {
        let v5: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
        let mut v7: Vec<std::string::String> = v5.unwrap();
        let v8: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        let v10: bool = true; let _vec_fold_ = v7.into_iter().fold(v8, |acc, x| { //;
        let v12: Rc<UH3> = acc;
        let v14: std::string::String = x;
        let v16: Rc<str> = Rc::<str>::from(String::as_str(&v14));
        let v18: Rc<UH3> = Rc::new(UH3::UH3_Cons(v16.clone(), v12.clone()));
        let v19: bool = true; v18 });
        let v21: Rc<UH3> = _vec_fold_;
        let v22: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
        method25(v21.clone(), v22.clone())
    }
}
fn chain_facts_21() -> Rc<UH2> {
    let mut v0: Rc<UH3> = job_names_24();
    let v1: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    method22(v0.clone(), v1.clone())
}
fn method27(mut v0: Rc<UH2>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH2::UH2_Cons(v2, v3, v4, v5) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<str> = v3.clone();
                let v4: US3 = v4.clone();
                let v5: Rc<UH2> = v5.clone();
                let v8: Rc<UH1> = match &v4 {
                    US3::US3_JobMissing => {
                        let mut v6: US2 = US2::US2_ChainToMissing(v2.clone(), v3.clone());
                        Rc::new(UH1::UH1_Cons(v6.clone(), v1.clone()))
                    }
                    _ => {
                        v1.clone()
                    }
                };
                (v0, v1) = (v5.clone(), v8.clone());
                continue;
            }
            UH2::UH2_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method28(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US2 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn environment_finding_29(mut v0: Rc<dyn Fn() -> (bool, Rc<str>)>) -> US4 {
    let (mut v1, mut v2): (bool, Rc<str>) = v0();
    let v3: bool = v1 == false;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("(an environment listing did not run)"); } LIT.with(|lit| lit.clone()) };
        let v5: US2 = US2::US2_HostEnvironmentDiffers(v4.clone());
        US4::US4_Some(v5.clone())
    } else {
        let mut v7: bool = &*v2 == "";
        if v7 {
            US4::US4_None
        } else {
            let v9: US2 = US2::US2_HostEnvironmentDiffers(v2.clone());
            US4::US4_Some(v9.clone())
        }
    }
}
fn method32(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_31(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method30(mut v0: Rc<UH1>) -> () {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v1, v2) => {
                let v1: US2 = v1.clone();
                let v2: Rc<UH1> = v2.clone();
                let v48: Rc<str> = match &v1 {
                    US2::US2_ChainToMissing(v33, v34) => {
                        let v33: Rc<str> = v33.clone();
                        let v34: Rc<str> = v34.clone();
                        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("chain-to-missing job="); } LIT.with(|lit| lit.clone()) };
                        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v33));
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v34));
                        v39.clone()
                    }
                    US2::US2_HostEnvironmentDiffers(v40) => {
                        let v40: Rc<str> = v40.clone();
                        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("host-environment-differs "); } LIT.with(|lit| lit.clone()) };
                        let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v40));
                        v42.clone()
                    }
                    US2::US2_IdleOrphanHost(v3) => {
                        let v3: u64 = *v3;
                        let v4: Rc<str> = format_real_31(v3);
                        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("idle-orphan-host pid="); } LIT.with(|lit| lit.clone()) };
                        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v4));
                        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (its parent is gone and it serves no job)"); } LIT.with(|lit| lit.clone()) };
                        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                        v8.clone()
                    }
                    US2::US2_SettledButSupervised(v9, v10) => {
                        let v9: u64 = *v9;
                        let v10: Rc<str> = v10.clone();
                        let v11: Rc<str> = format_real_31(v9);
                        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("settled-but-supervised pid="); } LIT.with(|lit| lit.clone()) };
                        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
                        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v14)), v10));
                        v16.clone()
                    }
                    US2::US2_WaitOnMissing(v25, v26) => {
                        let mut v25: u64 = *v25;
                        let mut v26: Rc<str> = v26.clone();
                        let v27: Rc<str> = format_real_31(v25);
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-missing pid="); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v27));
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v30)), v26));
                        v32.clone()
                    }
                    US2::US2_WaitOnSettled(v17, v18) => {
                        let v17: u64 = *v17;
                        let v18: Rc<str> = v18.clone();
                        let v19: Rc<str> = format_real_31(v17);
                        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait-on-settled pid="); } LIT.with(|lit| lit.clone()) };
                        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" job="); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v22)), v18));
                        v24.clone()
                    }
                };
                println!("{}", v48);
                v0 = v2.clone();
                continue;
            }
            UH1::UH1_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn method33(mut v0: Rc<UH0>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3, v4, v5) => {
                let v2: u64 = *v2;
                let v3: bool = *v3;
                let v4: US1 = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v5.clone(), v6);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn format_real_34(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn method35(mut v0: Rc<UH1>, mut v1: i32) -> i32 {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: US2 = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v3.clone(), v4);
                continue;
            }
            UH1::UH1_Nil => {
                return v1;
            }
        }
    }
}
fn doctor_run_1(mut v0: Rc<dyn Fn() -> (bool, Rc<str>)>) -> i32 {
    let v1: i32 = command_line_argument_count_2();
    let v2: bool = 2i32 < v1;
    let mut v7: US0 = if v2 {
        let v3: i32 = 2i32;
        let v4: Rc<str> = command_line_argument_3(v3);
        US0::US0_Some(v4.clone())
    } else {
        US0::US0_None
    };
    let v11: Rc<str> = match &v7 {
        US0::US0_None => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        }
        US0::US0_Some(v8) => {
            let v8: Rc<str> = v8.clone();
            v8.clone()
        }
    };
    let v12: bool = &*v11 == "--env";
    let v13: Rc<UH0> = process_facts_4();
    let v14: i32 = 0i32;
    let v15: i32 = method11(v13.clone(), v14);
    let v16: bool = v15 > 0i32;
    let v17: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v18: Rc<UH1> = method12(v16, v13.clone(), v17.clone());
    let v19: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v20: Rc<UH1> = method20(v18.clone(), v19.clone());
    let v21: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v22: Rc<UH1> = method20(v20.clone(), v21.clone());
    let v23: Rc<UH2> = chain_facts_21();
    let v24: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let mut v25: Rc<UH1> = method27(v23.clone(), v24.clone());
    let mut v26: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v27: Rc<UH1> = method20(v25.clone(), v26.clone());
    let v28: Rc<UH1> = method28(v27.clone(), v22.clone());
    let v29: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    let v30: Rc<UH1> = method20(v28.clone(), v29.clone());
    let v31: bool = v12 == false;
    let v41: Rc<UH1> = if v31 {
        v30.clone()
    } else {
        let v32: US4 = environment_finding_29(v0.clone());
        match &v32 {
            US4::US4_None => {
                v30.clone()
            }
            US4::US4_Some(v33) => {
                let v33: US2 = v33.clone();
                let v34: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v35: Rc<UH1> = method20(v30.clone(), v34.clone());
                let v36: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v37: Rc<UH1> = Rc::new(UH1::UH1_Cons(v33.clone(), v36.clone()));
                method20(v35.clone(), v37.clone())
            }
        }
    };
    method30(v41.clone());
    let v42: i32 = 0i32;
    let v43: i32 = method33(v13.clone(), v42);
    let v44: Rc<str> = format_real_34(v43);
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" processes="); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" findings="); } LIT.with(|lit| lit.clone()) };
    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
    let v49: i32 = 0i32;
    let v50: i32 = method35(v41.clone(), v49);
    let v51: Rc<str> = format_real_34(v50);
    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v51));
    match &*v41 {
        UH1::UH1_Nil => {
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job doctor ok"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v52));
            println!("{}", v54);
            0i32
        }
        _ => {
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit job doctor refused:"); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v52));
            println!("{}", v56);
            1i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        let mut v0: Rc<dyn Fn() -> (bool, Rc<str>)> = closure1();
        doctor_run_1(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_doctor_run() -> i32 {
    closure0()()
}
