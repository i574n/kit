#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let v0: Rc<str> = std::env::temp_dir().join("kit patch batch ü ".to_owned() + &std::process::id().to_string() + "-" + &std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos().to_string()).display().to_string().into();
    let v1: bool = std::fs::create_dir(&*v0).is_ok();
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target-a.txt"); } LIT.with(|lit| lit.clone()) };
        let v3: Rc<str> = std::path::Path::new(&*v0).join(&*v2).display().to_string().into();
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stage-a.txt"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = std::path::Path::new(&*v0).join(&*v4).display().to_string().into();
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backup-a.txt"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = std::path::Path::new(&*v0).join(&*v6).display().to_string().into();
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("target-b.txt"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = std::path::Path::new(&*v0).join(&*v8).display().to_string().into();
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("stage-b.txt"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = std::path::Path::new(&*v0).join(&*v10).display().to_string().into();
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backup-b.txt"); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = std::path::Path::new(&*v0).join(&*v12).display().to_string().into();
        let v14: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let v16: bool = if v14 {
            let v15: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v15
        } else {
            false
        };
        let v18: bool = if v16 {
            let v17: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v17
        } else {
            false
        };
        let v20: bool = if v18 {
            let v19: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v19
        } else {
            false
        };
        let v22: bool = if v20 {
            let v21: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v21
        } else {
            false
        };
        let v24: bool = if v22 {
            let v23: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v23
        } else {
            false
        };
        let v68: bool = if v24 {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("original-a"); } LIT.with(|lit| lit.clone()) };
            let v26: bool = std::fs::write(&*v3, &*v25).is_ok();
            let v29: bool = if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("original-b"); } LIT.with(|lit| lit.clone()) };
                let v28: bool = std::fs::write(&*v9, &*v27).is_ok();
                v28
            } else {
                false
            };
            let v31: bool = if v29 {
                let v30: bool = std::fs::write(&*v7, &*v25).is_ok();
                v30
            } else {
                false
            };
            let v34: bool = if v31 {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("original-b"); } LIT.with(|lit| lit.clone()) };
                let v33: bool = std::fs::write(&*v13, &*v32).is_ok();
                v33
            } else {
                false
            };
            let v37: bool = if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("changed-a"); } LIT.with(|lit| lit.clone()) };
                let v36: bool = std::fs::write(&*v5, &*v35).is_ok();
                v36
            } else {
                false
            };
            let v40: bool = if v37 {
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("changed-b"); } LIT.with(|lit| lit.clone()) };
                let v39: bool = std::fs::write(&*v11, &*v38).is_ok();
                v39
            } else {
                false
            };
            let v42: bool = if v40 {
                let v41: bool = std::fs::rename(&*v5, &*v3).is_ok();
                v41
            } else {
                false
            };
            let v45: bool = if v42 {
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("changed-a"); } LIT.with(|lit| lit.clone()) };
                let v44: bool = std::fs::read_to_string(&*v3).is_ok_and(|value| value == *v43);
                v44
            } else {
                false
            };
            let v48: bool = if v45 {
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("original-b"); } LIT.with(|lit| lit.clone()) };
                let v47: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v46);
                v47
            } else {
                false
            };
            if v48 {
                let v49: bool = std::fs::rename(&*v7, &*v3).is_ok();
                let v51: bool = if v49 {
                    let v50: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
                    v50
                } else {
                    false
                };
                let v53: bool = if v51 {
                    let v52: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
                    v52
                } else {
                    false
                };
                let v58: bool = if v53 {
                    let v54: bool = std::fs::read_to_string(&*v3).is_ok_and(|value| value == *v25);
                    if v54 {
                        let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("original-b"); } LIT.with(|lit| lit.clone()) };
                        let v56: bool = std::fs::read_to_string(&*v9).is_ok_and(|value| value == *v55);
                        v56
                    } else {
                        false
                    }
                } else {
                    false
                };
                if v58 {
                    let v59: bool = !std::path::Path::new(&*v5).is_file();
                    let v61: bool = if v59 {
                        let v60: bool = !std::path::Path::new(&*v11).is_file();
                        v60
                    } else {
                        false
                    };
                    let v63: bool = if v61 {
                        let v62: bool = !std::path::Path::new(&*v7).is_file();
                        v62
                    } else {
                        false
                    };
                    if v63 {
                        let v64: bool = !std::path::Path::new(&*v13).is_file();
                        v64
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing.txt"); } LIT.with(|lit| lit.clone()) };
        let v70: Rc<str> = std::path::Path::new(&*v0).join(&*v69).display().to_string().into();
        let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing-promoted.txt"); } LIT.with(|lit| lit.clone()) };
        let v72: Rc<str> = std::path::Path::new(&*v0).join(&*v71).display().to_string().into();
        let v73: bool = std::fs::rename(&*v72, &*v70).is_ok();
        let v74: bool = v73 == false;
        let v75: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let v77: bool = if v75 {
            let v76: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v76
        } else {
            false
        };
        let v79: bool = if v77 {
            let v78: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v78
        } else {
            false
        };
        let v81: bool = if v79 {
            let v80: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v80
        } else {
            false
        };
        let v83: bool = if v81 {
            let v82: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v82
        } else {
            false
        };
        let v85: bool = if v83 {
            let v84: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v84
        } else {
            false
        };
        let v86: bool = std::fs::remove_file(&*v3).is_ok() || !std::path::Path::new(&*v3).exists();
        let v88: bool = if v86 {
            let v87: bool = std::fs::remove_file(&*v5).is_ok() || !std::path::Path::new(&*v5).exists();
            v87
        } else {
            false
        };
        let v90: bool = if v88 {
            let v89: bool = std::fs::remove_file(&*v7).is_ok() || !std::path::Path::new(&*v7).exists();
            v89
        } else {
            false
        };
        let v92: bool = if v90 {
            let v91: bool = std::fs::remove_file(&*v9).is_ok() || !std::path::Path::new(&*v9).exists();
            v91
        } else {
            false
        };
        let v94: bool = if v92 {
            let v93: bool = std::fs::remove_file(&*v11).is_ok() || !std::path::Path::new(&*v11).exists();
            v93
        } else {
            false
        };
        let v96: bool = if v94 {
            let v95: bool = std::fs::remove_file(&*v13).is_ok() || !std::path::Path::new(&*v13).exists();
            v95
        } else {
            false
        };
        let v97: bool = std::fs::remove_dir(&*v0).is_ok();
        let v98: bool = v68 && v74;
        let v99: bool = v98 && v85;
        let v100: bool = v99 && v96;
        let v101: bool = v100 && v97;
        if v101 {
            0i32
        } else {
            1i32
        }
    } else {
        10i32
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
