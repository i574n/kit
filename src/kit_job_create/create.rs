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
enum US1 {
    US1_Launched(Rc<str>, Rc<str>),
    US1_LaunchRefused(Rc<str>),
}
fn shape_of_word_1(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "heavy";
    if v1 {
        US0::US0_HeavyStart
    } else {
        let v3: bool = &*v0 == "compile";
        if v3 {
            US0::US0_CompileStart
        } else {
            US0::US0_PlainStart
        }
    }
}
fn now_ms_3() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn decimal_text_4(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
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
fn job_file_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_5(v0);
    joined_6(v2, v1)
}
fn text_written_10(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn heavy_gate_script_11() -> Rc<str> {
    let v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn job_argv_8(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match &v0 {
        US0::US0_CompileStart => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = job_file_9(v1.clone(), v5.clone());
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = text_written_10(v6.clone(), v7.clone());
            v3.clone()
        }
        US0::US0_HeavyStart => {
            let v9: Rc<str> = heavy_gate_script_11();
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = job_file_9(v1.clone(), v12.clone());
                let v14: Rc<str> = text_written_10(v13.clone(), v3.clone());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = job_file_9(v1.clone(), v15.clone());
                let v17: Rc<str> = job_file_9(v1.clone(), v12.clone());
                let v18: Rc<str> = kit_machine_windows::kit_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let v19: Rc<str> = text_written_10(v16.clone(), v18.clone());
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) };
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v16));
                v21.clone()
            }
        }
        US0::US0_PlainStart => {
            v3.clone()
        }
    }
}
fn environment_snapshot_12() -> Rc<str> {
    let v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0.clone()
}
fn get_environment_variable_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn job_launched_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: u64, mut v12: Rc<str>, mut v13: Rc<str>) -> US1 {
    let v14: u64 = now_ms_3();
    let v15: Rc<str> = decimal_text_4(v14);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: Rc<str> = std::rc::Rc::<str>::from(v10.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
    let v22: Rc<str> = job_directory_5(v21.clone());
    let v23: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v22).err().map(|error| format!("create {}: {error}", &*v22)).unwrap_or_default());
    let v24: bool = &*v23 != "";
    if v24 {
        US1::US1_LaunchRefused(v23.clone())
    } else {
        let v26: Rc<str> = job_argv_8(v0.clone(), v21.clone(), v10.clone(), v12.clone(), v11);
        let v27: bool = &*v26 == "";
        if v27 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set KIT_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US1::US1_LaunchRefused(v28.clone())
        } else {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = job_file_9(v21.clone(), v30.clone());
            let v32: Rc<str> = text_written_10(v31.clone(), v10.clone());
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = job_file_9(v21.clone(), v33.clone());
            let v35: Rc<str> = decimal_text_4(v11);
            let v36: Rc<str> = text_written_10(v34.clone(), v35.clone());
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v36));
            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let v39: Rc<str> = job_file_9(v21.clone(), v38.clone());
            let v40: Rc<str> = text_written_10(v39.clone(), v13.clone());
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v40));
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let v43: Rc<str> = job_file_9(v21.clone(), v42.clone());
            let v44: Rc<str> = text_written_10(v43.clone(), v26.clone());
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v44));
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = job_file_9(v21.clone(), v46.clone());
            let v48: u64 = now_ms_3();
            let v49: Rc<str> = decimal_text_4(v48);
            let v50: Rc<str> = text_written_10(v47.clone(), v49.clone());
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v50));
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = job_file_9(v21.clone(), v52.clone());
            let v54: Rc<str> = environment_snapshot_12();
            let v55: Rc<str> = text_written_10(v53.clone(), v54.clone());
            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v55));
            let v57: bool = &*v1 == "";
            let v65: Rc<str> = if v57 {
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v58.clone()
            } else {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v60: Rc<str> = job_file_9(v21.clone(), v59.clone());
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v63: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v61)), v1));
                text_written_10(v60.clone(), v63.clone())
            };
            let v66: bool = v4 == 0u64;
            let v72: Rc<str> = if v66 {
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v67.clone()
            } else {
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let v69: Rc<str> = job_file_9(v21.clone(), v68.clone());
                let v70: Rc<str> = decimal_text_4(v4);
                text_written_10(v69.clone(), v70.clone())
            };
            let v73: bool = v5 == 0u64;
            let v78: u64 = if v73 {
                v6
            } else {
                let v74: bool = v6 == 0u64;
                let v76: bool = if v74 {
                    true
                } else {
                    let v75: bool = v5 < v6;
                    v75
                };
                if v76 {
                    v5
                } else {
                    v6
                }
            };
            let v79: bool = v78 == 0u64;
            let v88: Rc<str> = if v79 {
                let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v80.clone()
            } else {
                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v82: Rc<str> = job_file_9(v21.clone(), v81.clone());
                let v83: u64 = now_ms_3();
                let v84: u64 = v78.wrapping_mul(60000u64);
                let v85: u64 = v83.wrapping_add(v84);
                let v86: Rc<str> = decimal_text_4(v85);
                text_written_10(v82.clone(), v86.clone())
            };
            let v89: bool = v6 == 0u64;
            let v98: Rc<str> = if v89 {
                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v90.clone()
            } else {
                let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v92: Rc<str> = job_file_9(v21.clone(), v91.clone());
                let v93: u64 = now_ms_3();
                let v94: u64 = v6.wrapping_mul(60000u64);
                let v95: u64 = v93.wrapping_add(v94);
                let v96: Rc<str> = decimal_text_4(v95);
                text_written_10(v92.clone(), v96.clone())
            };
            let v99: bool = v7 == 0u64;
            let v110: Rc<str> = if v99 {
                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v100.clone()
            } else {
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let v102: Rc<str> = job_file_9(v21.clone(), v101.clone());
                let v103: Rc<str> = decimal_text_4(v7);
                let v104: Rc<str> = text_written_10(v102.clone(), v103.clone());
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = job_file_9(v21.clone(), v105.clone());
                let v107: Rc<str> = decimal_text_4(v8);
                let v108: Rc<str> = text_written_10(v106.clone(), v107.clone());
                let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v108));
                v109.clone()
            };
            let v111: bool = v9 == 0u64;
            let v117: Rc<str> = if v111 {
                let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v112.clone()
            } else {
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
                let v114: Rc<str> = job_file_9(v21.clone(), v113.clone());
                let v115: Rc<str> = decimal_text_4(v9);
                text_written_10(v114.clone(), v115.clone())
            };
            let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v119: Rc<str> = get_environment_variable_13(v118.clone());
            let v120: bool = &*v119 == "";
            let v125: Rc<str> = if v120 {
                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v121.clone()
            } else {
                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v123: Rc<str> = job_file_9(v21.clone(), v122.clone());
                text_written_10(v123.clone(), v119.clone())
            };
            let v126: bool = &*v56 != "";
            let v128: bool = if v126 {
                true
            } else {
                let v127: bool = &*v65 != "";
                v127
            };
            let v130: bool = if v128 {
                true
            } else {
                let v129: bool = &*v72 != "";
                v129
            };
            let v132: bool = if v130 {
                true
            } else {
                let v131: bool = &*v88 != "";
                v131
            };
            let v134: bool = if v132 {
                true
            } else {
                let v133: bool = &*v98 != "";
                v133
            };
            let v136: bool = if v134 {
                true
            } else {
                let v135: bool = &*v110 != "";
                v135
            };
            let v138: bool = if v136 {
                true
            } else {
                let v137: bool = &*v117 != "";
                v137
            };
            let v140: bool = if v138 {
                true
            } else {
                let v139: bool = &*v125 != "";
                v139
            };
            if v140 {
                let v144: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v117, v56)), v65)), v72)), v88));
                let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v144, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v98, v110)), v125))));
                US1::US1_LaunchRefused(v147.clone())
            } else {
                let v151: Rc<str> = if v3 {
                    let v149: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v149.clone()
                } else {
                    let v150: Rc<str> = kit_job_host::kit_job_host_request(&*v21);
                    v150.clone()
                };
                let v152: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let v153: bool = v151.starts_with(&*v152);
                if v153 {
                    US1::US1_LaunchRefused(v151.clone())
                } else {
                    US1::US1_Launched(v21.clone(), v151.clone())
                }
            }
        }
    }
}
fn job_created_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: u64, mut v12: Rc<str>, mut v13: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v14: US0 = shape_of_word_1(v0.clone());
    let v15: US1 = job_launched_2(v14.clone(), v1.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10.clone(), v11, v12.clone(), v13.clone());
    match &v15 {
        US1::US1_LaunchRefused(v19) => {
            let v19: Rc<str> = v19.clone();
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v20.clone(), v20.clone(), v19.clone())
        }
        US1::US1_Launched(v16, v17) => {
            let v16: Rc<str> = v16.clone();
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v16.clone(), v17.clone(), v18.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: u64, mut v12: Rc<str>, mut v13: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>) {
        job_created_0(v0.clone(), v1.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10.clone(), v11, v12.clone(), v13.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_create(v0: &str, v1: &str, v2: &str, v3: bool, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: &str, v11: u64, v12: &str, v13: &str) -> (Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, v5, v6, v7, v8, v9, Rc::<str>::from(v10), v11, Rc::<str>::from(v12), Rc::<str>::from(v13))
}
