#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone)]
enum US1 {
    US1_AgileInit,
    US1_AgileBegin,
    US1_AgileLease,
    US1_AgileStatus,
    US1_AgileHandoff,
    US1_AgileList,
    US1_AgileCheck,
    US1_AgileRecord,
    US1_AgileSet,
    US1_AgileAdd,
    US1_AgileEdit,
    US1_AgileShow,
    US1_AgilePlan,
    US1_AgileMigrateTags,
    US1_AgileNote,
    US1_AgileNoteEdit,
    US1_AgileNotes,
    US1_AgileRename,
    US1_AgileTodos,
    US1_AgileNext,
    US1_AgileRoots,
    US1_AgileLinks,
    US1_AgileViews,
    US1_AgileView,
    US1_AgileViewAdd,
}
#[derive(Clone)]
enum US0 {
    US0_AgileKnown(US1),
    US0_AgileUnknownName,
}
fn agile_subcommand_of_1(mut v0: Rc<str>) -> US0 {
    let v1: bool = &*v0 == "init";
    let v5: US0 = if v1 {
        let v2: US1 = US1::US1_AgileInit;
        US0::US0_AgileKnown(v2)
    } else {
        US0::US0_AgileUnknownName
    };
    let v6: bool = &*v0 == "begin";
    let v9: US0 = if v6 {
        let v7: US1 = US1::US1_AgileBegin;
        US0::US0_AgileKnown(v7)
    } else {
        v5
    };
    let v10: bool = &*v0 == "lease";
    let v13: US0 = if v10 {
        let v11: US1 = US1::US1_AgileLease;
        US0::US0_AgileKnown(v11)
    } else {
        v9
    };
    let v14: bool = &*v0 == "status";
    let v17: US0 = if v14 {
        let v15: US1 = US1::US1_AgileStatus;
        US0::US0_AgileKnown(v15)
    } else {
        v13
    };
    let v18: bool = &*v0 == "handoff";
    let v21: US0 = if v18 {
        let v19: US1 = US1::US1_AgileHandoff;
        US0::US0_AgileKnown(v19)
    } else {
        v17
    };
    let v22: bool = &*v0 == "list";
    let v25: US0 = if v22 {
        let v23: US1 = US1::US1_AgileList;
        US0::US0_AgileKnown(v23)
    } else {
        v21
    };
    let v26: bool = &*v0 == "check";
    let v29: US0 = if v26 {
        let v27: US1 = US1::US1_AgileCheck;
        US0::US0_AgileKnown(v27)
    } else {
        v25
    };
    let v30: bool = &*v0 == "record";
    let v33: US0 = if v30 {
        let v31: US1 = US1::US1_AgileRecord;
        US0::US0_AgileKnown(v31)
    } else {
        v29
    };
    let v34: bool = &*v0 == "set";
    let v37: US0 = if v34 {
        let v35: US1 = US1::US1_AgileSet;
        US0::US0_AgileKnown(v35)
    } else {
        v33
    };
    let v38: bool = &*v0 == "add";
    let v41: US0 = if v38 {
        let v39: US1 = US1::US1_AgileAdd;
        US0::US0_AgileKnown(v39)
    } else {
        v37
    };
    let v42: bool = &*v0 == "edit";
    let v45: US0 = if v42 {
        let v43: US1 = US1::US1_AgileEdit;
        US0::US0_AgileKnown(v43)
    } else {
        v41
    };
    let v46: bool = &*v0 == "show";
    let v49: US0 = if v46 {
        let v47: US1 = US1::US1_AgileShow;
        US0::US0_AgileKnown(v47)
    } else {
        v45
    };
    let v50: bool = &*v0 == "plan";
    let v53: US0 = if v50 {
        let v51: US1 = US1::US1_AgilePlan;
        US0::US0_AgileKnown(v51)
    } else {
        v49
    };
    let v54: bool = &*v0 == "migrate-tags";
    let v57: US0 = if v54 {
        let v55: US1 = US1::US1_AgileMigrateTags;
        US0::US0_AgileKnown(v55)
    } else {
        v53
    };
    let v58: bool = &*v0 == "note";
    let v61: US0 = if v58 {
        let v59: US1 = US1::US1_AgileNote;
        US0::US0_AgileKnown(v59)
    } else {
        v57
    };
    let v62: bool = &*v0 == "note-edit";
    let v65: US0 = if v62 {
        let v63: US1 = US1::US1_AgileNoteEdit;
        US0::US0_AgileKnown(v63)
    } else {
        v61
    };
    let v66: bool = &*v0 == "notes";
    let v69: US0 = if v66 {
        let v67: US1 = US1::US1_AgileNotes;
        US0::US0_AgileKnown(v67)
    } else {
        v65
    };
    let v70: bool = &*v0 == "rename";
    let v73: US0 = if v70 {
        let v71: US1 = US1::US1_AgileRename;
        US0::US0_AgileKnown(v71)
    } else {
        v69
    };
    let v74: bool = &*v0 == "todos";
    let v77: US0 = if v74 {
        let v75: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v75)
    } else {
        v73
    };
    let v78: bool = &*v0 == "next";
    let v81: US0 = if v78 {
        let v79: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v79)
    } else {
        v77
    };
    let v82: bool = &*v0 == "roots";
    let v85: US0 = if v82 {
        let v83: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v83)
    } else {
        v81
    };
    let v86: bool = &*v0 == "links";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "views";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "view";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "view-add";
    if v98 {
        let v99: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    }
}
fn root_flag_refusal_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = agile_subcommand_of_1(v0.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let v3: US1 = v3.clone();
            let (mut v102, mut v103): (Rc<str>, Rc<str>) = match &v3 {
                US1::US1_AgileInit => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                    (v4.clone(), v5.clone())
                }
                _ => {
                    let (mut v100, mut v101): (Rc<str>, Rc<str>) = match &v3 {
                        US1::US1_AgileBegin => {
                            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
                            (v6.clone(), v7.clone())
                        }
                        _ => {
                            let (mut v98, mut v99): (Rc<str>, Rc<str>) = match &v3 {
                                US1::US1_AgileLease => {
                                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
                                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                    (v8.clone(), v9.clone())
                                }
                                _ => {
                                    let (mut v96, mut v97): (Rc<str>, Rc<str>) = match &v3 {
                                        US1::US1_AgileStatus => {
                                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                            (v10.clone(), v11.clone())
                                        }
                                        _ => {
                                            let (mut v94, mut v95): (Rc<str>, Rc<str>) = match &v3 {
                                                US1::US1_AgileHandoff => {
                                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
                                                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                    (v12.clone(), v13.clone())
                                                }
                                                _ => {
                                                    let (mut v92, mut v93): (Rc<str>, Rc<str>) = match &v3 {
                                                        US1::US1_AgileList => {
                                                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                                                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
                                                            (v14.clone(), v15.clone())
                                                        }
                                                        _ => {
                                                            let (mut v90, mut v91): (Rc<str>, Rc<str>) = match &v3 {
                                                                US1::US1_AgileCheck => {
                                                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
                                                                    (v16.clone(), v17.clone())
                                                                }
                                                                _ => {
                                                                    let (mut v88, mut v89): (Rc<str>, Rc<str>) = match &v3 {
                                                                        US1::US1_AgileRecord => {
                                                                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
                                                                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                            (v18.clone(), v19.clone())
                                                                        }
                                                                        _ => {
                                                                            let (mut v86, mut v87): (Rc<str>, Rc<str>) = match &v3 {
                                                                                US1::US1_AgileSet => {
                                                                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                                                    (v20.clone(), v21.clone())
                                                                                }
                                                                                _ => {
                                                                                    let (mut v84, mut v85): (Rc<str>, Rc<str>) = match &v3 {
                                                                                        US1::US1_AgileAdd => {
                                                                                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                                                                                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                            (v22.clone(), v23.clone())
                                                                                        }
                                                                                        _ => {
                                                                                            let (mut v82, mut v83): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                US1::US1_AgileEdit => {
                                                                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
                                                                                                    (v24.clone(), v25.clone())
                                                                                                }
                                                                                                _ => {
                                                                                                    let (mut v80, mut v81): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                        US1::US1_AgileShow => {
                                                                                                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                                                                                                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                            (v26.clone(), v27.clone())
                                                                                                        }
                                                                                                        _ => {
                                                                                                            let (mut v78, mut v79): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                US1::US1_AgilePlan => {
                                                                                                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    (v28.clone(), v29.clone())
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    let (mut v76, mut v77): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                        US1::US1_AgileMigrateTags => {
                                                                                                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            (v30.clone(), v31.clone())
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            let (mut v74, mut v75): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                US1::US1_AgileNote => {
                                                                                                                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    (v32.clone(), v33.clone())
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    let (mut v72, mut v73): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                        US1::US1_AgileNoteEdit => {
                                                                                                                                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            (v34.clone(), v35.clone())
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            let (mut v70, mut v71): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                US1::US1_AgileNotes => {
                                                                                                                                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    (v36.clone(), v37.clone())
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    let (mut v68, mut v69): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                        US1::US1_AgileRename => {
                                                                                                                                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            (v38.clone(), v39.clone())
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            let (mut v66, mut v67): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                US1::US1_AgileTodos => {
                                                                                                                                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    (v40.clone(), v41.clone())
                                                                                                                                                                }
                                                                                                                                                                _ => {
                                                                                                                                                                    let (mut v64, mut v65): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                        US1::US1_AgileNext => {
                                                                                                                                                                            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            (v42.clone(), v43.clone())
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            let (mut v62, mut v63): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                US1::US1_AgileRoots => {
                                                                                                                                                                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    (v44.clone(), v45.clone())
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    let (mut v60, mut v61): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                        US1::US1_AgileLinks => {
                                                                                                                                                                                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            (v46.clone(), v47.clone())
                                                                                                                                                                                        }
                                                                                                                                                                                        _ => {
                                                                                                                                                                                            let (mut v58, mut v59): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                US1::US1_AgileViews => {
                                                                                                                                                                                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    (v48.clone(), v49.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                                _ => {
                                                                                                                                                                                                    let (mut v56, mut v57): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                        US1::US1_AgileView => {
                                                                                                                                                                                                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            (v50.clone(), v51.clone())
                                                                                                                                                                                                        }
                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                            let (mut v54, mut v55): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                US1::US1_AgileViewAdd => {
                                                                                                                                                                                                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    (v52.clone(), v53.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                                _ => unreachable!(),
                                                                                                                                                                                                            };
                                                                                                                                                                                                            (v54.clone(), v55.clone())
                                                                                                                                                                                                        }
                                                                                                                                                                                                    };
                                                                                                                                                                                                    (v56.clone(), v57.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            (v58.clone(), v59.clone())
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    (v60.clone(), v61.clone())
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            (v62.clone(), v63.clone())
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    (v64.clone(), v65.clone())
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            (v66.clone(), v67.clone())
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    (v68.clone(), v69.clone())
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            (v70.clone(), v71.clone())
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    (v72.clone(), v73.clone())
                                                                                                                                }
                                                                                                                            };
                                                                                                                            (v74.clone(), v75.clone())
                                                                                                                        }
                                                                                                                    };
                                                                                                                    (v76.clone(), v77.clone())
                                                                                                                }
                                                                                                            };
                                                                                                            (v78.clone(), v79.clone())
                                                                                                        }
                                                                                                    };
                                                                                                    (v80.clone(), v81.clone())
                                                                                                }
                                                                                            };
                                                                                            (v82.clone(), v83.clone())
                                                                                        }
                                                                                    };
                                                                                    (v84.clone(), v85.clone())
                                                                                }
                                                                            };
                                                                            (v86.clone(), v87.clone())
                                                                        }
                                                                    };
                                                                    (v88.clone(), v89.clone())
                                                                }
                                                            };
                                                            (v90.clone(), v91.clone())
                                                        }
                                                    };
                                                    (v92.clone(), v93.clone())
                                                }
                                            };
                                            (v94.clone(), v95.clone())
                                        }
                                    };
                                    (v96.clone(), v97.clone())
                                }
                            };
                            (v98.clone(), v99.clone())
                        }
                    };
                    (v100.clone(), v101.clone())
                }
            };
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v104, v102));
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" takes <root> before any flag, got "); } LIT.with(|lit| lit.clone()) };
            let v108: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v105, v106)), v1));
            let v109: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile "); } LIT.with(|lit| lit.clone()) };
            let v111: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v108, v109)), v102));
            let v112: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v114: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v111, v112)), v103));
            v114.clone()
        }
        US0::US0_AgileUnknownName => {
            let v115: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v115.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        root_flag_refusal_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn command_more_usages_2() -> Rc<str> {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)"); } LIT.with(|lit| lit.clone()) };
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\n"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("repo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)"); } LIT.with(|lit| lit.clone()) };
    let v5: bool = &*v4 == "";
    let (mut v10, mut v11): (Rc<str>, Rc<str>) = if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>...) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        (v6.clone(), v6.clone())
    } else {
        let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)\n"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("table: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>...) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("job: start [--heavy] <label> <timeout-ms> <program> [args...] | run [--heavy] <label> <timeout-ms> <program> [args...] (attached: supervises in this process, ends with its caller, exits with the job's code; long runs from reapable agent shells use start + a background wait) | status <id> | wait <id> [max-ms] (default: until the job settles; a queued --heavy job's timeout starts when it is admitted) | list [count] | tail <id> [lines] | stop <id> (stops the job's own process tree)\nrepo: state <root> [<root>...] | affected <src> <package>... (Cargo crates whose package.spiproj graph reaches a changed package) | accept-review <before> <after> (passes when every changed line differs in numeric literals only)\ntable: query <file> [--where col=value|col!=value|col>value|col>=value|col<value|col<=value]... [--cols a,b] [--group col] [--mean|--sd|--sum|--min|--max|--count a,b]... [--limit n] [--quiet] | diff <left> <right> --key a,b [--cols x,y] [--latest-by col] [--limit n] | logs <file|dir>... [--glob *.log] [--since regex] (--count name=regex | --count-by name=regex | --sum name=regex | --last name=regex | --field name=regex-on-the-file-path)... | --values name=regex alone (one row per match) [--out file.tsv] [--quiet] | test <file.tsv> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <column> [--arms a[,b] [--group <column>]] [--prediction x] [--confidence 0.95] [--where ...]... | sequences <file|dir>... --time <regex (?<ms>-?\\d+)> (--interval name=A [--between B] [--modulo ms] | --window name=A --from M --ms lo..hi | --order name=A,B | --latency name=A --to B | [--section-on <regex>] [--carry <regex>] (--first|--last) name=<regex (?<id>)(?<value>)>...) [--out file.tsv]"); } LIT.with(|lit| lit.clone()) };
        (v9.clone(), v8.clone())
    };
    let v12: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v11;
    let (mut v20, mut v21): (Rc<str>, Rc<str>) = if v12 {
        (v10.clone(), v11.clone())
    } else {
        let v13: bool = &*v10 == "";
        let (mut v18, mut v19): (Rc<str>, Rc<str>) = if v13 {
            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v14.clone(), v14.clone())
        } else {
            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v1));
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
            (v17.clone(), v16.clone())
        };
        (v18.clone(), v19.clone())
    };
    let v22: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v21;
    let (mut v30, mut v31): (Rc<str>, Rc<str>) = if v22 {
        (v20.clone(), v21.clone())
    } else {
        let v23: bool = &*v20 == "";
        let (mut v28, mut v29): (Rc<str>, Rc<str>) = if v23 {
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v24.clone(), v24.clone())
        } else {
            let v25: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v1));
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = Rc::<str>::from(format!("{}{}", v25, v26));
            (v27.clone(), v26.clone())
        };
        (v28.clone(), v29.clone())
    };
    let v32: bool = "read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]" == &*v31;
    let (mut v40, mut v41): (Rc<str>, Rc<str>) = if v32 {
        (v30.clone(), v31.clone())
    } else {
        let v33: bool = &*v30 == "";
        let (mut v38, mut v39): (Rc<str>, Rc<str>) = if v33 {
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            (v34.clone(), v34.clone())
        } else {
            let v35: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v1));
            let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("read <path|glob>[:from-to|:-last]... [--max-lines n] [--json] | lines <path|glob>... | grep <root> <needle> [--glob pattern] [--context n] [--max n] [--ignore-case] [--regex] [--json]"); } LIT.with(|lit| lit.clone()) };
            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
            (v37.clone(), v36.clone())
        };
        (v38.clone(), v39.clone())
    };
    let v42: bool = "do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>" == &*v41;
    let (mut v50, mut v51): (Rc<str>, Rc<str>) = if v42 {
        (v40.clone(), v41.clone())
    } else {
        let v43: bool = &*v40 == "";
        let (mut v48, mut v49): (Rc<str>, Rc<str>) = if v43 {
            let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>"); } LIT.with(|lit| lit.clone()) };
            (v44.clone(), v44.clone())
        } else {
            let v45: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v1));
            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("do <root> [--dry-run] '<program>' | do <root> [--dry-run] - (stdin); edit <path|glob> [dry-run] [expect files n replacements n] / replace [pattern] once|all|<n> times \"old\" with \"new\" / patch <<<hunks>>> / show <path>; search [literal] \"re\" in <glob> [include a,b] [exclude glob] [context n[,m]] [first n per file] [paths] / read <path> around \"text\" [context n[,m]] / files <glob> / head <glob> n / tail <glob> n / wait <file> ~ \"re\" [timeout 5s]; hash <a,b,...> / env <A,B> / exists <a,b> / nearest \"re\" before lines n,m in <file> / json <file> get a.b,c.length / lines <file> count by \"(re)\" / files <glob> count by extension / table <file.tsv> [where col = \"v\"|col <> \"v\"] count by a,b / dirs <dir/*> having a and b + tag <name> = <file> ~ \"re\" [and|or|not ...] + count by <name> / diff <a.tsv> <b.tsv> by k1,k2 [on c1,c2]; run <cmd> [timeout 10s] [until output ~ \"re\" then stop] [background as name] / tail output n / search \"re\" in output / errors in output by \"(re)\" / processes where command ~ \"re\" / stop <name>; append <file> \"line {clock:HH:mm}\" / write <file> [overwrite] <<< text >>> / copy <glob> to <dir> [except *.tmp,...] / move <path> to <path> (no overwrite; absolute paths reach other roots), then read statements; git init|commit <repo> all \"msg\" [at <iso>] / git history <repo> \"text\" in <file> / git status a, b group by state / git log <repo> [since <date>] count by day / search \"re\" in <glob> with git status; job start <cmd> as <name> timeout 30s [notify] / job wait <name> [timeout 5m] / job stop <name>; lab audit <dir> against <backlog>"); } LIT.with(|lit| lit.clone()) };
            let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v45, v46));
            (v47.clone(), v46.clone())
        };
        (v48.clone(), v49.clone())
    };
    let v52: bool = "ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)" == &*v51;
    let (mut v60, mut v61): (Rc<str>, Rc<str>) = if v52 {
        (v50.clone(), v51.clone())
    } else {
        let v53: bool = &*v50 == "";
        let (mut v58, mut v59): (Rc<str>, Rc<str>) = if v53 {
            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            (v54.clone(), v54.clone())
        } else {
            let v55: Rc<str> = Rc::<str>::from(format!("{}{}", v50, v1));
            let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
            (v57.clone(), v56.clone())
        };
        (v58.clone(), v59.clone())
    };
    let v62: bool = "ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)" == &*v61;
    let (mut v70, mut v71): (Rc<str>, Rc<str>) = if v62 {
        (v60.clone(), v61.clone())
    } else {
        let v63: bool = &*v60 == "";
        let (mut v68, mut v69): (Rc<str>, Rc<str>) = if v63 {
            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            (v64.clone(), v64.clone())
        } else {
            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v60, v1));
            let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps [regex] [--tree] [--mine] | kill <pid>... [--tree] [--mine [--theirs <job-id>]] (never this kit or its calling shells)"); } LIT.with(|lit| lit.clone()) };
            let v67: Rc<str> = Rc::<str>::from(format!("{}{}", v65, v66));
            (v67.clone(), v66.clone())
        };
        (v68.clone(), v69.clone())
    };
    let v72: bool = "standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)" == &*v71;
    let (mut v80, mut v81): (Rc<str>, Rc<str>) = if v72 {
        (v70.clone(), v71.clone())
    } else {
        let v73: bool = &*v70 == "";
        let (mut v78, mut v79): (Rc<str>, Rc<str>) = if v73 {
            let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)"); } LIT.with(|lit| lit.clone()) };
            (v74.clone(), v74.clone())
        } else {
            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v70, v1));
            let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standards: <package-dir>... [--json] (rules from the package's standards.spi rows, else no-comment-lines, line-width 120, lf-only, test-targets; exit 1 on findings) | --census <dir>... | --baseline <file.spi> <dir>... | --ratchet <file.spi> <dir>... (typefeatureset and drift counts per package; the ratchet fails when a package gets worse than its baseline or a new package misses the floor)"); } LIT.with(|lit| lit.clone()) };
            let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
            (v77.clone(), v76.clone())
        };
        (v78.clone(), v79.clone())
    };
    let v82: bool = "window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)" == &*v81;
    let (mut v90, mut v91): (Rc<str>, Rc<str>) = if v82 {
        (v80.clone(), v81.clone())
    } else {
        let v83: bool = &*v80 == "";
        let (mut v88, mut v89): (Rc<str>, Rc<str>) = if v83 {
            let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)"); } LIT.with(|lit| lit.clone()) };
            (v84.clone(), v84.clone())
        } else {
            let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v1));
            let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("window: request <class> <minutes> [--label <text>] [--wait <minutes>|until-granted] [--min-free-mb <n>] | release <id> | list | check <class> | claim <path> | wait-clear <class,...> [--settle <n>] [--every <seconds>] [--max <minutes>] | run <class> <minutes> [--label <text>] [--wait <minutes>] [--min-free-mb <n>] -- <command> [args...] (machine windows shared by every session; live-gui and compiler-install exclude all other classes; exit 1 when blocked)"); } LIT.with(|lit| lit.clone()) };
            let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v85, v86));
            (v87.clone(), v86.clone())
        };
        (v88.clone(), v89.clone())
    };
    let v92: bool = "experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)" == &*v91;
    let (mut v100, mut v101): (Rc<str>, Rc<str>) = if v92 {
        (v90.clone(), v91.clone())
    } else {
        let v93: bool = &*v90 == "";
        let (mut v98, mut v99): (Rc<str>, Rc<str>) = if v93 {
            let v94: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)"); } LIT.with(|lit| lit.clone()) };
            (v94.clone(), v94.clone())
        } else {
            let v95: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v1));
            let v96: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("experiment: register <ledger> <ID> --test welch|one-sample|binomial|wilson|median|binomial-band --metric <col> --runs <n> --decision <rule> [--group <col> --arms a[,b]] [--prediction <x>] [--confidence 0.95] [--question <text>] [--time <col>] | analyze <ledger> <ID> <table.tsv> [--where col=value]... [--interim <reason>] | amend <ledger> <ID> --field <name> --value <v> --reason <text> | addendum <ledger> <ID> <text> | show <ledger> <ID> (pre-registered; one analysis at the planned count)"); } LIT.with(|lit| lit.clone()) };
            let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v96));
            (v97.clone(), v96.clone())
        };
        (v98.clone(), v99.clone())
    };
    let v102: bool = "guide <spec/one-call.md> [--out <file.md>]" == &*v101;
    let (mut v110, mut v111): (Rc<str>, Rc<str>) = if v102 {
        (v100.clone(), v101.clone())
    } else {
        let v103: bool = &*v100 == "";
        let (mut v108, mut v109): (Rc<str>, Rc<str>) = if v103 {
            let v104: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
            (v104.clone(), v104.clone())
        } else {
            let v105: Rc<str> = Rc::<str>::from(format!("{}{}", v100, v1));
            let v106: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("guide <spec/one-call.md> [--out <file.md>]"); } LIT.with(|lit| lit.clone()) };
            let v107: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v106));
            (v107.clone(), v106.clone())
        };
        (v108.clone(), v109.clone())
    };
    let v112: bool = "machine watch [--sample-seconds n]" == &*v111;
    let (mut v120, mut v121): (Rc<str>, Rc<str>) = if v112 {
        (v110.clone(), v111.clone())
    } else {
        let v113: bool = &*v110 == "";
        let (mut v118, mut v119): (Rc<str>, Rc<str>) = if v113 {
            let v114: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine watch [--sample-seconds n]"); } LIT.with(|lit| lit.clone()) };
            (v114.clone(), v114.clone())
        } else {
            let v115: Rc<str> = Rc::<str>::from(format!("{}{}", v110, v1));
            let v116: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("machine watch [--sample-seconds n]"); } LIT.with(|lit| lit.clone()) };
            let v117: Rc<str> = Rc::<str>::from(format!("{}{}", v115, v116));
            (v117.clone(), v116.clone())
        };
        (v118.clone(), v119.clone())
    };
    let v122: bool = "adoption [--days n] [--out <file.tsv>] [--habits]" == &*v121;
    let (mut v130, mut v131): (Rc<str>, Rc<str>) = if v122 {
        (v120.clone(), v121.clone())
    } else {
        let v123: bool = &*v120 == "";
        let (mut v128, mut v129): (Rc<str>, Rc<str>) = if v123 {
            let v124: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adoption [--days n] [--out <file.tsv>] [--habits]"); } LIT.with(|lit| lit.clone()) };
            (v124.clone(), v124.clone())
        } else {
            let v125: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v1));
            let v126: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("adoption [--days n] [--out <file.tsv>] [--habits]"); } LIT.with(|lit| lit.clone()) };
            let v127: Rc<str> = Rc::<str>::from(format!("{}{}", v125, v126));
            (v127.clone(), v126.clone())
        };
        (v128.clone(), v129.clone())
    };
    let v132: bool = "memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n]" == &*v131;
    let (mut v140, mut v141): (Rc<str>, Rc<str>) = if v132 {
        (v130.clone(), v131.clone())
    } else {
        let v133: bool = &*v130 == "";
        let (mut v138, mut v139): (Rc<str>, Rc<str>) = if v133 {
            let v134: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n]"); } LIT.with(|lit| lit.clone()) };
            (v134.clone(), v134.clone())
        } else {
            let v135: Rc<str> = Rc::<str>::from(format!("{}{}", v130, v1));
            let v136: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("memory import <memory-dir> <data.spi> | render <data.spi> <out-dir> | check <data.spi> | list <data.spi> [--kind user|feedback|project|reference] | recall <data.spi> <words> [--limit n]"); } LIT.with(|lit| lit.clone()) };
            let v137: Rc<str> = Rc::<str>::from(format!("{}{}", v135, v136));
            (v137.clone(), v136.clone())
        };
        (v138.clone(), v139.clone())
    };
    let v142: bool = "resume <backlog-root> [--team <team>] [--as <author>]" == &*v141;
    let (mut v150, mut v151): (Rc<str>, Rc<str>) = if v142 {
        (v140.clone(), v141.clone())
    } else {
        let v143: bool = &*v140 == "";
        let (mut v148, mut v149): (Rc<str>, Rc<str>) = if v143 {
            let v144: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
            (v144.clone(), v144.clone())
        } else {
            let v145: Rc<str> = Rc::<str>::from(format!("{}{}", v140, v1));
            let v146: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("resume <backlog-root> [--team <team>] [--as <author>]"); } LIT.with(|lit| lit.clone()) };
            let v147: Rc<str> = Rc::<str>::from(format!("{}{}", v145, v146));
            (v147.clone(), v146.clone())
        };
        (v148.clone(), v149.clone())
    };
    let v152: bool = "ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)" == &*v151;
    let (mut v160, mut v161): (Rc<str>, Rc<str>) = if v152 {
        (v150.clone(), v151.clone())
    } else {
        let v153: bool = &*v150 == "";
        let (mut v158, mut v159): (Rc<str>, Rc<str>) = if v153 {
            let v154: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
            (v154.clone(), v154.clone())
        } else {
            let v155: Rc<str> = Rc::<str>::from(format!("{}{}", v150, v1));
            let v156: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ci parity <repo> (setup-rust/setup-zig/setup-beam pins in <repo>/.github/workflows against the local toolchain; exit 1 on a difference)"); } LIT.with(|lit| lit.clone()) };
            let v157: Rc<str> = Rc::<str>::from(format!("{}{}", v155, v156));
            (v157.clone(), v156.clone())
        };
        (v158.clone(), v159.clone())
    };
    let v162: bool = "process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)" == &*v161;
    let (mut v170, mut v171): (Rc<str>, Rc<str>) = if v162 {
        (v160.clone(), v161.clone())
    } else {
        let v163: bool = &*v160 == "";
        let (mut v168, mut v169): (Rc<str>, Rc<str>) = if v163 {
            let v164: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)"); } LIT.with(|lit| lit.clone()) };
            (v164.clone(), v164.clone())
        } else {
            let v165: Rc<str> = Rc::<str>::from(format!("{}{}", v160, v1));
            let v166: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("process start-under-shell <program> [args...] (detached, the user shell as its live parent, no inherited handles, no caller job; prints pid and parent)"); } LIT.with(|lit| lit.clone()) };
            let v167: Rc<str> = Rc::<str>::from(format!("{}{}", v165, v166));
            (v167.clone(), v166.clone())
        };
        (v168.clone(), v169.clone())
    };
    let v172: bool = "crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)" == &*v171;
    let (mut v180, mut v181): (Rc<str>, Rc<str>) = if v172 {
        (v170.clone(), v171.clone())
    } else {
        let v173: bool = &*v170 == "";
        let (mut v178, mut v179): (Rc<str>, Rc<str>) = if v173 {
            let v174: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)"); } LIT.with(|lit| lit.clone()) };
            (v174.clone(), v174.clone())
        } else {
            let v175: Rc<str> = Rc::<str>::from(format!("{}{}", v170, v1));
            let v176: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("crate gate <root> [--enforce] [--limit n] (generated output lines of every [package.metadata.spiral] crate under root; --enforce fails on a crate over the limit or on finding none)"); } LIT.with(|lit| lit.clone()) };
            let v177: Rc<str> = Rc::<str>::from(format!("{}{}", v175, v176));
            (v177.clone(), v176.clone())
        };
        (v178.clone(), v179.clone())
    };
    let v182: bool = "env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)" == &*v181;
    let (mut v190, mut v191): (Rc<str>, Rc<str>) = if v182 {
        (v180.clone(), v181.clone())
    } else {
        let v183: bool = &*v180 == "";
        let (mut v188, mut v189): (Rc<str>, Rc<str>) = if v183 {
            let v184: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
            (v184.clone(), v184.clone())
        } else {
            let v185: Rc<str> = Rc::<str>::from(format!("{}{}", v180, v1));
            let v186: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("env check <tool>=<pinned-path>... (each tool is looked up on PATH in order; the first copy found must be the pinned one)"); } LIT.with(|lit| lit.clone()) };
            let v187: Rc<str> = Rc::<str>::from(format!("{}{}", v185, v186));
            (v187.clone(), v186.clone())
        };
        (v188.clone(), v189.clone())
    };
    let v192: bool = "mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)" == &*v191;
    let (mut v200, mut v201): (Rc<str>, Rc<str>) = if v192 {
        (v190.clone(), v191.clone())
    } else {
        let v193: bool = &*v190 == "";
        let (mut v198, mut v199): (Rc<str>, Rc<str>) = if v193 {
            let v194: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
            (v194.clone(), v194.clone())
        } else {
            let v195: Rc<str> = Rc::<str>::from(format!("{}{}", v190, v1));
            let v196: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mirror check <left-root> <right-root> <path>... (paths relative to both roots; a directory stands for every file under it)"); } LIT.with(|lit| lit.clone()) };
            let v197: Rc<str> = Rc::<str>::from(format!("{}{}", v195, v196));
            (v197.clone(), v196.clone())
        };
        (v198.clone(), v199.clone())
    };
    let v202: bool = "backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)" == &*v201;
    let (mut v210, mut v211): (Rc<str>, Rc<str>) = if v202 {
        (v200.clone(), v201.clone())
    } else {
        let v203: bool = &*v200 == "";
        let (mut v208, mut v209): (Rc<str>, Rc<str>) = if v203 {
            let v204: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v204.clone(), v204.clone())
        } else {
            let v205: Rc<str> = Rc::<str>::from(format!("{}{}", v200, v1));
            let v206: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("backlog: list|next|plan|views|view|view-add|roots|check|todos|links <root> ... (the team backlog; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v207: Rc<str> = Rc::<str>::from(format!("{}{}", v205, v206));
            (v207.clone(), v206.clone())
        };
        (v208.clone(), v209.clone())
    };
    let v212: bool = "story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)" == &*v211;
    let (mut v220, mut v221): (Rc<str>, Rc<str>) = if v212 {
        (v210.clone(), v211.clone())
    } else {
        let v213: bool = &*v210 == "";
        let (mut v218, mut v219): (Rc<str>, Rc<str>) = if v213 {
            let v214: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v214.clone(), v214.clone())
        } else {
            let v215: Rc<str> = Rc::<str>::from(format!("{}{}", v210, v1));
            let v216: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("story: add|edit|show|set|rename|migrate-tags|note|note-edit|notes <root> ... (one backlog item; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v217: Rc<str> = Rc::<str>::from(format!("{}{}", v215, v216));
            (v217.clone(), v216.clone())
        };
        (v218.clone(), v219.clone())
    };
    let v222: bool = "standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)" == &*v221;
    let (mut v230, mut v231): (Rc<str>, Rc<str>) = if v222 {
        (v220.clone(), v221.clone())
    } else {
        let v223: bool = &*v220 == "";
        let (mut v228, mut v229): (Rc<str>, Rc<str>) = if v223 {
            let v224: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            (v224.clone(), v224.clone())
        } else {
            let v225: Rc<str> = Rc::<str>::from(format!("{}{}", v220, v1));
            let v226: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("standup: status|handoff <root> (where the work stands; same grammar as agile <verb>)"); } LIT.with(|lit| lit.clone()) };
            let v227: Rc<str> = Rc::<str>::from(format!("{}{}", v225, v226));
            (v227.clone(), v226.clone())
        };
        (v228.clone(), v229.clone())
    };
    let v232: bool = "evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)" == &*v231;
    let (mut v240, mut v241): (Rc<str>, Rc<str>) = if v232 {
        (v230.clone(), v231.clone())
    } else {
        let v233: bool = &*v230 == "";
        let (mut v238, mut v239): (Rc<str>, Rc<str>) = if v233 {
            let v234: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)"); } LIT.with(|lit| lit.clone()) };
            (v234.clone(), v234.clone())
        } else {
            let v235: Rc<str> = Rc::<str>::from(format!("{}{}", v230, v1));
            let v236: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("evidence refresh <root> <relative path> [--as <new relative path>] [--dry-run] (rewrites that file's EvidenceRef row in state/evidence.spi from the file on disk: sha256 and byte length)"); } LIT.with(|lit| lit.clone()) };
            let v237: Rc<str> = Rc::<str>::from(format!("{}{}", v235, v236));
            (v237.clone(), v236.clone())
        };
        (v238.clone(), v239.clone())
    };
    let v242: bool = "lint macros <root> [<path>...] [--baseline <rows>] (host macros by class: long, block, flow, cast, arith)" == &*v241;
    let (mut v250, mut v251): (Rc<str>, Rc<str>) = if v242 {
        (v240.clone(), v241.clone())
    } else {
        let v243: bool = &*v240 == "";
        let (mut v248, mut v249): (Rc<str>, Rc<str>) = if v243 {
            let v244: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint macros <root> [<path>...] [--baseline <rows>] (host macros by class: long, block, flow, cast, arith)"); } LIT.with(|lit| lit.clone()) };
            (v244.clone(), v244.clone())
        } else {
            let v245: Rc<str> = Rc::<str>::from(format!("{}{}", v240, v1));
            let v246: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lint macros <root> [<path>...] [--baseline <rows>] (host macros by class: long, block, flow, cast, arith)"); } LIT.with(|lit| lit.clone()) };
            let v247: Rc<str> = Rc::<str>::from(format!("{}{}", v245, v246));
            (v247.clone(), v246.clone())
        };
        (v248.clone(), v249.clone())
    };
    v250.clone()
}
fn closure1() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        command_more_usages_2()
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_subcommand_known_3(mut v0: Rc<str>) -> i32 {
    let v1: US0 = agile_subcommand_of_1(v0);
    match &v1 {
        US0::US0_AgileKnown(v2) => {
            let v2: US1 = v2.clone();
            1i32
        }
        US0::US0_AgileUnknownName => {
            0i32
        }
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        agile_subcommand_known_3(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_root_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_command_more_usages() -> Rc<str> {
    closure1()()
}
pub fn kit_agile_subcommand_known(v0: &str) -> i32 {
    closure2()(Rc::<str>::from(v0))
}
