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
    US2_FrontStart,
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
                                    let v25: bool = &*v0 == "--front";
                                    if v25 {
                                        let v26: US2 = US2::US2_FrontStart;
                                        US1::US1_StartFlagKnown(v26)
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
fn flags_read_1(mut v0: i32, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: US0, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: bool, mut v8: u64, mut v9: bool, mut v10: u64, mut v11: bool, mut v12: bool, mut v13: u64, mut v14: bool, mut v15: u64, mut v16: bool, mut v17: u64, mut v18: bool, mut v19: u64, mut v20: bool) -> (i32, Rc<str>, Rc<str>, US0, Rc<str>, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) {
    loop {
        let v21: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v0 as usize).unwrap_or_default());
        let v22: bool = &*v21 == "--after";
        let v28: Rc<str> = if v22 {
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("any"); } LIT.with(|lit| lit.clone()) };
            v23.clone()
        } else {
            let v24: bool = &*v21 == "--after-ok";
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v25.clone()
            } else {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26.clone()
            }
        };
        let v29: bool = &*v21 == "--heavy";
        let v35: US0 = if v29 {
            US0::US0_HeavyStart
        } else {
            let v31: bool = &*v21 == "--compile";
            if v31 {
                US0::US0_CompileStart
            } else {
                US0::US0_PlainStart
            }
        };
        let v36: bool = &*v28 != "";
        if v36 {
            let v37: bool = &*v1 != "";
            if v37 {
                return (v0, v1.clone(), v2.clone(), v3.clone(), v21.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
            } else {
                let v38: i32 = v0.wrapping_add(2i32);
                let v39: i32 = v0.wrapping_add(1i32);
                let v40: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v39 as usize).unwrap_or_default());
                (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = (v38, v28.clone(), v40.clone(), v3.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
                continue;
            }
        } else {
            let v83: bool = match &v35 {
                US0::US0_PlainStart => {
                    true
                }
                _ => {
                    false
                }
            };
            let v84: bool = v83 != true;
            if v84 {
                let v85: bool = match &v3 {
                    US0::US0_PlainStart => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v86: bool = v85 != true;
                if v86 {
                    return (v0, v1.clone(), v2.clone(), v3.clone(), v21.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
                } else {
                    let v89: i32 = match &v35 {
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
                    let v90: i32 = v0.wrapping_add(v89);
                    (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = (v90, v1.clone(), v2.clone(), v35.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
                    continue;
                }
            } else {
                let v133: US1 = start_flag_of_2(v21.clone());
                match &v133 {
                    US1::US1_StartFlagKnown(v134) => {
                        let v134: US2 = v134.clone();
                        let v136: i32 = match &v134 {
                            US2::US2_FrontStart => {
                                1i32
                            }
                            US2::US2_QuietStart => {
                                1i32
                            }
                            _ => {
                                2i32
                            }
                        };
                        let v137: bool = v136 == 2i32;
                        let v141: u64 = if v137 {
                            let v138: i32 = v0.wrapping_add(1i32);
                            let v139: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v138 as usize).unwrap_or_default());
                            decimal_u64_3(v139.clone())
                        } else {
                            0u64
                        };
                        let v142: i32 = v0.wrapping_add(v136);
                        let (mut v271, mut v272, mut v273, mut v274, mut v275, mut v276, mut v277, mut v278, mut v279, mut v280, mut v281, mut v282, mut v283, mut v284, mut v285, mut v286): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                            US2::US2_MaxPrivateMb => {
                                (true, v141, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                            }
                            _ => {
                                let (mut v255, mut v256, mut v257, mut v258, mut v259, mut v260, mut v261, mut v262, mut v263, mut v264, mut v265, mut v266, mut v267, mut v268, mut v269, mut v270): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                    US2::US2_SlotWithin => {
                                        (v5, v6, true, v141, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                    }
                                    _ => {
                                        let (mut v239, mut v240, mut v241, mut v242, mut v243, mut v244, mut v245, mut v246, mut v247, mut v248, mut v249, mut v250, mut v251, mut v252, mut v253, mut v254): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                            US2::US2_DoneWithin => {
                                                (v5, v6, v7, v8, true, v141, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                            }
                                            _ => {
                                                let (mut v223, mut v224, mut v225, mut v226, mut v227, mut v228, mut v229, mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                    US2::US2_QuietStart => {
                                                        (v5, v6, v7, v8, v9, v10, true, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                                    }
                                                    _ => {
                                                        let (mut v207, mut v208, mut v209, mut v210, mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220, mut v221, mut v222): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                            US2::US2_QuietHold => {
                                                                (v5, v6, v7, v8, v9, v10, v11, true, v141, v14, v15, v16, v17, v18, v19, v20)
                                                            }
                                                            _ => {
                                                                let (mut v191, mut v192, mut v193, mut v194, mut v195, mut v196, mut v197, mut v198, mut v199, mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                                    US2::US2_QuietWait => {
                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, true, v141, v16, v17, v18, v19, v20)
                                                                    }
                                                                    _ => {
                                                                        let (mut v175, mut v176, mut v177, mut v178, mut v179, mut v180, mut v181, mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                                            US2::US2_MinFreeMb => {
                                                                                (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, true, v141, v18, v19, v20)
                                                                            }
                                                                            _ => {
                                                                                let (mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                                                    US2::US2_SettleMs => {
                                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, true, v141, v20)
                                                                                    }
                                                                                    _ => {
                                                                                        let (mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v134 {
                                                                                            US2::US2_FrontStart => {
                                                                                                (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, true)
                                                                                            }
                                                                                            _ => unreachable!(),
                                                                                        };
                                                                                        (v143, v144, v145, v146, v147, v148, v149, v150, v151, v152, v153, v154, v155, v156, v157, v158)
                                                                                    }
                                                                                };
                                                                                (v159, v160, v161, v162, v163, v164, v165, v166, v167, v168, v169, v170, v171, v172, v173, v174)
                                                                            }
                                                                        };
                                                                        (v175, v176, v177, v178, v179, v180, v181, v182, v183, v184, v185, v186, v187, v188, v189, v190)
                                                                    }
                                                                };
                                                                (v191, v192, v193, v194, v195, v196, v197, v198, v199, v200, v201, v202, v203, v204, v205, v206)
                                                            }
                                                        };
                                                        (v207, v208, v209, v210, v211, v212, v213, v214, v215, v216, v217, v218, v219, v220, v221, v222)
                                                    }
                                                };
                                                (v223, v224, v225, v226, v227, v228, v229, v230, v231, v232, v233, v234, v235, v236, v237, v238)
                                            }
                                        };
                                        (v239, v240, v241, v242, v243, v244, v245, v246, v247, v248, v249, v250, v251, v252, v253, v254)
                                    }
                                };
                                (v255, v256, v257, v258, v259, v260, v261, v262, v263, v264, v265, v266, v267, v268, v269, v270)
                            }
                        };
                        (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = (v142, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v271, v272, v273, v274, v275, v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v286);
                        continue;
                    }
                    US1::US1_StartFlagUnknown => {
                        return (v0, v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
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
fn start_args_0() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool) {
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
    let v18: bool = false;
    let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (i32, Rc<str>, Rc<str>, US0, Rc<str>, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = flags_read_1(v0, v1.clone(), v1.clone(), v2.clone(), v1.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18);
    let v41: u64 = if v31 {
        v32
    } else {
        if v30 {
            30u64
        } else {
            0u64
        }
    };
    let v43: u64 = if v33 {
        v34
    } else {
        if v30 {
            10u64
        } else {
            0u64
        }
    };
    let v44: bool = &*v20 == "";
    let v50: bool = if v44 {
        true
    } else {
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
        let v46: bool = v21.starts_with(&*v45);
        if v46 {
            let v47: Rc<str> = job_directory_5(v21.clone());
            directory_exists_8(v47.clone())
        } else {
            false
        }
    };
    let v51: Rc<str> = if v50 {
        v1.clone()
    } else {
        v21.clone()
    };
    let v52: i32 = argument_count_9();
    let v53: i32 = v19.wrapping_add(3i32);
    let v54: bool = v52 < v53;
    if v54 {
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
        (false, v55.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
    } else {
        let v56: bool = &*v23 != "";
        if v56 {
            let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is given twice (one --after or --after-ok, one --heavy or --compile)"); } LIT.with(|lit| lit.clone()) };
            let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v57));
            (false, v58.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
        } else {
            let v59: bool = &*v51 != "";
            if v59 {
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after names no job: "); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v51));
                (false, v61.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
            } else {
                let v63: bool = if v24 {
                    let v62: bool = v25 == 0u64;
                    v62
                } else {
                    false
                };
                if v63 {
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                    (false, v64.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                } else {
                    let v66: bool = if v26 {
                        let v65: bool = v27 == 0u64;
                        v65
                    } else {
                        false
                    };
                    if v66 {
                        let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                        (false, v67.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                    } else {
                        let v69: bool = if v28 {
                            let v68: bool = v29 == 0u64;
                            v68
                        } else {
                            false
                        };
                        if v69 {
                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                            (false, v70.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                        } else {
                            let v71: bool = v31 || v33;
                            let v73: bool = if v71 {
                                let v72: bool = v30 == false;
                                v72
                            } else {
                                false
                            };
                            if v73 {
                                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold and --quiet-wait follow --quiet"); } LIT.with(|lit| lit.clone()) };
                                (false, v74.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                            } else {
                                let v77: bool = if v30 {
                                    let v75: bool = match &v22 {
                                        US0::US0_HeavyStart => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    };
                                    let v76: bool = v75 != true;
                                    v76
                                } else {
                                    false
                                };
                                if v77 {
                                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet applies to --heavy jobs: it holds compiles off while the heavy work starts"); } LIT.with(|lit| lit.clone()) };
                                    (false, v78.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                } else {
                                    let v82: bool = if v30 {
                                        let v79: bool = v41 == 0u64;
                                        if v79 {
                                            true
                                        } else {
                                            let v80: bool = v41 > 60u64;
                                            v80
                                        }
                                    } else {
                                        false
                                    };
                                    if v82 {
                                        let v83: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold takes 1 to 60 minutes (compiles wait at most that long)"); } LIT.with(|lit| lit.clone()) };
                                        (false, v83.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                    } else {
                                        let v85: bool = if v26 {
                                            match &v22 {
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
                                        if v85 {
                                            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within applies to --heavy or --compile jobs, which wait for a slot or window"); } LIT.with(|lit| lit.clone()) };
                                            (false, v86.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                        } else {
                                            let v88: bool = if v35 {
                                                let v87: bool = v36 == 0u64;
                                                v87
                                            } else {
                                                false
                                            };
                                            if v88 {
                                                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                                                (false, v89.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                            } else {
                                                let v92: bool = if v39 {
                                                    let v90: bool = match &v22 {
                                                        US0::US0_HeavyStart => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    };
                                                    let v91: bool = v90 != true;
                                                    v91
                                                } else {
                                                    false
                                                };
                                                if v92 {
                                                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--front applies to --heavy jobs: it queues the job first in the heavy line"); } LIT.with(|lit| lit.clone()) };
                                                    (false, v93.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                } else {
                                                    let v95: bool = if v35 {
                                                        match &v22 {
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
                                                    if v95 {
                                                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb applies to --heavy or --compile jobs, which wait for a window"); } LIT.with(|lit| lit.clone()) };
                                                        (false, v96.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                    } else {
                                                        let v97: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
                                                        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                                        let v99: bool = v97.starts_with(&*v98);
                                                        if v99 {
                                                            let v100: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
                                                            let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) };
                                                            let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
                                                            let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) };
                                                            let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                                                            let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already failed prints 'job start failed' with its exit and first stderr line and exits with its code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
                                                            let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                                                            (false, v106.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                        } else {
                                                            let v107: i32 = v19.wrapping_add(2i32);
                                                            let v108: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v107 as usize).unwrap_or_default());
                                                            let v109: bool = v108.starts_with(&*v98);
                                                            if v109 {
                                                                let v110: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v107 as usize).unwrap_or_default());
                                                                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the program cannot start with --, got "); } LIT.with(|lit| lit.clone()) };
                                                                let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v110));
                                                                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; options go before the label"); } LIT.with(|lit| lit.clone()) };
                                                                let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
                                                                (false, v114.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                            } else {
                                                                let v115: i32 = v19.wrapping_add(1i32);
                                                                let v116: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v115 as usize).unwrap_or_default());
                                                                let v117: u64 = decimal_u64_3(v116.clone());
                                                                let v118: bool = v117 == 0u64;
                                                                if v118 {
                                                                    let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
                                                                    (false, v119.clone(), v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                                } else {
                                                                    let v120: u64 = if v37 {
                                                                        v38
                                                                    } else {
                                                                        2000u64
                                                                    };
                                                                    let v126: Rc<str> = match &v22 {
                                                                        US0::US0_CompileStart => {
                                                                            let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                                                            v123.clone()
                                                                        }
                                                                        US0::US0_HeavyStart => {
                                                                            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                                                            v122.clone()
                                                                        }
                                                                        US0::US0_PlainStart => {
                                                                            let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                                                            v121.clone()
                                                                        }
                                                                    };
                                                                    (true, v1.clone(), v126.clone(), v20.clone(), v21.clone(), v25, v27, v29, v41, v43, v36, v120, v19, v117, v39)
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
fn closure0() -> Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool)> = Rc::new(move || -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool) {
        start_args_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_start_args() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool) {
    closure0()()
}
