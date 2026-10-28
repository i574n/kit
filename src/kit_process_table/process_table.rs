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
    US1_EveryProcess,
    US1_SessionTree(u64),
    US1_NoSession,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoPsRows,
    UH0_PsRowThen(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
#[derive(Clone)]
enum US3 {
    US3_ListProcesses,
    US3_StopProcesses,
    US3_ProcessUnknown,
}
fn command_line_argument_count_3() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_2() -> i32 {
    let mut v0: i32 = command_line_argument_count_3();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn command_line_argument_6(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_5(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let v3: i32 = command_line_argument_count_3();
        let v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US0 = if v5 {
        let v6: Rc<str> = command_line_argument_6(v1);
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
fn pattern_from_4(mut v0: i32, mut v1: i32) -> Rc<str> {
    loop {
        let v2: bool = v0 < v1;
        if v2 {
            let v3: Rc<str> = argument_5(v0);
            let v4: bool = &*v3 == "--tree";
            let v7: bool = if v4 {
                true
            } else {
                let v5: Rc<str> = argument_5(v0);
                let v6: bool = &*v5 == "--mine";
                v6
            };
            if v7 {
                let v8: i32 = v0.wrapping_add(1i32);
                (v0, v1) = (v8, v1);
                continue;
            } else {
                return argument_5(v0);
            }
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v12.clone();
        }
    }
}
fn pattern_problem_7(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(regex::RegexBuilder::new(&*v0).case_insensitive(true).build().err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1.clone()
}
fn process_rows_sampled_8(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let kind = sysinfo::ProcessRefreshKind::nothing().with_cpu().with_memory().with_cmd(sysinfo::UpdateKind::Always); let mut system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::nothing().with_processes(kind)); std::thread::sleep(std::time::Duration::from_millis(v0)); system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, kind); std::rc::Rc::new(system.processes().iter().map(|(pid, process)| { let command = process.cmd().iter().map(|part| part.to_string_lossy().into_owned()).collect::<Vec<_>>().join(" "); (pid.as_u32() as u64, process.parent().map(|parent| parent.as_u32() as u64).unwrap_or(0), std::rc::Rc::<str>::from(process.name().to_string_lossy().trim_end_matches(".exe")), std::rc::Rc::<str>::from(command.as_str()), process.cpu_usage() as f64 / 100.0, process.run_time() / 60, process.accumulated_cpu_time() / 60000, process.memory() / 1048576) }).collect::<Vec<_>>()) };
    v1.clone()
}
fn flag_present_9(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> bool {
    loop {
        let v3: bool = v1 < v2;
        if v3 {
            let v4: Rc<str> = argument_5(v1);
            let v5: bool = &*v4 == &*v0;
            if v5 {
                return true;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v6, v2);
                continue;
            }
        } else {
            return false;
        }
    }
}
fn session_root_10(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
    loop {
        let v3: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
        let v4: bool = v3 < 0i32;
        let v6: bool = if v4 {
            true
        } else {
            let v5: bool = v2 > 32i32;
            v5
        };
        if v6 {
            return 0u64;
        } else {
            let v7: Rc<str> = v0[v3 as usize].2.clone();
            let v8: bool = &*v7 == "claude";
            let v10: bool = if v8 {
                true
            } else {
                let v9: bool = &*v7 == "codex";
                v9
            };
            if v10 {
                return v1;
            } else {
                let v11: u64 = v0[v3 as usize].1;
                let v12: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v11, v12);
                continue;
            }
        }
    }
}
fn local_offset_minutes_11() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn method14(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method14(v2.clone(), v3.clone());
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn descends_from_15(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
    loop {
        let v4: bool = v1 == v2;
        if v4 {
            return true;
        } else {
            let v5: bool = v1 == 0u64;
            let v7: bool = if v5 {
                true
            } else {
                let v6: bool = v3 > 64i32;
                v6
            };
            if v7 {
                return false;
            } else {
                let v8: i32 = v0.iter().position(|row| row.0 == v1).map(|at| at as i32).unwrap_or(-1);
                let v9: bool = v8 < 0i32;
                if v9 {
                    return false;
                } else {
                    let v10: u64 = v0[v8 as usize].1;
                    let v11: i32 = v3.wrapping_add(1i32);
                    (v0, v1, v2, v3) = (v0.clone(), v10, v2, v11);
                    continue;
                }
            }
        }
    }
}
fn pattern_matches_16(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = { static PATTERN: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new(); PATTERN.get_or_init(|| regex::RegexBuilder::new(&*v0).case_insensitive(true).build().ok()).as_ref().map(|found| found.is_match(&*v1)).unwrap_or(false) };
    v2
}
fn children_listed_17(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: u64, mut v2: i32, mut v3: i64, mut v4: i32, mut v5: Rc<UH0>, mut v6: Rc<str>, mut v7: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let v8: i32 = v0.len() as i32;
    let v9: bool = v4 >= v8;
    let (mut v53, mut v54, mut v55): (Rc<UH0>, Rc<str>, u64) = if v9 {
        (v5.clone(), v6.clone(), v7)
    } else {
        let v10: u64 = v0[v4 as usize].0;
        let v11: u64 = v0[v4 as usize].1;
        let v12: bool = v11 == v1;
        let v14: bool = if v12 {
            let v13: bool = v10 != v1;
            v13
        } else {
            false
        };
        let v21: bool = if v14 {
            let v15: Rc<str> = format_real_13(v10);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v15)), v16));
            let v19: bool = v6.contains(&*v18);
            let v20: bool = v19 == false;
            v20
        } else {
            false
        };
        let (mut v46, mut v47, mut v48): (Rc<UH0>, Rc<str>, u64) = if v21 {
            let v22: i32 = v2.wrapping_add(1i32);
            let v23: i32 = 0i32;
            let v24: u64 = v0[v4 as usize].0;
            let v25: Rc<str> = format_real_13(v24);
            let v26: u64 = v0[v4 as usize].1;
            let v27: Rc<str> = format_real_13(v26);
            let v28: u64 = v0[v4 as usize].5;
            let v29: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v28 as i64) * 60 + v3 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let v30: u64 = v0[v4 as usize].7;
            let v31: Rc<str> = format_real_13(v30);
            let v32: f64 = v0[v4 as usize].4;
            let v33: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v32).as_str());
            let v34: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(v2 as usize).as_str());
            let v35: Rc<str> = v0[v4 as usize].3.clone();
            let v36: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v35; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v36));
            let v38: Rc<str> = format_real_13(v24);
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v38));
            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v40));
            let v42: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v25.clone(), v27.clone(), v29.clone(), v31.clone(), v33.clone(), v37.clone(), v5.clone()));
            let (mut v43, mut v44, mut v45): (Rc<UH0>, Rc<str>, u64) = children_listed_17(v0.clone(), v10, v22, v3, v23, v42.clone(), v41.clone(), v7);
            (v43.clone(), v44.clone(), v45)
        } else {
            (v5.clone(), v6.clone(), v7)
        };
        let v49: i32 = v4.wrapping_add(1i32);
        let (mut v50, mut v51, mut v52): (Rc<UH0>, Rc<str>, u64) = children_listed_17(v0.clone(), v1, v2, v3, v49, v46.clone(), v47.clone(), v48);
        (v50.clone(), v51.clone(), v52)
    };
    (v53.clone(), v54.clone(), v55)
}
fn listed_12(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: US1, mut v3: bool, mut v4: u64, mut v5: i64, mut v6: i32, mut v7: Rc<UH0>, mut v8: Rc<str>, mut v9: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let v10: i32 = v0.len() as i32;
    let v11: bool = v6 >= v10;
    let (mut v71, mut v72, mut v73): (Rc<UH0>, Rc<str>, u64) = if v11 {
        (v7.clone(), v8.clone(), v9)
    } else {
        let v12: u64 = v0[v6 as usize].0;
        let v13: bool = v12 != v4;
        let v20: bool = if v13 {
            let v14: Rc<str> = format_real_13(v12);
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v14)), v15));
            let v18: bool = v8.contains(&*v17);
            let v19: bool = v18 == false;
            v19
        } else {
            false
        };
        let v27: bool = if v20 {
            match &v2 {
                US1::US1_EveryProcess => {
                    true
                }
                US1::US1_NoSession => {
                    false
                }
                US1::US1_SessionTree(v21) => {
                    let v21: u64 = *v21;
                    let v22: i32 = 0i32;
                    descends_from_15(v0.clone(), v12, v21, v22)
                }
            }
        } else {
            false
        };
        let v34: bool = if v27 {
            let v28: Rc<str> = v0[v6 as usize].2.clone();
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            let v31: Rc<str> = v0[v6 as usize].3.clone();
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            pattern_matches_16(v1.clone(), v32.clone())
        } else {
            false
        };
        let (mut v64, mut v65, mut v66): (Rc<UH0>, Rc<str>, u64) = if v34 {
            let v35: u64 = v9.wrapping_add(1u64);
            let v36: u64 = v0[v6 as usize].0;
            let v37: Rc<str> = format_real_13(v36);
            let v38: u64 = v0[v6 as usize].1;
            let v39: Rc<str> = format_real_13(v38);
            let v40: u64 = v0[v6 as usize].5;
            let v41: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v40 as i64) * 60 + v5 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let v42: u64 = v0[v6 as usize].7;
            let v43: Rc<str> = format_real_13(v42);
            let v44: f64 = v0[v6 as usize].4;
            let v45: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v44).as_str());
            let v46: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(0i32 as usize).as_str());
            let v47: Rc<str> = v0[v6 as usize].3.clone();
            let v48: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v47; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v48));
            let v50: Rc<str> = format_real_13(v36);
            let v51: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v50));
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v52));
            let (mut v61, mut v62, mut v63): (Rc<UH0>, Rc<str>, u64) = if v3 {
                let v54: i32 = 1i32;
                let v55: i32 = 0i32;
                let v56: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v37.clone(), v39.clone(), v41.clone(), v43.clone(), v45.clone(), v49.clone(), v7.clone()));
                let (mut v57, mut v58, mut v59): (Rc<UH0>, Rc<str>, u64) = children_listed_17(v0.clone(), v12, v54, v5, v55, v56.clone(), v53.clone(), v35);
                (v57.clone(), v58.clone(), v59)
            } else {
                let v60: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v37.clone(), v39.clone(), v41.clone(), v43.clone(), v45.clone(), v49.clone(), v7.clone()));
                (v60.clone(), v53.clone(), v35)
            };
            (v61.clone(), v62.clone(), v63)
        } else {
            (v7.clone(), v8.clone(), v9)
        };
        let v67: i32 = v6.wrapping_add(1i32);
        let (mut v68, mut v69, mut v70): (Rc<UH0>, Rc<str>, u64) = listed_12(v0.clone(), v1.clone(), v2.clone(), v3, v4, v5, v67, v64.clone(), v65.clone(), v66);
        (v68.clone(), v69.clone(), v70)
    };
    (v71.clone(), v72.clone(), v73)
}
fn widths_of_19(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> (i32, i32, i32, i32, i32) {
    let (mut v38, mut v39, mut v40, mut v41, mut v42): (i32, i32, i32, i32, i32) = match &*v0 {
        UH0::UH0_NoPsRows => {
            (v1, v2, v3, v4, v5)
        }
        _ => {
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32) = match &*v0 {
                UH0::UH0_PsRowThen(v6, v7, v8, v9, v10, v11, v12) => {
                    let v6: Rc<str> = v6.clone();
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = v9.clone();
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = v11.clone();
                    let v12: Rc<UH0> = v12.clone();
                    let v13: i32 = (&*v6).chars().count() as i32;
                    let v14: bool = v1 < v13;
                    let v15: i32 = if v14 {
                        v13
                    } else {
                        v1
                    };
                    let v16: i32 = (&*v7).chars().count() as i32;
                    let v17: bool = v2 < v16;
                    let v18: i32 = if v17 {
                        v16
                    } else {
                        v2
                    };
                    let v19: i32 = (&*v8).chars().count() as i32;
                    let v20: bool = v3 < v19;
                    let v21: i32 = if v20 {
                        v19
                    } else {
                        v3
                    };
                    let v22: i32 = (&*v9).chars().count() as i32;
                    let v23: bool = v4 < v22;
                    let v24: i32 = if v23 {
                        v22
                    } else {
                        v4
                    };
                    let v25: i32 = (&*v10).chars().count() as i32;
                    let v26: bool = v5 < v25;
                    let v27: i32 = if v26 {
                        v25
                    } else {
                        v5
                    };
                    widths_of_19(v12, v15, v18, v21, v24, v27)
                }
                _ => unreachable!(),
            };
            (v33, v34, v35, v36, v37)
        }
    };
    (v38, v39, v40, v41, v42)
}
fn rows_text_20(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_NoPsRows => {
                return v6.clone();
            }
            UH0::UH0_PsRowThen(v7, v8, v9, v10, v11, v12, v13) => {
                let v7: Rc<str> = v7.clone();
                let v8: Rc<str> = v8.clone();
                let v9: Rc<str> = v9.clone();
                let v10: Rc<str> = v10.clone();
                let v11: Rc<str> = v11.clone();
                let v12: Rc<str> = v12.clone();
                let v13: Rc<UH0> = v13.clone();
                let v14: bool = &*v6 == "";
                let v50: Rc<str> = if v14 {
                    let v15: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                    let v18: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v18)), v16));
                    let v21: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v21)), v16));
                    let v24: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v23, v24)), v16));
                    let v27: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v26, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v27, v16)), v12))));
                    v30.clone()
                } else {
                    let v31: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    let v34: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v33, v34)), v32));
                    let v37: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v32));
                    let v40: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v39, v40)), v32));
                    let v43: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v42, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v43, v32)), v12))));
                    let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                    let v49: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v47)), v6));
                    v49.clone()
                };
                (v0, v1, v2, v3, v4, v5, v6) = (v13.clone(), v1, v2, v3, v4, v5, v50.clone());
                continue;
            }
        }
    }
}
fn listing_text_18(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pid"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = (&*v3).chars().count() as i32;
    let v5: bool = 0i32 < v4;
    let v6: i32 = if v5 {
        v4
    } else {
        0i32
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ppid"); } LIT.with(|lit| lit.clone()) };
    let v8: i32 = (&*v7).chars().count() as i32;
    let v9: bool = 0i32 < v8;
    let v10: i32 = if v9 {
        v8
    } else {
        0i32
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("started"); } LIT.with(|lit| lit.clone()) };
    let v12: i32 = (&*v11).chars().count() as i32;
    let v13: bool = 0i32 < v12;
    let v14: i32 = if v13 {
        v12
    } else {
        0i32
    };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mem_mb"); } LIT.with(|lit| lit.clone()) };
    let v16: i32 = (&*v15).chars().count() as i32;
    let v17: bool = 0i32 < v16;
    let v18: i32 = if v17 {
        v16
    } else {
        0i32
    };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cores"); } LIT.with(|lit| lit.clone()) };
    let v20: i32 = (&*v19).chars().count() as i32;
    let v21: bool = 0i32 < v20;
    let v22: i32 = if v21 {
        v20
    } else {
        0i32
    };
    let (mut v23, mut v24, mut v25, mut v26, mut v27): (i32, i32, i32, i32, i32) = widths_of_19(v0.clone(), v6, v10, v14, v18, v22);
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = rows_text_20(v0.clone(), v23, v24, v25, v26, v27, v28.clone());
    let v30: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v3, width = v23 as usize).as_str());
    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
    let v33: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v24 as usize).as_str());
    let v35: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v33)), v31));
    let v36: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v25 as usize).as_str());
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v31));
    let v39: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v15, width = v26 as usize).as_str());
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v38, v39)), v31));
    let v42: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v19, width = v27 as usize).as_str());
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v31));
    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command"); } LIT.with(|lit| lit.clone()) };
    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v41, Rc::<str>::from(format!("{}{}", v43, v44))));
    let v47: bool = &*v29 == "";
    if v47 {
        v46.clone()
    } else {
        let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v46, v48)), v29));
        v50.clone()
    }
}
fn decimal_text_21(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_23(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn clock_text_24() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_11();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_22(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US2::US2_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_23(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_24();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn list_run_1(mut v0: u64) -> i32 {
    let v1: i32 = argument_count_2();
    let v2: i32 = 2i32;
    let v3: Rc<str> = pattern_from_4(v2, v1);
    let v4: Rc<str> = pattern_problem_7(v3.clone());
    let v5: bool = &*v4 != "";
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps: invalid pattern: "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v4));
        println!("{}", v7);
        2i32
    } else {
        let v8: u64 = 500u64;
        let v9: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = process_rows_sampled_8(v8);
        let v10: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64)>> = { let mut sorted = (*v9).clone(); sorted.sort_by(|left, right| right.5.cmp(&left.5).then(left.0.cmp(&right.0))); std::rc::Rc::new(sorted) };
        let v11: u64 = std::process::id() as u64;
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--mine"); } LIT.with(|lit| lit.clone()) };
        let v13: i32 = 2i32;
        let v14: bool = flag_present_9(v12.clone(), v13, v1);
        let v22: US1 = if v14 {
            let v15: i32 = 0i32;
            let v16: u64 = session_root_10(v10.clone(), v11, v15);
            let v17: bool = v16 == 0u64;
            if v17 {
                US1::US1_NoSession
            } else {
                US1::US1_SessionTree(v16)
            }
        } else {
            US1::US1_EveryProcess
        };
        match &v22 {
            US1::US1_NoSession => {
                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps --mine: no claude or codex process above this kit"); } LIT.with(|lit| lit.clone()) };
                println!("{}", v23);
                2i32
            }
            _ => {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--tree"); } LIT.with(|lit| lit.clone()) };
                let v25: i32 = 2i32;
                let v26: bool = flag_present_9(v24.clone(), v25, v1);
                let v27: i64 = local_offset_minutes_11();
                let v28: i32 = 0i32;
                let v29: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoPsRows); } CASE.with(|case| case.clone()) };
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v31: u64 = 0u64;
                let (mut v32, mut v33, mut v34): (Rc<UH0>, Rc<str>, u64) = listed_12(v10.clone(), v3.clone(), v22.clone(), v26, v11, v27, v28, v29.clone(), v30.clone(), v31);
                let v35: Rc<str> = listing_text_18(v32.clone(), v33.clone(), v34);
                println!("{}", v35);
                let v36: US2 = US2::US2_Succeeded;
                let v37: Rc<str> = decimal_text_21(v34);
                let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes "); } LIT.with(|lit| lit.clone()) };
                let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v37));
                let v46: Rc<str> = match &v22 {
                    US1::US1_EveryProcess => {
                        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        v40.clone()
                    }
                    US1::US1_SessionTree(v41) => {
                        let v41: u64 = *v41;
                        let v42: Rc<str> = format_real_13(v41);
                        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  session "); } LIT.with(|lit| lit.clone()) };
                        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v42));
                        v44.clone()
                    }
                    _ => unreachable!(),
                };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v46));
                receipt_line_22(v36.clone(), v47.clone(), v0);
                0i32
            }
        }
    }
}
fn now_ms_25() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn process_run_0() -> i32 {
    let mut v0: u64 = now_ms_25();
    let v1: i32 = 1i32;
    let v2: Rc<str> = argument_5(v1);
    let v3: bool = &*v2 == "ps";
    let v9: US3 = if v3 {
        US3::US3_ListProcesses
    } else {
        let v5: bool = &*v2 == "kill";
        if v5 {
            US3::US3_StopProcesses
        } else {
            US3::US3_ProcessUnknown
        }
    };
    match &v9 {
        US3::US3_ListProcesses => {
            list_run_1(v0)
        }
        US3::US3_ProcessUnknown => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit "); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
            println!("{}", v14);
            2i32
        }
        US3::US3_StopProcesses => {
            let v11: i32 = kit_process_kill::kit_process_kill_run();
            v11
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        process_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_process_run() -> i32 {
    closure0()()
}
