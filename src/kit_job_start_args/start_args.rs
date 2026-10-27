#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
fn start_flag_of_2(mut v0: Rc<str>) -> US1 {
    let v1: bool = &*v0 == "--max-private-mb";
    if v1 {
        let v2: US2 = US2::US2_MaxPrivateMb;
        US1::US1_StartFlagKnown(v2)
    } else {
        let mut v4: bool = &*v0 == "--slot-within";
        if v4 {
            let v5: US2 = US2::US2_SlotWithin;
            US1::US1_StartFlagKnown(v5)
        } else {
            let v7: bool = &*v0 == "--done-within";
            if v7 {
                let v8: US2 = US2::US2_DoneWithin;
                US1::US1_StartFlagKnown(v8)
            } else {
                let v10: bool = &*v0 == "--quiet";
                if v10 {
                    let v11: US2 = US2::US2_QuietStart;
                    US1::US1_StartFlagKnown(v11)
                } else {
                    let v13: bool = &*v0 == "--quiet-hold";
                    if v13 {
                        let v14: US2 = US2::US2_QuietHold;
                        US1::US1_StartFlagKnown(v14)
                    } else {
                        let v16: bool = &*v0 == "--quiet-wait";
                        if v16 {
                            let v17: US2 = US2::US2_QuietWait;
                            US1::US1_StartFlagKnown(v17)
                        } else {
                            let v19: bool = &*v0 == "--min-free-mb";
                            if v19 {
                                let v20: US2 = US2::US2_MinFreeMb;
                                US1::US1_StartFlagKnown(v20)
                            } else {
                                let v22: bool = &*v0 == "--settle-ms";
                                if v22 {
                                    let v23: US2 = US2::US2_SettleMs;
                                    US1::US1_StartFlagKnown(v23)
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
fn method4(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_3(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method4(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
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
fn flags_read_1(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: bool, mut v8: u64, mut v9: bool, mut v10: u64, mut v11: bool, mut v12: bool, mut v13: u64, mut v14: bool, mut v15: u64, mut v16: bool, mut v17: u64, mut v18: bool, mut v19: u64) -> (i32, Rc<str>, Rc<str>, US0, Rc<str>, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) {
    loop {
        let v20: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let v21: bool = &*v20 == "--after";
        let v27: Rc<str> = if v21 {
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
            v22.clone()
        } else {
            let v23: bool = &*v20 == "--after-ok";
            if v23 {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            } else {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v25.clone()
            }
        };
        let v28: bool = &*v20 == "--heavy";
        let v34: US0 = if v28 {
            US0::US0_HeavyStart
        } else {
            let v30: bool = &*v20 == "--compile";
            if v30 {
                US0::US0_CompileStart
            } else {
                US0::US0_PlainStart
            }
        };
        let v35: bool = &*v27 != "";
        if v35 {
            let v36: bool = &*v1 != "";
            if v36 {
                return (v0, v1.clone(), v2.clone(), v3.clone(), v20.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
            } else {
                let v37: i32 = v0.wrapping_add(2i32);
                let v38: i32 = v0.wrapping_add(1i32);
                let v39: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v38 as usize).unwrap_or_default());
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = (v37, v27.clone(), v39.clone(), v3.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
                continue;
            }
        } else {
            let v80: bool = match &v34 {
                US0::US0_PlainStart => {
                    true
                }
                _ => {
                    false
                }
            };
            let v81: bool = v80 != true;
            if v81 {
                let v82: bool = match &v3 {
                    US0::US0_PlainStart => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v83: bool = v82 != true;
                if v83 {
                    return (v0, v1.clone(), v2.clone(), v3.clone(), v20.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
                } else {
                    let v86: i32 = match &v34 {
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
                    let v87: i32 = v0.wrapping_add(v86);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = (v87, v1.clone(), v2.clone(), v34.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
                    continue;
                }
            } else {
                let v128: US1 = start_flag_of_2(v20.clone());
                match &v128 {
                    US1::US1_StartFlagKnown(v129) => {
                        let v129: US2 = v129.clone();
                        let v130: i32 = match &v129 {
                            US2::US2_QuietStart => {
                                1i32
                            }
                            _ => {
                                2i32
                            }
                        };
                        let v131: bool = v130 == 2i32;
                        let v135: u64 = if v131 {
                            let v132: i32 = v0.wrapping_add(1i32);
                            let v133: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v132 as usize).unwrap_or_default());
                            decimal_u64_3(v133.clone())
                        } else {
                            0u64
                        };
                        let v136: i32 = v0.wrapping_add(v130);
                        let (mut v242, mut v243, mut v244, mut v245, mut v246, mut v247, mut v248, mut v249, mut v250, mut v251, mut v252, mut v253, mut v254, mut v255, mut v256): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                            US2::US2_MaxPrivateMb => {
                                (true, v135, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
                            }
                            _ => {
                                let (mut v227, mut v228, mut v229, mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240, mut v241): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                    US2::US2_SlotWithin => {
                                        (v5, v6, true, v135, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
                                    }
                                    _ => {
                                        let (mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220, mut v221, mut v222, mut v223, mut v224, mut v225, mut v226): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                            US2::US2_DoneWithin => {
                                                (v5, v6, v7, v8, true, v135, v11, v12, v13, v14, v15, v16, v17, v18, v19)
                                            }
                                            _ => {
                                                let (mut v197, mut v198, mut v199, mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206, mut v207, mut v208, mut v209, mut v210, mut v211): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                                    US2::US2_QuietStart => {
                                                        (v5, v6, v7, v8, v9, v10, true, v12, v13, v14, v15, v16, v17, v18, v19)
                                                    }
                                                    _ => {
                                                        let (mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193, mut v194, mut v195, mut v196): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                                            US2::US2_QuietHold => {
                                                                (v5, v6, v7, v8, v9, v10, v11, true, v135, v14, v15, v16, v17, v18, v19)
                                                            }
                                                            _ => {
                                                                let (mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                                                    US2::US2_QuietWait => {
                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, true, v135, v16, v17, v18, v19)
                                                                    }
                                                                    _ => {
                                                                        let (mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                                                            US2::US2_MinFreeMb => {
                                                                                (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, true, v135, v18, v19)
                                                                            }
                                                                            _ => {
                                                                                let (mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = match &v129 {
                                                                                    US2::US2_SettleMs => {
                                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, true, v135)
                                                                                    }
                                                                                    _ => unreachable!(),
                                                                                };
                                                                                (v137, v138, v139, v140, v141, v142, v143, v144, v145, v146, v147, v148, v149, v150, v151)
                                                                            }
                                                                        };
                                                                        (v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166)
                                                                    }
                                                                };
                                                                (v167, v168, v169, v170, v171, v172, v173, v174, v175, v176, v177, v178, v179, v180, v181)
                                                            }
                                                        };
                                                        (v182, v183, v184, v185, v186, v187, v188, v189, v190, v191, v192, v193, v194, v195, v196)
                                                    }
                                                };
                                                (v197, v198, v199, v200, v201, v202, v203, v204, v205, v206, v207, v208, v209, v210, v211)
                                            }
                                        };
                                        (v212, v213, v214, v215, v216, v217, v218, v219, v220, v221, v222, v223, v224, v225, v226)
                                    }
                                };
                                (v227, v228, v229, v230, v231, v232, v233, v234, v235, v236, v237, v238, v239, v240, v241)
                            }
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = (v136, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v242, v243, v244, v245, v246, v247, v248, v249, v250, v251, v252, v253, v254, v255, v256);
                        continue;
                    }
                    US1::US1_StartFlagUnknown => {
                        return (v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
                    }
                }
            }
        }
    }
}
fn joined_6(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_7() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_7();
    joined_6(v1, v0)
}
fn directory_exists_8(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_10() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_9() -> i32 {
    let v0: i32 = command_line_argument_count_10();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn start_args_0() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64) {
    let v0: i32 = 3i32;
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: US0 = US0::US0_PlainStart;
    let mut v3: bool = false;
    let mut v4: u64 = 0u64;
    let v5: bool = false;
    let mut v6: u64 = 0u64;
    let v7: bool = false;
    let v8: u64 = 0u64;
    let v9: bool = false;
    let v10: bool = false;
    let v11: u64 = 0u64;
    let v12: bool = false;
    let v13: u64 = 0u64;
    let v14: bool = false;
    let v15: u64 = 0u64;
    let v16: bool = false;
    let v17: u64 = 0u64;
    let (mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37): (i32, Rc<str>, Rc<str>, US0, Rc<str>, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64) = flags_read_1(v0, v1.clone(), v1.clone(), v2.clone(), v1.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17);
    let v39: u64 = if v30 {
        v31
    } else {
        if v29 {
            30u64
        } else {
            0u64
        }
    };
    let v41: u64 = if v32 {
        v33
    } else {
        if v29 {
            10u64
        } else {
            0u64
        }
    };
    let v42: bool = &*v19 == "";
    let v48: bool = if v42 {
        true
    } else {
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let v44: bool = v20.starts_with(&*v43);
        if v44 {
            let v45: Rc<str> = job_directory_5(v20.clone());
            directory_exists_8(v45.clone())
        } else {
            false
        }
    };
    let v49: Rc<str> = if v48 {
        v1.clone()
    } else {
        v20.clone()
    };
    let v50: i32 = argument_count_9();
    let v51: i32 = v18.wrapping_add(3i32);
    let v52: bool = v50 < v51;
    if v52 {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
        (false, v53.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
    } else {
        let v54: bool = &*v22 != "";
        if v54 {
            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is given twice (one --after or --after-ok, one --heavy or --compile)"); } LIT.with(|lit| lit.clone()) };
            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v55));
            (false, v56.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
        } else {
            let v57: bool = &*v49 != "";
            if v57 {
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after names no job: "); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v49));
                (false, v59.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
            } else {
                let v61: bool = if v23 {
                    let v60: bool = v24 == 0u64;
                    v60
                } else {
                    false
                };
                if v61 {
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                    (false, v62.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                } else {
                    let v64: bool = if v25 {
                        let v63: bool = v26 == 0u64;
                        v63
                    } else {
                        false
                    };
                    if v64 {
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                        (false, v65.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                    } else {
                        let v67: bool = if v27 {
                            let v66: bool = v28 == 0u64;
                            v66
                        } else {
                            false
                        };
                        if v67 {
                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                            (false, v68.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                        } else {
                            let v69: bool = v30 || v32;
                            let v71: bool = if v69 {
                                let v70: bool = v29 == false;
                                v70
                            } else {
                                false
                            };
                            if v71 {
                                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold and --quiet-wait follow --quiet"); } LIT.with(|lit| lit.clone()) };
                                (false, v72.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                            } else {
                                let v75: bool = if v29 {
                                    let v73: bool = match &v21 {
                                        US0::US0_HeavyStart => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v74: bool = v73 != true;
                                    v74
                                } else {
                                    false
                                };
                                if v75 {
                                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet applies to --heavy jobs: it holds compiles off while the heavy work starts"); } LIT.with(|lit| lit.clone()) };
                                    (false, v76.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                } else {
                                    let v80: bool = if v29 {
                                        let v77: bool = v39 == 0u64;
                                        if v77 {
                                            true
                                        } else {
                                            let v78: bool = v39 > 60u64;
                                            v78
                                        }
                                    } else {
                                        false
                                    };
                                    if v80 {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold takes 1 to 60 minutes (compiles wait at most that long)"); } LIT.with(|lit| lit.clone()) };
                                        (false, v81.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                    } else {
                                        let v83: bool = if v25 {
                                            match &v21 {
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
                                        if v83 {
                                            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within applies to --heavy or --compile jobs, which wait for a slot or window"); } LIT.with(|lit| lit.clone()) };
                                            (false, v84.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                        } else {
                                            let v86: bool = if v34 {
                                                let v85: bool = v35 == 0u64;
                                                v85
                                            } else {
                                                false
                                            };
                                            if v86 {
                                                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                                                (false, v87.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                            } else {
                                                let v89: bool = if v34 {
                                                    match &v21 {
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
                                                if v89 {
                                                    let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb applies to --heavy or --compile jobs, which wait for a window"); } LIT.with(|lit| lit.clone()) };
                                                    (false, v90.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                                } else {
                                                    let v91: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v18 as usize).unwrap_or_default());
                                                    let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                                    let v93: bool = v91.starts_with(&*v92);
                                                    if v93 {
                                                        let v94: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v18 as usize).unwrap_or_default());
                                                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                                        let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v94));
                                                        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                                                        let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                                                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
                                                        let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v99));
                                                        (false, v100.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                                    } else {
                                                        let v101: i32 = v18.wrapping_add(2i32);
                                                        let v102: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v101 as usize).unwrap_or_default());
                                                        let v103: bool = v102.starts_with(&*v92);
                                                        if v103 {
                                                            let v104: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v101 as usize).unwrap_or_default());
                                                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the program cannot start with --, got "); } LIT.with(|lit| lit.clone()) };
                                                            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v104));
                                                            let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; options go before the label"); } LIT.with(|lit| lit.clone()) };
                                                            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v107));
                                                            (false, v108.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                                        } else {
                                                            let v109: i32 = v18.wrapping_add(1i32);
                                                            let v110: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v109 as usize).unwrap_or_default());
                                                            let v111: u64 = decimal_u64_3(v110.clone());
                                                            let v112: bool = v111 == 0u64;
                                                            if v112 {
                                                                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
                                                                (false, v113.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64)
                                                            } else {
                                                                let v114: u64 = if v36 {
                                                                    v37
                                                                } else {
                                                                    2000u64
                                                                };
                                                                let v120: Rc<str> = match &v21 {
                                                                    US0::US0_CompileStart => {
                                                                        let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                                                        v117.clone()
                                                                    }
                                                                    US0::US0_HeavyStart => {
                                                                        let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                                                        v116.clone()
                                                                    }
                                                                    US0::US0_PlainStart => {
                                                                        let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                                                        v115.clone()
                                                                    }
                                                                };
                                                                (true, v1.clone(), v120.clone(), v19.clone(), v20.clone(), v24, v26, v28, v39, v41, v35, v114, v18, v111)
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
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64) {
        start_args_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_start_args() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64) {
    closure0()()
}
