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
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v1)), v2)), v3)), v4));
    v8
}
fn git_run_2(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { let mut command = std::process::Command::new("git"); command.current_dir(std::path::Path::new(&*v0).join(&*v1)).args(["-c", "user.name=kit", "-c"]).arg(format!("user.email=kit{}localhost", char::from(64u8))).args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args((v2).split(char::from(10u8))).stdin(std::process::Stdio::null()); if !(v3).is_empty() { command.env("GIT_AUTHOR_DATE", &*v3).env("GIT_COMMITTER_DATE", &*v3); } let mark = char::from(30u8); std::rc::Rc::<str>::from(match command.output() { Ok(output) => format!("{}{mark}{}{mark}{}", output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr).trim()), Err(error) => format!("1{mark}{mark}git: {error}") }) };
    v4
}
fn part_at_4(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn succeeded_3(mut v0: Rc<str>) -> bool {
    let v1: i32 = 0i32;
    let v2: Rc<str> = part_at_4(v0, v1);
    let v3: bool = &*v2 == "0";
    v3
}
fn row_5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_1(v0, v2, v1, v3.clone(), v3)
}
fn go_8(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v5.ends_with(&*v7);
            let v9: bool = v8 == false;
            let v15: Rc<str> = if v9 {
                v5.clone()
            } else {
                let v10: bool = v6 == 1i32;
                if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                } else {
                    let v12: i32 = v6.wrapping_sub(2i32);
                    let v13: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v12 as i64);
                    v13.clone()
                }
            };
            let v16: i32 = v2.wrapping_add(1i32);
            let v17: bool = &*v15 == "";
            let v19: i32 = if v17 {
                v3
            } else {
                let v18: i32 = v3.wrapping_add(1i32);
                v18
            };
            (v2, v3) = (v16, v19);
            continue;
        }
    }
}
fn line_count_7(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let v6: Rc<str> = v2.borrow()[v5 as usize].clone();
        let v7: bool = &*v6 == "";
        v7
    } else {
        false
    };
    let v10: i32 = if v8 {
        let v9: i32 = v3.wrapping_sub(1i32);
        v9
    } else {
        v3
    };
    let v11: i32 = 0i32;
    let v12: i32 = 0i32;
    let v13: i32 = go_8(v2.clone(), v10, v11, v12);
    let v14: u64 = (v13 as u64);
    v14
}
fn go_10(mut v0: i32, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> (i32, Rc<str>) {
    loop {
        let v6: bool = v3 >= v2;
        if v6 {
            return (v4, v5.clone());
        } else {
            let v7: Rc<str> = v1.borrow()[v3 as usize].clone();
            let v8: i32 = (v7.clone().len() as i32);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v10: bool = v7.ends_with(&*v9);
            let v11: bool = v10 == false;
            let v17: Rc<str> = if v11 {
                v7.clone()
            } else {
                let v12: bool = v8 == 1i32;
                if v12 {
                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                } else {
                    let v14: i32 = v8.wrapping_sub(2i32);
                    let v15: Rc<str> = string_slice(&v7.clone(), 0i32 as i64, v14 as i64);
                    v15.clone()
                }
            };
            let v18: i32 = v3.wrapping_add(1i32);
            let v19: bool = &*v17 == "";
            let (mut v23, mut v24): (i32, Rc<str>) = if v19 {
                (v4, v5.clone())
            } else {
                let v20: i32 = v4.wrapping_add(1i32);
                let v21: bool = v4 == v0;
                let v22: Rc<str> = if v21 {
                    v17.clone()
                } else {
                    v5.clone()
                };
                (v20, v22.clone())
            };
            (v3, v4, v5) = (v18, v23, v24.clone());
            continue;
        }
    }
}
fn line_at_9(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: i32 = (v1 as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 > 0i32;
    let v10: bool = if v6 {
        let v7: i32 = v5.wrapping_sub(1i32);
        let v8: Rc<str> = v4.borrow()[v7 as usize].clone();
        let v9: bool = &*v8 == "";
        v9
    } else {
        false
    };
    let v12: i32 = if v10 {
        let v11: i32 = v5.wrapping_sub(1i32);
        v11
    } else {
        v5
    };
    let v13: i32 = 0i32;
    let v14: i32 = 0i32;
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v16, mut v17): (i32, Rc<str>) = go_10(v2, v4.clone(), v12, v13, v14, v15.clone());
    v17.clone()
}
fn tab_part_11(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn occurrences_12(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let v2: u64 = (v0).matches(&*v1).count() as u64;
    v2
}
fn joined_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0.clone()
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_1(v0, v4, v1, v5.clone(), v5)
        }
    }
}
fn history_rows_6(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let v7: u64 = line_count_7(v4.clone());
        let v8: bool = v5 < v7;
        if v8 {
            let v9: Rc<str> = line_at_9(v4.clone(), v5);
            let v10: i32 = 0i32;
            let v11: Rc<str> = tab_part_11(v9.clone(), v10);
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show\n"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = concat5_1(v12.clone(), v11.clone(), v13.clone(), v3.clone(), v14.clone());
            let v16: Rc<str> = git_run_2(v0.clone(), v1.clone(), v15.clone(), v14.clone());
            let v17: i32 = 1i32;
            let v18: Rc<str> = part_at_4(v16.clone(), v17);
            let v19: u64 = occurrences_12(v18.clone(), v2.clone());
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = concat5_1(v12.clone(), v11.clone(), v20.clone(), v3.clone(), v14.clone());
            let v22: Rc<str> = git_run_2(v0.clone(), v1.clone(), v21.clone(), v14.clone());
            let v23: i32 = 1i32;
            let v24: Rc<str> = part_at_4(v22.clone(), v23);
            let v25: u64 = occurrences_12(v24.clone(), v2.clone());
            let v26: bool = v19 < v25;
            let v29: Rc<str> = if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("removed"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("added"); } LIT.with(|lit| lit.clone()) };
                v28.clone()
            };
            let v30: i32 = 1i32;
            let v31: Rc<str> = tab_part_11(v9.clone(), v30);
            let v32: i32 = 2i32;
            let v33: Rc<str> = tab_part_11(v9.clone(), v32);
            let v34: Rc<str> = row_5(v29.clone(), v33.clone());
            let v35: Rc<str> = row_5(v31.clone(), v34.clone());
            let v36: Rc<str> = row_5(v11.clone(), v35.clone());
            let v37: u64 = v5.wrapping_add(1u64);
            let v38: Rc<str> = joined_13(v6.clone(), v36.clone());
            (v5, v6) = (v37, v38.clone());
            continue;
        } else {
            return v6.clone();
        }
    }
}
fn matching_lines_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default().lines().enumerate().filter(|(_, line)| line.contains(&*v2)).map(|(index, line)| format!("{}:{}  {}", &*v1, index + 1, line)).collect::<Vec<_>>().join("
"));
    v3.clone()
}
fn aligned_15(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn failure_row_16(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed: "); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = 2i32;
    let v4: Rc<str> = part_at_4(v1, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_1(v0, v2, v4, v5.clone(), v5)
}
fn history_report_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--abbrev=8\n--format=%h%x09%ad%x09%s\n--date=short\n-S"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--\n"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = concat5_1(v4, v2.clone(), v5, v3.clone(), v6.clone());
    let v8: Rc<str> = git_run_2(v0.clone(), v1.clone(), v7, v6.clone());
    let v9: bool = succeeded_3(v8.clone());
    if v9 {
        let v10: i32 = 1i32;
        let v11: Rc<str> = part_at_4(v8.clone(), v10);
        let v12: u64 = 0u64;
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit"); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("date"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("change"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subject"); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = row_5(v15, v16);
        let v18: Rc<str> = row_5(v14, v17);
        let v19: Rc<str> = row_5(v13, v18);
        let v20: Rc<str> = history_rows_6(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v11.clone(), v12, v19);
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = concat5_1(v1.clone(), v21, v3, v6.clone(), v6.clone());
        let v23: Rc<str> = matching_lines_14(v0, v22, v2);
        let v24: Rc<str> = aligned_15(v20);
        let v25: Rc<str> = joined_13(v24, v23.clone());
        let v26: u64 = line_count_7(v11);
        let v27: u64 = line_count_7(v23);
        (v25, false, v26, v27)
    } else {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history "); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = concat5_1(v28, v1, v6.clone(), v6.clone(), v6);
        let v30: Rc<str> = failure_row_16(v29, v8);
        (v30, true, 0u64, 0u64)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        history_report_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_counted_18(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut days = std::collections::BTreeMap::<&str, u64>::new(); for day in (v0).lines().map(str::trim).filter(|day| !day.is_empty()) { *days.entry(day).or_default() += 1; } std::rc::Rc::<str>::from(days.iter().rev().map(|(day, count)| format!("{day}{}{count}", char::from(31u8))).collect::<Vec<_>>().join("
")) };
    v1.clone()
}
fn days_report_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let v3: bool = (v2).len() == 10;
    let v4: bool = &*v2 == "";
    let v12: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--since="); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = if v3 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" 00:00:00"); } LIT.with(|lit| lit.clone()) };
            v7
        } else {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8
        };
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v6, v2, v9, v10.clone(), v10)
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--format=%ad\n--date=short"); } LIT.with(|lit| lit.clone()) };
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = concat5_1(v13, v12, v14.clone(), v14.clone(), v14.clone());
    let v16: Rc<str> = git_run_2(v0, v1.clone(), v15, v14.clone());
    let v17: bool = succeeded_3(v16.clone());
    if v17 {
        let v18: i32 = 1i32;
        let v19: Rc<str> = part_at_4(v16.clone(), v18);
        let v20: Rc<str> = days_counted_18(v19.clone());
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("day"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = row_5(v21, v22);
        let v24: Rc<str> = joined_13(v23, v20.clone());
        let v25: Rc<str> = aligned_15(v24);
        let v26: u64 = line_count_7(v19);
        let v27: u64 = line_count_7(v20);
        (v25, false, v26, v27)
    } else {
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git log "); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = concat5_1(v28, v1, v14.clone(), v14.clone(), v14);
        let v30: Rc<str> = failure_row_16(v29, v16);
        (v30, true, 0u64, 0u64)
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        days_report_17(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn porcelain_paths_21(mut v0: Rc<str>, mut v1: bool) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().filter(|line| line.len() > 3 && line.starts_with("??") == v1).map(|line| line[3..].trim_matches(char::from(34u8)).to_owned()).collect::<Vec<_>>().join("
"));
    v2.clone()
}
fn staged_from_23(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: i32 = (v0.clone().borrow().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2.clone();
        } else {
            let v5: Rc<str> = v0.borrow()[v1 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: bool = v6 > 3i32;
            let v12: Rc<str> = if v7 {
                let v8: usize = ((0i32) as usize);
                let v9: usize = ((1i32) as usize);
                let v10: Rc<str> = Rc::<str>::from(&(v5)[v8..v9]);
                v10.clone()
            } else {
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                v11.clone()
            };
            let v22: Rc<str> = if v7 {
                let v13: i32 = v6.wrapping_sub(3i32);
                let v14: usize = ((3i32) as usize);
                let v15: i32 = 3i32.wrapping_add(v13);
                let v16: usize = ((v15) as usize);
                let v17: Rc<str> = Rc::<str>::from(&(v5)[v14..v16]);
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v20: Rc<str> = Rc::<str>::from(v17.replace(&*v18, &*v19));
                v20.clone()
            } else {
                let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v21.clone()
            };
            let v23: i32 = v1.wrapping_add(1i32);
            let v24: bool = &*v12 == " ";
            let v26: bool = if v24 {
                true
            } else {
                let v25: bool = &*v12 == "?";
                v25
            };
            let v28: Rc<str> = if v26 {
                v2.clone()
            } else {
                joined_13(v2.clone(), v22.clone())
            };
            (v1, v2) = (v23, v28.clone());
            continue;
        }
    }
}
fn staged_paths_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = 0i32;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    staged_from_23(v2.clone(), v3, v4.clone())
}
fn decimal_text_25(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn state_row_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = &*v2 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: u64 = line_count_7(v2.clone());
        let v6: Rc<str> = decimal_text_25(v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v7, &v8));
        let v10: Rc<str> = row_5(v6, v9);
        let v11: Rc<str> = row_5(v1, v10);
        row_5(v0, v11)
    }
}
fn status_rows_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64) -> (Rc<str>, bool, u64, u64, u64) {
    loop {
        let v8: u64 = line_count_7(v1.clone());
        let v9: bool = v2 < v8;
        if v9 {
            let v10: Rc<str> = line_at_9(v1.clone(), v2);
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = git_run_2(v0.clone(), v10.clone(), v11.clone(), v12.clone());
            let v14: bool = succeeded_3(v13.clone());
            if v14 {
                let v15: i32 = 1i32;
                let v16: Rc<str> = part_at_4(v13.clone(), v15);
                let v17: bool = false;
                let v18: Rc<str> = porcelain_paths_21(v16.clone(), v17);
                let v19: bool = true;
                let v20: Rc<str> = porcelain_paths_21(v16.clone(), v19);
                let v21: bool = &*v16 == "";
                let v35: Rc<str> = if v21 {
                    let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clean"); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                    let v24: Rc<str> = row_5(v22.clone(), v23.clone());
                    row_5(v10.clone(), v24.clone())
                } else {
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("staged"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = staged_paths_22(v16.clone());
                    let v28: Rc<str> = state_row_24(v10.clone(), v26.clone(), v27.clone());
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modified"); } LIT.with(|lit| lit.clone()) };
                    let v30: Rc<str> = state_row_24(v10.clone(), v29.clone(), v18.clone());
                    let v31: Rc<str> = joined_13(v30.clone(), v28.clone());
                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("untracked"); } LIT.with(|lit| lit.clone()) };
                    let v33: Rc<str> = state_row_24(v10.clone(), v32.clone(), v20.clone());
                    joined_13(v31.clone(), v33.clone())
                };
                let v36: u64 = line_count_7(v18.clone());
                let v37: u64 = v6.wrapping_add(v36);
                let v38: u64 = line_count_7(v20.clone());
                let v39: u64 = v7.wrapping_add(v38);
                let v40: u64 = v2.wrapping_add(1u64);
                let v41: Rc<str> = joined_13(v3.clone(), v35.clone());
                let v42: u64 = v5.wrapping_add(1u64);
                (v2, v3, v5, v6, v7) = (v40, v41.clone(), v42, v37, v39);
                continue;
            } else {
                let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
                let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
                let v50: i32 = 2i32;
                let v51: Rc<str> = part_at_4(v13.clone(), v50);
                let v52: Rc<str> = row_5(v49.clone(), v51.clone());
                let v53: Rc<str> = row_5(v48.clone(), v52.clone());
                let v54: Rc<str> = row_5(v10.clone(), v53.clone());
                let v55: u64 = v2.wrapping_add(1u64);
                let v56: Rc<str> = joined_13(v3.clone(), v54.clone());
                let v57: bool = true;
                (v2, v3, v4) = (v55, v56.clone(), v57);
                continue;
            }
        } else {
            let v68: Rc<str> = aligned_15(v3.clone());
            return (v68.clone(), v4, v5, v6, v7);
        }
    }
}
fn status_report_19(mut v0: Rc<str>, mut v1: Rc<str>) -> (Rc<str>, bool, u64, u64, u64) {
    let v2: u64 = 0u64;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = row_5(v5, v6);
    let v8: Rc<str> = row_5(v4, v7);
    let v9: Rc<str> = row_5(v3, v8);
    let v10: bool = false;
    let v11: u64 = 0u64;
    let v12: u64 = 0u64;
    let v13: u64 = 0u64;
    status_rows_20(v0, v1, v2, v9, v10, v11, v12, v13)
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> (Rc<str>, bool, u64, u64, u64) {
        status_report_19(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_git_history(v0: &str, v1: &str, v2: &str, v3: &str) -> (Rc<str>, bool, u64, u64) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn kit_do_git_days(v0: &str, v1: &str, v2: &str) -> (Rc<str>, bool, u64, u64) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
pub fn kit_do_git_status(v0: &str, v1: &str) -> (Rc<str>, bool, u64, u64, u64) {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
