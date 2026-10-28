#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_Apply,
    US1_Preview,
}
#[derive(Clone)]
enum US2 {
    US2_Unchecked,
    US2_Expected(u64, u64),
}
#[derive(Clone)]
enum US3 {
    US3_Once,
    US3_Every,
    US3_Exactly(u64),
}
#[derive(Clone)]
enum US4 {
    US4_Literal,
    US4_Pattern,
}
#[derive(Clone)]
enum UH1 {
    UH1_NoRules,
    UH1_Rule(Rc<str>, Rc<str>, US3, US4, Rc<UH1>),
}
#[derive(Clone)]
enum US0 {
    US0_EditStep(Rc<str>, US1, US2, Rc<str>, Rc<UH1>),
    US0_ShowStep(Rc<str>),
    US0_QueryStep(Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Done,
    UH0_Step(US0, Rc<UH0>),
}
#[derive(Clone)]
enum US5 {
    US5_Succeeded,
    US5_Failed,
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
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_6() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_5() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_6();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_3(mut v0: US5, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US5::US5_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US5::US5_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_4(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_5();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn refused_2(mut v0: Rc<str>, mut v1: i32, mut v2: u64) -> i32 {
    println!("{}", &*v0);
    let v3: US5 = US5::US5_Failed;
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0  replacements 0"); } LIT.with(|lit| lit.clone()) };
    receipt_line_3(v3.clone(), v4.clone(), v2);
    v1
}
fn finished_7(mut v0: bool, mut v1: Rc<str>, mut v2: u64) -> i32 {
    let mut v5: US5 = if v0 {
        US5::US5_Failed
    } else {
        US5::US5_Succeeded
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
    (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, false, false, v0.clone(), v0.clone())
}
fn target_paths_12(mut v0: Rc<str>, mut v1: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v2: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &str, name: &str) -> bool { let parts = mask.split(char::from(42u8)).collect::<Vec<_>>(); if parts.len() == 1 { return mask == name; } let mut rest = name; if !rest.starts_with(parts[0]) { return false; } rest = &rest[parts[0].len()..]; for middle in &parts[1..parts.len() - 1] { match rest.find(middle) { Some(at) => rest = &rest[at + middle.len()..], None => return false } } rest.len() >= parts[parts.len() - 1].len() && rest.ends_with(parts[parts.len() - 1]) } fn joined(shown: &str, name: &str) -> String { if shown.is_empty() { name.to_owned() } else { format!("{shown}/{name}") } } fn walk(base: &std::path::Path, shown: &str, segments: &[String], found: &mut Vec<String>) { let Some((head, rest)) = segments.split_first() else { if base.is_file() { found.push(shown.to_owned()); } return; }; let children = || std::fs::read_dir(base).map(|entries| entries.filter_map(|entry| entry.ok()).filter(|entry| entry.file_type().map(|kind| !kind.is_symlink()).unwrap_or(false)).collect::<Vec<_>>()).unwrap_or_default(); if head == "**" { walk(base, shown, rest, found); for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if entry.path().is_dir() && ![".git", "target", "node_modules"].contains(&name.as_str()) { walk(&entry.path(), &joined(shown, &name), segments, found); } } } else if head.contains(char::from(42u8)) { for entry in children() { let name = entry.file_name().to_string_lossy().into_owned(); if wildcard(head, &name) { walk(&entry.path(), &joined(shown, &name), rest, found); } } } else { walk(&base.join(head), &joined(shown, head), rest, found); } } let pattern = (&*v1).replace(char::from(92u8), "/"); let segments = pattern.split(char::from(47u8)).filter(|segment| !segment.is_empty() && *segment != ".").map(str::to_owned).collect::<Vec<_>>(); let lead = if pattern.starts_with(char::from(47u8)) { "/" } else { "" }; let mut found = Vec::new(); if pattern.contains(char::from(42u8)) { let fixed = segments.iter().take_while(|segment| !segment.contains(char::from(42u8))).count(); let prefix = format!("{lead}{}", segments[..fixed].join("/")); walk(&std::path::Path::new(&*v0).join(&prefix), &prefix, &segments[fixed..], &mut found); found.sort(); found.dedup(); } else { found.push(format!("{lead}{}", segments.join("/"))); } std::rc::Rc::new(found.into_iter().map(std::rc::Rc::<str>::from).collect::<Vec<_>>()) };
    v2.clone()
}
fn paths_kept_13(mut v0: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v1: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let v2: std::rc::Rc<Vec<std::rc::Rc<str>>> = { fn wildcard(mask: &[u8], name: &[u8]) -> bool { match (mask.first(), name.first()) { (None, None) => true, (Some(&42u8), _) => wildcard(&mask[1..], name) || (!name.is_empty() && wildcard(mask, &name[1..])), (Some(a), Some(b)) => a == b && wildcard(&mask[1..], &name[1..]), _ => false } } let masks = (&*v1).split(char::from(44u8)).map(|mask| mask.trim().replace(char::from(92u8), "/")).filter(|mask| !mask.is_empty()).collect::<Vec<_>>(); std::rc::Rc::new(v0.iter().filter(|path| { let shown = path.replace(char::from(92u8), "/"); let name = shown.rsplit(char::from(47u8)).next().unwrap_or("").to_owned(); !masks.iter().any(|mask| wildcard(mask.as_bytes(), shown.as_bytes()) || wildcard(mask.as_bytes(), name.as_bytes())) }).cloned().collect::<Vec<_>>()) };
    v2.clone()
}
fn opened_block_14(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (0u64, 0u64, 0u64, v10.clone(), v0, v1, v2, v3, v4, v5, v6, v7, v8.clone(), v9.clone())
}
fn text_now_17(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Result<std::rc::Rc<str>, std::rc::Rc<str>> {
    let v3: Result<std::rc::Rc<str>, std::rc::Rc<str>> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.clone()); match staged { Some(text) => Ok(text), None => std::fs::read(std::path::Path::new(&*v1).join(&*v2)).map_err(|error| std::rc::Rc::<str>::from(format!("unreadable: {error}"))).and_then(|bytes| String::from_utf8(bytes).map(std::rc::Rc::<str>::from).map_err(|_| { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from("not UTF-8 text"); } LIT.with(|lit| lit.clone()) })) } };
    v3.clone()
}
fn opened_file_18(mut v0: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), 0u64, v1.clone(), v1.clone(), v1.clone())
}
fn replaced_in_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> {
    let v3: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = { let text = &*v0; let crlf = text.contains("
"); let adapt = |value: &str| if crlf && value.contains(char::from(10u8)) && !value.contains("
") { value.replace("
", "
") } else { value.to_owned() }; let anchor = adapt(&*v1); let replacement = adapt(&*v2); if anchor.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(text), 0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })); } let starts = text.match_indices(anchor.as_str()).map(|(at, _)| at).collect::<Vec<_>>(); let lines = starts.iter().map(|at| (text[..*at].matches(char::from(10u8)).count() + 1).to_string()).collect::<Vec<_>>().join(","); std::rc::Rc::new((std::rc::Rc::<str>::from(text.replace(anchor.as_str(), replacement.as_str())), starts.len() as u64, std::rc::Rc::<str>::from(lines))) };
    v3.clone()
}
fn replaced_by_pattern_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> {
    let v3: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = { let text = &*v0; match regex::Regex::new(&*v1) { Ok(matcher) => { let starts = matcher.find_iter(text).map(|found| found.start()).collect::<Vec<_>>(); let lines = starts.iter().map(|at| (text[..*at].matches(char::from(10u8)).count() + 1).to_string()).collect::<Vec<_>>().join(","); std::rc::Rc::new((std::rc::Rc::<str>::from(matcher.replace_all(text, &*v2).as_ref()), starts.len() as u64, std::rc::Rc::<str>::from(lines))) } Err(_) => std::rc::Rc::new((std::rc::Rc::<str>::from(text), 0, { thread_local!{ static LIT: std::rc::Rc<str> = std::rc::Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })) } };
    v3.clone()
}
fn pattern_problem_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(regex::Regex::new(&*v0).err().map(|error| error.to_string().lines().last().unwrap_or("").trim().to_owned()).unwrap_or_default());
    v1.clone()
}
fn joined_with_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = &*v2 == "";
    if v3 {
        v1.clone()
    } else {
        let v4: bool = &*v1 == "";
        if v4 {
            v2.clone()
        } else {
            let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = std::rc::Rc::<str>::from([&*v1, &*v0, &*v2, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn quoted_26(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let shown = format!("{:?}", &*v0); std::rc::Rc::<str>::from(if shown.chars().count() > 80 { format!("{}...{}", shown.chars().take(76).collect::<String>(), char::from(34u8)) } else { shown }) };
    v1
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn mismatch_25(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = v1 == 0u64;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor not found: "); } LIT.with(|lit| lit.clone()) };
        let mut v5: Rc<str> = quoted_26(v0.clone());
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (0 matches, expected "); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = std::rc::Rc::<str>::from([&*v4, &*v5, &*v6, &*v2, &*v7].concat());
        v8.clone()
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("anchor matched "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = decimal_text_27(v1);
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" times: "); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = quoted_26(v0.clone());
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" (expected "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = std::rc::Rc::<str>::from([&*v13, &*v2, &*v14, &*v15, &*v15].concat());
        let v17: Rc<str> = std::rc::Rc::<str>::from([&*v9, &*v10, &*v11, &*v12, &*v16].concat());
        v17.clone()
    }
}
fn count_problem_24(mut v0: US3, mut v1: u64, mut v2: bool, mut v3: Rc<str>) -> Rc<str> {
    let v4: bool = v1 == 0u64;
    let mut v5: bool = v4 && v2;
    match &v0 {
        US3::US3_Every => {
            let v20: bool = if v4 {
                let v19: bool = v2 == false;
                v19
            } else {
                false
            };
            if v20 {
                let v21: u64 = 0u64;
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at least 1"); } LIT.with(|lit| lit.clone()) };
                mismatch_25(v3.clone(), v21, v22.clone())
            } else {
                let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v24.clone()
            }
        }
        US3::US3_Exactly(v12) => {
            let v12: u64 = *v12;
            let v13: bool = v1 == v12;
            let v14: bool = v13 || v5;
            if v14 {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v15.clone()
            } else {
                let v16: Rc<str> = decimal_text_27(v12);
                mismatch_25(v3.clone(), v1, v16.clone())
            }
        }
        US3::US3_Once => {
            let v6: bool = v1 == 1u64;
            let v7: bool = v6 || v5;
            if v7 {
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v8.clone()
            } else {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
                mismatch_25(v3.clone(), v1, v9.clone())
            }
        }
    }
}
fn sites_of_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let text = &*v0; let anchor = &*v1; let first = anchor.lines().next().unwrap_or(anchor); let sites = if anchor.is_empty() { Vec::new() } else { text.lines().enumerate().filter(|(_, line)| line.contains(first)).map(|(index, line)| format!("{}:{}  {}", &*v2, index + 1, line)).collect::<Vec<_>>() }; std::rc::Rc::<str>::from(sites.join("
")) };
    v3.clone()
}
fn pattern_sites_of_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let sites = match regex::Regex::new(&*v1) { Ok(matcher) => (&*v0).lines().enumerate().filter(|(_, line)| matcher.is_match(line)).map(|(index, line)| format!("{}:{}  {}", &*v2, index + 1, line)).collect::<Vec<_>>(), Err(_) => Vec::new() }; std::rc::Rc::<str>::from(sites.join("
")) };
    v3.clone()
}
fn lines_joined_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    joined_with_23(v2.clone(), v0.clone(), v1.clone())
}
fn rules_over_19(mut v0: Rc<UH1>, mut v1: bool, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: u64, mut v5: Rc<str>, mut v6: Rc<str>, mut v7: Rc<str>) -> (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) {
    let (mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
        UH1::UH1_NoRules => {
            (v3.clone(), v4, v5.clone(), v6.clone(), v7.clone())
        }
        _ => {
            let (mut v51, mut v52, mut v53, mut v54, mut v55): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = match &*v0 {
                UH1::UH1_Rule(v8, v9, v10, v11, v12) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = v9.clone();
                    let v10: US3 = v10.clone();
                    let v11: US4 = v11.clone();
                    let v12: Rc<UH1> = v12.clone();
                    let v13: bool = &*v6 == "";
                    if v13 {
                        let v17: std::rc::Rc<(std::rc::Rc<str>, u64, std::rc::Rc<str>)> = match &v11 {
                            US4::US4_Literal => {
                                replaced_in_20(v3.clone(), v8.clone(), v9.clone())
                            }
                            US4::US4_Pattern => {
                                replaced_by_pattern_21(v3.clone(), v8.clone(), v9.clone())
                            }
                        };
                        let v21: Rc<str> = match &v11 {
                            US4::US4_Literal => {
                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                v18.clone()
                            }
                            US4::US4_Pattern => {
                                pattern_problem_22(v8.clone())
                            }
                        };
                        let v22: u64 = v17.clone().1;
                        let v23: Rc<str> = v17.0.clone();
                        let v24: u64 = v4.wrapping_add(v22);
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(","); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = v17.2.clone();
                        let v27: Rc<str> = joined_with_23(v25.clone(), v5.clone(), v26.clone());
                        let v28: bool = &*v21 == "";
                        let v35: Rc<str> = if v28 {
                            count_problem_24(v10.clone(), v22, v1, v8.clone())
                        } else {
                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("invalid pattern "); } LIT.with(|lit| lit.clone()) };
                            let v31: Rc<str> = quoted_26(v8.clone());
                            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v34: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v21, &*v33].concat());
                            v34.clone()
                        };
                        let v39: Rc<str> = match &v11 {
                            US4::US4_Literal => {
                                sites_of_28(v3.clone(), v8.clone(), v2.clone())
                            }
                            US4::US4_Pattern => {
                                pattern_sites_of_29(v3.clone(), v8.clone(), v2.clone())
                            }
                        };
                        let v40: Rc<str> = lines_joined_30(v7.clone(), v39.clone());
                        let (mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_19(v12.clone(), v1, v2.clone(), v23.clone(), v24, v27.clone(), v35.clone(), v40.clone());
                        (v41.clone(), v42, v43.clone(), v44.clone(), v45.clone())
                    } else {
                        (v3.clone(), v4, v5.clone(), v6.clone(), v7.clone())
                    }
                }
                _ => unreachable!(),
            };
            (v51.clone(), v52, v53.clone(), v54.clone(), v55.clone())
        }
    };
    (v56.clone(), v57, v58.clone(), v59.clone(), v60.clone())
}
fn emptied_problem_31(mut v0: Rc<str>) -> Rc<str> {
    let v1: u64 = v0.len() as u64;
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the edits would leave the file empty (was "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = decimal_text_27(v1);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" bytes); empty it with kit proxy fs-write --expect-hash"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v3, &*v4, &*v5, &*v5].concat());
    v6.clone()
}
fn file_result_of_16(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<UH1>, mut v3: bool, mut v4: Rc<str>) -> (US7, Rc<str>) {
    let mut v5: Result<std::rc::Rc<str>, std::rc::Rc<str>> = text_now_17(v1.clone(), v0.clone(), v4.clone());
    let v6: bool = v5.is_ok();
    let (mut v34, mut v35): (US7, Rc<str>) = if v6 {
        let v7: Rc<str> = v5.clone().unwrap_or_default();
        let (mut v8, mut v9, mut v10, mut v11, mut v12): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = opened_file_18(v7.clone());
        let (mut v13, mut v14, mut v15, mut v16, mut v17): (Rc<str>, u64, Rc<str>, Rc<str>, Rc<str>) = rules_over_19(v2.clone(), v3, v4.clone(), v8.clone(), v9, v10.clone(), v11.clone(), v12.clone());
        let v18: bool = &*v16 != "";
        let v30: US7 = if v18 {
            US7::US7_Refused(v16.clone())
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
                    let v25: Rc<str> = emptied_problem_31(v7.clone());
                    US7::US7_Refused(v25.clone())
                } else {
                    US7::US7_Changed(v7.clone(), v13.clone(), v14, v15.clone())
                }
            }
        };
        (v30.clone(), v17.clone())
    } else {
        let v31: Rc<str> = v5.clone().err().unwrap_or_default();
        let v32: US7 = US7::US7_Refused(v31.clone());
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (v32.clone(), v33.clone())
    };
    (v34.clone(), v35.clone())
}
fn row2_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn staged_34(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> () {
    { let mut writes = v0.borrow_mut(); match writes.iter_mut().find(|entry| &*entry.0 == &*v1) { Some(entry) => entry.1 = v2.clone(), None => writes.push((v1.clone(), v2.clone())) } };
    ()
}
fn sha256_of_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { use sha2::Digest; std::rc::Rc::<str>::from(sha2::Sha256::digest(v0.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect::<String>()) };
    v1.clone()
}
fn counted_36(mut v0: u64, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = decimal_text_27(v0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v5: bool = v0 == 1u64;
    let v6: Rc<str> = if v5 {
        v1.clone()
    } else {
        v2.clone()
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = std::rc::Rc::<str>::from([&*v3, &*v4, &*v6, &*v7, &*v7].concat());
    v8.clone()
}
fn lines_label_37(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut lines = (&*v0).split(char::from(44u8)).filter_map(|value| value.parse::<u64>().ok()).collect::<Vec<_>>(); lines.sort(); lines.dedup(); let mut ranges: Vec<(u64, u64)> = Vec::new(); for line in lines.iter().copied() { match ranges.last_mut() { Some(range) if range.1 + 1 == line => range.1 = line, _ => ranges.push((line, line)) } } let text = ranges.iter().map(|(first, last)| if first == last { first.to_string() } else { format!("{first}-{last}") }).collect::<Vec<_>>().join(","); std::rc::Rc::<str>::from(if lines.len() == 1 { format!("line {text}") } else { format!("lines {text}") }) };
    v1.clone()
}
fn row4_38(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v2, &*v4, &*v3, &*v5, &*v5].concat());
    let v7: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v4, &*v6].concat());
    v7.clone()
}
fn preview_of_39(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let old = (&*v0).lines().collect::<Vec<_>>(); let new = (&*v1).lines().collect::<Vec<_>>(); let mut shown = Vec::new(); if old.len() == new.len() { for (index, (left, right)) in old.iter().zip(new.iter()).enumerate() { if left != right { shown.push(format!("-{}  {}", index + 1, left)); shown.push(format!("+{}  {}", index + 1, right)); } } } else { let prefix = old.iter().zip(new.iter()).take_while(|(left, right)| left == right).count(); let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(left, right)| left == right).count(); for index in prefix..old.len() - suffix { shown.push(format!("-{}  {}", index + 1, old[index])); } for index in prefix..new.len() - suffix { shown.push(format!("+{}  {}", index + 1, new[index])); } } std::rc::Rc::<str>::from(shown.join("
")) };
    v2.clone()
}
fn pass_with_32(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: US1, mut v2: Rc<str>, mut v3: US7, mut v4: Rc<str>, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: bool, mut v16: bool, mut v17: Rc<str>, mut v18: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v2));
    let v21: Rc<str> = lines_joined_30(v8.clone(), v4.clone());
    let (mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
        US7::US7_Untouched => {
            (v5, v6, v7, v21.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17.clone(), v18.clone())
        }
        _ => {
            let (mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
                US7::US7_Refused(v22) => {
                    let v22: Rc<str> = v22.clone();
                    let v23: u64 = v7.wrapping_add(1u64);
                    let v24: Rc<str> = row2_33(v20.clone(), v22.clone());
                    let v25: Rc<str> = lines_joined_30(v18.clone(), v24.clone());
                    (v5, v6, v23, v21.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17.clone(), v25.clone())
                }
                _ => {
                    let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v3 {
                        US7::US7_Changed(v26, v27, v28, v29) => {
                            let v26: Rc<str> = v26.clone();
                            let v27: Rc<str> = v27.clone();
                            let v28: u64 = *v28;
                            let v29: Rc<str> = v29.clone();
                            let v38: Rc<str> = match &v1 {
                                US1::US1_Apply => {
                                    staged_34(v0.clone(), v2.clone(), v27.clone());
                                    let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sha256 "); } LIT.with(|lit| lit.clone()) };
                                    let v31: Rc<str> = sha256_of_35(v26.clone());
                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" -> "); } LIT.with(|lit| lit.clone()) };
                                    let v33: Rc<str> = sha256_of_35(v27.clone());
                                    let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v35: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v31, &*v32, &*v33, &*v34].concat());
                                    v35.clone()
                                }
                                US1::US1_Preview => {
                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dry run, nothing written"); } LIT.with(|lit| lit.clone()) };
                                    v36.clone()
                                }
                            };
                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacement"); } LIT.with(|lit| lit.clone()) };
                            let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                            let v41: Rc<str> = counted_36(v28, v39.clone(), v40.clone());
                            let v42: Rc<str> = lines_label_37(v29.clone());
                            let v43: Rc<str> = row4_38(v20.clone(), v41.clone(), v42.clone(), v38.clone());
                            let (mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v1 {
                                US1::US1_Apply => {
                                    let v44: u64 = v11.wrapping_add(1u64);
                                    let v45: u64 = v13.wrapping_add(v28);
                                    let v46: Rc<str> = lines_joined_30(v17.clone(), v43.clone());
                                    (v9, v10, v44, v12, v45, v14, v15, v16, v46.clone(), v18.clone())
                                }
                                _ => {
                                    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v1 {
                                        US1::US1_Preview => {
                                            let v47: u64 = v14.wrapping_add(v28);
                                            let v48: Rc<str> = lines_joined_30(v17.clone(), v43.clone());
                                            let v49: Rc<str> = preview_of_39(v26.clone(), v27.clone());
                                            let v50: Rc<str> = lines_joined_30(v48.clone(), v49.clone());
                                            (v9, v10, v11, v12, v13, v47, v15, v16, v50.clone(), v18.clone())
                                        }
                                        _ => unreachable!(),
                                    };
                                    (v51, v52, v53, v54, v55, v56, v57, v58, v59.clone(), v60.clone())
                                }
                            };
                            let v71: u64 = v5.wrapping_add(1u64);
                            let v72: u64 = v6.wrapping_add(v28);
                            (v71, v72, v7, v21.clone(), v61, v62, v63, v64, v65, v66, v67, v68, v69.clone(), v70.clone())
                        }
                        _ => unreachable!(),
                    };
                    (v73, v74, v75, v76.clone(), v77, v78, v79, v80, v81, v82, v83, v84, v85.clone(), v86.clone())
                }
            };
            (v87, v88, v89, v90.clone(), v91, v92, v93, v94, v95, v96, v97, v98, v99.clone(), v100.clone())
        }
    };
    (v101, v102, v103, v104.clone(), v105, v106, v107, v108, v109, v110, v111, v112, v113.clone(), v114.clone())
}
fn files_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<str>, mut v3: US1, mut v4: US2, mut v5: Rc<str>, mut v6: Rc<UH1>, mut v7: bool, mut v8: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v9: i32, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64, mut v18: u64, mut v19: u64, mut v20: bool, mut v21: bool, mut v22: Rc<str>, mut v23: Rc<str>) -> (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v24: i32 = v8.len() as i32;
    let v25: bool = v9 < v24;
    let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v25 {
        let v26: Rc<str> = v8[v9 as usize].clone();
        let v27: i32 = v9.wrapping_add(1i32);
        let (mut v28, mut v29): (US7, Rc<str>) = file_result_of_16(v0.clone(), v1.clone(), v6.clone(), v7, v26.clone());
        let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = pass_with_32(v1.clone(), v3.clone(), v26, v28, v29, v10, v11, v12, v13.clone(), v14, v15, v16, v17, v18, v19, v20, v21, v22.clone(), v23.clone());
        let (mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_15(v0, v1, v2, v3, v4, v5, v6, v7, v8, v27, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43);
        (v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
    } else {
        (v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
    };
    (v58, v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70, v71)
}
fn block_run_11(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<str>, mut v3: US1, mut v4: US2, mut v5: Rc<str>, mut v6: Rc<UH1>, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: bool, mut v14: bool, mut v15: Rc<str>, mut v16: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("*"); } LIT.with(|lit| lit.clone()) };
    let v18: bool = v2.contains(&*v17);
    let v19: bool = &*v5 == "";
    let v23: std::rc::Rc<Vec<std::rc::Rc<str>>> = if v19 {
        target_paths_12(v0.clone(), v2.clone())
    } else {
        let v21: std::rc::Rc<Vec<std::rc::Rc<str>>> = target_paths_12(v0.clone(), v2.clone());
        paths_kept_13(v21.clone(), v5.clone())
    };
    let v24: u64 = v23.len() as u64;
    let v25: u64 = v7.wrapping_add(1u64);
    let v26: u64 = v10.wrapping_add(v24);
    let v27: bool = v13 || v18;
    let v28: i32 = 0i32;
    let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = opened_block_14(v25, v8, v9, v26, v11, v12, v27, v14, v15.clone(), v16.clone());
    let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (u64, u64, u64, Rc<str>, u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = files_run_15(v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v18, v23.clone(), v28, v29, v30, v31, v32.clone(), v33, v34, v35, v36, v37, v38, v39, v40, v41.clone(), v42.clone());
    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
    let v58: Rc<str> = Rc::<str>::from(format!("{}{}", v57, v2));
    let v59: bool = v45 == 0u64;
    let (mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = if v59 {
        let (mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v4 {
            US2::US2_Expected(v60, v61) => {
                let v60: u64 = *v60;
                let v61: u64 = *v61;
                let v62: bool = v60 == v43;
                let v64: bool = if v62 {
                    let v63: bool = v61 == v44;
                    v63
                } else {
                    false
                };
                if v64 {
                    (v47, v48, v49, v50, v51, v52, v53, v54, v55.clone(), v56.clone())
                } else {
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("expected files "); } LIT.with(|lit| lit.clone()) };
                    let v66: Rc<str> = decimal_text_27(v60);
                    let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" replacements "); } LIT.with(|lit| lit.clone()) };
                    let v68: Rc<str> = decimal_text_27(v61);
                    let v69: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
                    let v70: Rc<str> = std::rc::Rc::<str>::from([&*v65, &*v66, &*v67, &*v68, &*v69].concat());
                    let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("found files "); } LIT.with(|lit| lit.clone()) };
                    let v72: Rc<str> = decimal_text_27(v43);
                    let v73: Rc<str> = decimal_text_27(v44);
                    let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v75: Rc<str> = std::rc::Rc::<str>::from([&*v71, &*v72, &*v67, &*v73, &*v74].concat());
                    let v76: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v75));
                    let v77: Rc<str> = row2_33(v58.clone(), v76.clone());
                    let v78: Rc<str> = lines_joined_30(v56.clone(), v77.clone());
                    let v79: Rc<str> = lines_joined_30(v78.clone(), v46.clone());
                    (v47, v48, v49, v50, v51, v52, v53, true, v55.clone(), v79.clone())
                }
            }
            _ => {
                let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v4 {
                    US2::US2_Unchecked => {
                        let v90: bool = v43 == 0u64;
                        if v90 {
                            let v91: u64 = v48.wrapping_add(1u64);
                            let v92: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("no file matched"); } LIT.with(|lit| lit.clone()) };
                            let v93: Rc<str> = row2_33(v58.clone(), v92.clone());
                            let v94: Rc<str> = lines_joined_30(v56.clone(), v93.clone());
                            (v47, v91, v49, v50, v51, v52, v53, v54, v55.clone(), v94.clone())
                        } else {
                            (v47, v48, v49, v50, v51, v52, v53, v54, v55.clone(), v56.clone())
                        }
                    }
                    _ => unreachable!(),
                };
                (v105, v106, v107, v108, v109, v110, v111, v112, v113.clone(), v114.clone())
            }
        };
        (v115, v116, v117, v118, v119, v120, v121, v122, v123.clone(), v124.clone())
    } else {
        let v125: u64 = v48.wrapping_add(1u64);
        (v47, v125, v49, v50, v51, v52, v53, v54, v55.clone(), v56.clone())
    };
    (v126, v127, v128, v129, v130, v131, v132, v133, v134.clone(), v135.clone())
}
fn shown_file_40(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = { let staged = v0.borrow().iter().find(|entry| &*entry.0 == &*v2).map(|entry| entry.1.to_string()); let text = staged.or_else(|| std::fs::read(std::path::Path::new(&*v1).join(&*v2)).ok().map(|bytes| String::from_utf8_lossy(&bytes).into_owned())); std::rc::Rc::<str>::from(match text { Some(text) => { let lines = text.lines().collect::<Vec<_>>(); let width = lines.len().to_string().len(); let mut shown = vec![(&*v2).to_owned()]; for (index, line) in lines.iter().enumerate() { shown.push(format!("{:<width$}  {}", index + 1, line)); } shown.join("
") } None => format!("{} (unreadable)", &*v2) }) };
    v3.clone()
}
fn steps_run_10(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v2: Rc<UH0>, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: bool, mut v10: bool, mut v11: Rc<str>, mut v12: Rc<str>) -> (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) {
    let (mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &*v2 {
        UH0::UH0_Done => {
            (v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12.clone())
        }
        _ => {
            let (mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &*v2 {
                UH0::UH0_Step(v13, v14) => {
                    let v13: US0 = v13.clone();
                    let v14: Rc<UH0> = v14.clone();
                    match &v13 {
                        US0::US0_EditStep(v15, v16, v17, v18, v19) => {
                            let v15: Rc<str> = v15.clone();
                            let v16: US1 = v16.clone();
                            let v17: US2 = v17.clone();
                            let v18: Rc<str> = v18.clone();
                            let v19: Rc<UH1> = v19.clone();
                            let (mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = block_run_11(v0.clone(), v1.clone(), v15, v16, v17, v18, v19, v3, v4, v5, v6, v7, v8, v9, v10, v11.clone(), v12.clone());
                            steps_run_10(v0.clone(), v1.clone(), v14.clone(), v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
                        }
                        _ => {
                            let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v13 {
                                US0::US0_ShowStep(v40) => {
                                    let v40: Rc<str> = v40.clone();
                                    let v41: Rc<str> = shown_file_40(v1.clone(), v0.clone(), v40);
                                    let v42: Rc<str> = lines_joined_30(v11.clone(), v41);
                                    steps_run_10(v0.clone(), v1.clone(), v14.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v42, v12.clone())
                                }
                                _ => {
                                    let (mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = match &v13 {
                                        US0::US0_QueryStep(v53, v54) => {
                                            let v53: Rc<str> = v53.clone();
                                            let v54: Rc<str> = v54.clone();
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
fn committed_41(mut v0: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { let writes = v0.borrow(); let root = std::path::Path::new(&*v1); let mut temporaries = Vec::new(); let mut problem = String::new(); for (path, text) in writes.iter() { let target = root.join(&**path); let temporary = target.with_file_name(format!(".{}.kit-do.tmp", target.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default())); match std::fs::write(&temporary, text.as_bytes()) { Ok(()) => temporaries.push((temporary, target)), Err(error) => { problem = format!("write {path}: {error}"); break; } } } if problem.is_empty() { for (temporary, target) in &temporaries { if let Err(error) = std::fs::rename(temporary, target) { problem = format!("rename {}: {error}", target.display()); break; } } } else { for (temporary, _) in &temporaries { let _ = std::fs::remove_file(temporary); } } std::rc::Rc::<str>::from(problem) };
    v2.clone()
}
fn aligned_42(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn printed_43(mut v0: Rc<str>) -> () {
    let v1: bool = &*v0 == "";
    if v1 {
        ()
    } else {
        println!("{}", &*v0);
        ()
    }
}
fn success_counts_44(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>) -> Rc<str> {
    let v10: Rc<str> = decimal_text_27(v2);
    let v11: Rc<str> = decimal_text_27(v4);
    let v12: Rc<str> = decimal_text_27(v5);
    let v13: bool = v5 == 0u64;
    if v13 {
        if v6 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = decimal_text_27(v3);
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  replacements "); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v11));
            let v19: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v10, &*v15, &*v16, &*v18].concat());
            v19.clone()
        } else {
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  replacements "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = std::rc::Rc::<str>::from([&*v20, &*v10, &*v21, &*v11, &*v22].concat());
            v23.clone()
        }
    } else {
        let v25: bool = v4 == 0u64;
        if v25 {
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0 written  replacements "); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" previewed"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = std::rc::Rc::<str>::from([&*v26, &*v12, &*v27, &*v28, &*v28].concat());
            v29.clone()
        } else {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" written  replacements "); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  previewed "); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v12));
            let v34: Rc<str> = std::rc::Rc::<str>::from([&*v30, &*v10, &*v31, &*v11, &*v33].concat());
            v34.clone()
        }
    }
}
fn edits_refused_45(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: bool, mut v7: bool, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: u64) -> i32 {
    let v11: Rc<str> = aligned_42(v9.clone());
    printed_43(v11.clone());
    let v12: bool = v1 == 0u64;
    let v20: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: the counts differ"); } LIT.with(|lit| lit.clone()) };
        v13.clone()
    } else {
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = decimal_text_27(v1);
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" of "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = decimal_text_27(v0);
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" edits failed validation"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from([&*v14, &*v15, &*v16, &*v17, &*v18].concat());
        v19.clone()
    };
    let v21: i32 = 1i32;
    refused_2(v20.clone(), v21, v10)
}
fn edits_applied_8(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<UH0>, mut v3: u64) -> US6 {
    let v4: std::rc::Rc<std::cell::RefCell<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>> = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = empty_tally_9();
    let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24): (u64, u64, u64, u64, u64, u64, bool, bool, Rc<str>, Rc<str>) = steps_run_10(v0.clone(), v4.clone(), v2.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13.clone(), v14.clone());
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
            v28.clone()
        } else {
            committed_41(v4.clone(), v0.clone())
        };
        let v31: bool = &*v30 == "";
        if v31 {
            let v32: Rc<str> = aligned_42(v23.clone());
            printed_43(v32.clone());
            let v33: Rc<str> = success_counts_44(v15, v16, v17, v18, v19, v20, v21, v22, v23.clone(), v24.clone());
            let v36: Rc<str> = if v1 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  dry run, nothing written"); } LIT.with(|lit| lit.clone()) };
                v34.clone()
            } else {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v35.clone()
            };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v36));
            US6::US6_EditsCommitted(v37.clone())
        } else {
            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nothing written: "); } LIT.with(|lit| lit.clone()) };
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v39, v30));
            let v41: i32 = 1i32;
            let v42: i32 = refused_2(v40.clone(), v41, v3);
            US6::US6_EditsStopped(v42)
        }
    } else {
        let v45: i32 = edits_refused_45(v15, v16, v17, v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v3);
        US6::US6_EditsStopped(v45)
    }
}
fn part_at_46(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn edits_then_runs_1(mut v0: Rc<str>, mut v1: bool, mut v2: Rc<UH0>, mut v3: Rc<str>, mut v4: u64) -> i32 {
    let mut v5: Rc<str> = kit_do_process::kit_do_process_rejection(&*v3);
    let v6: bool = &*v5 != "";
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v5));
        let v9: i32 = 2i32;
        refused_2(v8.clone(), v9, v4)
    } else {
        let v11: US6 = edits_applied_8(v0.clone(), v1, v2.clone(), v4);
        match &v11 {
            US6::US6_EditsCommitted(v13) => {
                let v13: Rc<str> = v13.clone();
                let v16: Rc<str> = if v1 {
                    let v14: Rc<str> = std::rc::Rc::<str>::from(format!("{0}runs not started{0}0", char::from(30u8)));
                    v14.clone()
                } else {
                    let v15: Rc<str> = kit_do_process::kit_do_process_after_edits(&*v0, &*v3);
                    v15.clone()
                };
                let v17: i32 = 0i32;
                let v18: Rc<str> = part_at_46(v16.clone(), v17);
                printed_43(v18.clone());
                let v19: i32 = 2i32;
                let v20: Rc<str> = part_at_46(v16.clone(), v19);
                let v21: bool = &*v20 == "1";
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
                let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v22));
                let v24: i32 = 1i32;
                let v25: Rc<str> = part_at_46(v16.clone(), v24);
                let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v25));
                finished_7(v21, v26.clone(), v4)
            }
            US6::US6_EditsStopped(v12) => {
                let v12: i32 = *v12;
                v12
            }
        }
    }
}
fn now_ms_47() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_48(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_at_50(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn count_of_52(mut v0: Rc<str>, mut v1: Rc<str>) -> US3 {
    let v2: bool = &*v0 == "once";
    if v2 {
        US3::US3_Once
    } else {
        let v4: bool = &*v0 == "all";
        if v4 {
            US3::US3_Every
        } else {
            let v6: u64 = v1.trim().parse::<u64>().unwrap_or(0);
            US3::US3_Exactly(v6)
        }
    }
}
fn rules_decoded_51(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> (Rc<UH1>, i32) {
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
        let v20: US3 = count_of_52(v17.clone(), v19.clone());
        let v21: i32 = 0i32;
        let v22: Rc<str> = field_at_50(v0.clone(), v1, v21);
        let v23: bool = &*v22 == "pattern";
        let v26: US4 = if v23 {
            US4::US4_Pattern
        } else {
            US4::US4_Literal
        };
        let v27: i32 = v1.wrapping_add(1i32);
        let (mut v28, mut v29): (Rc<UH1>, i32) = rules_decoded_51(v0.clone(), v27);
        let v30: Rc<UH1> = Rc::new(UH1::UH1_Rule(v13.clone(), v15.clone(), v20.clone(), v26.clone(), v28.clone()));
        (v30.clone(), v29)
    } else {
        let v31: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_NoRules); } CASE.with(|case| case.clone()) };
        (v31.clone(), v1)
    }
}
fn steps_decoded_49(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<UH0> {
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
                let v16: US2 = if v9 {
                    US2::US2_Unchecked
                } else {
                    let v11: u64 = v8.trim().parse::<u64>().unwrap_or(0);
                    let v12: i32 = 4i32;
                    let v13: Rc<str> = field_at_50(v0.clone(), v1, v12);
                    let v14: u64 = v13.trim().parse::<u64>().unwrap_or(0);
                    US2::US2_Expected(v11, v14)
                };
                let v17: i32 = 2i32;
                let v18: Rc<str> = field_at_50(v0.clone(), v1, v17);
                let v19: bool = &*v18 == "preview";
                let v22: US1 = if v19 {
                    US1::US1_Preview
                } else {
                    US1::US1_Apply
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
                    v29.clone()
                };
                let v32: i32 = if v26 {
                    let v31: i32 = v1.wrapping_add(2i32);
                    v31
                } else {
                    v23
                };
                let (mut v33, mut v34): (Rc<UH1>, i32) = rules_decoded_51(v0.clone(), v32);
                let v35: i32 = 1i32;
                let v36: Rc<str> = field_at_50(v0.clone(), v1, v35);
                let v37: Rc<UH0> = steps_decoded_49(v0.clone(), v34);
                let v38: US0 = US0::US0_EditStep(v36.clone(), v22.clone(), v16.clone(), v30.clone(), v33.clone());
                return Rc::new(UH0::UH0_Step(v38.clone(), v37.clone()));
            } else {
                let v40: bool = &*v5 == "show";
                if v40 {
                    let v41: i32 = 1i32;
                    let v42: Rc<str> = field_at_50(v0.clone(), v1, v41);
                    let v43: i32 = v1.wrapping_add(1i32);
                    let v44: Rc<UH0> = steps_decoded_49(v0.clone(), v43);
                    let v45: US0 = US0::US0_ShowStep(v42.clone());
                    return Rc::new(UH0::UH0_Step(v45.clone(), v44.clone()));
                } else {
                    let v47: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v47);
                    continue;
                }
            }
        } else {
            return { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Done); } CASE.with(|case| case.clone()) };
        }
    }
}
fn runs_in_53(mut v0: Rc<str>) -> bool {
    let v1: Rc<str> = kit_do_verbs::kit_do_program_family(&*v0);
    let v2: bool = &*v1 == "runs";
    v2
}
fn edit_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_47();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_48(v1.clone());
    let v4: i32 = 1i32;
    let mut v5: Rc<UH0> = steps_decoded_49(v3.clone(), v4);
    let v6: i32 = 0i32;
    let v7: i32 = 0i32;
    let v8: Rc<str> = field_at_50(v3, v6, v7);
    let v9: bool = &*v8 == "dry";
    let v10: bool = runs_in_53(v1.clone());
    if v10 {
        edits_then_runs_1(v0.clone(), v9, v5.clone(), v1, v2)
    } else {
        let v12: US6 = edits_applied_8(v0, v9, v5, v2);
        match &v12 {
            US6::US6_EditsCommitted(v13) => {
                let v13: Rc<str> = v13.clone();
                let v14: bool = false;
                finished_7(v14, v13, v2)
            }
            US6::US6_EditsStopped(v16) => {
                let v16: i32 = *v16;
                v16
            }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        edit_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_edit_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
