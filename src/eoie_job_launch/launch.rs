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
    US0_PlainStart,
    US0_HeavyStart,
    US0_CompileStart,
}
#[derive(Clone)]
enum US2 {
    US2_MaxPrivateMb,
    US2_SlotWithin,
    US2_DoneWithin,
    US2_QuietStart,
    US2_QuietHold,
    US2_QuietWait,
    US2_MinFreeMb,
    US2_SettleMs,
}
#[derive(Clone)]
enum US1 {
    US1_StartFlagKnown(US2),
    US1_StartFlagUnknown,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_Launched(Rc<str>, Rc<str>),
    US4_LaunchRefused(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_SettleHealthy,
    US5_SettleFailed(i32, Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_JobPending,
    US7_JobRunning,
}
#[derive(Clone)]
enum US8 {
    US8_JobFinished(i32),
    US8_JobTimedOut,
    US8_JobStopped,
    US8_JobSpawnFailed(Rc<str>),
    US8_JobSkipped(Rc<str>),
    US8_JobLost,
    US8_JobMemoryCapped,
    US8_JobExpiredInQueue,
}
#[derive(Clone)]
enum US6 {
    US6_Live(US7),
    US6_Settled(US8),
}
struct Mut0 { l0: Rc<str> }
fn start_flag_of_3(mut v0: Rc<str>) -> US1 {
    let mut v1: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb"); } LIT.with(|lit| lit.clone()) };
    if v1 {
        let mut v2: US2 = US2::US2_MaxPrivateMb;
        US1::US1_StartFlagKnown(v2.clone())
    } else {
        let mut v4: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within"); } LIT.with(|lit| lit.clone()) };
        if v4 {
            let mut v5: US2 = US2::US2_SlotWithin;
            US1::US1_StartFlagKnown(v5.clone())
        } else {
            let mut v7: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within"); } LIT.with(|lit| lit.clone()) };
            if v7 {
                let mut v8: US2 = US2::US2_DoneWithin;
                US1::US1_StartFlagKnown(v8.clone())
            } else {
                let mut v10: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet"); } LIT.with(|lit| lit.clone()) };
                if v10 {
                    let mut v11: US2 = US2::US2_QuietStart;
                    US1::US1_StartFlagKnown(v11.clone())
                } else {
                    let mut v13: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold"); } LIT.with(|lit| lit.clone()) };
                    if v13 {
                        let mut v14: US2 = US2::US2_QuietHold;
                        US1::US1_StartFlagKnown(v14.clone())
                    } else {
                        let mut v16: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-wait"); } LIT.with(|lit| lit.clone()) };
                        if v16 {
                            let mut v17: US2 = US2::US2_QuietWait;
                            US1::US1_StartFlagKnown(v17.clone())
                        } else {
                            let mut v19: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
                            if v19 {
                                let mut v20: US2 = US2::US2_MinFreeMb;
                                US1::US1_StartFlagKnown(v20.clone())
                            } else {
                                let mut v22: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--settle-ms"); } LIT.with(|lit| lit.clone()) };
                                if v22 {
                                    let mut v23: US2 = US2::US2_SettleMs;
                                    US1::US1_StartFlagKnown(v23.clone())
                                } else {
                                    US1::US1_StartFlagUnknown
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn method5(mut v0: Option<u64>) -> Option<u64> {
    v0.clone()
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let mut v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_4(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method5(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let mut v7: US3 = v6.clone();
    let mut v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn start_options_read_2(mut v0: i32, mut v1: bool, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: bool, mut v6: u64, mut v7: bool, mut v8: bool, mut v9: u64, mut v10: bool, mut v11: u64, mut v12: bool, mut v13: u64, mut v14: bool, mut v15: u64) -> (i32, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) {
    loop {
        let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let mut v17: US1 = start_flag_of_3(v16.clone());
        match &v17 {
            US1::US1_StartFlagKnown(v18) => {
                let mut v18: US2 = v18.clone();
                let mut v19: i32 = match &v18 {
                    US2::US2_QuietStart => {
                        1i32
                    }
                    _ => {
                        2i32
                    }
                };
                let mut v20: bool = v19 == 2i32;
                let mut v24: u64 = if v20 {
                    let mut v21: i32 = v0.wrapping_add(1i32);
                    let mut v22: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v21 as usize).unwrap_or_default());
                    decimal_u64_4(v22.clone())
                } else {
                    0u64
                };
                let mut v25: i32 = v0.wrapping_add(v19);
                let (mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                    US2::US2_MaxPrivateMb => {
                        (true, v24, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => {
                        let (mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                            US2::US2_SlotWithin => {
                                (v1, v2, true, v24, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                            }
                            _ => {
                                let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                    US2::US2_DoneWithin => {
                                        (v1, v2, v3, v4, true, v24, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                                    }
                                    _ => {
                                        let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                            US2::US2_QuietStart => {
                                                (v1, v2, v3, v4, v5, v6, true, v8, v9, v10, v11, v12, v13, v14, v15)
                                            }
                                            _ => {
                                                let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                                    US2::US2_QuietHold => {
                                                        (v1, v2, v3, v4, v5, v6, v7, true, v24, v10, v11, v12, v13, v14, v15)
                                                    }
                                                    _ => {
                                                        let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                                            US2::US2_QuietWait => {
                                                                (v1, v2, v3, v4, v5, v6, v7, v8, v9, true, v24, v12, v13, v14, v15)
                                                            }
                                                            _ => {
                                                                let (mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                                                    US2::US2_MinFreeMb => {
                                                                        (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, true, v24, v14, v15)
                                                                    }
                                                                    _ => {
                                                                        let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v18 {
                                                                            US2::US2_SettleMs => {
                                                                                (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, true, v24)
                                                                            }
                                                                            _ => unreachable!(),
                                                                        };
                                                                        (v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40)
                                                                    }
                                                                };
                                                                (v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55)
                                                            }
                                                        };
                                                        (v56, v57, v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
                                                    }
                                                };
                                                (v71, v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
                                            }
                                        };
                                        (v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100)
                                    }
                                };
                                (v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112, v113, v114, v115)
                            }
                        };
                        (v116, v117, v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130)
                    }
                };
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = (v25, v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145);
                continue;
            }
            US1::US1_StartFlagUnknown => {
                return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
            }
        }
    }
}
fn joined_7(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2.clone()
}
fn jobs_root_8() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("eoie").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("eoie").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".eoie-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = jobs_root_8();
    joined_7(v1.clone(), v0.clone())
}
fn directory_exists_9(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let mut v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_11() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_10() -> i32 {
    let mut v0: i32 = command_line_argument_count_11();
    let mut v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn stderr_line_13(mut v0: Rc<str>) -> () {
    eprintln!("{}", &*v0);
    ()
}
fn rejected_12(mut v0: Rc<str>) -> i32 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie error: job "); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v2, &*v2].concat());
    stderr_line_13(v3.clone());
    2i32
}
fn now_ms_15() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn decimal_text_16(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1.clone()
}
fn job_file_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = job_directory_6(v0.clone());
    joined_7(v2.clone(), v1.clone())
}
fn text_written_19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn heavy_gate_script_20() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("EOIE_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn job_argv_17(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match &v0 {
        US0::US0_CompileStart => {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = job_file_18(v1.clone(), v5.clone());
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let mut v8: Rc<str> = text_written_19(v6.clone(), v7.clone());
            v3.clone()
        }
        US0::US0_HeavyStart => {
            let mut v9: Rc<str> = heavy_gate_script_20();
            let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            if v10 {
                let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            } else {
                let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let mut v13: Rc<str> = job_file_18(v1.clone(), v12.clone());
                let mut v14: Rc<str> = text_written_19(v13.clone(), v3.clone());
                let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let mut v16: Rc<str> = job_file_18(v1.clone(), v15.clone());
                let mut v17: Rc<str> = job_file_18(v1.clone(), v12.clone());
                let mut v18: Rc<str> = eoie_machine_windows::eoie_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let mut v19: Rc<str> = text_written_19(v16.clone(), v18.clone());
                let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) };
                let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
                v21.clone()
            }
        }
        US0::US0_PlainStart => {
            v3.clone()
        }
    }
}
fn environment_snapshot_21() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0.clone()
}
fn get_environment_variable_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1.clone()
}
fn job_launched_14(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: u64, mut v12: Rc<str>, mut v13: Rc<str>) -> US4 {
    let mut v14: u64 = now_ms_15();
    let mut v15: Rc<str> = decimal_text_16(v14);
    let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
    let mut v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let mut v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let mut v20: Rc<str> = std::rc::Rc::<str>::from(v10.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let mut v22: Rc<str> = job_directory_6(v21.clone());
    let mut v23: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v22).err().map(|error| format!("create {}: {error}", &*v22)).unwrap_or_default());
    let mut v24: bool = v23.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v24 {
        US4::US4_LaunchRefused(v23.clone())
    } else {
        let mut v26: Rc<str> = job_argv_17(v0.clone(), v21.clone(), v10.clone(), v12.clone(), v11);
        let mut v27: bool = v26.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v27 {
            let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set EOIE_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US4::US4_LaunchRefused(v28.clone())
        } else {
            let mut v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let mut v31: Rc<str> = job_file_18(v21.clone(), v30.clone());
            let mut v32: Rc<str> = text_written_19(v31.clone(), v10.clone());
            let mut v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v34: Rc<str> = job_file_18(v21.clone(), v33.clone());
            let mut v35: Rc<str> = decimal_text_16(v11);
            let mut v36: Rc<str> = text_written_19(v34.clone(), v35.clone());
            let mut v37: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v36));
            let mut v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let mut v39: Rc<str> = job_file_18(v21.clone(), v38.clone());
            let mut v40: Rc<str> = text_written_19(v39.clone(), v13.clone());
            let mut v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v40));
            let mut v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let mut v43: Rc<str> = job_file_18(v21.clone(), v42.clone());
            let mut v44: Rc<str> = text_written_19(v43.clone(), v26.clone());
            let mut v45: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v44));
            let mut v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let mut v47: Rc<str> = job_file_18(v21.clone(), v46.clone());
            let mut v48: u64 = now_ms_15();
            let mut v49: Rc<str> = decimal_text_16(v48);
            let mut v50: Rc<str> = text_written_19(v47.clone(), v49.clone());
            let mut v51: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v50));
            let mut v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let mut v53: Rc<str> = job_file_18(v21.clone(), v52.clone());
            let mut v54: Rc<str> = environment_snapshot_21();
            let mut v55: Rc<str> = text_written_19(v53.clone(), v54.clone());
            let mut v56: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v55));
            let mut v57: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v65: Rc<str> = if v57 {
                let mut v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v58.clone()
            } else {
                let mut v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let mut v60: Rc<str> = job_file_18(v21.clone(), v59.clone());
                let mut v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let mut v62: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v61));
                let mut v63: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v1));
                text_written_19(v60.clone(), v63.clone())
            };
            let mut v66: bool = v4 == 0u64;
            let mut v72: Rc<str> = if v66 {
                let mut v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v67.clone()
            } else {
                let mut v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let mut v69: Rc<str> = job_file_18(v21.clone(), v68.clone());
                let mut v70: Rc<str> = decimal_text_16(v4);
                text_written_19(v69.clone(), v70.clone())
            };
            let mut v73: bool = v5 == 0u64;
            let mut v78: u64 = if v73 {
                v6
            } else {
                let mut v74: bool = v6 == 0u64;
                let mut v76: bool = if v74 {
                    true
                } else {
                    let mut v75: bool = v5 < v6;
                    v75
                };
                if v76 {
                    v5
                } else {
                    v6
                }
            };
            let mut v79: bool = v78 == 0u64;
            let mut v88: Rc<str> = if v79 {
                let mut v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v80.clone()
            } else {
                let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let mut v82: Rc<str> = job_file_18(v21.clone(), v81.clone());
                let mut v83: u64 = now_ms_15();
                let mut v84: u64 = v78.wrapping_mul(60000u64);
                let mut v85: u64 = v83.wrapping_add(v84);
                let mut v86: Rc<str> = decimal_text_16(v85);
                text_written_19(v82.clone(), v86.clone())
            };
            let mut v89: bool = v6 == 0u64;
            let mut v98: Rc<str> = if v89 {
                let mut v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v90.clone()
            } else {
                let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let mut v92: Rc<str> = job_file_18(v21.clone(), v91.clone());
                let mut v93: u64 = now_ms_15();
                let mut v94: u64 = v6.wrapping_mul(60000u64);
                let mut v95: u64 = v93.wrapping_add(v94);
                let mut v96: Rc<str> = decimal_text_16(v95);
                text_written_19(v92.clone(), v96.clone())
            };
            let mut v99: bool = v7 == 0u64;
            let mut v110: Rc<str> = if v99 {
                let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v100.clone()
            } else {
                let mut v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let mut v102: Rc<str> = job_file_18(v21.clone(), v101.clone());
                let mut v103: Rc<str> = decimal_text_16(v7);
                let mut v104: Rc<str> = text_written_19(v102.clone(), v103.clone());
                let mut v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let mut v106: Rc<str> = job_file_18(v21.clone(), v105.clone());
                let mut v107: Rc<str> = decimal_text_16(v8);
                let mut v108: Rc<str> = text_written_19(v106.clone(), v107.clone());
                let mut v109: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v108));
                v109.clone()
            };
            let mut v111: bool = v9 == 0u64;
            let mut v117: Rc<str> = if v111 {
                let mut v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v112.clone()
            } else {
                let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
                let mut v114: Rc<str> = job_file_18(v21.clone(), v113.clone());
                let mut v115: Rc<str> = decimal_text_16(v9);
                text_written_19(v114.clone(), v115.clone())
            };
            let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("EOIE_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let mut v119: Rc<str> = get_environment_variable_22(v118.clone());
            let mut v120: bool = v119.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v125: Rc<str> = if v120 {
                let mut v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v121.clone()
            } else {
                let mut v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let mut v123: Rc<str> = job_file_18(v21.clone(), v122.clone());
                text_written_19(v123.clone(), v119.clone())
            };
            let mut v126: bool = v56.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v128: bool = if v126 {
                true
            } else {
                let mut v127: bool = v65.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v127
            };
            let mut v130: bool = if v128 {
                true
            } else {
                let mut v129: bool = v72.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v129
            };
            let mut v132: bool = if v130 {
                true
            } else {
                let mut v131: bool = v88.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v131
            };
            let mut v134: bool = if v132 {
                true
            } else {
                let mut v133: bool = v98.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v133
            };
            let mut v136: bool = if v134 {
                true
            } else {
                let mut v135: bool = v110.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v135
            };
            let mut v138: bool = if v136 {
                true
            } else {
                let mut v137: bool = v117.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v137
            };
            let mut v140: bool = if v138 {
                true
            } else {
                let mut v139: bool = v125.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v139
            };
            if v140 {
                let mut v141: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v56));
                let mut v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v65));
                let mut v143: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v72));
                let mut v144: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v88));
                let mut v145: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v110));
                let mut v146: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v125));
                let mut v147: Rc<str> = Rc::<str>::from(format!("{}{}", v144, v146));
                US4::US4_LaunchRefused(v147.clone())
            } else {
                let mut v151: Rc<str> = if v3 {
                    let mut v149: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v149.clone()
                } else {
                    let mut v150: Rc<str> = eoie_job_host::eoie_job_host_request(&*v21);
                    v150.clone()
                };
                let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let mut v153: bool = v151.starts_with(&*v152);
                if v153 {
                    US4::US4_LaunchRefused(v151.clone())
                } else {
                    US4::US4_Launched(v21.clone(), v151.clone())
                }
            }
        }
    }
}
fn state_of_parts_26(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> US6 {
    let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pending"); } LIT.with(|lit| lit.clone()) };
    if v3 {
        let mut v4: US7 = US7::US7_JobPending;
        US6::US6_Live(v4.clone())
    } else {
        let mut v6: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
        if v6 {
            let mut v7: US7 = US7::US7_JobRunning;
            US6::US6_Live(v7.clone())
        } else {
            let mut v9: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished"); } LIT.with(|lit| lit.clone()) };
            if v9 {
                let mut v10: US8 = US8::US8_JobFinished(v1);
                US6::US6_Settled(v10.clone())
            } else {
                let mut v12: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed-out"); } LIT.with(|lit| lit.clone()) };
                if v12 {
                    let mut v13: US8 = US8::US8_JobTimedOut;
                    US6::US6_Settled(v13.clone())
                } else {
                    let mut v15: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stopped"); } LIT.with(|lit| lit.clone()) };
                    if v15 {
                        let mut v16: US8 = US8::US8_JobStopped;
                        US6::US6_Settled(v16.clone())
                    } else {
                        let mut v18: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lost"); } LIT.with(|lit| lit.clone()) };
                        if v18 {
                            let mut v19: US8 = US8::US8_JobLost;
                            US6::US6_Settled(v19.clone())
                        } else {
                            let mut v21: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory-capped"); } LIT.with(|lit| lit.clone()) };
                            if v21 {
                                let mut v22: US8 = US8::US8_JobMemoryCapped;
                                US6::US6_Settled(v22.clone())
                            } else {
                                let mut v24: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired-in-queue"); } LIT.with(|lit| lit.clone()) };
                                if v24 {
                                    let mut v25: US8 = US8::US8_JobExpiredInQueue;
                                    US6::US6_Settled(v25.clone())
                                } else {
                                    let mut v27: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("skipped"); } LIT.with(|lit| lit.clone()) };
                                    if v27 {
                                        let mut v28: US8 = US8::US8_JobSkipped(v2.clone());
                                        US6::US6_Settled(v28.clone())
                                    } else {
                                        let mut v30: US8 = US8::US8_JobSpawnFailed(v2.clone());
                                        US6::US6_Settled(v30.clone())
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
fn shared_state_25(mut v0: Rc<str>) -> US6 {
    let (mut v1, mut v2, mut v3): (Rc<str>, i32, Rc<str>) = eoie_job_state::eoie_job_state_parts(&*v0);
    state_of_parts_26(v1.clone(), v2, v3.clone())
}
fn settled_within_24(mut v0: Rc<str>, mut v1: u64) -> US6 {
    loop {
        let mut v2: US6 = shared_state_25(v0.clone());
        match &v2 {
            US6::US6_Live(v3) => {
                let mut v3: US7 = v3.clone();
                let mut v4: bool = v1 == 0u64;
                if v4 {
                    return v2.clone();
                } else {
                    let mut v5: u64 = 100u64;
                    std::thread::sleep(std::time::Duration::from_millis(v5));
                    let mut v6: bool = v1 < 100u64;
                    let mut v8: u64 = if v6 {
                        0u64
                    } else {
                        let mut v7: u64 = v1.wrapping_sub(100u64);
                        v7
                    };
                    (v0, v1) = (v0.clone(), v8);
                    continue;
                }
            }
            US6::US6_Settled(v11) => {
                let mut v11: US8 = v11.clone();
                return v2.clone();
            }
        }
    }
}
fn text_or_empty_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(&*v0).unwrap_or_default());
    v1.clone()
}
fn method28(mut v0: Option<usize>) -> Option<usize> {
    v0.clone()
}
fn closure2() -> Rc<dyn Fn((usize)) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((usize)) -> i32> = Rc::new(move |mut v0: (usize)| -> i32 {
        let mut v1: usize = (v0);
        let mut v2: i32 = ((v1) as i32);
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method29(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let mut v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                        let mut v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method30(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let mut v3: i32 = v1.wrapping_sub(1i32);
            let mut v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let mut v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
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
                        let mut v8: bool = v4 == b'\n';
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
fn start_settle_of_23(mut v0: Rc<str>, mut v1: u64) -> US5 {
    let mut v2: US6 = settled_within_24(v0.clone(), v1);
    match &v2 {
        US6::US6_Settled(v3) => {
            let mut v3: US8 = v3.clone();
            match &v3 {
                US8::US8_JobFinished(v4) => {
                    let mut v4: i32 = *v4;
                    let mut v5: bool = v4 == 0i32;
                    if v5 {
                        US5::US5_SettleHealthy
                    } else {
                        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stderr.log"); } LIT.with(|lit| lit.clone()) };
                        let mut v8: Rc<str> = job_file_18(v0.clone(), v7.clone());
                        let mut v9: Rc<str> = text_or_empty_27(v8.clone());
                        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                        let mut v11: Option<usize> = v9.find(&*v10);
                        let mut v12: Option<usize> = method28(v11.clone());
                        let mut v13: Rc<dyn Fn((usize)) -> i32> = closure2();
                        let mut v14: Option<i32> = v12.map(|x| v13(x));
                        let mut v16: i32 = v14.unwrap_or(-1i32);
                        let mut v17: bool = v16 < 0i32;
                        let mut v21: Rc<str> = if v17 {
                            v9.clone()
                        } else {
                            let mut v18: usize = ((0i32) as usize);
                            let mut v19: usize = ((v16) as usize);
                            let mut v20: Rc<str> = Rc::<str>::from(&(v9)[v18..v19]);
                            v20.clone()
                        };
                        let mut v22: i32 = (v21.clone().len() as i32);
                        let mut v23: i32 = 0i32;
                        let mut v24: i32 = method29(v21.clone(), v22, v23);
                        let mut v25: i32 = v22.wrapping_sub(1i32);
                        let mut v26: Rc<str> = string_slice(&v21.clone(), v24 as i64, v25 as i64);
                        let mut v27: i32 = (v26.clone().len() as i32);
                        let mut v28: i32 = method30(v26.clone(), v27);
                        let mut v29: Rc<str> = string_slice(&v26.clone(), 0i32 as i64, v28 as i64);
                        US5::US5_SettleFailed(v4, v29.clone())
                    }
                }
                US8::US8_JobLost => {
                    let mut v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the supervisor ended before the job settled"); } LIT.with(|lit| lit.clone()) };
                    US5::US5_SettleFailed(125i32, v34.clone())
                }
                US8::US8_JobSpawnFailed(v32) => {
                    let mut v32: Rc<str> = v32.clone();
                    US5::US5_SettleFailed(127i32, v32.clone())
                }
                _ => {
                    US5::US5_SettleHealthy
                }
            }
        }
        _ => {
            US5::US5_SettleHealthy
        }
    }
}
fn method32(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3.clone();
    ()
}
fn format_real_31(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method32(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn stdout_line_33(mut v0: Rc<str>) -> () {
    println!("{}", &*v0);
    ()
}
fn job_started_1(mut v0: bool) -> (i32, Rc<str>) {
    let mut v1: i32 = 3i32;
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v1 as usize).unwrap_or_default());
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after"); } LIT.with(|lit| lit.clone()) };
    let mut v9: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let mut v5: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after-ok"); } LIT.with(|lit| lit.clone()) };
        if v5 {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v6.clone()
        } else {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        }
    };
    let mut v10: bool = v9.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = if v10 {
        let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v11.clone()
    } else {
        let mut v12: i32 = 4i32;
        let mut v13: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v12 as usize).unwrap_or_default());
        v13.clone()
    };
    let mut v15: i32 = if v10 {
        3i32
    } else {
        5i32
    };
    let mut v16: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v15 as usize).unwrap_or_default());
    let mut v17: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy"); } LIT.with(|lit| lit.clone()) };
    let mut v23: US0 = if v17 {
        US0::US0_HeavyStart
    } else {
        let mut v19: bool = v16.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--compile"); } LIT.with(|lit| lit.clone()) };
        if v19 {
            US0::US0_CompileStart
        } else {
            US0::US0_PlainStart
        }
    };
    let mut v26: i32 = match &v23 {
        US0::US0_CompileStart => {
            1i32
        }
        US0::US0_HeavyStart => {
            1i32
        }
        US0::US0_PlainStart => {
            0i32
        }
    };
    let mut v27: i32 = v15.wrapping_add(v26);
    let mut v28: bool = false;
    let mut v29: u64 = 0u64;
    let mut v30: bool = false;
    let mut v31: u64 = 0u64;
    let mut v32: bool = false;
    let mut v33: u64 = 0u64;
    let mut v34: bool = false;
    let mut v35: bool = false;
    let mut v36: u64 = 0u64;
    let mut v37: bool = false;
    let mut v38: u64 = 0u64;
    let mut v39: bool = false;
    let mut v40: u64 = 0u64;
    let mut v41: bool = false;
    let mut v42: u64 = 0u64;
    let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (i32, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = start_options_read_2(v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42);
    let mut v60: u64 = if v51 {
        v52
    } else {
        if v50 {
            30u64
        } else {
            0u64
        }
    };
    let mut v62: u64 = if v53 {
        v54
    } else {
        if v50 {
            10u64
        } else {
            0u64
        }
    };
    let mut v68: bool = if v10 {
        true
    } else {
        let mut v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let mut v64: bool = v14.starts_with(&*v63);
        if v64 {
            let mut v65: Rc<str> = job_directory_6(v14.clone());
            directory_exists_9(v65.clone())
        } else {
            false
        }
    };
    let mut v70: Rc<str> = if v68 {
        let mut v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v69.clone()
    } else {
        v14.clone()
    };
    let mut v71: i32 = argument_count_10();
    let mut v72: i32 = v43.wrapping_add(3i32);
    let mut v73: bool = v71 < v72;
    if v73 {
        let mut v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits eoie processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
        let mut v75: i32 = rejected_12(v74.clone());
        let mut v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v75, v76.clone())
    } else {
        let mut v77: bool = v70.clone() != { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v77 {
            let mut v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after names no job: "); } LIT.with(|lit| lit.clone()) };
            let mut v79: Rc<str> = Rc::<str>::from(format!("{}{}", v78, v70));
            let mut v80: i32 = rejected_12(v79.clone());
            let mut v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v80, v81.clone())
        } else {
            let mut v83: bool = if v44 {
                let mut v82: bool = v45 == 0u64;
                v82
            } else {
                false
            };
            if v83 {
                let mut v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                let mut v85: i32 = rejected_12(v84.clone());
                let mut v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                (v85, v86.clone())
            } else {
                let mut v88: bool = if v46 {
                    let mut v87: bool = v47 == 0u64;
                    v87
                } else {
                    false
                };
                if v88 {
                    let mut v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                    let mut v90: i32 = rejected_12(v89.clone());
                    let mut v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    (v90, v91.clone())
                } else {
                    let mut v93: bool = if v48 {
                        let mut v92: bool = v49 == 0u64;
                        v92
                    } else {
                        false
                    };
                    if v93 {
                        let mut v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                        let mut v95: i32 = rejected_12(v94.clone());
                        let mut v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        (v95, v96.clone())
                    } else {
                        let mut v97: bool = v51 || v53;
                        let mut v99: bool = if v97 {
                            let mut v98: bool = v50 == false;
                            v98
                        } else {
                            false
                        };
                        if v99 {
                            let mut v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold and --quiet-wait follow --quiet"); } LIT.with(|lit| lit.clone()) };
                            let mut v101: i32 = rejected_12(v100.clone());
                            let mut v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            (v101, v102.clone())
                        } else {
                            let mut v105: bool = if v50 {
                                let mut v103: bool = match &v23 {
                                    US0::US0_HeavyStart => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                let mut v104: bool = v103 != true;
                                v104
                            } else {
                                false
                            };
                            if v105 {
                                let mut v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet applies to --heavy jobs: it holds compiles off while the heavy work starts"); } LIT.with(|lit| lit.clone()) };
                                let mut v107: i32 = rejected_12(v106.clone());
                                let mut v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                (v107, v108.clone())
                            } else {
                                let mut v112: bool = if v50 {
                                    let mut v109: bool = v60 == 0u64;
                                    if v109 {
                                        true
                                    } else {
                                        let mut v110: bool = v60 > 60u64;
                                        v110
                                    }
                                } else {
                                    false
                                };
                                if v112 {
                                    let mut v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold takes 1 to 60 minutes (compiles wait at most that long)"); } LIT.with(|lit| lit.clone()) };
                                    let mut v114: i32 = rejected_12(v113.clone());
                                    let mut v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    (v114, v115.clone())
                                } else {
                                    let mut v117: bool = if v46 {
                                        match &v23 {
                                            US0::US0_PlainStart => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }
                                    } else {
                                        false
                                    };
                                    if v117 {
                                        let mut v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within applies to --heavy or --compile jobs, which wait for a slot or window"); } LIT.with(|lit| lit.clone()) };
                                        let mut v119: i32 = rejected_12(v118.clone());
                                        let mut v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                        (v119, v120.clone())
                                    } else {
                                        let mut v122: bool = if v55 {
                                            let mut v121: bool = v56 == 0u64;
                                            v121
                                        } else {
                                            false
                                        };
                                        if v122 {
                                            let mut v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                                            let mut v124: i32 = rejected_12(v123.clone());
                                            let mut v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                            (v124, v125.clone())
                                        } else {
                                            let mut v127: bool = if v55 {
                                                match &v23 {
                                                    US0::US0_PlainStart => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                }
                                            } else {
                                                false
                                            };
                                            if v127 {
                                                let mut v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb applies to --heavy or --compile jobs, which wait for a window"); } LIT.with(|lit| lit.clone()) };
                                                let mut v129: i32 = rejected_12(v128.clone());
                                                let mut v130: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                (v129, v130.clone())
                                            } else {
                                                let mut v131: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v43 as usize).unwrap_or_default());
                                                let mut v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                                let mut v133: bool = v131.starts_with(&*v132);
                                                if v133 {
                                                    let mut v134: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v43 as usize).unwrap_or_default());
                                                    let mut v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v136: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v134));
                                                    let mut v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                                                    let mut v138: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v137));
                                                    let mut v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits eoie processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v140: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v139));
                                                    let mut v141: i32 = rejected_12(v140.clone());
                                                    let mut v142: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                    (v141, v142.clone())
                                                } else {
                                                    let mut v143: i32 = v43.wrapping_add(2i32);
                                                    let mut v144: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v143 as usize).unwrap_or_default());
                                                    let mut v145: bool = v144.starts_with(&*v132);
                                                    if v145 {
                                                        let mut v146: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v143 as usize).unwrap_or_default());
                                                        let mut v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the program cannot start with --, got "); } LIT.with(|lit| lit.clone()) };
                                                        let mut v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v146));
                                                        let mut v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; options go before the label"); } LIT.with(|lit| lit.clone()) };
                                                        let mut v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                                                        let mut v151: i32 = rejected_12(v150.clone());
                                                        let mut v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                        (v151, v152.clone())
                                                    } else {
                                                        let mut v153: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v43 as usize).unwrap_or_default());
                                                        let mut v154: i32 = v43.wrapping_add(1i32);
                                                        let mut v155: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v154 as usize).unwrap_or_default());
                                                        let mut v156: u64 = decimal_u64_4(v155.clone());
                                                        let mut v157: bool = v156 == 0u64;
                                                        if v157 {
                                                            let mut v158: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
                                                            let mut v159: i32 = rejected_12(v158.clone());
                                                            let mut v160: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                            (v159, v160.clone())
                                                        } else {
                                                            let mut v161: Rc<str> = std::rc::Rc::<str>::from(std::env::args().skip(v143 as usize).collect::<Vec<_>>().join("
"));
                                                            let mut v162: Rc<str> = std::rc::Rc::<str>::from(std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
                                                            let mut v163: US4 = job_launched_14(v23.clone(), v9.clone(), v14.clone(), v0, v45, v47, v49, v60, v62, v56, v153.clone(), v156, v161.clone(), v162.clone());
                                                            match &v163 {
                                                                US4::US4_LaunchRefused(v164) => {
                                                                    let mut v164: Rc<str> = v164.clone();
                                                                    let mut v165: i32 = rejected_12(v164.clone());
                                                                    let mut v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                    (v165, v166.clone())
                                                                }
                                                                US4::US4_Launched(v167, v168) => {
                                                                    let mut v167: Rc<str> = v167.clone();
                                                                    let mut v168: Rc<str> = v168.clone();
                                                                    let mut v169: u64 = if v57 {
                                                                        v58
                                                                    } else {
                                                                        2000u64
                                                                    };
                                                                    let mut v170: bool = v0 == false;
                                                                    let mut v171: bool = v170 && v10;
                                                                    let mut v173: bool = if v171 {
                                                                        match &v23 {
                                                                            US0::US0_PlainStart => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    } else {
                                                                        false
                                                                    };
                                                                    let mut v175: bool = if v173 {
                                                                        let mut v174: bool = v169 > 0u64;
                                                                        v174
                                                                    } else {
                                                                        false
                                                                    };
                                                                    let mut v178: US5 = if v175 {
                                                                        start_settle_of_23(v167.clone(), v169)
                                                                    } else {
                                                                        US5::US5_SettleHealthy
                                                                    };
                                                                    match &v178 {
                                                                        US5::US5_SettleFailed(v179, v180) => {
                                                                            let mut v179: i32 = *v179;
                                                                            let mut v180: Rc<str> = v180.clone();
                                                                            let mut v181: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job start failed id="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v182: Rc<str> = Rc::<str>::from(format!("{}{}", v181, v167));
                                                                            let mut v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" exit="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v184: Rc<str> = Rc::<str>::from(format!("{}{}", v182, v183));
                                                                            let mut v185: Rc<str> = format_real_31(v179);
                                                                            let mut v186: Rc<str> = Rc::<str>::from(format!("{}{}", v184, v185));
                                                                            let mut v187: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stderr="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v188: Rc<str> = Rc::<str>::from(format!("{}{}", v186, v187));
                                                                            let mut v189: Rc<str> = Rc::<str>::from(format!("{}{}", v188, v180));
                                                                            stderr_line_13(v189.clone());
                                                                            (v179, v167.clone())
                                                                        }
                                                                        US5::US5_SettleHealthy => {
                                                                            let mut v192: Rc<str> = if v0 {
                                                                                let mut v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job run started id="); } LIT.with(|lit| lit.clone()) };
                                                                                v190.clone()
                                                                            } else {
                                                                                let mut v191: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("eoie job start ok id="); } LIT.with(|lit| lit.clone()) };
                                                                                v191.clone()
                                                                            };
                                                                            let mut v193: Rc<str> = Rc::<str>::from(format!("{}{}", v192, v167));
                                                                            let mut v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" supervisor_pid="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v195: Rc<str> = Rc::<str>::from(format!("{}{}", v193, v194));
                                                                            let mut v196: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v168));
                                                                            let mut v197: Rc<str> = decimal_text_16(v156);
                                                                            let mut v198: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" timeout_ms="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v199: Rc<str> = Rc::<str>::from(format!("{}{}", v198, v197));
                                                                            let mut v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" gate="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v201: Rc<str> = Rc::<str>::from(format!("{}{}", v199, v200));
                                                                            let mut v207: Rc<str> = match &v23 {
                                                                                US0::US0_CompileStart => {
                                                                                    let mut v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                                                                    v204.clone()
                                                                                }
                                                                                US0::US0_HeavyStart => {
                                                                                    let mut v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                                                                    v203.clone()
                                                                                }
                                                                                US0::US0_PlainStart => {
                                                                                    let mut v202: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                                                                    v202.clone()
                                                                                }
                                                                            };
                                                                            let mut v208: Rc<str> = Rc::<str>::from(format!("{}{}", v201, v207));
                                                                            let mut v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" label="); } LIT.with(|lit| lit.clone()) };
                                                                            let mut v210: Rc<str> = Rc::<str>::from(format!("{}{}", v209, v153));
                                                                            let mut v217: Rc<str> = if v10 {
                                                                                let mut v211: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                v211.clone()
                                                                            } else {
                                                                                let mut v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v213: Rc<str> = Rc::<str>::from(format!("{}{}", v212, v14));
                                                                                let mut v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" after_mode="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v215: Rc<str> = Rc::<str>::from(format!("{}{}", v213, v214));
                                                                                let mut v216: Rc<str> = Rc::<str>::from(format!("{}{}", v215, v9));
                                                                                v216.clone()
                                                                            };
                                                                            let mut v218: Rc<str> = Rc::<str>::from(format!("{}{}", v210, v217));
                                                                            let mut v219: bool = v45 == 0u64;
                                                                            let mut v224: Rc<str> = if v219 {
                                                                                let mut v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                v220.clone()
                                                                            } else {
                                                                                let mut v221: Rc<str> = decimal_text_16(v45);
                                                                                let mut v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" max_private_mb="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v223: Rc<str> = Rc::<str>::from(format!("{}{}", v222, v221));
                                                                                v223.clone()
                                                                            };
                                                                            let mut v225: Rc<str> = Rc::<str>::from(format!("{}{}", v218, v224));
                                                                            let mut v226: bool = v47 == 0u64;
                                                                            let mut v231: Rc<str> = if v226 {
                                                                                let mut v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                v227.clone()
                                                                            } else {
                                                                                let mut v228: Rc<str> = decimal_text_16(v47);
                                                                                let mut v229: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" slot_within_min="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v230: Rc<str> = Rc::<str>::from(format!("{}{}", v229, v228));
                                                                                v230.clone()
                                                                            };
                                                                            let mut v232: bool = v49 == 0u64;
                                                                            let mut v237: Rc<str> = if v232 {
                                                                                let mut v233: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                v233.clone()
                                                                            } else {
                                                                                let mut v234: Rc<str> = decimal_text_16(v49);
                                                                                let mut v235: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" done_within_min="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v236: Rc<str> = Rc::<str>::from(format!("{}{}", v235, v234));
                                                                                v236.clone()
                                                                            };
                                                                            let mut v238: Rc<str> = Rc::<str>::from(format!("{}{}", v231, v237));
                                                                            let mut v239: bool = v60 == 0u64;
                                                                            let mut v248: Rc<str> = if v239 {
                                                                                let mut v240: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                                                                v240.clone()
                                                                            } else {
                                                                                let mut v241: Rc<str> = decimal_text_16(v60);
                                                                                let mut v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_hold_min="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v243: Rc<str> = Rc::<str>::from(format!("{}{}", v242, v241));
                                                                                let mut v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" quiet_wait_min="); } LIT.with(|lit| lit.clone()) };
                                                                                let mut v245: Rc<str> = Rc::<str>::from(format!("{}{}", v243, v244));
                                                                                let mut v246: Rc<str> = decimal_text_16(v62);
                                                                                let mut v247: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v246));
                                                                                v247.clone()
                                                                            };
                                                                            let mut v249: Rc<str> = Rc::<str>::from(format!("{}{}", v238, v248));
                                                                            let mut v250: Rc<str> = Rc::<str>::from(format!("{}{}", v225, v249));
                                                                            let mut v251: Rc<str> = Rc::<str>::from(format!("{}{}", v208, v250));
                                                                            let mut v252: Rc<str> = Rc::<str>::from(format!("{}{}", v196, v251));
                                                                            stdout_line_33(v252.clone());
                                                                            if v0 {
                                                                                let mut v253: i32 = eoie_job_supervise::eoie_job_supervise(&*v167);
                                                                                (v253, v167.clone())
                                                                            } else {
                                                                                (0i32, v167.clone())
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
                            }
                        }
                    }
                }
            }
        }
    }
}
fn started_text_0(mut v0: bool) -> Rc<str> {
    let (mut v1, mut v2): (i32, Rc<str>) = job_started_1(v0);
    let mut v3: Rc<str> = std::rc::Rc::<str>::from((v1).to_string().as_str());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    v6.clone()
}
fn closure0() -> Rc<dyn Fn(bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool) -> Rc<str>> = Rc::new(move |mut v0: bool| -> Rc<str> {
        started_text_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn launched_text_34(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: US0 = US0::US0_PlainStart;
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: bool = false;
    let mut v7: u64 = 0u64;
    let mut v8: u64 = 0u64;
    let mut v9: u64 = 0u64;
    let mut v10: u64 = 0u64;
    let mut v11: u64 = 0u64;
    let mut v12: u64 = 0u64;
    let mut v13: US4 = job_launched_14(v4.clone(), v5.clone(), v5.clone(), v6, v7, v8, v9, v10, v11, v12, v0.clone(), v1, v2.clone(), v3.clone());
    match &v13 {
        US4::US4_LaunchRefused(v19) => {
            let mut v19: Rc<str> = v19.clone();
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v19));
            v21.clone()
        }
        US4::US4_Launched(v14, v15) => {
            let mut v14: Rc<str> = v14.clone();
            let mut v15: Rc<str> = v15.clone();
            let mut v16: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
            let mut v17: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v16));
            let mut v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v15));
            v18.clone()
        }
    }
}
fn closure3() -> Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>| -> Rc<str> {
        launched_text_34(v0.clone(), v1, v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_job_start(v0: bool) -> Rc<str> {
    closure0()(v0)
}
pub fn eoie_job_launched_text(v0: &str, v1: u64, v2: &str, v3: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2), Rc::<str>::from(v3))
}
