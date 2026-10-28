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
    US2_Some(u64),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Succeeded,
    US3_Failed,
}
struct Mut0 { l0: Rc<str> }
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let mut v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_between_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v0.get(v1 as usize).map(|fields| fields.iter().take(if v3 < 0 { fields.len() } else { v3 as usize }).skip(v2 as usize).filter(|raw| raw.len() > 1).map(|raw| if raw.starts_with(char::from(113u8)) { format!("{}{}{}", char::from(34u8), &raw[1..], char::from(34u8)) } else { raw[1..].to_owned() }).collect::<Vec<_>>().join(" ")).unwrap_or_default());
    v4.clone()
}
fn body_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn duration_ms_9(mut v0: Rc<str>) -> u64 {
    let v1: u64 = { let text = (&*v0).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<f64>().map(|value| (value * scale as f64) as u64).unwrap_or(0) };
    v1
}
fn start_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as"); } LIT.with(|lit| lit.clone()) };
    let mut v3: i32 = field_index_6(v0.clone(), v1, v2.clone());
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_6(v0.clone(), v1, v4.clone());
    let v6: i32 = 3i32;
    let v7: Rc<str> = words_between_7(v0.clone(), v1, v6, v3);
    let v8: bool = v3 < 0i32;
    let v10: bool = if v8 {
        true
    } else {
        let v9: bool = &*v7 == "";
        v9
    };
    if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected job start <command> as <name> timeout <duration> [notify]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v11.clone())
    } else {
        let v13: bool = v5 < 0i32;
        let v18: bool = if v13 {
            true
        } else {
            let v14: i32 = v5.wrapping_add(1i32);
            let v15: Rc<str> = body_at_8(v0.clone(), v1, v14);
            let v16: u64 = duration_ms_9(v15.clone());
            let v17: bool = v16 == 0u64;
            v17
        };
        if v18 {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job start: expected timeout <duration> such as 30s or 5m"); } LIT.with(|lit| lit.clone()) };
            US0::US0_JobRejected(v19.clone())
        } else {
            let v21: i32 = v3.wrapping_add(1i32);
            let v22: Rc<str> = body_at_8(v0.clone(), v1, v21);
            let v23: bool = &*v22 == "";
            let v25: bool = if v23 {
                true
            } else {
                let v24: bool = &*v22 == "timeout";
                v24
            };
            if v25 {
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("as: expected a job name after it"); } LIT.with(|lit| lit.clone()) };
                US0::US0_JobRejected(v26.clone())
            } else {
                let v28: i32 = v5.wrapping_add(1i32);
                let v29: Rc<str> = body_at_8(v0.clone(), v1, v28);
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify"); } LIT.with(|lit| lit.clone()) };
                let v31: i32 = field_index_6(v0.clone(), v1, v30.clone());
                let v32: bool = v31 > 0i32;
                let v33: US1 = US1::US1_JobStart(v22.clone(), v7.clone(), v29.clone(), v32);
                US0::US0_JobParsed(v33.clone())
            }
        }
    }
}
fn wait_parsed_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 3i32;
    let mut v3: Rc<str> = body_at_8(v0.clone(), v1, v2);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout"); } LIT.with(|lit| lit.clone()) };
    let v5: i32 = field_index_6(v0.clone(), v1, v4.clone());
    let v6: bool = &*v3 == "";
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = &*v3 == "timeout";
        v7
    };
    if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: expected job wait <name> [timeout <duration>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v9.clone())
    } else {
        let v11: bool = v5 < 0i32;
        let v15: u64 = if v11 {
            0u64
        } else {
            let v12: i32 = v5.wrapping_add(1i32);
            let v13: Rc<str> = body_at_8(v0.clone(), v1, v12);
            duration_ms_9(v13.clone())
        };
        let v16: US1 = US1::US1_JobWait(v3.clone(), v15);
        US0::US0_JobParsed(v16.clone())
    }
}
fn stop_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 3i32;
    let mut v3: Rc<str> = body_at_8(v0.clone(), v1, v2);
    let mut v4: bool = &*v3 == "";
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job stop: expected job stop <name>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_JobRejected(v5.clone())
    } else {
        let v7: US1 = US1::US1_JobStop(v3.clone());
        US0::US0_JobParsed(v7.clone())
    }
}
fn field_at_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn job_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let mut v3: Rc<str> = field_at_12(v0.clone(), v1, v2);
    let mut v4: i32 = 2i32;
    let v5: Rc<str> = body_at_8(v0.clone(), v1, v4);
    let v6: bool = &*v3 == "";
    if v6 {
        let v7: US1 = US1::US1_Blank;
        US0::US0_JobParsed(v7.clone())
    } else {
        let v9: bool = &*v3 != "query";
        let v13: bool = if v9 {
            true
        } else {
            let v10: i32 = 1i32;
            let v11: Rc<str> = field_at_12(v0.clone(), v1, v10);
            let v12: bool = &*v11 != "job";
            v12
        };
        if v13 {
            let v14: i32 = 1i32;
            let v15: Rc<str> = field_at_12(v0.clone(), v1, v14);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with job yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            US0::US0_JobRejected(v17.clone())
        } else {
            let v19: bool = &*v5 == "start";
            if v19 {
                start_parsed_5(v0.clone(), v1)
            } else {
                let v21: bool = &*v5 == "wait";
                if v21 {
                    wait_parsed_10(v0.clone(), v1)
                } else {
                    let v23: bool = &*v5 == "stop";
                    if v23 {
                        stop_parsed_11(v0.clone(), v1)
                    } else {
                        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: unknown action "); } LIT.with(|lit| lit.clone()) };
                        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v5));
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected start, wait or stop"); } LIT.with(|lit| lit.clone()) };
                        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                        US0::US0_JobRejected(v28.clone())
                    }
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
            match &v4 {
                US0::US0_JobParsed(v6) => {
                    let v6: US1 = v6.clone();
                    let v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v7);
                    continue;
                }
                US0::US0_JobRejected(v5) => {
                    let v5: Rc<str> = v5.clone();
                    return v5.clone();
                }
            }
        } else {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v11.clone();
        }
    }
}
fn empty_job_tally_13() -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, v0.clone(), v0.clone(), 0i32)
}
fn current_directory_of_17(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from(std::fs::canonicalize(&*v0).map(|path| path.to_string_lossy().trim_start_matches(&format!("{0}{0}?{0}", char::from(92u8))).to_owned()).unwrap_or_else(|_| (&*v0).to_owned()));
    v1.clone()
}
fn launch_argv_via_18(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = kit_command_line::kit_launch_argv(&*v0, &*v1);
    v2
}
fn launched_19(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = kit_job_control::kit_job_launched(&*v0, v1, &*v2, &*v3);
    v4
}
fn lines_joined_20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v1 == "";
    if v2 {
        v0.clone()
    } else {
        let mut v3: bool = &*v0 == "";
        if v3 {
            v1.clone()
        } else {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v4)), v1));
            v6.clone()
        }
    }
}
fn part_at_21(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn started_pid_22(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let mut pid = String::new(); for _ in 0..40 { let summary = kit_job_control::kit_job_settled(&*v0, 0); if let Some(at) = summary.find(" pid=") { pid = summary[at + 5..].split(char::from(32u8)).next().unwrap_or("").to_owned(); } if !pid.is_empty() { break; } std::thread::sleep(std::time::Duration::from_millis(50)); } std::rc::Rc::<str>::from(pid) };
    v1
}
fn start_run_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: bool, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: Rc<str>, mut v12: Rc<str>, mut v13: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v14: Rc<str> = current_directory_of_17(v0.clone());
    let v15: u64 = duration_ms_9(v3.clone());
    let v16: Rc<str> = launch_argv_via_18(v14.clone(), v2.clone());
    let v17: Rc<str> = launched_19(v1.clone(), v15, v16.clone(), v14.clone());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
    let v19: bool = v17.starts_with(&*v18);
    let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v19 {
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v1));
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v21, v22)), v17));
        let v25: Rc<str> = lines_joined_20(v5.clone(), v24.clone());
        (v25.clone(), true, v7, v8, v9, v10, v11.clone(), v12.clone(), 1i32)
    } else {
        let v26: i32 = 0i32;
        let v27: Rc<str> = part_at_21(v17.clone(), v26);
        let v28: Rc<str> = started_pid_22(v27.clone());
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v1));
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  started "); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v30, v31)), v2));
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  pid "); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v34, v28));
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timeout "); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = Rc::<str>::from(format!("{}{}", v33, Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v35, v36)), v3))));
        let v40: Rc<str> = lines_joined_20(v5.clone(), v39.clone());
        let v41: u64 = v7.wrapping_add(1u64);
        let v42: Rc<str> = lines_joined_20(v11.clone(), v1.clone());
        let v44: Rc<str> = if v4 {
            lines_joined_20(v12.clone(), v1.clone())
        } else {
            v12.clone()
        };
        (v40.clone(), v6, v41, v8, v9, v10, v42.clone(), v44.clone(), v13)
    };
    (v45.clone(), v46, v47, v48, v49, v50, v51.clone(), v52.clone(), v53)
}
fn labeled_24(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_job_control::kit_job_labeled(&*v0);
    v1
}
fn name_listed_25(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: bool = (v0).lines().any(|item| item == &*v1);
    v2
}
fn settled_26(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = kit_job_control::kit_job_settled(&*v0, v1);
    v2
}
fn summary_field_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from({ let marker = format!(" {}=", &*v1); let summary = &*v0; match summary.find(&marker) { Some(at) => { let rest = &summary[at + marker.len()..]; if let Some(quoted) = rest.strip_prefix(char::from(34u8)) { quoted.split(char::from(34u8)).next().unwrap_or("").to_owned() } else { rest.split(char::from(32u8)).next().unwrap_or("").to_owned() } } None => String::new() } });
    v2
}
fn notice_of_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let mut v3: bool = &*v1 == "finished";
    let v11: Rc<str> = if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished (exit "); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v2));
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
        v7.clone()
    } else {
        let v8: bool = &*v1 == "timed-out";
        if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out"); } LIT.with(|lit| lit.clone()) };
            v9.clone()
        } else {
            v1.clone()
        }
    };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notify  "); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v0));
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v14)), v11));
    v16.clone()
}
fn method31(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US2> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US2> = Rc::new(move |mut v0: (u64)| -> US2 {
        let v1: u64 = (v0);
        US2::US2_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_30(mut v0: Rc<str>) -> u64 {
    let v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let v2: Option<u64> = method31(v1.clone());
    let mut v3: Rc<dyn Fn((u64)) -> US2> = closure1();
    let mut v4: Option<US2> = v2.map(|x| v3(x));
    let v6: US2 = US2::US2_None;
    let v7: US2 = v6.clone();
    let v8: US2 = v4.unwrap_or(v7);
    match &v8 {
        US2::US2_None => {
            0u64
        }
        US2::US2_Some(v9) => {
            let v9: u64 = *v9;
            v9
        }
    }
}
fn duration_text_32(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from({ let ms = v0; if ms % 1000 == 0 && ms > 0 { format!("{}s", ms / 1000) } else if ms >= 1000 { format!("{:.1}s", ms as f64 / 1000.0) } else { format!("{ms}ms") } });
    v1
}
fn settled_line_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64) -> Rc<str> {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = summary_field_27(v1.clone(), v3.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("detail"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = summary_field_27(v1.clone(), v5.clone());
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v0));
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
    let v11: bool = &*v4 == "finished";
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("finished after "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v12));
        let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("elapsed_ms"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = summary_field_27(v1.clone(), v14.clone());
        let v16: u64 = decimal_of_30(v15.clone());
        let v17: Rc<str> = duration_text_32(v16);
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v17));
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = summary_field_27(v1.clone(), v21.clone());
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v22));
        v23.clone()
    } else {
        let v24: bool = &*v4 == "timed-out";
        if v24 {
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timed out after "); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v25));
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("timeout_ms"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = summary_field_27(v1.clone(), v27.clone());
            let v29: u64 = decimal_of_30(v28.clone());
            let v30: Rc<str> = duration_text_32(v29);
            let v31: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v30));
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process tree stopped"); } LIT.with(|lit| lit.clone()) };
            let v33: bool = v6.contains(&*v32);
            let v39: Rc<str> = if v33 {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped (tree)"); } LIT.with(|lit| lit.clone()) };
                v34.clone()
            } else {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wall-clock timeout; "); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v37: Rc<str> = Rc::<str>::from(v6.replace(&*v35, &*v36));
                let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v37));
                v38.clone()
            };
            let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v31, v39));
            let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit killed"); } LIT.with(|lit| lit.clone()) };
            let v42: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v41));
            v42.clone()
        } else {
            let v43: bool = &*v4 == "running";
            let v45: bool = if v43 {
                true
            } else {
                let v44: bool = &*v4 == "pending";
                v44
            };
            if v45 {
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("still running after "); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v46));
                let v48: Rc<str> = duration_text_32(v2);
                let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v47, v48));
                v49.clone()
            } else {
                let v52: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v4)), v9)), v6));
                v52.clone()
            }
        }
    }
}
fn output_tail_34(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: Rc<str> = kit_job_control::kit_job_output(&*v0, v1);
    v2
}
fn loop_35(mut v0: Rc<str>) -> Rc<str> {
    loop {
        let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v2: bool = v0.ends_with(&*v1);
        if v2 {
            let mut v3: i32 = (v0.clone().len() as i32);
            let mut v4: i32 = v3.wrapping_sub(1i32);
            let v5: i32 = v4.wrapping_sub(1i32);
            let v6: Rc<str> = string_slice(&v0.clone(), 0i32 as i64, v5 as i64);
            v0 = v6.clone();
            continue;
        } else {
            return v0.clone();
        }
    }
}
fn go_37(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let mut v4: bool = v2 >= v1;
        if v4 {
            return v3;
        } else {
            let v5: Rc<str> = v0.clone().borrow()[v2 as usize].clone();
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
            (v0, v1, v2, v3) = (v0.clone(), v1, v16, v19);
            continue;
        }
    }
}
fn line_count_36(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let mut v3: i32 = (v2.clone().borrow().len() as i32);
    let mut v4: bool = v3 > 0i32;
    let v8: bool = if v4 {
        let v5: i32 = v3.wrapping_sub(1i32);
        let v6: Rc<str> = v2.clone().borrow()[v5 as usize].clone();
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
    let v13: i32 = go_37(v2.clone(), v10, v11, v12);
    let v14: u64 = (v13 as u64);
    v14
}
fn decimal_text_38(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn output_block_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: u64 = 10u64;
    let mut v3: Rc<str> = output_tail_34(v1.clone(), v2);
    let mut v4: Rc<str> = loop_35(v3.clone());
    let v5: u64 = line_count_36(v4.clone());
    let v6: bool = v5 == 0u64;
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (no lines)"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v7));
        v8.clone()
    } else {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" output (last "); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v9));
        let v11: Rc<str> = decimal_text_38(v5);
        let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
        let v13: bool = v5 == 1u64;
        let v16: Rc<str> = if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" line)"); } LIT.with(|lit| lit.clone()) };
            v14.clone()
        } else {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" lines)"); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        };
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v16));
        lines_joined_20(v17.clone(), v4.clone())
    }
}
fn wait_run_23(mut v0: Rc<str>, mut v1: u64, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v11: Rc<str> = labeled_24(v0.clone());
    let v12: bool = name_listed_25(v8.clone(), v0.clone());
    let v14: u64 = if v12 {
        v4
    } else {
        let v13: u64 = v4.wrapping_add(1u64);
        v13
    };
    let v15: bool = &*v11 == "";
    let (mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v15 {
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job wait: no job named "); } LIT.with(|lit| lit.clone()) };
        let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v0));
        let v18: Rc<str> = lines_joined_20(v2.clone(), v17.clone());
        (v18.clone(), true, v14, v5, v6, v7, v8.clone(), v9.clone(), 1i32)
    } else {
        let v19: bool = v1 == 0u64;
        let v20: u64 = if v19 {
            86400000u64
        } else {
            v1
        };
        let v21: Rc<str> = settled_26(v11.clone(), v20);
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = summary_field_27(v21.clone(), v22.clone());
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exit"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = summary_field_27(v21.clone(), v24.clone());
        let v26: bool = name_listed_25(v9.clone(), v0.clone());
        let v29: Rc<str> = if v26 {
            notice_of_28(v0.clone(), v23.clone(), v25.clone())
        } else {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v28.clone()
        };
        let v30: bool = &*v23 == "timed-out";
        let v32: bool = if v30 {
            true
        } else {
            let v31: bool = &*v23 == "running";
            v31
        };
        let v34: bool = if v32 {
            true
        } else {
            let v33: bool = &*v23 == "pending";
            v33
        };
        let v39: i32 = if v34 {
            3i32
        } else {
            let v35: bool = &*v23 == "finished";
            let v37: bool = if v35 {
                let v36: bool = &*v25 == "0";
                v36
            } else {
                false
            };
            if v37 {
                0i32
            } else {
                1i32
            }
        };
        let v40: Rc<str> = settled_line_29(v0.clone(), v21.clone(), v1);
        let v41: Rc<str> = lines_joined_20(v2.clone(), v40.clone());
        let v42: Rc<str> = lines_joined_20(v41.clone(), v29.clone());
        let v43: Rc<str> = output_block_33(v0.clone(), v11.clone());
        let v44: Rc<str> = lines_joined_20(v42.clone(), v43.clone());
        let v45: bool = &*v23 == "finished";
        let v46: u64 = if v45 {
            1u64
        } else {
            0u64
        };
        let v47: u64 = if v30 {
            1u64
        } else {
            0u64
        };
        let v48: bool = &*v29 == "";
        let v49: u64 = if v48 {
            0u64
        } else {
            1u64
        };
        let v51: bool = if v3 {
            true
        } else {
            let v50: bool = v39 != 0i32;
            v50
        };
        let v52: u64 = v5.wrapping_add(v46);
        let v53: u64 = v6.wrapping_add(v47);
        let v54: u64 = v7.wrapping_add(v49);
        let v55: bool = v39 > v10;
        let v56: i32 = if v55 {
            v39
        } else {
            v10
        };
        (v44.clone(), v51, v14, v52, v53, v54, v8.clone(), v9.clone(), v56)
    };
    (v57.clone(), v58, v59, v60, v61, v62, v63.clone(), v64.clone(), v65)
}
fn stopped_text_40(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = kit_job_control::kit_job_stop_text(&*v0);
    v1
}
fn stop_run_39(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: Rc<str>, mut v8: Rc<str>, mut v9: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v10: Rc<str> = labeled_24(v0.clone());
    let v11: bool = &*v10 == "";
    let (mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job stop: no job named "); } LIT.with(|lit| lit.clone()) };
        let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v0));
        let v14: Rc<str> = lines_joined_20(v1.clone(), v13.clone());
        (v14.clone(), true, v3, v4, v5, v6, v7.clone(), v8.clone(), 1i32)
    } else {
        let v15: Rc<str> = stopped_text_40(v10.clone());
        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: "); } LIT.with(|lit| lit.clone()) };
        let v17: bool = v15.starts_with(&*v16);
        let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v17 {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v0));
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v19, v20)), v15));
            let v23: Rc<str> = lines_joined_20(v1.clone(), v22.clone());
            (v23.clone(), true, v3, v4, v5, v6, v7.clone(), v8.clone(), 1i32)
        } else {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job "); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v0));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  stopped ("); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("detail"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = summary_field_27(v15.clone(), v28.clone());
            let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v29));
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
            let v33: Rc<str> = lines_joined_20(v1.clone(), v32.clone());
            (v33.clone(), v2, v3, v4, v5, v6, v7.clone(), v8.clone(), v9)
        };
        (v34.clone(), v35, v36, v37, v38, v39, v40.clone(), v41.clone(), v42)
    };
    (v43.clone(), v44, v45, v46, v47, v48, v49.clone(), v50.clone(), v51)
}
fn step_run_15(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: Rc<str>, mut v9: Rc<str>, mut v10: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
        US1::US1_JobStart(v11, v12, v13, v14) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            let v14: bool = *v14;
            start_run_16(v0, v11, v12, v13, v14, v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
        }
        _ => {
            let (mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                US1::US1_JobWait(v24, v25) => {
                    let v24: Rc<str> = v24.clone();
                    let v25: u64 = *v25;
                    wait_run_23(v24, v25, v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
                }
                _ => {
                    let (mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
                        US1::US1_JobStop(v35) => {
                            let v35: Rc<str> = v35.clone();
                            stop_run_39(v35, v2.clone(), v3, v4, v5, v6, v7, v8.clone(), v9.clone(), v10)
                        }
                        _ => {
                            let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v1 {
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
fn jobs_run_14(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: i32) -> (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) {
    let v12: i32 = v1.len() as i32;
    let v13: bool = v2 < v12;
    let (mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = if v13 {
        let v14: US0 = job_parsed_4(v1.clone(), v2);
        let (mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v14 {
            US0::US0_JobParsed(v15) => {
                let v15: US1 = v15.clone();
                let v16: i32 = v2.wrapping_add(1i32);
                let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = step_run_15(v0.clone(), v15, v3.clone(), v4, v5, v6, v7, v8, v9.clone(), v10.clone(), v11);
                jobs_run_14(v0.clone(), v1.clone(), v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
            }
            _ => {
                let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = match &v14 {
                    US0::US0_JobRejected(v35) => {
                        let v35: Rc<str> = v35.clone();
                        let v36: i32 = v2.wrapping_add(1i32);
                        jobs_run_14(v0, v1, v36, v3.clone(), v4, v5, v6, v7, v8, v9.clone(), v10.clone(), v11)
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
fn method44(mut v0: Rc<RefCell<Mut0>>, mut v1: Rc<str>) -> () {
    let v2: Rc<str> = v0.borrow().l0.clone();
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v1));
    v0.borrow_mut().l0 = v3;
    ()
}
fn format_real_43(mut v0: i32) -> Rc<str> {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Mut0>> = Rc::new(RefCell::new(Mut0 { l0: v1.clone() }));
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}", v0));
    method44(v2.clone(), v3.clone());
    let mut v4: Rc<str> = v2.borrow().l0.clone();
    v4.clone()
}
fn code_text_42(mut v0: i32) -> Rc<str> {
    format_real_43(v0)
}
fn receipt_counts_41(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: Rc<str>, mut v7: Rc<str>, mut v8: i32) -> Rc<str> {
    let v9: Rc<str> = decimal_text_38(v2);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs "); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v9));
    let v12: bool = v3 > 0u64;
    let v17: Rc<str> = if v12 {
        let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  finished "); } LIT.with(|lit| lit.clone()) };
        let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v13));
        let v15: Rc<str> = decimal_text_38(v3);
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
        v16.clone()
    } else {
        v11.clone()
    };
    let v18: bool = v4 > 0u64;
    let v23: Rc<str> = if v18 {
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timed out "); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v19));
        let v21: Rc<str> = decimal_text_38(v4);
        let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
        v22.clone()
    } else {
        v17.clone()
    };
    let v24: bool = &*v7 != "";
    let v29: Rc<str> = if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  notifications "); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v23, v25));
        let v27: Rc<str> = decimal_text_38(v5);
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
        v28.clone()
    } else {
        v23.clone()
    };
    let v30: bool = v8 == 0i32;
    if v30 {
        v29.clone()
    } else {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit "); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v29, v31));
        let v33: Rc<str> = code_text_42(v8);
        let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v33));
        v34.clone()
    }
}
fn elapsed_text_46(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_48() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_47() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_48();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_45(mut v0: US3, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US3::US3_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US3::US3_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_46(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_47();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn job_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let mut v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = empty_job_tally_13();
        let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25): (Rc<str>, bool, u64, u64, u64, u64, Rc<str>, Rc<str>, i32) = jobs_run_14(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14.clone(), v15.clone(), v16);
        let v26: bool = &*v17 == "";
        if v26 {
            ()
        } else {
            println!("{}", v17);
            ()
        };
        if v18 {
            let v27: US3 = US3::US3_Failed;
            let v28: Rc<str> = receipt_counts_41(v17.clone(), v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25);
            receipt_line_45(v27.clone(), v28.clone(), v2);
            v25
        } else {
            let v29: US3 = US3::US3_Succeeded;
            let v30: Rc<str> = receipt_counts_41(v17.clone(), v18, v19, v20, v21, v22, v23.clone(), v24.clone(), v25);
            receipt_line_45(v29.clone(), v30.clone(), v2);
            0i32
        }
    } else {
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v5));
        println!("{}", v33);
        let v34: US3 = US3::US3_Failed;
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jobs 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_45(v34.clone(), v35.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        job_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_jobs_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
