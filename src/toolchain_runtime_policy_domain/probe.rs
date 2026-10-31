#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn toolchain_execution_gate_binding_0(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v1 < 0i32;
    if v2 {
        -1i32
    } else {
        let v3: bool = 0i32 < v1;
        if v3 {
            -1i32
        } else {
            let v4: bool = v0 < 0i32;
            if v4 {
                -1i32
            } else {
                let v5: bool = 5i32 < v0;
                if v5 {
                    -1i32
                } else {
                    let v6: bool = v0 == 1i32;
                    if v6 {
                        1i32
                    } else {
                        0i32
                    }
                }
            }
        }
    }
}
fn toolchain_observed_outcome_binding_1(mut v0: i32, mut v1: i32) -> i32 {
    let v2: bool = v0 < 0i32;
    if v2 {
        -1i32
    } else {
        let v3: bool = 1i32 < v0;
        if v3 {
            -1i32
        } else {
            let v4: bool = v1 < 0i32;
            if v4 {
                -1i32
            } else {
                let v5: bool = 1i32 < v1;
                if v5 {
                    -1i32
                } else {
                    let v6: bool = v0 == 1i32;
                    if v6 {
                        3i32
                    } else {
                        let v7: bool = v1 == 1i32;
                        if v7 {
                            1i32
                        } else {
                            2i32
                        }
                    }
                }
            }
        }
    }
}
fn spiral_main() -> i32 {
    let v0: i32 = 1i32;
    let v1: i32 = 0i32;
    let v2: i32 = toolchain_execution_gate_binding_0(v0, v1);
    let v3: i32 = 2i32;
    let v4: i32 = 0i32;
    let v5: i32 = toolchain_execution_gate_binding_0(v3, v4);
    let v6: i32 = 6i32;
    let v7: i32 = 0i32;
    let v8: i32 = toolchain_execution_gate_binding_0(v6, v7);
    let v9: i32 = 1i32;
    let v10: i32 = 0i32;
    let v11: i32 = toolchain_observed_outcome_binding_1(v9, v10);
    let v12: i32 = 0i32;
    let v13: i32 = 1i32;
    let v14: i32 = toolchain_observed_outcome_binding_1(v12, v13);
    let v15: i32 = 0i32;
    let v16: i32 = 0i32;
    let v17: i32 = toolchain_observed_outcome_binding_1(v15, v16);
    let v18: i32 = 2i32;
    let v19: i32 = 0i32;
    let v20: i32 = toolchain_observed_outcome_binding_1(v18, v19);
    let v21: bool = v2 == 1i32;
    let v23: bool = if v21 {
        let v22: bool = v5 == 0i32;
        v22
    } else {
        false
    };
    let v25: bool = if v23 {
        let v24: bool = v8 < 0i32;
        v24
    } else {
        false
    };
    let v27: bool = if v25 {
        let v26: bool = v11 == 3i32;
        v26
    } else {
        false
    };
    let v29: bool = if v27 {
        let v28: bool = v14 == 1i32;
        v28
    } else {
        false
    };
    let v31: bool = if v29 {
        let v30: bool = v17 == 2i32;
        v30
    } else {
        false
    };
    let v33: bool = if v31 {
        let v32: bool = v20 < 0i32;
        v32
    } else {
        false
    };
    if v33 {
        0i32
    } else {
        70i32
    }
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
