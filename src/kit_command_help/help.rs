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
    US1_AgileSeal,
    US1_AgileTodos,
    US1_AgileNext,
    US1_AgileClaim,
    US1_AgileRelease,
    US1_AgileRoots,
    US1_AgileLinks,
    US1_AgileWip,
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
    let v74: bool = &*v0 == "seal";
    let v77: US0 = if v74 {
        let v75: US1 = US1::US1_AgileSeal;
        US0::US0_AgileKnown(v75)
    } else {
        v73
    };
    let v78: bool = &*v0 == "todos";
    let v81: US0 = if v78 {
        let v79: US1 = US1::US1_AgileTodos;
        US0::US0_AgileKnown(v79)
    } else {
        v77
    };
    let v82: bool = &*v0 == "next";
    let v85: US0 = if v82 {
        let v83: US1 = US1::US1_AgileNext;
        US0::US0_AgileKnown(v83)
    } else {
        v81
    };
    let v86: bool = &*v0 == "claim";
    let v89: US0 = if v86 {
        let v87: US1 = US1::US1_AgileClaim;
        US0::US0_AgileKnown(v87)
    } else {
        v85
    };
    let v90: bool = &*v0 == "release";
    let v93: US0 = if v90 {
        let v91: US1 = US1::US1_AgileRelease;
        US0::US0_AgileKnown(v91)
    } else {
        v89
    };
    let v94: bool = &*v0 == "roots";
    let v97: US0 = if v94 {
        let v95: US1 = US1::US1_AgileRoots;
        US0::US0_AgileKnown(v95)
    } else {
        v93
    };
    let v98: bool = &*v0 == "links";
    let v101: US0 = if v98 {
        let v99: US1 = US1::US1_AgileLinks;
        US0::US0_AgileKnown(v99)
    } else {
        v97
    };
    let v102: bool = &*v0 == "wip";
    let v105: US0 = if v102 {
        let v103: US1 = US1::US1_AgileWip;
        US0::US0_AgileKnown(v103)
    } else {
        v101
    };
    let v106: bool = &*v0 == "views";
    let v109: US0 = if v106 {
        let v107: US1 = US1::US1_AgileViews;
        US0::US0_AgileKnown(v107)
    } else {
        v105
    };
    let v110: bool = &*v0 == "view";
    let v113: US0 = if v110 {
        let v111: US1 = US1::US1_AgileView;
        US0::US0_AgileKnown(v111)
    } else {
        v109
    };
    let v114: bool = &*v0 == "view-add";
    if v114 {
        let v115: US1 = US1::US1_AgileViewAdd;
        US0::US0_AgileKnown(v115)
    } else {
        v113
    }
}
fn root_flag_refusal_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v2: US0 = agile_subcommand_of_1(v0.clone());
    match &v2 {
        US0::US0_AgileKnown(v3) => {
            let v3: US1 = v3.clone();
            let (mut v118, mut v119): (Rc<str>, Rc<str>) = match &v3 {
                US1::US1_AgileInit => {
                    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("init"); } LIT.with(|lit| lit.clone()) };
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                    (v4.clone(), v5.clone())
                }
                _ => {
                    let (mut v116, mut v117): (Rc<str>, Rc<str>) = match &v3 {
                        US1::US1_AgileBegin => {
                            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("begin"); } LIT.with(|lit| lit.clone()) };
                            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <title>"); } LIT.with(|lit| lit.clone()) };
                            (v6.clone(), v7.clone())
                        }
                        _ => {
                            let (mut v114, mut v115): (Rc<str>, Rc<str>) = match &v3 {
                                US1::US1_AgileLease => {
                                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lease"); } LIT.with(|lit| lit.clone()) };
                                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                    (v8.clone(), v9.clone())
                                }
                                _ => {
                                    let (mut v112, mut v113): (Rc<str>, Rc<str>) = match &v3 {
                                        US1::US1_AgileStatus => {
                                            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("status"); } LIT.with(|lit| lit.clone()) };
                                            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                            (v10.clone(), v11.clone())
                                        }
                                        _ => {
                                            let (mut v110, mut v111): (Rc<str>, Rc<str>) = match &v3 {
                                                US1::US1_AgileHandoff => {
                                                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("handoff"); } LIT.with(|lit| lit.clone()) };
                                                    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                    (v12.clone(), v13.clone())
                                                }
                                                _ => {
                                                    let (mut v108, mut v109): (Rc<str>, Rc<str>) = match &v3 {
                                                        US1::US1_AgileList => {
                                                            let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("list"); } LIT.with(|lit| lit.clone()) };
                                                            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [all|Planned|Active|Blocked|Paused|Done] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--sort priority|difficulty|progress] [--load light|medium|heavy|none] [--assignee agent|pair|user|none] [--json]"); } LIT.with(|lit| lit.clone()) };
                                                            (v14.clone(), v15.clone())
                                                        }
                                                        _ => {
                                                            let (mut v106, mut v107): (Rc<str>, Rc<str>) = match &v3 {
                                                                US1::US1_AgileCheck => {
                                                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("check"); } LIT.with(|lit| lit.clone()) };
                                                                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--compiler <absolute>]"); } LIT.with(|lit| lit.clone()) };
                                                                    (v16.clone(), v17.clone())
                                                                }
                                                                _ => {
                                                                    let (mut v104, mut v105): (Rc<str>, Rc<str>) = match &v3 {
                                                                        US1::US1_AgileRecord => {
                                                                            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("record"); } LIT.with(|lit| lit.clone()) };
                                                                            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                            (v18.clone(), v19.clone())
                                                                        }
                                                                        _ => {
                                                                            let (mut v102, mut v103): (Rc<str>, Rc<str>) = match &v3 {
                                                                                US1::US1_AgileSet => {
                                                                                    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("set"); } LIT.with(|lit| lit.clone()) };
                                                                                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <progress> <Planned|Active|Blocked|Paused|Done>"); } LIT.with(|lit| lit.clone()) };
                                                                                    (v20.clone(), v21.clone())
                                                                                }
                                                                                _ => {
                                                                                    let (mut v100, mut v101): (Rc<str>, Rc<str>) = match &v3 {
                                                                                        US1::US1_AgileAdd => {
                                                                                            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("add"); } LIT.with(|lit| lit.clone()) };
                                                                                            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> <Epic|Feature|Story|TaskKind> <P0..P5> <D0..D8> <title> <tags> [--estimate <m>] [--load <l>] [--assignee <a>] [--deps <A,B>] [--note <text> --as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                            (v22.clone(), v23.clone())
                                                                                        }
                                                                                        _ => {
                                                                                            let (mut v98, mut v99): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                US1::US1_AgileEdit => {
                                                                                                    let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <title|title-append|kind|priority|difficulty|deps|tags|estimate|load|assignee> <value>"); } LIT.with(|lit| lit.clone()) };
                                                                                                    (v24.clone(), v25.clone())
                                                                                                }
                                                                                                _ => {
                                                                                                    let (mut v96, mut v97): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                        US1::US1_AgileShow => {
                                                                                                            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("show"); } LIT.with(|lit| lit.clone()) };
                                                                                                            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                            (v26.clone(), v27.clone())
                                                                                                        }
                                                                                                        _ => {
                                                                                                            let (mut v94, mut v95): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                US1::US1_AgilePlan => {
                                                                                                                    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("plan"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                    (v28.clone(), v29.clone())
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    let (mut v92, mut v93): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                        US1::US1_AgileMigrateTags => {
                                                                                                                            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("migrate-tags"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("preview|apply <root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                            (v30.clone(), v31.clone())
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            let (mut v90, mut v91): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                US1::US1_AgileNote => {
                                                                                                                                    let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <text> [--as <author>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                    (v32.clone(), v33.clone())
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    let (mut v88, mut v89): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                        US1::US1_AgileNoteEdit => {
                                                                                                                                            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("note-edit"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <id> <note-number> <text> [--as <author>] [--append] [--expect-at <iso> | --expect-prefix <text>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                            (v34.clone(), v35.clone())
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            let (mut v86, mut v87): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                US1::US1_AgileNotes => {
                                                                                                                                                    let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("notes"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("from-title <root> <id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                    (v36.clone(), v37.clone())
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    let (mut v84, mut v85): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                        US1::US1_AgileRename => {
                                                                                                                                                            let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rename"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <old-id> <new-id>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                            (v38.clone(), v39.clone())
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            let (mut v82, mut v83): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                US1::US1_AgileSeal => {
                                                                                                                                                                    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("seal"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--dry-run]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                    (v40.clone(), v41.clone())
                                                                                                                                                                }
                                                                                                                                                                _ => {
                                                                                                                                                                    let (mut v80, mut v81): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                        US1::US1_AgileTodos => {
                                                                                                                                                                            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("todos"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [<path>...] [--json] [--tagged | --id <ID>] | <root> --record"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                            (v42.clone(), v43.clone())
                                                                                                                                                                        }
                                                                                                                                                                        _ => {
                                                                                                                                                                            let (mut v78, mut v79): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                US1::US1_AgileNext => {
                                                                                                                                                                                    let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("next"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [--as main|subagent] [--team <team>] [--tag <tag>] [--prefix <ID prefix>] [--words <w1,w2>] [--open] [--ready] [--blocked] [--load <loads>] [--fits] [count] | <root> --any --as <lane> [--ttl <minutes>] (claims the top free item)"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                    (v44.clone(), v45.clone())
                                                                                                                                                                                }
                                                                                                                                                                                _ => {
                                                                                                                                                                                    let (mut v76, mut v77): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                        US1::US1_AgileClaim => {
                                                                                                                                                                                            let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("claim"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane> [--paths <glob,glob>] [--ttl <minutes>]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                            (v46.clone(), v47.clone())
                                                                                                                                                                                        }
                                                                                                                                                                                        _ => {
                                                                                                                                                                                            let (mut v74, mut v75): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                US1::US1_AgileRelease => {
                                                                                                                                                                                                    let v48: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("release"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    let v49: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <ID> --as <lane>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                    (v48.clone(), v49.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                                _ => {
                                                                                                                                                                                                    let (mut v72, mut v73): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                        US1::US1_AgileRoots => {
                                                                                                                                                                                                            let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("roots"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> [add|drop <path>...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                            (v50.clone(), v51.clone())
                                                                                                                                                                                                        }
                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                            let (mut v70, mut v71): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                US1::US1_AgileLinks => {
                                                                                                                                                                                                                    let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("links"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    let v53: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> --memory <dir> [--json]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                    (v52.clone(), v53.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                    let (mut v68, mut v69): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                        US1::US1_AgileWip => {
                                                                                                                                                                                                                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("wip"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                            (v54.clone(), v55.clone())
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                            let (mut v66, mut v67): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                US1::US1_AgileViews => {
                                                                                                                                                                                                                                    let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("views"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                    let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root>"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                    (v56.clone(), v57.clone())
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                _ => {
                                                                                                                                                                                                                                    let (mut v64, mut v65): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                        US1::US1_AgileView => {
                                                                                                                                                                                                                                            let v58: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                            let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> [extra flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                            (v58.clone(), v59.clone())
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        _ => {
                                                                                                                                                                                                                                            let (mut v62, mut v63): (Rc<str>, Rc<str>) = match &v3 {
                                                                                                                                                                                                                                                US1::US1_AgileViewAdd => {
                                                                                                                                                                                                                                                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("view-add"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<root> <name> <list|next> [flags...]"); } LIT.with(|lit| lit.clone()) };
                                                                                                                                                                                                                                                    (v60.clone(), v61.clone())
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                _ => unreachable!(),
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
                                                                            (v102.clone(), v103.clone())
                                                                        }
                                                                    };
                                                                    (v104.clone(), v105.clone())
                                                                }
                                                            };
                                                            (v106.clone(), v107.clone())
                                                        }
                                                    };
                                                    (v108.clone(), v109.clone())
                                                }
                                            };
                                            (v110.clone(), v111.clone())
                                        }
                                    };
                                    (v112.clone(), v113.clone())
                                }
                            };
                            (v114.clone(), v115.clone())
                        }
                    };
                    (v116.clone(), v117.clone())
                }
            };
            let v120: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("agile "); } LIT.with(|lit| lit.clone()) };
            let v121: Rc<str> = Rc::<str>::from(format!("{}{}", v120, v118));
            let v122: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" takes <root> before any flag, got "); } LIT.with(|lit| lit.clone()) };
            let v124: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v121, v122)), v1));
            let v125: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("; usage: agile "); } LIT.with(|lit| lit.clone()) };
            let v127: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v124, v125)), v118));
            let v128: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
            let v130: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v127, v128)), v119));
            v130.clone()
        }
        US0::US0_AgileUnknownName => {
            let v131: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v131.clone()
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        root_flag_refusal_0(v0.clone(), v1.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn agile_subcommand_known_2(mut v0: Rc<str>) -> i32 {
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
fn closure1() -> Rc<dyn Fn(Rc<str>) -> i32> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> i32> = Rc::new(move |mut v0: Rc<str>| -> i32 {
        agile_subcommand_known_2(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn kit_agile_root_flag_refusal(v0: &str, v1: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn kit_agile_subcommand_known(v0: &str) -> i32 {
    closure1()(Rc::<str>::from(v0))
}
