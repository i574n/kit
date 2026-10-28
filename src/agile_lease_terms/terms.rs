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
        US0::US0_Some(v6.clone())
    } else {
        US0::US0_None
    };
    match &v9 {
        US0::US0_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11.clone()
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10.clone()
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
    let mut v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
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
    let v22: Rc<str> = match &v16 {
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
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v22));
    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" policy="); } LIT.with(|lit| lit.clone()) };
    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v24));
    let v26: Rc<str> = lease_policy_entry_4();
    let v27: bool = v26.is_empty() == false;
    let v29: Rc<str> = if v27 {
        v26.clone()
    } else {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bounded"); } LIT.with(|lit| lit.clone()) };
        v28.clone()
    };
    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started="); } LIT.with(|lit| lit.clone()) };
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v30));
    let v32: bool = v1 <= v7;
    let v111: Rc<str> = if v32 {
        let v33: u64 = v7.saturating_sub(v1);
        let v34: u64 = v33.wrapping_div(1000u64);
        let v35: u64 = v34.wrapping_div(3600u64);
        let v36: u64 = v34.wrapping_div(60u64);
        let v37: u64 = v36.wrapping_rem(60u64);
        let v38: u64 = v34.wrapping_rem(60u64);
        let v39: bool = 0u64 < v35;
        let v69: Rc<str> = if v39 {
            let v40: Rc<str> = format_real_7(v35);
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
            let v43: bool = v37 <= 9u64;
            let v48: Rc<str> = if v43 {
                let v44: Rc<str> = format_real_7(v37);
                let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v44));
                v46.clone()
            } else {
                format_real_7(v37)
            };
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v48));
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v50));
            v51.clone()
        } else {
            let v52: bool = 0u64 < v37;
            if v52 {
                let v53: Rc<str> = format_real_7(v37);
                let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v53, v54));
                let v56: bool = v38 <= 9u64;
                let v61: Rc<str> = if v56 {
                    let v57: Rc<str> = format_real_7(v38);
                    let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v57));
                    v59.clone()
                } else {
                    format_real_7(v38)
                };
                let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v61));
                let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
                v64.clone()
            } else {
                let v65: Rc<str> = format_real_7(v38);
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
                v67.clone()
            }
        };
        let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v70));
        v71.clone()
    } else {
        let v72: u64 = v1.saturating_sub(v7);
        let v73: u64 = v72.wrapping_div(1000u64);
        let v74: u64 = v73.wrapping_div(3600u64);
        let v75: u64 = v73.wrapping_div(60u64);
        let v76: u64 = v75.wrapping_rem(60u64);
        let v77: u64 = v73.wrapping_rem(60u64);
        let v78: bool = 0u64 < v74;
        let v108: Rc<str> = if v78 {
            let v79: Rc<str> = format_real_7(v74);
            let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v79, v80));
            let v82: bool = v76 <= 9u64;
            let v87: Rc<str> = if v82 {
                let v83: Rc<str> = format_real_7(v76);
                let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v84, v83));
                v85.clone()
            } else {
                format_real_7(v76)
            };
            let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v87));
            let v89: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v90: Rc<str> = Rc::<str>::from(format!("{}{}", v88, v89));
            v90.clone()
        } else {
            let v91: bool = 0u64 < v76;
            if v91 {
                let v92: Rc<str> = format_real_7(v76);
                let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                let v95: bool = v77 <= 9u64;
                let v100: Rc<str> = if v95 {
                    let v96: Rc<str> = format_real_7(v77);
                    let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v97, v96));
                    v98.clone()
                } else {
                    format_real_7(v77)
                };
                let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v100));
                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                v103.clone()
            } else {
                let v104: Rc<str> = format_real_7(v77);
                let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                v106.clone()
            }
        };
        let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v110: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v108));
        v110.clone()
    };
    let v112: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v111));
    let v113: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at="); } LIT.with(|lit| lit.clone()) };
    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", v112, v113));
    let v115: bool = v2 <= v7;
    let v194: Rc<str> = if v115 {
        let v116: u64 = v7.saturating_sub(v2);
        let v117: u64 = v116.wrapping_div(1000u64);
        let v118: u64 = v117.wrapping_div(3600u64);
        let v119: u64 = v117.wrapping_div(60u64);
        let v120: u64 = v119.wrapping_rem(60u64);
        let v121: u64 = v117.wrapping_rem(60u64);
        let v122: bool = 0u64 < v118;
        let v152: Rc<str> = if v122 {
            let v123: Rc<str> = format_real_7(v118);
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v123, v124));
            let v126: bool = v120 <= 9u64;
            let v131: Rc<str> = if v126 {
                let v127: Rc<str> = format_real_7(v120);
                let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v129: Rc<str> = Rc::<str>::from(format!("{}{}", v128, v127));
                v129.clone()
            } else {
                format_real_7(v120)
            };
            let v132: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v131));
            let v133: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v134: Rc<str> = Rc::<str>::from(format!("{}{}", v132, v133));
            v134.clone()
        } else {
            let v135: bool = 0u64 < v120;
            if v135 {
                let v136: Rc<str> = format_real_7(v120);
                let v137: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v138: Rc<str> = Rc::<str>::from(format!("{}{}", v136, v137));
                let v139: bool = v121 <= 9u64;
                let v144: Rc<str> = if v139 {
                    let v140: Rc<str> = format_real_7(v121);
                    let v141: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v142: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v140));
                    v142.clone()
                } else {
                    format_real_7(v121)
                };
                let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v138, v144));
                let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v146));
                v147.clone()
            } else {
                let v148: Rc<str> = format_real_7(v121);
                let v149: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v150: Rc<str> = Rc::<str>::from(format!("{}{}", v148, v149));
                v150.clone()
            }
        };
        let v153: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v154: Rc<str> = Rc::<str>::from(format!("{}{}", v152, v153));
        v154.clone()
    } else {
        let v155: u64 = v2.saturating_sub(v7);
        let v156: u64 = v155.wrapping_div(1000u64);
        let v157: u64 = v156.wrapping_div(3600u64);
        let v158: u64 = v156.wrapping_div(60u64);
        let v159: u64 = v158.wrapping_rem(60u64);
        let v160: u64 = v156.wrapping_rem(60u64);
        let v161: bool = 0u64 < v157;
        let v191: Rc<str> = if v161 {
            let v162: Rc<str> = format_real_7(v157);
            let v163: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v164: Rc<str> = Rc::<str>::from(format!("{}{}", v162, v163));
            let v165: bool = v159 <= 9u64;
            let v170: Rc<str> = if v165 {
                let v166: Rc<str> = format_real_7(v159);
                let v167: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v168: Rc<str> = Rc::<str>::from(format!("{}{}", v167, v166));
                v168.clone()
            } else {
                format_real_7(v159)
            };
            let v171: Rc<str> = Rc::<str>::from(format!("{}{}", v164, v170));
            let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v173: Rc<str> = Rc::<str>::from(format!("{}{}", v171, v172));
            v173.clone()
        } else {
            let v174: bool = 0u64 < v159;
            if v174 {
                let v175: Rc<str> = format_real_7(v159);
                let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
                let v178: bool = v160 <= 9u64;
                let v183: Rc<str> = if v178 {
                    let v179: Rc<str> = format_real_7(v160);
                    let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v181: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v179));
                    v181.clone()
                } else {
                    format_real_7(v160)
                };
                let v184: Rc<str> = Rc::<str>::from(format!("{}{}", v177, v183));
                let v185: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v186: Rc<str> = Rc::<str>::from(format!("{}{}", v184, v185));
                v186.clone()
            } else {
                let v187: Rc<str> = format_real_7(v160);
                let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v189: Rc<str> = Rc::<str>::from(format!("{}{}", v187, v188));
                v189.clone()
            }
        };
        let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v193: Rc<str> = Rc::<str>::from(format!("{}{}", v192, v191));
        v193.clone()
    };
    let v195: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v194));
    let v196: bool = v3 <= v7;
    let v275: Rc<str> = if v196 {
        let v197: u64 = v7.saturating_sub(v3);
        let v198: u64 = v197.wrapping_div(1000u64);
        let v199: u64 = v198.wrapping_div(3600u64);
        let v200: u64 = v198.wrapping_div(60u64);
        let v201: u64 = v200.wrapping_rem(60u64);
        let v202: u64 = v198.wrapping_rem(60u64);
        let v203: bool = 0u64 < v199;
        let v233: Rc<str> = if v203 {
            let v204: Rc<str> = format_real_7(v199);
            let v205: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v206: Rc<str> = Rc::<str>::from(format!("{}{}", v204, v205));
            let v207: bool = v201 <= 9u64;
            let v212: Rc<str> = if v207 {
                let v208: Rc<str> = format_real_7(v201);
                let v209: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v210: Rc<str> = Rc::<str>::from(format!("{}{}", v209, v208));
                v210.clone()
            } else {
                format_real_7(v201)
            };
            let v213: Rc<str> = Rc::<str>::from(format!("{}{}", v206, v212));
            let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v215: Rc<str> = Rc::<str>::from(format!("{}{}", v213, v214));
            v215.clone()
        } else {
            let v216: bool = 0u64 < v201;
            if v216 {
                let v217: Rc<str> = format_real_7(v201);
                let v218: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v219: Rc<str> = Rc::<str>::from(format!("{}{}", v217, v218));
                let v220: bool = v202 <= 9u64;
                let v225: Rc<str> = if v220 {
                    let v221: Rc<str> = format_real_7(v202);
                    let v222: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v223: Rc<str> = Rc::<str>::from(format!("{}{}", v222, v221));
                    v223.clone()
                } else {
                    format_real_7(v202)
                };
                let v226: Rc<str> = Rc::<str>::from(format!("{}{}", v219, v225));
                let v227: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v228: Rc<str> = Rc::<str>::from(format!("{}{}", v226, v227));
                v228.clone()
            } else {
                let v229: Rc<str> = format_real_7(v202);
                let v230: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v231: Rc<str> = Rc::<str>::from(format!("{}{}", v229, v230));
                v231.clone()
            }
        };
        let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_ago"); } LIT.with(|lit| lit.clone()) };
        let v235: Rc<str> = Rc::<str>::from(format!("{}{}", v233, v234));
        v235.clone()
    } else {
        let v236: u64 = v3.saturating_sub(v7);
        let v237: u64 = v236.wrapping_div(1000u64);
        let v238: u64 = v237.wrapping_div(3600u64);
        let v239: u64 = v237.wrapping_div(60u64);
        let v240: u64 = v239.wrapping_rem(60u64);
        let v241: u64 = v237.wrapping_rem(60u64);
        let v242: bool = 0u64 < v238;
        let v272: Rc<str> = if v242 {
            let v243: Rc<str> = format_real_7(v238);
            let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v245: Rc<str> = Rc::<str>::from(format!("{}{}", v243, v244));
            let v246: bool = v240 <= 9u64;
            let v251: Rc<str> = if v246 {
                let v247: Rc<str> = format_real_7(v240);
                let v248: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v249: Rc<str> = Rc::<str>::from(format!("{}{}", v248, v247));
                v249.clone()
            } else {
                format_real_7(v240)
            };
            let v252: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v251));
            let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v254: Rc<str> = Rc::<str>::from(format!("{}{}", v252, v253));
            v254.clone()
        } else {
            let v255: bool = 0u64 < v240;
            if v255 {
                let v256: Rc<str> = format_real_7(v240);
                let v257: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
                let v258: Rc<str> = Rc::<str>::from(format!("{}{}", v256, v257));
                let v259: bool = v241 <= 9u64;
                let v264: Rc<str> = if v259 {
                    let v260: Rc<str> = format_real_7(v241);
                    let v261: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v262: Rc<str> = Rc::<str>::from(format!("{}{}", v261, v260));
                    v262.clone()
                } else {
                    format_real_7(v241)
                };
                let v265: Rc<str> = Rc::<str>::from(format!("{}{}", v258, v264));
                let v266: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v267: Rc<str> = Rc::<str>::from(format!("{}{}", v265, v266));
                v267.clone()
            } else {
                let v268: Rc<str> = format_real_7(v241);
                let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
                let v270: Rc<str> = Rc::<str>::from(format!("{}{}", v268, v269));
                v270.clone()
            }
        };
        let v273: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in_"); } LIT.with(|lit| lit.clone()) };
        let v274: Rc<str> = Rc::<str>::from(format!("{}{}", v273, v272));
        v274.clone()
    };
    let v276: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline="); } LIT.with(|lit| lit.clone()) };
    let v277: Rc<str> = Rc::<str>::from(format!("{}{}", v276, v275));
    let v278: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining="); } LIT.with(|lit| lit.clone()) };
    let v279: Rc<str> = Rc::<str>::from(format!("{}{}", v277, v278));
    let v280: u64 = v4.wrapping_div(1000u64);
    let v281: u64 = v280.wrapping_div(3600u64);
    let v282: u64 = v280.wrapping_div(60u64);
    let v283: u64 = v282.wrapping_rem(60u64);
    let v284: u64 = v280.wrapping_rem(60u64);
    let v285: bool = 0u64 < v281;
    let v315: Rc<str> = if v285 {
        let v286: Rc<str> = format_real_7(v281);
        let v287: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let v288: Rc<str> = Rc::<str>::from(format!("{}{}", v286, v287));
        let v289: bool = v283 <= 9u64;
        let v294: Rc<str> = if v289 {
            let v290: Rc<str> = format_real_7(v283);
            let v291: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            let v292: Rc<str> = Rc::<str>::from(format!("{}{}", v291, v290));
            v292.clone()
        } else {
            format_real_7(v283)
        };
        let v295: Rc<str> = Rc::<str>::from(format!("{}{}", v288, v294));
        let v296: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
        let v297: Rc<str> = Rc::<str>::from(format!("{}{}", v295, v296));
        v297.clone()
    } else {
        let v298: bool = 0u64 < v283;
        if v298 {
            let v299: Rc<str> = format_real_7(v283);
            let v300: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("m"); } LIT.with(|lit| lit.clone()) };
            let v301: Rc<str> = Rc::<str>::from(format!("{}{}", v299, v300));
            let v302: bool = v284 <= 9u64;
            let v307: Rc<str> = if v302 {
                let v303: Rc<str> = format_real_7(v284);
                let v304: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v305: Rc<str> = Rc::<str>::from(format!("{}{}", v304, v303));
                v305.clone()
            } else {
                format_real_7(v284)
            };
            let v308: Rc<str> = Rc::<str>::from(format!("{}{}", v301, v307));
            let v309: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let v310: Rc<str> = Rc::<str>::from(format!("{}{}", v308, v309));
            v310.clone()
        } else {
            let v311: Rc<str> = format_real_7(v284);
            let v312: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("s"); } LIT.with(|lit| lit.clone()) };
            let v313: Rc<str> = Rc::<str>::from(format!("{}{}", v311, v312));
            v313.clone()
        }
    };
    let v316: Rc<str> = Rc::<str>::from(format!("{}{}", v279, v315));
    let v319: Rc<str> = if v5 {
        let v317: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v317.clone()
    } else {
        let v318: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v318.clone()
    };
    let v320: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_wrap="); } LIT.with(|lit| lit.clone()) };
    let v321: Rc<str> = Rc::<str>::from(format!("{}{}", v320, v319));
    let v322: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" should_yield="); } LIT.with(|lit| lit.clone()) };
    let v323: Rc<str> = Rc::<str>::from(format!("{}{}", v321, v322));
    let v326: Rc<str> = if v6 {
        let v324: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v324.clone()
    } else {
        let v325: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v325.clone()
    };
    let v327: Rc<str> = Rc::<str>::from(format!("{}{}", v323, v326));
    let v328: Rc<str> = format_real_7(v1);
    let v329: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" started_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v330: Rc<str> = Rc::<str>::from(format!("{}{}", v329, v328));
    let v331: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" wrap_at_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v332: Rc<str> = Rc::<str>::from(format!("{}{}", v330, v331));
    let v333: Rc<str> = format_real_7(v2);
    let v334: Rc<str> = Rc::<str>::from(format!("{}{}", v332, v333));
    let v335: Rc<str> = format_real_7(v3);
    let v336: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" deadline_unix_ms="); } LIT.with(|lit| lit.clone()) };
    let v337: Rc<str> = Rc::<str>::from(format!("{}{}", v336, v335));
    let v338: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" remaining_ms="); } LIT.with(|lit| lit.clone()) };
    let v339: Rc<str> = Rc::<str>::from(format!("{}{}", v337, v338));
    let v340: Rc<str> = format_real_7(v4);
    let v341: Rc<str> = Rc::<str>::from(format!("{}{}", v339, v340));
    let v346: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v25, v195)), v316)), v327)), v334)), v341));
    v346.clone()
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
