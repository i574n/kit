#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US2 {
    US2_SwitchJson,
}
#[derive(Clone)]
enum US1 {
    US1_Some(US2),
    US1_None,
}
#[derive(Clone)]
enum US4 {
    US4_ValuedMaxLines,
}
#[derive(Clone)]
enum US3 {
    US3_Some(US4),
    US3_None,
}
#[derive(Clone)]
enum US5 {
    US5_Some(u64),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_WholeFile,
    US6_FromTo(u64, u64),
    US6_OneLine(u64),
    US6_LastLines(u64),
}
pub struct Mut0 { pub l0: Rc<str> }
#[derive(Clone)]
enum US7 {
    US7_CommandRead,
    US7_CommandLines,
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Some(US7),
    US9_None,
}
fn decimal_text_3(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_5(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_7() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_6() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_7();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_4(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US0::US0_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_5(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_6();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn lines_receipt_2(mut v0: [u64; 4], mut v1: u64) -> i32 {
    let v2: u64 = 0u64;
    let mut v3: u64 = v0[v2 as usize];
    let mut v4: Rc<str> = decimal_text_3(v3);
    let v5: u64 = 1u64;
    let v6: u64 = v0[v5 as usize];
    let v7: Rc<str> = decimal_text_3(v6);
    let v8: u64 = 2u64;
    let v9: u64 = v0[v8 as usize];
    let v10: Rc<str> = decimal_text_3(v9);
    let v11: u64 = 3u64;
    let v12: u64 = v0[v11 as usize];
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v4));
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) }, v7));
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) }));
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v10));
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v16));
    let v18: bool = v12 == 0u64;
    if v18 {
        let v19: US0 = US0::US0_Succeeded;
        receipt_line_4(v19, v17, v1);
        0i32
    } else {
        let v20: US0 = US0::US0_Failed;
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) }));
        let v22: Rc<str> = decimal_text_3(v12);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
        receipt_line_4(v20, v23, v1);
        2i32
    }
}
fn read_receipt_9(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: Rc<str>, mut v4: bool, mut v5: u64) -> i32 {
    let v6: Rc<str> = decimal_text_3(v0);
    let v7: Rc<str> = decimal_text_3(v2);
    if v4 {
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"files\":["); } LIT.with(|lit| lit.clone()) }, v3));
        let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("],\"lines\":"); } LIT.with(|lit| lit.clone()) }));
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v7));
        let v11: Rc<str> = decimal_text_3(v1);
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"unreadable\":"); } LIT.with(|lit| lit.clone()) }, v11));
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v13));
        println!("{}", v14);
        let v15: bool = v1 == 0u64;
        if v15 {
            0i32
        } else {
            2i32
        }
    } else {
        let v17: bool = v1 == 0u64;
        if v17 {
            let v18: US0 = US0::US0_Succeeded;
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v6));
            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) }));
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v7));
            receipt_line_4(v18, v21, v5);
            0i32
        } else {
            let v22: US0 = US0::US0_Failed;
            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v6));
            let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  unreadable "); } LIT.with(|lit| lit.clone()) }));
            let v25: Rc<str> = decimal_text_3(v1);
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) }, v7));
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
            receipt_line_4(v22, v28, v5);
            2i32
        }
    }
}
fn method13(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US5> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US5> = Rc::new(move |mut v0: (u64)| -> US5 {
        let mut v1: u64 = (v0);
        US5::US5_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_12(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method13(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let v7: US5 = v6.clone();
    let v8: US5 = v4.unwrap_or(v7);
    match v8 {
        US5::US5_None => {
            0u64
        }
        US5::US5_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn flags_read_11(mut v0: Rc<UH0>, mut v1: u64, mut v2: bool, mut v3: bool, mut v4: Rc<UH0>) -> (u64, bool, bool, Rc<UH0>) {
    let (mut v58, mut v59, mut v60, mut v61): (u64, bool, bool, Rc<UH0>) = match &*v0 {
        UH0::UH0_Nil => {
            (v1, v2, v3, v4)
        }
        _ => {
            let (mut v54, mut v55, mut v56, mut v57): (u64, bool, bool, Rc<UH0>) = match Rc::unwrap_or_clone(v0) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH0> = v6;
                    let v7: bool = &*v5 == "--json";
                    let v11: US1 = if v7 {
                        let v8: US2 = US2::US2_SwitchJson;
                        US1::US1_Some(v8)
                    } else {
                        US1::US1_None
                    };
                    let v12: bool = &*v5 == "--max-lines";
                    let v16: US3 = if v12 {
                        let v13: US4 = US4::US4_ValuedMaxLines;
                        US3::US3_Some(v13)
                    } else {
                        US3::US3_None
                    };
                    match v11 {
                        US1::US1_Some(v17) => {
                            let v17: US2 = v17;
                            let v18: bool = true;
                            flags_read_11(v6, v1, v2, v18, v4)
                        }
                        _ => {
                            let (mut v46, mut v47, mut v48, mut v49): (u64, bool, bool, Rc<UH0>) = match v16 {
                                US3::US3_Some(v23) => {
                                    let v23: US4 = v23;
                                    match &*v6 {
                                        UH0::UH0_Cons(v24, v25) => {
                                            let v24: Rc<str> = v24.clone();
                                            let v25: Rc<UH0> = v25.clone();
                                            let v27: u64 = if v2 {
                                                v1
                                            } else {
                                                decimal_u64_12(v24)
                                            };
                                            let v28: bool = true;
                                            flags_read_11(v25, v27, v28, v3, v4)
                                        }
                                        _ => {
                                            let (mut v33, mut v34, mut v35, mut v36): (u64, bool, bool, Rc<UH0>) = match Rc::unwrap_or_clone(v6) {
                                                UH0::UH0_Nil => {
                                                    (v1, v2, v3, v4)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v33, v34, v35, v36)
                                        }
                                    }
                                }
                                _ => {
                                    let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v4));
                                    let (mut v42, mut v43, mut v44, mut v45): (u64, bool, bool, Rc<UH0>) = flags_read_11(v6, v1, v2, v3, v41);
                                    (v42, v43, v44, v45)
                                }
                            };
                            (v46, v47, v48, v49)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v54, v55, v56, v57)
        }
    };
    (v58, v59, v60, v61)
}
fn method14(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
                let mut v3: Rc<UH0> = v3;
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            UH0::UH0_Nil => {
                return v1;
            }
        }
    }
}
fn files_read_10(mut v0: Rc<UH0>) -> (u64, bool, bool, Rc<UH0>) {
    let mut v1: u64 = 400u64;
    let v2: bool = false;
    let mut v3: bool = false;
    let mut v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let (mut v5, mut v6, mut v7, mut v8): (u64, bool, bool, Rc<UH0>) = flags_read_11(v0, v1, v2, v3, v4);
    let v9: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v10: Rc<UH0> = method14(v8, v9);
    (v5, v6, v7, v10)
}
fn read_start_15() -> (u64, u64, u64, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, v0)
}
fn decimal_or_zero_18(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method13(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US5> = closure1();
    let mut v4: Option<US5> = v2.map(|x| v3(x));
    let v6: US5 = US5::US5_None;
    let v7: US5 = v6.clone();
    let v8: US5 = v4.unwrap_or(v7);
    match v8 {
        US5::US5_None => {
            0u64
        }
        US5::US5_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn read_spec_of_17(mut v0: Rc<str>) -> (Rc<str>, US6) {
    let mut v1: i64 = { let bytes = v0.as_bytes(); bytes.iter().rposition(|byte| *byte == 58u8).filter(|index| *index > 1 || !(bytes.len() > 1 && bytes[1] == 58u8)).map(|index| index as i64).unwrap_or(-1) };
    let v2: bool = v1 < 0i64;
    let (mut v45, mut v46): (Rc<str>, US6) = if v2 {
        let mut v3: US6 = US6::US6_WholeFile;
        (v0, v3)
    } else {
        let mut v4: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[(v1 as usize) + 1..]);
        let v5: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
        let v12: bool = if v5 {
            true
        } else {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v7: bool = v4.contains(&*v6);
            if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = Rc::<str>::from(v4.replace(&*v6, &*v8));
                let v10: bool = !v9.is_empty() && v9.bytes().all(|byte| byte.is_ascii_digit());
                v10
            } else {
                false
            }
        };
        let (mut v43, mut v44): (Rc<str>, US6) = if v12 {
            let v13: Rc<str> = std::rc::Rc::<str>::from(&(&*v0)[..v1 as usize]);
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v15: bool = v4.starts_with(&*v14);
            let v41: US6 = if v15 {
                let v16: i64 = 0i64;
                let v17: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[(v16 as usize) + 1..]);
                let v18: u64 = decimal_or_zero_18(v17);
                let v19: bool = v18 == 0u64;
                if v19 {
                    US6::US6_WholeFile
                } else {
                    US6::US6_LastLines(v18)
                }
            } else {
                let v23: bool = v4.contains(&*v14);
                if v23 {
                    let v24: i64 = v4.find(char::from(45u8)).map(|index| index as i64).unwrap_or(-1);
                    let v25: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[..v24 as usize]);
                    let v26: u64 = decimal_or_zero_18(v25);
                    let v27: Rc<str> = std::rc::Rc::<str>::from(&(&*v4)[(v24 as usize) + 1..]);
                    let v28: u64 = decimal_or_zero_18(v27);
                    let v29: bool = v26 == 0u64;
                    let v31: bool = if v29 {
                        true
                    } else {
                        let v30: bool = v28 < v26;
                        v30
                    };
                    if v31 {
                        US6::US6_WholeFile
                    } else {
                        US6::US6_FromTo(v26, v28)
                    }
                } else {
                    let v35: bool = !v4.is_empty() && v4.bytes().all(|byte| byte.is_ascii_digit());
                    if v35 {
                        let v36: u64 = decimal_or_zero_18(v4);
                        US6::US6_OneLine(v36)
                    } else {
                        US6::US6_WholeFile
                    }
                }
            };
            (v13, v41)
        } else {
            let v42: US6 = US6::US6_WholeFile;
            (v0, v42)
        };
        (v43, v44)
    };
    (v45, v46)
}
fn range_bounds_22(mut v0: US6, mut v1: u64) -> (u64, u64) {
    match v0 {
        US6::US6_FromTo(v2, v3) => {
            let v2: u64 = v2;
            let mut v3: u64 = v3;
            (v2, v3)
        }
        US6::US6_LastLines(v5) => {
            let v5: u64 = v5;
            let v6: bool = v5 >= v1;
            let v9: u64 = if v6 {
                1u64
            } else {
                let v7: u64 = v1.wrapping_sub(v5);
                let v8: u64 = v7.wrapping_add(1u64);
                v8
            };
            (v9, v1)
        }
        US6::US6_OneLine(v4) => {
            let mut v4: u64 = v4;
            (v4, v4)
        }
        US6::US6_WholeFile => {
            (1u64, v1)
        }
    }
}
fn method24(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_23(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method24(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn slice_json_25(mut v0: std::rc::Rc<Result<Vec<String>, String>>, mut v1: u64, mut v2: u64, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: bool = v1 <= v2;
        if v4 {
            let v5: Rc<str> = std::rc::Rc::<str>::from((*v0).as_ref().ok().and_then(|lines| lines.get((v1 - 1) as usize)).map(|line| line.as_str()).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v5).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v7: u64 = v1.wrapping_add(1u64);
            let v8: bool = &*v3 == "";
            let v11: Rc<str> = if v8 {
                v6
            } else {
                let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v3, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
                let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v6));
                v10
            };
            (v1, v3) = (v7, v11);
            continue;
        } else {
            return v3;
        }
    }
}
fn slice_printed_26(mut v0: std::rc::Rc<Result<Vec<String>, String>>, mut v1: u64, mut v2: u64, mut v3: u64) -> () {
    loop {
        let mut v4: bool = v1 <= v2;
        if v4 {
            let v5: Rc<str> = std::rc::Rc::<str>::from((*v0).as_ref().ok().and_then(|lines| lines.get((v1 - 1) as usize)).map(|line| line.as_str()).unwrap_or(""));
            let v6: Rc<str> = std::rc::Rc::<str>::from(format!("{:>width$}  {}", v1, &*v5, width = v3 as usize).as_str());
            println!("{}", v6);
            let v7: u64 = v1.wrapping_add(1u64);
            v1 = v7;
            continue;
        }
        return ();
    }
}
fn file_slice_21(mut v0: Rc<str>, mut v1: US6, mut v2: u64, mut v3: bool) -> (i64, Rc<str>) {
    let mut v4: std::rc::Rc<Result<Vec<String>, String>> = std::rc::Rc::new(std::fs::read(&*v0).map(|bytes| String::from_utf8_lossy(&bytes).lines().map(str::to_owned).collect::<Vec<_>>()).map_err(|error| error.to_string()));
    let v5: bool = (v4).is_err();
    let (mut v75, mut v76): (i64, Rc<str>) = if v5 {
        let v6: Rc<str> = std::rc::Rc::<str>::from((*v4).as_ref().err().map(|error| error.as_str()).unwrap_or(""));
        let (mut v17, mut v18): (i64, Rc<str>) = if v3 {
            let v7: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v0).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) }, v7));
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"error\":"); } LIT.with(|lit| lit.clone()) }));
            let v10: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v6).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
            let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) }));
            (-1i64, v12)
        } else {
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (unreadable: "); } LIT.with(|lit| lit.clone()) }));
            let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v6));
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v14, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
            println!("{}", v15);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (-1i64, v16)
        };
        (v17, v18)
    } else {
        let v19: u64 = (*v4).as_ref().map(|lines| lines.len() as u64).unwrap_or(0);
        let (mut v20, mut v21): (u64, u64) = range_bounds_22(v1, v19);
        let v22: u64 = (v20).max(1u64);
        let v23: u64 = (v21).min(v19);
        let v24: u64 = (v22).saturating_add(v2);
        let v25: u64 = (v24).saturating_sub(1u64);
        let v26: u64 = (v23).min(v25);
        let v27: bool = v22 <= v26;
        let v30: u64 = if v27 {
            let v28: u64 = v26.wrapping_sub(v22);
            let v29: u64 = v28.wrapping_add(1u64);
            v29
        } else {
            0u64
        };
        let v31: i64 = (v30 as i64);
        let (mut v73, mut v74): (i64, Rc<str>) = if v3 {
            let v32: Rc<str> = std::rc::Rc::<str>::from({ let q = char::from(34u8); let b = char::from(92u8); let mut out = String::from(q); for c in (v0).chars() { match c { c if c == q || c == b => { out.push(b); out.push(c); } c if (c as u32) < 32 => out.push_str(&format!("{b}u{:04x}", c as u32)), c => out.push(c) } } out.push(q); out }.as_str());
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) }, v32));
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"from\":"); } LIT.with(|lit| lit.clone()) }));
            let v35: Rc<str> = format_real_23(v22);
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v35));
            let v37: u64 = (v22).saturating_sub(1u64);
            let v38: u64 = (v26).max(v37);
            let v39: Rc<str> = format_real_23(v38);
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"to\":"); } LIT.with(|lit| lit.clone()) }, v39));
            let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"total\":"); } LIT.with(|lit| lit.clone()) }));
            let v42: Rc<str> = format_real_23(v19);
            let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v41, v42));
            let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v43));
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"more\":"); } LIT.with(|lit| lit.clone()) }));
            let v46: u64 = (v23).saturating_sub(v26);
            let v47: Rc<str> = format_real_23(v46);
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v47));
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"lines\":["); } LIT.with(|lit| lit.clone()) }));
            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v51: Rc<str> = slice_json_25(v4, v22, v26, v50);
            let v52: Rc<str> = Rc::<str>::from(format!("{}{}", v51, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) }));
            let v53: Rc<str> = Rc::<str>::from(format!("{}{}", v49, v52));
            (v31, v53)
        } else {
            let v54: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (lines "); } LIT.with(|lit| lit.clone()) }));
            let v55: Rc<str> = format_real_23(v22);
            let v56: Rc<str> = Rc::<str>::from(format!("{}{}", v54, v55));
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v56, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) }));
            let v58: u64 = (v22).saturating_sub(1u64);
            let v59: u64 = (v26).max(v58);
            let v60: Rc<str> = format_real_23(v59);
            let v61: Rc<str> = Rc::<str>::from(format!("{}{}", v60, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) }));
            let v62: Rc<str> = format_real_23(v19);
            let v63: Rc<str> = Rc::<str>::from(format!("{}{}", v61, v62));
            let v64: Rc<str> = Rc::<str>::from(format!("{}{}", v63, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v64));
            println!("{}", v65);
            let v66: u64 = (v26).to_string().len() as u64;
            slice_printed_26(v4, v22, v26, v66);
            let v67: bool = v26 < v23;
            if v67 {
                let v68: u64 = v23.wrapping_sub(v26);
                let v69: Rc<str> = format_real_23(v68);
                let v70: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("... "); } LIT.with(|lit| lit.clone()) }, v69));
                let v71: Rc<str> = Rc::<str>::from(format!("{}{}", v70, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" more lines (raise --max-lines or pass a range)"); } LIT.with(|lit| lit.clone()) }));
                println!("{}", v71);
                ()
            };
            let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (v31, v72)
        };
        (v73, v74)
    };
    (v75, v76)
}
fn slice_tallied_20(mut v0: Rc<str>, mut v1: US6, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let (mut v8, mut v9): (i64, Rc<str>) = file_slice_21(v0, v1, v2, v3);
    let v10: u64 = v4.wrapping_add(1u64);
    let v11: bool = &*v7 == "";
    let v14: Rc<str> = if v11 {
        v9
    } else {
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) }));
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v9));
        v13
    };
    let v15: bool = v8 < 0i64;
    let (mut v19, mut v20, mut v21, mut v22): (u64, u64, u64, Rc<str>) = if v15 {
        let v16: u64 = v5.wrapping_add(1u64);
        (v10, v16, v6, v14)
    } else {
        let v17: u64 = (v8 as u64);
        let v18: u64 = v6.wrapping_add(v17);
        (v10, v5, v18, v14)
    };
    (v19, v20, v21, v22)
}
fn globbed_from_27(mut v0: Rc<str>, mut v1: i32, mut v2: US6, mut v3: u64, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let v9: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).nth(v1 as usize).unwrap_or(""));
    let v10: bool = &*v9 == "";
    let (mut v20, mut v21, mut v22, mut v23): (u64, u64, u64, Rc<str>) = if v10 {
        (v5, v6, v7, v8)
    } else {
        let v11: i32 = v1.wrapping_add(1i32);
        let (mut v12, mut v13, mut v14, mut v15): (u64, u64, u64, Rc<str>) = slice_tallied_20(v9, v2.clone(), v3, v4, v5, v6, v7, v8);
        let (mut v16, mut v17, mut v18, mut v19): (u64, u64, u64, Rc<str>) = globbed_from_27(v0, v11, v2, v3, v4, v12, v13, v14, v15);
        (v16, v17, v18, v19)
    };
    (v20, v21, v22, v23)
}
fn spec_tallied_19(mut v0: Rc<str>, mut v1: US6, mut v2: u64, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**"); } LIT.with(|lit| lit.clone()) };
    let v9: bool = v0.contains(&*v8);
    let v12: Rc<str> = if v9 {
        let v10: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v0; let at = text.find("**").unwrap_or(0); let base = text[..at].trim_end_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let mask = text[at + 2..].trim_start_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let root = if base.is_empty() { std::path::PathBuf::from(".") } else { std::path::PathBuf::from(base) }; let mut found = Vec::new(); let mut pending = vec![root.clone()]; while let Some(folder) = pending.pop() { for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let name = entry.file_name().to_string_lossy().into_owned(); if path.is_dir() { if name != "target" && name != ".git" && name != "node_modules" { pending.push(path); } } else if name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len() { let relative = path.strip_prefix(&root).map(|rest| rest.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or(name); found.push(if base.is_empty() { relative } else { format!("{base}/{relative}") }); } } } found.sort(); found.join(&char::from(10u8).to_string()) }.as_str());
        v10
    } else {
        let v11: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v0; if !text.contains(char::from(42u8)) { text.to_owned() } else { let split = text.rfind(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (folder_text, separator, mask) = match split { Some(at) => (&text[..at], &text[at..at + 1], &text[at + 1..]), None => (".", "", text) }; let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let mut found = std::fs::read_dir(folder_text).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len()).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found.into_iter().map(|name| if separator.is_empty() { name } else { format!("{folder_text}{separator}{name}") }).collect::<Vec<_>>().join(&char::from(10u8).to_string()) } }.as_str());
        v11
    };
    let v13: bool = &*v12 == "";
    let (mut v23, mut v24, mut v25, mut v26): (u64, u64, u64, Rc<str>) = if v13 {
        let (mut v14, mut v15, mut v16, mut v17): (u64, u64, u64, Rc<str>) = slice_tallied_20(v0, v1, v2, v3, v4, v5, v6, v7);
        (v14, v15, v16, v17)
    } else {
        let v18: i32 = 0i32;
        let (mut v19, mut v20, mut v21, mut v22): (u64, u64, u64, Rc<str>) = globbed_from_27(v12, v18, v1, v2, v3, v4, v5, v6, v7);
        (v19, v20, v21, v22)
    };
    (v23, v24, v25, v26)
}
fn method16(mut v0: u64, mut v1: bool, mut v2: bool, mut v3: &Rc<UH0>, mut v4: Rc<UH0>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>) -> (u64, u64, u64, Rc<str>) {
    let (mut v25, mut v26, mut v27, mut v28): (u64, u64, u64, Rc<str>) = match &*v4 {
        UH0::UH0_Cons(v9, v10) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<UH0> = v10.clone();
            let (mut v11, mut v12): (Rc<str>, US6) = read_spec_of_17(v9);
            let (mut v13, mut v14, mut v15, mut v16): (u64, u64, u64, Rc<str>) = spec_tallied_19(v11, v12, v0, v2, v5, v6, v7, v8);
            method16(v0, v1, v2, v3, v10, v13, v14, v15, v16)
        }
        _ => {
            let (mut v21, mut v22, mut v23, mut v24): (u64, u64, u64, Rc<str>) = match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Nil => {
                    (v5, v6, v7, v8)
                }
                _ => unreachable!(),
            };
            (v21, v22, v23, v24)
        }
    };
    (v25, v26, v27, v28)
}
fn files_receipt_8(mut v0: Rc<UH0>, mut v1: u64) -> i32 {
    let (mut v2, mut v3, mut v4, mut v5): (u64, bool, bool, Rc<UH0>) = files_read_10(v0);
    let (mut v6, mut v7, mut v8, mut v9): (u64, u64, u64, Rc<str>) = read_start_15();
    let (mut v10, mut v11, mut v12, mut v13): (u64, u64, u64, Rc<str>) = method16(v2, v3, v4, &v5, v5.clone(), v6, v7, v8, v9);
    read_receipt_9(v10, v11, v12, v13, v4, v1)
}
fn method28(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: Rc<UH0>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            UH0::UH0_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<UH0> = v4;
                v0.borrow_mut().push(v3);
                let v5: i32 = v2.wrapping_add(1i32);
                (v1, v2) = (v4, v5);
                continue;
            }
            UH0::UH0_Nil => {
                return v2;
            }
        }
    }
}
fn expanded_from_29(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(10u8)).nth(v1 as usize).unwrap_or(""));
        let mut v4: i32 = (v0).split(char::from(10u8)).count() as i32;
        let v5: bool = v4 <= v1;
        if v5 {
            return v2;
        } else {
            let v6: bool = &*v3 == "";
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                v1 = v7;
                continue;
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("**"); } LIT.with(|lit| lit.clone()) };
                let v10: bool = v3.contains(&*v9);
                let v13: Rc<str> = if v10 {
                    let v11: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v3; let at = text.find("**").unwrap_or(0); let base = text[..at].trim_end_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let mask = text[at + 2..].trim_start_matches(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let root = if base.is_empty() { std::path::PathBuf::from(".") } else { std::path::PathBuf::from(base) }; let mut found = Vec::new(); let mut pending = vec![root.clone()]; while let Some(folder) = pending.pop() { for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() { let path = entry.path(); let name = entry.file_name().to_string_lossy().into_owned(); if path.is_dir() { if name != "target" && name != ".git" && name != "node_modules" { pending.push(path); } } else if name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len() { let relative = path.strip_prefix(&root).map(|rest| rest.to_string_lossy().replace(char::from(92u8), "/")).unwrap_or(name); found.push(if base.is_empty() { relative } else { format!("{base}/{relative}") }); } } } found.sort(); found.join(&char::from(10u8).to_string()) }.as_str());
                    v11
                } else {
                    let v12: Rc<str> = std::rc::Rc::<str>::from({ let text: &str = &*v3; if !text.contains(char::from(42u8)) { text.to_owned() } else { let split = text.rfind(|character: char| character == char::from(47u8) || character == char::from(92u8)); let (folder_text, separator, mask) = match split { Some(at) => (&text[..at], &text[at..at + 1], &text[at + 1..]), None => (".", "", text) }; let (head, tail) = mask.split_once(char::from(42u8)).unwrap_or((mask, "")); let mut found = std::fs::read_dir(folder_text).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_file()).map(|entry| entry.file_name().to_string_lossy().into_owned()).filter(|name| name.starts_with(head) && name.ends_with(tail) && name.len() >= head.len() + tail.len()).collect::<Vec<_>>()).unwrap_or_default(); found.sort(); found.into_iter().map(|name| if separator.is_empty() { name } else { format!("{folder_text}{separator}{name}") }).collect::<Vec<_>>().join(&char::from(10u8).to_string()) } }.as_str());
                    v12
                };
                let v14: i32 = v1.wrapping_add(1i32);
                let v15: bool = &*v13 == "";
                let v20: Rc<str> = if v15 {
                    v2
                } else {
                    let v16: bool = &*v2 == "";
                    if v16 {
                        v13
                    } else {
                        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v2, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
                        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v13));
                        v18
                    }
                };
                (v1, v2) = (v14, v20);
                continue;
            }
        }
    }
}
fn lines_counted_30(mut v0: Rc<str>) -> [u64; 4] {
    let mut v1: [u64; 4] = (|| -> [u64; 4] { let mut rows = Vec::new(); for shown in (&*v0).lines().filter(|line| !line.is_empty()) { rows.push((shown.to_owned(), std::fs::read(shown).map(|bytes| (bytes.len() as u64, String::from_utf8_lossy(&bytes).lines().count() as u64)).map_err(|error| error.to_string()))); } let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.as_ref().map(|counts| counts.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); println!("{:<path_width$}  {:<bytes_width$}  {}", "path", "bytes", "lines"); let mut tally = [0u64; 4]; for (shown, counts) in &rows { tally[0] += 1; match counts { Ok((bytes, lines)) => { tally[1] += bytes; tally[2] += lines; println!("{:<path_width$}  {:<bytes_width$}  {}", shown, bytes, lines); } Err(error) => { tally[3] += 1; println!("{:<path_width$}  {:<bytes_width$}  -  (unreadable: {error})", shown, "-"); } } } tally })();
    v1
}
fn command_ran_1(mut v0: US7, mut v1: Rc<UH0>, mut v2: u64) -> i32 {
    match v0 {
        US7::US7_CommandLines => {
            let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(Vec::new()));
            let v5: i32 = 0i32;
            let v6: i32 = method28(v4.clone(), v1, v5);
            let v7: Rc<Vec<Rc<str>>> = Rc::new(v4.borrow().clone());
            let v8: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new((v7).as_ref().clone()));
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from((*v8.borrow()).join(&*v9));
            let v11: i32 = 0i32;
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = expanded_from_29(v10, v11, v12);
            let v14: [u64; 4] = lines_counted_30(v13);
            lines_receipt_2(v14, v2)
        }
        US7::US7_CommandRead => {
            files_receipt_8(v1, v2)
        }
    }
}
fn now_ms_31() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_33() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_34(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_32(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_33();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US8 = if v4 {
        let v5: Rc<str> = command_line_argument_34(v0);
        US8::US8_Some(v5)
    } else {
        US8::US8_None
    };
    match v8 {
        US8::US8_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US8::US8_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_32(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn read_run_0() -> i32 {
    let mut v0: u64 = now_ms_31();
    let mut v1: i32 = 0i32;
    let v2: Rc<UH0> = arguments_from_32(v1);
    match Rc::unwrap_or_clone(v2) {
        UH0::UH0_Cons(v3, v4) => {
            let mut v3: Rc<str> = v3;
            let mut v4: Rc<UH0> = v4;
            match Rc::unwrap_or_clone(v4) {
                UH0::UH0_Cons(v5, v6) => {
                    let v5: Rc<str> = v5;
                    let v6: Rc<UH0> = v6;
                    let v7: bool = &*v3 == "read";
                    let v15: US9 = if v7 {
                        let v8: US7 = US7::US7_CommandRead;
                        US9::US9_Some(v8)
                    } else {
                        let v10: bool = &*v3 == "lines";
                        if v10 {
                            let v11: US7 = US7::US7_CommandLines;
                            US9::US9_Some(v11)
                        } else {
                            US9::US9_None
                        }
                    };
                    match v15 {
                        US9::US9_None => {
                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) }, v19));
                            eprintln!("{}", v20);
                            2i32
                        }
                        US9::US9_Some(v16) => {
                            let v16: US7 = v16;
                            let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v6));
                            command_ran_1(v16, v17, v0)
                        }
                    }
                }
                _ => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) }, v23));
                    eprintln!("{}", v24);
                    2i32
                }
            }
        }
        _ => {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) }, v26));
            eprintln!("{}", v27);
            2i32
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        read_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_read_run() -> i32 {
    closure0()()
}
pub type SpiralShape_97df63ebabc3 = Mut0;
