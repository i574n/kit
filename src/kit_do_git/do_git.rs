#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
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
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn words_span_7(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: i32) -> Rc<str> {
    let v4: Rc<str> = { let fields = v0.get(v1 as usize).cloned().unwrap_or_default(); let until = if v3 < 0 { fields.len() } else { (v3 as usize).min(fields.len()) }; std::rc::Rc::<str>::from(fields.iter().take(until).skip(v2 as usize).filter(|raw| raw.len() > 1).flat_map(|raw| raw[1..].split(|c: char| c == char::from(44u8) || c.is_whitespace()).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>()).collect::<Vec<_>>().join("
")) };
    v4
}
fn body_at_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).filter(|field| !field.is_empty()).map(|field| std::rc::Rc::<str>::from(&field[1..])).unwrap_or_default();
    v3
}
fn status_parsed_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("group"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = field_index_6(v0.clone(), v1, v2);
    let v4: i32 = 3i32;
    let v5: Rc<str> = words_span_7(v0.clone(), v1, v4, v3);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected git status <repo>[, <repo>...] group by state"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v7)
    } else {
        let v9: bool = v3 < 0i32;
        let v13: bool = if v9 {
            true
        } else {
            let v10: i32 = v3.wrapping_add(2i32);
            let v11: Rc<str> = body_at_8(v0, v1, v10);
            let v12: bool = &*v11 != "state";
            v12
        };
        if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git status: expected group by state"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v14)
        } else {
            let v16: US1 = US1::US1_GitStatus(v5);
            US0::US0_GitParsed(v16)
        }
    }
}
fn quoted_at_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> bool {
    let v3: bool = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).map(|field| field.starts_with(char::from(113u8))).unwrap_or(false);
    v3
}
fn commit_parsed_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("at"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3);
    let v5: i32 = 4i32;
    let v6: Rc<str> = body_at_8(v0.clone(), v1, v5);
    let v7: bool = &*v6 != "all";
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected git commit <repo> all \"message\" [at <iso time>]"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8)
    } else {
        let v10: i32 = 5i32;
        let v11: bool = quoted_at_10(v0.clone(), v1, v10);
        let v12: bool = v11 == false;
        if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit: expected the quoted message after all"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v13)
        } else {
            let v15: i32 = 5i32;
            let v16: Rc<str> = body_at_8(v0.clone(), v1, v15);
            let v17: bool = v4 < 0i32;
            let v21: Rc<str> = if v17 {
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v18
            } else {
                let v19: i32 = v4.wrapping_add(1i32);
                body_at_8(v0, v1, v19)
            };
            let v22: US1 = US1::US1_GitCommit(v2, v16, v21);
            US0::US0_GitParsed(v22)
        }
    }
}
fn history_parsed_11(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("in"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3);
    let v5: i32 = 4i32;
    let v6: bool = quoted_at_10(v0.clone(), v1, v5);
    let v7: bool = v6 == false;
    if v7 {
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected git history <repo> \"text\" in <file>"); } LIT.with(|lit| lit.clone()) };
        US0::US0_GitRejected(v8)
    } else {
        let v10: bool = v4 < 0i32;
        if v10 {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git history: expected in <file>"); } LIT.with(|lit| lit.clone()) };
            US0::US0_GitRejected(v11)
        } else {
            let v13: i32 = 4i32;
            let v14: Rc<str> = body_at_8(v0.clone(), v1, v13);
            let v15: i32 = v4.wrapping_add(1i32);
            let v16: Rc<str> = body_at_8(v0, v1, v15);
            let v17: US1 = US1::US1_GitHistory(v2, v14, v16);
            US0::US0_GitParsed(v17)
        }
    }
}
fn days_parsed_12(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("since"); } LIT.with(|lit| lit.clone()) };
    let v4: i32 = field_index_6(v0.clone(), v1, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("by"); } LIT.with(|lit| lit.clone()) };
    let v6: i32 = field_index_6(v0.clone(), v1, v5);
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
        US0::US0_GitRejected(v12)
    } else {
        let v14: bool = v4 < 0i32;
        let v18: Rc<str> = if v14 {
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v15
        } else {
            let v16: i32 = v4.wrapping_add(1i32);
            body_at_8(v0, v1, v16)
        };
        let v19: US1 = US1::US1_GitDays(v2, v18);
        US0::US0_GitParsed(v19)
    }
}
fn field_at_13(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn concat5_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn git_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 0i32;
    let v3: Rc<str> = field_at_13(v0.clone(), v1, v2);
    let v4: i32 = 2i32;
    let v5: Rc<str> = body_at_8(v0.clone(), v1, v4);
    let v6: i32 = 3i32;
    let v7: Rc<str> = body_at_8(v0.clone(), v1, v6);
    let v8: bool = &*v3 == "";
    if v8 {
        let v9: US1 = US1::US1_Blank;
        US0::US0_GitParsed(v9)
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
            let v17: Rc<str> = field_at_13(v0, v1, v16);
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" cannot share one program with git yet; run it as its own call"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = concat5_14(v17, v18, v19.clone(), v19.clone(), v19);
            US0::US0_GitRejected(v20)
        } else {
            let v22: bool = &*v5 == "status";
            if v22 {
                status_parsed_5(v0, v1)
            } else {
                let v24: bool = &*v7 == "";
                if v24 {
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git "); } LIT.with(|lit| lit.clone()) };
                    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a repo folder"); } LIT.with(|lit| lit.clone()) };
                    let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v28: Rc<str> = concat5_14(v25, v5, v26, v27.clone(), v27);
                    US0::US0_GitRejected(v28)
                } else {
                    let v30: bool = &*v5 == "init";
                    if v30 {
                        let v31: US1 = US1::US1_GitInit(v7);
                        US0::US0_GitParsed(v31)
                    } else {
                        let v33: bool = &*v5 == "commit";
                        if v33 {
                            commit_parsed_9(v0, v1, v7)
                        } else {
                            let v35: bool = &*v5 == "history";
                            if v35 {
                                history_parsed_11(v0, v1, v7)
                            } else {
                                let v37: bool = &*v5 == "log";
                                if v37 {
                                    days_parsed_12(v0, v1, v7)
                                } else {
                                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git: unknown action "); } LIT.with(|lit| lit.clone()) };
                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; expected init, commit, history, status or log"); } LIT.with(|lit| lit.clone()) };
                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                    let v42: Rc<str> = concat5_14(v39, v5, v40, v41.clone(), v41);
                                    US0::US0_GitRejected(v42)
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
        let mut v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US0 = git_parsed_4(v0.clone(), v1);
            match v4 {
                US0::US0_GitParsed(v6) => {
                    let v6: US1 = v6;
                    let v7: i32 = v1.wrapping_add(1i32);
                    v1 = v7;
                    continue;
                }
                US0::US0_GitRejected(v5) => {
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
fn empty_git_tally_15() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn created_before_17(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> bool {
    loop {
        let v4: bool = v2 >= v3;
        if v4 {
            return false;
        } else {
            let v5: US0 = git_parsed_4(v0.clone(), v2);
            match v5 {
                US0::US0_GitParsed(v6) => {
                    let v6: US1 = v6;
                    match v6 {
                        US1::US1_GitInit(v7) => {
                            let v7: Rc<str> = v7;
                            let v8: bool = &*v7 == &*v1;
                            if v8 {
                                return true;
                            } else {
                                let v9: i32 = v2.wrapping_add(1i32);
                                v2 = v9;
                                continue;
                            }
                        }
                        _ => {
                            let v12: i32 = v2.wrapping_add(1i32);
                            v2 = v12;
                            continue;
                        }
                    }
                }
                _ => {
                    let v15: i32 = v2.wrapping_add(1i32);
                    v2 = v15;
                    continue;
                }
            }
        }
    }
}
fn get_environment_variable_20(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = Rc::<str>::from(std::env::var(&*v0).unwrap_or_default());
    v1
}
fn agent_session_19() -> bool {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("AI_AGENT"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = get_environment_variable_20(v0);
    let mut v2: bool = &*v1 != "";
    if v2 {
        true
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("CLAUDECODE"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = get_environment_variable_20(v3);
        let v5: bool = &*v4 != "";
        v5
    }
}
fn lines_joined_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "";
    if v2 {
        v0
    } else {
        let v3: bool = &*v0 == "";
        if v3 {
            v1
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v0, v4, v1, v5.clone(), v5)
        }
    }
}
fn reported_21(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: Rc<str>, mut v14: bool) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v15: Rc<str> = lines_joined_22(v0, v13);
    let v16: bool = v1 || v14;
    (v15, v16, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
fn git_run_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<str> {
    let v4: Rc<str> = { let mut command = std::process::Command::new("git"); command.current_dir(std::path::Path::new(&*v0).join(&*v1)).args(["-c", "user.name=kit", "-c"]).arg(format!("user.email=kit{}localhost", char::from(64u8))).args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args((v2).split(char::from(10u8))).stdin(std::process::Stdio::null()); if !(v3).is_empty() { command.env("GIT_AUTHOR_DATE", &*v3).env("GIT_COMMITTER_DATE", &*v3); } let mark = char::from(30u8); std::rc::Rc::<str>::from(match command.output() { Ok(output) => format!("{}{mark}{}{mark}{}", output.status.code().unwrap_or(1), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr).trim()), Err(error) => format!("1{mark}{mark}git: {error}") }) };
    v4
}
fn part_at_25(mut v0: Rc<str>, mut v1: i32) -> Rc<str> {
    let mut v2: Rc<str> = std::rc::Rc::<str>::from(v0.split(char::from(30u8)).nth(v1 as usize).unwrap_or(""));
    v2
}
fn succeeded_24(mut v0: Rc<str>) -> bool {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<str> = part_at_25(v0, v1);
    let v3: bool = &*v2 == "0";
    v3
}
fn failure_row_26(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  failed: "); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = 2i32;
    let v4: Rc<str> = part_at_25(v1, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_14(v0, v2, v4, v5.clone(), v5)
}
fn commit_run_18(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: bool, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64, mut v17: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git commit "); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = concat5_14(v18, v1.clone(), v19, v2.clone(), v20.clone());
    let v22: bool = agent_session_19();
    let v24: bool = if v22 {
        let v23: bool = v4 == false;
        v23
    } else {
        false
    };
    let (mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v24 {
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  refused: an agent session commits only to a repo this program created with git init; "); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("the user reviews, stages and commits their own repos"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = concat5_14(v21, v25, v26, v20.clone(), v20.clone());
        let v28: bool = true;
        let (mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v27, v28);
        (v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41)
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add\n-A"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = git_run_23(v0.clone(), v1.clone(), v42, v20.clone());
        let v44: bool = succeeded_24(v43.clone());
        let (mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v44 {
            let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commit\n-q\n-m\n"); } LIT.with(|lit| lit.clone()) };
            let v46: Rc<str> = concat5_14(v45, v2, v20.clone(), v20.clone(), v20);
            let v47: Rc<str> = git_run_23(v0, v1, v46, v3);
            let v48: bool = succeeded_24(v47.clone());
            let (mut v79, mut v80, mut v81, mut v82, mut v83, mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v48 {
                let v49: u64 = v8.wrapping_add(1u64);
                let v50: bool = false;
                let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v5, v6, v7, v49, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v50);
                (v51, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63)
            } else {
                let v64: Rc<str> = failure_row_26(v21, v47);
                let v65: bool = true;
                let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v64, v65);
                (v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77, v78)
            };
            (v79, v80, v81, v82, v83, v84, v85, v86, v87, v88, v89, v90, v91)
        } else {
            let v92: Rc<str> = failure_row_26(v21, v43);
            let v93: bool = true;
            let (mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v92, v93);
            (v94, v95, v96, v97, v98, v99, v100, v101, v102, v103, v104, v105, v106)
        };
        (v107, v108, v109, v110, v111, v112, v113, v114, v115, v116, v117, v118, v119)
    };
    (v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132)
}
fn init_run_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init\n-q\n-b\nmain\n"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = concat5_14(v16, v1.clone(), v15.clone(), v15.clone(), v15.clone());
    let v18: Rc<str> = git_run_23(v0, v15.clone(), v17, v15.clone());
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("git init "); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = concat5_14(v19, v1, v15.clone(), v15.clone(), v15);
    let v21: bool = succeeded_24(v18.clone());
    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v21 {
        let v22: u64 = v4.wrapping_add(1u64);
        let v23: bool = false;
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2, v3, v22, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v20, v23);
        (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)
    } else {
        let v37: Rc<str> = failure_row_26(v20, v18);
        let v38: bool = true;
        let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v37, v38);
        (v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51)
    };
    (v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64)
}
fn history_run_29(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64, mut v16: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64) = kit_do_git_reports::kit_do_git_history(&*v0, &*v1, &*v2, &*v3);
    let (mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v18 {
        let v21: bool = true;
        let (mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21);
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
fn repos_status_30(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v15, mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64, u64) = kit_do_git_reports::kit_do_git_status(&*v0, &*v1);
    let v20: u64 = v9.wrapping_add(v17);
    let v21: u64 = v10.wrapping_add(v18);
    let v22: u64 = v11.wrapping_add(v19);
    let v23: u64 = v12.wrapping_add(1u64);
    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v2, v3, v4, v5, v6, v7, v8, v20, v21, v22, v23, v13, v14, v15, v16);
    (v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36)
}
fn days_run_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v16, mut v17, mut v18, mut v19): (Rc<str>, bool, u64, u64) = kit_do_git_reports::kit_do_git_days(&*v0, &*v1, &*v2);
    let (mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v20: bool = true;
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = reported_21(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v20);
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
fn step_run_27(mut v0: Rc<str>, mut v1: US1, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let (mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165, mut v166, mut v167, mut v168): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
        US1::US1_GitInit(v15) => {
            let v15: Rc<str> = v15.clone();
            init_run_28(v0, v15, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
        }
        _ => {
            let (mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153, mut v154, mut v155): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                US1::US1_GitCommit(v29, v30, v31) => {
                    let v29: Rc<str> = v29.clone();
                    let v30: Rc<str> = v30.clone();
                    let v31: Rc<str> = v31.clone();
                    let v32: bool = false;
                    commit_run_18(v0, v29, v30, v31, v32, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                }
                _ => {
                    let (mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                        US1::US1_GitHistory(v46, v47, v48) => {
                            let v46: Rc<str> = v46.clone();
                            let v47: Rc<str> = v47.clone();
                            let v48: Rc<str> = v48.clone();
                            history_run_29(v0, v46, v47, v48, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                        }
                        _ => {
                            let (mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                US1::US1_GitStatus(v62) => {
                                    let v62: Rc<str> = v62.clone();
                                    repos_status_30(v0, v62, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                }
                                _ => {
                                    let (mut v104, mut v105, mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v1 {
                                        US1::US1_GitDays(v76, v77) => {
                                            let v76: Rc<str> = v76.clone();
                                            let v77: Rc<str> = v77.clone();
                                            days_run_31(v0, v76, v77, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                        }
                                        _ => {
                                            let (mut v91, mut v92, mut v93, mut v94, mut v95, mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v1 {
                                                US1::US1_Blank => {
                                                    (v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                                                }
                                                _ => unreachable!(),
                                            };
                                            (v91, v92, v93, v94, v95, v96, v97, v98, v99, v100, v101, v102, v103)
                                        }
                                    };
                                    (v104, v105, v106, v107, v108, v109, v110, v111, v112, v113, v114, v115, v116)
                                }
                            };
                            (v117, v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129)
                        }
                    };
                    (v130, v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141, v142)
                }
            };
            (v143, v144, v145, v146, v147, v148, v149, v150, v151, v152, v153, v154, v155)
        }
    };
    (v156, v157, v158, v159, v160, v161, v162, v163, v164, v165, v166, v167, v168)
}
fn gits_run_16(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v2: i32, mut v3: Rc<str>, mut v4: bool, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: i32 = v1.len() as i32;
    let v17: bool = v2 < v16;
    let (mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141, mut v142, mut v143, mut v144, mut v145): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: US0 = git_parsed_4(v1.clone(), v2);
        let (mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129, mut v130, mut v131, mut v132): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_GitParsed(v19) => {
                let v19: US1 = v19.clone();
                match &v19 {
                    US1::US1_GitCommit(v20, v21, v22) => {
                        let v20: Rc<str> = v20.clone();
                        let v21: Rc<str> = v21.clone();
                        let v22: Rc<str> = v22.clone();
                        let v23: i32 = 1i32;
                        let v24: bool = created_before_17(v1.clone(), v20.clone(), v23, v2);
                        let v25: i32 = v2.wrapping_add(1i32);
                        let (mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = commit_run_18(v0.clone(), v20, v21, v22, v24, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                        gits_run_16(v0, v1, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v36, v37, v38)
                    }
                    _ => {
                        let v52: i32 = v2.wrapping_add(1i32);
                        let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = step_run_27(v0.clone(), v19, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15);
                        let (mut v66, mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = gits_run_16(v0, v1, v52, v53, v54, v55, v56, v57, v58, v59, v60, v61, v62, v63, v64, v65);
                        (v66, v67, v68, v69, v70, v71, v72, v73, v74, v75, v76, v77, v78)
                    }
                }
            }
            _ => {
                let (mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117, mut v118, mut v119): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v18 {
                    US0::US0_GitRejected(v92) => {
                        let v92: Rc<str> = v92;
                        let v93: i32 = v2.wrapping_add(1i32);
                        gits_run_16(v0, v1, v93, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v107, v108, v109, v110, v111, v112, v113, v114, v115, v116, v117, v118, v119)
            }
        };
        (v120, v121, v122, v123, v124, v125, v126, v127, v128, v129, v130, v131, v132)
    } else {
        (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v133, v134, v135, v136, v137, v138, v139, v140, v141, v142, v143, v144, v145)
}
fn segments_joined_33(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: bool = &*v1 == "";
        if v3 {
            v0
        } else {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) };
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            concat5_14(v0, v4, v1, v5.clone(), v5)
        }
    }
}
fn decimal_text_34(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn receipt_counts_32(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let v13: u64 = v2.wrapping_add(v3);
    let v14: bool = v13 > 0u64;
    let v22: Rc<str> = if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("initialized "); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = decimal_text_34(v2);
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  committed "); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = decimal_text_34(v3);
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_14(v15, v16, v17, v18, v19)
    } else {
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v21
    };
    let v23: bool = v6 > 0u64;
    let v31: Rc<str> = if v23 {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("commits "); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = decimal_text_34(v4);
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  matches "); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = decimal_text_34(v5);
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_14(v24, v25, v26, v27, v28)
    } else {
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v30
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
        let v40: Rc<str> = concat5_14(v37, v38, v39.clone(), v39.clone(), v39);
        concat5_14(v33, v34, v35, v36, v40)
    } else {
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v42
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
        concat5_14(v47, v48, v49, v50, v51)
    } else {
        let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v53
    };
    let v55: Rc<str> = segments_joined_33(v22, v31);
    let v56: Rc<str> = segments_joined_33(v43, v54);
    segments_joined_33(v55, v56)
}
fn elapsed_text_36(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_38() -> i64 {
    let v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_37() -> Rc<str> {
    let v0: i64 = local_offset_minutes_38();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_35(mut v0: US2, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US2::US2_Failed => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_Succeeded => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_36(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_37();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn git_program_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let v4: i32 = 1i32;
    let v5: Rc<str> = first_rejection_3(v3.clone(), v4);
    let v6: bool = &*v5 == "";
    if v6 {
        let v7: i32 = 1i32;
        let (mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_git_tally_15();
        let (mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = gits_run_16(v0, v3, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20);
        let v34: bool = &*v21 == "";
        if v34 {
            ()
        } else {
            println!("{}", v21);
            ()
        };
        if v22 {
            let v35: US2 = US2::US2_Failed;
            let v36: Rc<str> = receipt_counts_32(&v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_35(v35, v36, v2);
            1i32
        } else {
            let v37: US2 = US2::US2_Succeeded;
            let v38: Rc<str> = receipt_counts_32(&v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33);
            receipt_line_35(v37, v38, v2);
            0i32
        }
    } else {
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = concat5_14(v40, v5, v41.clone(), v41.clone(), v41);
        println!("{}", v42);
        let v43: US2 = US2::US2_Failed;
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repos 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_35(v43, v44, v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        git_program_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_git_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
