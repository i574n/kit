#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_model::{query_verb};
#[derive(Clone)]
enum US0 {
    US0_QueryParsed(query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64),
    US0_QueryRejected(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(query_verb),
    US2_None,
}
#[derive(Clone)]
enum US3 {
    US3_Bare(Rc<str>),
    US3_Text(Rc<str>),
    US3_Finished,
}
#[derive(Clone)]
enum US5 {
    US5_FlagLiteral,
    US5_FlagPaths,
}
#[derive(Clone)]
enum US4 {
    US4_Some(US5),
    US4_None,
}
#[derive(Clone)]
enum US7 {
    US7_ValuedIn,
    US7_ValuedContext,
    US7_ValuedFirst,
    US7_ValuedInclude,
    US7_ValuedExclude,
    US7_ValuedTimeout,
}
#[derive(Clone)]
enum US6 {
    US6_Some(US7),
    US6_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(u64),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Succeeded,
    US9_Failed,
}
fn now_ms_1() -> u64 {
    let mut v0: u64 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0);
    v0
}
fn records_of_2(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn field_at_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn verb_of_6(mut v0: Rc<str>) -> query_verb {
    let mut v1: query_verb = query_verb::query_verb_Search;
    let mut v2: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v1);
    let mut v3: bool = &*v0 == &*v2;
    let v37: US2 = if v3 {
        let mut v4: query_verb = query_verb::query_verb_Search;
        US2::US2_Some(v4)
    } else {
        let v6: query_verb = query_verb::query_verb_Read;
        let v7: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: query_verb = query_verb::query_verb_Read;
            US2::US2_Some(v9)
        } else {
            let v11: query_verb = query_verb::query_verb_Files;
            let v12: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: query_verb = query_verb::query_verb_Files;
                US2::US2_Some(v14)
            } else {
                let v16: query_verb = query_verb::query_verb_Head;
                let v17: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: query_verb = query_verb::query_verb_Head;
                    US2::US2_Some(v19)
                } else {
                    let v21: query_verb = query_verb::query_verb_Tail;
                    let mut v22: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: query_verb = query_verb::query_verb_Tail;
                        US2::US2_Some(v24)
                    } else {
                        let v26: query_verb = query_verb::query_verb_Wait;
                        let v27: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: query_verb = query_verb::query_verb_Wait;
                            US2::US2_Some(v29)
                        } else {
                            US2::US2_None
                        }
                    }
                }
            }
        }
    };
    match v37 {
        US2::US2_None => {
            query_verb::query_verb_Tail
        }
        US2::US2_Some(v38) => {
            let v38: query_verb = v38;
            v38
        }
    }
}
fn opened_query_7(mut v0: query_verb) -> (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, v1.clone(), false, v1.clone(), 0u64, 0u64, 0u64, v1.clone(), v1, false, 0u64, 0u64)
}
fn item_at_9(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> US3 {
    let mut v3: Rc<str> = field_at_5(v0, v1, v2);
    let mut v4: bool = &*v3 == "";
    if v4 {
        US3::US3_Finished
    } else {
        let v6: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[..1]);
        let v7: Rc<str> = std::rc::Rc::<str>::from(&(&*v3)[1..]);
        let v8: bool = &*v6 == "q";
        if v8 {
            US3::US3_Text(v7)
        } else {
            US3::US3_Bare(v7)
        }
    }
}
fn value_at_10(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: US3 = item_at_9(v0, v1, v2);
    match v3 {
        US3::US3_Bare(v4) => {
            let mut v4: Rc<str> = v4;
            v4
        }
        US3::US3_Finished => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v6
        }
        US3::US3_Text(v5) => {
            let v5: Rc<str> = v5;
            v5
        }
    }
}
fn method12(mut v0: Option<u64>) -> Option<u64> {
    v0
}
fn closure1() -> Rc<dyn Fn((u64)) -> US8> {
    thread_local!{ static CLOSURE: Rc<dyn Fn((u64)) -> US8> = Rc::new(move |mut v0: (u64)| -> US8 {
        let mut v1: u64 = (v0);
        US8::US8_Some(v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn decimal_of_11(mut v0: Rc<str>) -> u64 {
    let mut v1: Option<u64> = (v0).trim().parse::<u64>().ok();
    let mut v2: Option<u64> = method12(v1);
    let mut v3: Rc<dyn Fn((u64)) -> US8> = closure1();
    let mut v4: Option<US8> = v2.map(|x| v3(x));
    let v6: US8 = US8::US8_None;
    let v7: US8 = v6.clone();
    let v8: US8 = v4.unwrap_or(v7);
    match v8 {
        US8::US8_None => {
            0u64
        }
        US8::US8_Some(v9) => {
            let v9: u64 = v9;
            v9
        }
    }
}
fn query_options_8(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32, mut v3: query_verb, mut v4: Rc<str>, mut v5: bool, mut v6: Rc<str>, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: Rc<str>, mut v11: Rc<str>, mut v12: bool, mut v13: u64, mut v14: u64) -> US0 {
    loop {
        let v15: US3 = item_at_9(v0.clone(), v1, v2);
        match v15 {
            US3::US3_Bare(v20) => {
                let v20: Rc<str> = v20;
                let v21: i32 = v2.wrapping_add(1i32);
                let mut v22: Rc<str> = value_at_10(v0.clone(), v1, v21);
                let v23: bool = &*v20 == "literal";
                let v31: US4 = if v23 {
                    let v24: US5 = US5::US5_FlagLiteral;
                    US4::US4_Some(v24)
                } else {
                    let v26: bool = &*v20 == "paths";
                    if v26 {
                        let v27: US5 = US5::US5_FlagPaths;
                        US4::US4_Some(v27)
                    } else {
                        US4::US4_None
                    }
                };
                match v31 {
                    US4::US4_None => {
                        let v58: bool = &*v20 == "around";
                        let v68: bool = if v58 {
                            true
                        } else {
                            let v59: bool = &*v20 == "per";
                            if v59 {
                                true
                            } else {
                                let v60: bool = &*v20 == "file";
                                if v60 {
                                    true
                                } else {
                                    let v61: bool = &*v20 == "with";
                                    if v61 {
                                        true
                                    } else {
                                        let v62: bool = &*v20 == "git";
                                        if v62 {
                                            true
                                        } else {
                                            let v63: bool = &*v20 == "status";
                                            v63
                                        }
                                    }
                                }
                            }
                        };
                        let v69: bool = &*v20 == "~";
                        let v70: bool = v69 || v68;
                        if v70 {
                            v2 = v21;
                            continue;
                        } else {
                            let v72: bool = &*v20 == "in";
                            let v96: US6 = if v72 {
                                let v73: US7 = US7::US7_ValuedIn;
                                US6::US6_Some(v73)
                            } else {
                                let v75: bool = &*v20 == "context";
                                if v75 {
                                    let v76: US7 = US7::US7_ValuedContext;
                                    US6::US6_Some(v76)
                                } else {
                                    let v78: bool = &*v20 == "first";
                                    if v78 {
                                        let v79: US7 = US7::US7_ValuedFirst;
                                        US6::US6_Some(v79)
                                    } else {
                                        let v81: bool = &*v20 == "include";
                                        if v81 {
                                            let v82: US7 = US7::US7_ValuedInclude;
                                            US6::US6_Some(v82)
                                        } else {
                                            let v84: bool = &*v20 == "exclude";
                                            if v84 {
                                                let v85: US7 = US7::US7_ValuedExclude;
                                                US6::US6_Some(v85)
                                            } else {
                                                let v87: bool = &*v20 == "timeout";
                                                if v87 {
                                                    let v88: US7 = US7::US7_ValuedTimeout;
                                                    US6::US6_Some(v88)
                                                } else {
                                                    US6::US6_None
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match v96 {
                                US6::US6_None => {
                                    let v180: bool = !v20.is_empty() && v20.bytes().all(|byte| byte.is_ascii_digit());
                                    if v180 {
                                        let v181: u64 = decimal_of_11(v20);
                                        (v2, v13) = (v21, v181);
                                        continue;
                                    } else {
                                        let v183: bool = &*v6 == "";
                                        if v183 {
                                            (v2, v6) = (v21, v20);
                                            continue;
                                        } else {
                                            let v185: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("unexpected word: "); } LIT.with(|lit| lit.clone()) }, v20));
                                            return US0::US0_QueryRejected(v185);
                                        }
                                    }
                                }
                                US6::US6_Some(v97) => {
                                    let v97: US7 = v97;
                                    let v98: bool = &*v22 == "";
                                    if v98 {
                                        let v99: Rc<str> = Rc::<str>::from(format!("{}{}", v20, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": expected a value after it"); } LIT.with(|lit| lit.clone()) }));
                                        return US0::US0_QueryRejected(v99);
                                    } else {
                                        let v101: i32 = v2.wrapping_add(2i32);
                                        let (mut v166, mut v167, mut v168, mut v169, mut v170, mut v171, mut v172, mut v173, mut v174, mut v175, mut v176, mut v177): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v97 {
                                            US7::US7_ValuedIn => {
                                                (v3, v4, v5, v22, v7, v8, v9, v10, v11, v12, v13, v14)
                                            }
                                            _ => {
                                                let (mut v154, mut v155, mut v156, mut v157, mut v158, mut v159, mut v160, mut v161, mut v162, mut v163, mut v164, mut v165): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v97 {
                                                    US7::US7_ValuedContext => {
                                                        let v102: u64 = v22.split(char::from(44u8)).next().and_then(|value| value.trim().parse::<u64>().ok()).unwrap_or(0);
                                                        let v103: u64 = v22.rsplit(char::from(44u8)).next().and_then(|value| value.trim().parse::<u64>().ok()).unwrap_or(0);
                                                        (v3, v4, v5, v6, v102, v103, v9, v10, v11, v12, v13, v14)
                                                    }
                                                    _ => {
                                                        let (mut v142, mut v143, mut v144, mut v145, mut v146, mut v147, mut v148, mut v149, mut v150, mut v151, mut v152, mut v153): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v97 {
                                                            US7::US7_ValuedFirst => {
                                                                let v104: u64 = decimal_of_11(v22);
                                                                (v3, v4, v5, v6, v7, v8, v104, v10, v11, v12, v13, v14)
                                                            }
                                                            _ => {
                                                                let (mut v130, mut v131, mut v132, mut v133, mut v134, mut v135, mut v136, mut v137, mut v138, mut v139, mut v140, mut v141): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v97 {
                                                                    US7::US7_ValuedInclude => {
                                                                        (v3, v4, v5, v6, v7, v8, v9, v22, v11, v12, v13, v14)
                                                                    }
                                                                    _ => {
                                                                        let (mut v118, mut v119, mut v120, mut v121, mut v122, mut v123, mut v124, mut v125, mut v126, mut v127, mut v128, mut v129): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v97 {
                                                                            US7::US7_ValuedExclude => {
                                                                                (v3, v4, v5, v6, v7, v8, v9, v10, v22, v12, v13, v14)
                                                                            }
                                                                            _ => {
                                                                                let (mut v106, mut v107, mut v108, mut v109, mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116, mut v117): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match v97 {
                                                                                    US7::US7_ValuedTimeout => {
                                                                                        let v105: u64 = { let text = (&*v22).trim(); let (digits, scale) = if let Some(value) = text.strip_suffix("ms") { (value, 1) } else if let Some(value) = text.strip_suffix(char::from(115u8)) { (value, 1000) } else if let Some(value) = text.strip_suffix(char::from(109u8)) { (value, 60000) } else { (text, 1000) }; digits.trim().parse::<u64>().map(|value| value * scale).unwrap_or(0) };
                                                                                        (v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v105)
                                                                                    }
                                                                                    _ => unreachable!(),
                                                                                };
                                                                                (v106, v107, v108, v109, v110, v111, v112, v113, v114, v115, v116, v117)
                                                                            }
                                                                        };
                                                                        (v118, v119, v120, v121, v122, v123, v124, v125, v126, v127, v128, v129)
                                                                    }
                                                                };
                                                                (v130, v131, v132, v133, v134, v135, v136, v137, v138, v139, v140, v141)
                                                            }
                                                        };
                                                        (v142, v143, v144, v145, v146, v147, v148, v149, v150, v151, v152, v153)
                                                    }
                                                };
                                                (v154, v155, v156, v157, v158, v159, v160, v161, v162, v163, v164, v165)
                                            }
                                        };
                                        (v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v101, v166, v167, v168, v169, v170, v171, v172, v173, v174, v175, v176, v177);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    US4::US4_Some(v32) => {
                        let v32: US5 = v32;
                        let (mut v45, mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match &v32 {
                            US5::US5_FlagLiteral => {
                                (v3, v4, true, v6, v7, v8, v9, v10, v11, v12, v13, v14)
                            }
                            _ => {
                                let (mut v33, mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = match v32 {
                                    US5::US5_FlagPaths => {
                                        (v3, v4, v5, v6, v7, v8, v9, v10, v11, true, v13, v14)
                                    }
                                    _ => unreachable!(),
                                };
                                (v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44)
                            }
                        };
                        (v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = (v21, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55, v56);
                        continue;
                    }
                }
            }
            US3::US3_Finished => {
                return US0::US0_QueryParsed(v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14);
            }
            US3::US3_Text(v17) => {
                let v17: Rc<str> = v17;
                let v18: i32 = v2.wrapping_add(1i32);
                (v2, v4) = (v18, v17);
                continue;
            }
        }
    }
}
fn query_problem_13(mut v0: query_verb, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<str>, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: &Rc<str>, mut v8: &Rc<str>, mut v9: bool, mut v10: u64, mut v11: u64) -> Rc<str> {
    match v0 {
        query_verb::query_verb_Files => {
            let v26: bool = &*v3 == "";
            if v26 {
                let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v27
            } else {
                let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v28
            }
        }
        query_verb::query_verb_Head => {
            let v30: bool = &*v3 == "";
            if v30 {
                let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("head: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v31
            } else {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v32
            }
        }
        query_verb::query_verb_Read => {
            let v19: bool = &*v3 == "";
            if v19 {
                let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read: expected a path"); } LIT.with(|lit| lit.clone()) };
                v20
            } else {
                let v21: bool = &*v1 == "";
                if v21 {
                    let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read: expected around \"<text>\""); } LIT.with(|lit| lit.clone()) };
                    v22
                } else {
                    let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v23
                }
            }
        }
        query_verb::query_verb_Search => {
            let v12: bool = &*v1 == "";
            if v12 {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: expected a quoted pattern"); } LIT.with(|lit| lit.clone()) };
                v13
            } else {
                let v14: bool = &*v3 == "";
                if v14 {
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("search: expected in <path|glob>"); } LIT.with(|lit| lit.clone()) };
                    v15
                } else {
                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v16
                }
            }
        }
        query_verb::query_verb_Tail => {
            let v34: bool = &*v3 == "";
            if v34 {
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tail: expected a path or glob"); } LIT.with(|lit| lit.clone()) };
                v35
            } else {
                let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v36
            }
        }
        query_verb::query_verb_Wait => {
            let v38: bool = &*v3 == "";
            if v38 {
                let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: expected a file"); } LIT.with(|lit| lit.clone()) };
                v39
            } else {
                let v40: bool = &*v1 == "";
                if v40 {
                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wait: expected ~ \"<regex>\""); } LIT.with(|lit| lit.clone()) };
                    v41
                } else {
                    let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v42
                }
            }
        }
    }
}
fn query_parsed_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US0 {
    let mut v2: i32 = 2i32;
    let mut v3: i32 = 1i32;
    let mut v4: Rc<str> = field_at_5(v0.clone(), v1, v3);
    let v5: query_verb = verb_of_6(v4);
    let (mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = opened_query_7(v5);
    let v18: US0 = query_options_8(v0, v1, v2, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17);
    match v18 {
        US0::US0_QueryParsed(v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32) => {
            let v21: query_verb = v21;
            let mut v22: Rc<str> = v22;
            let v23: bool = v23;
            let v24: Rc<str> = v24;
            let v25: u64 = v25;
            let v26: u64 = v26;
            let v27: u64 = v27;
            let v28: Rc<str> = v28;
            let v29: Rc<str> = v29;
            let v30: bool = v30;
            let v31: u64 = v31;
            let v32: u64 = v32;
            let v33: Rc<str> = query_problem_13(v21.clone(), v22.clone(), v23, v24.clone(), v25, v26, v27, &v28, &v29, v30, v31, v32);
            let v34: bool = v31 == 0u64;
            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = if v34 {
                (v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, 10u64, v32)
            } else {
                (v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32)
            };
            let v47: bool = &*v33 == "";
            if v47 {
                let v48: bool = v46 == 0u64;
                let (mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59, mut v60): (query_verb, Rc<str>, bool, Rc<str>, u64, u64, u64, Rc<str>, Rc<str>, bool, u64, u64) = if v48 {
                    (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, 60000u64)
                } else {
                    (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46)
                };
                US0::US0_QueryParsed(v49, v50, v51, v52, v53, v54, v55, v56, v57, v58, v59, v60)
            } else {
                US0::US0_QueryRejected(v33)
            }
        }
        US0::US0_QueryRejected(v19) => {
            let v19: Rc<str> = v19;
            US0::US0_QueryRejected(v19)
        }
    }
}
fn first_rejection_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> Rc<str> {
    loop {
        let mut v2: i32 = v0.len() as i32;
        let mut v3: bool = v1 < v2;
        if v3 {
            let mut v4: US0 = query_parsed_4(v0.clone(), v1);
            match v4 {
                US0::US0_QueryParsed(v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) => {
                    let v6: query_verb = v6;
                    let v7: Rc<str> = v7;
                    let v8: bool = v8;
                    let v9: Rc<str> = v9;
                    let v10: u64 = v10;
                    let v11: u64 = v11;
                    let v12: u64 = v12;
                    let v13: Rc<str> = v13;
                    let v14: Rc<str> = v14;
                    let v15: bool = v15;
                    let v16: u64 = v16;
                    let v17: u64 = v17;
                    let v18: i32 = v1.wrapping_add(1i32);
                    v1 = v18;
                    continue;
                }
                US0::US0_QueryRejected(v5) => {
                    let v5: Rc<str> = v5;
                    return v5;
                }
            }
        } else {
            let mut v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            return v22;
        }
    }
}
fn empty_query_tally_14() -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    (v0, false, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64)
}
fn git_marked_16(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> bool {
    let mut v2: bool = v0.get(v1 as usize).map(|fields| fields.iter().any(|field| &**field == "wgit") && fields.iter().any(|field| &**field == "wstatus")).unwrap_or(false);
    v2
}
fn listing_18(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v2: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let rows = v1.iter().map(|path| (path.to_string(), std::fs::read(root.join(&**path)).ok().map(|bytes| (bytes.len() as u64, String::from_utf8_lossy(&bytes).lines().count() as u64)))).collect::<Vec<_>>(); let path_width = rows.iter().map(|row| row.0.len()).max().unwrap_or(0).max(4); let bytes_width = rows.iter().map(|row| row.1.map(|counts| counts.0.to_string().len()).unwrap_or(1)).max().unwrap_or(0).max(5); let mut shown = vec![format!("{:<path_width$}  {:<bytes_width$}  {}", "path", "bytes", "lines")]; let mut tally = [rows.len() as u64, 0, 0, 0]; let mut failed = false; for (path, counts) in &rows { match counts { Some((bytes, lines)) => { tally[1] += bytes; tally[2] += lines; shown.push(format!("{:<path_width$}  {:<bytes_width$}  {}", path, bytes, lines)); } None => { failed = true; shown.push(format!("{:<path_width$}  {:<bytes_width$}  unreadable", path, "-")); } } } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), tally, failed)) };
    v2
}
fn edge_lines_19(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: u64, mut v3: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = { let root = std::path::Path::new(&*v0); let count = v2 as usize; let from_end = v3; let files = v1.iter().map(|path| (path.to_string(), std::fs::read(root.join(&**path)).ok().map(|bytes| String::from_utf8_lossy(&bytes).lines().map(str::to_owned).collect::<Vec<_>>()))).collect::<Vec<_>>(); let mut failed = false; let mut shown = Vec::new(); let path_width = files.iter().map(|file| file.0.len()).max().unwrap_or(0); for (path, lines) in &files { let Some(lines) = lines else { failed = true; shown.push(format!("{path}  unreadable")); continue }; let first = if from_end { lines.len().saturating_sub(count) } else { 0 }; let last = if from_end { lines.len() } else { count.min(lines.len()) }; if count == 1 { shown.push(format!("{:<path_width$}  {}", path, lines.get(first).map(String::as_str).unwrap_or("(empty)"))); continue; } shown.push(format!("{path} ({} {} lines)", if from_end { "last" } else { "first" }, count)); let width = last.to_string().len(); for index in first..last { if from_end { shown.push(lines[index].clone()); } else { shown.push(format!("{:<width$}  {}", index + 1, lines[index])); } } } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [files.len() as u64, 0, 0, 0], failed)) };
    v4
}
fn around_20(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64, mut v4: u64) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let v5: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let path = &*v1; let text = match std::fs::read(std::path::Path::new(&*v0).join(path)) { Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(), Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  unreadable: {error}")), [1, 0, 0, 0], true)) }; let lines = text.lines().collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| line.contains(&*v2)).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("{path}  no match for {:?}", &*v2)), [1, 0, 0, 0], true)); } let (before, after) = (v3 as usize, v4 as usize); let numbers = found.iter().map(|index| (index + 1).to_string()).collect::<Vec<_>>().join(","); let mut shown = vec![if found.len() == 1 { format!("{path}  match at line {numbers}") } else { format!("{path}  matches at lines {numbers}") }]; let last_line = found.iter().map(|index| (index + after).min(lines.len() - 1)).max().unwrap_or(0); let width = (last_line + 1).to_string().len(); let mut printed: Option<usize> = None; let mut count = 0u64; for index in &found { let start = index.saturating_sub(before).max(printed.map_or(0, |line| line + 1)); let end = (index + after).min(lines.len() - 1); for line in start..=end { let marker = if found.contains(&line) { char::from(62u8) } else { char::from(32u8) }; shown.push(format!("{:>width$}{} {}", line + 1, marker, lines[line])); count += 1; } printed = Some(printed.map_or(end, |line| line.max(end))); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [1, found.len() as u64, count, 0], false)) })();
    v5
}
fn git_hits_21(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: Rc<str>, mut v3: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let source = if v3 { regex::escape(&*v2) } else { (&*v2).to_owned() }; let pattern = match regex::Regex::new(&source) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("search: invalid pattern: {error}")), [0, 0, 0, 0], true)) }; let root = std::path::PathBuf::from(&*v0); let state_of = |path: &str| { let full = root.join(path); let folder = full.parent().map(std::path::Path::to_path_buf).unwrap_or_else(|| root.clone()); let name = full.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(); match std::process::Command::new("git").current_dir(&folder).args(["status", "--porcelain", "--untracked-files=all", "--"]).arg(&name).stdin(std::process::Stdio::null()).output() { Ok(output) if output.status.success() => { let text = String::from_utf8_lossy(&output.stdout).into_owned(); if text.trim().is_empty() { "clean" } else if text.starts_with("??") { "untracked" } else { "modified" } } _ => "no repo" } }; let mut hits: Vec<(String, String, &str)> = Vec::new(); let mut matched = 0u64; for path in v1.clone().iter() { let text = String::from_utf8_lossy(&std::fs::read(root.join(&**path)).unwrap_or_default()).into_owned(); let found = text.lines().enumerate().filter(|(_, line)| pattern.is_match(line)).map(|(index, line)| (format!("{}:{}", path, index + 1), line.to_owned())).collect::<Vec<_>>(); if found.is_empty() { continue; } matched += 1; let state = state_of(path); hits.extend(found.into_iter().map(|(place, line)| (place, line, state))); } let place_width = hits.iter().map(|hit| hit.0.len()).max().unwrap_or(0); let text_width = hits.iter().map(|hit| hit.1.chars().count()).max().unwrap_or(0); let shown = hits.iter().map(|(place, line, state)| format!("{:<place_width$}  {:<text_width$}  {}", place, line, state)).collect::<Vec<_>>(); std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [v1.len() as u64, matched, hits.len() as u64, 0], false)) })();
    v4
}
fn searched_22(mut v0: Rc<str>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>, mut v2: Rc<str>, mut v3: bool, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let v8: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { use rayon::prelude::*; let source = if v3 { regex::escape(&*v2) } else { (&*v2).to_owned() }; let pattern = match regex::Regex::new(&source) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("search: invalid pattern: {error}")), [0, 0, 0, 0], true)) }; let root = std::path::PathBuf::from(&*v0); let paths = v1.iter().map(|path| path.to_string()).collect::<Vec<_>>(); let hits = paths.par_iter().filter_map(|path| { let bytes = std::fs::read(root.join(path)).ok()?; let text = String::from_utf8_lossy(&bytes).into_owned(); let lines = text.lines().map(str::to_owned).collect::<Vec<_>>(); let found = lines.iter().enumerate().filter(|(_, line)| pattern.is_match(line)).map(|(index, _)| index).collect::<Vec<_>>(); if found.is_empty() { None } else { Some((path.clone(), found, lines)) } }).collect::<Vec<_>>(); let (before, after, first) = (v4 as usize, v5 as usize, v6 as usize); let clip = |line: &str| if line.chars().count() > 240 { format!("{}...", line.chars().take(240).collect::<String>()) } else { line.to_owned() }; let mut shown = Vec::new(); let mut total = 0u64; for (path, found, lines) in &hits { total += found.len() as u64; let taken = if first > 0 { found.iter().copied().take(first).collect::<Vec<_>>() } else { found.clone() }; if v7 { for index in &taken { shown.push(format!("{}:{}", path, index + 1)); } continue; } shown.push(path.clone()); let last_line = taken.iter().map(|index| (index + after).min(lines.len() - 1)).max().unwrap_or(0); let width = (last_line + 1).to_string().len(); let mut printed: Option<usize> = None; for index in &taken { let start = index.saturating_sub(before).max(printed.map_or(0, |line| line + 1)); let end = (index + after).min(lines.len() - 1); for line in start..=end { let marker = if taken.contains(&line) { char::from(62u8) } else { char::from(32u8) }; shown.push(format!("{:>width$}{} {}", line + 1, marker, clip(&lines[line]))); } printed = Some(printed.map_or(end, |line| line.max(end))); } } if shown.len() > 4000 { let hidden = shown.len() - 4000; shown.truncate(4000); shown.push(format!("... {hidden} more lines (narrow the search or use first n per file)")); } std::rc::Rc::new((std::rc::Rc::<str>::from(shown.join("
")), [paths.len() as u64, hits.len() as u64, total, 0], false)) })();
    v8
}
fn waited_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: u64) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let mut v4: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = (|| -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> { let pattern = match regex::Regex::new(&*v2) { Ok(pattern) => pattern, Err(error) => return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("wait: invalid pattern: {error}")), [1, 0, 0, 0], true)) }; let path = &*v1; let file = std::path::Path::new(&*v0).join(path); let started = std::time::Instant::now(); let timeout = v3; let limit = std::time::Duration::from_millis(timeout); let elapsed_text = |millis: u128| if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }; let timeout_text = if timeout % 1000 == 0 { format!("{}s", timeout / 1000) } else { format!("{timeout}ms") }; loop { let text = std::fs::read(&file).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()).unwrap_or_default(); if let Some((index, line)) = text.lines().enumerate().find(|(_, line)| pattern.is_match(line)) { return std::rc::Rc::new((std::rc::Rc::<str>::from(format!("wait {path}  matched {:?} at line {} after {}", line, index + 1, elapsed_text(started.elapsed().as_millis()))), [1, 0, 0, 0], false)); } if started.elapsed() >= limit { let lines = text.lines().collect::<Vec<_>>(); let last = lines.last().copied().unwrap_or("(empty)"); let report = [format!("wait {path}  no match for {:?} in {timeout_text}  lines {}  bytes {}", &*v2, lines.len(), text.len()), format!("{path} (last 1 line)"), last.to_owned()].join(&char::from(10u8).to_string()); return std::rc::Rc::new((std::rc::Rc::<str>::from(report), [1, 1, 0, 0], true)); } std::thread::sleep(std::time::Duration::from_millis(100)); } })();
    v4
}
fn files_matching_24(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> std::rc::Rc<Vec<std::rc::Rc<str>>> {
    let mut v4: std::rc::Rc<Vec<std::rc::Rc<str>>> = kit_do_walk::kit_do_walk_matching(&*v0, &*v1, &*v2, &*v3);
    v4
}
fn touched_with_25(mut v0: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v1: std::rc::Rc<Vec<std::rc::Rc<str>>>) -> () {
    v0.borrow_mut().extend(v1.iter().cloned());
    ()
}
fn query_step_17(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v2: query_verb, mut v3: Rc<str>, mut v4: bool, mut v5: Rc<str>, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: Rc<str>, mut v10: Rc<str>, mut v11: bool, mut v12: u64, mut v13: u64, mut v14: bool) -> std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> {
    let v15: std::rc::Rc<Vec<std::rc::Rc<str>>> = files_matching_24(v0.clone(), v5.clone(), v9, v10);
    touched_with_25(v1, v15.clone());
    match v2 {
        query_verb::query_verb_Files => {
            listing_18(v0, v15)
        }
        query_verb::query_verb_Head => {
            let v21: bool = false;
            edge_lines_19(v0, v15, v12, v21)
        }
        query_verb::query_verb_Read => {
            around_20(v0, v5, v3, v6, v7)
        }
        query_verb::query_verb_Search => {
            if v14 {
                git_hits_21(v0, v15, v3, v4)
            } else {
                searched_22(v0, v15, v3, v4, v6, v7, v8, v11)
            }
        }
        query_verb::query_verb_Tail => {
            let v23: bool = true;
            edge_lines_19(v0, v15, v12, v23)
        }
        query_verb::query_verb_Wait => {
            waited_23(v0, v5, v3, v13)
        }
    }
}
fn lines_joined_27(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v1 == "";
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
fn outcome_merged_26(mut v0: Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: query_verb, mut v13: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)>) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v14: Rc<str> = v13.0.clone();
    let v15: Rc<str> = lines_joined_27(v0, v14);
    let v17: bool = if v1 {
        true
    } else {
        let v16: bool = v13.clone().2;
        v16
    };
    let v18: u64 = 0u64;
    let v19: u64 = v13.1[v18 as usize];
    let v20: u64 = 1u64;
    let v21: u64 = v13.1[v20 as usize];
    let mut v22: u64 = 2u64;
    let v23: u64 = v13.1[v22 as usize];
    let (mut v71, mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
        query_verb::query_verb_Search => {
            let v24: u64 = v2.wrapping_add(v19);
            let v25: u64 = v3.wrapping_add(v21);
            let v26: u64 = v4.wrapping_add(v23);
            let v27: u64 = v7.wrapping_add(1u64);
            (v15, v17, v24, v25, v26, v5, v6, v27, v8, v9, v10, v11)
        }
        _ => {
            let (mut v59, mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66, mut v67, mut v68, mut v69, mut v70): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
                query_verb::query_verb_Read => {
                    let v28: u64 = v4.wrapping_add(v21);
                    let v29: u64 = v5.wrapping_add(v23);
                    let v30: u64 = v8.wrapping_add(1u64);
                    (v15, v17, v2, v3, v28, v29, v6, v7, v30, v9, v10, v11)
                }
                _ => {
                    let (mut v47, mut v48, mut v49, mut v50, mut v51, mut v52, mut v53, mut v54, mut v55, mut v56, mut v57, mut v58): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v12 {
                        query_verb::query_verb_Files => {
                            let v31: u64 = v6.wrapping_add(v21);
                            let v32: u64 = v9.wrapping_add(1u64);
                            (v15, v17, v2, v3, v4, v5, v31, v7, v8, v32, v10, v11)
                        }
                        _ => {
                            let (mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45, mut v46): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v12 {
                                query_verb::query_verb_Wait => {
                                    let v33: u64 = v10.wrapping_add(1u64);
                                    let v34: u64 = v11.wrapping_add(v21);
                                    (v15, v17, v2, v3, v4, v5, v6, v7, v8, v9, v33, v34)
                                }
                                _ => {
                                    (v15, v17, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
                                }
                            };
                            (v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45, v46)
                        }
                    };
                    (v47, v48, v49, v50, v51, v52, v53, v54, v55, v56, v57, v58)
                }
            };
            (v59, v60, v61, v62, v63, v64, v65, v66, v67, v68, v69, v70)
        }
    };
    (v71, v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82)
}
fn queries_run_15(mut v0: Rc<str>, mut v1: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>>, mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v3: i32, mut v4: Rc<str>, mut v5: bool, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64, mut v13: u64, mut v14: u64, mut v15: u64) -> (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) {
    let v16: i32 = v2.len() as i32;
    let v17: bool = v3 < v16;
    let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102, mut v103, mut v104, mut v105, mut v106, mut v107): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = if v17 {
        let v18: US0 = query_parsed_4(v2.clone(), v3);
        let (mut v84, mut v85, mut v86, mut v87, mut v88, mut v89, mut v90, mut v91, mut v92, mut v93, mut v94, mut v95): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match &v18 {
            US0::US0_QueryParsed(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
                let v19: query_verb = v19.clone();
                let v20: Rc<str> = v20.clone();
                let v21: bool = *v21;
                let mut v22: Rc<str> = v22.clone();
                let v23: u64 = *v23;
                let v24: u64 = *v24;
                let v25: u64 = *v25;
                let v26: Rc<str> = v26.clone();
                let v27: Rc<str> = v27.clone();
                let v28: bool = *v28;
                let v29: u64 = *v29;
                let v30: u64 = *v30;
                let v31: i32 = v3.wrapping_add(1i32);
                let v32: bool = git_marked_16(v2.clone(), v3);
                let v33: std::rc::Rc<(std::rc::Rc<str>, [u64; 4], bool)> = query_step_17(v0.clone(), v1.clone(), v19.clone(), v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v32);
                let (mut v34, mut v35, mut v36, mut v37, mut v38, mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = outcome_merged_26(v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v19, v33);
                queries_run_15(v0, v1, v2, v31, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44, v45)
            }
            _ => {
                let (mut v72, mut v73, mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80, mut v81, mut v82, mut v83): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = match v18 {
                    US0::US0_QueryRejected(v58) => {
                        let v58: Rc<str> = v58;
                        let v59: i32 = v3.wrapping_add(1i32);
                        queries_run_15(v0, v1, v2, v59, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
                    }
                    _ => unreachable!(),
                };
                (v72, v73, v74, v75, v76, v77, v78, v79, v80, v81, v82, v83)
            }
        };
        (v84, v85, v86, v87, v88, v89, v90, v91, v92, v93, v94, v95)
    } else {
        (v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    };
    (v96, v97, v98, v99, v100, v101, v102, v103, v104, v105, v106, v107)
}
fn decimal_text_29(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from((v0).to_string());
    v1
}
fn receipt_counts_28(mut v0: &Rc<str>, mut v1: bool, mut v2: u64, mut v3: u64, mut v4: u64, mut v5: u64, mut v6: u64, mut v7: u64, mut v8: u64, mut v9: u64, mut v10: u64, mut v11: u64, mut v12: u64) -> Rc<str> {
    let v13: bool = v7 > 0u64;
    let v24: Rc<str> = if v13 {
        let v14: Rc<str> = decimal_text_29(v2);
        let v15: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v14));
        let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v15, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" searched  "); } LIT.with(|lit| lit.clone()) }));
        let v17: Rc<str> = decimal_text_29(v3);
        let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
        let v19: Rc<str> = decimal_text_29(v4);
        let v20: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" matched  matches "); } LIT.with(|lit| lit.clone()) }, v19));
        let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v20));
        v21
    } else {
        let mut v22: Rc<str> = decimal_text_29(v12);
        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files "); } LIT.with(|lit| lit.clone()) }, v22));
        v23
    };
    let v25: bool = v7 == 0u64;
    let v27: bool = if v25 {
        let v26: bool = v8 > 0u64;
        v26
    } else {
        false
    };
    let v31: Rc<str> = if v27 {
        let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v24, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  matches "); } LIT.with(|lit| lit.clone()) }));
        let v29: Rc<str> = decimal_text_29(v4);
        let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
        v30
    } else {
        v24
    };
    let v32: bool = v8 > 0u64;
    let v36: Rc<str> = if v32 {
        let v33: Rc<str> = Rc::<str>::from(format!("{}{}", v31, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  lines "); } LIT.with(|lit| lit.clone()) }));
        let v34: Rc<str> = decimal_text_29(v5);
        let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v33, v34));
        v35
    } else {
        v31
    };
    let v37: bool = v9 > 0u64;
    let v41: Rc<str> = if v37 {
        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  bytes "); } LIT.with(|lit| lit.clone()) }));
        let v39: Rc<str> = decimal_text_29(v6);
        let v40: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v39));
        v40
    } else {
        v36
    };
    let v42: Rc<str> = decimal_text_29(v10);
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("waits "); } LIT.with(|lit| lit.clone()) }, v42));
    let v44: bool = v10 == 0u64;
    if v44 {
        v41
    } else {
        let v45: bool = v11 == 0u64;
        if v45 {
            v43
        } else {
            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v43, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  timed out "); } LIT.with(|lit| lit.clone()) }));
            let v47: Rc<str> = decimal_text_29(v11);
            let v48: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v47));
            let v49: Rc<str> = Rc::<str>::from(format!("{}{}", v48, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  exit 3"); } LIT.with(|lit| lit.clone()) }));
            v49
        }
    }
}
fn elapsed_text_31(mut v0: u64) -> Rc<str> {
    let mut v1: Rc<str> = { let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as u64).unwrap_or(0).saturating_sub(v0); std::rc::Rc::<str>::from(if millis < 10000 { format!("{millis}ms") } else { format!("{:.1}s", millis as f64 / 1000.0) }) };
    v1
}
fn local_offset_minutes_33() -> i64 {
    let mut v0: i64 = { #[cfg(windows)] fn offset() -> i64 { #[repr(C)] #[derive(Default)] #[allow(dead_code)] struct WindowsTime { year: u16, month: u16, weekday: u16, day: u16, hour: u16, minute: u16, second: u16, millis: u16 } unsafe extern "system" { fn GetLocalTime(time: *mut WindowsTime); fn GetSystemTime(time: *mut WindowsTime); } let mut local = WindowsTime::default(); let mut universal = WindowsTime::default(); unsafe { GetLocalTime(&mut local); GetSystemTime(&mut universal); } let minutes = |time: &WindowsTime| time.day as i64 * 1440 + time.hour as i64 * 60 + time.minute as i64; let raw = (minutes(&local) - minutes(&universal)).rem_euclid(1440); ((if raw > 840 { raw - 1440 } else { raw }) as f64 / 15.0).round() as i64 * 15 } #[cfg(not(windows))] fn offset() -> i64 { 0 } offset() };
    v0
}
fn clock_text_32() -> Rc<str> {
    let mut v0: i64 = local_offset_minutes_33();
    let mut v1: Rc<str> = { let offset = v0; let seconds = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_secs()).unwrap_or(0) as i64 + offset * 60).rem_euclid(86400); let sign = if offset < 0 { char::from(45u8) } else { char::from(43u8) }; std::rc::Rc::<str>::from(if cfg!(windows) { format!("{:02}:{:02}:{:02}{}{:02}:{:02}", seconds / 3600, seconds / 60 % 60, seconds % 60, sign, offset.abs() / 60, offset.abs() % 60) } else { format!("{:02}:{:02}:{:02}Z", seconds / 3600, seconds / 60 % 60, seconds % 60) }) };
    v1
}
fn receipt_line_30(mut v0: US9, mut v1: Rc<str>, mut v2: u64) -> () {
    let v6: Rc<str> = match v0 {
        US9::US9_Failed => {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("failed"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US9::US9_Succeeded => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ok"); } LIT.with(|lit| lit.clone()) };
            v3
        }
    };
    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v6, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v1));
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v10: Rc<str> = elapsed_text_31(v2);
    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v11, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  "); } LIT.with(|lit| lit.clone()) }));
    let v13: Rc<str> = clock_text_32();
    let v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
    println!("{}", v14);
    ()
}
fn read_run_0(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: u64 = now_ms_1();
    let mut v3: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let mut v4: bool = v3.iter().skip(1).any(|fields| fields.first().map(|kind| !kind.is_empty() && &**kind != "query").unwrap_or(false));
    let v8: Rc<str> = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit and query statements cannot share one program yet; run them as two calls"); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: i32 = 1i32;
        first_rejection_3(v3.clone(), v6)
    };
    let v9: bool = &*v8 == "";
    if v9 {
        let v10: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeSet::new()));
        let v11: i32 = 1i32;
        let (mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_query_tally_14();
        let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31, mut v32, mut v33, mut v34, mut v35): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = queries_run_15(v0, v10.clone(), v3, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23);
        let v36: bool = &*v24 == "";
        if v36 {
            ()
        } else {
            println!("{}", v24);
            ()
        };
        let v37: u64 = v10.borrow().len() as u64;
        let v38: Rc<str> = receipt_counts_28(&v24, v25, v26, v27, v28, v29, v30, v31, v32, v33, v34, v35, v37);
        if v25 {
            let v39: US9 = US9::US9_Failed;
            receipt_line_30(v39, v38, v2);
            let v40: bool = v35 > 0u64;
            if v40 {
                3i32
            } else {
                1i32
            }
        } else {
            let v42: US9 = US9::US9_Succeeded;
            receipt_line_30(v42, v38, v2);
            0i32
        }
    } else {
        let v44: Rc<str> = Rc::<str>::from(format!("{}{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("program rejected: "); } LIT.with(|lit| lit.clone()) }, v8));
        println!("{}", v44);
        let v45: US9 = US9::US9_Failed;
        let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("files 0"); } LIT.with(|lit| lit.clone()) };
        receipt_line_30(v45, v46, v2);
        2i32
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> i32 {
        read_run_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn matching_text_34(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v3: std::rc::Rc<Vec<std::rc::Rc<str>>> = files_matching_24(v0, v1, v2.clone(), v2.clone());
    let mut v4: Rc<str> = std::rc::Rc::<str>::from(v3.iter().map(|path| path.to_string()).collect::<Vec<_>>().join("
"));
    v4
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        matching_text_34(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v0);
        let mut v2: i32 = 1i32;
        first_rejection_3(v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn report_text_35(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_2(v1);
    let mut v3: std::rc::Rc<std::cell::RefCell<std::collections::BTreeSet<std::rc::Rc<str>>>> = std::rc::Rc::new(std::cell::RefCell::new(std::collections::BTreeSet::new()));
    let mut v4: i32 = 1i32;
    let (mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = empty_query_tally_14();
    let (mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28): (Rc<str>, bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) = queries_run_15(v0, v3, v2, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16);
    let v29: Rc<str> = std::rc::Rc::<str>::from(char::from(30u8).to_string());
    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v29));
    let v33: Rc<str> = if v18 {
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
        v31
    } else {
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v32
    };
    let v34: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v33));
    v34
}
fn closure4() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        report_text_35(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_read_run(v0: &str, v1: &str) -> i32 {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_do_read_matching(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_do_read_rejection(v0: &str) -> Rc<str> {
    closure3()(Rc::<str>::from(v0))
}
pub fn kit_do_read_report(v0: &str, v1: &str) -> Rc<str> {
    closure4()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
