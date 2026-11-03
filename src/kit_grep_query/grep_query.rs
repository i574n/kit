#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;

pub type KitGrepHits = std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>;
pub type KitGrepMatcher = std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync>;
pub fn kit_grep_files_under(root: &str) -> std::rc::Rc<Vec<std::path::PathBuf>> {
    let skip = ["target", "node_modules", ".git", "_build", "deps", "dist", ".cache", "obj"];
    let root = std::path::PathBuf::from(root);
    let mut stack = vec![root.clone()];
    let mut files = Vec::new();
    while let Some(folder) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&folder) else { continue };
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_symlink() { continue; }
            if kind.is_dir() {
                if !skip.iter().any(|name| path.file_name().map(|value| value == *name).unwrap_or(false)) { stack.push(path); }
            } else if kind.is_file() { files.push(path); }
        }
    }
    std::rc::Rc::new(files)
}
pub fn kit_grep_literal(needle: &str) -> KitGrepMatcher {
    let needle = needle.to_owned();
    std::sync::Arc::new(move |line: &str| line.contains(needle.as_str()))
}
pub fn kit_grep_caseless(needle: &str) -> KitGrepMatcher {
    let needle = needle.to_lowercase();
    std::sync::Arc::new(move |line: &str| line.to_lowercase().contains(needle.as_str()))
}
pub fn kit_grep_pattern(pattern: &str, caseless: bool) -> KitGrepMatcher {
    let found = regex::RegexBuilder::new(pattern).case_insensitive(caseless).build().unwrap();
    std::sync::Arc::new(move |line: &str| found.is_match(line))
}
pub fn kit_grep_hits(root: &str, files: &std::rc::Rc<Vec<std::path::PathBuf>>, matcher: &KitGrepMatcher, glob: &str) -> KitGrepHits {
    use rayon::prelude::*;
    let root = std::path::PathBuf::from(root);
    let mut files = (**files).clone();
    let (head, tail) = glob.split_once(char::from(42u8)).unwrap_or((glob, ""));
    let wanted = |path: &std::path::Path| glob.is_empty() || path.file_name().map(|name| {
        let name = name.to_string_lossy();
        if glob.contains(char::from(42u8)) { name.starts_with(head) && name.ends_with(tail) } else { name == glob }
    }).unwrap_or(false);
    files.retain(|path| wanted(path));
    let matcher = matcher.clone();
    let mut hits = files.par_iter().filter_map(|path| {
        let bytes = std::fs::read(path).ok()?;
        let text = String::from_utf8_lossy(&bytes);
        let lines = text.lines().map(str::to_owned).collect::<Vec<_>>();
        let found = lines.iter().enumerate().filter(|(_, line)| matcher(line.as_str())).map(|(index, _)| index).collect::<Vec<_>>();
        if found.is_empty() { return None; }
        Some((path.clone(), found, lines))
    }).collect::<Vec<_>>();
    hits.sort_by(|left, right| left.0.cmp(&right.0));
    let shown = |path: &std::path::Path| path.strip_prefix(&root).ok().filter(|relative| !relative.as_os_str().is_empty()).unwrap_or(path).to_string_lossy().replace(char::from(92u8), "/");
    std::rc::Rc::new((files.len() as u64, hits.into_iter().map(|(path, found, lines)| (shown(&path), found, lines)).collect::<Vec<_>>()))
}
pub fn kit_grep_hit_path(hits: &KitGrepHits, file: i32) -> std::rc::Rc<str> { std::rc::Rc::<str>::from(hits.1[file as usize].0.as_str()) }
pub fn kit_grep_taken_line(hits: &KitGrepHits, file: i32, taken: i32, line: i32) -> bool {
    hits.1[file as usize].1.iter().take(taken as usize).any(|index| *index == line as usize)
}
pub fn kit_grep_clipped_line(hits: &KitGrepHits, file: i32, line: i32) -> std::rc::Rc<str> {
    let line = &hits.1[file as usize].2[line as usize];
    std::rc::Rc::<str>::from(if line.chars().count() > 200 { format!("{}...", line.chars().take(200).collect::<String>()) } else { line.to_owned() }.as_str())
}
pub fn kit_grep_numbered(number: i32, width: i32, taken: bool, text: &str) -> std::rc::Rc<str> {
    std::rc::Rc::<str>::from(format!("{:>width$}{} {}", number, if taken { char::from(62u8) } else { char::from(32u8) }, text, width = width as usize).as_str())
}
pub fn kit_grep_pattern_problem(pattern: &str) -> std::rc::Rc<str> {
    std::rc::Rc::<str>::from(regex::Regex::new(pattern).err().map(|error| error.to_string()).unwrap_or_default().as_str())
}


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
enum US2 {
    US2_Some(Rc<str>),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Some(u64),
    US3_None,
}
#[derive(Clone)]
enum US4 {
    US4_LiteralNeedle,
    US4_CaselessNeedle,
    US4_PatternNeedle(bool),
}
#[derive(Clone)]
enum US5 {
    US5_FolderRoot,
    US5_FileRoot,
    US5_MissingRoot,
}
fn counts_start_2() -> (i32, i32, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0i32, 0i32, v0)
}
fn json_text_5(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = spiral_json_quote(&*v0);
    v1
}
fn method7(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_6(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method7(v2.clone(), v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn json_matches_4(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: Rc<str>) -> Rc<str> {
    loop {
        let v5: bool = v3 < v2;
        if v5 {
            let mut v6: usize = ((v1) as usize);
            let v7: usize = ((v3) as usize);
            let v8: usize = (v0).1[v6].1[v7];
            let v9: i32 = ((v8) as i32);
            let v10: Rc<str> = kit_grep_hit_path(&v0, v1);
            let v11: Rc<str> = json_text_5(v10.clone());
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"path\":"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v11));
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"line\":"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
            let v16: i32 = v9.wrapping_add(1i32);
            let v17: Rc<str> = format_real_6(v16);
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v17));
            let v19: Rc<str> = kit_grep_clipped_line(&v0, v1, v9);
            let v20: Rc<str> = json_text_5(v19.clone());
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"text\":"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v20));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("}"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v18, Rc::<str>::from(format!("{}{}", v22, v23))));
            let v26: i32 = v3.wrapping_add(1i32);
            let v27: bool = &*v4 == "";
            let v31: Rc<str> = if v27 {
                v25.clone()
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                let v30: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v4, v28)), v25));
                v30.clone()
            };
            (v3, v4) = (v26, v31.clone());
            continue;
        } else {
            return v4.clone();
        }
    }
}
fn last_window_line_8(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> i32 {
    loop {
        let mut v6: bool = v4 < v2;
        if v6 {
            let v7: i32 = v4.wrapping_add(1i32);
            let v8: usize = ((v1) as usize);
            let v9: usize = ((v4) as usize);
            let v10: usize = (v0).1[v8].1[v9];
            let v11: i32 = ((v10) as i32);
            let v12: i32 = v11.wrapping_add(v3);
            let v13: usize = ((v1) as usize);
            let v14: usize = (v0).1[v13].2.len();
            let v15: i32 = ((v14) as i32);
            let v16: i32 = v15.wrapping_sub(1i32);
            let v17: bool = v12 < v16;
            let v18: i32 = if v17 {
                v12
            } else {
                v16
            };
            let v19: bool = v5 < v18;
            let v20: i32 = if v19 {
                v18
            } else {
                v5
            };
            (v4, v5) = (v7, v20);
            continue;
        } else {
            return v5;
        }
    }
}
fn window_lines_10(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32) -> () {
    loop {
        let mut v6: bool = v4 <= v5;
        if v6 {
            let v7: i32 = v4.wrapping_add(1i32);
            let v8: bool = kit_grep_taken_line(&v0, v1, v2, v4);
            let v9: Rc<str> = kit_grep_clipped_line(&v0, v1, v4);
            let v10: Rc<str> = kit_grep_numbered(v7, v3, v8, &*v9);
            println!("{}", v10);
            v4 = v7;
            continue;
        }
        return ();
    }
}
fn windows_printed_9(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: i32, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32) -> () {
    loop {
        let v7: bool = v5 < v2;
        if v7 {
            let v8: usize = ((v1) as usize);
            let v9: usize = ((v5) as usize);
            let v10: usize = (v0).1[v8].1[v9];
            let v11: i32 = ((v10) as i32);
            let v12: i32 = v11.wrapping_sub(v3);
            let v13: bool = v12 < 0i32;
            let v14: i32 = if v13 {
                0i32
            } else {
                v12
            };
            let v15: bool = 0i32 <= v6;
            let v18: bool = if v15 {
                let v16: i32 = v6.wrapping_add(1i32);
                let v17: bool = v14 < v16;
                v17
            } else {
                false
            };
            let v20: i32 = if v18 {
                let v19: i32 = v6.wrapping_add(1i32);
                v19
            } else {
                v14
            };
            let v21: i32 = v11.wrapping_add(v3);
            let v22: usize = ((v1) as usize);
            let v23: usize = (v0).1[v22].2.len();
            let v24: i32 = ((v23) as i32);
            let v25: i32 = v24.wrapping_sub(1i32);
            let v26: bool = v21 < v25;
            let v27: i32 = if v26 {
                v21
            } else {
                v25
            };
            window_lines_10(v0.clone(), v1, v2, v4, v20, v27);
            let v28: i32 = v5.wrapping_add(1i32);
            let v29: bool = v6 < v27;
            let v30: i32 = if v29 {
                v27
            } else {
                v6
            };
            (v5, v6) = (v28, v30);
            continue;
        }
        return ();
    }
}
fn files_rendered_3(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: bool, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: i32, mut v6: i32, mut v7: Rc<str>) -> (i32, i32, Rc<str>) {
    let v8: usize = (v0).1.len();
    let v9: i32 = ((v8) as i32);
    let v10: bool = v9 <= v4;
    let (mut v46, mut v47, mut v48): (i32, i32, Rc<str>) = if v10 {
        (v5, v6, v7.clone())
    } else {
        let v11: usize = ((v4) as usize);
        let v12: usize = (v0).1[v11].1.len();
        let v13: i32 = ((v12) as i32);
        let v14: i32 = v6.wrapping_add(v13);
        let v15: bool = v3 <= v5;
        let (mut v43, mut v44, mut v45): (i32, i32, Rc<str>) = if v15 {
            let v16: i32 = v4.wrapping_add(1i32);
            let (mut v17, mut v18, mut v19): (i32, i32, Rc<str>) = files_rendered_3(v0.clone(), v1, v2, v3, v16, v5, v14, v7.clone());
            (v17, v18, v19)
        } else {
            let v20: i32 = v3.wrapping_sub(v5);
            let v21: bool = v13 < v20;
            let v22: i32 = if v21 {
                v13
            } else {
                v20
            };
            let (mut v36, mut v37, mut v38): (i32, i32, Rc<str>) = if v1 {
                let v23: i32 = v5.wrapping_add(v22);
                let v24: i32 = 0i32;
                let v25: Rc<str> = json_matches_4(v0.clone(), v4, v22, v24, v7.clone());
                (v23, v14, v25)
            } else {
                let v26: Rc<str> = kit_grep_hit_path(&v0, v4);
                println!("{}", v26);
                let v27: i32 = 0i32;
                let v28: i32 = 0i32;
                let v29: i32 = last_window_line_8(v0.clone(), v4, v22, v2, v27, v28);
                let v30: i32 = v29.wrapping_add(1i32);
                let v31: Rc<str> = format_real_6(v30);
                let v32: i32 = (v31.len() as i32);
                let v33: i32 = 0i32;
                let v34: i32 = -1i32;
                windows_printed_9(v0.clone(), v4, v22, v2, v32, v33, v34);
                let v35: i32 = v5.wrapping_add(v22);
                (v35, v14, v7)
            };
            let v39: i32 = v4.wrapping_add(1i32);
            let (mut v40, mut v41, mut v42): (i32, i32, Rc<str>) = files_rendered_3(v0, v1, v2, v3, v39, v36, v37, v38);
            (v40, v41, v42)
        };
        (v43, v44, v45)
    };
    (v46, v47, v48)
}
fn decimal_text_11(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn elapsed_text_13(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_15() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_14() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_15();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_12(mut v0: US1, mut v1: Rc<str>, mut v2: u64) -> () {
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
    let v11: Rc<str> = elapsed_text_13(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_14();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn search_receipt_1(mut v0: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)>, mut v1: bool, mut v2: i32, mut v3: i32, mut v4: US0, mut v5: u64) -> i32 {
    let mut v6: i32 = 0i32;
    let (mut v7, mut v8, mut v9): (i32, i32, Rc<str>) = counts_start_2();
    let (mut v10, mut v11, mut v12): (i32, i32, Rc<str>) = files_rendered_3(v0.clone(), v1, v2, v3, v6, v7, v8, v9);
    let v13: u64 = (v0.clone()).0;
    let v14: Rc<str> = decimal_text_11(v13);
    let v15: usize = (v0).1.len();
    let v16: i32 = ((v15) as i32);
    let v17: Rc<str> = format_real_6(v16);
    if v1 {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("{\"files\":"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v14));
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matched\":"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v17));
        let v23: Rc<str> = format_real_6(v11);
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"total\":"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v23));
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"shown\":"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
        let v28: Rc<str> = format_real_6(v10);
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v22, Rc::<str>::from(format!("{}{}", v27, v28))));
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(",\"matches\":["); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v30, v31)), v12));
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("]}"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
        println!("{}", v35);
        0i32
    } else {
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v14));
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v37, v38)), v17));
        let v41: Rc<str> = format_real_6(v11);
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v40, Rc::<str>::from(format!("{}{}", v42, v41))));
        let v45: US1 = US1::US1_Succeeded;
        let v46: bool = v10 < v11;
        let v51: Rc<str> = if v46 {
            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  shown "); } LIT.with(|lit| lit.clone()) };
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v47));
            let v49: Rc<str> = format_real_6(v10);
            let v50: Rc<str> = Rc::<str>::from(format!("{}{}", v48, v49));
            v50
        } else {
            v44
        };
        let v52: bool = v11 != 0i32;
        let v58: Rc<str> = if v52 {
            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v53
        } else {
            match &v4 {
                US0::US0_LiteralLooksLikePattern => {
                    let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  hint: the needle is literal; pass --regex to use it as a pattern"); } LIT.with(|lit| lit.clone()) };
                    v55
                }
                US0::US0_NoNeedleHint => {
                    let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v54
                }
            }
        };
        let v59: Rc<str> = Rc::<str>::from(format!("{}{}", v51, v58));
        receipt_line_12(v45, v59, v5);
        0i32
    }
}
fn now_ms_16() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn command_line_argument_count_18() -> i32 {
    let mut v0: usize = std::env::args().count();
    let v1: i32 = ((v0) as i32);
    let v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn argument_count_17() -> i32 {
    let mut v0: i32 = command_line_argument_count_18();
    let v1: i32 = v0.wrapping_add(1i32);
    v1
}
fn search_start_19() -> (Rc<str>, u64, u64, bool, bool, bool) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, 0u64, 200u64, false, false, false)
}
fn command_line_argument_22(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_add(1i32);
    let v2: usize = ((v1) as usize);
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn argument_21(mut v0: i32) -> Rc<str> {
    let v1: i32 = v0.wrapping_sub(1i32);
    let v2: bool = v1 >= 0i32;
    let v5: bool = if v2 {
        let mut v3: i32 = command_line_argument_count_18();
        let mut v4: bool = v1 < v3;
        v4
    } else {
        false
    };
    let v9: US2 = if v5 {
        let mut v6: Rc<str> = command_line_argument_22(v1);
        US2::US2_Some(v6)
    } else {
        US2::US2_None
    };
    match &v9 {
        US2::US2_None => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v11
        }
        US2::US2_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            v10
        }
    }
}
fn method24(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US3> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US3> = Rc::new(move |mut v0: (u64)| -> US3 {
        let v1: u64 = (v0);
        US3::US3_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_u64_23(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method24(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US3> = closure1();
    let mut v4: Option<US3> = v2.map(|x| v3(x));
    let mut v6: US3 = US3::US3_None;
    let v7: US3 = v6.clone();
    let v8: US3 = v4.unwrap_or(v7);
    match &v8 {
        US3::US3_None => {
            0u64
        }
        US3::US3_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn search_options_from_20(mut v0: i32, mut v1: i32, mut v2: Rc<str>, mut v3: u64, mut v4: u64, mut v5: bool, mut v6: bool, mut v7: bool) -> (Rc<str>, u64, u64, bool, bool, bool) {
    let v8: bool = v0 < v1;
    let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (Rc<str>, u64, u64, bool, bool, bool) = if v8 {
        let v9: Rc<str> = argument_21(v0);
        let v10: bool = &*v9 == "--ignore-case";
        let (mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (Rc<str>, u64, u64, bool, bool, bool) = if v10 {
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: bool = true;
            let (mut v13, mut v14, mut v15, mut v16, mut v17, mut v18): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v11, v1, v2.clone(), v3, v4, v12, v6, v7);
            (v13, v14, v15, v16, v17, v18)
        } else {
            let v19: bool = &*v9 == "--json";
            let (mut v102, mut v103, mut v104, mut v105, mut v106, mut v107): (Rc<str>, u64, u64, bool, bool, bool) = if v19 {
                let v20: i32 = v0.wrapping_add(1i32);
                let v21: bool = true;
                let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v20, v1, v2.clone(), v3, v4, v5, v6, v21);
                (v22, v23, v24, v25, v26, v27)
            } else {
                let v28: bool = &*v9 == "--regex";
                let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (Rc<str>, u64, u64, bool, bool, bool) = if v28 {
                    let v29: i32 = v0.wrapping_add(1i32);
                    let v30: bool = true;
                    let (mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v29, v1, v2.clone(), v3, v4, v5, v30, v7);
                    (v31, v32, v33, v34, v35, v36)
                } else {
                    let v37: i32 = v0.wrapping_add(1i32);
                    let v38: bool = v37 < v1;
                    let (mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (Rc<str>, u64, u64, bool, bool, bool) = if v38 {
                        let v39: Rc<str> = argument_21(v37);
                        let v40: bool = &*v9 == "--glob";
                        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89): (Rc<str>, u64, u64, bool, bool, bool) = if v40 {
                            let v41: i32 = v0.wrapping_add(2i32);
                            let (mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v41, v1, v39.clone(), v3, v4, v5, v6, v7);
                            (v42, v43, v44, v45, v46, v47)
                        } else {
                            let v48: bool = &*v9 == "--context";
                            let (mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, u64, u64, bool, bool, bool) = if v48 {
                                let v49: i32 = v0.wrapping_add(2i32);
                                let v50: u64 = decimal_u64_23(v39.clone());
                                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v49, v1, v2.clone(), v50, v4, v5, v6, v7);
                                (v51, v52, v53, v54, v55, v56)
                            } else {
                                let v57: bool = &*v9 == "--max";
                                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77): (Rc<str>, u64, u64, bool, bool, bool) = if v57 {
                                    let v58: i32 = v0.wrapping_add(2i32);
                                    let v59: u64 = decimal_u64_23(v39);
                                    let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v58, v1, v2.clone(), v3, v59, v5, v6, v7);
                                    (v60, v61, v62, v63, v64, v65)
                                } else {
                                    let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v37, v1, v2.clone(), v3, v4, v5, v6, v7);
                                    (v66, v67, v68, v69, v70, v71)
                                };
                                (v72, v73, v74, v75, v76, v77)
                            };
                            (v78, v79, v80, v81, v82, v83)
                        };
                        (v84, v85, v86, v87, v88, v89)
                    } else {
                        (v2.clone(), v3, v4, v5, v6, v7)
                    };
                    (v90, v91, v92, v93, v94, v95)
                };
                (v96, v97, v98, v99, v100, v101)
            };
            (v102, v103, v104, v105, v106, v107)
        };
        (v108, v109, v110, v111, v112, v113)
    } else {
        (v2, v3, v4, v5, v6, v7)
    };
    (v114, v115, v116, v117, v118, v119)
}
fn directory_exists_25(mut v0: Rc<str>) -> bool {
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
fn file_exists_26(mut v0: Rc<str>) -> bool {
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
fn grep_run_0() -> i32 {
    let mut v0: u64 = now_ms_16();
    let v1: i32 = argument_count_17();
    let v2: bool = v1 < 4i32;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
        eprintln!("{}", v5);
        2i32
    } else {
        let mut v6: i32 = 4i32;
        let v7: i32 = argument_count_17();
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13): (Rc<str>, u64, u64, bool, bool, bool) = search_start_19();
        let (mut v14, mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, u64, u64, bool, bool, bool) = search_options_from_20(v6, v7, v8.clone(), v9, v10, v11, v12, v13);
        let v20: i32 = 2i32;
        let v21: Rc<str> = argument_21(v20);
        let v22: i32 = 3i32;
        let v23: Rc<str> = argument_21(v22);
        let v28: US4 = if v18 {
            US4::US4_PatternNeedle(v17)
        } else {
            if v17 {
                US4::US4_CaselessNeedle
            } else {
                US4::US4_LiteralNeedle
            }
        };
        let v31: Rc<str> = if v18 {
            let v29: Rc<str> = kit_grep_pattern_problem(&*v23);
            v29.clone()
        } else {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v30.clone()
        };
        let v32: bool = &*v31 != "";
        if v32 {
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep: invalid --regex pattern: "); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v31));
            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v34));
            eprintln!("{}", v36);
            2i32
        } else {
            let v37: bool = directory_exists_25(v21.clone());
            let v43: US5 = if v37 {
                US5::US5_FolderRoot
            } else {
                let v39: bool = file_exists_26(v21.clone());
                if v39 {
                    US5::US5_FileRoot
                } else {
                    US5::US5_MissingRoot
                }
            };
            match &v43 {
                US5::US5_MissingRoot => {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("grep root not found: "); } LIT.with(|lit| lit.clone()) };
                    let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v21));
                    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kit error: "); } LIT.with(|lit| lit.clone()) };
                    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v45));
                    eprintln!("{}", v47);
                    2i32
                }
                _ => {
                    let v50: std::rc::Rc<Vec<std::path::PathBuf>> = match &v43 {
                        US5::US5_FileRoot => {
                            let v48: std::rc::Rc<Vec<std::path::PathBuf>> = std::rc::Rc::new(vec![std::path::PathBuf::from(&*v21)]);
                            v48.clone()
                        }
                        _ => {
                            let v49: std::rc::Rc<Vec<std::path::PathBuf>> = kit_grep_files_under(&*v21);
                            v49.clone()
                        }
                    };
                    let v57: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = match &v28 {
                        US4::US4_CaselessNeedle => {
                            let v52: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = kit_grep_caseless(&*v23);
                            v52.clone()
                        }
                        US4::US4_LiteralNeedle => {
                            let v51: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = kit_grep_literal(&*v23);
                            v51.clone()
                        }
                        US4::US4_PatternNeedle(v53) => {
                            let v53: bool = *v53;
                            let v54: std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync> = kit_grep_pattern(&*v23, v53);
                            v54.clone()
                        }
                    };
                    let v58: std::rc::Rc<(u64, Vec<(String, Vec<usize>, Vec<String>)>)> = kit_grep_hits(&*v21, &v50, &v57, &*v14);
                    let v59: i32 = (v15 as i32);
                    let v60: i32 = (v16 as i32);
                    let v61: bool = v18 == false;
                    let v76: bool = if v61 {
                        let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("|"); } LIT.with(|lit| lit.clone()) };
                        let v63: bool = v23.contains(&*v62);
                        let v66: bool = if v63 {
                            true
                        } else {
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".*"); } LIT.with(|lit| lit.clone()) };
                            let v65: bool = v23.contains(&*v64);
                            v65
                        };
                        let v69: bool = if v66 {
                            true
                        } else {
                            let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("["); } LIT.with(|lit| lit.clone()) };
                            let v68: bool = v23.contains(&*v67);
                            v68
                        };
                        let v72: bool = if v69 {
                            true
                        } else {
                            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("^"); } LIT.with(|lit| lit.clone()) };
                            let v71: bool = v23.starts_with(&*v70);
                            v71
                        };
                        if v72 {
                            true
                        } else {
                            let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("$"); } LIT.with(|lit| lit.clone()) };
                            let v74: bool = v23.ends_with(&*v73);
                            v74
                        }
                    } else {
                        false
                    };
                    let v79: US0 = if v76 {
                        US0::US0_LiteralLooksLikePattern
                    } else {
                        US0::US0_NoNeedleHint
                    };
                    search_receipt_1(v58.clone(), v19, v59, v60, v79.clone(), v0)
                }
            }
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
