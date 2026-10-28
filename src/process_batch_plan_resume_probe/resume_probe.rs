#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn spiral_main() -> i32 {
    let v0: i32 = 2i32;
    let v1: i32 = 3i32;
    let v2: i32 = process_batch_plan_domain::kit_batch_plan_advance(v0, v1);
    let v3: i32 = 4i32;
    let v4: i32 = process_batch_plan_domain::kit_batch_plan_advance(v2, v3);
    let v5: i32 = 2i32;
    let v6: i32 = 0i32;
    let v7: i32 = process_batch_plan_domain::kit_batch_matrix_row_policy(v5, v6);
    let v8: i32 = 1i32;
    let v9: i32 = 0i32;
    let v10: i32 = process_batch_plan_domain::kit_batch_matrix_dependency(v8, v9);
    let v11: i32 = 0i32;
    let v12: i32 = 1i32;
    let v13: i32 = process_batch_plan_domain::kit_batch_matrix_observed_accept(v11, v12);
    let v14: i32 = 0i32;
    let v15: i32 = 1i32;
    let v16: i32 = process_batch_plan_domain::kit_batch_matrix_resume_action(v14, v15);
    let v17: i32 = 0i32;
    let v18: i32 = 0i32;
    let v19: i32 = process_batch_plan_domain::kit_batch_matrix_resume_action(v17, v18);
    let v20: bool = v2 == 4i32;
    if v20 {
        let v21: bool = v4 == 5i32;
        if v21 {
            let v22: bool = v7 < 0i32;
            if v22 {
                let v23: bool = v10 == 0i32;
                if v23 {
                    let v24: bool = v13 == 0i32;
                    if v24 {
                        let v25: bool = v16 == 1i32;
                        if v25 {
                            let v26: bool = v19 == 2i32;
                            if v26 {
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
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
