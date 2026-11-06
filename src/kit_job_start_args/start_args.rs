#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_StartPlain,
    US0_StartHeavy,
    US0_StartCompile,
}
#[derive(Clone)]
enum US2 {
    US2_StartMaxPrivateMb,
    US2_StartSlotWithin,
    US2_StartDoneWithin,
    US2_StartQuiet,
    US2_StartQuietHold,
    US2_StartQuietWait,
    US2_StartMinFreeMb,
    US2_StartSettleMs,
    US2_StartFront,
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
fn shape_of_word_2(mut v0: Rc<str>) -> US0 {
    let v1: bool = "plain" == &*v0;
    if v1 {
        US0::US0_StartPlain
    } else {
        let mut v3: bool = "heavy" == &*v0;
        if v3 {
            US0::US0_StartHeavy
        } else {
            let v5: bool = "compile" == &*v0;
            if v5 {
                US0::US0_StartCompile
            } else {
                US0::US0_StartPlain
            }
        }
    }
}
fn start_flag_word_4(mut v0: US2) -> Rc<str> {
    let v10: Rc<str> = match v0 {
        US2::US2_StartDoneWithin => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("DoneWithin"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_StartFront => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Front"); } LIT.with(|lit| lit.clone()) };
            v9
        }
        US2::US2_StartMaxPrivateMb => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MaxPrivateMb"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_StartMinFreeMb => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("MinFreeMb"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        US2::US2_StartQuiet => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("Quiet"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_StartQuietHold => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("QuietHold"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_StartQuietWait => {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("QuietWait"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US2::US2_StartSettleMs => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SettleMs"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        US2::US2_StartSlotWithin => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("SlotWithin"); } LIT.with(|lit| lit.clone()) };
            v2
        }
    };
    let v11: Rc<str> = kit_flag_words::kit_flag_word(&v10);
    v11
}
fn start_flag_of_3(mut v0: Rc<str>) -> US1 {
    let v1: US2 = US2::US2_StartMaxPrivateMb;
    let v2: Rc<str> = start_flag_word_4(v1);
    let mut v3: bool = &*v0 == &*v2;
    if v3 {
        let mut v4: US2 = US2::US2_StartMaxPrivateMb;
        US1::US1_StartFlagKnown(v4)
    } else {
        let mut v6: US2 = US2::US2_StartSlotWithin;
        let v7: Rc<str> = start_flag_word_4(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: US2 = US2::US2_StartSlotWithin;
            US1::US1_StartFlagKnown(v9)
        } else {
            let v11: US2 = US2::US2_StartDoneWithin;
            let v12: Rc<str> = start_flag_word_4(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: US2 = US2::US2_StartDoneWithin;
                US1::US1_StartFlagKnown(v14)
            } else {
                let v16: US2 = US2::US2_StartQuiet;
                let v17: Rc<str> = start_flag_word_4(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: US2 = US2::US2_StartQuiet;
                    US1::US1_StartFlagKnown(v19)
                } else {
                    let v21: US2 = US2::US2_StartQuietHold;
                    let v22: Rc<str> = start_flag_word_4(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: US2 = US2::US2_StartQuietHold;
                        US1::US1_StartFlagKnown(v24)
                    } else {
                        let v26: US2 = US2::US2_StartQuietWait;
                        let v27: Rc<str> = start_flag_word_4(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: US2 = US2::US2_StartQuietWait;
                            US1::US1_StartFlagKnown(v29)
                        } else {
                            let v31: US2 = US2::US2_StartMinFreeMb;
                            let v32: Rc<str> = start_flag_word_4(v31);
                            let v33: bool = &*v0 == &*v32;
                            if v33 {
                                let v34: US2 = US2::US2_StartMinFreeMb;
                                US1::US1_StartFlagKnown(v34)
                            } else {
                                let v36: US2 = US2::US2_StartSettleMs;
                                let v37: Rc<str> = start_flag_word_4(v36);
                                let v38: bool = &*v0 == &*v37;
                                if v38 {
                                    let v39: US2 = US2::US2_StartSettleMs;
                                    US1::US1_StartFlagKnown(v39)
                                } else {
                                    let v41: US2 = US2::US2_StartFront;
                                    let v42: Rc<str> = start_flag_word_4(v41);
                                    let v43: bool = &*v0 == &*v42;
                                    if v43 {
                                        let v44: US2 = US2::US2_StartFront;
                                        US1::US1_StartFlagKnown(v44)
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
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_5(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method6(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = v9;
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
            v23
        } else {
            let v24: bool = &*v21 == "--after-ok";
            if v24 {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v25
            } else {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v26
            }
        };
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
        let v30: bool = v21.starts_with(&*v29);
        let v39: US0 = if v30 {
            let v31: i32 = (v21.clone().len() as i32);
            let v32: i32 = v31.wrapping_sub(2i32);
            let v33: usize = ((2i32) as usize);
            let v34: i32 = 2i32.wrapping_add(v32);
            let v35: usize = ((v34) as usize);
            let v36: Rc<str> = Rc::<str>::from(&(v21)[v33..v35]);
            shape_of_word_2(v36)
        } else {
            US0::US0_StartPlain
        };
        let v40: bool = &*v28 != "";
        if v40 {
            let v41: bool = &*v1 != "";
            if v41 {
                return (v0, v1, v2, v3, v21, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
            } else {
                let v42: i32 = v0.wrapping_add(2i32);
                let v43: i32 = v0.wrapping_add(1i32);
                let v44: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v43 as usize).unwrap_or_default());
                (v0, v1, v2) = (v42, v28, v44);
                continue;
            }
        } else {
            let v87: bool = match &v39 {
                US0::US0_StartPlain => {
                    true
                }
                _ => {
                    false
                }
            };
            let v88: bool = v87 != true;
            if v88 {
                let v89: bool = match &v3 {
                    US0::US0_StartPlain => {
                        true
                    }
                    _ => {
                        false
                    }
                };
                let v90: bool = v89 != true;
                if v90 {
                    return (v0, v1, v2, v3, v21, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
                } else {
                    let v93: i32 = match &v39 {
                        US0::US0_StartCompile => {
                            1i32
                        }
                        US0::US0_StartHeavy => {
                            1i32
                        }
                        US0::US0_StartPlain => {
                            0i32
                        }
                    };
                    let v94: i32 = v0.wrapping_add(v93);
                    (v0, v3) = (v94, v39);
                    continue;
                }
            } else {
                let v137: US1 = start_flag_of_3(v21);
                match v137 {
                    US1::US1_StartFlagKnown(v138) => {
                        let v138: US2 = v138;
                        let v140: i32 = match &v138 {
                            US2::US2_StartFront => {
                                1i32
                            }
                            US2::US2_StartQuiet => {
                                1i32
                            }
                            _ => {
                                2i32
                            }
                        };
                        let v141: bool = v140 == 2i32;
                        let v145: u64 = if v141 {
                            let v142: i32 = v0.wrapping_add(1i32);
                            let v143: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v142 as usize).unwrap_or_default());
                            decimal_u64_5(v143)
                        } else {
                            0u64
                        };
                        let v146: i32 = v0.wrapping_add(v140);
                        let (mut v275, mut v276, mut v277, mut v278, mut v279, mut v280, mut v281, mut v282, mut v283, mut v284, mut v285, mut v286, mut v287, mut v288, mut v289, mut v290): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                            US2::US2_StartMaxPrivateMb => {
                                (true, v145, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                            }
                            _ => {
                                let (mut v259, mut v260, mut v261, mut v262, mut v263, mut v264, mut v265, mut v266, mut v267, mut v268, mut v269, mut v270, mut v271, mut v272, mut v273, mut v274): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                    US2::US2_StartSlotWithin => {
                                        (v5, v6, true, v145, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                    }
                                    _ => {
                                        let (mut v243, mut v244, mut v245, mut v246, mut v247, mut v248, mut v249, mut v250, mut v251, mut v252, mut v253, mut v254, mut v255, mut v256, mut v257, mut v258): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                            US2::US2_StartDoneWithin => {
                                                (v5, v6, v7, v8, true, v145, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                            }
                                            _ => {
                                                let (mut v227, mut v228, mut v229, mut v230, mut v231, mut v232, mut v233, mut v234, mut v235, mut v236, mut v237, mut v238, mut v239, mut v240, mut v241, mut v242): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                                    US2::US2_StartQuiet => {
                                                        (v5, v6, v7, v8, v9, v10, true, v12, v13, v14, v15, v16, v17, v18, v19, v20)
                                                    }
                                                    _ => {
                                                        let (mut v211, mut v212, mut v213, mut v214, mut v215, mut v216, mut v217, mut v218, mut v219, mut v220, mut v221, mut v222, mut v223, mut v224, mut v225, mut v226): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                                            US2::US2_StartQuietHold => {
                                                                (v5, v6, v7, v8, v9, v10, v11, true, v145, v14, v15, v16, v17, v18, v19, v20)
                                                            }
                                                            _ => {
                                                                let (mut v195, mut v196, mut v197, mut v198, mut v199, mut v200, mut v201, mut v202, mut v203, mut v204, mut v205, mut v206, mut v207, mut v208, mut v209, mut v210): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                                                    US2::US2_StartQuietWait => {
                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, true, v145, v16, v17, v18, v19, v20)
                                                                    }
                                                                    _ => {
                                                                        let (mut v179, mut v180, mut v181, mut v182, mut v183, mut v184, mut v185, mut v186, mut v187, mut v188, mut v189, mut v190, mut v191, mut v192, mut v193, mut v194): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                                                            US2::US2_StartMinFreeMb => {
                                                                                (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, true, v145, v18, v19, v20)
                                                                            }
                                                                            _ => {
                                                                                let (mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177, mut v178): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match &v138 {
                                                                                    US2::US2_StartSettleMs => {
                                                                                        (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, true, v145, v20)
                                                                                    }
                                                                                    _ => {
                                                                                        let (mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162): (bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = match v138 {
                                                                                            US2::US2_StartFront => {
                                                                                                (v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, true)
                                                                                            }
                                                                                            _ => unreachable!(),
                                                                                        };
                                                                                        (v147, v148, v149, v150, v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162)
                                                                                    }
                                                                                };
                                                                                (v163, v164, v165, v166, v167, v168, v169, v170, v171, v172, v173, v174, v175, v176, v177, v178)
                                                                            }
                                                                        };
                                                                        (v179, v180, v181, v182, v183, v184, v185, v186, v187, v188, v189, v190, v191, v192, v193, v194)
                                                                    }
                                                                };
                                                                (v195, v196, v197, v198, v199, v200, v201, v202, v203, v204, v205, v206, v207, v208, v209, v210)
                                                            }
                                                        };
                                                        (v211, v212, v213, v214, v215, v216, v217, v218, v219, v220, v221, v222, v223, v224, v225, v226)
                                                    }
                                                };
                                                (v227, v228, v229, v230, v231, v232, v233, v234, v235, v236, v237, v238, v239, v240, v241, v242)
                                            }
                                        };
                                        (v243, v244, v245, v246, v247, v248, v249, v250, v251, v252, v253, v254, v255, v256, v257, v258)
                                    }
                                };
                                (v259, v260, v261, v262, v263, v264, v265, v266, v267, v268, v269, v270, v271, v272, v273, v274)
                            }
                        };
                        (v0, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = (v146, v275, v276, v277, v278, v279, v280, v281, v282, v283, v284, v285, v286, v287, v288, v289, v290);
                        continue;
                    }
                    US1::US1_StartFlagUnknown => {
                        return (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
                    }
                }
            }
        }
    }
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_9() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_9();
    joined_8(v1, v0)
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.is_dir();
    v8
}
fn command_line_argument_count_12() -> i32 {
    let v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_11() -> i32 {
    let v0: i32 = command_line_argument_count_12();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn start_args_0() -> (bool, Rc<str>, Rc<str>, Rc<str>, Rc<str>, u64, u64, u64, u64, u64, u64, u64, i32, u64, bool) {
    let v0: i32 = 3i32;
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: US0 = US0::US0_StartPlain;
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
    let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39): (i32, Rc<str>, Rc<str>, US0, Rc<str>, bool, u64, bool, u64, bool, u64, bool, bool, u64, bool, u64, bool, u64, bool, u64, bool) = flags_read_1(v0, v1.clone(), v1.clone(), v2, v1.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18);
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
            let v47: Rc<str> = job_directory_7(v21.clone());
            directory_exists_10(v47)
        } else {
            false
        }
    };
    let v51: Rc<str> = if v50 {
        v1.clone()
    } else {
        v21.clone()
    };
    let v52: i32 = argument_count_11();
    let v53: i32 = v19.wrapping_add(3i32);
    let v54: bool = v52 < v53;
    if v54 {
        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job coordinator build --spiral <dir> --kit <dir> (builds the resident job coordinator through Cells --build-only and records its receipt) | job coordinator start --spool <dir> (starts it as a resident job that takes <name>.sub files from the spool) | job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) };
        (false, v55, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
    } else {
        let v56: bool = &*v23 != "";
        if v56 {
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" is given twice (one --after or --after-ok, one --heavy or --compile)"); } LIT.with(|lit| lit.clone()) }));
            (false, v57, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
        } else {
            let v58: bool = &*v51 != "";
            if v58 {
                let v59: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--after names no job: "); } LIT.with(|lit| lit.clone()) }, v51));
                (false, v59, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
            } else {
                let v61: bool = if v24 {
                    let v60: bool = v25 == 0u64;
                    v60
                } else {
                    false
                };
                if v61 {
                    let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-private-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                    (false, v62, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                } else {
                    let v64: bool = if v26 {
                        let v63: bool = v27 == 0u64;
                        v63
                    } else {
                        false
                    };
                    if v64 {
                        let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--slot-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                        (false, v65, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                    } else {
                        let v67: bool = if v28 {
                            let v66: bool = v29 == 0u64;
                            v66
                        } else {
                            false
                        };
                        if v67 {
                            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--done-within needs a positive number of minutes"); } LIT.with(|lit| lit.clone()) };
                            (false, v68, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                        } else {
                            let v69: bool = v31 || v33;
                            let v71: bool = if v69 {
                                let v70: bool = v30 == false;
                                v70
                            } else {
                                false
                            };
                            if v71 {
                                let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold and --quiet-wait follow --quiet"); } LIT.with(|lit| lit.clone()) };
                                (false, v72, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                            } else {
                                let v75: bool = if v30 {
                                    let v73: bool = match &v22 {
                                        US0::US0_StartHeavy => {
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
                                    (false, v76, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                } else {
                                    let v80: bool = if v30 {
                                        let v77: bool = v41 == 0u64;
                                        if v77 {
                                            true
                                        } else {
                                            let v78: bool = v41 > 60u64;
                                            v78
                                        }
                                    } else {
                                        false
                                    };
                                    if v80 {
                                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--quiet-hold takes 1 to 60 minutes (compiles wait at most that long)"); } LIT.with(|lit| lit.clone()) };
                                        (false, v81, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                    } else {
                                        let v83: bool = if v26 {
                                            match &v22 {
                                                US0::US0_StartPlain => {
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
                                            (false, v84, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                        } else {
                                            let v86: bool = if v35 {
                                                let v85: bool = v36 == 0u64;
                                                v85
                                            } else {
                                                false
                                            };
                                            if v86 {
                                                let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb needs a positive number of MB"); } LIT.with(|lit| lit.clone()) };
                                                (false, v87, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                            } else {
                                                let v90: bool = if v39 {
                                                    let v88: bool = match &v22 {
                                                        US0::US0_StartHeavy => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    };
                                                    let v89: bool = v88 != true;
                                                    v89
                                                } else {
                                                    false
                                                };
                                                if v90 {
                                                    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--front applies to --heavy jobs: it queues the job first in the heavy line"); } LIT.with(|lit| lit.clone()) };
                                                    (false, v91, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                } else {
                                                    let v93: bool = if v35 {
                                                        match &v22 {
                                                            US0::US0_StartPlain => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    } else {
                                                        false
                                                    };
                                                    if v93 {
                                                        let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb applies to --heavy or --compile jobs, which wait for a window"); } LIT.with(|lit| lit.clone()) };
                                                        (false, v94, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                    } else {
                                                        let v95: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
                                                        let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                                        let v97: bool = v95.starts_with(&*v96);
                                                        if v97 {
                                                            let v98: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v19 as usize).unwrap_or_default());
                                                            let v99: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v98));
                                                            let v100: Rc<str> = Rc::<str>::from(format!("{}{}", v99, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; "); } LIT.with(|lit| lit.clone()) }));
                                                            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job coordinator build --spiral <dir> --kit <dir> (builds the resident job coordinator through Cells --build-only and records its receipt) | job coordinator start --spool <dir> (starts it as a resident job that takes <name>.sub files from the spool) | job doctor [--env] (audits kit processes, job files and after-chains; --env also compares a detached job's environment with this shell's; exit 1 on a finding) | job after <queued-id> <predecessor-id> [--ok] (re-points a job still waiting on --after/--after-ok; label, timeout, gate and command are kept) | job chain [--ok] [--serial] [<after-id>] -- <label> <timeout-ms> <program> [args...] -- <label> ... (starts each job after the previous one and prints every id; --serial runs the links in order inside one job, so waiting links hold no process) | job start [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] [--settle-ms <n>] <label> <timeout-ms> <program> [args...] (the cap covers the whole job tree; reaching it stops the job as memory-capped, exit 137; a plain start without --after watches the job for --settle-ms, default 2000, 0 skips, and a job that already settled badly prints 'job start failed' with its state, exit and detail and exits with that code) | job run [--after <id>|--after-ok <id>] [--heavy|--compile] [--max-private-mb <n>] [--slot-within <minutes>] [--done-within <minutes>] [--min-free-mb <n>] [--quiet [--quiet-hold <min>] [--quiet-wait <min>]] <label> <timeout-ms> <program> [args...] (supervises in this process, so the job's processes stay in the caller's tree and end with it; exits with the job's code; for long runs from an agent shell that can be reaped, use job start plus a background job wait) | job status <id> | job wait <id> [max-ms, default until it settles] | job list [count] | job tail <id> [lines] | job stop <id> | job amend <id> --timeout <ms> (a queued job only; the limit is read when its child starts)"); } LIT.with(|lit| lit.clone()) }));
                                                            (false, v101, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                        } else {
                                                            let v102: i32 = v19.wrapping_add(2i32);
                                                            let v103: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v102 as usize).unwrap_or_default());
                                                            let v104: bool = v103.starts_with(&*v96);
                                                            if v104 {
                                                                let v105: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v102 as usize).unwrap_or_default());
                                                                let v106: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the program cannot start with --, got "); } LIT.with(|lit| lit.clone()) }, v105));
                                                                let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; options go before the label"); } LIT.with(|lit| lit.clone()) }));
                                                                (false, v107, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                            } else {
                                                                let v108: i32 = v19.wrapping_add(1i32);
                                                                let v109: Rc<str> = std::rc::Rc::<str>::from(std::env::args().nth(v108 as usize).unwrap_or_default());
                                                                let v110: u64 = decimal_u64_5(v109);
                                                                let v111: bool = v110 == 0u64;
                                                                if v111 {
                                                                    let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout-ms must be a positive integer"); } LIT.with(|lit| lit.clone()) };
                                                                    (false, v112, v1.clone(), v1.clone(), v1.clone(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0i32, 0u64, false)
                                                                } else {
                                                                    let v113: u64 = if v37 {
                                                                        v38
                                                                    } else {
                                                                        2000u64
                                                                    };
                                                                    let v117: Rc<str> = match v22 {
                                                                        US0::US0_StartCompile => {
                                                                            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
                                                                            v116
                                                                        }
                                                                        US0::US0_StartHeavy => {
                                                                            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy"); } LIT.with(|lit| lit.clone()) };
                                                                            v115
                                                                        }
                                                                        US0::US0_StartPlain => {
                                                                            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plain"); } LIT.with(|lit| lit.clone()) };
                                                                            v114
                                                                        }
                                                                    };
                                                                    (true, v1, v117, v20, v21, v25, v27, v29, v41, v43, v36, v113, v19, v110, v39)
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
