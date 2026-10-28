#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let v0: i32 = 0i32;
    let v1: i32 = 0i32;
    let v2: i32 = process_batch_plan_domain::kit_batch_plan_advance(v0, v1);
    let v3: i32 = 1i32;
    let v4: i32 = process_batch_plan_domain::kit_batch_plan_advance(v2, v3);
    let v5: i32 = 2i32;
    let v6: i32 = process_batch_plan_domain::kit_batch_plan_advance(v4, v5);
    let v7: i32 = 4i32;
    let v8: i32 = process_batch_plan_domain::kit_batch_plan_advance(v6, v7);
    let v9: i32 = 0i32;
    let v10: i32 = 0i32;
    let v11: i32 = process_batch_plan_domain::kit_batch_matrix_row_policy(v9, v10);
    let v12: i32 = 1i32;
    let v13: i32 = 1i32;
    let v14: i32 = process_batch_plan_domain::kit_batch_matrix_row_policy(v12, v13);
    let v15: i32 = 0i32;
    let v16: i32 = 0i32;
    let v17: i32 = process_batch_plan_domain::kit_batch_matrix_dependency(v15, v16);
    let v18: i32 = 1i32;
    let v19: i32 = 1i32;
    let v20: i32 = process_batch_plan_domain::kit_batch_matrix_dependency(v18, v19);
    let v21: i32 = 0i32;
    let v22: i32 = 0i32;
    let v23: i32 = process_batch_plan_domain::kit_batch_matrix_observed_accept(v21, v22);
    let v24: i32 = 1i32;
    let v25: i32 = 1i32;
    let v26: i32 = process_batch_plan_domain::kit_batch_matrix_observed_accept(v24, v25);
    let v27: i32 = 1i32;
    let v28: i32 = 0i32;
    let v29: i32 = process_batch_plan_domain::kit_batch_matrix_resume_action(v27, v28);
    let v30: i32 = 2i32;
    let v31: i32 = 1i32;
    let v32: i32 = process_batch_plan_domain::kit_batch_matrix_workspace_policy(v30, v31);
    let v33: i32 = 3i32;
    let v34: i32 = 1i32;
    let v35: i32 = process_batch_plan_domain::kit_batch_matrix_workspace_policy(v33, v34);
    let v36: i32 = 0i32;
    let v37: i32 = 1i32;
    let v38: i32 = process_batch_plan_domain::kit_batch_matrix_workspace_policy(v36, v37);
    let v39: i32 = 4i32;
    let v40: i32 = 1i32;
    let v41: i32 = process_batch_plan_domain::kit_batch_matrix_workspace_policy(v39, v40);
    let v42: bool = v2 == 1i32;
    if v42 {
        let v43: bool = v4 == 2i32;
        if v43 {
            let v44: bool = v6 == 3i32;
            if v44 {
                let v45: bool = v8 == 5i32;
                if v45 {
                    let v46: bool = v11 == 1i32;
                    if v46 {
                        let v47: bool = v14 == 1i32;
                        if v47 {
                            let v48: bool = v17 == 1i32;
                            if v48 {
                                let v49: bool = v20 == 1i32;
                                if v49 {
                                    let v50: bool = v23 == 1i32;
                                    if v50 {
                                        let v51: bool = v26 == 1i32;
                                        if v51 {
                                            let v52: bool = v29 == 0i32;
                                            if v52 {
                                                let v53: bool = v32 == 1i32;
                                                if v53 {
                                                    let v54: bool = v35 == 0i32;
                                                    if v54 {
                                                        let v55: bool = v38 == 0i32;
                                                        if v55 {
                                                            let v56: bool = v41 < 0i32;
                                                            if v56 {
                                                                0i32
                                                            } else {
                                                                1i32
                                                            }
                                                        } else {
                                                            1i32
                                                        }
                                                    } else {
                                                        1i32
                                                    }
                                                } else {
                                                    1i32
                                                }
                                            } else {
                                                1i32
                                            }
                                        } else {
                                            1i32
                                        }
                                    } else {
                                        1i32
                                    }
                                } else {
                                    1i32
                                }
                            } else {
                                1i32
                            }
                        } else {
                            1i32
                        }
                    } else {
                        1i32
                    }
                } else {
                    1i32
                }
            } else {
                1i32
            }
        } else {
            1i32
        }
    } else {
        1i32
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
