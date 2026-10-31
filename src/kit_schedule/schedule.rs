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
    US0_AskDue(Rc<str>, u64),
    US0_MarkRan(Rc<str>),
    US0_ListSchedules,
    US0_ScheduleUsage,
}
#[derive(Clone)]
enum US1 {
    US1_Some(u64),
    US1_None,
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_ScheduleDue,
    US3_ScheduleWaits(u64),
}
struct Mut0 { l0: Rc<str> }
fn refused_1(mut v0: Rc<str>) -> i32 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: schedule "); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = Rc::<str>::from(format!("{}{}", v1, v0));
    println!("{}", v2);
    2i32
}
fn command_line_argument_4(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn command_line_arguments_back_3(mut v0: i32, mut v1: i32, mut v2: Rc<UH0>) -> Rc<UH0> {
    loop {
        let v3: bool = v1 < v0;
        if v3 {
            return v2.clone();
        } else {
            let v4: i32 = v1.wrapping_sub(1i32);
            let v5: Rc<str> = command_line_argument_4(v1);
            let mut v6: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5.clone(), v2.clone()));
            (v0, v1, v2) = (v0, v4, v6.clone());
            continue;
        }
    }
}
fn command_line_argument_count_5() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_arguments_from_2(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 < 0i32;
    let v2: i32 = if v1 {
        0i32
    } else {
        v0
    };
    let v3: i32 = command_line_argument_count_5();
    let v4: i32 = v3.wrapping_sub(1i32);
    let v5: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    command_line_arguments_back_3(v2, v4, v5)
}
fn method6(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US1> = Rc::new(move |mut v0: (u64)| -> US1 {
        let v1: u64 = (v0);
        US1::US1_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn get_environment_variable_8(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn method10(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let mut v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method10(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn schedules_root_7() -> US2 {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("KIT_SCHEDULES_HOME"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = get_environment_variable_8(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("LOCALAPPDATA"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = get_environment_variable_8(v2);
    let v4: bool = &*v1 != "";
    if v4 {
        US2::US2_Some(v1)
    } else {
        let mut v6: bool = &*v3 != "";
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = method9(v3, v7);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("schedules"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = method9(v8, v9);
            US2::US2_Some(v10)
        } else {
            US2::US2_None
        }
    }
}
fn file_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn method13(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn method14(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: bool = v2 >= v1;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v2 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                let mut v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v1, v12);
                continue;
            } else {
                return v2;
            }
        }
    }
}
fn method15(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 <= 0i32;
        if v2 {
            return -1i32;
        } else {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            let mut v11: bool = if v5 {
                true
            } else {
                let mut v6: bool = v4 == b'\t';
                if v6 {
                    true
                } else {
                    let mut v7: bool = v4 == b'\r';
                    if v7 {
                        true
                    } else {
                        let v8: bool = v4 == b'\n';
                        v8
                    }
                }
            };
            if v11 {
                (v0, v1) = (v0.clone(), v3);
                continue;
            } else {
                return v3;
            }
        }
    }
}
fn stamp_of_11(mut v0: Rc<str>, mut v1: Rc<str>) -> US1 {
    let v2: Rc<str> = method9(v0.clone(), v1.clone());
    let v3: bool = file_exists_12(v2.clone());
    if v3 {
        let v5: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v2);
        let mut v7: Vec<u8> = v5.unwrap();
        let v8: Vec<u8> = method13(v7.clone());
        let v10: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v8);
        let mut v12: std::string::String = v10.unwrap();
        let mut v14: Rc<str> = Rc::<str>::from(String::as_str(&v12));
        let v15: i32 = (v14.clone().len() as i32);
        let v16: i32 = 0i32;
        let v17: i32 = method14(v14.clone(), v15, v16);
        let v18: i32 = v15.wrapping_sub(1i32);
        let v19: Rc<str> = string_slice(&v14.clone(), v17 as i64, v18 as i64);
        let v20: i32 = (v19.clone().len() as i32);
        let v21: i32 = method15(v19.clone(), v20);
        let v22: Rc<str> = string_slice(&v19.clone(), 0i32 as i64, v21 as i64);
        let v23: Option<u64> = (v22).trim().parse::<u64>().ok();
        let v24: Option<u64> = method6(v23.clone());
        let mut v25: Rc<dyn Fn((u64)) -> US1> = closure1();
        let mut v26: Option<US1> = v24.map(|x| v25(x));
        let v28: US1 = US1::US1_None;
        let v29: US1 = v28.clone();
        let v30: US1 = v26.unwrap_or(v29);
        v30.clone()
    } else {
        US1::US1_None
    }
}
fn method17(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_16(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method17(v2.clone(), v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn stamp_written_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> () {
    let v4: Result<(), std::io::Error> = std::fs::create_dir_all(&*v0);
    let v5: Rc<str> = method9(v0, v1);
    let mut v6: Rc<str> = format_real_16(v2);
    std::fs::write(&*v5, &*v6).unwrap();
    ()
}
fn schedule_run_0() -> i32 {
    let mut v0: i32 = 1i32;
    let v1: Rc<UH0> = command_line_arguments_from_2(v0);
    let v232: US0 = match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH0> = v3.clone();
            let v4: bool = "due" == &*v2;
            if v4 {
                match &*v3 {
                    UH0::UH0_Cons(v5, v6) => {
                        let v5: Rc<str> = v5.clone();
                        let mut v6: Rc<UH0> = v6.clone();
                        match &*v6 {
                            UH0::UH0_Cons(v7, v8) => {
                                let mut v7: Rc<str> = v7.clone();
                                let v8: Rc<UH0> = v8.clone();
                                match &*v8 {
                                    UH0::UH0_Nil => {
                                        let v9: Option<u64> = (v7).trim().parse::<u64>().ok();
                                        let v10: Option<u64> = method6(v9.clone());
                                        let mut v11: Rc<dyn Fn((u64)) -> US1> = closure1();
                                        let mut v12: Option<US1> = v10.map(|x| v11(x));
                                        let mut v14: US1 = US1::US1_None;
                                        let v15: US1 = v14.clone();
                                        let v16: US1 = v12.unwrap_or(v15);
                                        match &v16 {
                                            US1::US1_None => {
                                                US0::US0_ScheduleUsage
                                            }
                                            US1::US1_Some(v17) => {
                                                let v17: u64 = *v17;
                                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                                let v19: bool = v5.contains(&*v18);
                                                let v20: bool = v19 == false;
                                                let v24: bool = if v20 {
                                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                    let v22: bool = v5.contains(&*v21);
                                                    let v23: bool = v22 == false;
                                                    v23
                                                } else {
                                                    false
                                                };
                                                let v28: bool = if v24 {
                                                    let mut v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                                                    let mut v26: bool = v5.contains(&*v25);
                                                    let v27: bool = v26 == false;
                                                    v27
                                                } else {
                                                    false
                                                };
                                                let v32: bool = if v28 {
                                                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                                    let v30: bool = v5.contains(&*v29);
                                                    let v31: bool = v30 == false;
                                                    v31
                                                } else {
                                                    false
                                                };
                                                let v36: bool = if v32 {
                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                                                    let v34: bool = v5.contains(&*v33);
                                                    let v35: bool = v34 == false;
                                                    v35
                                                } else {
                                                    false
                                                };
                                                let v40: bool = if v36 {
                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("?"); } LIT.with(|lit| lit.clone()) };
                                                    let v38: bool = v5.contains(&*v37);
                                                    let v39: bool = v38 == false;
                                                    v39
                                                } else {
                                                    false
                                                };
                                                let v44: bool = if v40 {
                                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                    let v42: bool = v5.contains(&*v41);
                                                    let v43: bool = v42 == false;
                                                    v43
                                                } else {
                                                    false
                                                };
                                                let v48: bool = if v44 {
                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                                    let v46: bool = v5.contains(&*v45);
                                                    let v47: bool = v46 == false;
                                                    v47
                                                } else {
                                                    false
                                                };
                                                let v52: bool = if v48 {
                                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                                    let v50: bool = v5.contains(&*v49);
                                                    let v51: bool = v50 == false;
                                                    v51
                                                } else {
                                                    false
                                                };
                                                let v56: bool = if v52 {
                                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                                                    let v54: bool = v5.contains(&*v53);
                                                    let v55: bool = v54 == false;
                                                    v55
                                                } else {
                                                    false
                                                };
                                                let v60: bool = if v56 {
                                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                    let v58: bool = v5.contains(&*v57);
                                                    let v59: bool = v58 == false;
                                                    v59
                                                } else {
                                                    false
                                                };
                                                let v61: i32 = (v5.clone().len() as i32);
                                                let v62: bool = v61 > 0i32;
                                                let v64: bool = if v62 {
                                                    let v63: bool = v61 <= 64i32;
                                                    v63
                                                } else {
                                                    false
                                                };
                                                let v65: bool = v64 && v60;
                                                let v67: bool = if v65 {
                                                    let v66: bool = v17 > 0u64;
                                                    v66
                                                } else {
                                                    false
                                                };
                                                if v67 {
                                                    let v68: u64 = v17.wrapping_mul(60000u64);
                                                    US0::US0_AskDue(v5.clone(), v68)
                                                } else {
                                                    US0::US0_ScheduleUsage
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        let v75: bool = "ran" == &*v2;
                                        if v75 {
                                            US0::US0_ScheduleUsage
                                        } else {
                                            US0::US0_ScheduleUsage
                                        }
                                    }
                                }
                            }
                            _ => {
                                let v86: bool = "ran" == &*v2;
                                if v86 {
                                    match &*v6 {
                                        UH0::UH0_Nil => {
                                            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                            let v88: bool = v5.contains(&*v87);
                                            let v89: bool = v88 == false;
                                            let v93: bool = if v89 {
                                                let v90: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                                let v91: bool = v5.contains(&*v90);
                                                let v92: bool = v91 == false;
                                                v92
                                            } else {
                                                false
                                            };
                                            let v97: bool = if v93 {
                                                let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                                                let v95: bool = v5.contains(&*v94);
                                                let v96: bool = v95 == false;
                                                v96
                                            } else {
                                                false
                                            };
                                            let v101: bool = if v97 {
                                                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                                let v99: bool = v5.contains(&*v98);
                                                let v100: bool = v99 == false;
                                                v100
                                            } else {
                                                false
                                            };
                                            let v105: bool = if v101 {
                                                let v102: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                                                let v103: bool = v5.contains(&*v102);
                                                let v104: bool = v103 == false;
                                                v104
                                            } else {
                                                false
                                            };
                                            let v109: bool = if v105 {
                                                let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("?"); } LIT.with(|lit| lit.clone()) };
                                                let v107: bool = v5.contains(&*v106);
                                                let v108: bool = v107 == false;
                                                v108
                                            } else {
                                                false
                                            };
                                            let v113: bool = if v109 {
                                                let v110: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                                let v111: bool = v5.contains(&*v110);
                                                let v112: bool = v111 == false;
                                                v112
                                            } else {
                                                false
                                            };
                                            let v117: bool = if v113 {
                                                let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                                let v115: bool = v5.contains(&*v114);
                                                let v116: bool = v115 == false;
                                                v116
                                            } else {
                                                false
                                            };
                                            let v121: bool = if v117 {
                                                let v118: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                                let v119: bool = v5.contains(&*v118);
                                                let v120: bool = v119 == false;
                                                v120
                                            } else {
                                                false
                                            };
                                            let v125: bool = if v121 {
                                                let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                                                let v123: bool = v5.contains(&*v122);
                                                let v124: bool = v123 == false;
                                                v124
                                            } else {
                                                false
                                            };
                                            let v129: bool = if v125 {
                                                let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                                let v127: bool = v5.contains(&*v126);
                                                let v128: bool = v127 == false;
                                                v128
                                            } else {
                                                false
                                            };
                                            let v130: i32 = (v5.clone().len() as i32);
                                            let v131: bool = v130 > 0i32;
                                            let v133: bool = if v131 {
                                                let v132: bool = v130 <= 64i32;
                                                v132
                                            } else {
                                                false
                                            };
                                            let v134: bool = v133 && v129;
                                            if v134 {
                                                US0::US0_MarkRan(v5.clone())
                                            } else {
                                                US0::US0_ScheduleUsage
                                            }
                                        }
                                        _ => unreachable!(),
                                    }
                                } else {
                                    US0::US0_ScheduleUsage
                                }
                            }
                        }
                    }
                    _ => {
                        let v145: bool = "ran" == &*v2;
                        if v145 {
                            let v146: bool = "list" == &*v2;
                            if v146 {
                                match &*v3 {
                                    UH0::UH0_Nil => {
                                        US0::US0_ListSchedules
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                US0::US0_ScheduleUsage
                            }
                        } else {
                            let v151: bool = "list" == &*v2;
                            if v151 {
                                match &*v3 {
                                    UH0::UH0_Nil => {
                                        US0::US0_ListSchedules
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                US0::US0_ScheduleUsage
                            }
                        }
                    }
                }
            } else {
                let v158: bool = "ran" == &*v2;
                if v158 {
                    match &*v3 {
                        UH0::UH0_Cons(v159, v160) => {
                            let v159: Rc<str> = v159.clone();
                            let v160: Rc<UH0> = v160.clone();
                            match &*v160 {
                                UH0::UH0_Nil => {
                                    let v161: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
                                    let v162: bool = v159.contains(&*v161);
                                    let v163: bool = v162 == false;
                                    let v167: bool = if v163 {
                                        let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
                                        let v165: bool = v159.contains(&*v164);
                                        let v166: bool = v165 == false;
                                        v166
                                    } else {
                                        false
                                    };
                                    let v171: bool = if v167 {
                                        let v168: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
                                        let v169: bool = v159.contains(&*v168);
                                        let v170: bool = v169 == false;
                                        v170
                                    } else {
                                        false
                                    };
                                    let v175: bool = if v171 {
                                        let v172: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
                                        let v173: bool = v159.contains(&*v172);
                                        let v174: bool = v173 == false;
                                        v174
                                    } else {
                                        false
                                    };
                                    let v179: bool = if v175 {
                                        let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
                                        let v177: bool = v159.contains(&*v176);
                                        let v178: bool = v177 == false;
                                        v178
                                    } else {
                                        false
                                    };
                                    let v183: bool = if v179 {
                                        let v180: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("?"); } LIT.with(|lit| lit.clone()) };
                                        let v181: bool = v159.contains(&*v180);
                                        let v182: bool = v181 == false;
                                        v182
                                    } else {
                                        false
                                    };
                                    let v187: bool = if v183 {
                                        let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                                        let v185: bool = v159.contains(&*v184);
                                        let v186: bool = v185 == false;
                                        v186
                                    } else {
                                        false
                                    };
                                    let v191: bool = if v187 {
                                        let v188: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
                                        let v189: bool = v159.contains(&*v188);
                                        let v190: bool = v189 == false;
                                        v190
                                    } else {
                                        false
                                    };
                                    let v195: bool = if v191 {
                                        let v192: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
                                        let v193: bool = v159.contains(&*v192);
                                        let v194: bool = v193 == false;
                                        v194
                                    } else {
                                        false
                                    };
                                    let v199: bool = if v195 {
                                        let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                                        let v197: bool = v159.contains(&*v196);
                                        let v198: bool = v197 == false;
                                        v198
                                    } else {
                                        false
                                    };
                                    let v203: bool = if v199 {
                                        let v200: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                                        let v201: bool = v159.contains(&*v200);
                                        let v202: bool = v201 == false;
                                        v202
                                    } else {
                                        false
                                    };
                                    let v204: i32 = (v159.clone().len() as i32);
                                    let v205: bool = v204 > 0i32;
                                    let v207: bool = if v205 {
                                        let v206: bool = v204 <= 64i32;
                                        v206
                                    } else {
                                        false
                                    };
                                    let v208: bool = v207 && v203;
                                    if v208 {
                                        US0::US0_MarkRan(v159.clone())
                                    } else {
                                        US0::US0_ScheduleUsage
                                    }
                                }
                                _ => {
                                    US0::US0_ScheduleUsage
                                }
                            }
                        }
                        _ => {
                            let v217: bool = "list" == &*v2;
                            if v217 {
                                match &*v3 {
                                    UH0::UH0_Nil => {
                                        US0::US0_ListSchedules
                                    }
                                    _ => unreachable!(),
                                }
                            } else {
                                US0::US0_ScheduleUsage
                            }
                        }
                    }
                } else {
                    let v223: bool = "list" == &*v2;
                    if v223 {
                        match &*v3 {
                            UH0::UH0_Nil => {
                                US0::US0_ListSchedules
                            }
                            _ => {
                                US0::US0_ScheduleUsage
                            }
                        }
                    } else {
                        US0::US0_ScheduleUsage
                    }
                }
            }
        }
        _ => {
            US0::US0_ScheduleUsage
        }
    };
    let v233: US2 = schedules_root_7();
    match &v233 {
        US2::US2_None => {
            let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("has no stamp folder: neither KIT_SCHEDULES_HOME nor LOCALAPPDATA is set"); } LIT.with(|lit| lit.clone()) };
            refused_1(v234.clone())
        }
        US2::US2_Some(v236) => {
            let v236: Rc<str> = v236.clone();
            match &v232 {
                US0::US0_AskDue(v237, v238) => {
                    let v237: Rc<str> = v237.clone();
                    let v238: u64 = *v238;
                    let v239: US1 = stamp_of_11(v236.clone(), v237.clone());
                    let v240: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    let v250: US3 = match &v239 {
                        US1::US1_None => {
                            US3::US3_ScheduleDue
                        }
                        US1::US1_Some(v241) => {
                            let v241: u64 = *v241;
                            let v242: u64 = v241.wrapping_add(v238);
                            let v243: bool = v240 >= v242;
                            if v243 {
                                US3::US3_ScheduleDue
                            } else {
                                let v245: u64 = v242.wrapping_sub(v240);
                                US3::US3_ScheduleWaits(v245)
                            }
                        }
                    };
                    let v264: Rc<str> = match &v250 {
                        US3::US3_ScheduleDue => {
                            let v251: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit schedule due ok name="); } LIT.with(|lit| lit.clone()) };
                            let v252: Rc<str> = Rc::<str>::from(format!("{}{}", v251, v237));
                            let v253: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" due=yes"); } LIT.with(|lit| lit.clone()) };
                            let v254: Rc<str> = Rc::<str>::from(format!("{}{}", v252, v253));
                            v254.clone()
                        }
                        US3::US3_ScheduleWaits(v255) => {
                            let v255: u64 = *v255;
                            let v256: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit schedule due ok name="); } LIT.with(|lit| lit.clone()) };
                            let v257: Rc<str> = Rc::<str>::from(format!("{}{}", v256, v237));
                            let v258: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" due=no next_in_s="); } LIT.with(|lit| lit.clone()) };
                            let v259: Rc<str> = Rc::<str>::from(format!("{}{}", v257, v258));
                            let v260: u64 = v255.wrapping_div(1000u64);
                            let v261: Rc<str> = format_real_16(v260);
                            let v262: Rc<str> = Rc::<str>::from(format!("{}{}", v259, v261));
                            v262.clone()
                        }
                    };
                    println!("{}", v264);
                    let v265: bool = match &v250 {
                        US3::US3_ScheduleDue => {
                            true
                        }
                        _ => {
                            false
                        }
                    };
                    if v265 {
                        0i32
                    } else {
                        1i32
                    }
                }
                US0::US0_ListSchedules => {
                    let v275: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit schedule list ok root="); } LIT.with(|lit| lit.clone()) };
                    let v276: Rc<str> = Rc::<str>::from(format!("{}{}", v275, v236));
                    println!("{}", v276);
                    0i32
                }
                US0::US0_MarkRan(v267) => {
                    let v267: Rc<str> = v267.clone();
                    let v268: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
                    stamp_written_18(v236.clone(), v267.clone(), v268);
                    let v269: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit schedule ran ok name="); } LIT.with(|lit| lit.clone()) };
                    let v270: Rc<str> = Rc::<str>::from(format!("{}{}", v269, v267));
                    let v271: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" at_ms="); } LIT.with(|lit| lit.clone()) };
                    let v272: Rc<str> = Rc::<str>::from(format!("{}{}", v270, v271));
                    let v273: Rc<str> = format_real_16(v268);
                    let v274: Rc<str> = Rc::<str>::from(format!("{}{}", v272, v273));
                    println!("{}", v274);
                    0i32
                }
                US0::US0_ScheduleUsage => {
                    let v277: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit schedule: due <name> <minutes> (exit 0 when due, 1 when not) | ran <name> | list (a name has no path characters; stamps under KIT_SCHEDULES_HOME, else LOCALAPPDATA\\kit\\schedules; never a relative folder)"); } LIT.with(|lit| lit.clone()) };
                    refused_1(v277.clone())
                }
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        schedule_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_schedule_run() -> i32 {
    closure0()()
}
