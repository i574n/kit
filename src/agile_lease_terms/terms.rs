#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_LeaseRunning,
    US1_LeaseWrapping,
    US1_LeaseExpired,
}
pub struct Mut0 { pub l0: Rc<str> }
fn command_line_argument_count_2() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_3(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn process_argument_1(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_sub(1i32);
    let mut v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_2();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_3(v1);
        US0::US0_Some(v6)
    } else {
        US0::US0_None
    };
    match v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            v10
        }
    }
}
fn lease_effect_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = process_argument_1(v0);
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
    let mut v3: i32 = 2i32;
    let v4: Rc<str> = process_argument_1(v3);
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v4));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
    let mut v7: Rc<str> = std::rc::Rc::<str>::from(std::env::args().count().saturating_sub(3).to_string());
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v8);
    let v10: bool = v9 <= 3u64;
    if v10 {
        let v11: i32 = v9 as i32;
        v11
    } else {
        let v12: i32 = 1i32;
        let v13: Rc<str> = process_argument_1(v12);
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
        let v15: i32 = 2i32;
        let v16: Rc<str> = process_argument_1(v15);
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v16));
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
        let v19: i32 = 3i32;
        let v20: Rc<str> = process_argument_1(v19);
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
        let v22: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v21);
        let v23: bool = v22 <= 3u64;
        if v23 {
            let v24: i32 = v22 as i32;
            v24
        } else {
            let v25: i32 = 1i32;
            let v26: Rc<str> = process_argument_1(v25);
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) }));
            let v28: i32 = 2i32;
            let v29: Rc<str> = process_argument_1(v28);
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v29));
            let v31: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v30);
            let v32: bool = v31 <= 3u64;
            if v32 {
                let v33: i32 = v31 as i32;
                v33
            } else {
                let v34: i32 = 1i32;
                let v35: Rc<str> = process_argument_1(v34);
                let v37: i32 = 2i32;
                let v38: Rc<str> = process_argument_1(v37);
                let v40: u64 = command_spec_domain::kit_command_effect_code(&v35);
                let v41: bool = v40 <= 3u64;
                if v41 {
                    let v42: i32 = v40 as i32;
                    v42
                } else {
                    let v43: i32 = 4u64 as i32;
                    v43
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        lease_effect_entry_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_5(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn lease_policy_entry_4() -> Rc<str> {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_LEASE_POLICY"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_5(v0);
    let mut v2: bool = v1.is_empty() == false;
    if v2 {
        v1
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease-policy"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = get_environment_variable_5(v4);
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("XDG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = get_environment_variable_5(v6);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = get_environment_variable_5(v8);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = get_environment_variable_5(v10);
        let v12: bool = v5.is_empty() == false;
        let v31: Rc<str> = if v12 {
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v5, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v3));
            v14
        } else {
            let v15: bool = v7.is_empty() == false;
            if v15 {
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/"); } LIT.with(|lit| lit.clone()) }));
                let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v3));
                v17
            } else {
                let v18: bool = v9.is_empty() == false;
                if v18 {
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v9, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/"); } LIT.with(|lit| lit.clone()) }));
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v3));
                    v20
                } else {
                    let v21: bool = v11.is_empty() == false;
                    if v21 {
                        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/kit/"); } LIT.with(|lit| lit.clone()) }));
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v3));
                        v23
                    } else {
                        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
                        let v25: Rc<str> = get_environment_variable_5(v24);
                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/kit/"); } LIT.with(|lit| lit.clone()) }));
                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v3));
                        v27
                    }
                }
            }
        };
        let v32: Rc<str> = std::fs::read_to_string(&*v31).map(|text| std::rc::Rc::<str>::from(text.trim())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        v32
    }
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        lease_policy_entry_4()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn lease_status_entry_6(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool) -> Rc<str> {
    let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile lease status root="); } LIT.with(|lit| lit.clone()) }, v0));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" phase="); } LIT.with(|lit| lit.clone()) }));
    let v14: US1 = if v6 {
        US1::US1_LeaseExpired
    } else {
        if v5 {
            US1::US1_LeaseWrapping
        } else {
            US1::US1_LeaseRunning
        }
    };
    let v18: Rc<str> = match v14 {
        US1::US1_LeaseExpired => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired"); } LIT.with(|lit| lit.clone()) };
            v17
        }
        US1::US1_LeaseRunning => {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v15
        }
        US1::US1_LeaseWrapping => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrapping"); } LIT.with(|lit| lit.clone()) };
            v16
        }
    };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v18));
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" policy="); } LIT.with(|lit| lit.clone()) }));
    let v21: Rc<str> = lease_policy_entry_4();
    let v22: bool = v21.is_empty() == false;
    let v24: Rc<str> = if v22 {
        v21
    } else {
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bounded"); } LIT.with(|lit| lit.clone()) };
        v23
    };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started="); } LIT.with(|lit| lit.clone()) }));
    let v26: bool = v1 <= v7;
    let v89: Rc<str> = if v26 {
        let v27: u64 = v7.saturating_sub(v1);
        let v28: u64 = v27.wrapping_div(1000u64);
        let v29: u64 = v28.wrapping_div(3600u64);
        let v30: u64 = v28.wrapping_div(60u64);
        let v31: u64 = v30.wrapping_rem(60u64);
        let v32: u64 = v28.wrapping_rem(60u64);
        let v33: bool = 0u64 < v29;
        let v56: Rc<str> = if v33 {
            let v34: Rc<str> = format_real_7(v29);
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v36: bool = v31 <= 9u64;
            let v40: Rc<str> = if v36 {
                let v37: Rc<str> = format_real_7(v31);
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v37));
                v38
            } else {
                format_real_7(v31)
            };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v40));
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v42
        } else {
            let v43: bool = 0u64 < v31;
            if v43 {
                let v44: Rc<str> = format_real_7(v31);
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v46: bool = v32 <= 9u64;
                let v50: Rc<str> = if v46 {
                    let v47: Rc<str> = format_real_7(v32);
                    let v48: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v47));
                    v48
                } else {
                    format_real_7(v32)
                };
                let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v50));
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v52
            } else {
                let v53: Rc<str> = format_real_7(v32);
                let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v53, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v54
            }
        };
        let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) }));
        v57
    } else {
        let v58: u64 = v1.saturating_sub(v7);
        let v59: u64 = v58.wrapping_div(1000u64);
        let v60: u64 = v59.wrapping_div(3600u64);
        let v61: u64 = v59.wrapping_div(60u64);
        let v62: u64 = v61.wrapping_rem(60u64);
        let v63: u64 = v59.wrapping_rem(60u64);
        let v64: bool = 0u64 < v60;
        let v87: Rc<str> = if v64 {
            let v65: Rc<str> = format_real_7(v60);
            let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v65, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v67: bool = v62 <= 9u64;
            let v71: Rc<str> = if v67 {
                let v68: Rc<str> = format_real_7(v62);
                let v69: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v68));
                v69
            } else {
                format_real_7(v62)
            };
            let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v71));
            let v73: Rc<str> = Rc::<str>::from(format!("{}{}", v72, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v73
        } else {
            let v74: bool = 0u64 < v62;
            if v74 {
                let v75: Rc<str> = format_real_7(v62);
                let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v75, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v77: bool = v63 <= 9u64;
                let v81: Rc<str> = if v77 {
                    let v78: Rc<str> = format_real_7(v63);
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v78));
                    v79
                } else {
                    format_real_7(v63)
                };
                let v82: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v81));
                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v83
            } else {
                let v84: Rc<str> = format_real_7(v63);
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v85
            }
        };
        let v88: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) }, v87));
        v88
    };
    let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v89));
    let v91: Rc<str> = Rc::<str>::from(format!("{}{}", v90, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at="); } LIT.with(|lit| lit.clone()) }));
    let v92: bool = v2 <= v7;
    let v155: Rc<str> = if v92 {
        let v93: u64 = v7.saturating_sub(v2);
        let v94: u64 = v93.wrapping_div(1000u64);
        let v95: u64 = v94.wrapping_div(3600u64);
        let v96: u64 = v94.wrapping_div(60u64);
        let v97: u64 = v96.wrapping_rem(60u64);
        let v98: u64 = v94.wrapping_rem(60u64);
        let v99: bool = 0u64 < v95;
        let v122: Rc<str> = if v99 {
            let v100: Rc<str> = format_real_7(v95);
            let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v100, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v102: bool = v97 <= 9u64;
            let v106: Rc<str> = if v102 {
                let v103: Rc<str> = format_real_7(v97);
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v103));
                v104
            } else {
                format_real_7(v97)
            };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v106));
            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v107, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v108
        } else {
            let v109: bool = 0u64 < v97;
            if v109 {
                let v110: Rc<str> = format_real_7(v97);
                let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v110, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v112: bool = v98 <= 9u64;
                let v116: Rc<str> = if v112 {
                    let v113: Rc<str> = format_real_7(v98);
                    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v113));
                    v114
                } else {
                    format_real_7(v98)
                };
                let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v111, v116));
                let v118: Rc<str> = Rc::<str>::from(format!("{}{}", v117, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v118
            } else {
                let v119: Rc<str> = format_real_7(v98);
                let v120: Rc<str> = Rc::<str>::from(format!("{}{}", v119, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v120
            }
        };
        let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v122, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) }));
        v123
    } else {
        let v124: u64 = v2.saturating_sub(v7);
        let v125: u64 = v124.wrapping_div(1000u64);
        let v126: u64 = v125.wrapping_div(3600u64);
        let v127: u64 = v125.wrapping_div(60u64);
        let v128: u64 = v127.wrapping_rem(60u64);
        let v129: u64 = v125.wrapping_rem(60u64);
        let v130: bool = 0u64 < v126;
        let v153: Rc<str> = if v130 {
            let v131: Rc<str> = format_real_7(v126);
            let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v131, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v133: bool = v128 <= 9u64;
            let v137: Rc<str> = if v133 {
                let v134: Rc<str> = format_real_7(v128);
                let v135: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v134));
                v135
            } else {
                format_real_7(v128)
            };
            let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v137));
            let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v138, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v139
        } else {
            let v140: bool = 0u64 < v128;
            if v140 {
                let v141: Rc<str> = format_real_7(v128);
                let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v143: bool = v129 <= 9u64;
                let v147: Rc<str> = if v143 {
                    let v144: Rc<str> = format_real_7(v129);
                    let v145: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v144));
                    v145
                } else {
                    format_real_7(v129)
                };
                let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v142, v147));
                let v149: Rc<str> = Rc::<str>::from(format!("{}{}", v148, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v149
            } else {
                let v150: Rc<str> = format_real_7(v129);
                let v151: Rc<str> = Rc::<str>::from(format!("{}{}", v150, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v151
            }
        };
        let v154: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) }, v153));
        v154
    };
    let v156: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v155));
    let v157: bool = v3 <= v7;
    let v220: Rc<str> = if v157 {
        let v158: u64 = v7.saturating_sub(v3);
        let v159: u64 = v158.wrapping_div(1000u64);
        let v160: u64 = v159.wrapping_div(3600u64);
        let v161: u64 = v159.wrapping_div(60u64);
        let v162: u64 = v161.wrapping_rem(60u64);
        let v163: u64 = v159.wrapping_rem(60u64);
        let v164: bool = 0u64 < v160;
        let v187: Rc<str> = if v164 {
            let v165: Rc<str> = format_real_7(v160);
            let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v167: bool = v162 <= 9u64;
            let v171: Rc<str> = if v167 {
                let v168: Rc<str> = format_real_7(v162);
                let v169: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v168));
                v169
            } else {
                format_real_7(v162)
            };
            let v172: Rc<str> = Rc::<str>::from(format!("{}{}", v166, v171));
            let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v172, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v173
        } else {
            let v174: bool = 0u64 < v162;
            if v174 {
                let v175: Rc<str> = format_real_7(v162);
                let v176: Rc<str> = Rc::<str>::from(format!("{}{}", v175, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v177: bool = v163 <= 9u64;
                let v181: Rc<str> = if v177 {
                    let v178: Rc<str> = format_real_7(v163);
                    let v179: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v178));
                    v179
                } else {
                    format_real_7(v163)
                };
                let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v176, v181));
                let v183: Rc<str> = Rc::<str>::from(format!("{}{}", v182, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v183
            } else {
                let v184: Rc<str> = format_real_7(v163);
                let v185: Rc<str> = Rc::<str>::from(format!("{}{}", v184, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v185
            }
        };
        let v188: Rc<str> = Rc::<str>::from(format!("{}{}", v187, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) }));
        v188
    } else {
        let v189: u64 = v3.saturating_sub(v7);
        let v190: u64 = v189.wrapping_div(1000u64);
        let v191: u64 = v190.wrapping_div(3600u64);
        let v192: u64 = v190.wrapping_div(60u64);
        let v193: u64 = v192.wrapping_rem(60u64);
        let v194: u64 = v190.wrapping_rem(60u64);
        let v195: bool = 0u64 < v191;
        let v218: Rc<str> = if v195 {
            let v196: Rc<str> = format_real_7(v191);
            let v197: Rc<str> = Rc::<str>::from(format!("{}{}", v196, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
            let v198: bool = v193 <= 9u64;
            let v202: Rc<str> = if v198 {
                let v199: Rc<str> = format_real_7(v193);
                let v200: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v199));
                v200
            } else {
                format_real_7(v193)
            };
            let v203: Rc<str> = Rc::<str>::from(format!("{}{}", v197, v202));
            let v204: Rc<str> = Rc::<str>::from(format!("{}{}", v203, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            v204
        } else {
            let v205: bool = 0u64 < v193;
            if v205 {
                let v206: Rc<str> = format_real_7(v193);
                let v207: Rc<str> = Rc::<str>::from(format!("{}{}", v206, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
                let v208: bool = v194 <= 9u64;
                let v212: Rc<str> = if v208 {
                    let v209: Rc<str> = format_real_7(v194);
                    let v210: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v209));
                    v210
                } else {
                    format_real_7(v194)
                };
                let v213: Rc<str> = Rc::<str>::from(format!("{}{}", v207, v212));
                let v214: Rc<str> = Rc::<str>::from(format!("{}{}", v213, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v214
            } else {
                let v215: Rc<str> = format_real_7(v194);
                let v216: Rc<str> = Rc::<str>::from(format!("{}{}", v215, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
                v216
            }
        };
        let v219: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) }, v218));
        v219
    };
    let v221: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline="); } LIT.with(|lit| lit.clone()) }, v220));
    let v222: Rc<str> = Rc::<str>::from(format!("{}{}", v221, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining="); } LIT.with(|lit| lit.clone()) }));
    let v223: u64 = v4.wrapping_div(1000u64);
    let v224: u64 = v223.wrapping_div(3600u64);
    let v225: u64 = v223.wrapping_div(60u64);
    let v226: u64 = v225.wrapping_rem(60u64);
    let v227: u64 = v223.wrapping_rem(60u64);
    let v228: bool = 0u64 < v224;
    let v251: Rc<str> = if v228 {
        let v229: Rc<str> = format_real_7(v224);
        let v230: Rc<str> = Rc::<str>::from(format!("{}{}", v229, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) }));
        let v231: bool = v226 <= 9u64;
        let v235: Rc<str> = if v231 {
            let v232: Rc<str> = format_real_7(v226);
            let v233: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v232));
            v233
        } else {
            format_real_7(v226)
        };
        let v236: Rc<str> = Rc::<str>::from(format!("{}{}", v230, v235));
        let v237: Rc<str> = Rc::<str>::from(format!("{}{}", v236, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
        v237
    } else {
        let v238: bool = 0u64 < v226;
        if v238 {
            let v239: Rc<str> = format_real_7(v226);
            let v240: Rc<str> = Rc::<str>::from(format!("{}{}", v239, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) }));
            let v241: bool = v227 <= 9u64;
            let v245: Rc<str> = if v241 {
                let v242: Rc<str> = format_real_7(v227);
                let v243: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) }, v242));
                v243
            } else {
                format_real_7(v227)
            };
            let v246: Rc<str> = Rc::<str>::from(format!("{}{}", v240, v245));
            let v247: Rc<str> = Rc::<str>::from(format!("{}{}", v246, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
            v247
        } else {
            let v248: Rc<str> = format_real_7(v227);
            let v249: Rc<str> = Rc::<str>::from(format!("{}{}", v248, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) }));
            v249
        }
    };
    let v252: Rc<str> = Rc::<str>::from(format!("{}{}", v222, v251));
    let v255: Rc<str> = if v5 {
        let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v253
    } else {
        let v254: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v254
    };
    let v256: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_wrap="); } LIT.with(|lit| lit.clone()) }, v255));
    let v257: Rc<str> = Rc::<str>::from(format!("{}{}", v256, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_yield="); } LIT.with(|lit| lit.clone()) }));
    let v260: Rc<str> = if v6 {
        let v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v258
    } else {
        let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v259
    };
    let v261: Rc<str> = Rc::<str>::from(format!("{}{}", v257, v260));
    let v262: Rc<str> = format_real_7(v1);
    let v263: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started_unix_ms="); } LIT.with(|lit| lit.clone()) }, v262));
    let v264: Rc<str> = Rc::<str>::from(format!("{}{}", v263, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at_unix_ms="); } LIT.with(|lit| lit.clone()) }));
    let v265: Rc<str> = format_real_7(v2);
    let v266: Rc<str> = Rc::<str>::from(format!("{}{}", v264, v265));
    let v267: Rc<str> = format_real_7(v3);
    let v268: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline_unix_ms="); } LIT.with(|lit| lit.clone()) }, v267));
    let v269: Rc<str> = Rc::<str>::from(format!("{}{}", v268, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining_ms="); } LIT.with(|lit| lit.clone()) }));
    let v270: Rc<str> = format_real_7(v4);
    let v271: Rc<str> = Rc::<str>::from(format!("{}{}", v269, v270));
    let v272: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v156));
    let v273: Rc<str> = Rc::<str>::from(format!("{}{}", v272, v252));
    let v274: Rc<str> = Rc::<str>::from(format!("{}{}", v273, v261));
    let v275: Rc<str> = Rc::<str>::from(format!("{}{}", v274, v266));
    let v276: Rc<str> = Rc::<str>::from(format!("{}{}", v275, v271));
    v276
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool| -> Rc<str> {
        lease_status_entry_6(v0, v1, v2, v3, v4, v5, v6)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_lease_effect_code() -> i32 {
    closure0()()
}
pub fn kit_lease_policy_text() -> Rc<str> {
    closure1()()
}
pub fn kit_lease_status_text(v0: &str, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: bool) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), v1, v2, v3, v4, v5, v6)
}
pub type SpiralShape_97df63ebabc3 = Mut0;
