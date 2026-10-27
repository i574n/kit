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
    US1_GitInit(Rc<str>),
    US1_GitCommit(Rc<str>, Rc<str>, Rc<str>),
    US1_GitHistory(Rc<str>, Rc<str>, Rc<str>),
    US1_GitStatus(Rc<str>),
    US1_GitDays(Rc<str>, Rc<str>),
    US1_Blank,
}
#[derive(Clone)]
enum US0 {
    US0_GitParsed(US1),
    US0_GitRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Succeeded,
    US2_Failed,
}
fn now_ms_1() -> u64 {
    let v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1.clone()
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_span_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; std::rc::Rc::<str>::from(fields.iter().take(until).skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")) };
    v4.clone()
}
fn body_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3.clone()
}
fn status_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = field_index_6(v0.clone(), v1, v2.clone());
    let v4: i32 = 3i32;
    let v5: Rc<str> = words_span_7(v0.clone(), v1, v4, v3);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected git status <repo>[, <repo>...] group by state"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v7.clone())
    } else {
        let v9: bool = v3 < 0i32;
        let v13: bool = if v9 {
            true
        } else {
            let v10: i32 = v3.wrapping_add(2i32);
            let v11: Rc<str> = body_at_8(v0.clone(), v1, v10);
            let v12: bool = &*v11 != "state";
            v12
        };
        if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected group by state"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v14.clone())
        } else {
            let v16: US1 = US1::US1_GitStatus(v5.clone());
            US0::US0_GitParsed(v16.clone())
        }
    }
}
fn quoted_at_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn commit_parsed_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let v5: i32 = 4i32;
    let v6: Rc<str> = body_at_8(v0.clone(), v1, v5);
    let v7: bool = &*v6 != "all";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected git commit <repo> all \"message\" [at <iso time>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8.clone())
    } else {
        let v10: i32 = 5i32;
        let v11: bool = quoted_at_10(v0.clone(), v1, v10);
        let v12: bool = v11 == false;
        if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected the quoted message after all"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v13.clone())
        } else {
            let v15: i32 = 5i32;
            let v16: Rc<str> = body_at_8(v0.clone(), v1, v15);
            let v17: bool = v4 < 0i32;
            let v21: Rc<str> = if v17 {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v18.clone()
            } else {
                let v19: i32 = v4.wrapping_add(1i32);
                body_at_8(v0.clone(), v1, v19)
            };
            let v22: US1 = US1::US1_GitCommit(v2.clone(), v16.clone(), v21.clone());
            US0::US0_GitParsed(v22.clone())
        }
    }
}
fn history_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let v5: i32 = 4i32;
    let v6: bool = quoted_at_10(v0.clone(), v1, v5);
    let v7: bool = v6 == false;
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected git history <repo> \"text\" in <file>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8.clone())
    } else {
        let v10: bool = v4 < 0i32;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected in <file>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v11.clone())
        } else {
            let v13: i32 = 4i32;
            let v14: Rc<str> = body_at_8(v0.clone(), v1, v13);
            let v15: i32 = v4.wrapping_add(1i32);
            let v16: Rc<str> = body_at_8(v0.clone(), v1, v15);
            let v17: US1 = US1::US1_GitHistory(v2.clone(), v14.clone(), v16.clone());
            US0::US0_GitParsed(v17.clone())
        }
    }
}
fn days_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("since"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3.clone());
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let v6: i32 = field_index_6(v0.clone(), v1, v5.clone());
    let v7: bool = v6 < 0i32;
    let v11: bool = if v7 {
        true
    } else {
        let v8: i32 = v6.wrapping_add(1i32);
        let v9: Rc<str> = body_at_8(v0.clone(), v1, v8);
        let v10: bool = &*v9 != "day";
        v10
    };
    if v11 {
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git log: expected git log <repo> [since <date>] count by day"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v12.clone())
    } else {
        let v14: bool = v4 < 0i32;
        let v18: Rc<str> = if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15.clone()
        } else {
            let v16: i32 = v4.wrapping_add(1i32);
            body_at_8(v0.clone(), v1, v16)
        };
        let v19: US1 = US1::US1_GitDays(v2.clone(), v18.clone());
        US0::US0_GitParsed(v19.clone())
    }
}
fn field_at_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3.clone()
}
fn git_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let v2: i32 = 0i32;
    let v3: Rc<str> = field_at_13(v0.clone(), v1, v2);
    let v4: i32 = 2i32;
    let v5: Rc<str> = body_at_8(v0.clone(), v1, v4);
    let v6: i32 = 3i32;
    let v7: Rc<str> = body_at_8(v0.clone(), v1, v6);
    let v8: bool = &*v3 == "";
    if v8 {
        let v9: US1 = US1::US1_Blank;
        US0::US0_GitParsed(v9.clone())
    } else {
        let v11: bool = &*v3 != "query";
        let v15: bool = if v11 {
            true
        } else {
            let v12: i32 = 1i32;
            let v13: Rc<str> = field_at_13(v0.clone(), v1, v12);
            let v14: bool = &*v13 != "git";
            v14
        };
        if v15 {
            let v16: i32 = 1i32;
            let v17: Rc<str> = field_at_13(v0.clone(), v1, v16);
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with git yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v18, &*v19, &*v19, &*v19].concat());
            US0::US0_GitRejected(v20.clone())
        } else {
            let v22: bool = &*v5 == "status";
            if v22 {
                status_parsed_5(v0.clone(), v1)
            } else {
                let v24: bool = &*v7 == "";
                if v24 {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git "); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a repo folder"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = std::rc::Rc::<str>::from([&*v25, &*v5, &*v26, &*v27, &*v27].concat());
                    US0::US0_GitRejected(v28.clone())
                } else {
                    let v30: bool = &*v5 == "init";
                    if v30 {
                        let v31: US1 = US1::US1_GitInit(v7.clone());
                        US0::US0_GitParsed(v31.clone())
                    } else {
                        let v33: bool = &*v5 == "commit";
                        if v33 {
                            commit_parsed_9(v0.clone(), v1, v7.clone())
                        } else {
                            let v35: bool = &*v5 == "history";
                            if v35 {
                                history_parsed_11(v0.clone(), v1, v7.clone())
                            } else {
                                let v37: bool = &*v5 == "log";
                                if v37 {
                                    days_parsed_12(v0.clone(), v1, v7.clone())
                                } else {
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git: unknown action "); } LIT.with(|lit| lit.clone()) };
                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected init, commit, history, status or log"); } LIT.with(|lit| lit.clone()) };
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v42: Rc<str> = std::rc::Rc::<str>::from([&*v39, &*v5, &*v40, &*v41, &*v41].concat());
                                    US0::US0_GitRejected(v42.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US0 = git_parsed_4(v0.clone(), v1);
            match &v4 {
                US0::US0_GitParsed(v6) => {
                    let v6: US1 = v6.clone();
                    let v7: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v7);
                    continue;
                }
                US0::US0_GitRejected(v5) => {
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
fn empty_git_tally_14() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0.clone(), false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn git_run_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { let mut command = std::process::Command::new("git"); command.current_dir(std::path::Path::new(&*v0).join(&*v1)).args(["-c", "user.name=kit", "-c"]).arg(format!("user.email=kit{}localhost", char::from(64u8))).args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args((v2).split(char::from(10u8))).stdin(std::process::Stdio::null()); if !(v3).is_empty() { command.env("GIT_AUTHOR_DATE", &*v3).env("GIT_COMMITTER_DATE", &*v3); } let mark = char::from(30u8); std::rc::Rc::<str>::from(match command.output() { Ok(output) => format!("{}{mark}{}{mark}{}", output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr).trim()), Err(error) => format!("1{mark}{mark}git: {error}") }) };
    v4
}
fn part_at_20(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn succeeded_19(mut v0: Rc<str>) -> bool {
    let v1: i32 = 0i32;
    let v2: Rc<str> = part_at_20(v0, v1);
    let v3: bool = &*v2 == "0";
    v3
}
fn lines_joined_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
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
            let v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn reported_21(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: bool) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v15: Rc<str> = lines_joined_22(v0.clone(), v13.clone());
    let v16: bool = v1 || v14;
    (v15.clone(), v16, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
fn failure_row_23(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed: "); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = 2i32;
    let v4: Rc<str> = part_at_20(v1.clone(), v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v4, &*v5, &*v5].concat());
    v6.clone()
}
fn init_run_17(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init\n-q\n-b\nmain\n"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = std::rc::Rc::<str>::from([&*v16, &*v1, &*v15, &*v15, &*v15].concat());
    let v18: Rc<str> = git_run_18(v0.clone(), v15.clone(), v17.clone(), v15.clone());
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git init "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = std::rc::Rc::<str>::from([&*v19, &*v1, &*v15, &*v15, &*v15].concat());
    let v21: bool = succeeded_19(v18.clone());
    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v21 {
        let v22: u64 = v4.wrapping_add(1u64);
        let v23: bool = false;
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2.clone(), v3, v22, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v20.clone(), v23);
        (v24.clone(), v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)
    } else {
        let v37: Rc<str> = failure_row_23(v20.clone(), v18.clone());
        let v38: bool = true;
        let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v37.clone(), v38);
        (v39.clone(), v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51)
    };
    (v52.clone(), v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn commit_run_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit "); } LIT.with(|lit| lit.clone()) };
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = std::rc::Rc::<str>::from([&*v17, &*v1, &*v18, &*v2, &*v19].concat());
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add\n-A"); } LIT.with(|lit| lit.clone()) };
    let v22: Rc<str> = git_run_18(v0.clone(), v1.clone(), v21.clone(), v19.clone());
    let v23: bool = succeeded_19(v22.clone());
    let (mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v23 {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit\n-q\n-m\n"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v2, &*v19, &*v19, &*v19].concat());
        let v26: Rc<str> = git_run_18(v0.clone(), v1.clone(), v25.clone(), v3.clone());
        let v27: bool = succeeded_19(v26.clone());
        let (mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v27 {
            let v28: u64 = v7.wrapping_add(1u64);
            let v29: bool = false;
            let (mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v28, v8, v9, v10, v11, v12, v13, v14, v15, v16, v20.clone(), v29);
            (v30.clone(), v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42)
        } else {
            let v43: Rc<str> = failure_row_23(v20.clone(), v26.clone());
            let v44: bool = true;
            let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v43.clone(), v44);
            (v45.clone(), v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57)
        };
        (v58.clone(), v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
    } else {
        let v71: Rc<str> = failure_row_23(v20.clone(), v22.clone());
        let v72: bool = true;
        let (mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v71.clone(), v72);
        (v73.clone(), v74, v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85)
    };
    (v86.clone(), v87, v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98)
}
fn history_run_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64) = kit_do_git_reports::kit_do_git_history(&*v0, &*v1, &*v2, &*v3);
    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v18 {
        let v21: bool = true;
        let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17.clone(), v21);
        (v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34)
    } else {
        let v35: u64 = v8.wrapping_add(v19);
        let v36: u64 = v9.wrapping_add(v20);
        let v37: u64 = v10.wrapping_add(1u64);
        let v38: bool = false;
        let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4, v5, v6, v7, v35, v36, v37, v11, v12, v13, v14, v15, v16, v17, v38);
        (v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51)
    };
    (v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn row_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v2, &*v1, &*v3, &*v3].concat());
    v4.clone()
}
fn go_29(mut v0: Rc<RefCell<Vec<Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> i32 {
    loop {
        let v4: bool = v2 >= v1;
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
fn line_count_28(mut v0: Rc<str>) -> u64 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v1).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v3: i32 = (v2.clone().borrow().len() as i32);
    let v4: bool = v3 > 0i32;
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
    let v13: i32 = go_29(v2.clone(), v10, v11, v12);
    let v14: u64 = (v13 as u64);
    v14
}
fn go_31(mut v0: i32, mut v1: Rc<RefCell<Vec<Rc<str>>>>, mut v2: i32, mut v3: i32, mut v4: i32, mut v5: Rc<str>) -> (i32, Rc<str>) {
    loop {
        let v6: bool = v3 >= v2;
        if v6 {
            return (v4, v5.clone());
        } else {
            let v7: Rc<str> = v1.clone().borrow()[v3 as usize].clone();
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
            (v0, v1, v2, v3, v4, v5) = (v0, v1.clone(), v2, v18, v23, v24.clone());
            continue;
        }
    }
}
fn line_at_30(mut v0: Rc<str>, mut v1: u64) -> Rc<str> {
    let v2: i32 = (v1 as i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(v0.split(&*v3).map(|x| Rc::<str>::from(x)).collect::<Vec<Rc<str>>>()));
    let v5: i32 = (v4.clone().borrow().len() as i32);
    let v6: bool = v5 > 0i32;
    let v10: bool = if v6 {
        let v7: i32 = v5.wrapping_sub(1i32);
        let v8: Rc<str> = v4.clone().borrow()[v7 as usize].clone();
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
    let (mut v16, mut v17): (i32, Rc<str>) = go_31(v2, v4.clone(), v12, v13, v14, v15.clone());
    v17.clone()
}
fn porcelain_paths_32(mut v0: Rc<str>, mut v1: bool) -> Rc<str> {
    let v2: Rc<str> = std::rc::Rc::<str>::from((v0).lines().filter(|line| line.len() > 3 && line.starts_with("??") == v1).map(|line| line[3..].trim_matches(char::from(34u8)).to_owned()).collect::<Vec<_>>().join("
"));
    v2.clone()
}
fn decimal_text_34(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn state_row_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<str> {
    let v3: bool = &*v2 == "";
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4.clone()
    } else {
        let v5: u64 = line_count_28(v2.clone());
        let v6: Rc<str> = decimal_text_34(v5);
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(", "); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = std::rc::Rc::<str>::from(v2.replace(&*v7, &v8));
        let v10: Rc<str> = row_26(v6.clone(), v9.clone());
        let v11: Rc<str> = row_26(v1.clone(), v10.clone());
        row_26(v0.clone(), v11.clone())
    }
}
fn aligned_35(mut v0: Rc<str>) -> Rc<str> {
    let v1: Rc<str> = { let separator = char::from(31u8); let rows = (&*v0).lines().collect::<Vec<_>>(); let mut widths: Vec<usize> = Vec::new(); for row in &rows { if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); for (index, cell) in cells[..cells.len() - 1].iter().enumerate() { let length = cell.chars().count(); if widths.len() <= index { widths.push(length); } else if widths[index] < length { widths[index] = length; } } } } let shown = rows.iter().map(|row| if row.contains(separator) { let cells = row.split(separator).collect::<Vec<_>>(); let last = cells.len() - 1; cells.iter().enumerate().map(|(index, cell)| if index == last { cell.to_string() } else { format!("{:<width$}  ", cell, width = widths[index]) }).collect::<String>() } else { row.to_string() }).collect::<Vec<_>>(); std::rc::Rc::<str>::from(shown.join("
")) };
    v1.clone()
}
fn repos_status_27(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: u64, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v17: u64 = line_count_28(v1.clone());
    let v18: bool = v2 < v17;
    let (mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v18 {
        let v19: Rc<str> = line_at_30(v1.clone(), v2);
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status\n--porcelain\n--untracked-files=all"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = git_run_18(v0.clone(), v19.clone(), v20.clone(), v21.clone());
        let v23: bool = succeeded_19(v22.clone());
        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v23 {
            let v24: i32 = 1i32;
            let v25: Rc<str> = part_at_20(v22.clone(), v24);
            let v26: bool = false;
            let v27: Rc<str> = porcelain_paths_32(v25.clone(), v26);
            let v28: bool = true;
            let v29: Rc<str> = porcelain_paths_32(v25.clone(), v28);
            let v30: bool = &*v25 == "";
            let v40: Rc<str> = if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("clean"); } LIT.with(|lit| lit.clone()) };
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = row_26(v31.clone(), v32.clone());
                row_26(v19.clone(), v33.clone())
            } else {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("modified"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<str> = state_row_33(v19.clone(), v35.clone(), v27.clone());
                let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("untracked"); } LIT.with(|lit| lit.clone()) };
                let v38: Rc<str> = state_row_33(v19.clone(), v37.clone(), v29.clone());
                lines_joined_22(v36.clone(), v38.clone())
            };
            let v41: u64 = v11.wrapping_add(1u64);
            let v42: u64 = line_count_28(v27.clone());
            let v43: u64 = v12.wrapping_add(v42);
            let v44: u64 = line_count_28(v29.clone());
            let v45: u64 = v13.wrapping_add(v44);
            let v46: u64 = v2.wrapping_add(1u64);
            let v47: Rc<str> = lines_joined_22(v3.clone(), v40.clone());
            let (mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = repos_status_27(v0.clone(), v1.clone(), v46, v47.clone(), v4.clone(), v5, v6, v7, v8, v9, v10, v41, v43, v45, v14, v15, v16);
            (v48.clone(), v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
        } else {
            let v61: u64 = v2.wrapping_add(1u64);
            let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            let v64: i32 = 2i32;
            let v65: Rc<str> = part_at_20(v22.clone(), v64);
            let v66: Rc<str> = row_26(v63.clone(), v65.clone());
            let v67: Rc<str> = row_26(v62.clone(), v66.clone());
            let v68: Rc<str> = row_26(v19.clone(), v67.clone());
            let v69: Rc<str> = lines_joined_22(v3.clone(), v68.clone());
            let v70: bool = true;
            let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = repos_status_27(v0.clone(), v1.clone(), v61, v69.clone(), v4.clone(), v70, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
            (v71.clone(), v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
        };
        (v84.clone(), v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95, v96)
    } else {
        let v97: u64 = v14.wrapping_add(1u64);
        let v98: Rc<str> = aligned_35(v3.clone());
        let v99: bool = false;
        let (mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4.clone(), v5, v6, v7, v8, v9, v10, v11, v12, v13, v97, v15, v16, v98.clone(), v99);
        (v100.clone(), v101, v102, v103, v104, v105, v106, v107, v108, v109, v110, v111, v112)
    };
    (v113.clone(), v114, v115, v116, v117, v118, v119, v120, v121, v122, v123, v124, v125)
}
fn days_run_36(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64) = kit_do_git_reports::kit_do_git_days(&*v0, &*v1, &*v2);
    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v20: bool = true;
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16.clone(), v20);
        (v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
    } else {
        let v34: u64 = v7.wrapping_add(v18);
        let v35: u64 = v14.wrapping_add(v19);
        let v36: u64 = v15.wrapping_add(1u64);
        let v37: bool = false;
        let (mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v3, v4, v5, v6, v34, v8, v9, v10, v11, v12, v13, v35, v36, v16, v37);
        (v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50)
    };
    (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
}
fn step_run_16(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v163, mut v164, mut v165, mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
        US1::US1_GitInit(v15) => {
            let v15: Rc<str> = v15.clone();
            init_run_17(v0.clone(), v15.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
        }
        _ => {
            let (mut v150, mut v151, mut v152, mut v153, mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                US1::US1_GitCommit(v29, v30, v31) => {
                    let v29: Rc<str> = v29.clone();
                    let v30: Rc<str> = v30.clone();
                    let v31: Rc<str> = v31.clone();
                    commit_run_24(v0.clone(), v29.clone(), v30.clone(), v31.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                }
                _ => {
                    let (mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                        US1::US1_GitHistory(v45, v46, v47) => {
                            let v45: Rc<str> = v45.clone();
                            let v46: Rc<str> = v46.clone();
                            let v47: Rc<str> = v47.clone();
                            history_run_25(v0.clone(), v45.clone(), v46.clone(), v47.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                        }
                        _ => {
                            let (mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                US1::US1_GitStatus(v61) => {
                                    let v61: Rc<str> = v61.clone();
                                    let v62: u64 = 0u64;
                                    let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo"); } LIT.with(|lit| lit.clone()) };
                                    let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("state"); } LIT.with(|lit| lit.clone()) };
                                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
                                    let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("paths"); } LIT.with(|lit| lit.clone()) };
                                    let v67: Rc<str> = row_26(v65.clone(), v66.clone());
                                    let v68: Rc<str> = row_26(v64.clone(), v67.clone());
                                    let v69: Rc<str> = row_26(v63.clone(), v68.clone());
                                    repos_status_27(v0.clone(), v61.clone(), v62, v69.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                }
                                _ => {
                                    let (mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                        US1::US1_GitDays(v83, v84) => {
                                            let v83: Rc<str> = v83.clone();
                                            let v84: Rc<str> = v84.clone();
                                            days_run_36(v0.clone(), v83.clone(), v84.clone(), v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                        }
                                        _ => {
                                            let (mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                                US1::US1_Blank => {
                                                    (v2.clone(), v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v98.clone(), v99, v100, v101, v102, v103, v104, v105, v106, v107, v108, v109, v110)
                                        }
                                    };
                                    (v111.clone(), v112, v113, v114, v115, v116, v117, v118, v119, v120, v121, v122, v123)
                                }
                            };
                            (v124.clone(), v125, v126, v127, v128, v129, v130, v131, v132, v133, v134, v135, v136)
                        }
                    };
                    (v137.clone(), v138, v139, v140, v141, v142, v143, v144, v145, v146, v147, v148, v149)
                }
            };
            (v150.clone(), v151, v152, v153, v154, v155, v156, v157, v158, v159, v160, v161, v162)
        }
    };
    (v163.clone(), v164, v165, v166, v167, v168, v169, v170, v171, v172, v173, v174, v175)
}
fn gits_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: i32 = v1.len() as i32;
    let v17: bool = v2 < v16;
    let (mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: US0 = git_parsed_4(v1.clone(), v2);
        let (mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_GitParsed(v19) => {
                let v19: US1 = v19.clone();
                let v20: i32 = v2.wrapping_add(1i32);
                let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_16(v0.clone(), v19, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                gits_run_15(v0.clone(), v1.clone(), v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33)
            }
            _ => {
                let (mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
                    US0::US0_GitRejected(v47) => {
                        let v47: Rc<str> = v47.clone();
                        let v48: i32 = v2.wrapping_add(1i32);
                        gits_run_15(v0, v1, v48, v3.clone(), v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v62, v63, v64, v65, v66, v67, v68, v69, v70, v71, v72, v73, v74)
            }
        };
        (v75, v76, v77, v78, v79, v80, v81, v82, v83, v84, v85, v86, v87)
    } else {
        (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v88, v89, v90, v91, v92, v93, v94, v95, v96, v97, v98, v99, v100)
}
fn segments_joined_38(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let v3: bool = &*v1 == "";
        if v3 {
            v0.clone()
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v6: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v4, &*v1, &*v5, &*v5].concat());
            v6.clone()
        }
    }
}
fn receipt_counts_37(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let v13: u64 = v2.wrapping_add(v3);
    let v14: bool = v13 > 0u64;
    let v22: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("initialized "); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = decimal_text_34(v2);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  committed "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = decimal_text_34(v3);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = std::rc::Rc::<str>::from([&*v15, &*v16, &*v17, &*v18, &*v19].concat());
        v20.clone()
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v21.clone()
    };
    let v23: bool = v6 > 0u64;
    let v31: Rc<str> = if v23 {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = decimal_text_34(v4);
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  matches "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = decimal_text_34(v5);
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = std::rc::Rc::<str>::from([&*v24, &*v25, &*v26, &*v27, &*v28].concat());
        v29.clone()
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30.clone()
    };
    let v32: bool = v10 > 0u64;
    let v43: Rc<str> = if v32 {
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repos "); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = decimal_text_34(v7);
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  changed "); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = decimal_text_34(v8);
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  untracked "); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = decimal_text_34(v9);
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = std::rc::Rc::<str>::from([&*v37, &*v38, &*v39, &*v39, &*v39].concat());
        let v41: Rc<str> = std::rc::Rc::<str>::from([&*v33, &*v34, &*v35, &*v36, &*v40].concat());
        v41.clone()
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42.clone()
    };
    let v44: bool = v12 > 0u64;
    let v46: bool = if v44 {
        let v45: bool = v6 == 0u64;
        v45
    } else {
        false
    };
    let v54: Rc<str> = if v46 {
        let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits "); } LIT.with(|lit| lit.clone()) };
        let v48: Rc<str> = decimal_text_34(v4);
        let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  days "); } LIT.with(|lit| lit.clone()) };
        let v50: Rc<str> = decimal_text_34(v11);
        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v52: Rc<str> = std::rc::Rc::<str>::from([&*v47, &*v48, &*v49, &*v50, &*v51].concat());
        v52.clone()
    } else {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v53.clone()
    };
    let v55: Rc<str> = segments_joined_38(v22.clone(), v31.clone());
    let v56: Rc<str> = segments_joined_38(v43.clone(), v54.clone());
    segments_joined_38(v55.clone(), v56.clone())
}
fn elapsed_text_40(mut v0: u64) -> Rc<str> {
    let v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1.clone()
}
fn local_offset_minutes_42() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_41() -> Rc<str> {
    let v0: i64 = local_offset_minutes_42();
    let v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1.clone()
}
fn receipt_line_39(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match &v0 {
        US2::US2_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4.clone()
        }
        US2::US2_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3.clone()
        }
    };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v6, v7)), v1)), v7));
    let v11: Rc<str> = elapsed_text_40(v2);
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v10, v11)), v7));
    let v14: Rc<str> = clock_text_41();
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
    println!("{}", v15);
    ()
}
fn git_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let v2: u64 = now_ms_1();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1.clone());
    let v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_git_tally_14();
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = gits_run_15(v0.clone(), v3.clone(), v7, v8.clone(), v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
        let v34: bool = &*v21 == "";
        if v34 {
            ()
        } else {
            println!("{}", &*v21);
            ()
        };
        if v22 {
            let v35: US2 = US2::US2_Failed;
            let v36: Rc<str> = receipt_counts_37(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_39(v35.clone(), v36.clone(), v2);
            1i32
        } else {
            let v37: US2 = US2::US2_Succeeded;
            let v38: Rc<str> = receipt_counts_37(v21.clone(), v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_39(v37.clone(), v38.clone(), v2);
            0i32
        }
    } else {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = std::rc::Rc::<str>::from([&*v40, &*v5, &*v41, &*v41, &*v41].concat());
        println!("{}", &*v42);
        let v43: US2 = US2::US2_Failed;
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repos 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_39(v43.clone(), v44.clone(), v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        git_program_run_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_git_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
