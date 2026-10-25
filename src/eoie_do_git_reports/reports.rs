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
fn git_run_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = { let mut command = std::process::Command::new("git"); command.current_dir(std::path::Path::new(&*v0).join(&*v1)).args(["-c", "user.name=eoie", "-c"]).arg(format!("user.email=eoie{}localhost", char::from(64u8))).args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args((v2).split(char::from(10u8))).stdin(std::process::Stdio::null()); if !(v3).is_empty() { command.env("GIT_AUTHOR_DATE", &*v3).env("GIT_COMMITTER_DATE", &*v3); } let mark = char::from(30u8); std::rc::Rc::<str>::from(match command.output() { Ok(output) => format!("{}{mark}{}{mark}{}", output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr).trim()), Err(error) => format!("1{mark}{mark}git: {error}") }) };
    v4.clone()
}
fn part_at_3(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn succeeded_2(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<str> = part_at_3(v0.clone(), v1);
    let mut v3: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
    v3
}
fn row_4(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn go_7(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let mut v5: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
            let mut v6: i32 = (v5.clone().len() as i32);
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let mut v8: bool = v5.ends_with(&*v7);
            let mut v9: bool = v8 == false;
            let mut v15: Rc<str> = if v9 {
                v5.clone()
            } else {
                let mut v10: bool = v6 == 1i32;
                if v10 {
                    let mut v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11.clone()
                } else {
                    let mut v12: i32 = v6.wrapping_sub(2i32);
                    let mut v13: Rc<str> = string_slice(&v5.clone(), 0i32 as i64, v12 as i64);
                    v13.clone()
                }
            };
            let mut v16: i32 = v2.wrapping_add(1i32);
            let mut v17: bool = v15.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v19: i32 = if v17 {
                v3
            } else {
                let mut v18: i32 = v3.wrapping_add(1i32);
                v18
            };
            (v0, v1, v2, v3) = (v0.clone(), v1, v16, v19);
            continue;
        }
    }
}
fn line_count_6(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 > 0i32;
    let mut v8: bool = if v4 {
        let mut v5: i32 = v3.wrapping_sub(1i32);
        let mut v6: Rc<str> = v2.clone().borrow()[v5 as usize].clone();
        let mut v7: bool = v6.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v7
    } else {
        false
    };
    let mut v10: i32 = if v8 {
        let mut v9: i32 = v3.wrapping_sub(1i32);
        v9
    } else {
        v3
    };
    let mut v11: i32 = 0i32;
    let mut v12: i32 = 0i32;
    let mut v13: i32 = go_7(v2.clone(), v10, v11, v12);
    let mut v14: u64 = (v13 as u64);
    v14
}
fn go_9(mut v0: i32, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> (i32, Rc<str>) {
    loop {
        let mut v6: bool = v3 >= v2;
        if v6 {
            return (v4, v5.clone());
        } else {
            let mut v7: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
            let mut v8: i32 = (v7.clone().len() as i32);
            let mut v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let mut v10: bool = v7.ends_with(&*v9);
            let mut v11: bool = v10 == false;
            let mut v17: Rc<str> = if v11 {
                v7.clone()
            } else {
                let mut v12: bool = v8 == 1i32;
                if v12 {
                    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v13.clone()
                } else {
                    let mut v14: i32 = v8.wrapping_sub(2i32);
                    let mut v15: Rc<str> = string_slice(&v7.clone(), 0i32 as i64, v14 as i64);
                    v15.clone()
                }
            };
            let mut v18: i32 = v3.wrapping_add(1i32);
            let mut v19: bool = v17.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let (mut v23, mut v24): (i32, Rc<str>) = if v19 {
                (v4, v5.clone())
            } else {
                let mut v20: i32 = v4.wrapping_add(1i32);
                let mut v21: bool = v4 == v0;
                let mut v22: Rc<str> = if v21 {
                    v17.clone()
                } else {
                    v5.clone()
                };
                (v20, v22.clone())
            };
            (v0, v1, v2, v3, v4, v5) = (v0, v1.clone(), v2, v18, v23, v24.clone());
            continue;
        }
    }
}
fn line_at_8(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let mut v2: i32 = (v1 as i32);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v5: i32 = (v4.clone().borrow().len() as i32);
    let mut v6: bool = v5 > 0i32;
    let mut v10: bool = if v6 {
        let mut v7: i32 = v5.wrapping_sub(1i32);
        let mut v8: Rc<str> = v4.clone().borrow()[v7 as usize].clone();
        let mut v9: bool = v8.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v9
    } else {
        false
    };
    let mut v12: i32 = if v10 {
        let mut v11: i32 = v5.wrapping_sub(1i32);
        v11
    } else {
        v5
    };
    let mut v13: i32 = 0i32;
    let mut v14: i32 = 0i32;
    let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let (mut v16, mut v17): (i32, Rc<str>) = go_9(v2, v4.clone(), v12, v13, v14, v15.clone());
    v17.clone()
}
fn tab_part_10(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from((v0).split(char::from(9u8)).nth(v1 as usize).unwrap_or(""));
    v2.clone()
}
fn occurrences_11(mut v0: Rc<str>, mut v1: Rc<str>) -> u64 {
    let mut v2: u64 = (v0).matches(&*v1).count() as u64;
    v2
}
fn joined_12(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = v1.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = v0.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn history_rows_5(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: u64, mut v6: Rc<str>) -> Rc<str> {
    loop {
        let mut v7: u64 = line_count_6(v4.clone());
        let mut v8: bool = v5 < v7;
        if v8 {
            let mut v9: Rc<str> = line_at_8(v4.clone(), v5);
            let mut v10: i32 = 0i32;
            let mut v11: Rc<str> = tab_part_10(v9.clone(), v10);
            let mut v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show\n"); } LIT.with(|lit| lit.clone()) };
            let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(":"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v13, &*v3, &*v14].concat());
            let mut v16: Rc<str> = git_run_1(v0.clone(), v1.clone(), v15.clone(), v14.clone());
            let mut v17: i32 = 1i32;
            let mut v18: Rc<str> = part_at_3(v16.clone(), v17);
            let mut v19: u64 = occurrences_11(v18.clone(), v2.clone());
            let mut v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^:"); } LIT.with(|lit| lit.clone()) };
            let mut v21: Rc<str> = std::rc::Rc::<str>::from([&*v12, &*v11, &*v20, &*v3, &*v14].concat());
            let mut v22: Rc<str> = git_run_1(v0.clone(), v1.clone(), v21.clone(), v14.clone());
            let mut v23: i32 = 1i32;
            let mut v24: Rc<str> = part_at_3(v22.clone(), v23);
            let mut v25: u64 = occurrences_11(v24.clone(), v2.clone());
            let mut v26: bool = v19 < v25;
            let mut v29: Rc<str> = if v26 {
                let mut v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("removed"); } LIT.with(|lit| lit.clone()) };
                v27.clone()
            } else {
                let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("added"); } LIT.with(|lit| lit.clone()) };
                v28.clone()
            };
            let mut v30: i32 = 1i32;
            let mut v31: Rc<str> = tab_part_10(v9.clone(), v30);
            let mut v32: i32 = 2i32;
            let mut v33: Rc<str> = tab_part_10(v9.clone(), v32);
            let mut v34: Rc<str> = row_4(v29.clone(), v33.clone());
            let mut v35: Rc<str> = row_4(v31.clone(), v34.clone());
            let mut v36: Rc<str> = row_4(v11.clone(), v35.clone());
            let mut v37: u64 = v5.wrapping_add(1u64);
            let mut v38: Rc<str> = joined_12(v6.clone(), v36.clone());
            (v0, v1, v2, v3, v4, v5, v6) = (v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v37, v38.clone());
            continue;
        } else {
            return v6.clone();
        }
    }
}
fn matching_lines_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: Rc<str> = std::rc::Rc::<str>::from(std::fs::read_to_string(std::path::Path::new(&*v0).join(&*v1)).unwrap_or_default().lines().enumerate().filter(|(_, line)| line.contains(&*v2)).map(|(index, line)| format!("{}:{}  {}", &*v1, index + 1, line)).collect::<Vec<_>>().join("
"));
    v3.clone()
}
fn aligned_14(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn failure_row_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed: "); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = 2i32;
    let mut v4: Rc<str> = part_at_3(v1.clone(), v3);
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v4, &*v5, &*v5].concat());
    v6.clone()
}
fn history_report_0(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--abbrev=8\n--format=%h%x09%ad%x09%s\n--date=short\n-S"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--\n"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v7: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v2, &*v5, &*v3, &*v6].concat());
    let mut v8: Rc<str> = git_run_1(v0.clone(), v1.clone(), v7.clone(), v6.clone());
    let mut v9: bool = succeeded_2(v8.clone());
    if v9 {
        let mut v10: i32 = 1i32;
        let mut v11: Rc<str> = part_at_3(v8.clone(), v10);
        let mut v12: u64 = 0u64;
        let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("date"); } LIT.with(|lit| lit.clone()) };
        let mut v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("change"); } LIT.with(|lit| lit.clone()) };
        let mut v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("subject"); } LIT.with(|lit| lit.clone()) };
        let mut v17: Rc<str> = row_4(v15.clone(), v16.clone());
        let mut v18: Rc<str> = row_4(v14.clone(), v17.clone());
        let mut v19: Rc<str> = row_4(v13.clone(), v18.clone());
        let mut v20: Rc<str> = history_rows_5(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v11.clone(), v12, v19.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v21, &*v3, &*v6, &*v6].concat());
        let mut v23: Rc<str> = matching_lines_13(v0.clone(), v22.clone(), v2.clone());
        let mut v24: Rc<str> = aligned_14(v20.clone());
        let mut v25: Rc<str> = joined_12(v24.clone(), v23.clone());
        let mut v26: u64 = line_count_6(v11.clone());
        let mut v27: u64 = line_count_6(v23.clone());
        (v25.clone(), false, v26, v27)
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v1, &*v6, &*v6, &*v6].concat());
        let mut v30: Rc<str> = failure_row_15(v29.clone(), v8.clone());
        (v30.clone(), true, 0u64, 0u64)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        history_report_0(v0.clone(), v1.clone(), v2.clone(), v3.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn days_counted_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut days = std::collections::BTreeMap::<&str, u64>::new(); for day in (v0).lines().map(str::trim).filter(|day| !day.is_empty()) { *days.entry(day).or_default() += 1; } std::rc::Rc::<str>::from(days.iter().rev().map(|(day, count)| format!("{day}{}{count}", char::from(31u8))).collect::<Vec<_>>().join("
")) };
    v1.clone()
}
fn days_report_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> (Rc<str>, bool, u64, u64) {
    let mut v3: bool = (v2).len() == 10;
    let mut v4: bool = v2.clone() == { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v12: Rc<str> = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5.clone()
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n--since="); } LIT.with(|lit| lit.clone()) };
        let mut v9: Rc<str> = if v3 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" 00:00:00"); } LIT.with(|lit| lit.clone()) };
            v7.clone()
        } else {
            let mut v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v8.clone()
        };
        let mut v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let mut v11: Rc<str> = std::rc::Rc::<str>::from([&*v6, &*v2, &*v9, &*v10, &*v10].concat());
        v11.clone()
    };
    let mut v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("log\n--format=%ad\n--date=short"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v15: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v12, &*v14, &*v14, &*v14].concat());
    let mut v16: Rc<str> = git_run_1(v0.clone(), v1.clone(), v15.clone(), v14.clone());
    let mut v17: bool = succeeded_2(v16.clone());
    if v17 {
        let mut v18: i32 = 1i32;
        let mut v19: Rc<str> = part_at_3(v16.clone(), v18);
        let mut v20: Rc<str> = days_counted_17(v19.clone());
        let mut v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("day"); } LIT.with(|lit| lit.clone()) };
        let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits"); } LIT.with(|lit| lit.clone()) };
        let mut v23: Rc<str> = row_4(v21.clone(), v22.clone());
        let mut v24: Rc<str> = joined_12(v23.clone(), v20.clone());
        let mut v25: Rc<str> = aligned_14(v24.clone());
        let mut v26: u64 = line_count_6(v19.clone());
        let mut v27: u64 = line_count_6(v20.clone());
        (v25.clone(), false, v26, v27)
    } else {
        let mut v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git log "); } LIT.with(|lit| lit.clone()) };
        let mut v29: Rc<str> = std::rc::Rc::<str>::from([&*v28, &*v1, &*v14, &*v14, &*v14].concat());
        let mut v30: Rc<str> = failure_row_15(v29.clone(), v16.clone());
        (v30.clone(), true, 0u64, 0u64)
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>, Rc<str>) -> (Rc<str>, bool, u64, u64)> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>| -> (Rc<str>, bool, u64, u64) {
        days_report_16(v0.clone(), v1.clone(), v2.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn eoie_do_git_history(v0: &str, v1: &str, v2: &str, v3: &str) -> (Rc<str>, bool, u64, u64) {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2), Rc::<str>::from(v3))
}
pub fn eoie_do_git_days(v0: &str, v1: &str, v2: &str) -> (Rc<str>, bool, u64, u64) {
    closure1()(Rc::<str>::from(v0), Rc::<str>::from(v1), Rc::<str>::from(v2))
}
