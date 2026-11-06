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
enum US1 {
    US1_Launched(Rc<str>, Rc<str>),
    US1_LaunchRefused(Rc<str>),
}
pub struct Mut0 { pub l0: Rc<str> }
fn shape_of_word_1(mut v0: Rc<str>) -> US0 {
    let v1: bool = "plain" == &*v0;
    if v1 {
        US0::US0_StartPlain
    } else {
        let v3: bool = "heavy" == &*v0;
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
fn now_ms_3() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn method6(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_5(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method6(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
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
    v0
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
    v2
}
fn heavy_gate_script_13() -> Rc<str> {
    let mut v0: Rc<str> = std::rc::Rc::<str>::from(std::env::var_os("KIT_HEAVY_GATE").map(std::path::PathBuf::from).filter(|path| path.is_file()).or_else(|| std::env::current_dir().ok().and_then(|cwd| cwd.ancestors().find_map(|base| { let compiler = base.join("spiral").join("apps").join("compiler"); [compiler.join("scripts").join("heavy.ps1"), compiler.join("tmp").join("scripts").join("heavy.ps1")].into_iter().find(|candidate| candidate.is_file()) }))).map(|path| path.to_string_lossy().into_owned()).unwrap_or_default());
    v0
}
fn job_argv_10(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64) -> Rc<str> {
    match v0 {
        US0::US0_StartCompile => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window-class"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = job_file_11(v1, v5);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("compile"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = text_written_12(v6, v7);
            v3
        }
        US0::US0_StartHeavy => {
            let v9: Rc<str> = heavy_gate_script_13();
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v11
            } else {
                let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv.inner"); } LIT.with(|lit| lit.clone()) };
                let v13: Rc<str> = job_file_11(v1.clone(), v12.clone());
                let v14: Rc<str> = text_written_12(v13, v3);
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("heavy-wrapper.ps1"); } LIT.with(|lit| lit.clone()) };
                let v16: Rc<str> = job_file_11(v1.clone(), v15);
                let v17: Rc<str> = job_file_11(v1.clone(), v12);
                let v18: Rc<str> = kit_machine_windows::kit_heavy_wrapper(&*v9, &*v17, &*v2, &*v1, v4);
                let v19: Rc<str> = text_written_12(v16.clone(), v18);
                let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pwsh\n-NoProfile\n-File\n"); } LIT.with(|lit| lit.clone()) }, v16));
                v20
            }
        }
        US0::US0_StartPlain => {
            v3
        }
    }
}
fn environment_snapshot_14() -> Rc<str> {
    let mut v0: Rc<str> = Rc::<str>::from(std::env::vars_os().map(|(key, value)| format!("{}={}", key.to_string_lossy(), value.to_string_lossy())).collect::<Vec<_>>().join(&char::from(0u8).to_string()).as_str());
    v0
}
fn get_environment_variable_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn job_launched_2(mut v0: US0, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>) -> US1 {
    let v15: u64 = now_ms_3();
    let v16: Rc<str> = decimal_text_4(v15);
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job-"); } LIT.with(|lit| lit.clone()) }, v16));
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
    let v19: Rc<str> = std::rc::Rc::<str>::from(v11.chars().map(|c| if c.is_ascii_alphanumeric() || c == char::from(45u8) { c } else { char::from(95u8) }).take(40).collect::<String>());
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: Rc<str> = job_directory_7(v20.clone());
    let v22: Rc<str> = std::rc::Rc::<str>::from(std::fs::create_dir_all(&*v21).err().map(|error| format!("create {}: {error}", &*v21)).unwrap_or_default());
    let v23: bool = &*v22 != "";
    if v23 {
        US1::US1_LaunchRefused(v22)
    } else {
        let v25: Rc<str> = job_argv_10(v0, v20.clone(), v11.clone(), v13, v12);
        let v26: bool = &*v25 == "";
        if v26 {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--heavy needs spiral's scripts/heavy.ps1: set KIT_HEAVY_GATE or run from inside a tree that has a spiral checkout"); } LIT.with(|lit| lit.clone()) };
            US1::US1_LaunchRefused(v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("label"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = job_file_11(v20.clone(), v29);
            let v31: Rc<str> = text_written_12(v30, v11);
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = job_file_11(v20.clone(), v32);
            let v34: Rc<str> = decimal_text_4(v12);
            let v35: Rc<str> = text_written_12(v33, v34);
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v35));
            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cwd"); } LIT.with(|lit| lit.clone()) };
            let v38: Rc<str> = job_file_11(v20.clone(), v37);
            let v39: Rc<str> = text_written_12(v38, v14);
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v39));
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("argv"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = job_file_11(v20.clone(), v41);
            let v43: Rc<str> = text_written_12(v42, v25);
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v43));
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started_ms"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = job_file_11(v20.clone(), v45);
            let v47: u64 = now_ms_3();
            let v48: Rc<str> = decimal_text_4(v47);
            let v49: Rc<str> = text_written_12(v46, v48);
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v49));
            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            let v52: Rc<str> = job_file_11(v20.clone(), v51);
            let v53: Rc<str> = environment_snapshot_14();
            let v54: Rc<str> = text_written_12(v52, v53);
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v54));
            let v56: bool = &*v1 == "";
            let v63: Rc<str> = if v56 {
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v57
            } else {
                let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("after"); } LIT.with(|lit| lit.clone()) };
                let v59: Rc<str> = job_file_11(v20.clone(), v58);
                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) }));
                let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v1));
                text_written_12(v59, v61)
            };
            let v64: bool = v4 == 0u64;
            let v70: Rc<str> = if v64 {
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v65
            } else {
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory_cap_mb"); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = job_file_11(v20.clone(), v66);
                let v68: Rc<str> = decimal_text_4(v4);
                text_written_12(v67, v68)
            };
            let v71: bool = v5 == 0u64;
            let v76: u64 = if v71 {
                v6
            } else {
                let v72: bool = v6 == 0u64;
                let v74: bool = if v72 {
                    true
                } else {
                    let v73: bool = v5 < v6;
                    v73
                };
                if v74 {
                    v5
                } else {
                    v6
                }
            };
            let v77: bool = v76 == 0u64;
            let v86: Rc<str> = if v77 {
                let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v78
            } else {
                let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("slot_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v80: Rc<str> = job_file_11(v20.clone(), v79);
                let v81: u64 = now_ms_3();
                let v82: u64 = v76.wrapping_mul(60000u64);
                let v83: u64 = v81.wrapping_add(v82);
                let v84: Rc<str> = decimal_text_4(v83);
                text_written_12(v80, v84)
            };
            let v87: bool = v6 == 0u64;
            let v96: Rc<str> = if v87 {
                let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v88
            } else {
                let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("done_by_ms"); } LIT.with(|lit| lit.clone()) };
                let v90: Rc<str> = job_file_11(v20.clone(), v89);
                let v91: u64 = now_ms_3();
                let v92: u64 = v6.wrapping_mul(60000u64);
                let v93: u64 = v91.wrapping_add(v92);
                let v94: Rc<str> = decimal_text_4(v93);
                text_written_12(v90, v94)
            };
            let v97: bool = v7 == 0u64;
            let v108: Rc<str> = if v97 {
                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v98
            } else {
                let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_hold_min"); } LIT.with(|lit| lit.clone()) };
                let v100: Rc<str> = job_file_11(v20.clone(), v99);
                let v101: Rc<str> = decimal_text_4(v7);
                let v102: Rc<str> = text_written_12(v100, v101);
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("quiet_wait_min"); } LIT.with(|lit| lit.clone()) };
                let v104: Rc<str> = job_file_11(v20.clone(), v103);
                let v105: Rc<str> = decimal_text_4(v8);
                let v106: Rc<str> = text_written_12(v104, v105);
                let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v106));
                v107
            };
            let v109: bool = v9 == 0u64;
            let v115: Rc<str> = if v109 {
                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v110
            } else {
                let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("min_free_mb"); } LIT.with(|lit| lit.clone()) };
                let v112: Rc<str> = job_file_11(v20.clone(), v111);
                let v113: Rc<str> = decimal_text_4(v9);
                text_written_12(v112, v113)
            };
            let v121: Rc<str> = if v10 {
                let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("front"); } LIT.with(|lit| lit.clone()) };
                let v117: Rc<str> = job_file_11(v20.clone(), v116);
                let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                text_written_12(v117, v118)
            } else {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v120
            };
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_AUTHOR"); } LIT.with(|lit| lit.clone()) };
            let v123: Rc<str> = get_environment_variable_15(v122);
            let v124: bool = &*v123 == "";
            let v129: Rc<str> = if v124 {
                let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v125
            } else {
                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("author"); } LIT.with(|lit| lit.clone()) };
                let v127: Rc<str> = job_file_11(v20.clone(), v126);
                text_written_12(v127, v123)
            };
            let v130: bool = &*v55 != "";
            let v132: bool = if v130 {
                true
            } else {
                let v131: bool = &*v63 != "";
                v131
            };
            let v134: bool = if v132 {
                true
            } else {
                let v133: bool = &*v70 != "";
                v133
            };
            let v136: bool = if v134 {
                true
            } else {
                let v135: bool = &*v86 != "";
                v135
            };
            let v138: bool = if v136 {
                true
            } else {
                let v137: bool = &*v96 != "";
                v137
            };
            let v140: bool = if v138 {
                true
            } else {
                let v139: bool = &*v108 != "";
                v139
            };
            let v142: bool = if v140 {
                true
            } else {
                let v141: bool = &*v115 != "";
                v141
            };
            let v144: bool = if v142 {
                true
            } else {
                let v143: bool = &*v121 != "";
                v143
            };
            let v146: bool = if v144 {
                true
            } else {
                let v145: bool = &*v129 != "";
                v145
            };
            if v146 {
                let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v55));
                let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v147, v63));
                let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v70));
                let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v149, v86));
                let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v108));
                let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v151, v121));
                let v153: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v129));
                let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v153));
                US1::US1_LaunchRefused(v154)
            } else {
                let v158: Rc<str> = if v3 {
                    let v156: Rc<str> = std::rc::Rc::<str>::from(std::process::id().to_string().as_str());
                    v156
                } else {
                    let v157: Rc<str> = kit_job_host_request::kit_job_host_request(&*v20);
                    v157
                };
                let v159: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error:"); } LIT.with(|lit| lit.clone()) };
                let v160: bool = v158.starts_with(&*v159);
                if v160 {
                    US1::US1_LaunchRefused(v158)
                } else {
                    US1::US1_Launched(v20, v158)
                }
            }
        }
    }
}
fn job_created_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: bool, mut v12: Rc<str>, mut v13: Rc<str>, mut v14: Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>) {
    let v15: US0 = shape_of_word_1(v14);
    let v16: US1 = job_launched_2(v15, v13, v12, v11, v10, v9, v8, v7, v6, v5, v4, v3, v2, v1, v0);
    match v16 {
        US1::US1_LaunchRefused(v20) => {
            let v20: Rc<str> = v20;
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v21.clone(), v21, v20)
        }
        US1::US1_Launched(v17, v18) => {
            let v17: Rc<str> = v17;
            let v18: Rc<str> = v18;
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v17, v18, v19)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, bool, u64, u64, u64, u64, u64, u64, bool, Rc<str>, u64, Rc<str>, Rc<str>) -> (Rc<str>, Rc<str>, Rc<str>)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: bool, mut v11: Rc<str>, mut v12: u64, mut v13: Rc<str>, mut v14: Rc<str>| -> (Rc<str>, Rc<str>, Rc<str>) {
        job_created_0(v14, v13, v12, v11, v10, v9, v8, v7, v6, v5, v4, v3, v2, v1, v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_job_create(v0: &str, v1: &str, v2: &str, v3: bool, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: bool, v11: &str, v12: u64, v13: &str, v14: &str) -> (Rc<str>, Rc<str>, Rc<str>) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), v3, v4, v5, v6, v7, v8, v9, v10, Rc::<str>::from(v11), v12, Rc::<str>::from(v13), Rc::<str>::from(v14))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
