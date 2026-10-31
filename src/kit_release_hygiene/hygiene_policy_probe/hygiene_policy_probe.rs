#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let v0: i32 = 1i32;
    let v1: i32 = 0i32;
    let v2: i32 = kit_release_hygiene::kit_hygiene_primary_binding(v0, v1);
    let v3: i32 = 0i32;
    let v4: i32 = 1i32;
    let v5: i32 = kit_release_hygiene::kit_hygiene_primary_binding(v3, v4);
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: i32 = kit_release_hygiene::kit_hygiene_primary_binding(v6, v7);
    let v9: i32 = 1i32;
    let v10: i32 = 0i32;
    let v11: i32 = kit_release_hygiene::kit_hygiene_secondary_binding(v9, v10);
    let v12: i32 = 0i32;
    let v13: i32 = 1i32;
    let v14: i32 = kit_release_hygiene::kit_hygiene_secondary_binding(v12, v13);
    let v15: i32 = 0i32;
    let v16: i32 = 0i32;
    let v17: i32 = kit_release_hygiene::kit_hygiene_secondary_binding(v15, v16);
    let v18: i32 = 1i32;
    let v19: i32 = 0i32;
    let v20: i32 = kit_release_hygiene::kit_hygiene_root_file_binding(v18, v19);
    let v21: i32 = 0i32;
    let v22: i32 = 1i32;
    let v23: i32 = kit_release_hygiene::kit_hygiene_root_file_binding(v21, v22);
    let v24: i32 = 0i32;
    let v25: i32 = 0i32;
    let v26: i32 = kit_release_hygiene::kit_hygiene_root_file_binding(v24, v25);
    let v27: i32 = 1i32;
    let v28: i32 = 0i32;
    let v29: i32 = kit_release_hygiene::kit_hygiene_root_aux_binding(v27, v28);
    let v30: i32 = 0i32;
    let v31: i32 = 1i32;
    let v32: i32 = kit_release_hygiene::kit_hygiene_root_aux_binding(v30, v31);
    let v33: i32 = 0i32;
    let v34: i32 = 0i32;
    let v35: i32 = kit_release_hygiene::kit_hygiene_root_aux_binding(v33, v34);
    let v36: i32 = 1i32;
    let v37: i32 = 0i32;
    let v38: i32 = kit_release_hygiene::kit_hygiene_root_directory_binding(v36, v37);
    let v39: i32 = 0i32;
    let v40: i32 = 1i32;
    let v41: i32 = kit_release_hygiene::kit_hygiene_root_directory_binding(v39, v40);
    let v42: i32 = 0i32;
    let v43: i32 = 0i32;
    let v44: i32 = kit_release_hygiene::kit_hygiene_root_directory_binding(v42, v43);
    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".spiral-entry"); } LIT.with(|lit| lit.clone()) };
    let v46: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v45);
    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".kit-typecheck-9.rs"); } LIT.with(|lit| lit.clone()) };
    let v48: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v47);
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".kit-typecheck-9.rs.kit-1.tmp"); } LIT.with(|lit| lit.clone()) };
    let v50: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v49);
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x.receipt"); } LIT.with(|lit| lit.clone()) };
    let v52: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v51);
    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".release-kit-candidate-1"); } LIT.with(|lit| lit.clone()) };
    let v54: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v53);
    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("probe42.rs"); } LIT.with(|lit| lit.clone()) };
    let v56: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v55);
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cold-rebuild-a"); } LIT.with(|lit| lit.clone()) };
    let v58: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v57);
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".cargo"); } LIT.with(|lit| lit.clone()) };
    let v60: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v59);
    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("keep.txt"); } LIT.with(|lit| lit.clone()) };
    let v62: u64 = kit_release_hygiene::kit_hygiene_name_classification(&v61);
    let v63: bool = v2 < 1i32;
    if v63 {
        1i32
    } else {
        let v64: bool = v5 < 1i32;
        if v64 {
            1i32
        } else {
            let v65: bool = 0i32 < v8;
            if v65 {
                1i32
            } else {
                let v66: bool = v11 < 1i32;
                if v66 {
                    1i32
                } else {
                    let v67: bool = v14 < 1i32;
                    if v67 {
                        1i32
                    } else {
                        let v68: bool = 0i32 < v17;
                        if v68 {
                            1i32
                        } else {
                            let v69: bool = v20 < 1i32;
                            if v69 {
                                1i32
                            } else {
                                let v70: bool = v23 < 1i32;
                                if v70 {
                                    1i32
                                } else {
                                    let v71: bool = 0i32 < v26;
                                    if v71 {
                                        1i32
                                    } else {
                                        let v72: bool = v29 < 1i32;
                                        if v72 {
                                            1i32
                                        } else {
                                            let v73: bool = v32 < 1i32;
                                            if v73 {
                                                1i32
                                            } else {
                                                let v74: bool = 0i32 < v35;
                                                if v74 {
                                                    1i32
                                                } else {
                                                    let v75: bool = v38 < 1i32;
                                                    if v75 {
                                                        1i32
                                                    } else {
                                                        let v76: bool = v41 < 1i32;
                                                        if v76 {
                                                            1i32
                                                        } else {
                                                            let v77: bool = 0i32 < v44;
                                                            if v77 {
                                                                1i32
                                                            } else {
                                                                let v78: bool = v46 != 1u64;
                                                                if v78 {
                                                                    1i32
                                                                } else {
                                                                    let v79: bool = v48 != 1u64;
                                                                    if v79 {
                                                                        1i32
                                                                    } else {
                                                                        let v80: bool = v50 != 1u64;
                                                                        if v80 {
                                                                            1i32
                                                                        } else {
                                                                            let v81: bool = v52 != 2u64;
                                                                            if v81 {
                                                                                1i32
                                                                            } else {
                                                                                let v82: bool = v54 != 2u64;
                                                                                if v82 {
                                                                                    1i32
                                                                                } else {
                                                                                    let v83: bool = v56 != 2u64;
                                                                                    if v83 {
                                                                                        1i32
                                                                                    } else {
                                                                                        let v84: bool = v58 != 3u64;
                                                                                        if v84 {
                                                                                            1i32
                                                                                        } else {
                                                                                            let v85: bool = v60 != 3u64;
                                                                                            if v85 {
                                                                                                1i32
                                                                                            } else {
                                                                                                let v86: bool = v62 != 0u64;
                                                                                                if v86 {
                                                                                                    1i32
                                                                                                } else {
                                                                                                    0i32
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
                }
            }
        }
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
