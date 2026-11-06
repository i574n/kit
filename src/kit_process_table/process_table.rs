#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_EveryProcess,
    US0_SessionTree(u64),
    US0_NoSession,
}
#[derive(Clone)]
enum UH0 {
    UH0_NoPsRows,
    UH0_PsRowThen(Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<str>, Rc<UH0>),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Succeeded,
    US1_Failed,
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<str>, Rc<UH1>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_ProcessPs,
    US3_ProcessKill,
    US3_ProcessUnknown,
}
#[derive(Clone)]
enum US5 {
    US5_PsTree,
    US5_PsMine,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
fn pattern_problem_2(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(regex::RegexBuilder::new(&*v0).case_insensitive(true).build().err().map(|error| error.to_string()).unwrap_or_default().as_str());
    v1
}
fn process_rows_sampled_3(mut v0: u64) -> std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> {
    let v1: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = kit_machine_sample::kit_machine_sampled(v0);
    v1
}
fn session_root_4(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: i32) -> u64 {
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
                (v1, v2) = (v11, v12);
                continue;
            }
        }
    }
}
fn local_offset_minutes_5() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn method8(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(&v2, v3);
    let v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn descends_from_9(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: u64, mut v3: i32) -> bool {
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
                    (v1, v3) = (v10, v11);
                    continue;
                }
            }
        }
    }
}
fn pattern_matches_10(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = { static PATTERN: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new(); PATTERN.get_or_init(|| regex::RegexBuilder::new(&*v0).case_insensitive(true).build().ok()).as_ref().map(|found| found.is_match(&*v1)).unwrap_or(false) };
    v2
}
fn children_listed_11(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: u64, mut v2: i32, mut v3: i64, mut v4: i32, mut v5: Rc<UH0>, mut v6: Rc<str>, mut v7: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let v8: i32 = v0.len() as i32;
    let v9: bool = v4 >= v8;
    let (mut v51, mut v52, mut v53): (Rc<UH0>, Rc<str>, u64) = if v9 {
        (v5, v6, v7)
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
        let v20: bool = if v14 {
            let v15: Rc<str> = format_real_7(v10);
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v15));
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let v18: bool = v6.contains(&*v17);
            let v19: bool = v18 == false;
            v19
        } else {
            false
        };
        let (mut v44, mut v45, mut v46): (Rc<UH0>, Rc<str>, u64) = if v20 {
            let v21: i32 = v2.wrapping_add(1i32);
            let v22: i32 = 0i32;
            let v23: u64 = v0[v4 as usize].0;
            let v24: Rc<str> = format_real_7(v23);
            let v25: u64 = v0[v4 as usize].1;
            let v26: Rc<str> = format_real_7(v25);
            let v27: u64 = v0[v4 as usize].5;
            let v28: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v27 as i64) * 60 + v3 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let v29: u64 = v0[v4 as usize].7;
            let v30: Rc<str> = format_real_7(v29);
            let v31: f64 = v0[v4 as usize].4;
            let v32: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v31).as_str());
            let v33: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(v2 as usize).as_str());
            let v34: Rc<str> = v0[v4 as usize].3.clone();
            let v35: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v34; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v35));
            let v37: Rc<str> = format_real_7(v23);
            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v37));
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v38, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let v40: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v24, v26, v28, v30, v32, v36, v5));
            let (mut v41, mut v42, mut v43): (Rc<UH0>, Rc<str>, u64) = children_listed_11(v0.clone(), v10, v21, v3, v22, v40, v39, v7);
            (v41, v42, v43)
        } else {
            (v5, v6, v7)
        };
        let v47: i32 = v4.wrapping_add(1i32);
        let (mut v48, mut v49, mut v50): (Rc<UH0>, Rc<str>, u64) = children_listed_11(v0, v1, v2, v3, v47, v44, v45, v46);
        (v48, v49, v50)
    };
    (v51, v52, v53)
}
fn listed_6(mut v0: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>>, mut v1: Rc<str>, mut v2: US0, mut v3: bool, mut v4: u64, mut v5: i64, mut v6: i32, mut v7: Rc<UH0>, mut v8: Rc<str>, mut v9: u64) -> (Rc<UH0>, Rc<str>, u64) {
    let v10: i32 = v0.len() as i32;
    let v11: bool = v6 >= v10;
    let (mut v68, mut v69, mut v70): (Rc<UH0>, Rc<str>, u64) = if v11 {
        (v7, v8, v9)
    } else {
        let v12: u64 = v0[v6 as usize].0;
        let v13: bool = v12 != v4;
        let v19: bool = if v13 {
            let v14: Rc<str> = format_real_7(v12);
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }, v14));
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let v17: bool = v8.contains(&*v16);
            let v18: bool = v17 == false;
            v18
        } else {
            false
        };
        let v26: bool = if v19 {
            match &v2 {
                US0::US0_EveryProcess => {
                    true
                }
                US0::US0_NoSession => {
                    false
                }
                US0::US0_SessionTree(v20) => {
                    let v20: u64 = *v20;
                    let v21: i32 = 0i32;
                    descends_from_9(v0.clone(), v12, v20, v21)
                }
            }
        } else {
            false
        };
        let v32: bool = if v26 {
            let v27: Rc<str> = v0[v6 as usize].2.clone();
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
            let v29: Rc<str> = v0[v6 as usize].3.clone();
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
            pattern_matches_10(v1.clone(), v30)
        } else {
            false
        };
        let (mut v61, mut v62, mut v63): (Rc<UH0>, Rc<str>, u64) = if v32 {
            let v33: u64 = v9.wrapping_add(1u64);
            let v34: u64 = v0[v6 as usize].0;
            let v35: Rc<str> = format_real_7(v34);
            let v36: u64 = v0[v6 as usize].1;
            let v37: Rc<str> = format_real_7(v36);
            let v38: u64 = v0[v6 as usize].5;
            let v39: Rc<str> = std::rc::Rc::<str>::from({ let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64; let local = (now - (v38 as i64) * 60 + v5 * 60).rem_euclid(86400); format!("{:02}:{:02}", local / 3600, local / 60 % 60) }.as_str());
            let v40: u64 = v0[v6 as usize].7;
            let v41: Rc<str> = format_real_7(v40);
            let v42: f64 = v0[v6 as usize].4;
            let v43: Rc<str> = std::rc::Rc::<str>::from(format!("{:.1}", v42).as_str());
            let v44: Rc<str> = std::rc::Rc::<str>::from("  ".repeat(0i32 as usize).as_str());
            let v45: Rc<str> = v0[v6 as usize].3.clone();
            let v46: Rc<str> = std::rc::Rc::<str>::from({ let text = &*v45; if text.chars().count() > 200 { format!("{}...", text.chars().take(200).collect::<String>()) } else { text.to_owned() } }.as_str());
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v46));
            let v48: Rc<str> = format_real_7(v34);
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v48));
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v49, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
            let (mut v58, mut v59, mut v60): (Rc<UH0>, Rc<str>, u64) = if v3 {
                let v51: i32 = 1i32;
                let v52: i32 = 0i32;
                let v53: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v35, v37, v39, v41, v43, v47, v7));
                let (mut v54, mut v55, mut v56): (Rc<UH0>, Rc<str>, u64) = children_listed_11(v0.clone(), v12, v51, v5, v52, v53, v50, v33);
                (v54, v55, v56)
            } else {
                let v57: Rc<UH0> = Rc::new(UH0::UH0_PsRowThen(v35, v37, v39, v41, v43, v47, v7));
                (v57, v50, v33)
            };
            (v58, v59, v60)
        } else {
            (v7, v8, v9)
        };
        let v64: i32 = v6.wrapping_add(1i32);
        let (mut v65, mut v66, mut v67): (Rc<UH0>, Rc<str>, u64) = listed_6(v0, v1, v2, v3, v4, v5, v64, v61, v62, v63);
        (v65, v66, v67)
    };
    (v68, v69, v70)
}
fn widths_of_13(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> (i32, i32, i32, i32, i32) {
    let (mut v38, mut v39, mut v40, mut v41, mut v42): (i32, i32, i32, i32, i32) = match &*v0 {
        UH0::UH0_NoPsRows => {
            (v1, v2, v3, v4, v5)
        }
        _ => {
            let (mut v33, mut v34, mut v35, mut v36, mut v37): (i32, i32, i32, i32, i32) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_PsRowThen(v6, v7, v8, v9, v10, v11, v12) => {
                    let v6: Rc<str> = v6;
                    let v7: Rc<str> = v7;
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = v9;
                    let v10: Rc<str> = v10;
                    let v11: Rc<str> = v11;
                    let v12: Rc<UH0> = v12;
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
                    widths_of_13(v12, v15, v18, v21, v24, v27)
                }
                _ => unreachable!(),
            };
            (v33, v34, v35, v36, v37)
        }
    };
    (v38, v39, v40, v41, v42)
}
fn rows_text_14(mut v0: Rc<UH0>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_NoPsRows => {
                return v6;
            }
            UH0::UH0_PsRowThen(v7, v8, v9, v10, v11, v12, v13) => {
                let v7: Rc<str> = v7;
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: Rc<str> = v10;
                let v11: Rc<str> = v11;
                let v12: Rc<str> = v12;
                let v13: Rc<UH0> = v13;
                let v14: bool = &*v6 == "";
                let v47: Rc<str> = if v14 {
                    let v15: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v17: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v20: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v23: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                    let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v26: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v26, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v12));
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v28));
                    v29
                } else {
                    let v30: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v1 as usize).as_str());
                    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v32: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v8, width = v2 as usize).as_str());
                    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
                    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v35: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v9, width = v3 as usize).as_str());
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
                    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v38: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v10, width = v4 as usize).as_str());
                    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
                    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v41: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v5 as usize).as_str());
                    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v12));
                    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v43));
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                    let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v6));
                    v46
                };
                (v0, v6) = (v13, v47);
                continue;
            }
        }
    }
}
fn listing_text_12(mut v0: Rc<UH0>, mut v1: &Rc<str>, mut v2: u64) -> Rc<str> {
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
    let (mut v23, mut v24, mut v25, mut v26, mut v27): (i32, i32, i32, i32, i32) = widths_of_13(v0.clone(), v6, v10, v14, v18, v22);
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v29: Rc<str> = rows_text_14(v0, v23, v24, v25, v26, v27, v28);
    let v30: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v3, width = v23 as usize).as_str());
    let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v32: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v7, width = v24 as usize).as_str());
    let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v32));
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v35: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v11, width = v25 as usize).as_str());
    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v38: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v15, width = v26 as usize).as_str());
    let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v37, v38));
    let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v41: Rc<str> = std::rc::Rc::<str>::from(format!("{:<width$}", &*v19, width = v27 as usize).as_str());
    let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v41, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("command"); } LIT.with(|lit| lit.clone()) }));
    let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v43));
    let v45: bool = &*v29 == "";
    if v45 {
        v44
    } else {
        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
        let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v29));
        v47
    }
}
fn decimal_text_15(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_17(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn clock_text_18() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_5();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_16(mut v0: US1, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US1::US1_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US1::US1_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_17(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_18();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn list_run_1(mut v0: u64, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<str>) -> i32 {
    let v5: Rc<str> = pattern_problem_2(v1.clone());
    let v6: bool = &*v4 != "";
    if v6 {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps: "); } LIT.with(|lit| lit.clone()) }, v4));
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: kit "); } LIT.with(|lit| lit.clone()) }));
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) }));
        println!("{}", v9);
        2i32
    } else {
        let v10: bool = &*v5 != "";
        if v10 {
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps: invalid pattern: "); } LIT.with(|lit| lit.clone()) }, v5));
            println!("{}", v11);
            2i32
        } else {
            let v12: u64 = 500u64;
            let v13: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = process_rows_sampled_3(v12);
            let v14: std::rc::Rc<Vec<(u64, u64, std::rc::Rc<str>, std::rc::Rc<str>, f64, u64, u64, u64, u64)>> = { let mut sorted = (*v13).clone(); sorted.sort_by(|left, right| right.5.cmp(&left.5).then(left.0.cmp(&right.0))); std::rc::Rc::new(sorted) };
            let v15: u64 = std::process::id() as u64;
            let v23: US0 = if v3 {
                let v16: i32 = 0i32;
                let v17: u64 = session_root_4(v14.clone(), v15, v16);
                let v18: bool = v17 == 0u64;
                if v18 {
                    US0::US0_NoSession
                } else {
                    US0::US0_SessionTree(v17)
                }
            } else {
                US0::US0_EveryProcess
            };
            match &v23 {
                US0::US0_NoSession => {
                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps --mine: no claude or codex process above this kit"); } LIT.with(|lit| lit.clone()) };
                    println!("{}", v24);
                    2i32
                }
                _ => {
                    let v25: i64 = local_offset_minutes_5();
                    let v26: i32 = 0i32;
                    let v27: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_NoPsRows); } CASE.with(|case| case.clone()) };
                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                    let v29: u64 = 0u64;
                    let (mut v30, mut v31, mut v32): (Rc<UH0>, Rc<str>, u64) = listed_6(v14, v1, v23.clone(), v2, v15, v25, v26, v27, v28, v29);
                    let v33: Rc<str> = listing_text_12(v30, &v31, v32);
                    println!("{}", v33);
                    let v34: US1 = US1::US1_Succeeded;
                    let v35: Rc<str> = decimal_text_15(v32);
                    let v36: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("processes "); } LIT.with(|lit| lit.clone()) }, v35));
                    let v42: Rc<str> = match v23 {
                        US0::US0_EveryProcess => {
                            let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            v37
                        }
                        US0::US0_SessionTree(v38) => {
                            let v38: u64 = v38;
                            let v39: Rc<str> = format_real_7(v38);
                            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  session "); } LIT.with(|lit| lit.clone()) }, v39));
                            v40
                        }
                        _ => unreachable!(),
                    };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v42));
                    receipt_line_16(v34, v43, v0);
                    0i32
                }
            }
        }
    }
}
fn now_ms_19() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_21() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_20(mut v0: i32) -> Rc<UH1> {
    let v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_21();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US2 = if v4 {
        let v5: Rc<str> = command_line_argument_22(v0);
        US2::US2_Some(v5)
    } else {
        US2::US2_None
    };
    match v8 {
        US2::US2_None => {
            { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) }
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH1> = arguments_from_20(v11);
            Rc::new(UH1::UH1_Cons(v10, v12))
        }
    }
}
fn ps_switched_25(mut v0: US5, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<str>) -> (Rc<str>, bool, bool, Rc<str>) {
    let (mut v9, mut v10, mut v11, mut v12): (Rc<str>, bool, bool, Rc<str>) = match &v0 {
        US5::US5_PsTree => {
            (v1, true, v3, v4)
        }
        _ => {
            let (mut v5, mut v6, mut v7, mut v8): (Rc<str>, bool, bool, Rc<str>) = match v0 {
                US5::US5_PsMine => {
                    (v1, v2, true, v4)
                }
                _ => unreachable!(),
            };
            (v5, v6, v7, v8)
        }
    };
    (v9, v10, v11, v12)
}
fn ps_read_24(mut v0: Rc<UH1>, mut v1: Rc<str>, mut v2: bool, mut v3: bool, mut v4: Rc<str>) -> (Rc<str>, bool, bool, Rc<str>) {
    let (mut v54, mut v55, mut v56, mut v57): (Rc<str>, bool, bool, Rc<str>) = match &*v0 {
        UH1::UH1_Nil => {
            (v1, v2, v3, v4)
        }
        _ => {
            let (mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, bool, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                UH1::UH1_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH1> = v6;
                    let v7: bool = "--tree" == &*v5;
                    let v15: US4 = if v7 {
                        let v8: US5 = US5::US5_PsTree;
                        US4::US4_Some(v8)
                    } else {
                        let v10: bool = "--mine" == &*v5;
                        if v10 {
                            let v11: US5 = US5::US5_PsMine;
                            US4::US4_Some(v11)
                        } else {
                            US4::US4_None
                        }
                    };
                    match &v15 {
                        US4::US4_Some(v16) => {
                            let v16: US5 = v16.clone();
                            let (mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, bool, Rc<str>) = ps_switched_25(v16, v1, v2, v3, v4);
                            ps_read_24(v6, v17, v18, v19, v20)
                        }
                        _ => {
                            let (mut v42, mut v43, mut v44, mut v45): (Rc<str>, bool, bool, Rc<str>) = match v15 {
                                US4::US4_None => {
                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
                                    let v26: bool = v5.starts_with(&*v25);
                                    if v26 {
                                        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unknown option "); } LIT.with(|lit| lit.clone()) }, v5));
                                        (v1, v2, v3, v27)
                                    } else {
                                        let v28: bool = &*v1 == "";
                                        let (mut v34, mut v35, mut v36, mut v37): (Rc<str>, bool, bool, Rc<str>) = if v28 {
                                            let (mut v29, mut v30, mut v31, mut v32): (Rc<str>, bool, bool, Rc<str>) = ps_read_24(v6, v5, v2, v3, v4);
                                            (v29, v30, v31, v32)
                                        } else {
                                            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("one pattern only, received "); } LIT.with(|lit| lit.clone()) }, v5));
                                            (v1, v2, v3, v33)
                                        };
                                        (v34, v35, v36, v37)
                                    }
                                }
                                _ => unreachable!(),
                            };
                            (v42, v43, v44, v45)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v50, v51, v52, v53)
        }
    };
    (v54, v55, v56, v57)
}
fn ps_of_23(mut v0: Rc<UH1>) -> (Rc<str>, bool, bool, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: bool = false;
    let v3: bool = false;
    let (mut v4, mut v5, mut v6, mut v7): (Rc<str>, bool, bool, Rc<str>) = ps_read_24(v0, v1.clone(), v2, v3, v1);
    (v4, v5, v6, v7)
}
fn process_run_0() -> i32 {
    let mut v0: u64 = now_ms_19();
    let v1: i32 = 0i32;
    let v2: Rc<UH1> = arguments_from_20(v1);
    let (mut v9, mut v10): (Rc<str>, Rc<UH1>) = match Rc::unwrap_or_clone(v2) {
        UH1::UH1_Cons(v3, v4) => {
            let v3: Rc<str> = v3;
            let v4: Rc<UH1> = v4;
            (v3, v4)
        }
        UH1::UH1_Nil => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
            (v5, v6)
        }
    };
    let v11: bool = "ps" == &*v9;
    let v20: US3 = if v11 {
        US3::US3_ProcessPs
    } else {
        let v13: bool = "kill" == &*v9;
        if v13 {
            US3::US3_ProcessKill
        } else {
            US3::US3_ProcessUnknown
        }
    };
    match v20 {
        US3::US3_ProcessKill => {
            let v26: i32 = kit_process_kill::kit_process_kill_run();
            v26
        }
        US3::US3_ProcessPs => {
            let (mut v21, mut v22, mut v23, mut v24): (Rc<str>, bool, bool, Rc<str>) = ps_of_23(v10);
            list_run_1(v0, v21, v22, v23, v24)
        }
        US3::US3_ProcessUnknown => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("usage: kit ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]]"); } LIT.with(|lit| lit.clone()) };
            println!("{}", v27);
            2i32
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
pub type SpiralShape_97df63ebabc3 = Mut0;
