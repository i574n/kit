#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_patch_compat_driver::patch_apply_gated_context_owned;
use kit_process::run_bounded_program_in_dir;

#[must_use]
fn patch_gated_error(message: &str) -> i32 {
    eprintln!("kit error: {message}");
    2
}

#[must_use]
fn patch_gated_context_owned(root: &str, source: &str, validated_count: i32, gate_program: &str, timeout_ms: u64) -> i32 {
    let gate = std::path::Path::new(gate_program);
    if !gate.is_absolute() || !gate.is_file() {
        return patch_gated_error(&format!("patch gate program must be an absolute regular file: {}", gate.display()));
    }
    let root_path = std::path::Path::new(root).to_path_buf();
    patch_apply_gated_context_owned(root, source, validated_count, || {
        let status = run_bounded_program_in_dir(gate_program, &root_path, timeout_ms)?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("gate program exited nonzero: {status}"))
        }
    })
}

use kit_patch_compat_driver::{patch_plan_error, patch_prepare_source_owned, patch_rehearse_context_owned};

#[must_use]
fn patch_mutation_error(message: &str) -> i32 {
    eprintln!("kit error: {message}");
    2
}

#[derive(Clone)]
enum US0 {
    US0_PatchPlanValid(i32),
    US0_PatchPlanEmpty,
    US0_PatchPlanMalformed,
    US0_PatchPlanTooLarge,
    US0_PatchPlanUnavailable,
}
#[derive(Clone)]
enum US1 {
    US1_QuotedOk(i32),
    US1_QuotedError,
}
#[derive(Clone)]
enum US2 {
    US2_ParsedPatch(i32),
    US2_InvalidPatch,
}
#[derive(Clone)]
enum US3 {
    US3_SeparatorOk(i32),
    US3_SeparatorError,
}
fn skip_spaces_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v1 < v2;
        if v3 {
            let v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let v5: bool = v4 == 32i32;
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            } else {
                let v8: bool = v4 == 9i32;
                if v8 {
                    let v9: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v9, v2);
                    continue;
                } else {
                    return v1;
                }
            }
        } else {
            return v1;
        }
    }
}
fn scan_quoted_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> US1 {
    loop {
        let v3: bool = v1 < v2;
        if v3 {
            let v4: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let v5: bool = v4 == 34i32;
            if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                return US1::US1_QuotedOk(v6);
            } else {
                let v8: bool = v4 == 92i32;
                if v8 {
                    let v9: i32 = v1.wrapping_add(1i32);
                    let v10: bool = v9 < v2;
                    if v10 {
                        let v11: i32 = usize::try_from(v9).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let v12: bool = v11 == 110i32;
                        let v20: bool = if v12 {
                            true
                        } else {
                            let v13: bool = v11 == 114i32;
                            if v13 {
                                true
                            } else {
                                let v14: bool = v11 == 116i32;
                                if v14 {
                                    true
                                } else {
                                    let v15: bool = v11 == 92i32;
                                    if v15 {
                                        true
                                    } else {
                                        let v16: bool = v11 == 34i32;
                                        v16
                                    }
                                }
                            }
                        };
                        if v20 {
                            let v21: i32 = v1.wrapping_add(2i32);
                            (v0, v1, v2) = (v0.clone(), v21, v2);
                            continue;
                        } else {
                            return US1::US1_QuotedError;
                        }
                    } else {
                        return US1::US1_QuotedError;
                    }
                } else {
                    let v27: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v27, v2);
                    continue;
                }
            }
        } else {
            return US1::US1_QuotedError;
        }
    }
}
fn safe_path_loop_5(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v1 < v2;
        if v4 {
            let v5: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let v6: bool = v5 == 92i32;
            if v6 {
                return false;
            } else {
                let v7: bool = v5 == 47i32;
                if v7 {
                    let v8: i32 = v1.wrapping_sub(v3);
                    let v9: bool = 0i32 < v8;
                    let v24: bool = if v9 {
                        let v10: bool = v8 == 1i32;
                        if v10 {
                            let v11: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let v12: bool = v11 == 46i32;
                            let v13: bool = v12 == false;
                            v13
                        } else {
                            let v14: bool = v8 == 2i32;
                            if v14 {
                                let v15: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let v16: bool = v15 == 46i32;
                                if v16 {
                                    let v17: i32 = v3.wrapping_add(1i32);
                                    let v18: i32 = usize::try_from(v17).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                    let v19: bool = v18 == 46i32;
                                    let v20: bool = v19 == false;
                                    v20
                                } else {
                                    true
                                }
                            } else {
                                true
                            }
                        }
                    } else {
                        false
                    };
                    if v24 {
                        let v25: i32 = v1.wrapping_add(1i32);
                        (v0, v1, v2, v3) = (v0.clone(), v25, v2, v25);
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    let v28: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v28, v2, v3);
                    continue;
                }
            }
        } else {
            let v32: i32 = v2.wrapping_sub(v3);
            let v33: bool = 0i32 < v32;
            if v33 {
                let v34: bool = v32 == 1i32;
                if v34 {
                    let v35: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let v36: bool = v35 == 46i32;
                    let v37: bool = v36 == false;
                    return v37;
                } else {
                    let v38: bool = v32 == 2i32;
                    if v38 {
                        let v39: i32 = usize::try_from(v3).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let v40: bool = v39 == 46i32;
                        if v40 {
                            let v41: i32 = v3.wrapping_add(1i32);
                            let v42: i32 = usize::try_from(v41).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let v43: bool = v42 == 46i32;
                            let v44: bool = v43 == false;
                            return v44;
                        } else {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            } else {
                return false;
            }
        }
    }
}
fn scan_plan_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: i32) -> US0 {
    loop {
        let v4: bool = v1 < v2;
        if v4 {
            let v5: i32 = usize::try_from(v1).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
            let v6: bool = v5 == 80i32;
            let v50: bool = if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: i32 = usize::try_from(v7).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                let v9: bool = v8 == 97i32;
                if v9 {
                    let v10: i32 = v1.wrapping_add(2i32);
                    let v11: i32 = usize::try_from(v10).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let v12: bool = v11 == 116i32;
                    if v12 {
                        let v13: i32 = v1.wrapping_add(3i32);
                        let v14: i32 = usize::try_from(v13).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                        let v15: bool = v14 == 99i32;
                        if v15 {
                            let v16: i32 = v1.wrapping_add(4i32);
                            let v17: i32 = usize::try_from(v16).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let v18: bool = v17 == 104i32;
                            if v18 {
                                let v19: i32 = v1.wrapping_add(5i32);
                                let v20: i32 = usize::try_from(v19).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let v21: bool = v20 == 69i32;
                                if v21 {
                                    let v22: i32 = v1.wrapping_add(6i32);
                                    let v23: i32 = usize::try_from(v22).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                    let v24: bool = v23 == 120i32;
                                    if v24 {
                                        let v25: i32 = v1.wrapping_add(7i32);
                                        let v26: i32 = usize::try_from(v25).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                        let v27: bool = v26 == 97i32;
                                        if v27 {
                                            let v28: i32 = v1.wrapping_add(8i32);
                                            let v29: i32 = usize::try_from(v28).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                            let v30: bool = v29 == 99i32;
                                            if v30 {
                                                let v31: i32 = v1.wrapping_add(9i32);
                                                let v32: i32 = usize::try_from(v31).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                let v33: bool = v32 == 116i32;
                                                if v33 {
                                                    let v34: i32 = v1.wrapping_add(10i32);
                                                    let v35: i32 = usize::try_from(v34).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                    let v36: bool = v35 == 32i32;
                                                    if v36 {
                                                        let v37: i32 = v1.wrapping_add(11i32);
                                                        let v38: i32 = usize::try_from(v37).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                        let v39: bool = v38 == 40i32;
                                                        v39
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
                                        }
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
            if v50 {
                let v51: i32 = v1.wrapping_add(12i32);
                let v52: i32 = skip_spaces_3(v0.clone(), v51, v2);
                let v53: bool = v52 < v2;
                let v61: US1 = if v53 {
                    let v54: i32 = usize::try_from(v52).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                    let v55: bool = v54 == 34i32;
                    if v55 {
                        let v56: i32 = v52.wrapping_add(1i32);
                        scan_quoted_4(v0.clone(), v56, v2)
                    } else {
                        US1::US1_QuotedError
                    }
                } else {
                    US1::US1_QuotedError
                };
                let v144: US2 = match &v61 {
                    US1::US1_QuotedError => {
                        US2::US2_InvalidPatch
                    }
                    US1::US1_QuotedOk(v62) => {
                        let v62: i32 = *v62;
                        let v63: i32 = v52.wrapping_add(1i32);
                        let v64: i32 = v62.wrapping_sub(1i32);
                        let v65: i32 = v64.wrapping_sub(v63);
                        let v66: bool = 0i32 < v65;
                        let v73: bool = if v66 {
                            let v67: i32 = usize::try_from(v63).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                            let v68: bool = v67 == 47i32;
                            if v68 {
                                false
                            } else {
                                let v69: bool = v67 == 92i32;
                                if v69 {
                                    false
                                } else {
                                    safe_path_loop_5(v0.clone(), v63, v64, v63)
                                }
                            }
                        } else {
                            false
                        };
                        if v73 {
                            let v74: i32 = skip_spaces_3(v0.clone(), v62, v2);
                            let v75: bool = v74 < v2;
                            let v84: US3 = if v75 {
                                let v76: i32 = usize::try_from(v74).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                let v77: bool = v76 == 44i32;
                                if v77 {
                                    let v78: i32 = v74.wrapping_add(1i32);
                                    let v79: i32 = skip_spaces_3(v0.clone(), v78, v2);
                                    US3::US3_SeparatorOk(v79)
                                } else {
                                    US3::US3_SeparatorError
                                }
                            } else {
                                US3::US3_SeparatorError
                            };
                            match &v84 {
                                US3::US3_SeparatorError => {
                                    US2::US2_InvalidPatch
                                }
                                US3::US3_SeparatorOk(v85) => {
                                    let v85: i32 = *v85;
                                    let v86: bool = v85 < v2;
                                    let v94: US1 = if v86 {
                                        let v87: i32 = usize::try_from(v85).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                        let v88: bool = v87 == 34i32;
                                        if v88 {
                                            let v89: i32 = v85.wrapping_add(1i32);
                                            scan_quoted_4(v0.clone(), v89, v2)
                                        } else {
                                            US1::US1_QuotedError
                                        }
                                    } else {
                                        US1::US1_QuotedError
                                    };
                                    match &v94 {
                                        US1::US1_QuotedError => {
                                            US2::US2_InvalidPatch
                                        }
                                        US1::US1_QuotedOk(v95) => {
                                            let v95: i32 = *v95;
                                            let v96: i32 = skip_spaces_3(v0.clone(), v95, v2);
                                            let v97: bool = v96 < v2;
                                            let v106: US3 = if v97 {
                                                let v98: i32 = usize::try_from(v96).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                let v99: bool = v98 == 44i32;
                                                if v99 {
                                                    let v100: i32 = v96.wrapping_add(1i32);
                                                    let v101: i32 = skip_spaces_3(v0.clone(), v100, v2);
                                                    US3::US3_SeparatorOk(v101)
                                                } else {
                                                    US3::US3_SeparatorError
                                                }
                                            } else {
                                                US3::US3_SeparatorError
                                            };
                                            match &v106 {
                                                US3::US3_SeparatorError => {
                                                    US2::US2_InvalidPatch
                                                }
                                                US3::US3_SeparatorOk(v107) => {
                                                    let v107: i32 = *v107;
                                                    let v108: bool = v107 < v2;
                                                    let v116: US1 = if v108 {
                                                        let v109: i32 = usize::try_from(v107).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                        let v110: bool = v109 == 34i32;
                                                        if v110 {
                                                            let v111: i32 = v107.wrapping_add(1i32);
                                                            scan_quoted_4(v0.clone(), v111, v2)
                                                        } else {
                                                            US1::US1_QuotedError
                                                        }
                                                    } else {
                                                        US1::US1_QuotedError
                                                    };
                                                    match &v116 {
                                                        US1::US1_QuotedError => {
                                                            US2::US2_InvalidPatch
                                                        }
                                                        US1::US1_QuotedOk(v117) => {
                                                            let v117: i32 = *v117;
                                                            let v118: i32 = skip_spaces_3(v0.clone(), v117, v2);
                                                            let v119: bool = v118 < v2;
                                                            if v119 {
                                                                let v120: i32 = usize::try_from(v118).ok().and_then(|index| v0.as_bytes().get(index).copied()).map_or(-1, i32::from);
                                                                let v121: bool = v120 == 41i32;
                                                                if v121 {
                                                                    let v122: i32 = v118.wrapping_add(1i32);
                                                                    US2::US2_ParsedPatch(v122)
                                                                } else {
                                                                    US2::US2_InvalidPatch
                                                                }
                                                            } else {
                                                                US2::US2_InvalidPatch
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            US2::US2_InvalidPatch
                        }
                    }
                };
                match &v144 {
                    US2::US2_InvalidPatch => {
                        return US0::US0_PatchPlanMalformed;
                    }
                    US2::US2_ParsedPatch(v145) => {
                        let v145: i32 = *v145;
                        let v146: i32 = v3.wrapping_add(1i32);
                        (v0, v1, v2, v3) = (v0.clone(), v145, v2, v146);
                        continue;
                    }
                }
            } else {
                let v151: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v151, v2, v3);
                continue;
            }
        } else {
            let v154: bool = 0i32 < v3;
            if v154 {
                return US0::US0_PatchPlanValid(v3);
            } else {
                return US0::US0_PatchPlanEmpty;
            }
        }
    }
}
fn patch_gated_entry_1() -> i32 {
    let v0: i32 = 4i32;
    let v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = 2i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: i32 = 3i32;
        let v7: Rc<str> = usize::try_from(v6).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v8: Rc<str> = patch_prepare_source_owned(v3.as_ref(), v5.as_ref());
        let v9: i32 = i32::try_from(v8.len()).unwrap_or(i32::MAX);
        let v10: bool = 0 < v9;
        if v10 {
            let v11: i32 = canonical_plan_ir_domain::plan_ir_manifest_count(&v8);
            let v12: bool = 0i32 < v11;
            let v27: US0 = if v12 {
                US0::US0_PatchPlanValid(v11)
            } else {
                let v14: bool = v11 < 0i32;
                if v14 {
                    US0::US0_PatchPlanMalformed
                } else {
                    let v16: i32 = i32::try_from(v8.len()).unwrap_or(i32::MAX);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        US0::US0_PatchPlanUnavailable
                    } else {
                        let v19: bool = v16 <= 262144i32;
                        if v19 {
                            let v20: i32 = 0i32;
                            let v21: i32 = 0i32;
                            scan_plan_2(v8.clone(), v20, v16, v21)
                        } else {
                            US0::US0_PatchPlanTooLarge
                        }
                    }
                }
            };
            let v33: i32 = match &v27 {
                US0::US0_PatchPlanEmpty => {
                    0i32
                }
                US0::US0_PatchPlanMalformed => {
                    -1i32
                }
                US0::US0_PatchPlanTooLarge => {
                    -2i32
                }
                US0::US0_PatchPlanUnavailable => {
                    -3i32
                }
                US0::US0_PatchPlanValid(v28) => {
                    let v28: i32 = *v28;
                    v28
                }
            };
            let v34: i32 = 0i32;
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gated"); } LIT.with(|lit| lit.clone()) };
            let v36: bool = usize::try_from(v34).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v35.as_ref());
            if v36 {
                let v37: bool = 0 < v33;
                if v37 {
                    let v38: u64 = 30000u64;
                    let v39: i32 = patch_gated_context_owned(v3.as_ref(), v8.as_ref(), v33, v7.as_ref(), v38);
                    v39
                } else {
                    let v40: i32 = patch_plan_error(v33);
                    v40
                }
            } else {
                let v42: i32 = patch_gated_error("patch gated expects: gated <root> <plan.spi> <gate-program>");
                v42
            }
        } else {
            2i32
        }
    } else {
        let v45: i32 = patch_gated_error("patch gated expects: gated <root> <plan.spi> <gate-program>");
        v45
    }
}
fn patch_gated_entry_public_0() -> i32 {
    patch_gated_entry_1()
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        patch_gated_entry_public_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn patch_rehearse_entry_7() -> i32 {
    let v0: i32 = 3i32;
    let v1: bool = i32::try_from(std::env::args().skip(2).count()).unwrap_or(i32::MAX) == v0 ;
    if v1 {
        let v2: i32 = 1i32;
        let v3: Rc<str> = usize::try_from(v2).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v4: i32 = 2i32;
        let v5: Rc<str> = usize::try_from(v4).ok().and_then(|index| std::env::args().skip(2).nth(index)).map_or_else(std::rc::Rc::<str>::default, std::rc::Rc::<str>::from);
        let v6: Rc<str> = patch_prepare_source_owned(v3.as_ref(), v5.as_ref());
        let v7: i32 = i32::try_from(v6.len()).unwrap_or(i32::MAX);
        let v8: bool = 0 < v7;
        if v8 {
            let v9: i32 = canonical_plan_ir_domain::plan_ir_manifest_count(&v6);
            let v10: bool = 0i32 < v9;
            let v25: US0 = if v10 {
                US0::US0_PatchPlanValid(v9)
            } else {
                let v12: bool = v9 < 0i32;
                if v12 {
                    US0::US0_PatchPlanMalformed
                } else {
                    let v14: i32 = i32::try_from(v6.len()).unwrap_or(i32::MAX);
                    let v15: bool = v14 < 0i32;
                    if v15 {
                        US0::US0_PatchPlanUnavailable
                    } else {
                        let v17: bool = v14 <= 262144i32;
                        if v17 {
                            let v18: i32 = 0i32;
                            let v19: i32 = 0i32;
                            scan_plan_2(v6.clone(), v18, v14, v19)
                        } else {
                            US0::US0_PatchPlanTooLarge
                        }
                    }
                }
            };
            let v31: i32 = match &v25 {
                US0::US0_PatchPlanEmpty => {
                    0i32
                }
                US0::US0_PatchPlanMalformed => {
                    -1i32
                }
                US0::US0_PatchPlanTooLarge => {
                    -2i32
                }
                US0::US0_PatchPlanUnavailable => {
                    -3i32
                }
                US0::US0_PatchPlanValid(v26) => {
                    let v26: i32 = *v26;
                    v26
                }
            };
            let v32: i32 = 0i32;
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rehearse"); } LIT.with(|lit| lit.clone()) };
            let v34: bool = usize::try_from(v32).ok().and_then(|index| std::env::args().skip(2).nth(index)).is_some_and(|value| value.as_str() == v33.as_ref());
            if v34 {
                let v35: bool = 0 < v31;
                if v35 {
                    let v36: i32 = patch_rehearse_context_owned(v3.as_ref(), v6.as_ref(), v31);
                    v36
                } else {
                    let v37: i32 = patch_plan_error(v31);
                    v37
                }
            } else {
                let v39: i32 = patch_mutation_error("patch rehearse expects: rehearse <root> <plan.spi>");
                v39
            }
        } else {
            2i32
        }
    } else {
        let v42: i32 = patch_mutation_error("patch rehearse expects: rehearse <root> <plan.spi>");
        v42
    }
}
fn patch_rehearsal_entry_public_6() -> i32 {
    patch_rehearse_entry_7()
}
fn closure1() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        patch_rehearsal_entry_public_6()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_patch_gated_run() -> i32 {
    closure0()()
}
pub fn kit_patch_rehearse_run() -> i32 {
    closure1()()
}
