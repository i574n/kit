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
struct Mut0 { l0: Rc<str> }
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
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method6(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn decimal_text_4(mut v0: u64) -> Rc<str> {
    format_real_5(v0)
}
fn joined_8(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(std::path::Path::new(&*v0).join(&*v1).to_string_lossy().as_ref());
    v2
}
fn jobs_root_9() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_JOBS_HOME").map(std::path::PathBuf::from).or_else(|| std::env::var_os("LOCALAPPDATA").map(|base| std::path::PathBuf::from(base).join("kit").join("jobs"))).or_else(|| std::env::var_os("HOME").map(|base| std::path::PathBuf::from(base).join(".cache").join("kit").join("jobs"))).unwrap_or_else(|| std::path::PathBuf::from(".kit-jobs")).to_string_lossy().as_ref());
    v0.clone()
}
fn job_directory_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = jobs_root_9();
    joined_8(v1, v0)
}
fn job_file_11(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = job_directory_7(v0);
    joined_8(v2, v1)
}
fn text_written_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let temporary = format!("{}.tmp", &*v0); std::rc::Rc::<str>::from(std::fs::write(&temporary, v1.as_bytes()).and_then(|_| std::fs::rename(&temporary, &*v0)).err().map(|error| format!("write {}: {error}", &*v0)).unwrap_or_default()) };
    v2.clone()
}
fn heavy_gate_script_13() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().map(|base| base.join("spiral").join("apps").join("compiler").join("tmp").join("scripts").join("heavy.ps1")).find(|candidate| candidate.is_file()))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0.clone()
}
fn job_argv_10(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match &v0 {
        US0::US0_CompileStart => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = job_file_11(v1.clone(), v5.clone());
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = text_written_12(v6.clone(), v7.clone());
            v3.clone()
        }
        US0::US0_HeavyStart => {
            let v9: Rc<str> = heavy_gate_script_13();
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = job_file_11(v1.clone(), v12.clone());
                let v14: Rc<str> = text_written_12(v13.clone(), v3.clone());
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = job_file_11(v1.clone(), v15.clone());
                let v17: Rc<str> = job_file_11(v1.clone(), v12.clone());
                let v18: Rc<str> = kit_machine_windows::kit_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let v19: Rc<str> = text_written_12(v16.clone(), v18.clone());
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
fn environment_snapshot_14() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0.clone()
}
fn get_environment_variable_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn job_launched_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>) -> US1 {
    let v15: u64 = now_ms_3();
    let v16: Rc<str> = decimal_text_4(v15);
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v16));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = std::rc::Rc::<str>::from(v11.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
    let v23: Rc<str> = job_directory_7(v22.clone());
    let v24: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v23).err().map(|error| format!("create {}: {error}", &*v23)).unwrap_or_default());
    let v25: bool = &*v24 != "";
    if v25 {
        US1::US1_LaunchRefused(v24.clone())
    } else {
        let v27: Rc<str> = job_argv_10(v0.clone(), v22.clone(), v11.clone(), v13.clone(), v12);
        let v28: bool = &*v27 == "";
        if v28 {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set KIT_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US1::US1_LaunchRefused(v29.clone())
        } else {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = job_file_11(v22.clone(), v31.clone());
            let v33: Rc<str> = text_written_12(v32.clone(), v11.clone());
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<str> = job_file_11(v22.clone(), v34.clone());
            let v36: Rc<str> = decimal_text_4(v12);
            let v37: Rc<str> = text_written_12(v35.clone(), v36.clone());
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v37));
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = job_file_11(v22.clone(), v39.clone());
            let v41: Rc<str> = text_written_12(v40.clone(), v14.clone());
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v41));
            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let v44: Rc<str> = job_file_11(v22.clone(), v43.clone());
            let v45: Rc<str> = text_written_12(v44.clone(), v27.clone());
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v45));
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = job_file_11(v22.clone(), v47.clone());
            let v49: u64 = now_ms_3();
            let v50: Rc<str> = decimal_text_4(v49);
            let v51: Rc<str> = text_written_12(v48.clone(), v50.clone());
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v51));
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let v54: Rc<str> = job_file_11(v22.clone(), v53.clone());
            let v55: Rc<str> = environment_snapshot_14();
            let v56: Rc<str> = text_written_12(v54.clone(), v55.clone());
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v56));
            let v58: bool = &*v1 == "";
            let v66: Rc<str> = if v58 {
                let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v59.clone()
            } else {
                let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v61: Rc<str> = job_file_11(v22.clone(), v60.clone());
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v2, v62)), v1));
                text_written_12(v61.clone(), v64.clone())
            };
            let v67: bool = v4 == 0u64;
            let v73: Rc<str> = if v67 {
                let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v68.clone()
            } else {
                let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let v70: Rc<str> = job_file_11(v22.clone(), v69.clone());
                let v71: Rc<str> = decimal_text_4(v4);
                text_written_12(v70.clone(), v71.clone())
            };
            let v74: bool = v5 == 0u64;
            let v79: u64 = if v74 {
                v6
            } else {
                let v75: bool = v6 == 0u64;
                let v77: bool = if v75 {
                    true
                } else {
                    let v76: bool = v5 < v6;
                    v76
                };
                if v77 {
                    v5
                } else {
                    v6
                }
            };
            let v80: bool = v79 == 0u64;
            let v89: Rc<str> = if v80 {
                let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v81.clone()
            } else {
                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v83: Rc<str> = job_file_11(v22.clone(), v82.clone());
                let v84: u64 = now_ms_3();
                let v85: u64 = v79.wrapping_mul(60000u64);
                let v86: u64 = v84.wrapping_add(v85);
                let v87: Rc<str> = decimal_text_4(v86);
                text_written_12(v83.clone(), v87.clone())
            };
            let v90: bool = v6 == 0u64;
            let v99: Rc<str> = if v90 {
                let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v91.clone()
            } else {
                let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v93: Rc<str> = job_file_11(v22.clone(), v92.clone());
                let v94: u64 = now_ms_3();
                let v95: u64 = v6.wrapping_mul(60000u64);
                let v96: u64 = v94.wrapping_add(v95);
                let v97: Rc<str> = decimal_text_4(v96);
                text_written_12(v93.clone(), v97.clone())
            };
            let v100: bool = v7 == 0u64;
            let v111: Rc<str> = if v100 {
                let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v101.clone()
            } else {
                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let v103: Rc<str> = job_file_11(v22.clone(), v102.clone());
                let v104: Rc<str> = decimal_text_4(v7);
                let v105: Rc<str> = text_written_12(v103.clone(), v104.clone());
                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let v107: Rc<str> = job_file_11(v22.clone(), v106.clone());
                let v108: Rc<str> = decimal_text_4(v8);
                let v109: Rc<str> = text_written_12(v107.clone(), v108.clone());
                let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v109));
                v110.clone()
            };
            let v112: bool = v9 == 0u64;
            let v118: Rc<str> = if v112 {
                let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v113.clone()
            } else {
                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
                let v115: Rc<str> = job_file_11(v22.clone(), v114.clone());
                let v116: Rc<str> = decimal_text_4(v9);
                text_written_12(v115.clone(), v116.clone())
            };
            let v124: Rc<str> = if v10 {
                let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
                let v120: Rc<str> = job_file_11(v22.clone(), v119.clone());
                let v121: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                text_written_12(v120.clone(), v121.clone())
            } else {
                let v123: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v123.clone()
            };
            let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v126: Rc<str> = get_environment_variable_15(v125.clone());
            let v127: bool = &*v126 == "";
            let v132: Rc<str> = if v127 {
                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v128.clone()
            } else {
                let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v130: Rc<str> = job_file_11(v22.clone(), v129.clone());
                text_written_12(v130.clone(), v126.clone())
            };
            let v133: bool = &*v57 != "";
            let v135: bool = if v133 {
                true
            } else {
                let v134: bool = &*v66 != "";
                v134
            };
            let v137: bool = if v135 {
                true
            } else {
                let v136: bool = &*v73 != "";
                v136
            };
            let v139: bool = if v137 {
                true
            } else {
                let v138: bool = &*v89 != "";
                v138
            };
            let v141: bool = if v139 {
                true
            } else {
                let v140: bool = &*v99 != "";
                v140
            };
            let v143: bool = if v141 {
                true
            } else {
                let v142: bool = &*v111 != "";
                v142
            };
            let v145: bool = if v143 {
                true
            } else {
                let v144: bool = &*v118 != "";
                v144
            };
            let v147: bool = if v145 {
                true
            } else {
                let v146: bool = &*v124 != "";
                v146
            };
            let v149: bool = if v147 {
                true
            } else {
                let v148: bool = &*v132 != "";
                v148
            };
            if v149 {
                let v153: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v118, v57)), v66)), v73)), v89));
                let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v153, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v99, v111)), v124)), v132))));
                US1::US1_LaunchRefused(v157.clone())
            } else {
                let v161: Rc<str> = if v3 {
                    let v159: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v159.clone()
                } else {
                    let v160: Rc<str> = kit_job_host::kit_job_host_request(&*v22);
                    v160.clone()
                };
                let v162: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let v163: bool = v161.starts_with(&*v162);
                if v163 {
                    US1::US1_LaunchRefused(v161.clone())
                } else {
                    US1::US1_Launched(v22.clone(), v161.clone())
                }
            }
        }
    }
}
fn job_created_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v15: US0 = shape_of_word_1(v0.clone());
    let v16: US1 = job_launched_2(v15.clone(), v1.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12, v13.clone(), v14.clone());
    match &v16 {
        US1::US1_LaunchRefused(v20) => {
            let v20: Rc<str> = v20.clone();
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v21.clone(), v21.clone(), v20.clone())
        }
        US1::US1_Launched(v17, v18) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v17.clone(), v18.clone(), v19.clone())
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>) {
        job_created_0(v0.clone(), v1.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12, v13.clone(), v14.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_create(v0: &str, v1: &str, v2: &str, v3: bool, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: bool, v11: &str, v12: u64, v13: &str, v14: &str) -> (Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, v5, v6, v7, v8, v9, v10, Rc::<str>::from(v11), v12, Rc::<str>::from(v13), Rc::<str>::from(v14))
}
