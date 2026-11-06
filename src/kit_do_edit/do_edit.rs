#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_model::{expectation, write_mode, replace_count, rule_matcher, rule_list, step, steps};
#[derive(Clone)]
enum US0 {
    US0_Succeeded,
    US0_Failed,
}
#[derive(Clone)]
enum US6 {
    US6_EditsCommitted(Rc<str>),
    US6_EditsStopped(i32),
}
#[derive(Clone)]
enum US7 {
    US7_Untouched,
    US7_Refused(Rc<str>),
    US7_Changed(Rc<str>, Rc<str>, u64, Rc<str>),
}
fn elapsed_text_4(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_6() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_6();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_3(mut v0: US0, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US0::US0_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US0::US0_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_4(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_5();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn refused_2(mut v0: Rc<str>, mut v1: i32, mut v2: u64) -> i32 {
    println!("{}", &*v0);
    let v3: US0 = US0::US0_Failed;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
    receipt_line_3(v3, v4, v2);
    v1
}
fn finished_7(mut v0: bool, mut v1: Rc<str>, mut v2: u64) -> i32 {
    let mut v5: US0 = if v0 {
        US0::US0_Failed
    } else {
        US0::US0_Succeeded
    };
    receipt_line_3(v5, v1, v2);
    if v0 {
        1i32
    } else {
        0i32
    }
}
fn empty_tally_9() -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, false, false, v0.clone(), v0)
}
fn target_paths_12(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v2: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &str, name: &str) -> bool { let parts = mask.split(char::from(42u8)).collect::<Vec<_>>(); if parts.len() == 1 { return mask == name; } let mut rest = name; if !rest.starts_with(parts[0]) { return false; } rest = &rest[parts[0].len()..]; for middle in &parts[1..parts.len() - 1] { match rest.find(middle) { Some(at) => rest = &rest[at + middle.len()..], None => return false } } rest.len() >= parts[parts.len() - 1].len() && rest.ends_with(parts[parts.len() - 1]) } fn joined(shown: &str, name: &str) -> String { if shown.is_empty() { name.to_owned() } else { format!("{shown}/{name}") } } fn walk(base: &std::path::Path, shown: &str, segments: &[String], found: &mut Vec<String>) { let Some((head, rest)) = segments.split_first() else { if base.is_file() { found.push(shown.to_owned()); } return; }; let children = || std::fs::read_dir(base).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.file_type().map(|kind| !kind.is_symlink()).unwrap_or(false)).collect::<Vec<_>>()).unwrap_or_default(); if head == "**" { walk(base, shown, rest, found); for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if entry.path().is_dir() && ![".git", "target", "node_modules"].contains(&name.as_str()) { walk(&entry.path(), &joined(shown, &name), segments, found); } } } else if head.contains(char::from(42u8)) { for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if wildcard(head, &name) { walk(&entry.path(), &joined(shown, &name), rest, found); } } } else { walk(&base.join(head), &joined(shown, head), rest, found); } } let pattern = (&*v1).replace(char::from(92u8), "/"); let segments = pattern.split(char::from(47u8)).filter(|segment| !segment.is_empty() && *segment != ".").map(str::to_owned).collect::<Vec<_>>(); let lead = if pattern.starts_with(char::from(47u8)) { "/" } else { "" }; let mut found = Vec::new(); if pattern.contains(char::from(42u8)) { let fixed = segments.iter().take_while(|segment| !segment.contains(char::from(42u8))).count(); let prefix = format!("{lead}{}", segments[..fixed].join("/")); walk(&std::path::Path::new(&*v0).join(&prefix), &prefix, &segments[fixed..], &mut found); found.sort(); found.dedup(); } else { found.push(format!("{lead}{}", segments.join("/"))); } std::rc::Rc::new(found.into_iter().map(std::rc::Rc::<str>::from).collect::<Vec<_>>()) };
    v2
}
fn paths_kept_13(mut v0: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v1: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v2: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &[u8], name: &[u8]) -> bool { match (mask.first(), name.first()) { (None, None) => true, (Some(&42u8), _) => wildcard(&mask[1..], name) || (!name.is_empty() && wildcard(mask, &name[1..])), (Some(a), Some(b)) => a == b && wildcard(&mask[1..], &name[1..]), _ => false } } let masks = (&*v1).split(char::from(44u8)).map(|mask| mask.trim().replace(char::from(92u8), "/")).filter(|mask| !mask.is_empty()).collect::<Vec<_>>(); std::rc::Rc::new(v0.iter().filter(|path| { let shown = path.replace(char::from(92u8), "/"); let name = shown.rsplit(char::from(47u8)).next().unwrap_or("").to_owned(); !masks.iter().any(|mask| wildcard(mask.as_bytes(), shown.as_bytes()) || wildcard(mask.as_bytes(), name.as_bytes())) }).cloned().collect::<Vec<_>>()) };
    v2
}
fn opened_block_14(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, v10, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
fn text_now_17(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Result<std::rc::Rc<str>, std::rc::Rc<str>> {
    let v3: Result<std::rc::Rc<str>, std::rc::Rc<str>> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.clone()); match staged { Some(text) => Ok(text), None => std::fs::read(std::path::Path::new(&*v1).join(&*v2)).map_err(|error| std::rc::Rc::<str>::from(format!("unreadable: {error}"))).and_then(|bytes| String::from_utf8(bytes).map(std::rc::Rc::<str>::from).map_err(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from("not UTF-8 text"); } LIT.with(|lit| lit.clone()) })) } };
    v3
}
fn opened_file_18(mut v0: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, 0u64, v1.clone(), v1.clone(), v1)
}
fn replaced_in_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> {
    let v3: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = { let text = &*v0; let crlf = text.contains("
"); let adapt = |value: &str| if crlf && value.contains(char::from(10u8)) && !value.contains("
") { value.replace("
", "
") } else { value.to_owned() }; let anchor = adapt(&*v1); let replacement = adapt(&*v2); if anchor.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(text), 0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })); } let starts = text.match_indices(anchor.as_str()).map(|(at, _)| at).collect::<Vec<_>>(); let lines = starts.iter().map(|at| (text[..*at].matches(char::from(10u8)).count() + 1).to_string()).collect::<Vec<_>>().join(","); std::rc::Rc::new((std::rc::Rc::<str>::from(text.replace(anchor.as_str(), replacement.as_str())), starts.len() as u64, std::rc::Rc::<str>::from(lines))) };
    v3
}
fn replaced_by_pattern_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> {
    let v3: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = { let text = &*v0; match regex::Regex::new(&*v1) { Ok(matcher) => { let starts = matcher.find_iter(text).map(|found| found.start()).collect::<Vec<_>>(); let lines = starts.iter().map(|at| (text[..*at].matches(char::from(10u8)).count() + 1).to_string()).collect::<Vec<_>>().join(","); std::rc::Rc::new((std::rc::Rc::<str>::from(matcher.replace_all(text, &*v2).as_ref()), starts.len() as u64, std::rc::Rc::<str>::from(lines))) } Err(_) => std::rc::Rc::new((std::rc::Rc::<str>::from(text), 0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })) } };
    v3
}
fn pattern_problem_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v0).err().map(|error| error.to_string().lines().last().unwrap_or("").trim().to_owned()).unwrap_or_default());
    v1
}
fn concat5_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let mut v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn joined_with_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = &*v2 == "";
    if v3 {
        v1
    } else {
        let v4: bool = &*v1 == "";
        if v4 {
            v2
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_24(v1, v0, v2, v5.clone(), v5)
        }
    }
}
fn quoted_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let shown = format!("{:?}", &*v0); std::rc::Rc::<str>::from(if shown.chars().count() > 80 { format!("{}...{}", shown.chars().take(76).collect::<String>(), char::from(34u8)) } else { shown }) };
    v1
}
fn decimal_text_28(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn mismatch_26(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = v1 == 0u64;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor not found: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = quoted_27(v0);
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (0 matches, expected "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        concat5_24(v4, v5, v6, v2, v7)
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor matched "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = decimal_text_28(v1);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" times: "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = quoted_27(v0);
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = concat5_24(v13, v2, v14, v15.clone(), v15);
        concat5_24(v9, v10, v11, v12, v16)
    }
}
fn count_problem_25(mut v0: replace_count, mut v1: u64, mut v2: bool, mut v3: Rc<str>) -> Rc<str> {
    let v4: bool = v1 == 0u64;
    let mut v5: bool = v4 && v2;
    match v0 {
        replace_count::replace_count_Every => {
            let v20: bool = if v4 {
                let v19: bool = v2 == false;
                v19
            } else {
                false
            };
            if v20 {
                let v21: u64 = 0u64;
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at least 1"); } LIT.with(|lit| lit.clone()) };
                mismatch_26(v3, v21, v22)
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24
            }
        }
        replace_count::replace_count_Exactly(v12) => {
            let v12: u64 = v12;
            let v13: bool = v1 == v12;
            let v14: bool = v13 || v5;
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15
            } else {
                let v16: Rc<str> = decimal_text_28(v12);
                mismatch_26(v3, v1, v16)
            }
        }
        replace_count::replace_count_Once => {
            let v6: bool = v1 == 1u64;
            let v7: bool = v6 || v5;
            if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v8
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                mismatch_26(v3, v1, v9)
            }
        }
    }
}
fn sites_of_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let text = &*v0; let anchor = &*v1; let first = anchor.lines().next().unwrap_or(anchor); let sites = if anchor.is_empty() { Vec::new() } else { text.lines().enumerate().filter(|(_, line)| line.contains(first)).map(|(index, line)| format!("{}:{}  {}", &*v2, index + 1, line)).collect::<Vec<_>>() }; std::rc::Rc::<str>::from(sites.join("
")) };
    v3
}
fn pattern_sites_of_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let sites = match regex::Regex::new(&*v1) { Ok(matcher) => (&*v0).lines().enumerate().filter(|(_, line)| matcher.is_match(line)).map(|(index, line)| format!("{}:{}  {}", &*v2, index + 1, line)).collect::<Vec<_>>(), Err(_) => Vec::new() }; std::rc::Rc::<str>::from(sites.join("
")) };
    v3
}
fn lines_joined_31(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    joined_with_23(v2, v0, v1)
}
fn rules_over_19(mut v0: Rc<rule_list>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        rule_list::rule_list_NoRules => {
            (v3, v4, v5, v6, v7)
        }
        _ => {
            let (mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v0) {
                rule_list::rule_list_Rule(v8, v9, v10, v11, v12) => {
                    let v8: Rc<str> = v8;
                    let v9: Rc<str> = v9;
                    let v10: replace_count = v10;
                    let v11: rule_matcher = v11;
                    let v12: Rc<rule_list> = v12;
                    let v13: bool = &*v6 == "";
                    if v13 {
                        let v17: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = match &v11 {
                            rule_matcher::rule_matcher_Literal => {
                                replaced_in_20(v3.clone(), v8.clone(), v9)
                            }
                            rule_matcher::rule_matcher_Pattern => {
                                replaced_by_pattern_21(v3.clone(), v8.clone(), v9)
                            }
                        };
                        let v21: Rc<str> = match &v11 {
                            rule_matcher::rule_matcher_Literal => {
                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v18
                            }
                            rule_matcher::rule_matcher_Pattern => {
                                pattern_problem_22(v8.clone())
                            }
                        };
                        let v22: u64 = v17.clone().1;
                        let v23: Rc<str> = v17.0.clone();
                        let v24: u64 = v4.wrapping_add(v22);
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = v17.2.clone();
                        let v27: Rc<str> = joined_with_23(v25, v5, v26);
                        let v28: bool = &*v21 == "";
                        let v35: Rc<str> = if v28 {
                            count_problem_25(v10, v22, v1, v8.clone())
                        } else {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid pattern "); } LIT.with(|lit| lit.clone()) };
                            let v31: Rc<str> = quoted_27(v8.clone());
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            concat5_24(v30, v31, v32, v21, v33)
                        };
                        let v39: Rc<str> = match v11 {
                            rule_matcher::rule_matcher_Literal => {
                                sites_of_29(v3, v8, v2.clone())
                            }
                            rule_matcher::rule_matcher_Pattern => {
                                pattern_sites_of_30(v3, v8, v2.clone())
                            }
                        };
                        let v40: Rc<str> = lines_joined_31(v7, v39);
                        let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_19(v12, v1, v2, v23, v24, v27, v35, v40);
                        (v41, v42, v43, v44, v45)
                    } else {
                        (v3, v4, v5, v6, v7)
                    }
                }
                _ => unreachable!(),
            };
            (v51, v52, v53, v54, v55)
        }
    };
    (v56, v57, v58, v59, v60)
}
fn emptied_problem_32(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: u64 = v0.len() as u64;
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the edits would leave the file empty (was "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = decimal_text_28(v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes); empty it with kit proxy fs-write --expect-hash"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_24(v2, v3, v4, v5.clone(), v5)
}
fn file_result_of_16(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<rule_list>, mut v3: bool, mut v4: Rc<str>) -> (US7, Rc<str>) {
    let mut v5: Result<std::rc::Rc<str>, std::rc::Rc<str>> = text_now_17(v1, v0, v4.clone());
    let v6: bool = v5.is_ok();
    let (mut v34, mut v35): (US7, Rc<str>) = if v6 {
        let v7: Rc<str> = v5.clone().unwrap_or_default();
        let (mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = opened_file_18(v7.clone());
        let (mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_19(v2, v3, v4, v8, v9, v10, v11, v12);
        let v18: bool = &*v16 != "";
        let v30: US7 = if v18 {
            US7::US7_Refused(v16)
        } else {
            let v20: bool = v14 == 0u64;
            if v20 {
                US7::US7_Untouched
            } else {
                let v22: bool = &*v13 == "";
                let v24: bool = if v22 {
                    let v23: bool = &*v7 != "";
                    v23
                } else {
                    false
                };
                if v24 {
                    let v25: Rc<str> = emptied_problem_32(v7);
                    US7::US7_Refused(v25)
                } else {
                    US7::US7_Changed(v7, v13, v14, v15)
                }
            }
        };
        (v30, v17)
    } else {
        let v31: Rc<str> = v5.err().unwrap_or_default();
        let v32: US7 = US7::US7_Refused(v31);
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v32, v33)
    };
    (v34, v35)
}
fn row2_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_24(v0, v2, v1, v3.clone(), v3)
}
fn sha256_of_35(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { use sha2::Digest; std::rc::Rc::<str>::from(sha2::Sha256::digest(v0.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect::<String>()) };
    v1
}
fn counted_36(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_28(v0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == 1u64;
    let v6: Rc<str> = if v5 {
        v1
    } else {
        v2
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_24(v3, v4, v6, v7.clone(), v7)
}
fn lines_label_37(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut lines = (&*v0).split(char::from(44u8)).filter_map(|value| value.parse::<u64>().ok()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); let mut ranges: Vec<(u64, u64)> = Vec::new(); for line in lines.iter().copied() { match ranges.last_mut() { Some(range) if range.1 + 1 == line => range.1 = line, _ => ranges.push((line, line)) } } let text = ranges.iter().map(|(first, last)| if first == last { first.to_string() } else { format!("{first}-{last}") }).collect::<Vec<_>>().join(","); std::rc::Rc::<str>::from(if lines.len() == 1 { format!("line {text}") } else { format!("lines {text}") }) };
    v1
}
fn row4_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = concat5_24(v2, v4.clone(), v3, v5.clone(), v5);
    concat5_24(v0, v4.clone(), v1, v4, v6)
}
fn preview_of_39(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let old = (&*v0).lines().collect::<Vec<_>>(); let new = (&*v1).lines().collect::<Vec<_>>(); let mut shown = Vec::new(); if old.len() == new.len() { for (index, (left, right)) in old.iter().zip(new.iter()).enumerate() { if left != right { shown.push(format!("-{}  {}", index + 1, left)); shown.push(format!("+{}  {}", index + 1, right)); } } } else { let prefix = old.iter().zip(new.iter()).take_while(|(left, right)| left == right).count(); let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(left, right)| left == right).count(); for index in prefix..old.len() - suffix { shown.push(format!("-{}  {}", index + 1, old[index])); } for index in prefix..new.len() - suffix { shown.push(format!("+{}  {}", index + 1, new[index])); } } std::rc::Rc::<str>::from(shown.join("
")) };
    v2
}
fn pass_with_33(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: write_mode, mut v2: Rc<str>, mut v3: US7, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: bool, mut v16: bool, mut v17: Rc<str>, mut v18: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) }, v2.clone()));
    let v20: Rc<str> = lines_joined_31(v8, v4);
    let (mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
        US7::US7_Untouched => {
            (v5, v6, v7, v20, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
        }
        _ => {
            let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
                US7::US7_Refused(v21) => {
                    let v21: Rc<str> = v21.clone();
                    let v22: u64 = v7.wrapping_add(1u64);
                    let v23: Rc<str> = row2_34(v19, v21);
                    let v24: Rc<str> = lines_joined_31(v18, v23);
                    (v5, v6, v22, v20, v9, v10, v11, v12, v13, v14, v15, v16, v17, v24)
                }
                _ => {
                    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match v3 {
                        US7::US7_Changed(v25, v26, v27, v28) => {
                            let v25: Rc<str> = v25;
                            let v26: Rc<str> = v26;
                            let v27: u64 = v27;
                            let v28: Rc<str> = v28;
                            let v39: Rc<str> = match &v1 {
                                write_mode::write_mode_Apply => {
                                    let v29: i32 = v0.borrow().iter().position(|entry| *entry.0 == *v2).map_or(-1, |at| at as i32);
                                    let v30: bool = v29 < 0i32;
                                    if v30 {
                                        v0.borrow_mut().push((v2.clone(), v26.clone(), v25.clone()));
                                        ()
                                    } else {
                                        v0.borrow_mut()[v29 as usize].1 = v26.clone();
                                        ()
                                    };
                                    let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256 "); } LIT.with(|lit| lit.clone()) };
                                    let v32: Rc<str> = sha256_of_35(v25.clone());
                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                                    let v34: Rc<str> = sha256_of_35(v26.clone());
                                    let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    concat5_24(v31, v32, v33, v34, v35)
                                }
                                write_mode::write_mode_Preview => {
                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry run, nothing written"); } LIT.with(|lit| lit.clone()) };
                                    v37
                                }
                            };
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacement"); } LIT.with(|lit| lit.clone()) };
                            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                            let v42: Rc<str> = counted_36(v27, v40, v41);
                            let v43: Rc<str> = lines_label_37(v28);
                            let v44: Rc<str> = row4_38(v19, v42, v43, v39);
                            let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v1 {
                                write_mode::write_mode_Apply => {
                                    let v45: u64 = v11.wrapping_add(1u64);
                                    let v46: u64 = v13.wrapping_add(v27);
                                    let v47: Rc<str> = lines_joined_31(v17, v44);
                                    (v9, v10, v45, v12, v46, v14, v15, v16, v47, v18)
                                }
                                _ => {
                                    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match v1 {
                                        write_mode::write_mode_Preview => {
                                            let v48: u64 = v14.wrapping_add(v27);
                                            let v49: Rc<str> = lines_joined_31(v17, v44);
                                            let v50: Rc<str> = preview_of_39(v25, v26);
                                            let v51: Rc<str> = lines_joined_31(v49, v50);
                                            (v9, v10, v11, v12, v13, v48, v15, v16, v51, v18)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v52, v53, v54, v55, v56, v57, v58, v59, v60, v61)
                                }
                            };
                            let v72: u64 = v5.wrapping_add(1u64);
                            let v73: u64 = v6.wrapping_add(v27);
                            (v72, v73, v7, v20, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71)
                        }
                        _ => unreachable!(),
                    };
                    (v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
                }
            };
            (v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100, v101)
        }
    };
    (v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112, v113, v114, v115)
}
fn files_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: &Rc<str>, mut v3: write_mode, mut v4: expectation, mut v5: &Rc<str>, mut v6: Rc<rule_list>, mut v7: bool, mut v8: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v9: i32, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: bool, mut v21: bool, mut v22: Rc<str>, mut v23: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v24: i32 = v8.len() as i32;
    let v25: bool = v9 < v24;
    let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v25 {
        let v26: Rc<str> = v8[v9 as usize].clone();
        let v27: i32 = v9.wrapping_add(1i32);
        let (mut v28, mut v29): (US7, Rc<str>) = file_result_of_16(v0.clone(), v1.clone(), v6.clone(), v7, v26.clone());
        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = pass_with_33(v1.clone(), v3.clone(), v26, v28, v29, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23);
        let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_15(v0, v1, v2, v3, v4, v5, v6, v7, v8, v27, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43);
        (v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
    } else {
        (v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
    };
    (v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71)
}
fn block_run_11(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<str>, mut v3: write_mode, mut v4: expectation, mut v5: Rc<str>, mut v6: Rc<rule_list>, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: bool, mut v14: bool, mut v15: Rc<str>, mut v16: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v18: bool = v2.contains(&*v17);
    let v19: bool = &*v5 == "";
    let v23: std::rc::Rc<Vec<std::rc::Rc<str>>> = if v19 {
        target_paths_12(v0.clone(), v2.clone())
    } else {
        let v21: std::rc::Rc<Vec<std::rc::Rc<str>>> = target_paths_12(v0.clone(), v2.clone());
        paths_kept_13(v21, v5.clone())
    };
    let v24: u64 = v23.len() as u64;
    let v25: u64 = v7.wrapping_add(1u64);
    let v26: u64 = v10.wrapping_add(v24);
    let v27: bool = v13 || v18;
    let v28: i32 = 0i32;
    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = opened_block_14(v25, v8, v9, v26, v11, v12, v27, v14, v15, v16);
    let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_15(v0, v1, &v2, v3, v4.clone(), &v5, v6, v18, v23, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42);
    let v57: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) }, v2));
    let v58: bool = v45 == 0u64;
    let (mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v58 {
        let (mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v4 {
            expectation::expectation_Expected(v59, v60) => {
                let v59: u64 = *v59;
                let v60: u64 = *v60;
                let v61: bool = v59 == v43;
                let v63: bool = if v61 {
                    let v62: bool = v60 == v44;
                    v62
                } else {
                    false
                };
                if v63 {
                    (v47, v48, v49, v50, v51, v52, v53, v54, v55, v56)
                } else {
                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected files "); } LIT.with(|lit| lit.clone()) };
                    let v65: Rc<str> = decimal_text_28(v59);
                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replacements "); } LIT.with(|lit| lit.clone()) };
                    let v67: Rc<str> = decimal_text_28(v60);
                    let v68: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v69: Rc<str> = concat5_24(v64, v65, v66.clone(), v67, v68);
                    let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("found files "); } LIT.with(|lit| lit.clone()) };
                    let v71: Rc<str> = decimal_text_28(v43);
                    let v72: Rc<str> = decimal_text_28(v44);
                    let v73: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v74: Rc<str> = concat5_24(v70, v71, v66, v72, v73);
                    let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v69, v74));
                    let v76: Rc<str> = row2_34(v57, v75);
                    let v77: Rc<str> = lines_joined_31(v56, v76);
                    let v78: Rc<str> = lines_joined_31(v77, v46);
                    (v47, v48, v49, v50, v51, v52, v53, true, v55, v78)
                }
            }
            _ => {
                let (mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match v4 {
                    expectation::expectation_Unchecked => {
                        let v89: bool = v43 == 0u64;
                        if v89 {
                            let v90: u64 = v48.wrapping_add(1u64);
                            let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no file matched"); } LIT.with(|lit| lit.clone()) };
                            let v92: Rc<str> = row2_34(v57, v91);
                            let v93: Rc<str> = lines_joined_31(v56, v92);
                            (v47, v90, v49, v50, v51, v52, v53, v54, v55, v93)
                        } else {
                            (v47, v48, v49, v50, v51, v52, v53, v54, v55, v56)
                        }
                    }
                    _ => unreachable!(),
                };
                (v104, v105, v106, v107, v108, v109, v110, v111, v112, v113)
            }
        };
        (v114, v115, v116, v117, v118, v119, v120, v121, v122, v123)
    } else {
        let v124: u64 = v48.wrapping_add(1u64);
        (v47, v124, v49, v50, v51, v52, v53, v54, v55, v56)
    };
    (v125, v126, v127, v128, v129, v130, v131, v132, v133, v134)
}
fn shown_file_40(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.to_string()); let text = staged.or_else(|| std::fs::read(std::path::Path::new(&*v1).join(&*v2)).ok().map(|bytes| String::from_utf8_lossy(&bytes).into_owned())); std::rc::Rc::<str>::from(match text { Some(text) => { let lines = text.lines().collect::<Vec<_>>(); let width = lines.len().to_string().len(); let mut shown = vec![(&*v2).to_owned()]; for (index, line) in lines.iter().enumerate() { shown.push(format!("{:<width$}  {}", index + 1, line)); } shown.join("
") } None => format!("{} (unreadable)", &*v2) }) };
    v3
}
fn steps_run_10(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<steps>, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: bool, mut v10: bool, mut v11: Rc<str>, mut v12: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &*v2 {
        steps::steps_Done => {
            (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
        }
        _ => {
            let (mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match Rc::unwrap_or_clone(v2) {
                steps::steps_Step(v13, v14) => {
                    let v13: step = v13;
                    let v14: Rc<steps> = v14;
                    match &v13 {
                        step::step_EditStep(v15, v16, v17, v18, v19) => {
                            let v15: Rc<str> = v15.clone();
                            let v16: write_mode = v16.clone();
                            let v17: expectation = v17.clone();
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<rule_list> = v19.clone();
                            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = block_run_11(v0.clone(), v1.clone(), v15, v16, v17, v18, v19, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12);
                            steps_run_10(v0, v1, v14, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
                        }
                        _ => {
                            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v13 {
                                step::step_ShowStep(v40) => {
                                    let v40: Rc<str> = v40.clone();
                                    let v41: Rc<str> = shown_file_40(v1.clone(), v0.clone(), v40);
                                    let v42: Rc<str> = lines_joined_31(v11, v41);
                                    steps_run_10(v0, v1, v14, v3, v4, v5, v6, v7, v8, v9, v10, v42, v12)
                                }
                                _ => {
                                    let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match v13 {
                                        step::step_QueryStep(v53, v54) => {
                                            let v53: Rc<str> = v53;
                                            let v54: Rc<str> = v54;
                                            steps_run_10(v0, v1, v14, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
                                }
                            };
                            (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84)
                        }
                    }
                }
                _ => unreachable!(),
            };
            (v95, v96, v97, v98, v99, v100, v101, v102, v103, v104)
        }
    };
    (v105, v106, v107, v108, v109, v110, v111, v112, v113, v114)
}
fn committed_41(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_do_commit::kit_do_committed(v0, &*v1);
    v2
}
fn aligned_42(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1
}
fn printed_43(mut v0: Rc<str>) -> () {
    let mut v1: bool = &*v0 == "";
    if v1 {
        ()
    } else {
        println!("{}", &*v0);
        ()
    }
}
fn success_counts_44(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: &Rc<str>, mut v9: &Rc<str>) -> Rc<str> {
    let v10: Rc<str> = decimal_text_28(v2);
    let v11: Rc<str> = decimal_text_28(v4);
    let v12: Rc<str> = decimal_text_28(v5);
    let v13: bool = v5 == 0u64;
    if v13 {
        if v6 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = decimal_text_28(v3);
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  replacements "); } LIT.with(|lit| lit.clone()) }, v11));
            concat5_24(v14, v10, v15, v16, v17)
        } else {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  replacements "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_24(v19, v10, v20, v11, v21)
        }
    } else {
        let v24: bool = v4 == 0u64;
        if v24 {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0 written  replacements "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" previewed"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_24(v25, v12, v26, v27.clone(), v27)
        } else {
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written  replacements "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  previewed "); } LIT.with(|lit| lit.clone()) }, v12));
            concat5_24(v29, v10, v30, v11, v31)
        }
    }
}
fn edits_refused_45(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: &Rc<str>, mut v9: Rc<str>, mut v10: u64) -> i32 {
    let v11: Rc<str> = aligned_42(v9);
    printed_43(v11);
    let v12: bool = v1 == 0u64;
    let v20: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: the counts differ"); } LIT.with(|lit| lit.clone()) };
        v13
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = decimal_text_28(v1);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = decimal_text_28(v0);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" edits failed validation"); } LIT.with(|lit| lit.clone()) };
        concat5_24(v14, v15, v16, v17, v18)
    };
    let v21: i32 = 1i32;
    refused_2(v20, v21, v10)
}
fn edits_applied_8(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<steps>, mut v3: u64) -> US6 {
    let v4: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>, std::rc::Rc<str>)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = empty_tally_9();
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = steps_run_10(v0.clone(), v4.clone(), v2, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
    let v25: bool = v16 == 0u64;
    let v27: bool = if v25 {
        let v26: bool = v22 == false;
        v26
    } else {
        false
    };
    if v27 {
        let v30: Rc<str> = if v1 {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v28
        } else {
            committed_41(v4, v0)
        };
        let v31: bool = &*v30 == "";
        if v31 {
            let v32: Rc<str> = aligned_42(v23.clone());
            printed_43(v32);
            let v33: Rc<str> = success_counts_44(v15, v16, v17, v18, v19, v20, v21, v22, &v23, &v24);
            let v36: Rc<str> = if v1 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  dry run, nothing written"); } LIT.with(|lit| lit.clone()) };
                v34
            } else {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v35
            };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v36));
            US6::US6_EditsCommitted(v37)
        } else {
            let v39: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) }, v30));
            let v40: i32 = 1i32;
            let v41: i32 = refused_2(v39, v40, v3);
            US6::US6_EditsStopped(v41)
        }
    } else {
        let v44: i32 = edits_refused_45(v15, v16, v17, v18, v19, v20, v21, v22, &v23, v24, v3);
        US6::US6_EditsStopped(v44)
    }
}
fn part_at_46(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn edits_then_runs_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<steps>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = kit_do_process::kit_do_process_rejection(&*v3);
    let v6: bool = &*v5 != "";
    if v6 {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) }, v5));
        let v8: i32 = 2i32;
        refused_2(v7, v8, v4)
    } else {
        let v10: US6 = edits_applied_8(v0.clone(), v1, v2, v4);
        match v10 {
            US6::US6_EditsCommitted(v12) => {
                let v12: Rc<str> = v12;
                let v15: Rc<str> = if v1 {
                    let v13: Rc<str> = std::rc::Rc::<str>::from(format!("{0}runs not started{0}0", char::from(30u8)));
                    v13
                } else {
                    let v14: Rc<str> = kit_do_process::kit_do_process_after_edits(&*v0, &*v3);
                    v14
                };
                let v16: i32 = 0i32;
                let v17: Rc<str> = part_at_46(v15.clone(), v16);
                printed_43(v17);
                let v18: i32 = 2i32;
                let v19: Rc<str> = part_at_46(v15.clone(), v18);
                let v20: bool = &*v19 == "1";
                let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v12, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v22: i32 = 1i32;
                let v23: Rc<str> = part_at_46(v15, v22);
                let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v23));
                finished_7(v20, v24, v4)
            }
            US6::US6_EditsStopped(v11) => {
                let v11: i32 = v11;
                v11
            }
        }
    }
}
fn now_ms_47() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_48(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_at_50(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn count_of_52(mut v0: Rc<str>, mut v1: Rc<str>) -> replace_count {
    let v2: bool = &*v0 == "once";
    if v2 {
        replace_count::replace_count_Once
    } else {
        let v4: bool = &*v0 == "all";
        if v4 {
            replace_count::replace_count_Every
        } else {
            let v6: u64 = v1.trim().parse::<u64>().unwrap_or(0);
            replace_count::replace_count_Exactly(v6)
        }
    }
}
fn rules_decoded_51(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> (Rc<rule_list>, i32) {
    let v2: i32 = v0.len() as i32;
    let v3: bool = v1 < v2;
    let v11: bool = if v3 {
        let v4: i32 = 0i32;
        let mut v5: Rc<str> = field_at_50(v0.clone(), v1, v4);
        let v6: bool = &*v5 == "replace";
        if v6 {
            true
        } else {
            let v7: i32 = 0i32;
            let v8: Rc<str> = field_at_50(v0.clone(), v1, v7);
            let v9: bool = &*v8 == "pattern";
            v9
        }
    } else {
        false
    };
    if v11 {
        let v12: i32 = 3i32;
        let v13: Rc<str> = field_at_50(v0.clone(), v1, v12);
        let v14: i32 = 4i32;
        let v15: Rc<str> = field_at_50(v0.clone(), v1, v14);
        let v16: i32 = 1i32;
        let v17: Rc<str> = field_at_50(v0.clone(), v1, v16);
        let v18: i32 = 2i32;
        let v19: Rc<str> = field_at_50(v0.clone(), v1, v18);
        let v20: replace_count = count_of_52(v17, v19);
        let v21: i32 = 0i32;
        let v22: Rc<str> = field_at_50(v0.clone(), v1, v21);
        let v23: bool = &*v22 == "pattern";
        let v26: rule_matcher = if v23 {
            rule_matcher::rule_matcher_Pattern
        } else {
            rule_matcher::rule_matcher_Literal
        };
        let v27: i32 = v1.wrapping_add(1i32);
        let (mut v28, mut v29): (Rc<rule_list>, i32) = rules_decoded_51(v0, v27);
        let v30: Rc<rule_list> = Rc::new(rule_list::rule_list_Rule(v13, v15, v20, v26, v28));
        (v30, v29)
    } else {
        let v31: Rc<rule_list> = { thread_local!{ static CASE: Rc<rule_list> = Rc::new(rule_list::rule_list_NoRules); } CASE.with(|case| case.clone()) };
        (v31, v1)
    }
}
fn steps_decoded_49(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<steps> {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: i32 = 0i32;
            let mut v5: Rc<str> = field_at_50(v0.clone(), v1, v4);
            let v6: bool = &*v5 == "edit";
            if v6 {
                let v7: i32 = 3i32;
                let v8: Rc<str> = field_at_50(v0.clone(), v1, v7);
                let v9: bool = &*v8 == "-";
                let v16: expectation = if v9 {
                    expectation::expectation_Unchecked
                } else {
                    let v11: u64 = v8.trim().parse::<u64>().unwrap_or(0);
                    let v12: i32 = 4i32;
                    let v13: Rc<str> = field_at_50(v0.clone(), v1, v12);
                    let v14: u64 = v13.trim().parse::<u64>().unwrap_or(0);
                    expectation::expectation_Expected(v11, v14)
                };
                let v17: i32 = 2i32;
                let v18: Rc<str> = field_at_50(v0.clone(), v1, v17);
                let v19: bool = &*v18 == "preview";
                let v22: write_mode = if v19 {
                    write_mode::write_mode_Preview
                } else {
                    write_mode::write_mode_Apply
                };
                let v23: i32 = v1.wrapping_add(1i32);
                let v24: i32 = 0i32;
                let v25: Rc<str> = field_at_50(v0.clone(), v23, v24);
                let v26: bool = &*v25 == "except";
                let v30: Rc<str> = if v26 {
                    let v27: i32 = 1i32;
                    field_at_50(v0.clone(), v23, v27)
                } else {
                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v29
                };
                let v32: i32 = if v26 {
                    let v31: i32 = v1.wrapping_add(2i32);
                    v31
                } else {
                    v23
                };
                let (mut v33, mut v34): (Rc<rule_list>, i32) = rules_decoded_51(v0.clone(), v32);
                let v35: i32 = 1i32;
                let v36: Rc<str> = field_at_50(v0.clone(), v1, v35);
                let v37: Rc<steps> = steps_decoded_49(v0, v34);
                let v38: step = step::step_EditStep(v36, v22, v16, v30, v33);
                return Rc::new(steps::steps_Step(v38, v37));
            } else {
                let v40: bool = &*v5 == "show";
                if v40 {
                    let v41: i32 = 1i32;
                    let v42: Rc<str> = field_at_50(v0.clone(), v1, v41);
                    let v43: i32 = v1.wrapping_add(1i32);
                    let v44: Rc<steps> = steps_decoded_49(v0, v43);
                    let v45: step = step::step_ShowStep(v42);
                    return Rc::new(steps::steps_Step(v45, v44));
                } else {
                    let v47: i32 = v1.wrapping_add(1i32);
                    v1 = v47;
                    continue;
                }
            }
        } else {
            return { thread_local!{ static CASE: Rc<steps> = Rc::new(steps::steps_Done); } CASE.with(|case| case.clone()) };
        }
    }
}
fn runs_in_53(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = kit_do_verbs::kit_do_program_family(&*v0);
    let v2: bool = &*v1 == "runs";
    v2
}
fn edit_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_47();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_48(v1.clone());
    let v4: i32 = 1i32;
    let mut v5: Rc<steps> = steps_decoded_49(v3.clone(), v4);
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: Rc<str> = field_at_50(v3, v6, v7);
    let v9: bool = &*v8 == "dry";
    let v10: bool = runs_in_53(v1.clone());
    if v10 {
        edits_then_runs_1(v0, v9, v5, v1, v2)
    } else {
        let v12: US6 = edits_applied_8(v0, v9, v5, v2);
        match v12 {
            US6::US6_EditsCommitted(v13) => {
                let v13: Rc<str> = v13;
                let v14: bool = false;
                finished_7(v14, v13, v2)
            }
            US6::US6_EditsStopped(v16) => {
                let v16: i32 = v16;
                v16
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        edit_program_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_edit_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
