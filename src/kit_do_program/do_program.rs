#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_model::{write_mode, expectation, replace_count, rule_matcher, rule_list, step, steps};
#[derive(Clone)]
enum US0 {
    US0_Parsed(Rc<steps>, i32),
    US0_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Word(Rc<str>),
    US6_Quoted(Rc<str>),
    US6_StatementEnd,
    US6_Unterminated(Rc<str>),
}
#[derive(Clone)]
enum US7 {
    US7_Parsed(step, i32),
    US7_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US8 {
    US8_Parsed(Rc<str>, write_mode, expectation, Rc<str>, Rc<rule_list>, i32),
    US8_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_Parsed(Rc<rule_list>, i32),
    US9_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US10 {
    US10_Parsed(Rc<str>, Rc<str>, replace_count, rule_matcher, i32),
    US10_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US11 {
    US11_Parsed(replace_count, i32),
    US11_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US12 {
    US12_Parsed(std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>, i32),
    US12_Rejected(Rc<str>),
}
#[derive(Clone)]
enum US13 {
    US13_Parsed(Rc<str>, i32),
    US13_Rejected(Rc<str>),
}
fn concat5_1(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v2));
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v3));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v4));
    v8
}
fn verb_family_word_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = kit_do_verbs::kit_do_verb_family(&*v0);
    v1
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> bool> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> bool> = Rc::new(move |mut v0: Rc<str>| -> bool {
        let mut v1: Rc<str> = verb_family_word_2(v0);
        let v2: bool = &*v1 != "";
        v2
    }); } CLOSURE.with(|closure| closure.clone())
}
fn token_at_6(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US6 {
    let v2: i32 = v0.len() as i32;
    let v3: bool = v1 >= v2;
    if v3 {
        US6::US6_StatementEnd
    } else {
        let v5: i32 = v0[v1 as usize].0;
        let v6: Rc<str> = v0[v1 as usize].1.clone();
        let v7: bool = v5 == 0i32;
        if v7 {
            US6::US6_Word(v6)
        } else {
            let v9: bool = v5 == 1i32;
            if v9 {
                US6::US6_Quoted(v6)
            } else {
                let v11: bool = v5 == 2i32;
                if v11 {
                    US6::US6_StatementEnd
                } else {
                    US6::US6_Unterminated(v6)
                }
            }
        }
    }
}
fn skip_ends_5(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v0.len() as i32;
        let v3: bool = v1 < v2;
        if v3 {
            let v4: US6 = token_at_6(v0.clone(), v1);
            match v4 {
                US6::US6_StatementEnd => {
                    let v5: i32 = v1.wrapping_add(1i32);
                    v1 = v5;
                    continue;
                }
                _ => {
                    return v1;
                }
            }
        } else {
            return v1;
        }
    }
}
fn word_is_11(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> bool {
    let v3: US6 = token_at_6(v0, v1);
    match v3 {
        US6::US6_Word(v4) => {
            let v4: Rc<str> = v4;
            let v5: bool = &*v4 == &*v2;
            v5
        }
        _ => {
            false
        }
    }
}
fn header_options_10(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>, mut v3: write_mode, mut v4: expectation, mut v5: Rc<str>, mut v6: Rc<rule_list>) -> US8 {
    loop {
        let v7: US6 = token_at_6(v0.clone(), v1);
        match v7 {
            US6::US6_StatementEnd => {
                return US8::US8_Parsed(v2, v3, v4, v5, v6, v1);
            }
            US6::US6_Word(v9) => {
                let v9: Rc<str> = v9;
                let v10: bool = &*v9 == "dry-run";
                if v10 {
                    let v11: i32 = v1.wrapping_add(1i32);
                    let v12: write_mode = write_mode::write_mode_Preview;
                    (v1, v3) = (v11, v12);
                    continue;
                } else {
                    let v14: bool = &*v9 == "except";
                    if v14 {
                        let v15: i32 = v1.wrapping_add(1i32);
                        let v16: US6 = token_at_6(v0.clone(), v15);
                        match v16 {
                            US6::US6_Quoted(v20) => {
                                let v20: Rc<str> = v20;
                                let v21: i32 = v1.wrapping_add(2i32);
                                (v1, v5) = (v21, v20);
                                continue;
                            }
                            US6::US6_Word(v17) => {
                                let v17: Rc<str> = v17;
                                let v18: i32 = v1.wrapping_add(2i32);
                                (v1, v5) = (v18, v17);
                                continue;
                            }
                            _ => {
                                let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: except expects a glob or a comma list of globs"); } LIT.with(|lit| lit.clone()) };
                                return US8::US8_Rejected(v23);
                            }
                        }
                    } else {
                        let v27: bool = &*v9 == "expect";
                        let v31: bool = if v27 {
                            let v28: i32 = v1.wrapping_add(1i32);
                            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files"); } LIT.with(|lit| lit.clone()) };
                            word_is_11(v0.clone(), v28, v29)
                        } else {
                            false
                        };
                        let v35: bool = if v31 {
                            let v32: i32 = v1.wrapping_add(3i32);
                            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replacements"); } LIT.with(|lit| lit.clone()) };
                            word_is_11(v0.clone(), v32, v33)
                        } else {
                            false
                        };
                        if v35 {
                            let v36: i32 = v1.wrapping_add(2i32);
                            let v37: US6 = token_at_6(v0.clone(), v36);
                            match v37 {
                                US6::US6_Word(v38) => {
                                    let v38: Rc<str> = v38;
                                    let v39: i32 = v1.wrapping_add(4i32);
                                    let v40: US6 = token_at_6(v0.clone(), v39);
                                    match v40 {
                                        US6::US6_Word(v41) => {
                                            let v41: Rc<str> = v41;
                                            let v42: bool = !v38.is_empty() && v38.bytes().all(|byte| byte.is_ascii_digit());
                                            let v44: bool = if v42 {
                                                let v43: bool = !v41.is_empty() && v41.bytes().all(|byte| byte.is_ascii_digit());
                                                v43
                                            } else {
                                                false
                                            };
                                            if v44 {
                                                let v45: i32 = v1.wrapping_add(5i32);
                                                let v46: u64 = v38.trim().parse::<u64>().unwrap_or(0);
                                                let v47: u64 = v41.trim().parse::<u64>().unwrap_or(0);
                                                let v48: expectation = expectation::expectation_Expected(v46, v47);
                                                (v1, v4) = (v45, v48);
                                                continue;
                                            } else {
                                                let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                                return US8::US8_Rejected(v50);
                                            }
                                        }
                                        _ => {
                                            let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                            return US8::US8_Rejected(v53);
                                        }
                                    }
                                }
                                _ => {
                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expect files <n> replacements <n>"); } LIT.with(|lit| lit.clone()) };
                                    return US8::US8_Rejected(v56);
                                }
                            }
                        } else {
                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unknown option "); } LIT.with(|lit| lit.clone()) };
                            let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v61: Rc<str> = concat5_1(v59, v9, v60.clone(), v60.clone(), v60.clone());
                            return US8::US8_Rejected(v61);
                        }
                    }
                }
            }
            _ => {
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
                return US8::US8_Rejected(v66);
            }
        }
    }
}
fn edit_header_at_9(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US8 {
    let v2: i32 = v1.wrapping_add(1i32);
    let v3: US6 = token_at_6(v0.clone(), v2);
    match v3 {
        US6::US6_Quoted(v11) => {
            let v11: Rc<str> = v11;
            let v12: i32 = v1.wrapping_add(2i32);
            let v13: write_mode = write_mode::write_mode_Apply;
            let v14: expectation = expectation::expectation_Unchecked;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<rule_list> = { thread_local!{ static CASE: Rc<rule_list> = Rc::new(rule_list::rule_list_NoRules); } CASE.with(|case| case.clone()) };
            header_options_10(v0, v12, v11, v13, v14, v15, v16)
        }
        US6::US6_Word(v4) => {
            let v4: Rc<str> = v4;
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: write_mode = write_mode::write_mode_Apply;
            let v7: expectation = expectation::expectation_Unchecked;
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<rule_list> = { thread_local!{ static CASE: Rc<rule_list> = Rc::new(rule_list::rule_list_NoRules); } CASE.with(|case| case.clone()) };
            header_options_10(v0, v5, v4, v6, v7, v8, v9)
        }
        _ => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
            US8::US8_Rejected(v18)
        }
    }
}
fn count_at_14(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US11 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
    let v3: bool = word_is_11(v0.clone(), v1, v2);
    if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: replace_count = replace_count::replace_count_Once;
        US11::US11_Parsed(v5, v4)
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = word_is_11(v0.clone(), v1, v7);
        if v8 {
            let v9: i32 = v1.wrapping_add(1i32);
            let v10: replace_count = replace_count::replace_count_Every;
            US11::US11_Parsed(v10, v9)
        } else {
            let v12: i32 = v1.wrapping_add(1i32);
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let v14: bool = word_is_11(v0.clone(), v12, v13);
            if v14 {
                let v15: US6 = token_at_6(v0, v1);
                match v15 {
                    US6::US6_Word(v16) => {
                        let v16: Rc<str> = v16;
                        let v17: bool = !v16.is_empty() && v16.bytes().all(|byte| byte.is_ascii_digit());
                        if v17 {
                            let v18: u64 = v16.trim().parse::<u64>().unwrap_or(0);
                            let v19: i32 = v1.wrapping_add(2i32);
                            let v20: replace_count = replace_count::replace_count_Exactly(v18);
                            US11::US11_Parsed(v20, v19)
                        } else {
                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: not a count: "); } LIT.with(|lit| lit.clone()) };
                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v24: Rc<str> = concat5_1(v22, v16, v23.clone(), v23.clone(), v23.clone());
                            US11::US11_Rejected(v24)
                        }
                    }
                    _ => {
                        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                        US11::US11_Rejected(v27)
                    }
                }
            } else {
                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected once, all or <n> times"); } LIT.with(|lit| lit.clone()) };
                US11::US11_Rejected(v30)
            }
        }
    }
}
fn quoted_problem_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": \""); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    concat5_1(v0, v2, v1, v3, v4)
}
fn rule_at_13(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US10 {
    let v2: i32 = v1.wrapping_add(1i32);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pattern"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = word_is_11(v0.clone(), v2, v3);
    let v7: rule_matcher = if v4 {
        rule_matcher::rule_matcher_Pattern
    } else {
        rule_matcher::rule_matcher_Literal
    };
    let v9: i32 = if v4 {
        let v8: i32 = v1.wrapping_add(2i32);
        v8
    } else {
        v2
    };
    let v10: US11 = count_at_14(v0.clone(), v9);
    match v10 {
        US11::US11_Parsed(v13, v14) => {
            let v13: replace_count = v13;
            let v14: i32 = v14;
            let v15: US6 = token_at_6(v0.clone(), v14);
            match v15 {
                US6::US6_Quoted(v16) => {
                    let v16: Rc<str> = v16;
                    let v17: i32 = v14.wrapping_add(1i32);
                    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("with"); } LIT.with(|lit| lit.clone()) };
                    let v19: bool = word_is_11(v0.clone(), v17, v18);
                    if v19 {
                        let v20: i32 = v14.wrapping_add(2i32);
                        let v21: US6 = token_at_6(v0.clone(), v20);
                        match v21 {
                            US6::US6_Quoted(v22) => {
                                let v22: Rc<str> = v22;
                                let v23: i32 = v14.wrapping_add(3i32);
                                let v24: US6 = token_at_6(v0, v23);
                                match v24 {
                                    US6::US6_StatementEnd => {
                                        US10::US10_Parsed(v16, v22, v13, v7, v23)
                                    }
                                    _ => {
                                        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: unexpected text after the replacement"); } LIT.with(|lit| lit.clone()) };
                                        US10::US10_Rejected(v26)
                                    }
                                }
                            }
                            US6::US6_Unterminated(v29) => {
                                let v29: Rc<str> = v29;
                                let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                                let v31: Rc<str> = quoted_problem_15(v30, v29);
                                US10::US10_Rejected(v31)
                            }
                            _ => {
                                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted replacement after with"); } LIT.with(|lit| lit.clone()) };
                                US10::US10_Rejected(v33)
                            }
                        }
                    } else {
                        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected with after the anchor"); } LIT.with(|lit| lit.clone()) };
                        US10::US10_Rejected(v37)
                    }
                }
                US6::US6_Unterminated(v40) => {
                    let v40: Rc<str> = v40;
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                    let v42: Rc<str> = quoted_problem_15(v41, v40);
                    US10::US10_Rejected(v42)
                }
                _ => {
                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: expected a quoted anchor"); } LIT.with(|lit| lit.clone()) };
                    US10::US10_Rejected(v44)
                }
            }
        }
        US11::US11_Rejected(v11) => {
            let v11: Rc<str> = v11;
            US10::US10_Rejected(v11)
        }
    }
}
fn patch_stray_line_17(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((&*v0).split(char::from(10u8)).find(|line| !(line.is_empty() || line.starts_with("@") || line.starts_with(char::from(32u8)) || line.starts_with(char::from(45u8)) || line.starts_with(char::from(43u8)))).unwrap_or(""));
    v1
}
fn hunks_of_18(mut v0: Rc<str>) -> std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> {
    let mut v1: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = { let newline = char::from(10u8).to_string(); let mut hunks: Vec<(std::rc::Rc<str>, std::rc::Rc<str>)> = Vec::new(); let mut old: Vec<&str> = Vec::new(); let mut new: Vec<&str> = Vec::new(); for line in (&*v0).split(char::from(10u8)).chain(std::iter::once("@")) { if line.starts_with("@") { if !old.is_empty() || !new.is_empty() { hunks.push((std::rc::Rc::from(old.join(newline.as_str())), std::rc::Rc::from(new.join(newline.as_str())))); } old.clear(); new.clear(); } else if line.is_empty() { old.push(line); new.push(line); } else if let Some(kept) = line.strip_prefix(char::from(32u8)) { old.push(kept); new.push(kept); } else if let Some(removed) = line.strip_prefix(char::from(45u8)) { old.push(removed); } else if let Some(added) = line.strip_prefix(char::from(43u8)) { new.push(added); } } std::rc::Rc::new(hunks) };
    v1
}
fn patch_at_16(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US12 {
    let v2: i32 = v1.wrapping_add(1i32);
    let v3: US6 = token_at_6(v0.clone(), v2);
    match v3 {
        US6::US6_Quoted(v4) => {
            let v4: Rc<str> = v4;
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: US6 = token_at_6(v0, v5);
            match v6 {
                US6::US6_StatementEnd => {
                    let v7: Rc<str> = patch_stray_line_17(v4.clone());
                    let v8: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = hunks_of_18(v4);
                    let v9: bool = &*v7 != "";
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: not a hunk line"); } LIT.with(|lit| lit.clone()) };
                        let v11: Rc<str> = quoted_problem_15(v10, v7);
                        US12::US12_Rejected(v11)
                    } else {
                        let v13: bool = v8.iter().any(|hunk| hunk.0 == hunk.1);
                        if v13 {
                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: a hunk changes nothing"); } LIT.with(|lit| lit.clone()) };
                            US12::US12_Rejected(v14)
                        } else {
                            let v16: bool = v8.iter().any(|hunk| hunk.0.is_empty());
                            if v16 {
                                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: a hunk needs context or removed lines to anchor it"); } LIT.with(|lit| lit.clone()) };
                                US12::US12_Rejected(v17)
                            } else {
                                US12::US12_Parsed(v8, v5)
                            }
                        }
                    }
                }
                _ => {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: unexpected text after the hunks"); } LIT.with(|lit| lit.clone()) };
                    US12::US12_Rejected(v23)
                }
            }
        }
        US6::US6_Unterminated(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = quoted_problem_15(v27, v26);
            US12::US12_Rejected(v28)
        }
        _ => {
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: expected a <<< block of hunks"); } LIT.with(|lit| lit.clone()) };
            US12::US12_Rejected(v30)
        }
    }
}
fn hunk_rules_19(mut v0: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<rule_list>) -> Rc<rule_list> {
    let v3: i32 = v0.len() as i32;
    let v4: bool = v1 < v3;
    if v4 {
        let v5: Rc<str> = v0[v1 as usize].0.clone();
        let v6: Rc<str> = v0[v1 as usize].1.clone();
        let v7: i32 = v1.wrapping_add(1i32);
        let v8: Rc<rule_list> = hunk_rules_19(v0, v7, v2);
        let v9: replace_count = replace_count::replace_count_Once;
        let v10: rule_matcher = rule_matcher::rule_matcher_Literal;
        Rc::new(rule_list::rule_list_Rule(v5, v6, v9, v10, v8))
    } else {
        v2
    }
}
fn rules_at_12(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US9 {
    let v2: i32 = skip_ends_5(v0.clone(), v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
    let v4: bool = word_is_11(v0.clone(), v2, v3);
    if v4 {
        let v5: US10 = rule_at_13(v0.clone(), v2);
        match v5 {
            US10::US10_Parsed(v8, v9, v10, v11, v12) => {
                let v8: Rc<str> = v8;
                let v9: Rc<str> = v9;
                let v10: replace_count = v10;
                let v11: rule_matcher = v11;
                let v12: i32 = v12;
                let v13: US9 = rules_at_12(v0, v12);
                match v13 {
                    US9::US9_Parsed(v16, v17) => {
                        let v16: Rc<rule_list> = v16;
                        let v17: i32 = v17;
                        let v18: Rc<rule_list> = Rc::new(rule_list::rule_list_Rule(v8, v9, v10, v11, v16));
                        US9::US9_Parsed(v18, v17)
                    }
                    US9::US9_Rejected(v14) => {
                        let v14: Rc<str> = v14;
                        US9::US9_Rejected(v14)
                    }
                }
            }
            US10::US10_Rejected(v6) => {
                let v6: Rc<str> = v6;
                US9::US9_Rejected(v6)
            }
        }
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch"); } LIT.with(|lit| lit.clone()) };
        let v25: bool = word_is_11(v0.clone(), v2, v24);
        if v25 {
            let v26: US12 = patch_at_16(v0.clone(), v2);
            match v26 {
                US12::US12_Parsed(v29, v30) => {
                    let v29: std::rc::Rc<Vec<(std::rc::Rc<str>, std::rc::Rc<str>)>> = v29;
                    let v30: i32 = v30;
                    let v31: US9 = rules_at_12(v0, v30);
                    match v31 {
                        US9::US9_Parsed(v34, v35) => {
                            let v34: Rc<rule_list> = v34;
                            let v35: i32 = v35;
                            let v36: i32 = 0i32;
                            let v37: Rc<rule_list> = hunk_rules_19(v29, v36, v34);
                            US9::US9_Parsed(v37, v35)
                        }
                        US9::US9_Rejected(v32) => {
                            let v32: Rc<str> = v32;
                            US9::US9_Rejected(v32)
                        }
                    }
                }
                US12::US12_Rejected(v27) => {
                    let v27: Rc<str> = v27;
                    US9::US9_Rejected(v27)
                }
            }
        } else {
            let v43: Rc<rule_list> = { thread_local!{ static CASE: Rc<rule_list> = Rc::new(rule_list::rule_list_NoRules); } CASE.with(|case| case.clone()) };
            US9::US9_Parsed(v43, v2)
        }
    }
}
fn edit_at_8(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let v2: US8 = edit_header_at_9(v0.clone(), v1);
    match v2 {
        US8::US8_Parsed(v5, v6, v7, v8, v9, v10) => {
            let v5: Rc<str> = v5;
            let v6: write_mode = v6;
            let v7: expectation = v7;
            let v8: Rc<str> = v8;
            let v9: Rc<rule_list> = v9;
            let v10: i32 = v10;
            let v11: US9 = rules_at_12(v0, v10);
            match v11 {
                US9::US9_Parsed(v14, v15) => {
                    let v14: Rc<rule_list> = v14;
                    let v15: i32 = v15;
                    match &*v14 {
                        rule_list::rule_list_NoRules => {
                            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit "); } LIT.with(|lit| lit.clone()) };
                            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": no replace or patch lines under it"); } LIT.with(|lit| lit.clone()) };
                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                            let v19: Rc<str> = concat5_1(v16, v5, v17, v18.clone(), v18);
                            US7::US7_Rejected(v19)
                        }
                        _ => {
                            let v21: step = step::step_EditStep(v5, v6, v7, v8, v14);
                            US7::US7_Parsed(v21, v15)
                        }
                    }
                }
                US9::US9_Rejected(v12) => {
                    let v12: Rc<str> = v12;
                    US7::US7_Rejected(v12)
                }
            }
        }
        US8::US8_Rejected(v3) => {
            let v3: Rc<str> = v3;
            US7::US7_Rejected(v3)
        }
    }
}
fn show_at_20(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32) -> US7 {
    let v2: i32 = v1.wrapping_add(1i32);
    let v3: US6 = token_at_6(v0.clone(), v2);
    match v3 {
        US6::US6_Word(v4) => {
            let v4: Rc<str> = v4;
            let v5: i32 = v1.wrapping_add(2i32);
            let v6: US6 = token_at_6(v0, v5);
            match v6 {
                US6::US6_StatementEnd => {
                    let v7: step = step::step_ShowStep(v4);
                    US7::US7_Parsed(v7, v5)
                }
                _ => {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected one path"); } LIT.with(|lit| lit.clone()) };
                    US7::US7_Rejected(v9)
                }
            }
        }
        _ => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show: expected a path"); } LIT.with(|lit| lit.clone()) };
            US7::US7_Rejected(v12)
        }
    }
}
fn query_words_22(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US13 {
    loop {
        let v3: US6 = token_at_6(v0.clone(), v1);
        match v3 {
            US6::US6_Quoted(v15) => {
                let v15: Rc<str> = v15;
                let v16: i32 = v1.wrapping_add(1i32);
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("q"); } LIT.with(|lit| lit.clone()) };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = concat5_1(v17, v15, v18.clone(), v18.clone(), v18.clone());
                let v20: bool = &*v2 == "";
                let v23: Rc<str> = if v20 {
                    v19
                } else {
                    let v21: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
                    concat5_1(v2, v21, v19, v18.clone(), v18.clone())
                };
                (v1, v2) = (v16, v23);
                continue;
            }
            US6::US6_StatementEnd => {
                return US13::US13_Parsed(v2, v1);
            }
            US6::US6_Unterminated(v25) => {
                let v25: Rc<str> = v25;
                let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
                let v27: Rc<str> = quoted_problem_15(v26, v25);
                return US13::US13_Rejected(v27);
            }
            US6::US6_Word(v5) => {
                let v5: Rc<str> = v5;
                let v6: i32 = v1.wrapping_add(1i32);
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("w"); } LIT.with(|lit| lit.clone()) };
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v9: Rc<str> = concat5_1(v7, v5, v8.clone(), v8.clone(), v8.clone());
                let v10: bool = &*v2 == "";
                let v13: Rc<str> = if v10 {
                    v9
                } else {
                    let v11: Rc<str> = std::rc::Rc::<str>::from(char::from(31u8).to_string());
                    concat5_1(v2, v11, v9, v8.clone(), v8.clone())
                };
                (v1, v2) = (v6, v13);
                continue;
            }
        }
    }
}
fn query_at_21(mut v0: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v1: i32, mut v2: Rc<str>) -> US7 {
    let v3: i32 = v1.wrapping_add(1i32);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: US13 = query_words_22(v0, v3, v4);
    match v5 {
        US13::US13_Parsed(v8, v9) => {
            let v8: Rc<str> = v8;
            let v9: i32 = v9;
            let v10: step = step::step_QueryStep(v2, v8);
            US7::US7_Parsed(v10, v9)
        }
        US13::US13_Rejected(v6) => {
            let v6: Rc<str> = v6;
            US7::US7_Rejected(v6)
        }
    }
}
fn step_at_7(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v2: i32) -> US7 {
    let v3: US6 = token_at_6(v1.clone(), v2);
    match v3 {
        US6::US6_Quoted(v30) => {
            let v30: Rc<str> = v30;
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected quoted text"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = quoted_problem_15(v31, v30);
            US7::US7_Rejected(v32)
        }
        US6::US6_StatementEnd => {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("empty statement"); } LIT.with(|lit| lit.clone()) };
            US7::US7_Rejected(v34)
        }
        US6::US6_Unterminated(v26) => {
            let v26: Rc<str> = v26;
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unterminated string"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = quoted_problem_15(v27, v26);
            US7::US7_Rejected(v28)
        }
        US6::US6_Word(v4) => {
            let v4: Rc<str> = v4;
            let v5: bool = &*v4 == "edit";
            if v5 {
                edit_at_8(v1, v2)
            } else {
                let v7: bool = &*v4 == "show";
                if v7 {
                    show_at_20(v1, v2)
                } else {
                    let v9: bool = v0(v4.clone());
                    if v9 {
                        query_at_21(v1, v2, v4)
                    } else {
                        let v11: bool = &*v4 == "replace";
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace: needs an edit line above it"); } LIT.with(|lit| lit.clone()) };
                            US7::US7_Rejected(v12)
                        } else {
                            let v14: bool = &*v4 == "patch";
                            if v14 {
                                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("patch: needs an edit line above it"); } LIT.with(|lit| lit.clone()) };
                                US7::US7_Rejected(v15)
                            } else {
                                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unsupported statement: "); } LIT.with(|lit| lit.clone()) };
                                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                                let v19: Rc<str> = concat5_1(v17, v4, v18.clone(), v18.clone(), v18);
                                US7::US7_Rejected(v19)
                            }
                        }
                    }
                }
            }
        }
    }
}
fn steps_at_4(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>>, mut v2: i32) -> US0 {
    let v3: i32 = skip_ends_5(v1.clone(), v2);
    let v4: i32 = v1.len() as i32;
    let v5: bool = v3 >= v4;
    if v5 {
        let v6: Rc<steps> = { thread_local!{ static CASE: Rc<steps> = Rc::new(steps::steps_Done); } CASE.with(|case| case.clone()) };
        US0::US0_Parsed(v6, v3)
    } else {
        let v8: US7 = step_at_7(v0.clone(), v1.clone(), v3);
        match v8 {
            US7::US7_Parsed(v11, v12) => {
                let v11: step = v11;
                let v12: i32 = v12;
                let v13: US0 = steps_at_4(v0, v1, v12);
                match v13 {
                    US0::US0_Parsed(v16, v17) => {
                        let v16: Rc<steps> = v16;
                        let v17: i32 = v17;
                        let v18: Rc<steps> = Rc::new(steps::steps_Step(v11, v16));
                        US0::US0_Parsed(v18, v17)
                    }
                    US0::US0_Rejected(v14) => {
                        let v14: Rc<str> = v14;
                        US0::US0_Rejected(v14)
                    }
                }
            }
            US7::US7_Rejected(v9) => {
                let v9: Rc<str> = v9;
                US0::US0_Rejected(v9)
            }
        }
    }
}
fn tokens_of_23(mut v0: Rc<str>) -> std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> {
    let mut v1: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = { let newline = char::from(10u8); let semicolon = char::from(59u8); let quote = char::from(34u8); let backslash = char::from(92u8); let angle_open = char::from(60u8); let angle_close = char::from(62u8); let carriage = char::from(13u8); let chars: Vec<char> = (&*v0).chars().collect(); let length = chars.len(); let mut found: Vec<(i32, std::rc::Rc<str>)> = Vec::new(); let mut index = 0usize; while index < length { let character = chars[index]; if character == newline || character == semicolon { found.push((2, std::rc::Rc::from(""))); index += 1; } else if character.is_whitespace() { index += 1; } else if character == angle_open && index + 3 < length && chars[index + 1] == angle_open && chars[index + 2] == angle_open && (chars[index + 3] == newline || (chars[index + 3] == carriage && index + 4 < length && chars[index + 4] == newline)) { let mut content = index + 3; while chars[content] != newline { content += 1; } content += 1; let mut line = content; let mut closed: Option<(usize, usize)> = None; while line < length { let mut first = line; while first < length && (chars[first] == char::from(32u8) || chars[first] == char::from(9u8)) { first += 1; } if first + 3 <= length && chars[first] == angle_close && chars[first + 1] == angle_close && chars[first + 2] == angle_close { closed = Some((line, first + 3)); break; } while line < length && chars[line] != newline { line += 1; } line += 1; } match closed { Some((close_line, after)) => { let end = if close_line > content { close_line - 1 } else { content }; let text: String = chars[content..end].iter().filter(|c| **c != carriage).collect(); found.push((1, std::rc::Rc::from(text))); index = after; } None => { found.push((3, std::rc::Rc::from("<<<"))); index = length; } } } else if character == quote { let mut text = String::new(); let mut closed = false; index += 1; while index < length { let inner = chars[index]; index += 1; if inner == backslash { if index < length { let escaped = chars[index]; index += 1; if escaped == char::from(110u8) { text.push(newline); } else if escaped == char::from(116u8) { text.push(char::from(9u8)); } else if escaped == quote || escaped == backslash { text.push(escaped); } else { text.push(backslash); text.push(escaped); } } } else if inner == quote { closed = true; break; } else { text.push(inner); } } found.push((if closed { 1 } else { 3 }, std::rc::Rc::from(text))); } else { let start = index; while index < length && chars[index].is_whitespace() == false && chars[index] != semicolon && chars[index] != quote { index += 1; } let text: String = chars[start..index].iter().collect(); found.push((0, std::rc::Rc::from(text))); } } std::rc::Rc::new(found) };
    v1
}
fn program_of_3(mut v0: Rc<dyn Fn(Rc<str>) -> bool>, mut v1: Rc<str>) -> US0 {
    let v2: std::rc::Rc<Vec<(i32, std::rc::Rc<str>)>> = tokens_of_23(v1);
    let v3: i32 = 0i32;
    steps_at_4(v0, v2, v3)
}
fn fields_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<str>) -> Rc<str> {
    let v5: Rc<str> = std::rc::Rc::<str>::from([&*v0, &*v1, &*v2, &*v3, &*v4].join(&char::from(31u8).to_string()));
    v5
}
fn decimal_text_27(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn edit_record_25(mut v0: Rc<str>, mut v1: write_mode, mut v2: expectation, mut v3: &Rc<str>, mut v4: &Rc<rule_list>) -> Rc<str> {
    let v8: Rc<str> = match v1 {
        write_mode::write_mode_Apply => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("apply"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        write_mode::write_mode_Preview => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    };
    match v2 {
        expectation::expectation_Expected(v12, v13) => {
            let v12: u64 = v12;
            let v13: u64 = v13;
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = decimal_text_27(v12);
            let v16: Rc<str> = decimal_text_27(v13);
            fields_26(v14, v0, v8, v15, v16)
        }
        expectation::expectation_Unchecked => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
            fields_26(v9, v0, v8, v10.clone(), v10)
        }
    }
}
fn records_joined_28(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v0, v3, v1, v4.clone(), v4)
    }
}
fn excluded_encoded_29(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("except"); } LIT.with(|lit| lit.clone()) };
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = fields_26(v3, v0, v4.clone(), v4.clone(), v4);
        records_joined_28(v1, v5)
    }
}
fn rule_record_31(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: replace_count, mut v3: rule_matcher) -> Rc<str> {
    let v7: Rc<str> = match v3 {
        rule_matcher::rule_matcher_Literal => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("replace"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        rule_matcher::rule_matcher_Pattern => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pattern"); } LIT.with(|lit| lit.clone()) };
            v5
        }
    };
    match v2 {
        replace_count::replace_count_Every => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("all"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
            fields_26(v7, v11, v12, v0, v1)
        }
        replace_count::replace_count_Exactly(v14) => {
            let v14: u64 = v14;
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("times"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = decimal_text_27(v14);
            fields_26(v7, v15, v16, v0, v1)
        }
        replace_count::replace_count_Once => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("once"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            fields_26(v7, v8, v9, v0, v1)
        }
    }
}
fn rules_encoded_30(mut v0: Rc<rule_list>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            rule_list::rule_list_NoRules => {
                return v1;
            }
            rule_list::rule_list_Rule(v2, v3, v4, v5, v6) => {
                let v2: Rc<str> = v2;
                let v3: Rc<str> = v3;
                let v4: replace_count = v4;
                let v5: rule_matcher = v5;
                let v6: Rc<rule_list> = v6;
                let v7: Rc<str> = rule_record_31(v2, v3, v4, v5);
                let v8: Rc<str> = records_joined_28(v1, v7);
                (v0, v1) = (v6, v8);
                continue;
            }
        }
    }
}
fn steps_encoded_24(mut v0: Rc<steps>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            steps::steps_Done => {
                return v1;
            }
            steps::steps_Step(v2, v3) => {
                let v2: step = v2;
                let v3: Rc<steps> = v3;
                match v2 {
                    step::step_EditStep(v4, v5, v6, v7, v8) => {
                        let v4: Rc<str> = v4;
                        let v5: write_mode = v5;
                        let v6: expectation = v6;
                        let v7: Rc<str> = v7;
                        let v8: Rc<rule_list> = v8;
                        let v9: Rc<str> = edit_record_25(v4, v5, v6, &v7, &v8);
                        let v10: Rc<str> = records_joined_28(v1, v9);
                        let v11: Rc<str> = excluded_encoded_29(v7, v10);
                        let v12: Rc<str> = rules_encoded_30(v8, v11);
                        (v0, v1) = (v3, v12);
                        continue;
                    }
                    step::step_QueryStep(v20, v21) => {
                        let v20: Rc<str> = v20;
                        let v21: Rc<str> = v21;
                        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("query"); } LIT.with(|lit| lit.clone()) };
                        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v24: Rc<str> = fields_26(v22, v20, v21, v23.clone(), v23.clone());
                        let v25: Rc<str> = records_joined_28(v1, v24);
                        (v0, v1) = (v3, v25);
                        continue;
                    }
                    step::step_ShowStep(v14) => {
                        let v14: Rc<str> = v14;
                        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                        let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v17: Rc<str> = fields_26(v15, v14, v16.clone(), v16.clone(), v16.clone());
                        let v18: Rc<str> = records_joined_28(v1, v17);
                        (v0, v1) = (v3, v18);
                        continue;
                    }
                }
            }
        }
    }
}
fn compiled_0(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: bool = !v0.contains(char::from(30u8)) && !v0.contains(char::from(31u8));
    if v1 {
        let v2: Rc<dyn Fn(Rc<str>) -> bool> = closure1();
        let v3: US0 = program_of_3(v2, v0);
        match v3 {
            US0::US0_Parsed(v9, v10) => {
                let v9: Rc<steps> = v9;
                let v10: i32 = v10;
                let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
                let v12: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v14: Rc<str> = steps_encoded_24(v9, v13.clone());
                concat5_1(v11, v12, v14, v13.clone(), v13)
            }
            US0::US0_Rejected(v4) => {
                let v4: Rc<str> = v4;
                let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rejected"); } LIT.with(|lit| lit.clone()) };
                let v6: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
                let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                concat5_1(v5, v6, v4, v7.clone(), v7)
            }
        }
    } else {
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rejected"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("control characters 30 and 31 are reserved"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        concat5_1(v18, v19, v20, v21.clone(), v21)
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        compiled_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_program_compiled(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
