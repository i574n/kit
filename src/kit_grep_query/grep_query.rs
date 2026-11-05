#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub fn spiral_json_quote(s: &str) -> std::rc::Rc<str> {
    let (q, b) = (char::from(34u8), char::from(92u8));
    let mut out = String::from(q);
    for c in s.chars() {
        match c as u32 {
            34 | 92 => { out.push(b); out.push(c); }
            n @ (8 | 9 | 10 | 12 | 13) => { out.push(b); out.push(char::from(b"btn_fr"[(n - 8) as usize])); }
            n if n < 32 => out.push_str(&format!("{b}u{n:04x}")),
            _ => out.push(c),
        }
    }
    out.push(q);
    std::rc::Rc::from(out.as_str())
}

#[derive(Clone)]
enum US0 {
    US0_NoNeedleHint,
    US0_LiteralLooksLikePattern,
}
struct Mut0 { l0: Rc<str> }
#[derive(Clone)]
enum US1 {
    US1_Succeeded,
    US1_Failed,
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
}
#[derive(Clone)]
enum US3 {
    US3_SwitchIgnoreCase,
    US3_SwitchJson,
    US3_SwitchRegex,
}
#[derive(Clone)]
enum US2 {
    US2_Some(US3),
    US2_None,
}
#[derive(Clone)]
enum US5 {
    US5_ValuedGlob,
    US5_ValuedContext,
    US5_ValuedMax,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(u64),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_LiteralNeedle,
    US7_CaselessNeedle,
    US7_PatternNeedle(bool),
}
#[derive(Clone)]
enum US8 {
    US8_FolderRoot,
    US8_FileRoot,
    US8_MissingRoot,
}
#[derive(Clone)]
enum US9 {
    US9_Some(Rc<str>),
    US9_None,
}
fn counts_start_3() -> (i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, v0)
}
fn json_text_6(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = spiral_json_quote(&*v0);
    v1
}
fn method8(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_7(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method8(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn clipped_line_9(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32) -> Rc<str> {
    let (mut v4, mut v5, mut v6): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
    let v7: Rc<str> = v6.borrow()[v3 as usize].clone();
    let v8: i32 = (v7.clone().len() as i32);
    let mut v9: bool = v8 > 200i32;
    if v9 {
        let v10: usize = ((0i32) as usize);
        let v11: usize = ((200i32) as usize);
        let v12: Rc<str> = Rc::<str>::from(&(v7)[v10..v11]);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("..."); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        v14
    } else {
        v7
    }
}
fn json_matches_5(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> Rc<str> {
    loop {
        let mut v6: bool = v4 < v3;
        if v6 {
            let (mut v7, mut v8, mut v9): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v10: i32 = v8.borrow()[v4 as usize];
            let (mut v11, mut v12, mut v13): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v14: Rc<str> = json_text_6(v11.clone());
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v14));
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"line\":"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
            let v19: i32 = v10.wrapping_add(1i32);
            let v20: Rc<str> = format_real_7(v19);
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
            let v22: Rc<str> = clipped_line_9(v0, v1.clone(), v2, v10);
            let v23: Rc<str> = json_text_6(v22.clone());
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"text\":"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v21, Rc::<str>::from(format!("{}{}", v25, v26))));
            let v29: i32 = v4.wrapping_add(1i32);
            let v30: bool = &*v5 == "";
            let v34: Rc<str> = if v30 {
                v28.clone()
            } else {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v5, v31)), v28));
                v33.clone()
            };
            (v4, v5) = (v29, v34.clone());
            continue;
        } else {
            return v5.clone();
        }
    }
}
fn last_window_line_10(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32) -> i32 {
    loop {
        let v7: bool = v5 < v3;
        if v7 {
            let v8: i32 = v5.wrapping_add(1i32);
            let (mut v9, mut v10, mut v11): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v12: i32 = v10.borrow()[v5 as usize];
            let v13: i32 = v12.wrapping_add(v4);
            let (mut v14, mut v15, mut v16): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v17: i32 = (v16.clone().borrow().len() as i32);
            let v18: i32 = v17.wrapping_sub(1i32);
            let v19: bool = v13 < v18;
            let v20: i32 = if v19 {
                v13
            } else {
                v18
            };
            let v21: bool = v6 < v20;
            let v22: i32 = if v21 {
                v20
            } else {
                v6
            };
            (v5, v6) = (v8, v22);
            continue;
        } else {
            return v6;
        }
    }
}
fn taken_from_14(mut v0: Rc<RefCell<Vec<i32>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let mut v4: bool = v3 >= v1;
        let v7: bool = if v4 {
            true
        } else {
            let mut v5: i32 = (v0.clone().borrow().len() as i32);
            let mut v6: bool = v3 >= v5;
            v6
        };
        if v7 {
            return false;
        } else {
            let v8: i32 = v0.borrow()[v3 as usize];
            let mut v9: bool = v8 == v2;
            if v9 {
                return true;
            } else {
                let v10: i32 = v3.wrapping_add(1i32);
                v3 = v10;
                continue;
            }
        }
    }
}
fn taken_line_13(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32, mut v4: i32) -> bool {
    let (mut v5, mut v6, mut v7): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
    let v8: i32 = 0i32;
    taken_from_14(v6, v3, v4, v8)
}
fn closure1(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    Rc::new(move |mut v3: Rc<str>| -> Rc<str> {
        let mut v4: bool = v2 >= v0;
        if v4 {
            v3.clone()
        } else {
            let mut v5: i32 = v2.wrapping_add(1i32);
            let mut v6: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method16(v0, v1.clone(), v5);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v1));
            v6(v7.clone())
        }
    })
}
fn method16(mut v0: i32, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    closure1(v0, v1, v2)
}
fn numbered_15(mut v0: i32, mut v1: i32, mut v2: bool, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = format_real_7(v0);
    let mut v5: Rc<str> = Rc::<str>::from(((b' ') as char).to_string());
    let mut v6: i32 = (v4.clone().len() as i32);
    let v7: i32 = v1.wrapping_sub(v6);
    let v8: i32 = 0i32;
    let mut v9: Rc<dyn Fn(Rc<str>) -> Rc<str>> = method16(v7, v5, v8);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = v9(v10);
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v4));
    let v15: Rc<str> = if v2 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        v14
    };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v15));
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v16, v17)), v3));
    v19
}
fn window_lines_12(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32) -> () {
    loop {
        let v7: bool = v5 <= v6;
        if v7 {
            let v8: i32 = v5.wrapping_add(1i32);
            let mut v9: bool = taken_line_13(v0, v1.clone(), v2, v3, v5);
            let v10: Rc<str> = clipped_line_9(v0, v1.clone(), v2, v5);
            let v11: Rc<str> = numbered_15(v8, v4, v9, v10.clone());
            println!("{}", v11);
            v5 = v8;
            continue;
        }
        return ();
    }
}
fn windows_printed_11(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32) -> () {
    loop {
        let v8: bool = v6 < v3;
        if v8 {
            let (mut v9, mut v10, mut v11): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v12: i32 = v10.borrow()[v6 as usize];
            let v13: i32 = v12.wrapping_sub(v4);
            let v14: bool = v13 < 0i32;
            let v15: i32 = if v14 {
                0i32
            } else {
                v13
            };
            let v16: bool = 0i32 <= v7;
            let v19: bool = if v16 {
                let v17: i32 = v7.wrapping_add(1i32);
                let v18: bool = v15 < v17;
                v18
            } else {
                false
            };
            let v21: i32 = if v19 {
                let v20: i32 = v7.wrapping_add(1i32);
                v20
            } else {
                v15
            };
            let v22: i32 = v12.wrapping_add(v4);
            let (mut v23, mut v24, mut v25): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v2 as usize].clone();
            let v26: i32 = (v25.clone().borrow().len() as i32);
            let v27: i32 = v26.wrapping_sub(1i32);
            let v28: bool = v22 < v27;
            let v29: i32 = if v28 {
                v22
            } else {
                v27
            };
            window_lines_12(v0, v1.clone(), v2, v3, v5, v21, v29);
            let v30: i32 = v6.wrapping_add(1i32);
            let v31: bool = v7 < v29;
            let v32: i32 = if v31 {
                v29
            } else {
                v7
            };
            (v6, v7) = (v30, v32);
            continue;
        }
        return ();
    }
}
fn files_rendered_4(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: bool, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: i32, mut v8: Rc<str>) -> (i32, i32, Rc<str>) {
    let mut v9: i32 = (v1.clone().borrow().len() as i32);
    let v10: bool = v9 <= v5;
    let (mut v49, mut v50, mut v51): (i32, i32, Rc<str>) = if v10 {
        (v6, v7, v8.clone())
    } else {
        let (mut v11, mut v12, mut v13): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v5 as usize].clone();
        let v14: i32 = (v12.borrow().len() as i32);
        let v15: i32 = v7.wrapping_add(v14);
        let v16: bool = v4 <= v6;
        let (mut v46, mut v47, mut v48): (i32, i32, Rc<str>) = if v16 {
            let v17: i32 = v5.wrapping_add(1i32);
            let (mut v18, mut v19, mut v20): (i32, i32, Rc<str>) = files_rendered_4(v0, v1.clone(), v2, v3, v4, v17, v6, v15, v8.clone());
            (v18, v19, v20)
        } else {
            let v21: i32 = v4.wrapping_sub(v6);
            let v22: bool = v14 < v21;
            let v23: i32 = if v22 {
                v14
            } else {
                v21
            };
            let (mut v39, mut v40, mut v41): (i32, i32, Rc<str>) = if v2 {
                let v24: i32 = v6.wrapping_add(v23);
                let v25: i32 = 0i32;
                let v26: Rc<str> = json_matches_5(v0, v1.clone(), v5, v23, v25, v8.clone());
                (v24, v15, v26)
            } else {
                let (mut v27, mut v28, mut v29): (Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>) = v1.borrow()[v5 as usize].clone();
                println!("{}", v27);
                let v30: i32 = 0i32;
                let v31: i32 = 0i32;
                let v32: i32 = last_window_line_10(v0, v1.clone(), v5, v23, v3, v30, v31);
                let v33: i32 = v32.wrapping_add(1i32);
                let v34: Rc<str> = format_real_7(v33);
                let v35: i32 = (v34.len() as i32);
                let v36: i32 = 0i32;
                let v37: i32 = -1i32;
                windows_printed_11(v0, v1.clone(), v5, v23, v3, v35, v36, v37);
                let v38: i32 = v6.wrapping_add(v23);
                (v38, v15, v8)
            };
            let v42: i32 = v5.wrapping_add(1i32);
            let (mut v43, mut v44, mut v45): (i32, i32, Rc<str>) = files_rendered_4(v0, v1, v2, v3, v4, v42, v39, v40, v41);
            (v43, v44, v45)
        };
        (v46, v47, v48)
    };
    (v49, v50, v51)
}
fn decimal_text_17(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_19(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_21() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_20() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_21();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_18(mut v0: US1, mut v1: Rc<str>, mut v2: u64) -> () {
    let mut v6: Rc<str> = match &v0 {
        US1::US1_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US1::US1_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_19(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_20();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn search_receipt_2(mut v0: u64, mut v1: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>, mut v2: bool, mut v3: i32, mut v4: i32, mut v5: US0, mut v6: u64) -> i32 {
    let v7: i32 = 0i32;
    let (mut v8, mut v9, mut v10): (i32, i32, Rc<str>) = counts_start_3();
    let (mut v11, mut v12, mut v13): (i32, i32, Rc<str>) = files_rendered_4(v0, v1.clone(), v2, v3, v4, v7, v8, v9, v10);
    let v14: Rc<str> = decimal_text_17(v0);
    let v15: i32 = (v1.borrow().len() as i32);
    let v16: Rc<str> = format_real_7(v15);
    if v2 {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"files\":"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v14));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matched\":"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v18, v19)), v16));
        let v22: Rc<str> = format_real_7(v12);
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"total\":"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v22));
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"shown\":"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
        let v27: Rc<str> = format_real_7(v11);
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v21, Rc::<str>::from(format!("{}{}", v26, v27))));
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matches\":["); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v29, v30)), v13));
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
        println!("{}", v34);
        0i32
    } else {
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v14));
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v36, v37)), v16));
        let v40: Rc<str> = format_real_7(v12);
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v39, Rc::<str>::from(format!("{}{}", v41, v40))));
        let v44: US1 = US1::US1_Succeeded;
        let v45: bool = v11 < v12;
        let v50: Rc<str> = if v45 {
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  shown "); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v43, v46));
            let v48: Rc<str> = format_real_7(v11);
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
            v49
        } else {
            v43
        };
        let v51: bool = v12 != 0i32;
        let v57: Rc<str> = if v51 {
            let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v52
        } else {
            match &v5 {
                US0::US0_LiteralLooksLikePattern => {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  hint: the needle is literal; pass --regex to use it as a pattern"); } LIT.with(|lit| lit.clone()) };
                    v54
                }
                US0::US0_NoNeedleHint => {
                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v53
                }
            }
        };
        let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v57));
        receipt_line_18(v44, v58, v6);
        0i32
    }
}
fn now_ms_22() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn search_start_23() -> (Rc<str>, u64, u64, bool, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, 0u64, 200u64, false, false, false)
}
fn switch_of_25(mut v0: Rc<str>) -> US2 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ignore-case"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--ignore-case"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = &*v0 == &*v3;
    if v4 {
        let mut v5: US3 = US3::US3_SwitchIgnoreCase;
        US2::US2_Some(v5)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--json"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = &*v0 == &*v8;
        if v9 {
            let v10: US3 = US3::US3_SwitchJson;
            US2::US2_Some(v10)
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("regex"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--regex"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = &*v0 == &*v13;
            if v14 {
                let v15: US3 = US3::US3_SwitchRegex;
                US2::US2_Some(v15)
            } else {
                US2::US2_None
            }
        }
    }
}
fn switch_applied_26(mut v0: US3, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: bool, mut v5: bool, mut v6: bool) -> (Rc<str>, u64, u64, bool, bool, bool) {
    let (mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
        US3::US3_SwitchIgnoreCase => {
            (v1.clone(), v2, v3, true, v5, v6)
        }
        _ => {
            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
                US3::US3_SwitchJson => {
                    (v1.clone(), v2, v3, v4, v5, true)
                }
                _ => {
                    let (mut v7, mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
                        US3::US3_SwitchRegex => {
                            (v1, v2, v3, v4, true, v6)
                        }
                        _ => unreachable!(),
                    };
                    (v7, v8, v9, v10, v11, v12)
                }
            };
            (v13, v14, v15, v16, v17, v18)
        }
    };
    (v19, v20, v21, v22, v23, v24)
}
fn valued_of_27(mut v0: Rc<str>) -> US4 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("glob"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--glob"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = &*v0 == &*v3;
    if v4 {
        let mut v5: US5 = US5::US5_ValuedGlob;
        US4::US4_Some(v5)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("context"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--context"); } LIT.with(|lit| lit.clone()) };
        let mut v9: bool = &*v0 == &*v8;
        if v9 {
            let v10: US5 = US5::US5_ValuedContext;
            US4::US4_Some(v10)
        } else {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--max"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = &*v0 == &*v13;
            if v14 {
                let v15: US5 = US5::US5_ValuedMax;
                US4::US4_Some(v15)
            } else {
                US4::US4_None
            }
        }
    }
}
fn method30(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure2() -> Rc<dyn Fn((u64)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US6> = Rc::new(move |mut v0: (u64)| -> US6 {
        let v1: u64 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_29(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method30(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US6> = closure2();
    let mut v4: Option<US6> = v2.map(|x| v3(x));
    let mut v6: US6 = US6::US6_None;
    let v7: US6 = v6.clone();
    let v8: US6 = v4.unwrap_or(v7);
    match &v8 {
        US6::US6_None => {
            0u64
        }
        US6::US6_Some(v9) => {
            let mut v9: u64 = *v9;
            v9
        }
    }
}
fn valued_applied_28(mut v0: US5, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool, mut v7: bool) -> (Rc<str>, u64, u64, bool, bool, bool) {
    let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
        US5::US5_ValuedGlob => {
            (v1.clone(), v3, v4, v5, v6, v7)
        }
        _ => {
            let (mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
                US5::US5_ValuedContext => {
                    let v8: u64 = decimal_u64_29(v1.clone());
                    (v2.clone(), v8, v4, v5, v6, v7)
                }
                _ => {
                    let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, u64, u64, bool, bool, bool) = match &v0 {
                        US5::US5_ValuedMax => {
                            let mut v9: u64 = decimal_u64_29(v1);
                            (v2, v3, v9, v5, v6, v7)
                        }
                        _ => unreachable!(),
                    };
                    (v10, v11, v12, v13, v14, v15)
                }
            };
            (v16, v17, v18, v19, v20, v21)
        }
    };
    (v22, v23, v24, v25, v26, v27)
}
fn options_read_24(mut v0: Rc<UH0>, mut v1: Rc<str>, mut v2: u64, mut v3: u64, mut v4: bool, mut v5: bool, mut v6: bool) -> (Rc<str>, u64, u64, bool, bool, bool) {
    let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92): (Rc<str>, u64, u64, bool, bool, bool) = match &*v0 {
        UH0::UH0_Nil => {
            (v1.clone(), v2, v3, v4, v5, v6)
        }
        _ => {
            let (mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (Rc<str>, u64, u64, bool, bool, bool) = match &*v0 {
                UH0::UH0_Cons(v7, v8) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<UH0> = v8.clone();
                    let mut v9: US2 = switch_of_25(v7.clone());
                    match &v9 {
                        US2::US2_Some(v10) => {
                            let v10: US3 = v10.clone();
                            let (mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, u64, u64, bool, bool, bool) = switch_applied_26(v10, v1.clone(), v2, v3, v4, v5, v6);
                            options_read_24(v8.clone(), v11, v12, v13, v14, v15, v16)
                        }
                        _ => {
                            let (mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, u64, u64, bool, bool, bool) = match &v9 {
                                US2::US2_None => {
                                    let v23: US4 = valued_of_27(v7);
                                    match &v23 {
                                        US4::US4_Some(v24) => {
                                            let v24: US5 = v24.clone();
                                            match &*v8 {
                                                UH0::UH0_Cons(v25, v26) => {
                                                    let v25: Rc<str> = v25.clone();
                                                    let v26: Rc<UH0> = v26.clone();
                                                    let (mut v27, mut v28, mut v29, mut v30, mut v31, mut v32): (Rc<str>, u64, u64, bool, bool, bool) = valued_applied_28(v24, v25, v1.clone(), v2, v3, v4, v5, v6);
                                                    options_read_24(v26, v27, v28, v29, v30, v31, v32)
                                                }
                                                _ => {
                                                    let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (Rc<str>, u64, u64, bool, bool, bool) = match &*v8 {
                                                        UH0::UH0_Nil => {
                                                            (v1.clone(), v2, v3, v4, v5, v6)
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    (v39, v40, v41, v42, v43, v44)
                                                }
                                            }
                                        }
                                        _ => {
                                            let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, u64, u64, bool, bool, bool) = match &v23 {
                                                US4::US4_None => {
                                                    options_read_24(v8, v1, v2, v3, v4, v5, v6)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v57, v58, v59, v60, v61, v62)
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            };
                            (v69, v70, v71, v72, v73, v74)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v81, v82, v83, v84, v85, v86)
        }
    };
    (v87, v88, v89, v90, v91, v92)
}
fn directory_exists_31(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn file_exists_32(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn searched_in_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: bool) -> (u64, Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>) {
    let mut v5: Rc<RefCell<Vec<Rc<str>>>> = kit_grep_found::kit_grep_found_files(&v0, &v1);
    let mut v6: i32 = (v5.clone().borrow().len() as i32);
    let v7: u64 = (v6 as u64);
    let v8: Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>> = kit_grep_found::kit_grep_found_hits(&v0, v5, &v2, v3, v4);
    (v7, v8)
}
fn grep_searched_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>) -> i32 {
    let mut v3: u64 = now_ms_22();
    let (mut v4, mut v5, mut v6, mut v7, mut v8, mut v9): (Rc<str>, u64, u64, bool, bool, bool) = search_start_23();
    let (mut v10, mut v11, mut v12, mut v13, mut v14, mut v15): (Rc<str>, u64, u64, bool, bool, bool) = options_read_24(v2.clone(), v4.clone(), v5, v6, v7, v8, v9);
    let v16: bool = &*v0 == "";
    let v18: bool = if v16 {
        true
    } else {
        let v17: bool = &*v1 == "";
        v17
    };
    if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v21);
        2i32
    } else {
        let v26: US7 = if v14 {
            US7::US7_PatternNeedle(v13)
        } else {
            if v13 {
                US7::US7_CaselessNeedle
            } else {
                US7::US7_LiteralNeedle
            }
        };
        let v29: Rc<str> = if v14 {
            let v27: Rc<str> = kit_grep_found::kit_grep_found_problem(&v1);
            v27.clone()
        } else {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v28.clone()
        };
        let v30: bool = &*v29 != "";
        if v30 {
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep: invalid --regex pattern: "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v29));
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v32));
            eprintln!("{}", v34);
            2i32
        } else {
            let v35: bool = directory_exists_31(v0.clone());
            let v41: US8 = if v35 {
                US8::US8_FolderRoot
            } else {
                let v37: bool = file_exists_32(v0.clone());
                if v37 {
                    US8::US8_FileRoot
                } else {
                    US8::US8_MissingRoot
                }
            };
            match &v41 {
                US8::US8_MissingRoot => {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep root not found: "); } LIT.with(|lit| lit.clone()) };
                    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v42, v0));
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v43));
                    eprintln!("{}", v45);
                    2i32
                }
                _ => {
                    let (mut v51, mut v52): (bool, bool) = match &v26 {
                        US7::US7_CaselessNeedle => {
                            (true, false)
                        }
                        US7::US7_LiteralNeedle => {
                            (false, false)
                        }
                        US7::US7_PatternNeedle(v46) => {
                            let v46: bool = *v46;
                            (v46, true)
                        }
                    };
                    let (mut v53, mut v54): (u64, Rc<RefCell<Vec<(Rc<str>, Rc<RefCell<Vec<i32>>>, Rc<RefCell<Vec<Rc<str>>>>)>>>) = searched_in_33(v0.clone(), v10.clone(), v1.clone(), v51, v52);
                    let v55: i32 = (v11 as i32);
                    let v56: i32 = (v12 as i32);
                    let v57: bool = v14 == false;
                    let v72: bool = if v57 {
                        let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                        let v59: bool = v1.contains(&*v58);
                        let v62: bool = if v59 {
                            true
                        } else {
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".*"); } LIT.with(|lit| lit.clone()) };
                            let v61: bool = v1.contains(&*v60);
                            v61
                        };
                        let v65: bool = if v62 {
                            true
                        } else {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                            let v64: bool = v1.contains(&*v63);
                            v64
                        };
                        let v68: bool = if v65 {
                            true
                        } else {
                            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^"); } LIT.with(|lit| lit.clone()) };
                            let v67: bool = v1.starts_with(&*v66);
                            v67
                        };
                        if v68 {
                            true
                        } else {
                            let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$"); } LIT.with(|lit| lit.clone()) };
                            let v70: bool = v1.ends_with(&*v69);
                            v70
                        }
                    } else {
                        false
                    };
                    let v75: US0 = if v72 {
                        US0::US0_LiteralLooksLikePattern
                    } else {
                        US0::US0_NoNeedleHint
                    };
                    search_receipt_2(v53, v54.clone(), v15, v55, v56, v75.clone(), v3)
                }
            }
        }
    }
}
fn command_line_argument_count_35() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_36(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_34(mut v0: i32) -> Rc<UH0> {
    let v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let v2: i32 = command_line_argument_count_35();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US9 = if v4 {
        let mut v5: Rc<str> = command_line_argument_36(v0);
        US9::US9_Some(v5)
    } else {
        US9::US9_None
    };
    match &v8 {
        US9::US9_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US9::US9_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_34(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn grep_run_0() -> i32 {
    let mut v0: i32 = 0i32;
    let v1: Rc<UH0> = arguments_from_34(v0);
    match &*v1 {
        UH0::UH0_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH0> = v3.clone();
            match &*v3 {
                UH0::UH0_Cons(v4, v5) => {
                    let mut v4: Rc<str> = v4.clone();
                    let mut v5: Rc<UH0> = v5.clone();
                    match &*v5 {
                        UH0::UH0_Cons(v6, v7) => {
                            let mut v6: Rc<str> = v6.clone();
                            let v7: Rc<UH0> = v7.clone();
                            grep_searched_1(v4, v6, v7)
                        }
                        _ => {
                            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v10: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                            grep_searched_1(v9.clone(), v9, v10)
                        }
                    }
                }
                _ => {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v14: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                    grep_searched_1(v13.clone(), v13, v14)
                }
            }
        }
        _ => {
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            grep_searched_1(v17.clone(), v17, v18)
        }
    }
}
fn closure0() -> Rc<dyn Fn() -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> i32> = Rc::new(move || -> i32 {
        grep_run_0()
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_grep_run() -> i32 {
    closure0()()
}
