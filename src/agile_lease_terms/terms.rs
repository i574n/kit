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
struct Mut0 { l0: Rc<str> }
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
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn lease_effect_entry_0() -> i32 {
    let mut v0: i32 = 1i32;
    let mut v1: Rc<str> = process_argument_1(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v2));
    let v4: i32 = 2i32;
    let v5: Rc<str> = process_argument_1(v4);
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v5));
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: Rc<str> = std::rc::Rc::<str>::from(std::env::args().count().saturating_sub(3).to_string());
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v10);
    let v12: bool = v11 <= 3u64;
    if v12 {
        let v13: i32 = v11 as i32;
        v13
    } else {
        let v14: i32 = 1i32;
        let v15: Rc<str> = process_argument_1(v14);
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v2));
        let v17: i32 = 2i32;
        let v18: Rc<str> = process_argument_1(v17);
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v18)), v2));
        let v21: i32 = 3i32;
        let v22: Rc<str> = process_argument_1(v21);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
        let v24: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v23);
        let v25: bool = v24 <= 3u64;
        if v25 {
            let v26: i32 = v24 as i32;
            v26
        } else {
            let v27: i32 = 1i32;
            let v28: Rc<str> = process_argument_1(v27);
            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2));
            let v30: i32 = 2i32;
            let v31: Rc<str> = process_argument_1(v30);
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v31));
            let v33: u64 = command_spec_domain::kit_command_subcommand_effect_code(&v32);
            let v34: bool = v33 <= 3u64;
            if v34 {
                let v35: i32 = v33 as i32;
                v35
            } else {
                let v36: i32 = 1i32;
                let v37: Rc<str> = process_argument_1(v36);
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v2));
                let v39: i32 = 2i32;
                let v40: Rc<str> = process_argument_1(v39);
                let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v40));
                let v42: u64 = command_spec_domain::kit_command_effect_code(&v37);
                let v43: bool = v42 <= 3u64;
                if v43 {
                    let v44: i32 = v42 as i32;
                    v44
                } else {
                    let v45: i32 = 4u64 as i32;
                    v45
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
    let mut v1: Rc<str> = get_environment_variable_5(v0.clone());
    let mut v2: bool = v1.is_empty() == false;
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease-policy"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = get_environment_variable_5(v4.clone());
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("XDG_CONFIG_HOME"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = get_environment_variable_5(v6.clone());
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("APPDATA"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = get_environment_variable_5(v8.clone());
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("USERPROFILE"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = get_environment_variable_5(v10.clone());
        let v12: bool = v5.is_empty() == false;
        let v36: Rc<str> = if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v13)), v3));
            v15.clone()
        } else {
            let v16: bool = v7.is_empty() == false;
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/"); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v7, v17)), v3));
                v19.clone()
            } else {
                let v20: bool = v9.is_empty() == false;
                if v20 {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/kit/"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v9, v21)), v3));
                    v23.clone()
                } else {
                    let v24: bool = v11.is_empty() == false;
                    if v24 {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/kit/"); } LIT.with(|lit| lit.clone()) };
                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v25)), v3));
                        v27.clone()
                    } else {
                        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("HOME"); } LIT.with(|lit| lit.clone()) };
                        let v29: Rc<str> = get_environment_variable_5(v28.clone());
                        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/.config/kit/"); } LIT.with(|lit| lit.clone()) };
                        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v30)), v3));
                        v32.clone()
                    }
                }
            }
        };
        let v37: Rc<str> = std::fs::read_to_string(&*v36).map(|text| std::rc::Rc::<str>::from(text.trim())).unwrap_or_else(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) });
        v37.clone()
    }
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        lease_policy_entry_4()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let mut v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn lease_status_entry_6(mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool) -> Rc<str> {
    let mut v7: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit agile lease status root="); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v0));
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" phase="); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v16: US1 = if v6 {
        US1::US1_LeaseExpired
    } else {
        if v5 {
            US1::US1_LeaseWrapping
        } else {
            US1::US1_LeaseRunning
        }
    };
    let v20: Rc<str> = match &v16 {
        US1::US1_LeaseExpired => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expired"); } LIT.with(|lit| lit.clone()) };
            v19.clone()
        }
        US1::US1_LeaseRunning => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("running"); } LIT.with(|lit| lit.clone()) };
            v17.clone()
        }
        US1::US1_LeaseWrapping => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wrapping"); } LIT.with(|lit| lit.clone()) };
            v18.clone()
        }
    };
    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v20));
    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" policy="); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
    let v24: Rc<str> = lease_policy_entry_4();
    let v25: bool = v24.is_empty() == false;
    let v27: Rc<str> = if v25 {
        v24.clone()
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bounded"); } LIT.with(|lit| lit.clone()) };
        v26.clone()
    };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started="); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
    let v30: bool = v1 <= v7;
    let v109: Rc<str> = if v30 {
        let v31: u64 = v7.saturating_sub(v1);
        let v32: u64 = v31.wrapping_div(1000u64);
        let v33: u64 = v32.wrapping_div(3600u64);
        let v34: u64 = v32.wrapping_div(60u64);
        let v35: u64 = v34.wrapping_rem(60u64);
        let v36: u64 = v32.wrapping_rem(60u64);
        let v37: bool = 0u64 < v33;
        let v67: Rc<str> = if v37 {
            let v38: Rc<str> = format_real_7(v33);
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
            let v41: bool = v35 <= 9u64;
            let v46: Rc<str> = if v41 {
                let v42: Rc<str> = format_real_7(v35);
                let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
                v44.clone()
            } else {
                format_real_7(v35)
            };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v46));
            let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
            v49.clone()
        } else {
            let v50: bool = 0u64 < v35;
            if v50 {
                let v51: Rc<str> = format_real_7(v35);
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
                let v54: bool = v36 <= 9u64;
                let v59: Rc<str> = if v54 {
                    let v55: Rc<str> = format_real_7(v36);
                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v55));
                    v57.clone()
                } else {
                    format_real_7(v36)
                };
                let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v59));
                let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
                v62.clone()
            } else {
                let v63: Rc<str> = format_real_7(v36);
                let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                v65.clone()
            }
        };
        let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v69: Rc<str> = Rc::<str>::from(format!("{}{}", v67, v68));
        v69.clone()
    } else {
        let v70: u64 = v1.saturating_sub(v7);
        let v71: u64 = v70.wrapping_div(1000u64);
        let v72: u64 = v71.wrapping_div(3600u64);
        let v73: u64 = v71.wrapping_div(60u64);
        let v74: u64 = v73.wrapping_rem(60u64);
        let v75: u64 = v71.wrapping_rem(60u64);
        let v76: bool = 0u64 < v72;
        let v106: Rc<str> = if v76 {
            let v77: Rc<str> = format_real_7(v72);
            let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
            let v80: bool = v74 <= 9u64;
            let v85: Rc<str> = if v80 {
                let v81: Rc<str> = format_real_7(v74);
                let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v81));
                v83.clone()
            } else {
                format_real_7(v74)
            };
            let v86: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v85));
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
            v88.clone()
        } else {
            let v89: bool = 0u64 < v74;
            if v89 {
                let v90: Rc<str> = format_real_7(v74);
                let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                let v93: bool = v75 <= 9u64;
                let v98: Rc<str> = if v93 {
                    let v94: Rc<str> = format_real_7(v75);
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v94));
                    v96.clone()
                } else {
                    format_real_7(v75)
                };
                let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v98));
                let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
                v101.clone()
            } else {
                let v102: Rc<str> = format_real_7(v75);
                let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v102, v103));
                v104.clone()
            }
        };
        let v107: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v108: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v106));
        v108.clone()
    };
    let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v109));
    let v111: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at="); } LIT.with(|lit| lit.clone()) };
    let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v111));
    let v113: bool = v2 <= v7;
    let v192: Rc<str> = if v113 {
        let v114: u64 = v7.saturating_sub(v2);
        let v115: u64 = v114.wrapping_div(1000u64);
        let v116: u64 = v115.wrapping_div(3600u64);
        let v117: u64 = v115.wrapping_div(60u64);
        let v118: u64 = v117.wrapping_rem(60u64);
        let v119: u64 = v115.wrapping_rem(60u64);
        let v120: bool = 0u64 < v116;
        let v150: Rc<str> = if v120 {
            let v121: Rc<str> = format_real_7(v116);
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v123: Rc<str> = Rc::<str>::from(format!("{}{}", v121, v122));
            let v124: bool = v118 <= 9u64;
            let v129: Rc<str> = if v124 {
                let v125: Rc<str> = format_real_7(v118);
                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v126, v125));
                v127.clone()
            } else {
                format_real_7(v118)
            };
            let v130: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v129));
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v131));
            v132.clone()
        } else {
            let v133: bool = 0u64 < v118;
            if v133 {
                let v134: Rc<str> = format_real_7(v118);
                let v135: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v136: Rc<str> = Rc::<str>::from(format!("{}{}", v134, v135));
                let v137: bool = v119 <= 9u64;
                let v142: Rc<str> = if v137 {
                    let v138: Rc<str> = format_real_7(v119);
                    let v139: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v140: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v138));
                    v140.clone()
                } else {
                    format_real_7(v119)
                };
                let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v142));
                let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v144));
                v145.clone()
            } else {
                let v146: Rc<str> = format_real_7(v119);
                let v147: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v148: Rc<str> = Rc::<str>::from(format!("{}{}", v146, v147));
                v148.clone()
            }
        };
        let v151: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v152: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v151));
        v152.clone()
    } else {
        let v153: u64 = v2.saturating_sub(v7);
        let v154: u64 = v153.wrapping_div(1000u64);
        let v155: u64 = v154.wrapping_div(3600u64);
        let v156: u64 = v154.wrapping_div(60u64);
        let v157: u64 = v156.wrapping_rem(60u64);
        let v158: u64 = v154.wrapping_rem(60u64);
        let v159: bool = 0u64 < v155;
        let v189: Rc<str> = if v159 {
            let v160: Rc<str> = format_real_7(v155);
            let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v162: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v161));
            let v163: bool = v157 <= 9u64;
            let v168: Rc<str> = if v163 {
                let v164: Rc<str> = format_real_7(v157);
                let v165: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v166: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v164));
                v166.clone()
            } else {
                format_real_7(v157)
            };
            let v169: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v168));
            let v170: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v171: Rc<str> = Rc::<str>::from(format!("{}{}", v169, v170));
            v171.clone()
        } else {
            let v172: bool = 0u64 < v157;
            if v172 {
                let v173: Rc<str> = format_real_7(v157);
                let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v173, v174));
                let v176: bool = v158 <= 9u64;
                let v181: Rc<str> = if v176 {
                    let v177: Rc<str> = format_real_7(v158);
                    let v178: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v179: Rc<str> = Rc::<str>::from(format!("{}{}", v178, v177));
                    v179.clone()
                } else {
                    format_real_7(v158)
                };
                let v182: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v181));
                let v183: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v184: Rc<str> = Rc::<str>::from(format!("{}{}", v182, v183));
                v184.clone()
            } else {
                let v185: Rc<str> = format_real_7(v158);
                let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v185, v186));
                v187.clone()
            }
        };
        let v190: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v191: Rc<str> = Rc::<str>::from(format!("{}{}", v190, v189));
        v191.clone()
    };
    let v193: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v192));
    let v194: bool = v3 <= v7;
    let v273: Rc<str> = if v194 {
        let v195: u64 = v7.saturating_sub(v3);
        let v196: u64 = v195.wrapping_div(1000u64);
        let v197: u64 = v196.wrapping_div(3600u64);
        let v198: u64 = v196.wrapping_div(60u64);
        let v199: u64 = v198.wrapping_rem(60u64);
        let v200: u64 = v196.wrapping_rem(60u64);
        let v201: bool = 0u64 < v197;
        let v231: Rc<str> = if v201 {
            let v202: Rc<str> = format_real_7(v197);
            let v203: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v204: Rc<str> = Rc::<str>::from(format!("{}{}", v202, v203));
            let v205: bool = v199 <= 9u64;
            let v210: Rc<str> = if v205 {
                let v206: Rc<str> = format_real_7(v199);
                let v207: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v208: Rc<str> = Rc::<str>::from(format!("{}{}", v207, v206));
                v208.clone()
            } else {
                format_real_7(v199)
            };
            let v211: Rc<str> = Rc::<str>::from(format!("{}{}", v204, v210));
            let v212: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v213: Rc<str> = Rc::<str>::from(format!("{}{}", v211, v212));
            v213.clone()
        } else {
            let v214: bool = 0u64 < v199;
            if v214 {
                let v215: Rc<str> = format_real_7(v199);
                let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v217: Rc<str> = Rc::<str>::from(format!("{}{}", v215, v216));
                let v218: bool = v200 <= 9u64;
                let v223: Rc<str> = if v218 {
                    let v219: Rc<str> = format_real_7(v200);
                    let v220: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v221: Rc<str> = Rc::<str>::from(format!("{}{}", v220, v219));
                    v221.clone()
                } else {
                    format_real_7(v200)
                };
                let v224: Rc<str> = Rc::<str>::from(format!("{}{}", v217, v223));
                let v225: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v226: Rc<str> = Rc::<str>::from(format!("{}{}", v224, v225));
                v226.clone()
            } else {
                let v227: Rc<str> = format_real_7(v200);
                let v228: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v229: Rc<str> = Rc::<str>::from(format!("{}{}", v227, v228));
                v229.clone()
            }
        };
        let v232: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v233: Rc<str> = Rc::<str>::from(format!("{}{}", v231, v232));
        v233.clone()
    } else {
        let v234: u64 = v3.saturating_sub(v7);
        let v235: u64 = v234.wrapping_div(1000u64);
        let v236: u64 = v235.wrapping_div(3600u64);
        let v237: u64 = v235.wrapping_div(60u64);
        let v238: u64 = v237.wrapping_rem(60u64);
        let v239: u64 = v235.wrapping_rem(60u64);
        let v240: bool = 0u64 < v236;
        let v270: Rc<str> = if v240 {
            let v241: Rc<str> = format_real_7(v236);
            let v242: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v243: Rc<str> = Rc::<str>::from(format!("{}{}", v241, v242));
            let v244: bool = v238 <= 9u64;
            let v249: Rc<str> = if v244 {
                let v245: Rc<str> = format_real_7(v238);
                let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v247: Rc<str> = Rc::<str>::from(format!("{}{}", v246, v245));
                v247.clone()
            } else {
                format_real_7(v238)
            };
            let v250: Rc<str> = Rc::<str>::from(format!("{}{}", v243, v249));
            let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v252: Rc<str> = Rc::<str>::from(format!("{}{}", v250, v251));
            v252.clone()
        } else {
            let v253: bool = 0u64 < v238;
            if v253 {
                let v254: Rc<str> = format_real_7(v238);
                let v255: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v256: Rc<str> = Rc::<str>::from(format!("{}{}", v254, v255));
                let v257: bool = v239 <= 9u64;
                let v262: Rc<str> = if v257 {
                    let v258: Rc<str> = format_real_7(v239);
                    let v259: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v260: Rc<str> = Rc::<str>::from(format!("{}{}", v259, v258));
                    v260.clone()
                } else {
                    format_real_7(v239)
                };
                let v263: Rc<str> = Rc::<str>::from(format!("{}{}", v256, v262));
                let v264: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v265: Rc<str> = Rc::<str>::from(format!("{}{}", v263, v264));
                v265.clone()
            } else {
                let v266: Rc<str> = format_real_7(v239);
                let v267: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v268: Rc<str> = Rc::<str>::from(format!("{}{}", v266, v267));
                v268.clone()
            }
        };
        let v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v272: Rc<str> = Rc::<str>::from(format!("{}{}", v271, v270));
        v272.clone()
    };
    let v274: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline="); } LIT.with(|lit| lit.clone()) };
    let v275: Rc<str> = Rc::<str>::from(format!("{}{}", v274, v273));
    let v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining="); } LIT.with(|lit| lit.clone()) };
    let v277: Rc<str> = Rc::<str>::from(format!("{}{}", v275, v276));
    let v278: u64 = v4.wrapping_div(1000u64);
    let v279: u64 = v278.wrapping_div(3600u64);
    let v280: u64 = v278.wrapping_div(60u64);
    let v281: u64 = v280.wrapping_rem(60u64);
    let v282: u64 = v278.wrapping_rem(60u64);
    let v283: bool = 0u64 < v279;
    let v313: Rc<str> = if v283 {
        let v284: Rc<str> = format_real_7(v279);
        let v285: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let v286: Rc<str> = Rc::<str>::from(format!("{}{}", v284, v285));
        let v287: bool = v281 <= 9u64;
        let v292: Rc<str> = if v287 {
            let v288: Rc<str> = format_real_7(v281);
            let v289: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            let v290: Rc<str> = Rc::<str>::from(format!("{}{}", v289, v288));
            v290.clone()
        } else {
            format_real_7(v281)
        };
        let v293: Rc<str> = Rc::<str>::from(format!("{}{}", v286, v292));
        let v294: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v295: Rc<str> = Rc::<str>::from(format!("{}{}", v293, v294));
        v295.clone()
    } else {
        let v296: bool = 0u64 < v281;
        if v296 {
            let v297: Rc<str> = format_real_7(v281);
            let v298: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v299: Rc<str> = Rc::<str>::from(format!("{}{}", v297, v298));
            let v300: bool = v282 <= 9u64;
            let v305: Rc<str> = if v300 {
                let v301: Rc<str> = format_real_7(v282);
                let v302: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v303: Rc<str> = Rc::<str>::from(format!("{}{}", v302, v301));
                v303.clone()
            } else {
                format_real_7(v282)
            };
            let v306: Rc<str> = Rc::<str>::from(format!("{}{}", v299, v305));
            let v307: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let v308: Rc<str> = Rc::<str>::from(format!("{}{}", v306, v307));
            v308.clone()
        } else {
            let v309: Rc<str> = format_real_7(v282);
            let v310: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let v311: Rc<str> = Rc::<str>::from(format!("{}{}", v309, v310));
            v311.clone()
        }
    };
    let v314: Rc<str> = Rc::<str>::from(format!("{}{}", v277, v313));
    let v317: Rc<str> = if v5 {
        let v315: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v315.clone()
    } else {
        let v316: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v316.clone()
    };
    let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_wrap="); } LIT.with(|lit| lit.clone()) };
    let v319: Rc<str> = Rc::<str>::from(format!("{}{}", v318, v317));
    let v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_yield="); } LIT.with(|lit| lit.clone()) };
    let v321: Rc<str> = Rc::<str>::from(format!("{}{}", v319, v320));
    let v324: Rc<str> = if v6 {
        let v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v322.clone()
    } else {
        let v323: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v323.clone()
    };
    let v325: Rc<str> = Rc::<str>::from(format!("{}{}", v321, v324));
    let v326: Rc<str> = format_real_7(v1);
    let v327: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v328: Rc<str> = Rc::<str>::from(format!("{}{}", v327, v326));
    let v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v330: Rc<str> = Rc::<str>::from(format!("{}{}", v328, v329));
    let v331: Rc<str> = format_real_7(v2);
    let v332: Rc<str> = Rc::<str>::from(format!("{}{}", v330, v331));
    let v333: Rc<str> = format_real_7(v3);
    let v334: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v335: Rc<str> = Rc::<str>::from(format!("{}{}", v334, v333));
    let v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining_ms="); } LIT.with(|lit| lit.clone()) };
    let v337: Rc<str> = Rc::<str>::from(format!("{}{}", v335, v336));
    let v338: Rc<str> = format_real_7(v4);
    let v339: Rc<str> = Rc::<str>::from(format!("{}{}", v337, v338));
    let v344: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v193)), v314)), v325)), v332)), v339));
    v344.clone()
}
fn closure2() -> Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, u64, u64, u64, u64, bool, bool) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool| -> Rc<str> {
        lease_status_entry_6(v0.clone(), v1, v2, v3, v4, v5, v6)
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
