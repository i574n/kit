#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use kit_do_model::{program_family, query_verb};
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(program_family, Rc<UH0>),
}
#[derive(Clone)]
enum US1 {
    US1_Some(program_family),
    US1_None,
}
#[derive(Clone)]
pub enum fact_verb {
    fact_verb_FactHash,
    fact_verb_FactEnv,
    fact_verb_FactExists,
    fact_verb_FactNearest,
    fact_verb_FactJson,
    fact_verb_FactLines,
    fact_verb_FactTable,
    fact_verb_FactDirs,
    fact_verb_FactTag,
    fact_verb_FactCount,
    fact_verb_FactDiff,
}
#[derive(Clone)]
pub enum run_verb {
    run_verb_StepRun,
    run_verb_StepErrors,
    run_verb_StepProcesses,
    run_verb_StepStop,
}
#[derive(Clone)]
pub enum write_verb {
    write_verb_FileAppend,
    write_verb_FileCopy,
    write_verb_FileMove,
    write_verb_FileWrite,
}
#[derive(Clone)]
pub enum tool_verb {
    tool_verb_ToolGit,
    tool_verb_ToolJob,
    tool_verb_ToolLab,
}
#[derive(Clone)]
enum US3 {
    US3_ReadVerb(query_verb),
    US3_FactVerb(fact_verb),
    US3_RunStepVerb(run_verb),
    US3_WriteVerb(write_verb),
    US3_ToolVerb(tool_verb),
}
#[derive(Clone)]
enum US2 {
    US2_Some(US3),
    US2_None,
}
#[derive(Clone)]
pub enum option_run_verb {
    option_run_verb_Some(run_verb),
    option_run_verb_None,
}
#[derive(Clone)]
pub enum option_write_verb {
    option_write_verb_Some(write_verb),
    option_write_verb_None,
}
#[derive(Clone)]
pub enum option_tool_verb {
    option_tool_verb_Some(tool_verb),
    option_tool_verb_None,
}
#[derive(Clone)]
enum US12 {
    US12_Some(query_verb),
    US12_None,
}
#[derive(Clone)]
pub enum option_fact_verb {
    option_fact_verb_Some(fact_verb),
    option_fact_verb_None,
}
impl fact_verb {
    pub fn tag(&self) -> i32 {
        match self {
            fact_verb::fact_verb_FactHash => 0,
            fact_verb::fact_verb_FactEnv => 1,
            fact_verb::fact_verb_FactExists => 2,
            fact_verb::fact_verb_FactNearest => 3,
            fact_verb::fact_verb_FactJson => 4,
            fact_verb::fact_verb_FactLines => 5,
            fact_verb::fact_verb_FactTable => 6,
            fact_verb::fact_verb_FactDirs => 7,
            fact_verb::fact_verb_FactTag => 8,
            fact_verb::fact_verb_FactCount => 9,
            fact_verb::fact_verb_FactDiff => 10,
        }
    }
}
impl run_verb {
    pub fn tag(&self) -> i32 {
        match self {
            run_verb::run_verb_StepRun => 0,
            run_verb::run_verb_StepErrors => 1,
            run_verb::run_verb_StepProcesses => 2,
            run_verb::run_verb_StepStop => 3,
        }
    }
}
impl write_verb {
    pub fn tag(&self) -> i32 {
        match self {
            write_verb::write_verb_FileAppend => 0,
            write_verb::write_verb_FileCopy => 1,
            write_verb::write_verb_FileMove => 2,
            write_verb::write_verb_FileWrite => 3,
        }
    }
}
impl tool_verb {
    pub fn tag(&self) -> i32 {
        match self {
            tool_verb::tool_verb_ToolGit => 0,
            tool_verb::tool_verb_ToolJob => 1,
            tool_verb::tool_verb_ToolLab => 2,
        }
    }
}
impl option_run_verb {
    pub fn tag(&self) -> i32 {
        match self {
            option_run_verb::option_run_verb_Some(..) => 0,
            option_run_verb::option_run_verb_None => 1,
        }
    }
}
impl option_write_verb {
    pub fn tag(&self) -> i32 {
        match self {
            option_write_verb::option_write_verb_Some(..) => 0,
            option_write_verb::option_write_verb_None => 1,
        }
    }
}
impl option_tool_verb {
    pub fn tag(&self) -> i32 {
        match self {
            option_tool_verb::option_tool_verb_Some(..) => 0,
            option_tool_verb::option_tool_verb_None => 1,
        }
    }
}
impl option_fact_verb {
    pub fn tag(&self) -> i32 {
        match self {
            option_fact_verb::option_fact_verb_Some(..) => 0,
            option_fact_verb::option_fact_verb_None => 1,
        }
    }
}
impl Default for fact_verb {
    fn default() -> Self {
        fact_verb::fact_verb_FactHash
    }
}
impl Default for run_verb {
    fn default() -> Self {
        run_verb::run_verb_StepRun
    }
}
impl Default for write_verb {
    fn default() -> Self {
        write_verb::write_verb_FileAppend
    }
}
impl Default for tool_verb {
    fn default() -> Self {
        tool_verb::tool_verb_ToolGit
    }
}
impl Default for option_run_verb {
    fn default() -> Self {
        option_run_verb::option_run_verb_Some(<run_verb>::default())
    }
}
impl Default for option_write_verb {
    fn default() -> Self {
        option_write_verb::option_write_verb_Some(<write_verb>::default())
    }
}
impl Default for option_tool_verb {
    fn default() -> Self {
        option_tool_verb::option_tool_verb_Some(<tool_verb>::default())
    }
}
impl Default for option_fact_verb {
    fn default() -> Self {
        option_fact_verb::option_fact_verb_Some(<fact_verb>::default())
    }
}
fn records_of_0(mut v0: Rc<str>) -> std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> {
    let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = std::rc::Rc::new((&*v0).split(char::from(30u8)).map(|record| record.split(char::from(31u8)).map(std::rc::Rc::<str>::from).collect::<Vec<_>>()).collect::<Vec<_>>());
    v1
}
fn first_seen_2(mut v0: Rc<UH0>, mut v1: bool, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: bool) -> program_family {
    loop {
        match Rc::unwrap_or_clone(v0) {
            UH0::UH0_Cons(v9, v10) => {
                let v9: program_family = v9;
                let v10: Rc<UH0> = v10;
                let v18: bool = match &v9 {
                    program_family::program_family_Edits => {
                        false
                    }
                    program_family::program_family_Facts => {
                        v6
                    }
                    program_family::program_family_Git => {
                        v4
                    }
                    program_family::program_family_Jobs => {
                        v2
                    }
                    program_family::program_family_Labs => {
                        v3
                    }
                    program_family::program_family_Reads => {
                        v7
                    }
                    program_family::program_family_Runs => {
                        v1
                    }
                    program_family::program_family_Writes => {
                        v5
                    }
                };
                if v18 {
                    return v9;
                } else {
                    v0 = v10;
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return program_family::program_family_Edits;
            }
        }
    }
}
fn field_at_4(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: Rc<str> = v0.get(v1 as usize).and_then(|fields| fields.get(v2 as usize)).cloned().unwrap_or_default();
    v3
}
fn field_index_6(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: Rc<str>) -> i32 {
    let v3: i32 = v0.get(v1 as usize).and_then(|fields| fields.iter().position(|field| field.len() > 1 && field.starts_with(char::from(119u8)) && &field[1..] == &*v2)).map(|index| index as i32).unwrap_or(-1);
    v3
}
fn run_named_9(mut v0: Rc<str>) -> option_run_verb {
    let v1: bool = &*v0 == "run";
    if v1 {
        let v2: run_verb = run_verb::run_verb_StepRun;
        option_run_verb::option_run_verb_Some(v2)
    } else {
        let v4: bool = &*v0 == "errors";
        if v4 {
            let v5: run_verb = run_verb::run_verb_StepErrors;
            option_run_verb::option_run_verb_Some(v5)
        } else {
            let v7: bool = &*v0 == "processes";
            if v7 {
                let v8: run_verb = run_verb::run_verb_StepProcesses;
                option_run_verb::option_run_verb_Some(v8)
            } else {
                let v10: bool = &*v0 == "stop";
                if v10 {
                    let v11: run_verb = run_verb::run_verb_StepStop;
                    option_run_verb::option_run_verb_Some(v11)
                } else {
                    option_run_verb::option_run_verb_None
                }
            }
        }
    }
}
fn write_named_10(mut v0: Rc<str>) -> option_write_verb {
    let v1: bool = &*v0 == "append";
    if v1 {
        let v2: write_verb = write_verb::write_verb_FileAppend;
        option_write_verb::option_write_verb_Some(v2)
    } else {
        let v4: bool = &*v0 == "copy";
        if v4 {
            let v5: write_verb = write_verb::write_verb_FileCopy;
            option_write_verb::option_write_verb_Some(v5)
        } else {
            let v7: bool = &*v0 == "move";
            if v7 {
                let v8: write_verb = write_verb::write_verb_FileMove;
                option_write_verb::option_write_verb_Some(v8)
            } else {
                let v10: bool = &*v0 == "write";
                if v10 {
                    let v11: write_verb = write_verb::write_verb_FileWrite;
                    option_write_verb::option_write_verb_Some(v11)
                } else {
                    option_write_verb::option_write_verb_None
                }
            }
        }
    }
}
fn tool_named_11(mut v0: Rc<str>) -> option_tool_verb {
    let v1: bool = &*v0 == "git";
    if v1 {
        let v2: tool_verb = tool_verb::tool_verb_ToolGit;
        option_tool_verb::option_tool_verb_Some(v2)
    } else {
        let v4: bool = &*v0 == "job";
        if v4 {
            let v5: tool_verb = tool_verb::tool_verb_ToolJob;
            option_tool_verb::option_tool_verb_Some(v5)
        } else {
            let v7: bool = &*v0 == "lab";
            if v7 {
                let v8: tool_verb = tool_verb::tool_verb_ToolLab;
                option_tool_verb::option_tool_verb_Some(v8)
            } else {
                option_tool_verb::option_tool_verb_None
            }
        }
    }
}
fn program_verb_tail_8(mut v0: Rc<str>) -> US2 {
    let v1: option_run_verb = run_named_9(v0.clone());
    match v1 {
        option_run_verb::option_run_verb_None => {
            let v5: option_write_verb = write_named_10(v0.clone());
            match v5 {
                option_write_verb::option_write_verb_None => {
                    let v9: option_tool_verb = tool_named_11(v0);
                    match v9 {
                        option_tool_verb::option_tool_verb_None => {
                            US2::US2_None
                        }
                        option_tool_verb::option_tool_verb_Some(v10) => {
                            let v10: tool_verb = v10;
                            let v11: US3 = US3::US3_ToolVerb(v10);
                            US2::US2_Some(v11)
                        }
                    }
                }
                option_write_verb::option_write_verb_Some(v6) => {
                    let v6: write_verb = v6;
                    let v7: US3 = US3::US3_WriteVerb(v6);
                    US2::US2_Some(v7)
                }
            }
        }
        option_run_verb::option_run_verb_Some(v2) => {
            let v2: run_verb = v2;
            let v3: US3 = US3::US3_RunStepVerb(v2);
            US2::US2_Some(v3)
        }
    }
}
fn fact_named_12(mut v0: Rc<str>) -> option_fact_verb {
    let v1: bool = &*v0 == "hash";
    if v1 {
        let v2: fact_verb = fact_verb::fact_verb_FactHash;
        option_fact_verb::option_fact_verb_Some(v2)
    } else {
        let v4: bool = &*v0 == "env";
        if v4 {
            let v5: fact_verb = fact_verb::fact_verb_FactEnv;
            option_fact_verb::option_fact_verb_Some(v5)
        } else {
            let v7: bool = &*v0 == "exists";
            if v7 {
                let v8: fact_verb = fact_verb::fact_verb_FactExists;
                option_fact_verb::option_fact_verb_Some(v8)
            } else {
                let v10: bool = &*v0 == "nearest";
                if v10 {
                    let v11: fact_verb = fact_verb::fact_verb_FactNearest;
                    option_fact_verb::option_fact_verb_Some(v11)
                } else {
                    let v13: bool = &*v0 == "json";
                    if v13 {
                        let v14: fact_verb = fact_verb::fact_verb_FactJson;
                        option_fact_verb::option_fact_verb_Some(v14)
                    } else {
                        let v16: bool = &*v0 == "lines";
                        if v16 {
                            let v17: fact_verb = fact_verb::fact_verb_FactLines;
                            option_fact_verb::option_fact_verb_Some(v17)
                        } else {
                            let v19: bool = &*v0 == "table";
                            if v19 {
                                let v20: fact_verb = fact_verb::fact_verb_FactTable;
                                option_fact_verb::option_fact_verb_Some(v20)
                            } else {
                                let v22: bool = &*v0 == "dirs";
                                if v22 {
                                    let v23: fact_verb = fact_verb::fact_verb_FactDirs;
                                    option_fact_verb::option_fact_verb_Some(v23)
                                } else {
                                    let v25: bool = &*v0 == "tag";
                                    if v25 {
                                        let v26: fact_verb = fact_verb::fact_verb_FactTag;
                                        option_fact_verb::option_fact_verb_Some(v26)
                                    } else {
                                        let v28: bool = &*v0 == "count";
                                        if v28 {
                                            let v29: fact_verb = fact_verb::fact_verb_FactCount;
                                            option_fact_verb::option_fact_verb_Some(v29)
                                        } else {
                                            let v31: bool = &*v0 == "diff";
                                            if v31 {
                                                let v32: fact_verb = fact_verb::fact_verb_FactDiff;
                                                option_fact_verb::option_fact_verb_Some(v32)
                                            } else {
                                                option_fact_verb::option_fact_verb_None
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn program_verb_named_7(mut v0: Rc<str>) -> US2 {
    let v1: query_verb = query_verb::query_verb_Search;
    let v2: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v1);
    let v3: bool = &*v0 == &*v2;
    let v37: US12 = if v3 {
        let v4: query_verb = query_verb::query_verb_Search;
        US12::US12_Some(v4)
    } else {
        let v6: query_verb = query_verb::query_verb_Read;
        let v7: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v6);
        let v8: bool = &*v0 == &*v7;
        if v8 {
            let v9: query_verb = query_verb::query_verb_Read;
            US12::US12_Some(v9)
        } else {
            let v11: query_verb = query_verb::query_verb_Files;
            let v12: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v11);
            let v13: bool = &*v0 == &*v12;
            if v13 {
                let v14: query_verb = query_verb::query_verb_Files;
                US12::US12_Some(v14)
            } else {
                let v16: query_verb = query_verb::query_verb_Head;
                let v17: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v16);
                let v18: bool = &*v0 == &*v17;
                if v18 {
                    let v19: query_verb = query_verb::query_verb_Head;
                    US12::US12_Some(v19)
                } else {
                    let v21: query_verb = query_verb::query_verb_Tail;
                    let v22: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v21);
                    let v23: bool = &*v0 == &*v22;
                    if v23 {
                        let v24: query_verb = query_verb::query_verb_Tail;
                        US12::US12_Some(v24)
                    } else {
                        let v26: query_verb = query_verb::query_verb_Wait;
                        let v27: Rc<str> = kit_do_model::kit_do_model_query_verb_word(v26);
                        let v28: bool = &*v0 == &*v27;
                        if v28 {
                            let v29: query_verb = query_verb::query_verb_Wait;
                            US12::US12_Some(v29)
                        } else {
                            US12::US12_None
                        }
                    }
                }
            }
        }
    };
    match v37 {
        US12::US12_None => {
            let v41: option_fact_verb = fact_named_12(v0.clone());
            match v41 {
                option_fact_verb::option_fact_verb_None => {
                    program_verb_tail_8(v0)
                }
                option_fact_verb::option_fact_verb_Some(v42) => {
                    let v42: fact_verb = v42;
                    let v43: US3 = US3::US3_FactVerb(v42);
                    US2::US2_Some(v43)
                }
            }
        }
        US12::US12_Some(v38) => {
            let v38: query_verb = v38;
            let v39: US3 = US3::US3_ReadVerb(v38);
            US2::US2_Some(v39)
        }
    }
}
fn query_family_5(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32) -> US1 {
    let v2: i32 = 1i32;
    let v3: Rc<str> = field_at_4(v0.clone(), v1, v2);
    let v4: bool = &*v3 == "files";
    let v8: bool = if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
        let v6: i32 = field_index_6(v0, v1, v5);
        let v7: bool = v6 >= 0i32;
        v7
    } else {
        false
    };
    if v8 {
        let v9: program_family = program_family::program_family_Facts;
        US1::US1_Some(v9)
    } else {
        let v11: US2 = program_verb_named_7(v3);
        match v11 {
            US2::US2_None => {
                US1::US1_None
            }
            US2::US2_Some(v12) => {
                let v12: US3 = v12;
                let v32: program_family = match v12 {
                    US3::US3_FactVerb(v15) => {
                        let v15: fact_verb = v15;
                        program_family::program_family_Facts
                    }
                    US3::US3_ReadVerb(v13) => {
                        let v13: query_verb = v13;
                        program_family::program_family_Reads
                    }
                    US3::US3_RunStepVerb(v17) => {
                        let v17: run_verb = v17;
                        program_family::program_family_Runs
                    }
                    US3::US3_ToolVerb(v21) => {
                        let v21: tool_verb = v21;
                        match v21 {
                            tool_verb::tool_verb_ToolGit => {
                                program_family::program_family_Git
                            }
                            tool_verb::tool_verb_ToolJob => {
                                program_family::program_family_Jobs
                            }
                            tool_verb::tool_verb_ToolLab => {
                                program_family::program_family_Labs
                            }
                        }
                    }
                    US3::US3_WriteVerb(v19) => {
                        let v19: write_verb = v19;
                        program_family::program_family_Writes
                    }
                };
                US1::US1_Some(v32)
            }
        }
    }
}
fn families_of_3(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>, mut v1: i32, mut v2: bool, mut v3: bool, mut v4: bool, mut v5: bool, mut v6: bool, mut v7: bool, mut v8: bool) -> (bool, bool, bool, bool, bool, bool, bool) {
    let v9: i32 = v0.len() as i32;
    let v10: bool = v1 >= v9;
    let (mut v117, mut v118, mut v119, mut v120, mut v121, mut v122, mut v123): (bool, bool, bool, bool, bool, bool, bool) = if v10 {
        (v2, v3, v4, v5, v6, v7, v8)
    } else {
        let v11: i32 = 0i32;
        let v12: Rc<str> = field_at_4(v0.clone(), v1, v11);
        let v13: bool = &*v12 != "query";
        let (mut v110, mut v111, mut v112, mut v113, mut v114, mut v115, mut v116): (bool, bool, bool, bool, bool, bool, bool) = if v13 {
            let v14: i32 = v1.wrapping_add(1i32);
            let (mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21): (bool, bool, bool, bool, bool, bool, bool) = families_of_3(v0, v14, v2, v3, v4, v5, v6, v7, v8);
            (v15, v16, v17, v18, v19, v20, v21)
        } else {
            let v22: US1 = query_family_5(v0.clone(), v1);
            let (mut v103, mut v104, mut v105, mut v106, mut v107, mut v108, mut v109): (bool, bool, bool, bool, bool, bool, bool) = match &v22 {
                US1::US1_Some(v23) => {
                    let v23: program_family = v23.clone();
                    let v24: i32 = v1.wrapping_add(1i32);
                    let (mut v74, mut v75, mut v76, mut v77, mut v78, mut v79, mut v80): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                        program_family::program_family_Runs => {
                            (true, v3, v4, v5, v6, v7, v8)
                        }
                        _ => {
                            let (mut v67, mut v68, mut v69, mut v70, mut v71, mut v72, mut v73): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                program_family::program_family_Jobs => {
                                    (v2, true, v4, v5, v6, v7, v8)
                                }
                                _ => {
                                    let (mut v60, mut v61, mut v62, mut v63, mut v64, mut v65, mut v66): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                        program_family::program_family_Labs => {
                                            (v2, v3, true, v5, v6, v7, v8)
                                        }
                                        _ => {
                                            let (mut v53, mut v54, mut v55, mut v56, mut v57, mut v58, mut v59): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                program_family::program_family_Git => {
                                                    (v2, v3, v4, true, v6, v7, v8)
                                                }
                                                _ => {
                                                    let (mut v46, mut v47, mut v48, mut v49, mut v50, mut v51, mut v52): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                        program_family::program_family_Writes => {
                                                            (v2, v3, v4, v5, true, v7, v8)
                                                        }
                                                        _ => {
                                                            let (mut v39, mut v40, mut v41, mut v42, mut v43, mut v44, mut v45): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                                program_family::program_family_Facts => {
                                                                    (v2, v3, v4, v5, v6, true, v8)
                                                                }
                                                                _ => {
                                                                    let (mut v32, mut v33, mut v34, mut v35, mut v36, mut v37, mut v38): (bool, bool, bool, bool, bool, bool, bool) = match &v23 {
                                                                        program_family::program_family_Reads => {
                                                                            (v2, v3, v4, v5, v6, v7, true)
                                                                        }
                                                                        _ => {
                                                                            let (mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31): (bool, bool, bool, bool, bool, bool, bool) = match v23 {
                                                                                program_family::program_family_Edits => {
                                                                                    (v2, v3, v4, v5, v6, v7, v8)
                                                                                }
                                                                                _ => unreachable!(),
                                                                            };
                                                                            (v25, v26, v27, v28, v29, v30, v31)
                                                                        }
                                                                    };
                                                                    (v32, v33, v34, v35, v36, v37, v38)
                                                                }
                                                            };
                                                            (v39, v40, v41, v42, v43, v44, v45)
                                                        }
                                                    };
                                                    (v46, v47, v48, v49, v50, v51, v52)
                                                }
                                            };
                                            (v53, v54, v55, v56, v57, v58, v59)
                                        }
                                    };
                                    (v60, v61, v62, v63, v64, v65, v66)
                                }
                            };
                            (v67, v68, v69, v70, v71, v72, v73)
                        }
                    };
                    families_of_3(v0, v24, v74, v75, v76, v77, v78, v79, v80)
                }
                _ => {
                    let (mut v96, mut v97, mut v98, mut v99, mut v100, mut v101, mut v102): (bool, bool, bool, bool, bool, bool, bool) = match v22 {
                        US1::US1_None => {
                            let v88: i32 = v1.wrapping_add(1i32);
                            families_of_3(v0, v88, v2, v3, v4, v5, v6, v7, v8)
                        }
                        _ => unreachable!(),
                    };
                    (v96, v97, v98, v99, v100, v101, v102)
                }
            };
            (v103, v104, v105, v106, v107, v108, v109)
        };
        (v110, v111, v112, v113, v114, v115, v116)
    };
    (v117, v118, v119, v120, v121, v122, v123)
}
fn records_family_1(mut v0: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>>) -> program_family {
    let v1: program_family = program_family::program_family_Runs;
    let v2: program_family = program_family::program_family_Jobs;
    let v3: program_family = program_family::program_family_Labs;
    let v4: program_family = program_family::program_family_Git;
    let v5: program_family = program_family::program_family_Writes;
    let v6: program_family = program_family::program_family_Facts;
    let v7: program_family = program_family::program_family_Reads;
    let v8: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v8));
    let v10: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v9));
    let v11: Rc<UH0> = Rc::new(UH0::UH0_Cons(v5, v10));
    let v12: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4, v11));
    let v13: Rc<UH0> = Rc::new(UH0::UH0_Cons(v3, v12));
    let v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2, v13));
    let v15: Rc<UH0> = Rc::new(UH0::UH0_Cons(v1, v14));
    let v16: i32 = 1i32;
    let v17: bool = false;
    let v18: bool = false;
    let v19: bool = false;
    let v20: bool = false;
    let v21: bool = false;
    let v22: bool = false;
    let v23: bool = false;
    let (mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30): (bool, bool, bool, bool, bool, bool, bool) = families_of_3(v0, v16, v17, v18, v19, v20, v21, v22, v23);
    first_seen_2(v15, v24, v25, v26, v27, v28, v29, v30)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: std::rc::Rc<Vec<Vec<std::rc::Rc<str>>>> = records_of_0(v0);
        let v2: program_family = records_family_1(v1);
        let v3: Rc<str> = kit_do_model::kit_do_model_program_family_word(v2);
        v3
    }); } CLOSURE.with(|closure| closure.clone())
}
fn do_verb_family_13(mut v0: Rc<str>) -> Rc<str> {
    let v1: US2 = program_verb_named_7(v0);
    match v1 {
        US2::US2_None => {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v24
        }
        US2::US2_Some(v2) => {
            let v2: US3 = v2;
            let v22: program_family = match v2 {
                US3::US3_FactVerb(v5) => {
                    let v5: fact_verb = v5;
                    program_family::program_family_Facts
                }
                US3::US3_ReadVerb(v3) => {
                    let v3: query_verb = v3;
                    program_family::program_family_Reads
                }
                US3::US3_RunStepVerb(v7) => {
                    let v7: run_verb = v7;
                    program_family::program_family_Runs
                }
                US3::US3_ToolVerb(v11) => {
                    let v11: tool_verb = v11;
                    match v11 {
                        tool_verb::tool_verb_ToolGit => {
                            program_family::program_family_Git
                        }
                        tool_verb::tool_verb_ToolJob => {
                            program_family::program_family_Jobs
                        }
                        tool_verb::tool_verb_ToolLab => {
                            program_family::program_family_Labs
                        }
                    }
                }
                US3::US3_WriteVerb(v9) => {
                    let v9: write_verb = v9;
                    program_family::program_family_Writes
                }
            };
            let v23: Rc<str> = kit_do_model::kit_do_model_program_family_word(v22);
            v23
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        do_verb_family_13(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> option_fact_verb> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> option_fact_verb> = Rc::new(move |mut v0: Rc<str>| -> option_fact_verb {
        fact_named_12(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn fact_word_shown_14(mut v0: fact_verb) -> Rc<str> {
    match v0 {
        fact_verb::fact_verb_FactCount => {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("count"); } LIT.with(|lit| lit.clone()) };
            v10
        }
        fact_verb::fact_verb_FactDiff => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("diff"); } LIT.with(|lit| lit.clone()) };
            v11
        }
        fact_verb::fact_verb_FactDirs => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dirs"); } LIT.with(|lit| lit.clone()) };
            v8
        }
        fact_verb::fact_verb_FactEnv => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        fact_verb::fact_verb_FactExists => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exists"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        fact_verb::fact_verb_FactHash => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("hash"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        fact_verb::fact_verb_FactJson => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("json"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        fact_verb::fact_verb_FactLines => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lines"); } LIT.with(|lit| lit.clone()) };
            v6
        }
        fact_verb::fact_verb_FactNearest => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("nearest"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        fact_verb::fact_verb_FactTable => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table"); } LIT.with(|lit| lit.clone()) };
            v7
        }
        fact_verb::fact_verb_FactTag => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("tag"); } LIT.with(|lit| lit.clone()) };
            v9
        }
    }
}
fn closure3() -> Rc<dyn Fn(fact_verb) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(fact_verb) -> Rc<str>> = Rc::new(move |mut v0: fact_verb| -> Rc<str> {
        fact_word_shown_14(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> option_run_verb> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> option_run_verb> = Rc::new(move |mut v0: Rc<str>| -> option_run_verb {
        run_named_9(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> option_write_verb> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> option_write_verb> = Rc::new(move |mut v0: Rc<str>| -> option_write_verb {
        write_named_10(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> option_tool_verb> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> option_tool_verb> = Rc::new(move |mut v0: Rc<str>| -> option_tool_verb {
        tool_named_11(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_do_program_family(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
pub fn kit_do_verb_family(v0: &str) -> Rc<str> {
    closure1()(Rc::<str>::from(v0))
}
pub fn kit_do_fact_named(v0: &str) -> option_fact_verb {
    closure2()(Rc::<str>::from(v0))
}
pub fn kit_do_fact_word(v0: fact_verb) -> Rc<str> {
    closure3()(v0)
}
pub fn kit_do_run_named(v0: &str) -> option_run_verb {
    closure4()(Rc::<str>::from(v0))
}
pub fn kit_do_write_named(v0: &str) -> option_write_verb {
    closure5()(Rc::<str>::from(v0))
}
pub fn kit_do_tool_named(v0: &str) -> option_tool_verb {
    closure6()(Rc::<str>::from(v0))
}
