#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_verbs::{tool_verb, option_tool_verb};
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
enum US1 {
    US1_JobStart(Rc<str>, Rc<str>, Rc<str>, bool),
    US1_JobWait(Rc<str>, u64),
    US1_JobStop(Rc<str>),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_JobParsed(US1),
    US0_JobRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_ActionStart,
    US2_ActionWait,
    US2_ActionStop,
}
#[derive(Clone)]
enum US5 {
    US5_Some(US2),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(u64),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Succeeded,
    US7_Failed,
}
pub struct Mut0 { pub l0: Rc<str> }
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_index_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_between_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize).map(|fields| fields.iter().take(if v3 < 0 { fields.len() } else { v3 as usize }).skip(v2 as usize).filter(|raw| raw.len() > 1).map(|raw| if raw.starts_with(char::from(113u8)) { format!("{}{}{}", char::from(34u8), &raw[1..], char::from(34u8)) } else { raw[1..].to_owned() }).collect::<Vec<_>>().join(" ")).unwrap_or_default());
    v4
}
fn body_at_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3
}
fn duration_ms_10(mut v0: Rc<str>) -> u64 {
    let mut v1: u64 = { let text = (&*v0).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<f64>().map(|value| (value * scale as f64) as u64).unwrap_or(0) };
    v1
}
fn start_parsed_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as"); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = field_index_7(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_7(v0.clone(), v1, v4);
    let v6: i32 = 3i32;
    let v7: Rc<str> = words_between_8(v0.clone(), v1, v6, v3);
    let v8: bool = v3 < 0i32;
    let v10: bool = if v8 {
        true
    } else {
        let v9: bool = &*v7 == "";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected job start <command> as <name> timeout <duration> [notify]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v11)
    } else {
        let v13: bool = v5 < 0i32;
        let v18: bool = if v13 {
            true
        } else {
            let v14: i32 = v5.wrapping_add(1i32);
            let v15: Rc<str> = body_at_9(v0.clone(), v1, v14);
            let v16: u64 = duration_ms_10(v15);
            let v17: bool = v16 == 0u64;
            v17
        };
        if v18 {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected timeout <duration> such as 30s or 5m"); } LIT.with(|lit| lit.clone()) };
            US0::US0_JobRejected(v19)
        } else {
            let v21: i32 = v3.wrapping_add(1i32);
            let v22: Rc<str> = body_at_9(v0.clone(), v1, v21);
            let v23: bool = &*v22 == "";
            let v25: bool = if v23 {
                true
            } else {
                let v24: bool = &*v22 == "timeout";
                v24
            };
            if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as: expected a job name after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_JobRejected(v26)
            } else {
                let v28: i32 = v5.wrapping_add(1i32);
                let v29: Rc<str> = body_at_9(v0.clone(), v1, v28);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify"); } LIT.with(|lit| lit.clone()) };
                let v31: i32 = field_index_7(v0, v1, v30);
                let v32: bool = v31 > 0i32;
                let v33: US1 = US1::US1_JobStart(v22, v7, v29, v32);
                US0::US0_JobParsed(v33)
            }
        }
    }
}
fn stop_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 3i32;
    let mut v3: Rc<str> = body_at_9(v0, v1, v2);
    let mut v4: bool = &*v3 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job stop: expected job stop <name>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v5)
    } else {
        let v7: US1 = US1::US1_JobStop(v3);
        US0::US0_JobParsed(v7)
    }
}
fn wait_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 3i32;
    let mut v3: Rc<str> = body_at_9(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_7(v0.clone(), v1, v4);
    let v6: bool = &*v3 == "";
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v3 == "timeout";
        v7
    };
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: expected job wait <name> [timeout <duration>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v9)
    } else {
        let v11: bool = v5 < 0i32;
        let v15: u64 = if v11 {
            0u64
        } else {
            let v12: i32 = v5.wrapping_add(1i32);
            let v13: Rc<str> = body_at_9(v0, v1, v12);
            duration_ms_10(v13)
        };
        let v16: US1 = US1::US1_JobWait(v3, v15);
        US0::US0_JobParsed(v16)
    }
}
fn action_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: US2) -> US0 {
    match v2 {
        US2::US2_ActionStart => {
            start_parsed_6(v0, v1)
        }
        US2::US2_ActionStop => {
            stop_parsed_11(v0, v1)
        }
        US2::US2_ActionWait => {
            wait_parsed_12(v0, v1)
        }
    }
}
fn field_at_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn job_tool_14(mut v0: Rc<str>) -> bool {
    let mut v1: option_tool_verb = kit_do_verbs::kit_do_tool_named(&v0);
    match v1 {
        option_tool_verb::option_tool_verb_None => {
            false
        }
        option_tool_verb::option_tool_verb_Some(v2) => {
            let v2: tool_verb = v2;
            match v2 {
                tool_verb::tool_verb_ToolJob => {
                    true
                }
                _ => {
                    false
                }
            }
        }
    }
}
fn action_named_15(mut v0: Rc<str>) -> US5 {
    let mut v1: bool = &*v0 == "start";
    if v1 {
        let v2: US2 = US2::US2_ActionStart;
        US5::US5_Some(v2)
    } else {
        let mut v4: bool = &*v0 == "wait";
        if v4 {
            let v5: US2 = US2::US2_ActionWait;
            US5::US5_Some(v5)
        } else {
            let v7: bool = &*v0 == "stop";
            if v7 {
                let v8: US2 = US2::US2_ActionStop;
                US5::US5_Some(v8)
            } else {
                US5::US5_None
            }
        }
    }
}
fn job_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_13(v0.clone(), v1, v2);
    let mut v4: i32 = 2i32;
    let v5: Rc<str> = body_at_9(v0.clone(), v1, v4);
    let v6: bool = &*v3 == "";
    if v6 {
        let v7: US1 = US1::US1_Blank;
        US0::US0_JobParsed(v7)
    } else {
        let v9: bool = &*v3 != "query";
        let v14: bool = if v9 {
            true
        } else {
            let v10: i32 = 1i32;
            let v11: Rc<str> = field_at_13(v0.clone(), v1, v10);
            let v12: bool = job_tool_14(v11);
            let v13: bool = v12 == false;
            v13
        };
        if v14 {
            let v15: i32 = 1i32;
            let v16: Rc<str> = field_at_13(v0, v1, v15);
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with job yet; run it as its own call"); } LIT.with(|lit| lit.clone()) }));
            US0::US0_JobRejected(v17)
        } else {
            let v19: US5 = action_named_15(v5.clone());
            match v19 {
                US5::US5_None => {
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: unknown action "); } LIT.with(|lit| lit.clone()) }, v5));
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v22, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected start, wait or stop"); } LIT.with(|lit| lit.clone()) }));
                    US0::US0_JobRejected(v23)
                }
                US5::US5_Some(v20) => {
                    let v20: US2 = v20;
                    action_parsed_5(v0, v1, v20)
                }
            }
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = job_parsed_4(v0.clone(), v1);
            match v4 {
                US0::US0_JobParsed(v6) => {
                    let v6: US1 = v6;
                    let v7: i32 = v1.wrapping_add(1i32);
                    v1 = v7;
                    continue;
                }
                US0::US0_JobRejected(v5) => {
                    let v5: Rc<str> = v5;
                    return v5;
                }
            }
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v11;
        }
    }
}
fn empty_job_tally_16() -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, v0.clone(), v0, 0i32)
}
fn current_directory_of_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::canonicalize(&*v0).map(|path| path.to_string_lossy().trim_start_matches(&format!("{0}{0}?{0}", char::from(92u8))).to_owned()).unwrap_or_else(|_| (&*v0).to_owned()));
    v1
}
fn launch_argv_via_21(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_command_line::kit_launch_argv(&*v0, &*v1);
    v2
}
fn launched_22(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = kit_job_control::kit_job_launched(&*v0, v1, &*v2, &*v3);
    v4
}
fn lines_joined_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        let mut v3: bool = &*v0 == "";
        if v3 {
            v1
        } else {
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) }));
            let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v1));
            v5
        }
    }
}
fn part_at_24(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn started_pid_25(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { let mut pid = String::new(); for _ in 0..40 { let summary = kit_job_control::kit_job_settled(&*v0, 0); if let Some(at) = summary.find(" pid=") { pid = summary[at + 5..].split(char::from(32u8)).next().unwrap_or("").to_owned(); } if !pid.is_empty() { break; } std::thread::sleep(std::time::Duration::from_millis(50)); } std::rc::Rc::<str>::from(pid) };
    v1
}
fn start_run_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: bool, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v14: Rc<str> = current_directory_of_20(v0);
    let v15: u64 = duration_ms_10(v3.clone());
    let v16: Rc<str> = launch_argv_via_21(v14.clone(), v2.clone());
    let v17: Rc<str> = launched_22(v1.clone(), v15, v16, v14);
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let v19: bool = v17.starts_with(&*v18);
    let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v19 {
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) }, v1));
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v17));
        let v23: Rc<str> = lines_joined_23(v5, v22);
        (v23, true, v7, v8, v9, v10, v11, v12, 1i32)
    } else {
        let v24: i32 = 0i32;
        let v25: Rc<str> = part_at_24(v17, v24);
        let v26: Rc<str> = started_pid_25(v25);
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) }, v1.clone()));
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v27, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  started "); } LIT.with(|lit| lit.clone()) }));
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v2));
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) }, v26));
        let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v30, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timeout "); } LIT.with(|lit| lit.clone()) }));
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v3));
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v32));
        let v34: Rc<str> = lines_joined_23(v5, v33);
        let v35: u64 = v7.wrapping_add(1u64);
        let v36: Rc<str> = lines_joined_23(v11, v1.clone());
        let v38: Rc<str> = if v4 {
            lines_joined_23(v12, v1)
        } else {
            v12
        };
        (v34, v6, v35, v8, v9, v10, v36, v38, v13)
    };
    (v39, v40, v41, v42, v43, v44, v45, v46, v47)
}
fn labeled_27(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_job_control::kit_job_labeled(&*v0);
    v1
}
fn name_listed_28(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v0).lines().any(|item| item == &*v1);
    v2
}
fn settled_29(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = kit_job_control::kit_job_settled(&*v0, v1);
    v2
}
fn summary_field_30(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let marker = format!(" {}=", &*v1); let summary = &*v0; match summary.find(&marker) { Some(at) => { let rest = &summary[at + marker.len()..]; if let Some(quoted) = rest.strip_prefix(char::from(34u8)) { quoted.split(char::from(34u8)).next().unwrap_or("").to_owned() } else { rest.split(char::from(32u8)).next().unwrap_or("").to_owned() } } None => String::new() } });
    v2
}
fn notice_of_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = &*v1 == "finished";
    let v9: Rc<str> = if v3 {
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished (exit "); } LIT.with(|lit| lit.clone()) }, v2));
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
        v5
    } else {
        let v6: bool = &*v1 == "timed-out";
        if v6 {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out"); } LIT.with(|lit| lit.clone()) };
            v7
        } else {
            v1
        }
    };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify  "); } LIT.with(|lit| lit.clone()) }, v0));
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) }));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v9));
    v12
}
fn method34(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US6> = Rc::new(move |mut v0: (u64)| -> US6 {
        let mut v1: u64 = (v0);
        US6::US6_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_33(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method34(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US6> = closure1();
    let mut v4: Option<US6> = v2.map(|x| v3(x));
    let v6: US6 = US6::US6_None;
    let v7: US6 = v6.clone();
    let v8: US6 = v4.unwrap_or(v7);
    match v8 {
        US6::US6_None => {
            0u64
        }
        US6::US6_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn duration_text_35(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from({ let ms = v0; if ms % 1000 == 0 && ms > 0 { format!("{}s", ms / 1000) } else if ms >= 1000 { format!("{:.1}s", ms as f64 / 1000.0) } else { format!("{ms}ms") } });
    v1
}
fn settled_line_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = summary_field_30(v1.clone(), v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("detail"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = summary_field_30(v1.clone(), v5);
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) }, v0));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v9: bool = &*v4 == "finished";
    if v9 {
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished after "); } LIT.with(|lit| lit.clone()) }));
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
        let v12: Rc<str> = summary_field_30(v1.clone(), v11);
        let v13: u64 = decimal_of_33(v12);
        let v14: Rc<str> = duration_text_35(v13);
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v14));
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) }));
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = summary_field_30(v1, v17);
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v18));
        v19
    } else {
        let v20: bool = &*v4 == "timed-out";
        if v20 {
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out after "); } LIT.with(|lit| lit.clone()) }));
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = summary_field_30(v1, v22);
            let v24: u64 = decimal_of_33(v23);
            let v25: Rc<str> = duration_text_35(v24);
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v25));
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
            let v28: bool = v6.contains(&*v27);
            let v34: Rc<str> = if v28 {
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
                v29
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout; "); } LIT.with(|lit| lit.clone()) };
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = Rc::<str>::from(v6.replace(&*v30, &*v31));
                let v33: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }, v32));
                v33
            };
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v34));
            let v36: Rc<str> = Rc::<str>::from(format!("{}{}", v35, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit killed"); } LIT.with(|lit| lit.clone()) }));
            v36
        } else {
            let v37: bool = &*v4 == "running";
            let v39: bool = if v37 {
                true
            } else {
                let v38: bool = &*v4 == "pending";
                v38
            };
            if v39 {
                let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("still running after "); } LIT.with(|lit| lit.clone()) }));
                let v41: Rc<str> = duration_text_35(v2);
                let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
                v42
            } else {
                let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v4));
                let v44: Rc<str> = Rc::<str>::from(format!("{}{}", v43, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
                let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v6));
                v45
            }
        }
    }
}
fn output_tail_37(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = kit_job_control::kit_job_output(&*v0, v1);
    v2
}
fn loop_38(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0, 0i32 as i64, v5 as i64);
            v0 = v6;
            continue;
        } else {
            return v0;
        }
    }
}
fn go_40(mut v0: &Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = v0.borrow()[v2 as usize].clone();
            let v6: i32 = (v5.clone().len() as i32);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r"); } LIT.with(|lit| lit.clone()) };
            let v8: bool = v5.ends_with(&*v7);
            let v9: bool = v8 == false;
            let v15: Rc<str> = if v9 {
                v5
            } else {
                let v10: bool = v6 == 1i32;
                if v10 {
                    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v11
                } else {
                    let v12: i32 = v6.wrapping_sub(2i32);
                    let v13: Rc<str> = string_slice(&v5, 0i32 as i64, v12 as i64);
                    v13
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
fn line_count_39(mut v0: Rc<str>) -> u64 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 > 0i32;
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
    let v13: i32 = go_40(&v2, v10, v11, v12);
    let v14: u64 = (v13 as u64);
    v14
}
fn decimal_text_41(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn output_block_36(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: u64 = 10u64;
    let mut v3: Rc<str> = output_tail_37(v1, v2);
    let mut v4: Rc<str> = loop_38(v3);
    let v5: u64 = line_count_39(v4.clone());
    let v6: bool = v5 == 0u64;
    if v6 {
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (no lines)"); } LIT.with(|lit| lit.clone()) }));
        v7
    } else {
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (last "); } LIT.with(|lit| lit.clone()) }));
        let v9: Rc<str> = decimal_text_41(v5);
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
        let v11: bool = v5 == 1u64;
        let v14: Rc<str> = if v11 {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" line)"); } LIT.with(|lit| lit.clone()) };
            v12
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)"); } LIT.with(|lit| lit.clone()) };
            v13
        };
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v14));
        lines_joined_23(v15, v4)
    }
}
fn wait_run_26(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v11: Rc<str> = labeled_27(v0.clone());
    let v12: bool = name_listed_28(v8.clone(), v0.clone());
    let v14: u64 = if v12 {
        v4
    } else {
        let v13: u64 = v4.wrapping_add(1u64);
        v13
    };
    let v15: bool = &*v11 == "";
    let (mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v15 {
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: no job named "); } LIT.with(|lit| lit.clone()) }, v0));
        let v17: Rc<str> = lines_joined_23(v2, v16);
        (v17, true, v14, v5, v6, v7, v8, v9, 1i32)
    } else {
        let v18: bool = v1 == 0u64;
        let v19: u64 = if v18 {
            86400000u64
        } else {
            v1
        };
        let v20: Rc<str> = settled_29(v11.clone(), v19);
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = summary_field_30(v20.clone(), v21);
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = summary_field_30(v20.clone(), v23);
        let v25: bool = name_listed_28(v9.clone(), v0.clone());
        let v28: Rc<str> = if v25 {
            notice_of_31(v0.clone(), v22.clone(), v24.clone())
        } else {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v27
        };
        let v29: bool = &*v22 == "timed-out";
        let v31: bool = if v29 {
            true
        } else {
            let v30: bool = &*v22 == "running";
            v30
        };
        let v33: bool = if v31 {
            true
        } else {
            let v32: bool = &*v22 == "pending";
            v32
        };
        let v38: i32 = if v33 {
            3i32
        } else {
            let v34: bool = &*v22 == "finished";
            let v36: bool = if v34 {
                let v35: bool = &*v24 == "0";
                v35
            } else {
                false
            };
            if v36 {
                0i32
            } else {
                1i32
            }
        };
        let v39: Rc<str> = settled_line_32(v0.clone(), v20, v1);
        let v40: Rc<str> = lines_joined_23(v2, v39);
        let v41: Rc<str> = lines_joined_23(v40, v28.clone());
        let v42: Rc<str> = output_block_36(v0, v11);
        let v43: Rc<str> = lines_joined_23(v41, v42);
        let v44: bool = &*v22 == "finished";
        let v45: u64 = if v44 {
            1u64
        } else {
            0u64
        };
        let v46: u64 = if v29 {
            1u64
        } else {
            0u64
        };
        let v47: bool = &*v28 == "";
        let v48: u64 = if v47 {
            0u64
        } else {
            1u64
        };
        let v50: bool = if v3 {
            true
        } else {
            let v49: bool = v38 != 0i32;
            v49
        };
        let v51: u64 = v5.wrapping_add(v45);
        let v52: u64 = v6.wrapping_add(v46);
        let v53: u64 = v7.wrapping_add(v48);
        let v54: bool = v38 > v10;
        let v55: i32 = if v54 {
            v38
        } else {
            v10
        };
        (v43, v50, v14, v51, v52, v53, v8, v9, v55)
    };
    (v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn stopped_text_43(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_job_control::kit_job_stop_text(&*v0);
    v1
}
fn stop_run_42(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v10: Rc<str> = labeled_27(v0.clone());
    let v11: bool = &*v10 == "";
    let (mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v11 {
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job stop: no job named "); } LIT.with(|lit| lit.clone()) }, v0));
        let v13: Rc<str> = lines_joined_23(v1, v12);
        (v13, true, v3, v4, v5, v6, v7, v8, 1i32)
    } else {
        let v14: Rc<str> = stopped_text_43(v10);
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
        let v16: bool = v14.starts_with(&*v15);
        let (mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v16 {
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) }, v0));
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v17, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v14));
            let v20: Rc<str> = lines_joined_23(v1, v19);
            (v20, true, v3, v4, v5, v6, v7, v8, 1i32)
        } else {
            let v21: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) }, v0));
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v21, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped ("); } LIT.with(|lit| lit.clone()) }));
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("detail"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = summary_field_30(v14, v23);
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v24));
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) }));
            let v27: Rc<str> = lines_joined_23(v1, v26);
            (v27, v2, v3, v4, v5, v6, v7, v8, v9)
        };
        (v28, v29, v30, v31, v32, v33, v34, v35, v36)
    };
    (v37, v38, v39, v40, v41, v42, v43, v44, v45)
}
fn step_run_18(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
        US1::US1_JobStart(v11, v12, v13, v14) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let v14: bool = *v14;
            start_run_19(v0, v11, v12, v13, v14, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        _ => {
            let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                US1::US1_JobWait(v24, v25) => {
                    let v24: Rc<str> = v24.clone();
                    let v25: u64 = *v25;
                    wait_run_26(v24, v25, v2, v3, v4, v5, v6, v7, v8, v9, v10)
                }
                _ => {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                        US1::US1_JobStop(v35) => {
                            let v35: Rc<str> = v35.clone();
                            stop_run_42(v35, v2, v3, v4, v5, v6, v7, v8, v9, v10)
                        }
                        _ => {
                            let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match v1 {
                                US1::US1_Blank => {
                                    (v2, v3, v4, v5, v6, v7, v8, v9, v10)
                                }
                                _ => unreachable!(),
                            };
                            (v45, v46, v47, v48, v49, v50, v51, v52, v53)
                        }
                    };
                    (v54, v55, v56, v57, v58, v59, v60, v61, v62)
                }
            };
            (v63, v64, v65, v66, v67, v68, v69, v70, v71)
        }
    };
    (v72, v73, v74, v75, v76, v77, v78, v79, v80)
}
fn jobs_run_17(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v12: i32 = v1.len() as i32;
    let v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v13 {
        let v14: US0 = job_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v14 {
            US0::US0_JobParsed(v15) => {
                let v15: US1 = v15.clone();
                let v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = step_run_18(v0.clone(), v15, v3, v4, v5, v6, v7, v8, v9, v10, v11);
                jobs_run_17(v0, v1, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match v14 {
                    US0::US0_JobRejected(v35) => {
                        let v35: Rc<str> = v35;
                        let v36: i32 = v2.wrapping_add(1i32);
                        jobs_run_17(v0, v1, v36, v3, v4, v5, v6, v7, v8, v9, v10, v11)
                    }
                    _ => unreachable!(),
                };
                (v46, v47, v48, v49, v50, v51, v52, v53, v54)
            }
        };
        (v55, v56, v57, v58, v59, v60, v61, v62, v63)
    } else {
        (v3, v4, v5, v6, v7, v8, v9, v10, v11)
    };
    (v64, v65, v66, v67, v68, v69, v70, v71, v72)
}
fn method47(mut v0: &Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_46(mut v0: i32) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1 }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method47(&v2, v3);
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4
}
fn code_text_45(mut v0: i32) -> Rc<str> {
    format_real_46(v0)
}
fn receipt_counts_44(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: &Rc<str>, mut v7: Rc<str>, mut v8: i32) -> Rc<str> {
    let v9: Rc<str> = decimal_text_41(v2);
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs "); } LIT.with(|lit| lit.clone()) }, v9));
    let v11: bool = v3 > 0u64;
    let v15: Rc<str> = if v11 {
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  finished "); } LIT.with(|lit| lit.clone()) }));
        let v13: Rc<str> = decimal_text_41(v3);
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
        v14
    } else {
        v10
    };
    let v16: bool = v4 > 0u64;
    let v20: Rc<str> = if v16 {
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timed out "); } LIT.with(|lit| lit.clone()) }));
        let v18: Rc<str> = decimal_text_41(v4);
        let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
        v19
    } else {
        v15
    };
    let v21: bool = &*v7 != "";
    let v25: Rc<str> = if v21 {
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  notifications "); } LIT.with(|lit| lit.clone()) }));
        let v23: Rc<str> = decimal_text_41(v5);
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
        v24
    } else {
        v20
    };
    let v26: bool = v8 == 0i32;
    if v26 {
        v25
    } else {
        let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) }));
        let v28: Rc<str> = code_text_45(v8);
        let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
        v29
    }
}
fn elapsed_text_49(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_51() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_50() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_51();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_48(mut v0: US7, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US7::US7_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US7::US7_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_49(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_50();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn job_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let mut v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = empty_job_tally_16();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = jobs_run_17(v0, v3, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
        let v26: bool = &*v17 == "";
        if v26 {
            ()
        } else {
            println!("{}", v17);
            ()
        };
        if v18 {
            let v27: US7 = US7::US7_Failed;
            let v28: Rc<str> = receipt_counts_44(&v17, v18, v19, v20, v21, v22, &v23, v24, v25);
            receipt_line_48(v27, v28, v2);
            v25
        } else {
            let v29: US7 = US7::US7_Succeeded;
            let v30: Rc<str> = receipt_counts_44(&v17, v18, v19, v20, v21, v22, &v23, v24, v25);
            receipt_line_48(v29, v30, v2);
            0i32
        }
    } else {
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) }, v5));
        println!("{}", v32);
        let v33: US7 = US7::US7_Failed;
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_48(v33, v34, v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        job_program_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_jobs_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub type SpiralShape_97df63ebabc3 = Mut0;
