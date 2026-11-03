#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US0 {
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_ClaudeSession(u64),
    US2_KitJob(Rc<str>),
    US2_CellsVm(u64),
    US2_JobLeftover(Rc<str>),
    US2_Orphan,
    US2_NotATool,
}
#[derive(Clone)]
enum US3 {
    US3_Healthy,
    US3_Unowned,
    US3_Dangling,
    US3_Leftover,
    US3_Hog,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_3();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_4(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_2(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn value_in_5(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v5: bool = &*v3 == &*v1;
                if v5 {
                    match &*v4 {
                        UH0::UH0_Cons(v6, v7) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<UH0> = v7.clone();
                            return v6.clone();
                        }
                        UH0::UH0_Nil => {
                            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            return v8.clone();
                        }
                    }
                } else {
                    v0 = v4.clone();
                    continue;
                }
            }
            UH0::UH0_Nil => {
                let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return v2.clone();
            }
        }
    }
}
fn method6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let v11: bool = if v5 {
                true
            } else {
                let v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                v1 = v3;
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn method8(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let mut v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn free_bytes_on_9(mut v0: std::rc::Rc<sysinfo::Disks>, mut v1: Rc<str>, mut v2: i32) -> u64 {
    loop {
        let v3: usize = ((v2) as usize);
        let v4: usize = v0.list().len();
        let v5: i32 = ((v4) as i32);
        let v6: bool = v2 >= v5;
        if v6 {
            return 0u64;
        } else {
            let v7: bool = v0.list()[v3.clone()].mount_point().eq(std::path::Path::new(&*v1));
            if v7 {
                let v8: u64 = v0.list()[v3].available_space();
                return v8;
            } else {
                let v9: i32 = v2.wrapping_add(1i32);
                v2 = v9;
                continue;
            }
        }
    }
}
fn process_rows_sampled_10(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let mut v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn cores_sum_11(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: f64) -> f64 {
    loop {
        let v3: i32 = v0.len() as i32;
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: i32 = v1.wrapping_add(1i32);
            let v6: f64 = v0[v1 as usize].4;
            let v7: f64 = v2 + v6;
            (v1, v2) = (v5, v7);
            continue;
        }
    }
}
fn gate_run_1() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-disk-gb"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = value_in_5(v1.clone(), v2.clone());
    let v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let v6: i32 = method6(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method7(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let v12: Option<u64> = (v11).trim().parse::<u64>().ok();
    let v13: Option<u64> = method8(v12.clone());
    let mut v14: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v15: Option<US1> = v13.map(|x| v14(x));
    let v17: US1 = US1::US1_None;
    let v18: US1 = v17.clone();
    let v19: US1 = v15.unwrap_or(v18);
    let v22: u64 = match &v19 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v20) => {
            let v20: u64 = *v20;
            v20
        }
    };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--min-free-mb"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = value_in_5(v1.clone(), v23.clone());
    let v25: i32 = (v24.clone().len() as i32);
    let v26: i32 = 0i32;
    let v27: i32 = method6(v24.clone(), v25, v26);
    let v28: i32 = v25.wrapping_sub(1i32);
    let v29: Rc<str> = string_slice(&v24.clone(), v27 as i64, v28 as i64);
    let v30: i32 = (v29.clone().len() as i32);
    let v31: i32 = method7(v29.clone(), v30);
    let v32: Rc<str> = string_slice(&v29.clone(), 0i32 as i64, v31 as i64);
    let v33: Option<u64> = (v32).trim().parse::<u64>().ok();
    let v34: Option<u64> = method8(v33.clone());
    let mut v35: Option<US1> = v34.map(|x| v14(x));
    let v37: US1 = US1::US1_None;
    let mut v38: US1 = v37.clone();
    let v39: US1 = v35.unwrap_or(v38);
    let v42: u64 = match &v39 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v40) => {
            let v40: u64 = *v40;
            v40
        }
    };
    let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max-busy-pct"); } LIT.with(|lit| lit.clone()) };
    let v44: Rc<str> = value_in_5(v1.clone(), v43.clone());
    let v45: i32 = (v44.clone().len() as i32);
    let v46: i32 = 0i32;
    let v47: i32 = method6(v44.clone(), v45, v46);
    let v48: i32 = v45.wrapping_sub(1i32);
    let v49: Rc<str> = string_slice(&v44.clone(), v47 as i64, v48 as i64);
    let v50: i32 = (v49.clone().len() as i32);
    let v51: i32 = method7(v49.clone(), v50);
    let v52: Rc<str> = string_slice(&v49.clone(), 0i32 as i64, v51 as i64);
    let v53: Option<u64> = (v52).trim().parse::<u64>().ok();
    let v54: Option<u64> = method8(v53.clone());
    let mut v55: Option<US1> = v54.map(|x| v14(x));
    let v57: US1 = US1::US1_None;
    let v58: US1 = v57.clone();
    let v59: US1 = v55.unwrap_or(v58);
    let v62: u64 = match &v59 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v60) => {
            let v60: u64 = *v60;
            v60
        }
    };
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mount"); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = value_in_5(v1.clone(), v63.clone());
    let v65: bool = &*v64 != "";
    let v70: Rc<str> = if v65 {
        v64.clone()
    } else {
        let v66: bool = cfg!(windows);
        if v66 {
            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("C:\\"); } LIT.with(|lit| lit.clone()) };
            v67.clone()
        } else {
            let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
            v68.clone()
        }
    };
    let v71: std::rc::Rc<sysinfo::Disks> = std::rc::Rc::new(sysinfo::Disks::new_with_refreshed_list());
    let v72: i32 = 0i32;
    let v73: u64 = free_bytes_on_9(v71.clone(), v70.clone(), v72);
    let v74: u64 = v73.wrapping_div(1073741824u64);
    let v75: sysinfo::MemoryRefreshKind = sysinfo::MemoryRefreshKind::nothing().with_ram();
    let v76: sysinfo::RefreshKind = sysinfo::RefreshKind::nothing().with_memory(v75);
    let mut v77: std::rc::Rc<sysinfo::System> = std::rc::Rc::new(sysinfo::System::new_with_specifics(v76));
    let v78: u64 = v77.available_memory();
    let v79: u64 = v78.wrapping_div(1048576u64);
    let v80: u64 = 2000u64;
    let v81: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_10(v80);
    let v82: usize = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let v83: i32 = ((v82) as i32);
    let v84: f64 = (v83 as f64);
    let v85: i32 = 0i32;
    let v86: f64 = 0.0f64;
    let v87: f64 = cores_sum_11(v81.clone(), v85, v86);
    let v88: f64 = v87 / v84;
    let v89: f64 = v88 * 100.0f64;
    let v90: u64 = (v89 as u64);
    let v91: bool = v74 >= v22;
    let v93: bool = if v91 {
        let v92: bool = v79 >= v42;
        v92
    } else {
        false
    };
    let v95: bool = if v93 {
        let v94: bool = v90 < v62;
        v94
    } else {
        false
    };
    let v96: bool = v74 < v22;
    let v123: Rc<str> = if v96 {
        let v97: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
        let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("disk-"); } LIT.with(|lit| lit.clone()) };
        let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v97));
        let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gb<"); } LIT.with(|lit| lit.clone()) };
        let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
        let v102: Rc<str> = std::rc::Rc::<str>::from((v22).to_string());
        let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
        v103.clone()
    } else {
        let v104: bool = v79 < v42;
        if v104 {
            let v105: Rc<str> = std::rc::Rc::<str>::from((v79).to_string());
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ram-"); } LIT.with(|lit| lit.clone()) };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v105));
            let v108: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mb<"); } LIT.with(|lit| lit.clone()) };
            let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v107, v108));
            let v110: Rc<str> = std::rc::Rc::<str>::from((v42).to_string());
            let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
            v111.clone()
        } else {
            let v112: bool = v90 >= v62;
            if v112 {
                let v113: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("busy-"); } LIT.with(|lit| lit.clone()) };
                let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v113));
                let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pct>="); } LIT.with(|lit| lit.clone()) };
                let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v116));
                let v118: Rc<str> = std::rc::Rc::<str>::from((v62).to_string());
                let v119: Rc<str> = Rc::<str>::from(format!("{}{}", v117, v118));
                v119.clone()
            } else {
                let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                v120.clone()
            }
        }
    };
    let v126: Rc<str> = if v95 {
        let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v124.clone()
    } else {
        let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("refused"); } LIT.with(|lit| lit.clone()) };
        v125.clone()
    };
    let v127: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit machine health gate="); } LIT.with(|lit| lit.clone()) };
    let v128: Rc<str> = Rc::<str>::from(format!("{}{}", v127, v126));
    let v129: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" reason="); } LIT.with(|lit| lit.clone()) };
    let v131: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v128, v129)), v123));
    let v132: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" disk="); } LIT.with(|lit| lit.clone()) };
    let v133: Rc<str> = Rc::<str>::from(format!("{}{}", v131, v132));
    let v134: Rc<str> = std::rc::Rc::<str>::from((v74).to_string());
    let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v133, v134));
    let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gb ram="); } LIT.with(|lit| lit.clone()) };
    let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
    let v138: Rc<str> = std::rc::Rc::<str>::from((v79).to_string());
    let v139: Rc<str> = Rc::<str>::from(format!("{}{}", v137, v138));
    let v140: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mb busy="); } LIT.with(|lit| lit.clone()) };
    let v141: Rc<str> = Rc::<str>::from(format!("{}{}", v139, v140));
    let v142: Rc<str> = std::rc::Rc::<str>::from((v90).to_string());
    let v143: Rc<str> = Rc::<str>::from(format!("{}{}", v141, v142));
    let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pct"); } LIT.with(|lit| lit.clone()) };
    let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v143, v144));
    println!("{}", v145);
    if v95 {
        0i32
    } else {
        1i32
    }
}
fn closure2(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    Rc::new(move |mut v1: Rc<str>| -> bool {
        let mut v2: bool = &*v1 == &*v0;
        v2
    })
}
fn method13(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> bool> {
    closure2(v0)
}
fn method14(mut v0: Rc<str>, mut v1: Rc<dyn Fn(Rc<str>) -> bool>, mut v2: Rc<UH0>) -> bool {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v3, v4) => {
                let v3: Rc<str> = v3.clone();
                let v4: Rc<UH0> = v4.clone();
                let v5: bool = v1(v3.clone());
                if v5 {
                    return true;
                } else {
                    let v6: Rc<dyn Fn(Rc<str>) -> bool> = method13(v0.clone());
                    (v1, v2) = (v6.clone(), v4.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn job_left_by_21(mut v0: u64, mut v1: u64) -> Rc<str> {
    let mut v2: Rc<str> = kit_machine_sample::kit_machine_job_left(v0, v1);
    v2
}
fn left_or_orphan_20(mut v0: u64, mut v1: u64) -> US2 {
    let mut v2: Rc<str> = job_left_by_21(v0, v1);
    let v3: bool = &*v2 == "";
    if v3 {
        US2::US2_Orphan
    } else {
        US2::US2_JobLeftover(v2)
    }
}
fn cells_vm_at_22(mut v0: Rc<str>) -> bool {
    let mut v1: bool = kit_machine_sample::kit_machine_cells_vm(&*v0);
    v1
}
fn owner_walk_19(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32, mut v2: i32) -> US2 {
    loop {
        let v3: bool = v1 < 0i32;
        let v5: bool = if v3 {
            true
        } else {
            let v4: bool = v2 >= 40i32;
            v4
        };
        if v5 {
            return US2::US2_Orphan;
        } else {
            let v7: Rc<str> = v0[v1 as usize].2.clone();
            let v8: Rc<str> = v0[v1 as usize].3.clone();
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).split_whitespace().skip_while(|word| *word != "supervise").nth(1).filter(|word| word.starts_with("job-")).unwrap_or(""));
            let v10: bool = &*v7 == "claude";
            if v10 {
                let v11: u64 = v0[v1 as usize].0;
                return US2::US2_ClaudeSession(v11);
            } else {
                let v13: bool = &*v9 != "";
                if v13 {
                    return US2::US2_KitJob(v9.clone());
                } else {
                    let mut v15: u64 = v0[v1 as usize].1;
                    let v16: i32 = v0.iter().position(|row| row.0 == v15).map(|at| at as i32).unwrap_or(-1);
                    let v17: bool = v16 < 0i32;
                    if v17 {
                        let v18: bool = &*v7 == "erl";
                        let v21: bool = if v18 {
                            let v19: Rc<str> = v0[v1 as usize].3.clone();
                            cells_vm_at_22(v19.clone())
                        } else {
                            false
                        };
                        if v21 {
                            let v22: u64 = v0[v1 as usize].0;
                            return US2::US2_CellsVm(v22);
                        } else {
                            let v24: u64 = v0[v1 as usize].1;
                            let v25: u64 = v0[v1 as usize].5;
                            return left_or_orphan_20(v24, v25);
                        }
                    } else {
                        let v28: i32 = v2.wrapping_add(1i32);
                        (v1, v2) = (v16, v28);
                        continue;
                    }
                }
            }
        }
    }
}
fn owner_of_18(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: i32) -> US2 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dotnet|rustc|cargo|erl|beam.smp|pwsh|powershell|node|bun|gcc|cc1|cc1plus|ld|zig|lean|java|swiftc|swift-frontend|python|kotlinc|gleam|wasmtime|link|clang|mix|escript|make|git"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = v0[v1 as usize].2.clone();
    let v4: bool = v2.split("|").any(|item| item == &*v3);
    if v4 {
        let v5: i32 = 0i32;
        owner_walk_19(v0.clone(), v1, v5)
    } else {
        US2::US2_NotATool
    }
}
fn owner_text_23(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_CellsVm(v8) => {
            let v8: u64 = *v8;
            let v9: Rc<str> = std::rc::Rc::<str>::from((v8).to_string());
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cells-vm:"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
            v11
        }
        US2::US2_ClaudeSession(v1) => {
            let mut v1: u64 = *v1;
            let mut v2: Rc<str> = std::rc::Rc::<str>::from((v1).to_string());
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claude:"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            v4
        }
        US2::US2_JobLeftover(v12) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("left-by:"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v12));
            v14
        }
        US2::US2_KitJob(v5) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit:"); } LIT.with(|lit| lit.clone()) };
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v5));
            v7
        }
        US2::US2_NotATool => {
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            v16
        }
        US2::US2_Orphan => {
            let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("orphan"); } LIT.with(|lit| lit.clone()) };
            v15
        }
    }
}
fn tallied_17(mut v0: u64, mut v1: u64, mut v2: f64, mut v3: u64, mut v4: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v5: bool, mut v6: i32, mut v7: Rc<UH0>, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64) -> (Rc<UH0>, u64, u64, u64, u64, u64, u64) {
    let mut v14: i32 = v4.len() as i32;
    let mut v15: bool = v6 >= v14;
    let (mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = if v15 {
        (v7.clone(), v8, v9, v10, v11, v12, v13)
    } else {
        let v16: US2 = owner_of_18(v4.clone(), v6);
        let (mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v16 {
            US2::US2_NotATool => {
                let v17: i32 = v6.wrapping_add(1i32);
                tallied_17(v0, v1, v2, v3, v4.clone(), v5, v17, v7.clone(), v8, v9, v10, v11, v12, v13)
            }
            _ => {
                let v25: u64 = v4[v6 as usize].7;
                let v26: f64 = v4[v6 as usize].4;
                let v27: u64 = v4[v6 as usize].5;
                let v41: US3 = match &v16 {
                    US2::US2_JobLeftover(v28) => {
                        let v28: Rc<str> = v28.clone();
                        US3::US3_Leftover
                    }
                    US2::US2_Orphan => {
                        let v30: bool = v26 >= v2;
                        let v32: bool = if v30 {
                            let v31: bool = v27 >= v1;
                            v31
                        } else {
                            false
                        };
                        if v32 {
                            US3::US3_Dangling
                        } else {
                            US3::US3_Unowned
                        }
                    }
                    _ => {
                        let v36: bool = v25 >= v3;
                        if v36 {
                            US3::US3_Hog
                        } else {
                            US3::US3_Healthy
                        }
                    }
                };
                let v43: bool = match &v41 {
                    US3::US3_Healthy => {
                        v5
                    }
                    US3::US3_Unowned => {
                        v5
                    }
                    _ => {
                        true
                    }
                };
                let (mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                    US3::US3_Healthy => {
                        let v44: u64 = v8.wrapping_add(1u64);
                        let v45: u64 = v9.wrapping_add(1u64);
                        (v7.clone(), v44, v45, v10, v11, v12, v13)
                    }
                    _ => {
                        let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                            US3::US3_Unowned => {
                                let v46: u64 = v8.wrapping_add(1u64);
                                let v47: u64 = v10.wrapping_add(1u64);
                                (v7.clone(), v46, v9, v47, v11, v12, v13)
                            }
                            _ => {
                                let (mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                    US3::US3_Dangling => {
                                        let v48: u64 = v8.wrapping_add(1u64);
                                        let v49: u64 = v11.wrapping_add(1u64);
                                        (v7.clone(), v48, v9, v10, v49, v12, v13)
                                    }
                                    _ => {
                                        let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                            US3::US3_Leftover => {
                                                let v50: u64 = v8.wrapping_add(1u64);
                                                let v51: u64 = v12.wrapping_add(1u64);
                                                (v7.clone(), v50, v9, v10, v11, v51, v13)
                                            }
                                            _ => {
                                                let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = match &v41 {
                                                    US3::US3_Hog => {
                                                        let v52: u64 = v8.wrapping_add(1u64);
                                                        let v53: u64 = v13.wrapping_add(1u64);
                                                        (v7, v52, v9, v10, v11, v12, v53)
                                                    }
                                                    _ => unreachable!(),
                                                };
                                                (v54, v55, v56, v57, v58, v59, v60)
                                            }
                                        };
                                        (v61, v62, v63, v64, v65, v66, v67)
                                    }
                                };
                                (v68, v69, v70, v71, v72, v73, v74)
                            }
                        };
                        (v75, v76, v77, v78, v79, v80, v81)
                    }
                };
                let (mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = if v43 {
                    let v89: u64 = v4[v6 as usize].0;
                    let v90: Rc<str> = std::rc::Rc::<str>::from((v89).to_string());
                    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v93: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v91, v90)), v91));
                    let v94: Rc<str> = v4[v6 as usize].2.clone();
                    let v96: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v93, v94)), v91));
                    let v97: f64 = v4[v6 as usize].4;
                    let v98: Rc<str> = std::rc::Rc::<str>::from(format!("{:.2}", v97));
                    let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v98));
                    let v100: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cores  "); } LIT.with(|lit| lit.clone()) };
                    let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v99, v100));
                    let v102: u64 = v4[v6 as usize].7;
                    let v103: Rc<str> = std::rc::Rc::<str>::from((v102).to_string());
                    let v104: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v103));
                    let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB  "); } LIT.with(|lit| lit.clone()) };
                    let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v105));
                    let v107: u64 = v4[v6 as usize].5;
                    let v108: Rc<str> = std::rc::Rc::<str>::from((v107).to_string());
                    let v109: Rc<str> = Rc::<str>::from(format!("{}{}", v106, v108));
                    let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" min  "); } LIT.with(|lit| lit.clone()) };
                    let v111: Rc<str> = Rc::<str>::from(format!("{}{}", v109, v110));
                    let v112: Rc<str> = owner_text_23(v16);
                    let v114: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v111, v112)), v91));
                    let v124: Rc<str> = match &v41 {
                        US3::US3_Dangling => {
                            let v117: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dangling"); } LIT.with(|lit| lit.clone()) };
                            v117
                        }
                        US3::US3_Healthy => {
                            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                            v115
                        }
                        US3::US3_Hog => {
                            let v119: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hog"); } LIT.with(|lit| lit.clone()) };
                            v119
                        }
                        US3::US3_Leftover => {
                            let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("leftover"); } LIT.with(|lit| lit.clone()) };
                            v118
                        }
                        US3::US3_Unowned => {
                            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unowned"); } LIT.with(|lit| lit.clone()) };
                            v116
                        }
                    };
                    let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v114, v124));
                    let v126: Rc<UH0> = Rc::new(UH0::UH0_Cons(v125, v82.clone()));
                    (v126, v83, v84, v85, v86, v87, v88)
                } else {
                    (v82, v83, v84, v85, v86, v87, v88)
                };
                let v134: i32 = v6.wrapping_add(1i32);
                let (mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = tallied_17(v0, v1, v2, v3, v4, v5, v134, v127, v128, v129, v130, v131, v132, v133);
                (v135, v136, v137, v138, v139, v140, v141)
            }
        };
        (v142, v143, v144, v145, v146, v147, v148)
    };
    (v149, v150, v151, v152, v153, v154, v155)
}
fn method24(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method25(mut v0: Rc<UH0>) -> () {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1.clone();
                let mut v2: Rc<UH0> = v2.clone();
                println!("{}", v1);
                v0 = v2.clone();
                continue;
            }
            UH0::UH0_Nil => {
                return ();
            }
        }
        return ();
    }
}
fn stalled_jobs_listed_26() -> Rc<str> {
    let mut v0: Rc<str> = kit_machine_sample::kit_machine_stalled_jobs();
    v0
}
fn history_written_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_machine_sample::kit_machine_history_appended(&*v0);
    v1
}
fn health_pass_16(mut v0: u64, mut v1: bool) -> bool {
    let mut v2: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_10(v0);
    let v3: u64 = 60u64;
    let v4: u64 = 30u64;
    let v5: f64 = 0.8f64;
    let v6: u64 = 8192u64;
    let v7: i32 = 0i32;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: u64 = 0u64;
    let v10: u64 = 0u64;
    let v11: u64 = 0u64;
    let v12: u64 = 0u64;
    let v13: u64 = 0u64;
    let mut v14: u64 = 0u64;
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<UH0>, u64, u64, u64, u64, u64, u64) = tallied_17(v3, v4, v5, v6, v2.clone(), v1, v7, v8.clone(), v9, v10, v11, v12, v13, v14);
    let v22: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v23: Rc<UH0> = method24(v15.clone(), v22.clone());
    method25(v23.clone());
    let v24: Rc<str> = stalled_jobs_listed_26();
    let v25: bool = &*v24 == "";
    if v25 {
        ()
    } else {
        println!("{}", v24);
        ()
    };
    let v30: u64 = if v25 {
        0u64
    } else {
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v24.split(&*v26).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
        let v28: i32 = (v27.clone().borrow().len() as i32);
        let v29: u64 = (v28 as u64);
        v29
    };
    let v31: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v32: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let v33: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v34: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let mut v35: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
    let v36: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v37: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let mut v38: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
    v38.borrow_mut().push(v31);
    v38.borrow_mut().push(v32);
    v38.borrow_mut().push(v33);
    v38.borrow_mut().push(v34);
    v38.borrow_mut().push(v35);
    v38.borrow_mut().push(v36);
    v38.borrow_mut().push(v37);
    let v39: Rc<Vec<Rc<str>>> = Rc::new(v38.borrow().clone());
    let v40: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v39).as_ref().clone()));
    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\t"); } LIT.with(|lit| lit.clone()) };
    let v42: Rc<str> = Rc::<str>::from((*v40.borrow()).join(&*v41));
    let v43: Rc<str> = history_written_27(v42.clone());
    let v44: u64 = v19.wrapping_add(v20);
    let v45: bool = v44 > 0u64;
    let v48: Rc<str> = if v45 {
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
        v46.clone()
    } else {
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
        v47.clone()
    };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit machine health "); } LIT.with(|lit| lit.clone()) };
    let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v48));
    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" tools="); } LIT.with(|lit| lit.clone()) };
    let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v51));
    let v53: Rc<str> = std::rc::Rc::<str>::from((v16).to_string());
    let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v52, v53));
    let mut v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" ok="); } LIT.with(|lit| lit.clone()) };
    let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
    let v57: Rc<str> = std::rc::Rc::<str>::from((v17).to_string());
    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v56, v57));
    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" unowned="); } LIT.with(|lit| lit.clone()) };
    let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
    let v61: Rc<str> = std::rc::Rc::<str>::from((v18).to_string());
    let v62: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v61));
    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" dangling="); } LIT.with(|lit| lit.clone()) };
    let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v62, v63));
    let v65: Rc<str> = std::rc::Rc::<str>::from((v19).to_string());
    let v66: Rc<str> = Rc::<str>::from(format!("{}{}", v64, v65));
    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" leftover="); } LIT.with(|lit| lit.clone()) };
    let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
    let v69: Rc<str> = std::rc::Rc::<str>::from((v20).to_string());
    let v70: Rc<str> = Rc::<str>::from(format!("{}{}", v68, v69));
    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" hogs="); } LIT.with(|lit| lit.clone()) };
    let v72: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v71));
    let v73: Rc<str> = std::rc::Rc::<str>::from((v21).to_string());
    let v74: Rc<str> = Rc::<str>::from(format!("{}{}", v72, v73));
    let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" stalled="); } LIT.with(|lit| lit.clone()) };
    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v74, v75));
    let mut v77: Rc<str> = std::rc::Rc::<str>::from((v30).to_string());
    let v78: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v77));
    let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" history="); } LIT.with(|lit| lit.clone()) };
    let v81: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v78, v79)), v43));
    println!("{}", v81);
    v45
}
fn health_passes_15(mut v0: u64, mut v1: bool, mut v2: u64, mut v3: bool) -> bool {
    loop {
        let v4: bool = v2 == 0u64;
        if v4 {
            return v3;
        } else {
            let v5: u64 = v2.wrapping_sub(1u64);
            let v6: bool = health_pass_16(v0, v1);
            (v2, v3) = (v5, v6);
            continue;
        }
    }
}
fn health_normal_run_12() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--every"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = value_in_5(v1.clone(), v2.clone());
    let v4: i32 = (v3.clone().len() as i32);
    let v5: i32 = 0i32;
    let v6: i32 = method6(v3.clone(), v4, v5);
    let v7: i32 = v4.wrapping_sub(1i32);
    let v8: Rc<str> = string_slice(&v3.clone(), v6 as i64, v7 as i64);
    let v9: i32 = (v8.clone().len() as i32);
    let v10: i32 = method7(v8.clone(), v9);
    let v11: Rc<str> = string_slice(&v8.clone(), 0i32 as i64, v10 as i64);
    let v12: Option<u64> = (v11).trim().parse::<u64>().ok();
    let v13: Option<u64> = method8(v12.clone());
    let mut v14: Rc<dyn Fn((u64)) -> US1> = closure1();
    let mut v15: Option<US1> = v13.map(|x| v14(x));
    let v17: US1 = US1::US1_None;
    let v18: US1 = v17.clone();
    let v19: US1 = v15.unwrap_or(v18);
    let v22: u64 = match &v19 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v20) => {
            let v20: u64 = *v20;
            v20
        }
    };
    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--times"); } LIT.with(|lit| lit.clone()) };
    let v24: Rc<str> = value_in_5(v1.clone(), v23.clone());
    let v25: i32 = (v24.clone().len() as i32);
    let v26: i32 = 0i32;
    let v27: i32 = method6(v24.clone(), v25, v26);
    let v28: i32 = v25.wrapping_sub(1i32);
    let v29: Rc<str> = string_slice(&v24.clone(), v27 as i64, v28 as i64);
    let v30: i32 = (v29.clone().len() as i32);
    let v31: i32 = method7(v29.clone(), v30);
    let v32: Rc<str> = string_slice(&v29.clone(), 0i32 as i64, v31 as i64);
    let v33: Option<u64> = (v32).trim().parse::<u64>().ok();
    let v34: Option<u64> = method8(v33.clone());
    let mut v35: Option<US1> = v34.map(|x| v14(x));
    let v37: US1 = US1::US1_None;
    let mut v38: US1 = v37.clone();
    let v39: US1 = v35.unwrap_or(v38);
    let v42: u64 = match &v39 {
        US1::US1_None => {
            0u64
        }
        US1::US1_Some(v40) => {
            let v40: u64 = *v40;
            v40
        }
    };
    let v43: bool = v22 == 0u64;
    let v45: u64 = if v43 {
        3000u64
    } else {
        let v44: u64 = v22.wrapping_mul(1000u64);
        v44
    };
    let v48: u64 = if v43 {
        1u64
    } else {
        let v46: bool = v42 == 0u64;
        if v46 {
            1u64
        } else {
            v42
        }
    };
    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--all"); } LIT.with(|lit| lit.clone()) };
    let v50: i32 = 0i32;
    let v51: Rc<UH0> = arguments_from_2(v50);
    let v52: Rc<dyn Fn(Rc<str>) -> bool> = method13(v49.clone());
    let v53: bool = method14(v49.clone(), v52.clone(), v51.clone());
    let v54: bool = false;
    let mut v55: bool = health_passes_15(v45, v53, v48, v54);
    if v55 {
        1i32
    } else {
        0i32
    }
}
fn health_run_0() -> i32 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--gate"); } LIT.with(|lit| lit.clone()) };
    let mut v1: i32 = 0i32;
    let mut v2: Rc<UH0> = arguments_from_2(v1);
    let v3: Rc<dyn Fn(Rc<str>) -> bool> = method13(v0.clone());
    let v4: bool = method14(v0, v3, v2);
    if v4 {
        gate_run_1()
    } else {
        health_normal_run_12()
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        health_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_machine_health_run() -> i32 {
    closure0()()
}
